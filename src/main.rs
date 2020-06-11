use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("\nConnection established!\n");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let buffer_str = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", buffer_str);

    let mut buffer_str_iter = buffer_str.split(" ");
    let method = buffer_str_iter.next().unwrap();
    let location = buffer_str_iter.next().unwrap();

    let response = match method {
        "GET" => get_request(location),
        _ => respond_400()
    };
    println!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_request(location: &str) -> String {
    match location {
        "/" => respond_static("index.html"),
        _ => respond_404()
    }
}

fn respond_static(filepath: &str) -> String {
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let payload = fs::read_to_string("static/".to_string() + filepath).unwrap();

    status_line.to_string() + &payload
}

fn respond_400() -> String {
    let status_line = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
    let payload = fs::read_to_string("static/responses/400.html").unwrap();

    status_line.to_string() + &payload
}

fn respond_404() -> String {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let payload = fs::read_to_string("static/responses/404.html").unwrap();

    status_line.to_string() + &payload
}
