use super::tree::Tree;

#[derive(PartialEq, Debug)]
pub enum DriverFlag {
    LeafToggle,
}

#[derive(Debug)]
pub enum DriverCommand<'a> {
    Backtrack,
    Transition(&'a str)
}

/// Specified a change in driver's internal state
#[derive(Debug)]
pub enum DriverSignal<'a> {
    NoOp,
    NodePicked(&'a Tree),
    LeafPicked(&'a Tree),
    LeafUnpicked(&'a Tree),
    DeadEnd,
    Popped
}

/// Driver allows statefully traversing through a tree.
pub struct Driver<'a> {
    root: Tree,
    flags: Vec<DriverFlag>,

    /// Stores all selected nodes/leafs from tree
    selections: Vec<&'a Tree>,

    /// Stores the current path in the tree
    path: Vec<&'a Tree>,

    input_buffer: String,

}

impl<'a> Driver<'a> {

    /// Returns new Driver
    pub fn new(root: Tree, flags: Vec<DriverFlag>) -> Driver<'a> {
        Driver {
            root: root,
            flags: flags,
            input_buffer: String::new(),
            path: Vec::new(),
            selections: Vec::new()
        }
    }

    pub fn default(root: Tree) ->  Driver<'a> {
        Self::new(root, Vec::new())
    }

    /// Receives a command which changes the driver's current state
    pub fn drive<'b>(&'a mut self, command: DriverCommand<'b>) -> DriverSignal<'a> {
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

    fn transition<'b>(&'a mut self, input: &'b str) -> DriverSignal<'a> {
        let mut result: DriverSignal = DriverSignal::NoOp;
        for c in String::from(input).chars() {
            result = self.evaluate_char(c)
        }
        result
        // only return the last transition? feels wrong.
        // overall i don't like this method... what to do?
    }

    fn toggle(&self) -> bool {
        self.flags.contains(&DriverFlag::LeafToggle)
    }

    /// Add node to list of selections and update path.
    /// If picked value is a leaf, the behavior depends on 
    /// whether the toggle flag is active or not.
    fn handle_pick(&'a mut self, tree: &'a Tree) -> DriverSignal<'a> {
        if let Tree::Node(_, _) = tree {
            self.selections.push(tree);
            self.path.push(tree);
            DriverSignal::NodePicked(tree)
        }
        else if self.toggle() && self.selections.contains(&tree) {
            self.selections = self.selections.into_iter()
                .filter(|t| *t != tree)
                .collect::<Vec<_>>();
            DriverSignal::LeafUnpicked(tree)
        }
        else {
            self.selections.push(tree);
            DriverSignal::LeafPicked(tree)
        }
    }

    /// Handle a partial transition
    fn handle_incomplete_transition(&mut self) -> DriverSignal {
        if self.root.transitions_by_prefix(self.input_buffer.as_str()).len() == 0 {
            self.input_buffer.clear();
            DriverSignal::DeadEnd
        }
        else {
            DriverSignal::NoOp
        }
    }

    fn evaluate_char(&'a mut self, c: char) -> DriverSignal<'a> {
        self.input_buffer.push(c);
        match self.root().transition(&self.input_buffer[..]) {
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

    fn build_tree() -> Tree {
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
    fn test_public_api() {
        let tree = build_tree();
        let driver = Driver::default(tree);
        assert_eq!(driver.drive(DriverCommand::Backtrack), DriverSignal::NoOp);
        assert_eq!(driver.drive(DriverCommand::Transition("n")), DriverSignal::NoOp);
        assert_eq!(driver.drive(DriverCommand::Transition("1")), DriverSignal::NodePicked(_));
        assert_eq!(driver.drive(DriverCommand::Transition("l")), DriverSignal::LeafPicked(_));
        assert_eq!(driver.drive(DriverCommand::Transition("k")), DriverSignal::DeadEnd(_));
        assert_eq!(driver.drive(DriverCommand::Backtrack), DriverSignal::Backtrack);
    }

}
