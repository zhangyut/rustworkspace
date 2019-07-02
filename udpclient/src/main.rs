use std::net::UdpSocket;
use std::thread;
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:22222").unwrap();
    let buf = [1u8; 60000];
    let mut count = 1;
    loop {
        socket.send_to(&buf[0..count], "127.0.0.1:8888").unwrap();
        count = count + 1;
        if count == 2 {
            break;
        }
    }

    thread::sleep_ms(1000);
}
