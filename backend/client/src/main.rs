use std::net::TcpStream;
use std::io::Read;

fn main() {
    let addr: &str = "127.0.0.1:8080";
    let mut stream: TcpStream = TcpStream::connect(addr)
        .expect("Could not connect!");

    // have a buffer to read from the stream.
    // convert that buffer to string.
    // print the string.

    let mut buf: Vec<u8> = Vec::new();
    let _ = stream.read_to_end(&mut buf)
        .expect("Failed to read!");

    // print the bytes as string
    println!("{:?}", std::str::from_utf8(&buf).unwrap());
}
