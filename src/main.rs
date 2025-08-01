use cgi::http::HeaderValue;
use rust_cgi as cgi;
use matchit::Router;
use std::env;
use std::fs;
use rustc_version_runtime::version;
use serde_json::json;

mod static_files;
mod markdown;
mod templates;
mod utils;
mod rss;

use static_files::serve_static_file;
use markdown::render_markdown;
use templates::render_template;
use rss::generate_rss_feed;

fn debug_page(request: &cgi::Request) -> cgi::Response {
    let headers_string = request.headers().iter().map(|(name, value)| {
        format!("{}: {}", name, value.to_str().unwrap())
    }).collect::<Vec<String>>().join("\n");
    let cwd_env = env::current_dir().unwrap();
    let cwd = cwd_env.to_str().unwrap();
    let rust_ver = version();
    let headers_string = format!("{}\n\nCWD: {}\nRust Version: {}", headers_string, cwd, rust_ver);
    return cgi::text_response(200, headers_string)
}

cgi::cgi_main! { |request: cgi::Request| -> cgi::Response {
    let path = request.headers().get("X-CGI-PATH-INFO").unwrap().to_str().unwrap();
    let full_path = request.headers().get("X-CGI-PATH-TRANSLATED").unwrap().to_str().unwrap();
    if path.contains("..") {
        return cgi::empty_response(403);
    }

    // Handle the Upgrade-Insecure-Requests header if x-cgi-server-port is 80 or x-forwarded-proto is http
    if request.headers().get("Upgrade-Insecure-Requests").is_some() && (request.headers().get("X-CGI-SERVER-PORT").unwrap().to_str().unwrap() == "80" || request.headers().get("X-FORWARDED-PROTO").unwrap_or(&HeaderValue::from_static("https")).to_str().unwrap() == "http") {
        let mut response = cgi::empty_response(301);
        response.headers_mut().insert("Location", format!("https://{}", request.headers().get("Host").unwrap().to_str().unwrap()).parse().unwrap());
        response.headers_mut().insert("Vary", "Upgrade-Insecure-Requests".parse().unwrap());
        return response;
    }

    let mut router = Router::new();
    router.insert("/", Box::new(|_params: matchit::Params| { render_markdown("index.md", "index.hbs", &request) }) as Box<dyn Fn(matchit::Params) -> cgi::Response>).unwrap();
    router.insert("/blog", Box::new(|_params: matchit::Params| {
        // get all files in content/blog and render them
        let mut blog_posts = vec![];
        let blog_dir = fs::read_dir("content/blog").unwrap();
        for entry in blog_dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            blog_posts.push(json!({
                "name": file_name,
                "path": format!("/blog/{}", file_name)
            }));
        }
        blog_posts.reverse();
        render_template("blog-index.hbs", json!({ "posts": blog_posts }), &request)
    })).unwrap();
    router.insert("/blog/:post", Box::new(|params: matchit::Params| {
        let post = params.get("post").unwrap().trim_end_matches(".md");
        render_markdown(&format!("blog/{}.md", post), "blog-post.hbs", &request)
    })).unwrap();
    router.insert("/index.xml", Box::new(|_params: matchit::Params| { generate_rss_feed(&request) })).unwrap();

    router.insert("/debug", Box::new(|_params: matchit::Params| { debug_page(&request) })).unwrap();
    router.insert("/*p", Box::new(|_params: matchit::Params| { serve_static_file(full_path, &request) })).unwrap();

    match router.at(&path) {
        Ok(matched) => {
            let params = matched.params;
            let response = (matched.value)(params);
            return response;
        }
        Err(matchit::MatchError::ExtraTrailingSlash) => {
            let redirect_path = path.trim_end_matches('/');
            let mut response = cgi::empty_response(301);
            response.headers_mut().insert("Location", redirect_path.parse().unwrap());
            return response;
        }
        Err(_) => {
            // For any other routing error, return 404
            return cgi::empty_response(404);
        }
    }
}}