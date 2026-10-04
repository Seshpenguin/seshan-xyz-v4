use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};

use rust_cgi as cgi;
use rust_cgi::http::{HeaderValue, Method, header};

use crate::error::{Error, Result};
use crate::request::RequestExt;

const SNIFF_LEN: u64 = 8192;

#[derive(Debug, PartialEq)]
enum ByteRange {
    Full,
    Partial { start: u64, end: u64 },
    Unsatisfiable,
}

pub fn serve(path: &str, request: &cgi::Request) -> Result<cgi::Response> {
    let metadata = fs::metadata(path)?;
    if !metadata.is_file() {
        return Err(Error::NotFound);
    }
    let len = metadata.len();
    let mut file = File::open(path)?;

    let (status, start, count) = match byte_range(request.header_str("Range"), len) {
        ByteRange::Full => (200, 0, len),
        ByteRange::Partial { start, end } => (206, start, end - start + 1),
        ByteRange::Unsatisfiable => {
            let mut response = cgi::empty_response(416);
            response
                .headers_mut()
                .insert(header::CONTENT_RANGE, content_range(None, len));
            return Ok(response);
        }
    };

    let content_type = sniff_content_type(path, &mut file)?;
    let body = if request.method() == Method::HEAD {
        Vec::new()
    } else {
        read_span(&mut file, start, count)?
    };

    let mut response = cgi::binary_response(status, content_type, body);
    let headers = response.headers_mut();
    headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    headers.insert(header::CONTENT_LENGTH, HeaderValue::from(count));
    if status == 206 {
        headers.insert(
            header::CONTENT_RANGE,
            content_range(Some((start, start + count - 1)), len),
        );
    }
    Ok(response)
}

/// Only single ranges are supported; anything else is ignored and the whole file is served,
/// which RFC 9110 permits.
fn byte_range(header: Option<&str>, len: u64) -> ByteRange {
    let Some((first, last)) = header
        .and_then(|value| value.trim().strip_prefix("bytes="))
        .and_then(|spec| spec.split_once('-'))
    else {
        return ByteRange::Full;
    };
    let (first, last) = (first.trim(), last.trim());

    let (start, end) = match (first.parse::<u64>(), last.parse::<u64>()) {
        (Ok(start), Ok(end)) if start <= end => (start, end.min(len.saturating_sub(1))),
        (Ok(start), Err(_)) if last.is_empty() => (start, len.saturating_sub(1)),
        (Err(_), Ok(suffix)) if first.is_empty() => {
            if suffix == 0 {
                return ByteRange::Unsatisfiable;
            }
            (len.saturating_sub(suffix), len.saturating_sub(1))
        }
        _ => return ByteRange::Full,
    };

    if start >= len {
        ByteRange::Unsatisfiable
    } else {
        ByteRange::Partial { start, end }
    }
}

fn content_range(span: Option<(u64, u64)>, len: u64) -> HeaderValue {
    let value = match span {
        Some((start, end)) => format!("bytes {start}-{end}/{len}"),
        None => format!("bytes */{len}"),
    };
    HeaderValue::try_from(value).expect("formatted range is a valid header value")
}

fn sniff_content_type(path: &str, file: &mut File) -> io::Result<&'static str> {
    let mut head = Vec::new();
    file.by_ref().take(SNIFF_LEN).read_to_end(&mut head)?;
    Ok(infer::get(&head)
        .map(|kind| kind.mime_type())
        .or_else(|| mime_guess::from_path(path).first_raw())
        .unwrap_or("application/octet-stream"))
}

fn read_span(file: &mut File, start: u64, count: u64) -> io::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(start))?;
    let mut buf = Vec::with_capacity(usize::try_from(count).unwrap_or_default());
    file.take(count).read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(header: &str, len: u64) -> ByteRange {
        byte_range(Some(header), len)
    }

    #[test]
    fn closed_ranges_are_inclusive() {
        assert_eq!(
            range("bytes=0-99", 1000),
            ByteRange::Partial { start: 0, end: 99 }
        );
        assert_eq!(
            range("bytes=0-0", 1000),
            ByteRange::Partial { start: 0, end: 0 }
        );
    }

    #[test]
    fn open_and_suffix_ranges() {
        assert_eq!(
            range("bytes=100-", 1000),
            ByteRange::Partial {
                start: 100,
                end: 999
            }
        );
        assert_eq!(
            range("bytes=-50", 1000),
            ByteRange::Partial {
                start: 950,
                end: 999
            }
        );
        assert_eq!(
            range("bytes=-5000", 1000),
            ByteRange::Partial { start: 0, end: 999 }
        );
    }

    #[test]
    fn end_is_clamped_to_file_length() {
        assert_eq!(
            range("bytes=900-5000", 1000),
            ByteRange::Partial {
                start: 900,
                end: 999
            }
        );
    }

    #[test]
    fn unsatisfiable_ranges() {
        assert_eq!(range("bytes=1000-", 1000), ByteRange::Unsatisfiable);
        assert_eq!(range("bytes=-0", 1000), ByteRange::Unsatisfiable);
        assert_eq!(range("bytes=0-", 0), ByteRange::Unsatisfiable);
    }

    #[test]
    fn malformed_or_multi_ranges_serve_everything() {
        assert_eq!(byte_range(None, 1000), ByteRange::Full);
        assert_eq!(range("garbage", 1000), ByteRange::Full);
        assert_eq!(range("bytes=5-1", 1000), ByteRange::Full);
        assert_eq!(range("bytes=0-1,5-9", 1000), ByteRange::Full);
        assert_eq!(range("items=0-1", 1000), ByteRange::Full);
    }
}
