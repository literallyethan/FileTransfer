use std::io::{Error, ErrorKind, Read, Write};
use std::sync::{LazyLock, Mutex, Arc};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::string::FromUtf8Error;

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
    // Endianness doesnt matter for strings, only multi-byte numbers
    pub header: u32,
    pub payload: Vec<u8>
}

impl Message {
    // reads bytes into Message Object assuming Big Endian Byte Order.
    // consider checking for a max length.
    pub fn from_tcp_stream(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut header_buf: [u8; 4] = [0u8; 4];
        stream.read_exact(&mut header_buf)?;
        
        // reconstruct into number
        let header_length = u32::from_be_bytes(header_buf);
        let mut msg_buf: Vec<u8> = vec![0u8; header_length.try_into().unwrap()];
        stream.read_exact(&mut msg_buf)?;

        return Ok(Self { header: header_length, payload: msg_buf });
    }

    pub fn from_string(string: String) -> Self {
        let header_num : u32 = string.len() as u32;
        let vec_buf: Vec<u8> = string.into_bytes();
        
        return Self {header: header_num, payload: vec_buf};
    }

    pub fn payload_to_string(&self) -> Result<String, FromUtf8Error> {
        let string: Result<String, FromUtf8Error> = String::from_utf8(self.payload.clone());
        return string;
    }

    pub fn to_tcp_stream(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let header_bytes: [u8; 4] = self.header.to_be_bytes();
        
        stream.write_all(&header_bytes)?;
        stream.write_all(&self.payload)?;

        return Ok(());
    }
}