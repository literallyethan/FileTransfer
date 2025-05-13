//use core::time;
use std::net::TcpStream;
use std::io::Read;
use dotenv::dotenv;
use std::env;
use common::Client;

fn main() {
    dotenv().ok(); // load .env into environment
    let bind_addr: String = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("looking on {}", bind_addr);
    let mut stream: TcpStream = TcpStream::connect(bind_addr)
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

                // means server is ready to bind client to another
                if text.contains("Ok!/") {
                    handle_peer(text.to_string());
                    break;
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

//before you can do this, the server needs to be forwarded so you can get public ip.
//TODO: Add authentication so you can safely expose the server port.
fn handle_peer(text: String) {

}
