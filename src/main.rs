use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn parse_args() -> (String, Arc<str>) {
    let args: Vec<String> = env::args().collect();
    let mut listen = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:5678".to_string());
    let mut text = env::var("TEXT").unwrap_or_else(|_| "hello world".to_string());

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--listen" if i + 1 < args.len() => {
                listen = args[i + 1].clone();
                i += 1;
            }
            "--text" if i + 1 < args.len() => {
                text = args[i + 1].clone();
                i += 1;
            }
            _ => {
                eprintln!("Usage: --listen <addr:port> --text <response>");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    (listen, Arc::from(text))
}

fn read_http_request(mut stream: &TcpStream) -> Option<String> {
    let mut buf = Vec::new();
    let mut temp = [0u8; 512];

    loop {
        let bytes_read = match stream.read(&mut temp) {
            Ok(0) => return None,
            Ok(n) => n,
            Err(_) => return None,
        };

        buf.extend_from_slice(&temp[..bytes_read]);

        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }

        if buf.len() > 8192 {
            return None;
        }
    }

    String::from_utf8(buf).ok()
}

fn handle_client(mut stream: TcpStream, response_text: Arc<str>) {
    if let Some(request) = read_http_request(&stream) {
        let status_line;
        let body;

        if let Some(first_line) = request.lines().next() {
            if first_line.starts_with("GET / ") {
                status_line = "HTTP/1.1 200 OK";
                body = &*response_text;
            } else {
                status_line = "HTTP/1.1 404 Not Found";
                body = "Not Found";
            }

            let response = format!(
                "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{body}",
                body.len()
            );

            let _ = stream.write_all(response.as_bytes());
        }
    }
}

fn main() {
    let (listen_addr, response_text) = parse_args();

    let listener = TcpListener::bind(&listen_addr).unwrap_or_else(|e| {
        eprintln!("Failed to bind to {}: {}", listen_addr, e);
        std::process::exit(1);
    });

    println!("Listening on http://{}", listen_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let text = Arc::clone(&response_text);
                thread::spawn(move || handle_client(stream, text));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
