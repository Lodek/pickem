use super::tree::Tree;


///Driver creates an abstraction that handles the "by char" nature of terminal stdin.
///This simplifies navigating through the tree by driving a new state for each character
///the user inputs.
pub struct Driver<'a> {
    trees: Vec<&'a Tree>,
    input_buffer: String
}


///Enumerate possible results for driver
pub enum DriverSignal {
    NoOp,
    NodePicked,
    LeafPicked,
    DeadEnd
}

impl<'a> Driver<'a> {

    ///Sends a new character to driver
    pub fn send_char(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    ///Returns new Driver with `root` as the first picked tree.
    pub fn new(root: &Tree) -> Driver {
        Driver {
            trees: vec![root],
            input_buffer: String::new()
        }
    }

    ///Returns current root for driver
    fn root(&self) -> &Tree {
        self.trees.last().unwrap()
    }

    ///Evaluates the current state based on the input_buffer and returns a
    ///signal indicating the result of the evaluation.
    pub fn evaluate(&mut self) -> DriverSignal {
        let root = self.trees.last().unwrap();
        match root.transition(self.input_buffer.as_str()) {
            Option::Some(tree) => {
                self.input_buffer.clear();
                self.trees.push(tree);
                match tree {
                    Tree::Leaf(_) => DriverSignal::LeafPicked,
                    Tree::Node(_, _) => DriverSignal::NodePicked
                }
            },
            Option::None => {
                if root.transitions_by_prefix(self.input_buffer.as_str()).len() == 0 {
                    DriverSignal::DeadEnd
                }
                else {
                    DriverSignal::NoOp
                }
            }
        }
    }
}
