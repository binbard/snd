use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{self, MoveTo}, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

fn main()  {

    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

    execute!(
        stdout(),
        MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print("Initial content"),
        ResetColor,
    ).unwrap();

    execute!(stdout(), MoveTo(0, 1)).unwrap();

    println!("Hello");

    let mut counter = 0u32;

    loop {
        thread::sleep(Duration::from_secs(1));
        counter+=1;

        execute!(stdout(), MoveTo(0, 0)).unwrap();
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();

        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("Updated content {counter}")),
            ResetColor,
        ).unwrap();

        // Flush the output to make sure it's immediately visible
        stdout().flush().unwrap();
    }   

}
