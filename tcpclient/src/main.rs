use std::io::prelude::*;
use std::net::TcpStream;
extern crate clap;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
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
    let server:String = ip.to_string()+":" +port;
    let mut stream = TcpStream::connect(server.as_str())?;

    stream.write(b"hello world")?;
    Ok(())
}
