use std::sync::{LazyLock, Mutex, Arc};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Client {
    pub stream: Arc<TcpStream>, // Arc is like a shared pointer
    pub addr: SocketAddr,
    pub id: u32,
    pub name: String,
}

impl Client {
    pub fn new(stream: Arc<TcpStream>, addr: SocketAddr, id: u32, name: String) -> Self {
       return Self {stream, addr, id, name};
    }
}