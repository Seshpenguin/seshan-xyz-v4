use std::fs;
use std::path::Path;

use chrono::{DateTime, NaiveDate, Utc};

use crate::error::Result;

const BLOG_DIR: &str = "content/blog";

pub fn file_names_newest_first() -> Result<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(BLOG_DIR)?
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .collect();
    names.sort_unstable_by(|a, b| b.cmp(a));
    Ok(names)
}

pub fn post_path(slug: &str) -> Option<String> {
    let slug = slug.trim_end_matches(".md");
    (!slug.contains(['/', '\\'])).then(|| format!("{BLOG_DIR}/{slug}.md"))
}

pub fn published_posts_newest_first() -> Result<Vec<Post>> {
    let mut posts: Vec<Post> = fs::read_dir(BLOG_DIR)?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .filter_map(|path| Post::load(&path))
        .collect();
    posts.sort_by(|a, b| b.published.cmp(&a.published));
    Ok(posts)
}

#[derive(Debug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub published: DateTime<Utc>,
    pub body: String,
}

impl Post {
    fn load(path: &Path) -> Option<Self> {
        let slug = path.file_stem()?.to_str()?;
        let source = fs::read_to_string(path).ok()?;
        Self::parse(slug, &source)
    }

    /// Returns `None` for drafts and for posts with no usable date.
    fn parse(slug: &str, source: &str) -> Option<Self> {
        let (front_matter, body) = split_front_matter(source);
        let field = |key| front_matter_field(front_matter, key);

        if field("draft") == Some("true") {
            return None;
        }

        let title = field("title")
            .map(unquote)
            .or_else(|| first_heading(body))
            .unwrap_or_else(|| slug.replace(['-', '_'], " "));

        let published = field("date")
            .and_then(|date| DateTime::parse_from_rfc3339(date).ok())
            .map(|date| date.to_utc())
            .or_else(|| date_prefix(slug))?;

        Some(Self {
            slug: slug.to_owned(),
            title,
            published,
            body: body.to_owned(),
        })
    }

    pub fn summary(&self, max_chars: usize) -> String {
        let paragraph: Vec<&str> = self
            .body
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with('#'))
            .skip_while(|line| line.is_empty())
            .take_while(|line| !line.is_empty())
            .collect();
        let paragraph = paragraph.join(" ");

        if paragraph.chars().count() <= max_chars {
            return paragraph;
        }
        let truncated: String = paragraph.chars().take(max_chars).collect();
        let cut = truncated.rfind(' ').unwrap_or(truncated.len());
        format!("{}...", &truncated[..cut])
    }
}

fn split_front_matter(source: &str) -> (&str, &str) {
    let Some(rest) = source
        .strip_prefix("---\n")
        .or_else(|| source.strip_prefix("---\r\n"))
    else {
        return ("", source);
    };
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return (&rest[..offset], &rest[offset + line.len()..]);
        }
        offset += line.len();
    }
    ("", source)
}

fn front_matter_field<'a>(front_matter: &'a str, key: &str) -> Option<&'a str> {
    front_matter
        .lines()
        .find_map(|line| line.strip_prefix(key)?.strip_prefix(':'))
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn unquote(value: &str) -> String {
    if let Some(inner) = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
        inner.replace("''", "'")
    } else if let Some(inner) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
        inner.replace("\\\"", "\"")
    } else {
        value.to_owned()
    }
}

fn first_heading(markdown: &str) -> Option<String> {
    markdown
        .lines()
        .find_map(|line| line.trim().strip_prefix("# "))
        .map(|title| title.trim().to_owned())
}

fn date_prefix(slug: &str) -> Option<DateTime<Utc>> {
    let date = NaiveDate::parse_from_str(slug.get(..10)?, "%Y-%m-%d").ok()?;
    Some(date.and_hms_opt(0, 0, 0)?.and_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORDPRESS_POST: &str = "---\ntitle: 'JS Tip: Don''t Panic!'\nauthor: Seshan\ndate: 2019-07-26T14:00:00+00:00\ndate_gmt: nope\ncategories:\n  - Uncategorized\n---\n\nFirst line\nof the intro.\n\nSecond paragraph.\n";

    #[test]
    fn parses_wordpress_front_matter() {
        let post = Post::parse("2019-07-26-js-tip", WORDPRESS_POST).unwrap();
        assert_eq!(post.title, "JS Tip: Don't Panic!");
        assert_eq!(post.published.to_rfc3339(), "2019-07-26T14:00:00+00:00");
        assert_eq!(post.summary(200), "First line of the intro.");
    }

    #[test]
    fn skips_drafts() {
        let draft = "---\ntitle: Draft\ndate: -001-11-30T00:00:00+00:00\ndraft: true\n---\nbody";
        assert!(Post::parse("2019-05-23-", draft).is_none());
    }

    #[test]
    fn falls_back_to_heading_and_filename_date() {
        let post = Post::parse("2024-10-02-test", "# Hello there\n\nbody text").unwrap();
        assert_eq!(post.title, "Hello there");
        assert_eq!(post.published.to_rfc3339(), "2024-10-02T00:00:00+00:00");
        assert_eq!(post.summary(200), "body text");
    }

    #[test]
    fn falls_back_to_slug_for_title() {
        let post = Post::parse("2020-01-01-another_post", "just text").unwrap();
        assert_eq!(post.title, "2020 01 01 another post");
    }

    #[test]
    fn skips_undated_posts() {
        assert!(Post::parse("another_post", "# Scratch\n\ntext").is_none());
    }

    #[test]
    fn handles_crlf_front_matter() {
        let source =
            "---\r\ntitle: Windows\r\ndate: 2003-04-24T00:00:00+00:00\r\n---\r\nServer 2003!\r\n";
        let post = Post::parse("x", source).unwrap();
        assert_eq!(post.title, "Windows");
        assert_eq!(post.summary(200), "Server 2003!");
    }

    #[test]
    fn summary_truncates_on_char_boundary_at_a_word() {
        let post = Post::parse("2020-01-01-x", "ünïcödé wörds everywhere here").unwrap();
        assert_eq!(post.summary(12), "ünïcödé...");
    }

    #[test]
    fn rejects_path_traversal_in_slugs() {
        assert_eq!(
            post_path("hello.md").as_deref(),
            Some("content/blog/hello.md")
        );
        assert_eq!(post_path("a\\b"), None);
    }
}
