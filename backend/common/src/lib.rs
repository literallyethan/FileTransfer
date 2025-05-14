use std::io::{Error, ErrorKind, Read};
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

pub struct Message {
    pub header: u32,
    pub payload: Vec<u8>
}

impl Message {
    // reads bytes into Message Object assuming Big Endian Byte Order.
    // consider checking for a max length.
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut header_buf: [u8; 4] = [0u8; 4];
        stream.read_exact(&mut header_buf)?;
        
        // reconstruct into number
        let header_length = u32::from_be_bytes(header_buf);
        let mut msg_buf: Vec<u8> = vec![0u8; header_length.try_into().unwrap()];
        stream.read_exact(&mut msg_buf)?;

        return Ok(Self { header: header_length, payload: msg_buf });
    }
}