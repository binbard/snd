use std::collections::HashSet;
use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read, Write};
use std::hash::{Hash,Hasher};
use std::ops::Deref;
use std::rc::Rc;

pub struct User {
    pub username: String,
    pub uid: String,
    pub is_me: bool,
    pub stream: Option<TcpStream>,
    pub chat: Option<ChatInfo>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.uid == other.uid
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state);
        self.uid.hash(state);
    }
}

impl User {
    pub fn new(username: String, uid: String, is_me: bool) -> User {
        let mut user = User {
            username,
            uid,
            is_me,
            stream: None,
            chat: None,
        };
        user
    }
    pub fn connect(&mut self, user: Rc<User>) -> Result<(), Error>{
        if(self.chat.is_some()) {
            return Ok(());
        }
        let stream: TcpStream = TcpStream::connect(&user.uid)?;
        let mut chat: ChatInfo = ChatInfo {
            messages: Vec::new(),
            msg_count: 0,
            unread_count: 0,
        };
        self.chat = Some(chat);
        Ok(())
    }
    pub fn send(&self, text: String) -> Result<(), Error>{
        if(self.stream.is_none()) {
            Error::new(std::io::ErrorKind::NotConnected, "No connection to user!");
        }
        self.stream.as_ref().unwrap().write(text.as_bytes())?;
        Ok(())
    }
    pub fn send_msg(&mut self, text: String) -> Result<(), Error>{
        let code = 11 as u8;
        let msg = format!("{}:{}", code, text);
        self.send(msg)?;
        // let user = Rc::new(self.my_ref());
        let message = Message::new(self.uid.clone(), text);
        self.chat.as_mut().unwrap().messages.push(message);
        self.chat.as_mut().unwrap().msg_count += 1;
        Ok(())
    }
}

pub struct ChatInfo {
    messages: Vec<Message>,
    msg_count: u32,
    unread_count: u32,
}

pub struct Message {
    sender: String,
    text: String,
}

impl Message {
    fn new(sender: String, text: String) -> Message {
        let message = Message {
            sender,
            text,
        };
        message
    }
}

pub struct Group {
    pub jid: String,
    pub owner: User,
    pub participants: HashSet<User>,
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
    fn remove(&mut self, participant: &User) {
        self.participants.remove(participant);
    }
    fn join(&mut self, owner: User) {
        let mut chat = ChatInfo {
            messages: Vec::new(),
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
