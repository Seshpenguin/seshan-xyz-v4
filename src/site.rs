use std::fs;

use chrono::{Datelike, Utc};
use handlebars::Handlebars;
use rust_cgi as cgi;
use rustc_version_runtime::version;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::Result;
use crate::request::RequestExt;

const CONFIG_PATH: &str = "content/seshanxyz.toml";
const TEMPLATES: [&str; 5] = ["base", "index", "blog-index", "blog-post", "rss"];

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub header: String,
    pub subheader: String,
    pub desc: String,
    pub copyright: String,
}

pub struct Site<'r> {
    pub request: &'r cgi::Request,
    config: Config,
    templates: Handlebars<'static>,
}

#[derive(Serialize)]
struct Context<'a, T> {
    parent: &'static str,
    config: &'a Config,
    server_software: &'a str,
    rust_ver: String,
    os_info: String,
    year: i32,
    date: String,
    #[serde(flatten)]
    page: T,
}

impl<'r> Site<'r> {
    pub fn load(request: &'r cgi::Request) -> Result<Self> {
        let config = toml::from_str(&fs::read_to_string(CONFIG_PATH)?)?;
        let mut templates = Handlebars::new();
        for name in TEMPLATES {
            templates.register_template_file(name, format!("templates/{name}.hbs"))?;
        }
        Ok(Self {
            request,
            config,
            templates,
        })
    }

    pub fn render(&self, template: &str, page: impl Serialize) -> Result<String> {
        let now = Utc::now();
        let context = Context {
            parent: "base",
            config: &self.config,
            server_software: self
                .request
                .header_str("X-CGI-Server-Software")
                .unwrap_or_default(),
            rust_ver: version().to_string(),
            os_info: os_info::get().to_string(),
            year: now.year(),
            date: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            page,
        };
        Ok(self.templates.render(template, &context)?)
    }

    pub fn page(&self, template: &str, page: impl Serialize) -> Result<cgi::Response> {
        Ok(cgi::html_response(200, self.render(template, page)?))
    }

    pub fn markdown_page(&self, markdown_path: &str, template: &str) -> Result<cgi::Response> {
        let markdown = fs::read_to_string(markdown_path)?;
        self.page(template, json!({ "content": markdown_to_html(&markdown) }))
    }
}

pub fn markdown_to_html(markdown: &str) -> String {
    let markdown = markdown.replace("legacy.seshan.xyz", "seshan.xyz");
    let mut options = comrak::Options::default();
    options.render.unsafe_ = true;
    comrak::markdown_to_html(&markdown, &options)
}
