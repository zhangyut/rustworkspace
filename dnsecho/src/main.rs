//! An UDP echo server that just sends back everything that it receives.
//!
//! If you're on Unix you can test this out by in one terminal executing:
//!
//!     cargo run --example echo-udp
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect -- --udp 127.0.0.1:8080
//!
//! Each line you type in to the `nc` terminal should be echo'd back to you!

#![deny(warnings, rust_2018_idioms)]

use futures::try_ready;
use std::net::SocketAddr;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::{env, io};
use tokio;
use tokio::net::UdpSocket;
use tokio::prelude::*;
//extern crate r53;
use r53::{SRV};
use r53::util::InputBuffer;
use r53::util::hex::from_hex;

enum ServerStatus {
    WaitQuery,
    RecvQuery,
    SendQuery,
    RecvResponse,
}

struct Server {
    socket: UdpSocket,
    query_socket: UdpSocket,
    query_buf: Vec<u8>,
    response_buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    to_query: Option<(usize, SocketAddr)>,
    to_response: Option<SocketAddr>,
    status: ServerStatus,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            match self.status {
                ServerStatus::WaitQuery => {
                    println!("1 wait query");
                    self.to_send = Some(try_ready!(self.socket.poll_recv_from(&mut self.query_buf)));
                    self.status = ServerStatus::RecvQuery;
                },
                ServerStatus::RecvQuery => {
                    println!("2 recv query");
                    let (size, peer) = self.to_send.unwrap();
                    self.to_response = Some(peer);
                    let dns_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8,8,8,8)),53);
                    try_ready!(self.query_socket.poll_send_to(&self.query_buf[..size], &dns_addr));
                    self.status = ServerStatus::SendQuery;
                    println!("3 send query");
                },
                ServerStatus::SendQuery => {
                    self.to_query = Some(try_ready!(self.query_socket.poll_recv_from(&mut self.response_buf)));
                    println!("4 receive response");
                    self.status = ServerStatus::RecvResponse;
                },
                ServerStatus::RecvResponse => {
                    println!("5 send response");
                    self.status = ServerStatus::WaitQuery;
                    let (size, _peer) = self.to_query.unwrap();
                    try_ready!(self.socket.poll_send_to(&self.response_buf[..size], &self.to_response.unwrap()));
                    self.to_query = None;
                    self.to_response = None;
                    self.to_send = None;
                },
            };
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)),0);
    let query_socket = UdpSocket::bind(&local_addr)?;

    let socket = UdpSocket::bind(&addr)?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket: socket,
        query_socket: query_socket,
        query_buf: vec![0; 1024],
        response_buf: vec![0; 1024],
        to_send: None,
        to_query: None,
        to_response: None,
        status:ServerStatus::WaitQuery,
    };

    // This starts the server task.
    //
    // `map_err` handles the error by logging it and maps the future to a type
    // that can be spawned.
    //
    // `tokio::run` spawns the task on the Tokio runtime and starts running.
    //12 10 53 www.baidu.com.
    let raw = from_hex("000c000a00350377777705626169647503636f6d00").unwrap();
    let mut buf = InputBuffer::new(raw.as_slice());
    let srv = SRV::from_wire(&mut buf, raw.len() as u16).unwrap();
    println!("{:?}",srv);
    tokio::run(server.map_err(|e| println!("server error = {:?}", e)));
    Ok(())
}
