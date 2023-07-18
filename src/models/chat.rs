use std::collections::HashSet;
use std::net::{TcpListener, TcpStream};
use std::io::Error;

pub struct User {
    pub uid: String,
    pub addr: String,
    pub is_me: bool,
    pub stream: Option<TcpStream>,
    pub chat: Option<ChatInfo>,
}

impl User {
    fn new(uid: String, addr: String, is_me: bool) -> User {
        let mut user = User {
            uid,
            addr,
            is_me,
            stream: None,
            chat: None,
        };
        user
    }
    fn connect(&mut self, user: User) -> Result<(), Error>{
        if(self.chat.is_some()) {
            println!("Already connected to {}", self.stream.unwrap().peer_addr()?);
            return Ok(());
        }
        let stream = TcpStream::connect(user.addr)?;
        let mut chat = ChatInfo {
            messages: Vec::new(Message::Direct),
            msg_count: 0,
            unread_count: 0,
        };
        self.chat = Some(chat);
        Ok(())
    }
    // fn listen(&self) {
    //     let mut buf = [0u8; 1024];
    //     let mut stream = self.stream.unwrap();
    //     loop {
    //         let (amt, src) = stream.read(&mut buf).unwrap();
    //         let msg: String = String::from_utf8_lossy(&buf[..amt]).to_string();
    //         let sender = src;
    //         println!("{}: {}", sender, msg);
    //     }
    // }
    fn send(&self, text: String) -> Result<(), Error>{
        if(self.stream.is_none()) {
            println!("No connection");
            return;
        }
        self.stream.write(text.as_bytes())?;
        Ok(())
    }
    fn sendDirect(&self, text: String) -> Result<(), Error>{
        let code = 11 as u8;
        let msg = format!("{}:{}", code, text);
        self.send(msg)?;
        let message = Message::Direct(DirectMsg::new(text));
        self.chat.messages.push(message);
        self.chat.msg_count += 1;
        self.chat.unread_count += 1;
        Ok(())
    }
}

pub struct ChatInfo {
    messages: Vec<Message>,
    msg_count: u32,
    unread_count: u32,
}

pub struct RoomMsg {
    sender: User,
    text: String,
}

impl RoomMsg {
    fn new(sender: User, text: String) -> RoomMsg {
        RoomMsg { sender, text }
    }
}

pub struct DirectMsg {
    text: String,
}

impl DirectMsg {
    fn new(text: String) -> DirectMsg {
        DirectMsg { text }
    }
}

pub enum Message {
    Direct(DirectMsg),
    Room(RoomMsg),
}

pub struct Group {
    pub jid: String,
    pub owner: User,
    pub participants: HashSet<User>,
    pub is_me: bool,
    pub chat: Option<ChatInfo>,
}

impl Group {
    fn new(jid: String, owner: User) -> Group {
        let group = Group {
            jid,
            owner,
            participants: HashSet::new(),
            chat: None,
        };
        group
    }
    fn is_mine(&self) -> bool {
        self.owner.is_me
    }
    fn add(&mut self, participant: User) {
        self.participants.insert(participant);
    }
    fn remove(&mut self, participant: User) {
        self.participants.remove(participant);
    }
    fn join(&mut self, owner: User) {
        let mut chat = ChatInfo {
            messages: Vec::new(Message::Room),
            msg_count: 0,
            unread_count: 0,
        };
        self.chat = Some(chat);
    }
}

pub enum Chat {
    Direct(User),
    Room(Group),
}
