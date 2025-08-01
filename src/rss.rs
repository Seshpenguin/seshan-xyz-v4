use rust_cgi as cgi;
use std::fs;
use serde_json::json;
use chrono::{DateTime, Utc};
use std::path::Path;
use comrak::{markdown_to_html, Options};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use crate::templates::Config;
use crate::utils::{get_date_string, get_year};
use rustc_version_runtime::version;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlogPost {
    title: String,
    filename: String,
    pub_date: DateTime<Utc>,
    pub_date_rss: String,  // RFC2822 formatted date for RSS
    description: String,
    content: String,
}

pub fn generate_rss_feed(request: &cgi::Request) -> cgi::Response {
    // Load config
    let config_file = fs::read_to_string("content/seshanxyz.toml").unwrap_or_default();
    let config: Config = toml::from_str(&config_file).unwrap_or_else(|_| Config {
        title: "My Blog".to_string(),
        header: "My Blog".to_string(),
        subheader: "Latest Posts".to_string(),
        desc: "A blog about various topics".to_string(),
        copyright: "All rights reserved".to_string(),
    });

    // Get blog posts
    let mut blog_posts = vec![];
    if let Ok(blog_dir) = fs::read_dir("content/blog") {
        for entry in blog_dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(post_data) = extract_post_metadata(&path) {
                        blog_posts.push(post_data);
                    }
                }
            }
        }
    }

    // Sort posts by date (newest first)
    blog_posts.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    // Get the base URL from the request
    let host = request.headers().get("Host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost");
    let scheme = if request.headers().get("X-FORWARDED-PROTO")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("https") == "http" { "http" } else { "https" };
    let base_url = format!("{}://{}", scheme, host);
    let domain = host.to_string();

    // Get system info
    let server_software = request.headers().get("X-CGI-SERVER-SOFTWARE")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");
    let rust_ver = version().to_string();
    let info = os_info::get();

    // Prepare template data
    let now = Utc::now();
    let template_data = json!({
        "config": config,
        "posts": blog_posts.iter().take(20).collect::<Vec<_>>(),  // Limit to 20 posts
        "base_url": base_url,
        "domain": domain,
        "date": get_date_string(),
        "year": get_year(),
        "rss_date": now.format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
        "server_software": server_software,
        "rust_ver": rust_ver,
        "os_info": info.to_string()
    });

    // Render RSS using Handlebars
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);  // Disable HTML escaping for RSS
    
    match handlebars.register_template_file("rss", "templates/rss.hbs") {
        Ok(_) => {
            match handlebars.render("rss", &template_data) {
                Ok(rendered) => {
                    let mut response = cgi::text_response(200, rendered);
                    response.headers_mut().insert("Content-Type", "application/xml; charset=utf-8".parse().unwrap());
                    response
                }
                Err(e) => {
                    eprintln!("RSS template render error: {}", e);
                    cgi::text_response(500, format!("RSS generation error: {}", e))
                }
            }
        }
        Err(e) => {
            eprintln!("RSS template load error: {}", e);
            cgi::text_response(500, format!("RSS template error: {}", e))
        }
    }
}

fn extract_post_metadata(path: &Path) -> Option<BlogPost> {
    let content = fs::read_to_string(path).ok()?;
    let filename = path.file_stem()?.to_str()?.to_string();
    
    // Try to extract title from the first # heading
    let title = extract_title_from_markdown(&content)
        .unwrap_or_else(|| filename.replace('-', " ").replace('_', " "));
    
    // Try to get file modification time, fallback to current time
    let pub_date = path.metadata()
        .and_then(|m| m.modified())
        .map(|t| DateTime::<Utc>::from(t))
        .unwrap_or_else(|_| Utc::now());
    
    // Format date for RSS (RFC2822)
    let pub_date_rss = pub_date.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    
    // Generate description from first paragraph or first 200 chars
    let description = extract_description_from_markdown(&content);
    
    // Convert markdown to HTML for content
    let mut options = Options::default();
    options.render.unsafe_ = true;
    let html_content = markdown_to_html(&content, &options);
    
    Some(BlogPost {
        title,
        filename,
        pub_date,
        pub_date_rss,
        description,
        content: html_content,
    })
}

fn extract_title_from_markdown(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return Some(trimmed[2..].trim().to_string());
        }
    }
    None
}

fn extract_description_from_markdown(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut description = String::new();
    let mut found_content = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Skip title lines
        if trimmed.starts_with('#') {
            continue;
        }
        
        // Skip empty lines at the beginning
        if !found_content && trimmed.is_empty() {
            continue;
        }
        
        // Found first content line
        if !trimmed.is_empty() {
            found_content = true;
            description.push_str(trimmed);
            description.push(' ');
            
            // Stop at first paragraph break or after 200 characters
            if description.len() > 200 {
                break;
            }
        } else if found_content {
            // Empty line after content = end of paragraph
            break;
        }
    }
    
    // Truncate and clean up
    if description.len() > 200 {
        description.truncate(200);
        if let Some(last_space) = description.rfind(' ') {
            description.truncate(last_space);
        }
        description.push_str("...");
    }
    
    description.trim().to_string()
}