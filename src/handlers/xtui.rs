use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{self, MoveTo}, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

pub fn load_cli()  {

    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

        execute!(
            stdout(),
            MoveTo(0, 0),
            SetForegroundColor(Color::Yellow),
            Print("snd cli chat"),
            ResetColor,
        ).unwrap();

        execute!(stdout(), MoveTo(0, 1)).unwrap();
        println!("loading..");
        thread::sleep(Duration::from_secs(1));
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();

    execute!(
        stdout(),
        MoveTo(0, 0),
        SetForegroundColor(Color::Green),
        Print("snd cli chat"),
        MoveTo(0, 1),
        ResetColor,
    ).unwrap();

}
