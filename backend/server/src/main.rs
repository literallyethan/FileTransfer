use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::{Write, BufWriter};
use std::sync::{LazyLock, Mutex, Arc};
use std::collections::HashMap;

struct Client {
    stream: Arc<TcpStream>, // Arc is like a shared pointer
    addr: SocketAddr,
    id: u32,
    name: String,
}

impl Client {
    fn new(stream: Arc<TcpStream>, addr: SocketAddr, id: u32, name: String) -> Self {
       return Self {stream, addr, id, name};
    }
}

// lazylock allows list to be sync-safe (I think, never used before)
static CLIENT_MAP: LazyLock<Mutex<HashMap<u32, Client>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

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
        // vector of byte arrays (&[u8])
        let parts: Vec<&[u8]> = vec![prefix.as_bytes(), addr_string.as_bytes()];
        let msg_bytes: Vec<u8> = parts.concat();
    
        // ignore output with _
        let _ = stream.write_all(&msg_bytes)
            .expect("Failed to write!");

        println!("Sent bytes!");
    }

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

    let mut writer = BufWriter::new(arc_stream.as_ref());
    writer.write_all(msg.as_bytes()).expect("Write failed");
    writer.flush().expect("Flush failed");

    /*
        job of handler:
        -accept a client
            -place client in list of available peers
        -provide a list of connectable peers (:check:)
        -recieve client's peer choice
        -connect peers
    */



}
