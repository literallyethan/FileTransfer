use std::net::TcpStream;

fn main() {
    let addr: &str = "127.0.0.1:8080";
    let mut stream: TcpStream = TcpStream::connect(addr)
        .expect("Could not connect!");

    
}
