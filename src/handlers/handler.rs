use crate::handlers::multiline::{local_query, local_room_chat, multiline_listen};
use crate::handlers::streamline::{streamline_connect, streamline_listen, local_direct_chat};
use crate::models::chat::User;

use std::hash::Hash;
use std::io::{stdin, Read};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn handle(myname: String, connect_to: String) {
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    let socket_clone = socket.try_clone().expect("Failed to clone socket");

    let user: User = User::new(myname.clone(), "192.168.29.248".to_string(), true);
    let th_local_chat = local_room_chat(&socket, myname);

    // local_room_listen(&socket)
    // query_listen(&socket);
    let result = local_query(9, "binbard".to_string(), true);
    let connect_to = result.splitn(2,'|').next().unwrap().to_string();
    println!("Result: '{}'", connect_to);
    streamline_listen();
    let th = local_direct_chat(user, connect_to);
    th.join().unwrap();

    // th_local_chat.join().unwrap();
}

// fn get_user(client: String) -> User{
// }

pub fn handle_streamline(client: String) {
    // HashMap<String, User> users;
    // let mut user = get_user(client);
    // users.insert(user.uid, user);

    let listener = streamline_listen();
    let mut stream = streamline_connect(client);
    let mut buf;
    loop {
        buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).unwrap();
        let msg = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received: {}", msg);
    }
}
