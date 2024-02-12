use rust_cgi as cgi;
use matchit::Router;
use std::env;
use std::fs;
use infer;
use mime_guess;
use handlebars::Handlebars;
use serde_json::json;
use rustc_version_runtime::version;
use comrak::{markdown_to_html, Options};
use serde::{Deserialize, Serialize};

use std::time::SystemTime;
use winapi;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    title: String,
    header: String,
    subheader: String,
    copyright: String,
}

fn get_date_string() -> String {
    // get date from winapi
     let mut system_time = winapi::um::minwinbase::SYSTEMTIME {
          wYear: 0,
          wMonth: 0,
          wDayOfWeek: 0,
          wDay: 0,
          wHour: 0,
          wMinute: 0,
          wSecond: 0,
          wMilliseconds: 0,
     };
     unsafe {
          winapi::um::sysinfoapi::GetSystemTime(&mut system_time);
     }
     let year = system_time.wYear;
     let month = system_time.wMonth;
     let day = system_time.wDay;
     let hour = system_time.wHour;
     let minute = system_time.wMinute;
     let second = system_time.wSecond;

     return format!("{}-{}-{} {}:{}:{}", year, month, day, hour, minute, second);
}

fn get_year() -> u32 {
     let now = SystemTime::now();
     let year = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() / 60 / 60 / 24 / 365;
     return year as u32 + 1970;
}

fn serve_static_file(path: &str) -> cgi::Response {
     match fs::read(&path) {
          Ok(content) => {
               let content_type = infer::get(&content);
               match content_type {
                    None => {
                         let mime_type = mime_guess::from_path(path).first_raw().unwrap_or("application/octet-stream");
                         return cgi::binary_response(200, mime_type, content);
                    },
                    _ => return cgi::binary_response(200, content_type.unwrap().mime_type(), content)
               }
          }
          Err(_) => return cgi::text_response(404, "Not Found")
     }
}
fn render_template(template_name: &str, data: serde_json::Value, request: &cgi::Request) -> cgi::Response {
     let mut handlebars = Handlebars::new();
     handlebars.register_template_file("base", "templates/base.hbs").unwrap();
     handlebars.register_template_file(template_name, format!("templates/{}", template_name)).unwrap();

     let server_software = request.headers().get("X-CGI-SERVER-SOFTWARE").unwrap().to_str().unwrap();
     let rust_ver = version().to_string();
     let info = os_info::get();

     let config_file = fs::read_to_string("content/seshanxyz.toml").unwrap(); 
     let config: Config = toml::from_str(&config_file).unwrap();

     let mut data = data.as_object().unwrap().clone();
     data.insert("parent".to_string(), json!("base"));
     data.insert("server_software".to_string(), json!(server_software));
     data.insert("rust_ver".to_string(), json!(rust_ver));
     data.insert("os_info".to_string(), json!(info.to_string()));
     data.insert("year".to_string(), json!(get_year()));
     data.insert("date".to_string(), json!(get_date_string()));
     data.insert("config".to_string(), json!(config));

     let rendered = handlebars.render(template_name, &data).unwrap();
     return cgi::html_response(200, rendered)
}

fn render_markdown(md_name: &str, template_name: &str, request: &cgi::Request) -> cgi::Response {
     let md_path = format!("content/{}", md_name);
     match fs::read_to_string(&md_path) {
          Ok(content) => {
               // rewrite all instances of "legacy.seshan.xyz" to "seshan.xyz"
               let content = content.replace("legacy.seshan.xyz", "seshan.xyz");
               let mut options = Options::default();
               options.render.unsafe_ = true;
               let mdhtml = markdown_to_html(&content, &options);
               let html = render_template(template_name, json!({
                    "content": mdhtml
               }), request);
               return html;
          },
          Err(_) => return cgi::text_response(404, "Not Found")
     }
}

fn debug_page(request: &cgi:: Request) -> cgi::Response {
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
          return cgi::text_response(403, "Forbidden");
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
     router.insert("/debug", Box::new(|_params: matchit::Params| { debug_page(&request) })).unwrap();
     router.insert("/*p", Box::new(|_params: matchit::Params| { serve_static_file(full_path) })).unwrap();

     let matched = router.at(&path).unwrap();
     let params = matched.params;
     let response = (matched.value)(params);
     return response;
}}