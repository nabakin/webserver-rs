mod status_codes;
mod thread_pool;

use thread_pool::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        println!("\nConnection established!\n");
        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("Shutting down.");
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
        _ => respond_sc(status_codes::r400())
    };
    println!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_request(location: &str) -> String {
    match location {
        "/" => respond_static("index.html"),
        _ => respond_sc(status_codes::r404())
    }
}

fn respond_static(filepath: &str) -> String {
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let payload = fs::read_to_string("static/".to_string() + filepath).unwrap();

    status_line.to_string() + &payload
}

fn respond_sc(status_code: status_codes::StatusCode) -> String {
    let status_line = "HTTP/1.1 ".to_string() + status_code.header + "\r\n\r\n";
    let mut payload = fs::read_to_string("static/client_error.html").unwrap();
    
    let message_offset = payload.find("<div id=\"status_code_message\"></div>").unwrap();
    payload.replace_range(message_offset..(message_offset+36), status_code.message);

    status_line.to_string() + &payload
}
