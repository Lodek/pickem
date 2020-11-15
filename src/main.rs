use termion;
use termion::color;
use termion::raw::{IntoRawMode};
use termion::input::{TermRead};
use termion::event::Key;
use std::io::{Write, stdin, stdout};
use std::io;

use self::tree::Tree;


/// Converts the selected trees and lingering characters into a
/// representative string.
fn repr_input(trees: &Vec<&Tree>, input_buffer: &String) -> String {
    let chords_selected = trees.iter().map(|tree| tree.data().chord).collect::<Vec<>>();
    let user_input = chords_selected.join(" > ");
    format!("{} > {}", user_input, input_buffer)
}

///Returns formatted string with the name of the selected trees
///separated by " > ".
fn repr_nodes(trees: &Vec<&Tree>) -> String {
    trees.iter()
        .map(|tree| tree.data().name)
        .collect::<Vec<>>()
        .join(" > ")
}

///Returns string of a choice formatted with colors for the terminal
fn format_choice(tree: &Tree) -> String {
    let data = tree.data();
    format!("{}{}{} - {}",
           color::Fg(color::Red),
           data.chord,
           color::Fg(color::Reset),
           data.name)
}


///Redraw function for each input
fn redraw<T: Write>(file: &mut T, trees: &Vec<&Tree>, input_buffer: &String) -> io::Result<()> {
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

fn get_tree() -> Tree {

}

fn handle_input(input_buffer: &mut String,
                picked_trees: &mut Vec<Tree>) -> io::Result<()>{
    let root = picked_trees.last().unwrap();
    match root.transition(input_buffer.as_str()) {
        Option::Some(tree) => {
            input_buffer = String::from("");
            picked_trees.push(tree);
            io::Result::Ok(())
        },
        Option::None => {
            if root.transitions_by_prefix(input_buffer).len() == 0 {
                io::Result::Err(())
            }
            io::Result::Ok(())
        }
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let tree = get_tree();

    let mut picked_trees: Vec<&Tree> = Vec::new();
    let input_buffer = String::new();

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char(c) => {
                input_buffer.push(c);
                match handle_input(&mut input_buffer, &mut picked_trees) {
                    io::Result::Ok(_) => {
                        redraw(&mut stdout, &picked_trees, &input_buffer);
                        continue
                    },
                    io::Result::Err(_) => break
                }
            },
            Key::Esc => break,
            _ => ()
        }
    }
}
