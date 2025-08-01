use rust_cgi as cgi;
use std::fs;
use handlebars::Handlebars;
use serde_json::json;
use rustc_version_runtime::version;
use serde::{Deserialize, Serialize};
use crate::utils::{get_date_string, get_year};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub header: String,
    pub subheader: String,
    pub desc: String,
    pub copyright: String,
}

pub fn render_template(template_name: &str, data: serde_json::Value, request: &cgi::Request) -> cgi::Response {
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