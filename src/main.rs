#![allow(unused)]

mod handlers{
    pub mod handler;
    pub mod multiline;
    pub mod streamline;
}
mod models{
    pub mod chat;
}

use handlers::handler::handle as handler;

use std::env;

fn main() {
    handler("notok".to_string(), "binbard".to_string());
}
