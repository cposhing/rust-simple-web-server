use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut _s = TcpStream::connect("localhost:3000").unwrap();
    _s.write("Hello".as_bytes()).unwrap();

    let mut b = [0; 5]; // hello from server
    _s.read(&mut b).unwrap();

    println!(
        "Response form server:{:?}",
        str::from_utf8(&b).unwrap()
    )
}
