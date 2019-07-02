//use futures::Future;
//use futures::future::ok;
//use std::error::Error;
use futures::executor;

fn main() {
    println!("Hello, world!");
    let a = executor::spawn(10);
    println!("{:?}", a.get_ref())
}

