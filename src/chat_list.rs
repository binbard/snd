use crossterm::{
    cursor::{self, MoveTo},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType}
};

use std::io::stdout;

fn main(){
    execute!(stdout(),Clear(ClearType::All)).unwrap();
}