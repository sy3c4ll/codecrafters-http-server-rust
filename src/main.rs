mod encoding;
mod http_request;
mod http_response;
mod http_version;

use http_request::HttpRequest;
use http_response::{HttpResponse, http_status::HttpStatus};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("[#] Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        eprintln!("[*] incoming traffic detected");
        std::thread::spawn(|| match (|| handle_client(stream?))() {
            Ok(status) => eprintln!("[+] transmission complete with status {status}"),
            Err(e) => eprintln!("[!] error during transmission: {e}"),
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<HttpStatus> {
    let mut buf = [0u8; 0x1000];
    let len = stream.read(&mut buf)?;

    let request = HttpRequest::parse_request(&buf[..len]);
    let response = request.as_ref().map_or_else(HttpResponse::bad_request_response, HttpResponse::request_response);
    let buf = response.compose_response();

    stream.write_all(&buf[..])?;
    Ok(response.status)
}
