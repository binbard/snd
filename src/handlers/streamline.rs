use std::io::{stdin, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::models::chat::User;

pub fn streamline_connect(ip: String) -> TcpStream {
    let stream = TcpStream::connect(ip).unwrap();
    stream
}

pub fn streamline_listen() {
    let listener = TcpListener::bind("0.0.0.0:4001").unwrap();
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    let mut buf;
                    loop {
                        buf = [0; 1024];
                        let bytes_read = stream.read(&mut buf).unwrap();
                        let msg = String::from_utf8_lossy(&buf[..bytes_read]);
                        println!("Received: {}", msg);
                    }
                }
                Err(e) => {
                    println!("Error connection closed: {}", e);
                }
            }
        }
    });
}

pub fn local_direct_chat(me: User, connect_to: String) -> thread::JoinHandle<()> {
    let mut stream = streamline_connect(connect_to);

    let th = thread::spawn(move || loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let msg = input.pop();
        stream.write(input.as_bytes()).unwrap();
    });
    th
}
