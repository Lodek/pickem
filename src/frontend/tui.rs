use std::io::{Result, Write, ErrorKind};
use std::fs::{OpenOptions, File};

use crate::util;
use crate::tree::Tree;
use crate::driver::{Driver, DriverSignal, DriverCommand};
use crate::frontend::View;
use crate::frontend::Controller as ControllerTrait;

use termion;
use termion::AsyncReader;
use termion::input::{TermRead, Keys};
use termion::event::Key;
use termios;
use termios::Termios;


#[derive(PartialEq)]
pub enum Flags {
    QuitDeadEnd,
    LoopMode,
    OutputOnPick,
}


// NOTE I think I finished the Controller impl.
// Run cargo check to verify for errors
pub struct Controller<'driver, 'tree, 'view> {
    driver: &'driver mut Driver<'tree>,
    views: Vec<&'view mut dyn View>,
}


impl<'driver, 'tree, 'view> Controller<'driver, 'tree, 'view> {

    pub fn new(driver: &'driver mut Driver<'tree>, views: Vec<&'view mut dyn View>) 
        -> Result<Self> {
        Ok(Self { driver, views })
    }

    /// Handles an user key press. Returns a Result of bool.
    /// If bool is false, the run is over and it should return to main
    fn handle_input(&mut self, key: Key) -> Result<bool> {
        match key {
            Key::Esc | Key::Char('\n') =>  Result::Ok(false),
            Key::Backspace => {
                let signal = self.driver.drive(DriverCommand::Backtrack);
                self.update_views(signal)
            },
            Key::Char(c) => {
                let signal = self.driver.drive(DriverCommand::Backtrack);
                self.update_views(signal)
            },
            _ => Result::Ok(true),
        }
    }

    /// Calls `update` on all views and folds `Result`s into a single
    /// Result
    fn update_views(&mut self, signal: DriverSignal) -> Result<(bool)> {
        // FIXME only the first error is preserved. Improve this to
        // maintain all `Err`s
        self.views.iter_mut()
            .map(|view| view.update(&signal))
            .fold(Ok(()),|acc, e| acc.and(e))
            .map(|_| true)
    }
}


impl<'driver, 'tree, 'view> ControllerTrait for Controller<'driver, 'tree, 'view> {
    /// Iterate over user inputs, handling each one. An `Ok(false)` means run should 
    /// return, `Ok(true)` repeats the loop and an `Error` returns.
    fn run(&mut self) -> Result<()> {
        // FIXME implement thread to implment stdin reader.
        // Use channels to communicate with reading thread
        let keys = termion::async_stdin().keys();
        for key in keys {
            match self.handle_input(key.unwrap()) {
                Result::Ok(false) => return Result::Ok(()),
                Result::Ok(true) => (),
                Result::Err(err) => return Result::Err(err),
            }
        }
        Ok(())
    }
}


/// NOTE: tty device is taken from stderr using the `/proc` directory in a linux system,
/// which is to say, if stderr is redirected, interactive elements will be a bust.
pub struct TUI<'a, 'b> {
    tty: File,
    backup_termios: Termios,
    driver: &'a Driver<'b>,
}


impl<'a, 'b> TUI<'a, 'b> {

    //This is a hack to make pickem play nice with input and output data.
    //Assuming stderr isn't redirected, fd 2 should always point to the tty itself
    //which means, it can be written to in order to redraw the terminal.
    const INTERFACE_FD: i32 = 2;

