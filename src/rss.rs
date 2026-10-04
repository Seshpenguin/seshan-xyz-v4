use chrono::{DateTime, Utc};
use rust_cgi as cgi;
use serde::Serialize;

use crate::blog::{self, Post};
use crate::error::Result;
use crate::request::RequestExt;
use crate::site::{Site, markdown_to_html};

const FEED_LENGTH: usize = 20;
const SUMMARY_CHARS: usize = 200;

#[derive(Serialize)]
struct Feed<'a> {
    base_url: String,
    domain: &'a str,
    rss_date: String,
    posts: Vec<Item>,
}

#[derive(Serialize)]
struct Item {
    title: String,
    filename: String,
    pub_date_rss: String,
    description: String,
    content: String,
}

impl From<Post> for Item {
    fn from(post: Post) -> Self {
        Self {
            description: post.summary(SUMMARY_CHARS),
            pub_date_rss: rfc822(post.published),
            content: markdown_to_html(&post.body).replace("]]>", "]]]]><![CDATA[>"),
            title: post.title,
            filename: post.slug,
        }
    }
}

pub fn feed(site: &Site) -> Result<cgi::Response> {
    let domain = site.request.header_str("Host").unwrap_or("localhost");
    let scheme = match site.request.header_str("X-Forwarded-Proto") {
        Some("http") => "http",
        _ => "https",
    };
    let feed = Feed {
        base_url: format!("{scheme}://{domain}"),
        domain,
        rss_date: rfc822(Utc::now()),
        posts: blog::published_posts_newest_first()?
            .into_iter()
            .take(FEED_LENGTH)
            .map(Item::from)
            .collect(),
    };
    let xml = site.render("rss", feed)?;
    Ok(cgi::binary_response(
        200,
        "application/xml; charset=utf-8",
        xml.into_bytes(),
    ))
}

fn rfc822(date: DateTime<Utc>) -> String {
    date.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}
