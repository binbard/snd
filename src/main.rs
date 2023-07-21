#![allow(unused)]

mod handlers{
    pub mod handler;
    pub mod multiline;
    pub mod streamline;
}
mod models{
    pub mod chat;
    pub mod mode;
}

use handlers::handler::handle as handler;
use models::{mode::{Mode, App}, chat::User};

use std::env;



fn main() {
    // handler("notok".to_string(), "binbard".to_string());

    let args: Vec<String> = env::args().collect();
    
    let mut username = "".to_string();
    let mut mode = Mode::None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "room" => {
                mode = Mode::Room;
            }
            s if s.starts_with('@') || s.starts_with('.') || s.starts_with('1') => {
                if(s.starts_with('@') || s.starts_with('.')) {
                    username = args[i].clone();
                    username.remove(0);
                } else {
                    username = args[i].clone();
                }
            }
            "--username" | "-u" => {
                if i + 1 < args.len() {
                    username = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => {
                // println!("Invalid argument: {}", args[i]);
                mode = Mode::None;
            }
        }
        i += 1;
    }

    let me = User::new(username.clone(), "".to_string(), true);

    let mut app = App::new(me, mode);
    app.run();

    // handler(mode, username);

    // match mode {
    //     Mode::None => handler(mode: Mode::None, username, "".to_string()),
    //     Mode::Room => handler()
    //     Mode::Direct(address) => println!("Mode: Direct, Address: {}", address),
    // }
    

}
