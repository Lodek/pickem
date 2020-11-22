use super::driver::{Driver, DriverSignal};
use super::util;
use super::tree::Tree;


use std::io::{Result, Write};
use std::fs::{OpenOptions, File};
use std::{io, process};

use termion;
use termion::AsyncReader;
use termion::input::{TermRead, Keys};
use termion::event::Key;

use termios::Termios;
use termios;

pub struct CliDriver<'a> {
    driver: Driver<'a>,
    tty: File,
    keys: Keys<AsyncReader>,
    backup_termios: Termios
}

impl<'a> CliDriver<'a> {


    ///Initializes and configures input stream, output stream and tty for driver.
    ///
    ///NOTE: tty device is taken from stderr using the `/proc` directory in a linux system,
    ///which is to say, if stderr is redirected, interactive elements will be a bust.
    pub fn new(tree: &'a Tree) -> Result<CliDriver<'a>> {
        let pid = process::id();
        let tty_file = format!("/proc/{}/fd/2", pid);
        let tty = OpenOptions::new()
            .read(true)
            .write(true)
            .open(tty_file)?;
        let backup_termios = Termios::from_fd(2)?;
        let mut raw_termios = Termios::from_fd(2)?;
        termios::cfmakeraw(&mut raw_termios);
        termios::tcsetattr(2, termios::TCSANOW, &raw_termios)?;
        let driver = CliDriver {
            driver: Driver::new(tree),
            keys: termion::async_stdin().keys(),
            tty: tty,
            backup_termios: backup_termios
        };
        Ok(driver)
    }


    ///Formats result and takes care of presenting it to user
    pub fn present_result(&mut self) -> Result<()> {
        self.reset_screen()?;
        let result: String = self.driver.trees.iter()
            .map(|tree| tree.data().value.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}", result.as_str());
        Ok(())
    }

    ///Iterator that returns DriverSignals
    fn signals() {

    }

    fn reset_screen(&mut self) -> Result<()> {
        write!(self.tty, "{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1))
            .and_then(|v| self.tty.flush())
    }

    ///Routine that restores tty before exit
    pub fn cleanup(&mut self) -> Result<()> {
        self.reset_screen()?;
        termios::tcsetattr(2, termios::TCSANOW, &self.backup_termios)?;
        Ok(())
    }

    ///Updates the screen based on the current state of driver
    fn redraw(&mut self) -> Result<()> {
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

    ///Awaits user inputs until an break condition occurs or an exit signal is received
    pub fn run(&mut self) -> Result<()>{
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
