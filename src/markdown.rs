use rust_cgi as cgi;
use std::fs;
use comrak::{markdown_to_html, Options};
use serde_json::json;
use crate::templates::render_template;

pub fn render_markdown(md_name: &str, template_name: &str, request: &cgi::Request) -> cgi::Response {
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
        Err(_) => return cgi::empty_response(404)
    }
}