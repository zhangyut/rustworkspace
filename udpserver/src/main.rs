extern crate futures;
extern crate tokio;

use std::net::ToSocketAddrs;

use futures::Future;
use tokio::net::TcpStream;
use std::{thread, time};

fn main() {
    let addr = "127.0.0.1:8080".to_socket_addrs().unwrap().next().unwrap();
    let response = TcpStream::connect(&addr)
        .and_then(|socket| tokio::io::write_all(socket, "Hello World".as_bytes()))
        .and_then(|(socket, _)| tokio::io::read_exact(socket, vec![0; 11]))
        .map(|(_, data)| println!("get data {:?}", data))
        .map_err(|e| println!("get error {}", e));

    tokio::run(response);

    let ten_millis = time::Duration::from_millis(1000);

    thread::sleep(ten_millis);
}
