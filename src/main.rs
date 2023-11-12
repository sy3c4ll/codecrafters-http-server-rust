mod http_request;
mod http_response;
mod http_version;

use http_request::{HttpRequest, http_method::HttpMethod};
use http_response::{HttpResponse, http_status::HttpStatus};
use http_version::HttpVersion;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("[#] Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                eprintln!("[+] accepted new connection");
                std::thread::spawn(|| match handle_client(stream) {
                    Ok(status) => {
                        eprintln!("[+] transaction complete with status {status}");
                    }
                    Err(e) => {
                        eprintln!("[!] error while responding: {e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("[!] error while connecting: {e}");
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<HttpStatus> {
    let mut buf = [0u8; 0x1000];
    let len = stream.read(&mut buf)?;

    let request = parse_request(&buf[..len]);
    let response = match request {
        Some(request) => request_response(&request),
        None => bad_request_response(),
    };
    let buf = compose_response(&response);

    stream.write_all(&buf[..])?;
    Ok(response.status)
}

fn parse_request(buf: &[u8]) -> Option<HttpRequest> {
    let mut lines = buf
        .split(|c| *c == b'\n')
        .map(|s| s.strip_suffix(b"\r").unwrap_or(s))
        .skip_while(|s| s.is_empty());
    let start = lines.next()?;

    let start = std::str::from_utf8(start).ok()?;
    let mut start = start
        .split_whitespace()
        .filter(|s| !s.is_empty());
    let (Some(method), Some(path), Some(version), None) =
        (start.next(), start.next(), start.next(), start.next()) else {
        return None;
    };
    let (method, path, version) =
        (str::parse(method).ok()?, PathBuf::from(path), str::parse(version).ok()?);

    let pairs = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|line|
            std::str::from_utf8(line)
                .map_or(None, |s| s.split_once(':') )
                .map(|(k, v)| (k.to_lowercase(), v.trim().to_owned()))
        );
    let headers = pairs.collect::<Option<HashMap<_, _>>>()?;
    
    let body = lines
        .by_ref()
        .skip(1)
        .take_while(|s| !s.is_empty())
        .collect::<Vec<&[u8]>>()
        .join::<&[u8]>(b"\r\n");

    Some(HttpRequest { method, path, version, headers, body })
}

fn request_response(request: &HttpRequest) -> HttpResponse {
    if request.path == Path::new("/user-agent") {
        match request.headers.get(&"User-Agent".to_lowercase()) {
            Some(data) => HttpResponse {
                version: request.version,
                status: HttpStatus::OK,
                headers: HashMap::from([
                    ("Content-Type".to_string(), "text/plain".to_string()),
                    ("Content-Length".to_string(), data.as_bytes().len().to_string()),
                ]),
                body: data.as_bytes().to_vec(),
            },
            None => HttpResponse {
                version: request.version,
                status: HttpStatus::NOT_FOUND,
                headers: HashMap::new(),
                body: Vec::new(),
            },
        }
    } else if let Ok(Some(data)) = request.path.strip_prefix("/echo").map(|p| p.to_str()) {
        HttpResponse {
            version: request.version,
            status: HttpStatus::OK,
            headers: HashMap::from([
                ("Content-Type".to_string(), "text/plain".to_string()),
                ("Content-Length".to_string(), data.as_bytes().len().to_string()),
            ]),
            body: data.as_bytes().to_vec(),
        }
    } else if request.path == Path::new("/") {
        HttpResponse {
            version: request.version,
            status: HttpStatus::OK,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    } else {
        HttpResponse {
            version: request.version,
            status: HttpStatus::NOT_FOUND,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

fn bad_request_response() -> HttpResponse {
    HttpResponse {
        version: HttpVersion::Http1_1,
        status: HttpStatus::BAD_REQUEST,
        headers: HashMap::new(),
        body: Vec::new(),
    }
}

fn compose_response(response: &HttpResponse) -> Vec<u8> {
    let header = (response.headers.iter().fold(
        format!("{} {}\r\n", response.version, response.status), 
        |acc, (key, value)| acc + format!("{}: {}\r\n", key, value).as_str()
    ) + "\r\n").into_bytes();
    let body = if response.body.is_empty() {
        Vec::new()
    } else {
        [&response.body[..], b"\r\n\r\n"].concat()
    };

    [header, body].concat()
}