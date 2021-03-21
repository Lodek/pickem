use super::driver::{Driver, DriverSignal};
use super::util;
use super::tree::Tree;

use std::io::{Result, Write};
use std::fs::{OpenOptions, File};

use termion;
use termion::AsyncReader;
use termion::input::{TermRead, Keys};
use termion::event::Key;

use termios::Termios;
use termios;


#[derive(PartialEq)]
pub enum Flags {
    QuitDeadEnd,
    OmmitNodeValues,
    OneShot
}


pub struct Controller {
    driver: Driver,
    view: View,
    keys: Keys<AsyncReader>,
}

impl Controller {

    pub fn new(tree: Tree) -> Result<Controller> {
        Ok(
            Self {
            driver: Driver::new(tree),
            view: View(),
            keys: termion::async_stdin().keys(),
        })
    }

    /// Awaits user inputs until an break condition occurs or an exit signal is received
    pub fn run(&mut self) -> Result<()> {
        self.redraw()?;
        loop {
            if let Some(key) = self.keys.next() {
                match key.unwrap() {
                    Key::Char('\n') => break,
                    Key::Char(c) => {
                        self.driver.send_char(c);
                        let signal = self.driver.evaluate();
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
                self.redraw()?;
            }
        }
        Ok(())
    }
}



/// NOTE: tty device is taken from stderr using the `/proc` directory in a linux system,
/// which is to say, if stderr is redirected, interactive elements will be a bust.
struct View {
    tty: File,
    backup_termios: Termios,
}

impl View {

    //This is a hack to make pickem play nice with input and output data.
    //Assuming stderr isn't redirected, fd 2 should always point to the tty itself
    //which means, it can be written to in order to redraw the terminal.
    const INTERFACE_FD: i32 = 2;

    pub fn new() -> View {
        let tty_file = format!("/dev/fd/{}", INTERFACE_FD);
        let tty = OpenOptions::new().read(true).write(true).open(tty_file)?;
        let backup_termios = Termios::from_fd(Self::INTERFACE_FD)?;
        let view = View {
            tty: tty,
            backup_termios: backup_termios,
        }
        view.set_cbreak_mode();
        return view;
    }

    /// Sets the tty given by `fd` into cbreak_mode
    fn set_cbreak_mode(&mut self) -> Result<()> {
        let cbreak_flags = termios::ICANON | termios::ECHO | termios::ECHOE 
            | termios::ECHOK | termios::IEXTEN;
        let mut cbreak_termios = Termios::from_fd(Self::INTERFACE_FD)?;
        cbreak_termios.c_lflag &= !cbreak_flags;
        cbreak_termios.c_lflag |= termios::ISIG;
        cbreak_termios.c_oflag &= !termios::OPOST;
        cbreak_termios.c_cc[termios::VMIN] = 1;
        cbreak_termios.c_cc[termios::VTIME] = 0;
        termios::tcsetattr(fd, termios::TCSANOW, &cbreak_termios)
    }

    pub fn redraw(&mut self) -> Result<()> {
        let root = self.driver.root();
        let transitions = root.transitions_by_prefix(self.driver.input_buffer.as_str());
        let mut transitions = transitions.values()
            .map(|tree| util::pprint_choice(*tree))
            .collect::<Vec<String>>();
        transitions.sort();
        let formatted_transitions = transitions.join("\n\r");

        write!(self.tty, "{}{}{}{}{}{}{}",
               termion::clear::All,
               termion::cursor::Goto(1,1),
               util::pprint_nodes(&self.driver.trees),
               termion::cursor::Goto(1,2),
               util::pprint_user_input(&self.driver.trees, &self.driver.input_buffer.as_str()),
               termion::cursor::Goto(1,4),
               formatted_transitions)
            .and_then(|v| self.tty.flush())
    }

    pub fn clean(&mut self) -> Result<()> {
        write!(self.tty, "{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1))
            .and_then(|v| self.tty.flush())
    }

    /// Routine that restores tty before exit
    pub fn cleanup(&mut self) -> Result<()> {
        self.reset_screen()?;
        termios::tcsetattr(INTERFACE_FD, termios::TCSANOW, &self.backup_termios)
    }

    /// Formats result and takes care of presenting it to user
    pub fn present_result(&mut self) -> Result<()> {
        self.reset_screen()?;
        let result: String = self.driver.trees.iter()
            .map(|tree| tree.data().value.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}", result.as_str());
        Ok(())
    }
}
