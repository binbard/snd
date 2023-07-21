use std::{
    io::{stdin, Read},
    net::{Ipv4Addr, TcpListener, TcpStream, UdpSocket},
    thread,
};

use super::chat::User;
// use super::super::handlers::streamline::listen_streamline;

#[derive(PartialEq)]
pub enum Mode {
    None,
    Room,
    Direct(String),
}

pub struct App {
    pub mode: Mode,
    pub me: User,
    pub user: Option<User>,
    pub sock_server: UdpSocket,
    pub sock_query: UdpSocket,
}

impl App {
    pub fn new(me: User, mode: Mode) -> App {
        let sock_server = UdpSocket::bind("0.0.0.0:4000").unwrap();
        sock_server
            .join_multicast_v4(&Ipv4Addr::new(224, 0, 0, 1), &Ipv4Addr::UNSPECIFIED)
            .unwrap();
        let sock_query: UdpSocket = UdpSocket::bind("0.0.0.0:4001").unwrap();
        sock_query
            .join_multicast_v4(&Ipv4Addr::new(224, 0, 0, 1), &Ipv4Addr::UNSPECIFIED)
            .unwrap();

        App {
            mode,
            me,
            user: None,
            sock_server,
            sock_query,
        }
    }
    pub fn send_multicast(&self, code: u8, msg: String) {
        let msg = format!("{}{}", code as char, msg);
        self.sock_server
            .send_to(msg.as_bytes(), "224.0.0.1:4000")
            .unwrap();
    }
    pub fn send_room(&self, mut msg: String) {
        if (self.me.username != "".to_string()) {
            msg = format!("({}): {}", self.me.username.clone(), msg);
        } else {
            msg = ": ".to_string() + &msg;
        }
        self.send_multicast(6, msg);
    }
    pub fn query(&self, code: u8, query: String, ack: bool) -> String {
        let qu = format!("{}{}", code as char, query);
        if (code == 0) {
            self.sock_query.send_to(qu.as_bytes(), "224.0.0.1:4000");
        } else {
            self.sock_query.send_to(qu.as_bytes(), "224.0.0.1:4000");
        }
        let mut result = String::new();
        if (ack == false) {
            return result;
        }

        // self.sock_query.set_read_timeout(Some(std::time::Duration::from_millis(1000)))
        //     .expect("Failed to set read timeout");

        let mut msg = [0u8; 1024];
        // let _ = self.sock_query.recv_from(&mut msg).unwrap();
        let (amt, src) = self.sock_query.recv_from(&mut msg).unwrap();
        result = String::from_utf8_lossy(&msg[1..amt]).to_string();
        result = format!("{}|{}", src.ip().to_string(), result);

        result
    }
    pub fn listen_multiline(&self) -> thread::JoinHandle<()> {
        let socket_clone = self
            .sock_server
            .try_clone()
            .expect("Failed to clone UDP Socket");

        let username = self.me.username.clone();
        let my_ip: String = self.me.uid.clone();
        let is_user = self.user.is_some();

        let th = thread::spawn(move || {
            let mut msg = [0u8; 1024];

            loop {
                match socket_clone.recv_from(&mut msg) {
                    Ok((amt, src)) => {
                        let code = msg[0];
                        let msg = String::from_utf8_lossy(&msg[1..amt]).to_string();
                        match code {
                            0 => {
                                // GET_SELF_IP
                                // println!("M] GET_SELF_IP Request from {}", src);
                                let res: String = format!("{}{}", 0 as char, src.ip().to_string());
                                socket_clone.send_to(res.as_bytes(), src).unwrap();
                            }
                            1 => {
                                println!("M] Connection Request from {} | Sent {}", src, username);
                                if is_user {
                                    // CONNECTION_REQUEST_REJECTED
                                    let res = format!("{}{}", 5 as char, username);
                                    socket_clone.send_to(res.as_bytes(), src).unwrap();
                                } else {
                                    // CONNECTION_REQUEST_ACCEPTED
                                    let res = format!("{}{}", 4 as char, username);
                                    socket_clone.send_to(res.as_bytes(), src).unwrap();
                                    // print!("STREAMING NOW");
                                }
                            }
                            6 => {
                                // ROOM_MESSAGE
                                println!("M] {}{}", src.ip().to_string(), msg);
                            }
                            9 => {
                                println!("M] Username query from {}: {}", src, msg);
                                if (msg == username
                                    || (msg.starts_with('.') && my_ip.ends_with(&msg)))
                                {
                                    println!("{} is me", msg);
                                    let res = format!("{}{}", 10 as char, username);
                                    socket_clone.send_to(res.as_bytes(), src).unwrap();
                                }
                            }
                            _ => println!("M_Unknown Message from {}: {}{}", src, code, msg),
                        }
                    }
                    Err(err) => {
                        if err.kind() == std::io::ErrorKind::ConnectionReset {
                            println!("Connection reset by peer");
                        }
                    }
                }
            }
        });
        th
    }

    pub fn listen_streamline(&self, th: thread::JoinHandle<()>) {
        let listener = TcpListener::bind("0.0.0.0:4002").unwrap();
        let (mut stream, mut src) = listener.accept().unwrap();
        let other_username = self.user.as_ref().unwrap().username.clone();
        // println!("Connected to {}", other_username);

        thread::spawn(move || {
            println!("New connection: {}", src);
            let mut buf = [0; 1024];
            loop {
                match stream.read(&mut buf) {
                    Ok(bytes_read) => {
                        let msg = String::from_utf8_lossy(&buf[..bytes_read]);
                        println!("{other_username}: {}", msg);
                    }
                    Err(e) => {
                        println!("Connection closed by the remote: {}", e);
                        break;
                    }
                }
            }    
        });
    }

    pub fn set_my_ip(&mut self) {
        // println!("Setting my IP");
        self.me.uid = self
            .query(0, "".to_string(), true)
            .splitn(2, '|')
            .last()
            .unwrap()
            .to_string();
        println!("My IP: '{}'", self.me.uid);
    }
    pub fn set_user(&mut self, uname: String) {
        let result = self.query(9, uname, true);
        let user_ip = result.splitn(2, '|').next().unwrap().to_string();
        let user_name = result.splitn(2, '|').last().unwrap().to_string();
        println!("Found user: {}", result);
        let mut user = User::new(user_name, user_ip.clone(), false);
        let dst = user_ip.clone() + &":4002".to_string();
        user.stream = Some(TcpStream::connect(dst).expect("Failed to connect to user"));
        println!("Connected to user: {}", user_ip);
        self.user = Some(user);
    }
    pub fn send_user(&self, msg: String) {
        if (self.user.is_none()) {
            println!("Could not send to user. Not connected!");
        }
        let code = 11 as u8;
        let msg = format!("{}:{}", code, msg);
        self.user
            .as_ref()
            .unwrap()
            .send(msg)
            .expect("Failed to send message to user");
    }

    pub fn run(&mut self) {
        let th = self.listen_multiline();
        self.set_my_ip();

        println!("Username: {}", self.me.username);

        match &self.mode {
            Mode::None => {
                println!("Mode: None");
                self.listen_streamline(th);
                loop {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let msg = input.pop();
                    // self.send_room(input);
                }
            }
            Mode::Room => {
                println!("Mode: Room");
                loop {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let msg = input.pop();
                    self.send_room(input);
                }
            }
            Mode::Direct(val) => {
                println!("Mode: Direct, Address: {}", val);
                self.set_user(val.clone());
                loop {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let msg = input.pop();
                    self.send_user(input);
                }
            }
        }
    }
}
