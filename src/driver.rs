use super::tree::Tree;


///Driver creates an abstraction that handles the "by char" nature of terminal stdin.
///This simplifies navigating through the tree by driving a new state for each character
///the user inputs.
pub struct Driver<'a> {
    trees: Vec<&'a Tree>,
    input_buffer: String
}


///Enumerate possible results for driver
#[derive(PartialEq, Debug)]
pub enum DriverSignal {
    NoOp,
    NodePicked,
    LeafPicked,
    DeadEnd,
    Popped
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
        //TODO Evaluate only works if it's called after every `send_char` invocation. Fix that.
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

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tree::LeafData;

    fn get_tree() -> Tree {
        let leaf_data = LeafData{
            name: String::from("leaf"),
            desc: String::from("leaf"),
            chord: String::from("l"),
            value: String::from("leaf")
        };
        let leaf = Tree::Leaf(leaf_data);
        let n1_data = LeafData {
            name: String::from("n1"),
            desc: String::from("n1"),
            chord: String::from("n1"),
            value: String::from("n1")
        };
        let n1 = Tree::Node(n1_data, vec![leaf]);

        let n2_data = LeafData {
            name: String::from("n2"),
            desc: String::from("n2"),
            chord: String::from("n2"),
            value: String::from("n2")
        };
        let n2 = Tree::Node(n2_data, Vec::new());

        let root_data = LeafData  {
            name: String::from("root"),
            desc: String::from("root"),
            chord: String::from(""),
            value: String::from("")
        };

        let root = Tree::Node(root_data, vec![n1, n2]);
        return root;
    }

    fn test_evaluate_signals() {
        let tree = get_tree();
        let mut driver = Driver::new(&tree);
        driver.send_char('n');
        driver.send_char('1');
        assert_eq!(driver.evaluate(), DriverSignal::NodePicked);
        driver.send_char('l');
        assert_eq!(driver.evaluate(), DriverSignal::LeafPicked);
        assert_eq!(driver.evaluate(), DriverSignal::DeadEnd);
    }

}
