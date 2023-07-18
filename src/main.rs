#![allow(unused)]

mod handlers{
    pub mod handler;
    pub mod multiline;
}

use handlers::handler::handle;

fn main() {
    handle();
}
