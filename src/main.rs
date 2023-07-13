#![allow(unused)]

use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();

    socket.set_broadcast(true).unwrap(); // Enable broadcasting on the socket

    let socket_clone = socket.try_clone().expect("Failed to clone socket");

    let mut msg = [0u8; 1024];

    let stdin = std::io::stdin();

    let th = thread::spawn(move || {
        loop {
            let (amt, src) = socket.recv_from(&mut msg).unwrap();
            let msg: String = String::from_utf8_lossy(&msg[..amt]).to_string();
            let sender = src;
            println!("{}: {}", sender, msg);
        }
    });

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let msg = input.pop();
        socket_clone.send_to(input.as_bytes(), "192.168.1.255:4000").unwrap(); // Use broadcast address
    }

    th.join().unwrap();
}
