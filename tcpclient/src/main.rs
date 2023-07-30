use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write(b"Hello, world!").unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Response from server: {}", String::from_utf8_lossy(&buffer));
}
