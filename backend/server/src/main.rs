use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::{BufWriter, Read, Write};
use std::sync::{LazyLock, Mutex, Arc};
use std::collections::HashMap;
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

// lazylock allows list to be sync-safe (I think, never used before)
// TODO: refactor to use SocketAddr instead of u32
static CLIENT_MAP: LazyLock<Mutex<HashMap<u32, Client>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn main() {
    // .expect() deals with the result instead of a pattern match
    dotenv().ok(); // load .env into environment
    let bind_addr: String = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let listener: TcpListener = TcpListener::bind(bind_addr)
        .expect("Could not bind!");

    loop {
        // with our listener, we constantly listen

        match listener.accept() {
            // use ok to parse the tuple
            Ok((stream, addr)) => {
                // move keyword gives the variables to the handler
                // so the handler can own them
                thread::spawn(move || authenticate_client(stream, addr));
            }

            Err(e) => {
                eprintln!("Failed to accept: {}", e);
            }
        }
    }
}

fn authenticate_client(mut stream: TcpStream, addr: SocketAddr)
{
    /* 
    let mut buf = [0u8; 1024];
    match stream.read(&mut buf) {
        Ok(0) => {
            println!("Client disconnected.");
        }
        Ok(n) => {
            let text = std::str::from_utf8(&buf[..n]).unwrap_or("[Invalid UTF-8]");
            //TODO: This would be auth info. parse and decrypt to auth
            
        }
        Err(e) => {
            eprintln!("Failed to authenticate: {}", e);
        }
    }
    */
    handle_client(stream, addr);
}

fn handle_client(mut stream: TcpStream, addr: SocketAddr) {
    // tells client their info
    // in scope to free before handling loop
    {
        let addr_string: String = addr.to_string();
        let msg_string: String = "Your info: ".to_string();
        let total_string: String = msg_string + &addr_string;


        let msg: Message = Message::from_string(total_string);
        msg.to_tcp_stream(&mut stream)
            .expect("Failed to write in handle_client");

        println!("Sent bytes!");
    }
/* 
    let arc_stream: Arc<TcpStream> = Arc::new(stream);
    let id: u32 = 0;
    let name: &str = "client0";
    let client: Client = Client::new(arc_stream.clone(), addr, id, name.to_string());
    let mut map = CLIENT_MAP.lock()
        .expect("Could not lock map!");
    
    map.insert(client.id, client);

    println!("Sending client list info: ");

    let mut msg: String = String::from("List of connected clients: \n");
    for (id, client) in map.iter() {
        msg.push_str(&format!("Id: {}, Name: {}\n", id, &client.name));
    }

    let mut writer: BufWriter<&TcpStream> = BufWriter::new(arc_stream.as_ref());
    writer.write_all(msg.as_bytes()).expect("Write failed");
    writer.flush().expect("Flush failed");
*/
}