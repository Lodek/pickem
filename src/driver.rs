use super::tree::Tree;


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
