use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {

    let l = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000...");

    for stream in l.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        
        let mut b = [0; 1024];

        stream.read(&mut b).unwrap();
        stream.write(&mut b).unwrap();
    }
}
