/// Converts the selected trees and lingering characters into a
/// representative string.
fn repr_input(trees: &Vec<&Tree>, input_buffer: &String) -> String {
    let chords_selected = trees.iter().map(|tree| tree.data().chord.as_str()).collect::<Vec<&str>>();
    let user_input = chords_selected.join(" > ");
    format!("{} > {}", user_input, input_buffer)
}

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

