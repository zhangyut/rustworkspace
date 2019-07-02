extern crate futures;
extern crate tokio; 
use std::net::ToSocketAddrs;
use futures::Future;
use tokio::net::TcpStream;

fn main() {
    let addr = "127.0.0.1:8080".to_socket_addrs().unwrap().next().unwrap();
    let socket = TcpStream::connect(&addr);
    let request = socket
        .and_then(|socket| tokio::io::write_all(socket, b"Hello World"));
    let response = request
        .and_then(|(socket, _)| tokio::io::read_exact(socket, Vec::new()))
        .map(|(_, data)| println!("get data {:?}", data))
        .map_err(|e| println!("get error {}", e));

    tokio::run(response);
}
