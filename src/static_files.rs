use rust_cgi as cgi;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use infer;
use mime_guess;

pub fn serve_static_file(path: &str, request: &cgi::Request) -> cgi::Response {
    // handle the partial content case (we should serve read and send only the partial data):
    // first check if the request is HEAD, if so, we should only send the accept ranges header and an empty body
    if request.method() == "HEAD" {
        let content_type = mime_guess::from_path(path).first_raw().unwrap_or("application/octet-stream");
        let mut response = cgi::empty_response(200);
        response.headers_mut().insert("Accept-Ranges", "bytes".parse().unwrap());
        response.headers_mut().insert("Content-Type", content_type.parse().unwrap());
        // get the file size and insert it into the response
        let file_size = fs::metadata(&path).unwrap().len();
        response.headers_mut().insert("Content-Length", file_size.to_string().parse().unwrap());
        return response;
    } else if request.method() == "GET" && request.headers().get("Range").is_some() {
        let range = request.headers().get("Range").unwrap().to_str().unwrap();
        let range = range.trim_start_matches("bytes=");
        let range: Vec<&str> = range.split("-").collect();
        // Note the case of "bytes=0-"
        let start = match range[0].parse::<u64>() {
            Ok(val) => val,
            Err(_) => 0
        };
        let mut end = match range[1].parse::<u64>() {
            Ok(val) => val,
            Err(_) => 0
        };
        // if the end is 0, we should serve the rest of the file from the starting byte
        let mut f = fs::File::open(&path).unwrap();
        f.seek(SeekFrom::Start(start)).unwrap();

        let mut buf;
        if end == 0 {
            buf = vec![];
            end = f.metadata().unwrap().len();
            f.read_to_end(&mut buf).unwrap();
        } else {
            buf = vec![0; end as usize - start as usize];
            f.read_exact(&mut buf).unwrap();
        }

        let content_type = mime_guess::from_path(path).first_raw().unwrap_or("application/octet-stream");
        let mut response = cgi::binary_response(206, content_type, buf);
        response.headers_mut().insert("Accept-Ranges", "bytes".parse().unwrap());
        response.headers_mut().insert("Content-Range", format!("bytes {}-{}/{}", start, end, f.metadata().unwrap().len()).parse().unwrap());
        return response;
    }
    match fs::read(&path) {
        Ok(content) => {
            let content_type = infer::get(&content);
            let length = content.len();
            match content_type {
                None => {
                    let mime_type = mime_guess::from_path(path).first_raw().unwrap_or("application/octet-stream");
                    let mut response = cgi::binary_response(200, mime_type, content);
                    response.headers_mut().insert("Accept-Ranges", "bytes".parse().unwrap());
                    response.headers_mut().insert("Content-Length", length.to_string().parse().unwrap());
                    return response;
                },
                _ => {
                    let mut response = cgi::binary_response(200, content_type.unwrap().mime_type(), content);
                    response.headers_mut().insert("Accept-Ranges", "bytes".parse().unwrap());
                    response.headers_mut().insert("Content-Length", length.to_string().parse().unwrap());
                    return response;
                }
            }
        }
        Err(_) => return cgi::empty_response(404)
    }
}