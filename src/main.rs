use termion;
use termion::color;
use termion::raw::{IntoRawMode};
use termion::input::{TermRead};
use termion::event::Key;
use std::io::{Write, stdin, stdout};
use std::io;
use std::io::Read;

use pickem::tree::Tree;
use pickem::parser;
use pickem::driver::{Driver, DriverSignal};
use pickem::util;

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

fn main() {
    let tree = load_tree();
    let mut driver = Driver::new(&tree);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut result = String::new();
    let tty = termion::get_tty().unwrap();
    let mut keys = tty.keys();

    redraw(&mut stdout, &driver).unwrap();
    stdout.flush();
    let mut keys = termion::async_stdin().keys();
    loop {
        if let Some(key) = keys.next() {
            match key.unwrap() {
                Key::Char(c) => {
                    driver.send_char(c);
                    let signal = driver.evaluate();
                    eprintln!("{:?}", result);
                    match signal {
                        DriverSignal::LeafPicked => {
                            result = driver.root().data().value.clone();
                            break
                        },
                        DriverSignal::NodePicked => (),
                        DriverSignal::DeadEnd => break,
                        DriverSignal::NoOp => (),
                        DriverSignal::Popped => ()
                    }
                },
                Key::Esc =>  break,
                _ => {
                    ()
                }
            }
            redraw(&mut stdout, &driver).unwrap();
            stdout.flush();
        }
    }
    stdout.flush();
    println!("{}", termion::clear::All);
    stdout.flush();
    stdout.suspend_raw_mode().unwrap();
    println!("{}", result);
}
