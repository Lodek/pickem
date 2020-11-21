use termion;
use termion::color;
use termion::raw::{IntoRawMode};
use termion::input::{TermRead};
use termion::event::Key;
use std::io::{Write, stdin, stdout};
use std::io;
use std::io::Read;
use std::process;
use std::fs::OpenOptions;

use termios::Termios;
use termios;

use clap::{App, Arg};

use pickem::tree::Tree;
use pickem::parser;
use pickem::driver::{Driver, DriverSignal};
use pickem::util;
use pickem::args::Config;

///Redraw function for each input
fn redraw<T: Write>(file: &mut T, driver: &Driver) -> io::Result<()> {
    let root = driver.root();
    let transitions = root.transitions_by_prefix(driver.input_buffer.as_str());
    let formatted_transitions = transitions.values()
        .map(|tree| util::pprint_choice(*tree))
        .collect::<Vec<String>>()
        .join("\n\r");

    write!(file, "{}{}{}{}{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1,1),
           util::pprint_nodes(&driver.trees),
           termion::cursor::Goto(1,2),
           util::pprint_user_input(&driver.trees, driver.input_buffer.as_str()),
           termion::cursor::Goto(1,4),
           formatted_transitions)
        .and_then(|v| file.flush())
}


fn main() {

    let config = Config::from_env();
    let data = config.raw_yaml().unwrap();
    let tree = parser::parse(data.as_str());
    let mut driver = Driver::new(&tree);


    let pid = process::id();
    let tty_file = format!("/proc/{}/fd/2", pid);
    let mut tty = OpenOptions::new().read(true).write(true).open(tty_file).unwrap();

    let backup_termios = Termios::from_fd(2).unwrap();
    let mut raw_termios = Termios::from_fd(2).unwrap();
    termios::cfmakeraw(&mut raw_termios);
    termios::tcsetattr(2, termios::TCSANOW, &raw_termios).unwrap();



    redraw(&mut tty, &driver).unwrap();
    tty.flush();
    let mut keys = termion::async_stdin().keys();
    loop {
        if let Some(key) = keys.next() {
            match key.unwrap() {
                Key::Char('\n') => break,
                Key::Char(c) => {
                    driver.send_char(c);
                    let signal = driver.evaluate();
                    match signal {
                        DriverSignal::LeafPicked => {
                            break
                        },
                        DriverSignal::NodePicked => (),
                        DriverSignal::DeadEnd => break,
                        DriverSignal::NoOp => (),
                        DriverSignal::Popped => ()
                    }
                },
                Key::Esc =>  break,
                _ => ()
                
            }
            redraw(&mut tty, &driver).unwrap();
            tty.flush();
        }
    }
    tty.flush();
    write!(tty, "{}{}", termion::clear::All,
           termion::cursor::Goto(1,1));
    tty.flush();
    termios::tcsetattr(2, termios::TCSANOW, &backup_termios).unwrap();
    let result: String = driver.trees.iter()
        .map(|tree| tree.data().value.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    println!("{}", result.as_str());
}
