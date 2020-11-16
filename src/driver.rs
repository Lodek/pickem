use super::tree::Tree;

/*

///Driver creates an abstraction that handles the "by char" nature of terminal stdin.
///This simplifies navigating through the tree by driving a new state for each character
///the user inputs.
pub struct Driver {
    trees: Vec<&Tree>,
    input_buffer: String
}


///Enumerate possible results for driver
pub enum DriverSignal {
    NoOp,
    NodePicked(&Tree),
    LeafPicked(&Tree),
    DeadEnd
}

impl Driver {

    ///Sends a new character to driver
    pub fn send_char(&self, c: char) {

    }

    ///Evaluates the current state based on the input_buffer and returns a
    ///signal indicating the result of the evaluation.
    pub fn evaluate(&self) -> DriverSignal {

    }
}
*/
/*
fn handle_input(input_buffer: &mut String,
                picked_trees: &mut Vec<Tree>) -> HandleResult {
    let root = picked_trees.last().unwrap();
    match root.transition(input_buffer.as_str()) {
        Option::Some(tree) => {
            input_buffer.clear();
            picked_trees.push(tree);
            match tree {
                Tree::Leaf(_) => HandleResult::PickedLeaf,
                Tree::Node(_, _) => HandleResult::PickedNode
            }
        },
        Option::None => {
            if root.transitions_by_prefix(input_buffer).len() == 0 {
                return HandleResult::InvalidPath;
            }
            return HandleResult::NoOp;
        }
    }
}
*/
