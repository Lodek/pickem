use termion;
use termion::color;
use termion::raw::{IntoRawMode};
use termion::input::{TermRead};
use termion::event::Key;
use std::io::{Write, stdin, stdout};
use std::io;

use pickem::tree::Tree;
/*

///Redraw function for each input
fn redraw<T: Write>(file: &mut T, trees: Vec<&Tree>, input_buffer: &String) -> io::Result<()> {
    let root = trees.last().unwrap();
    let transitions = root.transitions_by_prefix(input_buffer.as_str());
    let formatted_transitions = transitions.values()
        .map(format_choice)
        .collect::<Vec<>>()
        .join("\n\r");

    write!(file, "{}{}{}{}{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1,1),
           repr_nodes(trees),
           termion::cursor::Goto(1,2),
           repr_input(trees, input_buffer),
           termion::cursor::Goto(1,4),
           formatted_transitions)
        .and_then(|v| file.flush())
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let tree = get_tree();

    let mut picked_trees: Vec<&Tree> = Vec::new();
    let input_buffer = String::new();
    let mut result;

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char(c) => {
                input_buffer.push(c);
                match handle_input(&mut input_buffer, &mut picked_trees) {
                    PickedLeaf => {
                        result = picked_trees.last().unwrap().data().value;
                        break
                    },
                    PickedNode => redraw(&mut stdout, &mut picked_trees, &mut input_buffer),
                    InvalidPath => break,
                    NoOp => continue
                }
            },
            Key::Esc =>  break,
            _ => ()
        }
    }
}
*/

fn main() {

}
