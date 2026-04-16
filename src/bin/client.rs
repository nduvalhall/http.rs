use std::{io::Write, net::TcpStream};

fn send_request(message: &str) {
    let addr = "127.0.0.1:42069";
    let mut stream = TcpStream::connect(addr).expect(&format!("Failed to connect to {addr}"));

    stream
        .write(message.as_bytes())
        .expect("Failed to write get request");

    stream
        .shutdown(std::net::Shutdown::Both)
        .expect("Failed to shutdown stream");
}

fn main() {
    let message = "GET /hello-world HTTP/1.1
Host: example.com
User-Agent: curl/8.0
Accept: */*
Connection: close\r\n\r\n";

    send_request(message);

    let message = "GET /hello-world2 HTTP/1.1
Host: example.com
User-Agent: curl/8.0
Accept: */*
Connection: close\r\n\r\n";

    send_request(message);
}
