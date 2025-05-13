//use core::time;
use std::net::TcpStream;
use std::io::Read;

fn main() {
    let addr: String = std::env::var("BIND_ADDR").unwrap_or("127.0.0.1:8080".to_string());
    let mut stream: TcpStream = TcpStream::connect(addr)
        .expect("Could not connect!");

    // have a buffer to read from the stream.
    // convert that buffer to string.
    // print the string.

    // read() reads bytes as soon as recieved
    let mut buf = [0u8; 1024];
    //let _ = stream.read(&mut buf)
    //    .expect("Failed to read!");

    // print the bytes as string
    //println!("{:?}", std::str::from_utf8(&buf).unwrap());

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("Server closed connection.");
                break;
            }
            Ok(n) => {
                let text = std::str::from_utf8(&buf[..n]).unwrap_or("[Invalid UTF-8]");
                println!("Received: {}", text);
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

