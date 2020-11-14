use termion;
use termion::color;
use termion::raw::{IntoRawMode};
use termion::input::{TermRead};
use termion::event::Key;
use std::io::{Write, stdin, stdout};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (x_max, y_max) = termion::terminal_size().unwrap();

    write!(stdout, "{}{} Pickem{}Yayay{}", 
           termion::clear::All, 
           termion::cursor::Goto(1,1),
           termion::cursor::Goto(1,2),
           termion::cursor::Goto(1,3))
        .unwrap();
    stdout.flush().unwrap();
    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break
            },
            Key::Char('?') => {
                let x_margin = 2*(x_max / 10);
                let y_margin = 2*(y_max / 10);
                write!(stdout, "{}a{}b{}c{}d",
                       termion::cursor::Goto(x_margin,y_margin),
                       termion::cursor::Goto(x_max - x_margin, y_margin),
                       termion::cursor::Goto(x_margin, y_max - y_margin),
                       termion::cursor::Goto(x_max - x_margin, y_max - y_margin))
                    .unwrap();
            }
            Key::Char(c) => {
                write!(stdout, "{}", c).unwrap();
                stdout.flush().unwrap();
            }
            _ => ()
        }
    }
}
