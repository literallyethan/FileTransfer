use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::Write;
use std::sync::LazyLock;

struct Client {
    stream: TcpStream,
    addr: SocketAddr,
    id: u32,
    name: String,
}

// lazylock allows list to be sync-safe (I think, never used before)
static CLIENT_LIST: LazyLock<Vec<Client>> = LazyLock::new(|| vec![]);

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

fn handle_client(mut stream: TcpStream, addr: SocketAddr) {
    // tells client their info
    // in scope to free before handling loop
    {
        let addr_string: String = addr.to_string();
        let prefix: &str = "Your info: ";
        // the vec macro is awesome!
        let parts: Vec<&[u8]> = vec![prefix.as_bytes(), addr_string.as_bytes()];
        let msg_bytes: Vec<u8> = parts.concat();
    
        // ignore output with _
        let _ = stream.write_all(&msg_bytes)
            .expect("Failed to write!");

        println!("Sent bytes!");
    }

    /*
        job of handler:
        -accept a client
            -place client in list of available peers
        -provide a list of connectable peers
        -recieve client's peer choice
        -connect peers
    */



}
