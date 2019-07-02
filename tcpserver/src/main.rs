use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{self, Read};
use std::thread;
extern crate clap;
use clap::{Arg, App};

fn handle_connection(stream: TcpStream) {
    let mut buf = vec![];
    let mut stream_clone = stream.try_clone().expect("clone failed...");
    loop {
        match stream_clone.read_to_end(&mut buf) {
            Ok(_) => {
                println!("receive msg: {:?}", String::from_utf8(buf).unwrap());
                break;
            },
            Err(e) => panic!("encountered IO error 222: {}", e),
        };
    };
}
fn main() {
    let matches = App::new("tcpserver")
                          .arg(Arg::with_name("ip")
                               .short("s")
                               .long("server")
                               .help("server ip")
                               .takes_value(true))
                          .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .help("server port")
                               .takes_value(true))
                          .get_matches();
    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8080");
    //let mut server = String::new();
    /*
    server.push_str(ip);
    server.push(':');
    server.push_str(port);
    */
    let server:String = ip.to_string()+":" +port;

    let listener = TcpListener::bind(server.as_str()).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
        
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_connection(s);
                });
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();
                continue;
            }
            Err(e) => panic!("encountered IO error 111: {}", e),
        };
    }
}
