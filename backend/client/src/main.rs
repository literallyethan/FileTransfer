//use core::time;
use std::net::TcpStream;
use std::io::Read;
use dotenv::dotenv;
use std::env;
use common::Client;
use common::Message;
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    XChaCha20Poly1305,
    Key,
    XNonce,
};

fn main() {
    dotenv().ok(); // load .env into environment
    let bind_addr: String = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("looking on {}", bind_addr);
    let mut stream: TcpStream = TcpStream::connect(bind_addr)
        .expect("Could not connect!");


    // this will happen once authentication goes through.
    loop {
        let msg: Message = Message::from_tcp_stream(&mut stream)
            .expect("Failed to read message.");

        let printable: String = msg.payload_to_string()
            .expect("Invalid UTF8!");

        if printable == "Done!" {
            // server is closing
            break;
        }
        
        println!("{}", printable);
        /* 
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("Server closed connection.");
                break;
            }
            Ok(n) => {
                let text = std::str::from_utf8(&buf[..n]).unwrap_or("[Invalid UTF-8]");
                println!("Received: {}", text);


                /* 
                // means server is ready to bind client to another
                if text.contains("Ok!/") {
                    handle_peer(text.to_string());
                    break;
                }
                */
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
        */
    }
}

//before you can do this, the server needs to be forwarded so you can get public ip.
//TODO: Add authentication so you can safely expose the server port.
fn handle_peer(text: String) {

}
