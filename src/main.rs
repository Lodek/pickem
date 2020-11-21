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


fn load_tree() -> Tree {
    let mut stdin = stdin();
    let mut yml_str = String::new();
    stdin.read_to_string(&mut yml_str);
    parser::parse(yml_str.as_str());
    let _yml_str =
"
foo:
  .chord: f
  .value: foo
  baz:
    .value: baz
    .chord: z
bar:
  .chord: b
  .value: bar
";
    parser::parse(yml_str.as_str())
}

fn parse_args() -> Config {
    let m = App::new("Pickem")
        .version("0.1")
        .author("Bruno G. <gomes.bruno.ac@gmail.com>")
        .about("Command line selection tool")
        .arg(Arg::with_name("INPUT")
             .help("Set input yaml file. Leave blank to read from stdin.")
             .required(false)
             .default_value("")
             .index(1))
        .arg(Arg::with_name("result_mode")
             .short("r")
             .long("result")
             .takes_value(true)
             .help("Sets how result is going to be computed")
             .possible_values(&["last", "leaves", "all"])
             .default_value("last"))
        .arg(Arg::with_name("break_condition")
             .short("b")
             .long("break")
             .takes_value(true)
             .help("Configures when pickem is going to exit.")
             .possible_values(&["dead_end", "first_leaf"])
             .default_value("dead_end"))
        .get_matches();
    Config::new(m)
}

fn main() {

    let config = parse_args();


    let pid = process::id();
    let tty_file = format!("/proc/{}/fd/2", pid);
    let mut tty = OpenOptions::new().read(true).write(true).open(tty_file).unwrap();

    let backup_termios = Termios::from_fd(2).unwrap();
    let mut raw_termios = Termios::from_fd(2).unwrap();
    termios::cfmakeraw(&mut raw_termios);
    termios::tcsetattr(2, termios::TCSANOW, &raw_termios).unwrap();

    let tree = load_tree();
    let mut driver = Driver::new(&tree);


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
