use super::driver::{Driver, DriverSignal};
use super::util;
use super::tree::Tree;

use std::io::{Result, Write, ErrorKind};
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
    LoopMode,
    LeafToggle,
}

// TODO Think of a better way to handle different print modes
// Is a enum really the best way?
pub enum PrintMode {
    FullPathValue,
    SingleValue,
    ParentChild,
}


pub struct Controller {
    driver: Driver,
    view: View,
    keys: Keys<AsyncReader>,
}

impl Controller {

    pub fn new(driver: Driver, view: View) -> Result<Controller> {
        Ok(
            Self {
            driver: driver,
            view: view,
            keys: termion::async_stdin().keys(),
        })
    }

    fn handle_input(&self, key: Key) -> Result<bool> {
        match key {
            Key::Backspace => {
                let signal = self.driver.drive(DriverCommand::Backtrack);
                self.view.update(signal).map(|_| true)
            },
            Key::Char(c) => {
                let signal = self.driver.drive(DriverCommand::Backtrack);
                self.view.update(signal).map(|_| true)
            },
            Key::Esc | Key::Char('\n') =>  Result::Ok(false),
            _ => Result::Ok(true),
        }
    }

    /// Iterate over user inputs, handling each one. An `Ok(false)` means run should 
    /// return, `Ok(true)` repeats the loop and an `Error` returns.
    pub fn run(&mut self) -> Result<()> {
        for key in self.keys {
            match self.handle_input(key.unwrap()) {
                Result::Ok(false) => return Result::Ok(()),
                Result::Ok(true) => (),
                err @ Result::Err(_) => return err,
            }
        }
    }
}


// TODO Move some of the helpers to a helper module for TUI  view
// Maybe a view module with common functionalities + trait?
// Perhaps a TUI module?

/// NOTE: tty device is taken from stderr using the `/proc` directory in a linux system,
/// which is to say, if stderr is redirected, interactive elements will be a bust.
struct View<'a> {
    tty: File,
    backup_termios: Termios,
    driver: &'a Driver
}

impl<'a> View<'a> {

    //This is a hack to make pickem play nice with input and output data.
    //Assuming stderr isn't redirected, fd 2 should always point to the tty itself
    //which means, it can be written to in order to redraw the terminal.
    const INTERFACE_FD: i32 = 2;

    pub fn new(driver: &Driver ) -> View {
        let tty_file = format!("/dev/fd/{}", Self::INTERFACE_FD);
        let tty = OpenOptions::new().read(true).write(true).open(tty_file)?;
        let backup_termios = Termios::from_fd(Self::INTERFACE_FD)?;
        let view = View {
            driver: driver,
            tty: tty,
            backup_termios: backup_termios,
        };
        view.set_cbreak_mode();
        return view;
    }

    pub fn update(&self, signal: DriverSignal) -> Result<()> {

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
        let mut transitions = self.driver
            .get_transitions()
            .into_iter()
            .map(|tree| util::pprint_choice(tree))
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

impl<'a> Drop for View<'a> {

    /// Restore tty's termios settings
    pub fn drop(&mut self) -> {
        termios::tcsetattr(Self::INTERFACE_FD, termios::TCSANOW, &self.backup_termios)?;
    }
}
