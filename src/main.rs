// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                match stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
                    Ok(len) => {
                        println!("sent {len} bytes");
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
