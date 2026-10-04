mod blog;
mod error;
mod request;
mod rss;
mod site;
mod static_files;

use std::env;

use matchit::Router;
use rust_cgi as cgi;
use rust_cgi::http::{HeaderValue, header};
use rustc_version_runtime::version;
use serde_json::json;

use crate::error::{Error, Result};
use crate::request::{RequestExt, redirect};
use crate::site::Site;

#[derive(Clone, Copy, PartialEq)]
enum Route {
    Home,
    BlogIndex,
    BlogPost,
    Feed,
    Debug,
    Static,
}

const ROUTES: [(&str, Route); 6] = [
    ("/", Route::Home),
    ("/blog", Route::BlogIndex),
    ("/blog/{post}", Route::BlogPost),
    ("/index.xml", Route::Feed),
    ("/debug", Route::Debug),
    ("/{*path}", Route::Static),
];

cgi::cgi_main! { |request: cgi::Request| -> cgi::Response {
    handle(&request).unwrap_or_else(Error::into_response)
}}

fn handle(request: &cgi::Request) -> Result<cgi::Response> {
    let path = request.header_str("X-CGI-Path-Info").unwrap_or("/");
    if path.contains("..") {
        return Ok(cgi::empty_response(403));
    }
    if let Some(response) = upgrade_insecure_request(request, path) {
        return response;
    }

    let mut router = Router::new();
    for (pattern, route) in ROUTES {
        router.insert(pattern, route).expect("route table is valid");
    }

    let matched = router.at(path).map_err(|_| Error::NotFound)?;
    match *matched.value {
        Route::Static if path.len() > 1 && path.ends_with('/') => {
            let trimmed = path.trim_end_matches('/');
            match router.at(trimmed) {
                Ok(m) if *m.value != Route::Static => redirect(trimmed),
                _ => Err(Error::NotFound),
            }
        }
        Route::Home => Site::load(request)?.markdown_page("content/index.md", "index"),
        Route::BlogIndex => {
            let posts: Vec<_> = blog::file_names_newest_first()?
                .into_iter()
                .map(|name| json!({ "path": format!("/blog/{name}"), "name": name }))
                .collect();
            Site::load(request)?.page("blog-index", json!({ "posts": posts }))
        }
        Route::BlogPost => {
            let path = matched
                .params
                .get("post")
                .and_then(blog::post_path)
                .ok_or(Error::NotFound)?;
            Site::load(request)?.markdown_page(&path, "blog-post")
        }
        Route::Feed => rss::feed(&Site::load(request)?),
        Route::Debug => Ok(debug_page(request)),
        Route::Static => {
            let file = request
                .header_str("X-CGI-Path-Translated")
                .ok_or(Error::NotFound)?;
            static_files::serve(file, request)
        }
    }
}

/// Behind Cloudflare, plain-HTTP visitors arrive with `X-Forwarded-Proto: http`; direct hits
/// on IIS arrive on port 80.
fn upgrade_insecure_request(request: &cgi::Request, path: &str) -> Option<Result<cgi::Response>> {
    request.header_str("Upgrade-Insecure-Requests")?;
    let insecure = request.header_str("X-CGI-Server-Port") == Some("80")
        || request.header_str("X-Forwarded-Proto") == Some("http");
    if !insecure {
        return None;
    }
    let host = request.header_str("Host")?;
    let location = match request.header_str("X-CGI-Query-String") {
        Some(query) if !query.is_empty() => format!("https://{host}{path}?{query}"),
        _ => format!("https://{host}{path}"),
    };
    Some(redirect(&location).map(|mut response| {
        let vary = HeaderValue::from_static("Upgrade-Insecure-Requests");
        response.headers_mut().insert(header::VARY, vary);
        response
    }))
}

fn debug_page(request: &cgi::Request) -> cgi::Response {
    let headers: Vec<String> = request
        .headers()
        .iter()
        .map(|(name, value)| format!("{name}: {}", String::from_utf8_lossy(value.as_bytes())))
        .collect();
    let cwd = env::current_dir()
        .map(|dir| dir.display().to_string())
        .unwrap_or_default();
    cgi::text_response(
        200,
        format!(
            "{}\n\nCWD: {cwd}\nRust Version: {}",
            headers.join("\n"),
            version()
        ),
    )
}