    pub fn new(driver: &'a Driver<'b>) -> Result<TUI<'a, 'b>> {
        let tty_file = format!("/dev/fd/{}", Self::INTERFACE_FD);
        let tty = OpenOptions::new().read(true).write(true).open(tty_file)?;
        let backup_termios = Termios::from_fd(Self::INTERFACE_FD)?;
        let mut view = TUI { driver, tty, backup_termios };
        view.set_cbreak_mode()?;
        Result::Ok(view)
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
        termios::tcsetattr(Self::INTERFACE_FD, termios::TCSANOW, &cbreak_termios)
    }
}


impl<'a, 'b> View for TUI<'a, 'b> {
    fn update(&mut self, signal: &DriverSignal) -> Result<()> {
        let mut transitions = self.driver
            .get_transitions()
            .into_iter()
            .map(|tree| util::pprint_choice(tree))
            .collect::<Vec<_>>();
        transitions.sort();
        let formatted_transitions = transitions.join("\n\r");

        write!(self.tty, "{}{}{}{}{}{}{}",
               termion::clear::All,
               termion::cursor::Goto(1,1),
               util::pprint_nodes(&self.driver.path()),
               termion::cursor::Goto(1,2),
               util::pprint_user_input(&self.driver.path(), &self.driver.input_buffer()),
               termion::cursor::Goto(1,4),
               formatted_transitions)
            .and_then(|_| self.tty.flush())
    }

}

impl<'a, 'b> Drop for TUI<'a, 'b> {
    /// Restore tty's termios settings
    fn drop(&mut self) -> () {
        termios::tcsetattr(Self::INTERFACE_FD, termios::TCSANOW, &self.backup_termios).unwrap();
    }
}


pub struct OutputView<'a> {
    of: File,
    driver: &'a Driver<'a>,
    format: OutputFormat,
}


pub enum OutputFormat {
    Value,
    Signal
}


impl<'a> OutputView<'a> {
    pub fn new(driver: &'a Driver<'a>, format: OutputFormat) -> Result<Self> {
        let path = "/dev/stdout";
        let of = OpenOptions::new().read(false).write(true).open(path)?;
        Ok(OutputView { of, driver, format })
    }
}

impl<'a> View for OutputView<'a> {
    /// Formats result and takes care of presenting it to user
    fn update(&mut self, signal: &DriverSignal) -> Result<()> {
        match signal {
            DriverSignal::NodePicked(tree) | DriverSignal::LeafPicked(tree) => write!(self.of, "{}", tree.data().value),
            _ => Ok(())
        }
    }
}

// TODO move the common helpers to frontend/helpers.rs

mod view_helpers {

    use termion::{color, AsyncReader};
    use crate::tree::Tree;

    /// Converts the selected trees and lingering characters into a
    /// representative string.
    pub fn pprint_user_input(trees: &Vec<&Tree>, input_buffer: &str) -> String {
        let chords_selected = trees.iter().map(|tree| tree.data().chord.as_str()).collect::<Vec<&str>>();
        let user_input = chords_selected.join(" > ");
        format!("{} > {}", user_input, input_buffer)
    }


    ///Returns formatted string with the name of the selected trees
    ///separated by " > ".
    pub fn pprint_nodes(trees: &Vec<&Tree>) -> String {
        trees.iter()
            .map(|tree| tree.data().name.as_str())
            .collect::<Vec<_>>()
            .join(" > ")
    }

    ///Returns string of a choice formatted with colors for the terminal
    pub fn pprint_choice(tree: &Tree) -> String {
        let data = tree.data();
        format!("{}{}{} - {}",
               color::Fg(color::Red),
               data.chord,
               color::Fg(color::Reset),
               data.name)
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pprint_user_input() {
            let data = LeafData {
                name: String::from("name"),
                desc: String::from("desc"),
                chord: String::from("chord"),
                value: String::from("value"),
            };
            let tree = Tree::Leaf(data);
            let trees: Vec<&Tree> = vec![&tree];
            assert_eq!(pprint_user_input(&trees, &"a"), String::from("chord > a"));
        }

        #[test]
        fn test_pprint_nodes() {
            let d1 = LeafData {
                name: String::from("root"),
                desc: String::from("root"),
                chord: String::from("root"),
                value: String::from("root"),
            };
            let d2 = LeafData {
                name: String::from("programs"),
                desc: String::from("programs"),
                chord: String::from("programs"),
                value: String::from("programs"),
            };
            let root = Tree::Leaf(d1);
            let programs = Tree::Leaf(d2);
            let trees: Vec<&Tree> = vec![&root, &programs];
            assert_eq!(pprint_nodes(&trees), String::from("root > programs"));
        }
    }

}
