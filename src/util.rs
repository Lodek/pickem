use super::tree::{Tree, LeafData};
use termion::color;

/// Converts the selected trees and lingering characters into a
/// representative string.
fn pprint_user_input(trees: &Vec<&Tree>, input_buffer: &str) -> String {
    let chords_selected = trees.iter().map(|tree| tree.data().chord.as_str()).collect::<Vec<&str>>();
    let user_input = chords_selected.join(" > ");
    format!("{} > {}", user_input, input_buffer)
}


///Returns formatted string with the name of the selected trees
///separated by " > ".
fn pprint_nodes(trees: &Vec<&Tree>) -> String {
    trees.iter()
        .map(|tree| tree.data().name.as_str())
        .collect::<Vec<_>>()
        .join(" > ")
}

/*
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
