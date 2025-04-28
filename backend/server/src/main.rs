use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io;

fn main() {
    // .expect() deals with the result instead of a pattern match
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")
        .expect("Could not bind!");

    loop {
        // with our listener, we constantly listen

        match listener.accept() {
            // use ok to parse the tuple
            Ok((stream, addr)) => {
                // move keyword gives the variables to the handler
                // so the handler can own them
                thread::spawn(move || handle_client(stream, addr));
            }

            Err(e) => {
                eprintln!("Failed to accept: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, addr: SocketAddr) {
    println!("Connected, yo!");
}
