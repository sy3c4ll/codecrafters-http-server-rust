mod http_request;
mod http_response;
mod http_version;

use http_request::{HttpRequest, http_method::HttpMethod};
use http_response::{HttpResponse, http_status::HttpStatus};
use http_version::HttpVersion;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("[#] Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                eprintln!("[+] accepted new connection");
                match handle_client(stream) {
                    Ok(()) => {
                        eprintln!("[+] transaction complete");
                    }
                    Err(e) => {
                        eprintln!("[!] error while responding: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("[!] error while connecting: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 0x1000];
    let len = stream.read(&mut buf)?;

    let request = parse_request(&buf[..len]);
    let response = match request {
        Ok(ref request) => request_response(&request),
        Err(_) => HttpResponse { version: HttpVersion::Http1_1, status: HttpStatus::NOT_FOUND },
    };
    let buf = compose_response(&response);

    stream.write_all(&buf[..])?;
    request.map(|_| ())
}

fn parse_request(buf: &[u8]) -> io::Result<HttpRequest> {
    let mut lines = buf
        .split(|c| *c == b'\n')
        .map(|s| s.strip_suffix(b"\r").unwrap_or(s))
        .filter(|s| !s.is_empty());
    let Some(start) = lines.next() else {
        return io::Result::Err(io::Error::from(io::ErrorKind::InvalidData));
    };

    let mut start = start.
        split(|c| c.is_ascii_whitespace()).
        filter(|s| !s.is_empty());
    let (Some(method), Some(path), Some(version), None) =
        (start.next(), start.next(), start.next(), start.next()) else {
        return io::Result::Err(io::Error::from(io::ErrorKind::InvalidData));
    };
    let (Some(method), path, Some(version)) =
        (HttpMethod::from_bytes(method), PathBuf::from(OsStr::from_bytes(path)), HttpVersion::from_bytes(version)) else {
        return io::Result::Err(io::Error::from(io::ErrorKind::InvalidData));
    };

    let pairs = lines.map(|line| {
        let sep = line.iter().take_while(|c| **c != b':').count();
        if sep < line.len() {
            Some((line[..sep].to_ascii_lowercase(), trim_ascii(&line[sep + 1..]).to_owned()))
        } else {
            None
        }
    });
    let Some(headers) = pairs.collect::<Option<HashMap<_, _>>>() else {
        return io::Result::Err(io::Error::from(io::ErrorKind::InvalidData));
    };

    Ok(HttpRequest { method, path, version, headers })
}

fn request_response(request: &HttpRequest) -> HttpResponse {
    let version = request.version;
    let status = match request.path == Path::new("/") {
        true => HttpStatus::OK,
        false => HttpStatus::NOT_FOUND,
    };

    HttpResponse { version, status }
}

fn compose_response(response: &HttpResponse) -> Vec<u8> {
    format!("{} {}\r\n\r\n", response.version, response.status).into_bytes()
}

fn trim_ascii(s: &[u8]) -> &[u8] {
    let begin = s.iter().take_while(|c| c.is_ascii_whitespace()).count();
    let end = s.len() - s.iter().rev().take_while(|c| c.is_ascii_whitespace()).count();
    &s[begin..end]
}
