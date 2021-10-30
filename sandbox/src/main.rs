use std::fs::read_to_string;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listen = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listen.incoming() {
        let stream = stream.unwrap();
        connect(stream);
        println!("cool")
    }
}

fn connect(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1";
    let tidur = b"GET /tidur HTTP/1.1";
    let (filename, status) = if buffer.starts_with(get) {
        ("index.html", "HTTP/1.1 200 OK")
    } else if buffer.starts_with(tidur) {
        thread::sleep(Duration::from_secs(5)); //bakal loading 5 detik
        ("index.html", "HTTP/1.1 200 OK")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND")
    };

    let html = read_to_string(filename).unwrap();

    let respon = format!(
        "{}\r\nContent-Length: {}\r\n\r\n {}",
        status,
        html.len(),
        html
    );

    stream.write(respon.as_bytes()).unwrap();
    stream.flush().unwrap();
}
