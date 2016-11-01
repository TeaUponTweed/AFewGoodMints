extern crate termion;

use termion::event::*;
use termion::cursor;
use termion::input::{MouseTerminal};
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdin = async_stdin().bytes();

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    writeln!(stdout,
         "{}{}q to exit. Type stuff, use alt, click around...",
         termion::clear::All,
         termion::cursor::Goto(1, 1)).unwrap();
    loop {
        if let Some(b) = stdin.next(){
            if let Ok(evt) = parse_event(b, &mut stdin) {
                match evt {
                    Event::Key(Key::Char('q')) => break,
                    Event::Mouse(me) => {
                        match me {
                            MouseEvent::Press(_, a, b) |
                            MouseEvent::Release(a, b) |
                            MouseEvent::Hold(a, b) => {
                                write!(stdout, "o{}", cursor::Goto(a, b)).unwrap();
                            }
                        }
                    }
                    _ => {}
                }
                stdout.flush().unwrap();                
            }
            thread::sleep(Duration::from_millis(5));
        }
        // print!("{}", 2*2);
    }
    writeln!(stdout,
     "{}{}.",
     termion::clear::All,
     termion::cursor::Goto(1, 1)).unwrap();
}
