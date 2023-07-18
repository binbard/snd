use crate::handlers::multiline::{multiline_listen, multiline_send};

use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use std::io::stdin;


pub fn handle(){
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();

    let th = multiline_listen(&socket);
    
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let msg = input.pop();
        multiline_send(4, input, &socket);
    }
}