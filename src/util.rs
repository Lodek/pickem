use super::tree::{Tree, LeafData};
use termion::color;

/// Converts the selected trees and lingering characters into a
/// representative string.
fn pprint_user_input(trees: &Vec<&Tree>, input_buffer: &str) -> String {
    let chords_selected = trees.iter().map(|tree| tree.data().chord.as_str()).collect::<Vec<&str>>();
    let user_input = chords_selected.join(" > ");
    format!("{} > {}", user_input, input_buffer)
}
/*

///Returns formatted string with the name of the selected trees
///separated by " > ".
fn repr_nodes(trees: &Vec<&Tree>) -> String {
    trees.iter()
        .map(|tree| tree.data().name)
        .collect::<Vec<_>>()
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
*/

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
}
