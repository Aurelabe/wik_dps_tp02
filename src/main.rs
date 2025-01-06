use std::io::{Read, Write};
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; 
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        if request.starts_with("GET /ping ") {
            if let Some(headers_start) = request.find("\r\n") {
                let headers = &request[headers_start + 2..];
                let headers_end = headers.find("\r\n\r\n").unwrap_or(headers.len()); 
                let extracted_headers = &headers[..headers_end]; 

                // Créer la réponse JSON avec Content-Length et Content-Type
                let response_body = format!("{{\"headers\": {:?}}}", extracted_headers);
                let content_length = response_body.len();

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    content_length, response_body
                );

                // Envoyer la réponse complète
                stream.write_all(response.as_bytes()).unwrap();
            }
        } else {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let port = std::env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8001".to_string());
    let address = format!("0.0.0.0:{}", port);

    let listener = std::net::TcpListener::bind(&address).unwrap();
    println!("Server is running on {}", address);

    // Écouter les connexions
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Failed to establish connection: {}", e),
        }
    }
}
