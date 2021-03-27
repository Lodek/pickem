use super::tree::Tree;

#[derive(PartialEq, Debug)]
pub enum DriverFlag {
    LeafToggle,
}

#[derive(PartialEq, Debug)]
pub enum DriverCommand<'a> {
    Backtrack,
    Transition(&'a str)
}

/// Specified a change in driver's internal state
#[derive(PartialEq, Debug)]
pub enum DriverSignal<'a> {
    NoOp,
    NodePicked(&'a Tree),
    LeafPicked(&'a Tree),
    DeadEnd,
    Popped
}

/// Driver allows statefully traversing through a tree.
pub struct Driver<'a> {
    root: Tree,
    flags: Vec<DriverFlag>

    /// Stores all selected nodes/leafs from tree
    selections: Vec<&'a Tree>,

    /// Stores the current path in the tree
    path: Vec<&'a Tree>,

    input_buffer: String,

}

impl<'a> Driver<'a> {

    /// Returns new Driver
    pub fn new(root: Tree, flags: Vec<DriverFlag>) -> Driver {
        Driver {
            root: root,
            input_buffer: String::new(),
            path: Vec::new(),
            selections: Vec<&'a Tree>,
        }
    }

    /// Receives a command which changes the driver's current state
    pub fn drive<'a, 'b>(&'a mut self, command: DriverCommand<'b>) -> DriverSignal<'a> {
        match command {
            DriverCommand::Backtrack => self.backtrack(),
            DriverCommand::Transition(input) => self.transition(input),
        }
    }

    /// Walks up a level in the tree and clears input buffer
    fn backtrack(&mut self) -> DriverSignal {
        self.input_buffer.clear();
        match self.path.pop() {
            Some(tree) => DriverSignal::Popped,
            None => DriverSignal::NoOp,
        }
    }

    fn transition(&mut self, input: &str) -> DriverSignal {
        // don't let those fools send me an empty string
        // only return the last transition? the returning everything 
        // feels wrong.
        input.iter().map(evaluate_char).collect::Vec<_>().pop()?
    }

    /// Add `tree` to selections and return the according value
    fn handle_pick(&mut self, tree: &Tree) -> DriverSignal {
        self.selections.push(tree);
        if let Tree::Node(_, _) = tree {
            self.path.push(tree);
            DriverSignal::NodePicked(tree)
        }
        else {
            DriverSignal::LeafPicked(tree)
        }
    }

    /// Handle a partial transition
    fn handle_incomplete_transition(&mut self) => DriverSignal {
        if self.root.transitions_by_prefix(self.transition_buffer.as_str()).len() == 0 {
            self.transition_buffer.clear();
            DriverSignal::DeadEnd
        }
        else {
            DriverSignal::NoOp
        }
    }

    fn evaluate_char(&mut self, c: char) -> DriverSignal {
        self.input_buffer.push(c);
        match self.root.transition(self.transition_buffer.as_str()) {
            Option::Some(tree) => self.handle_pick(tree),
            Option::None => self.handle_incomplete_transition()
        }
    }

    /// Returns reference to root
    pub fn root(&self) -> &Tree {
        &self.root
    }

    /// Gets last selected node or returns root
    pub fn head(&self) -> &Tree {
        self.path.pop().unwrap_or(&self.root)
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

    #[test]
    fn test_evaluate_signals() {
        let tree = get_tree();
        let mut driver = Driver::new(&tree);
        driver.send_char('n');
        assert_eq!(driver.evaluate(), DriverSignal::NoOp);
        driver.send_char('1');
        assert_eq!(driver.evaluate(), DriverSignal::NodePicked);
        driver.send_char('l');
        assert_eq!(driver.evaluate(), DriverSignal::LeafPicked);
        assert_eq!(driver.evaluate(), DriverSignal::NoOp);
        driver.send_char('j');
        assert_eq!(driver.evaluate(), DriverSignal::DeadEnd);
    }

}
