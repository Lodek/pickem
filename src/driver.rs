use super::tree::Tree;


#[derive(PartialEq, Debug, Clone)]
pub enum DriverFlag {
}


#[derive(Debug)]
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
    LeafUnpicked(&'a Tree),
    DeadEnd(String),
    Popped
}


/// Driver allows statefully traversing through a tree.
#[derive(Clone)]
pub struct Driver<'a> {
    root: &'a Tree,
    flags: Vec<DriverFlag>,

    /// Stores all selected nodes/leafs from tree
    selections: Vec<&'a Tree>,

    /// Stores the current path in the tree
    path: Vec<&'a Tree>,

    input_buffer: String,
}


impl<'a> Driver<'a> {

    /// Returns new Driver
    pub fn new(root: &'a Tree, flags: Vec<DriverFlag>) -> Self {
        Self {
            root: root,
            flags: flags,
            input_buffer: String::new(),
            path: Vec::new(),
            selections: Vec::new()
        }
    }

    pub fn default(root: &'a Tree) ->  Self {
        Self::new(root, Vec::new())
    }

    pub fn path<'b>(&'b self) -> &'b Vec<&'a Tree> {
        &self.path
    }

    pub fn input_buffer(&self) -> &str {
        self.input_buffer.as_str()
    }

    /// Returns reference to root
    pub fn root(&self) -> &'a Tree {
        &self.root
    }

    /// Gets last selected node or returns root
    pub fn head(&self) -> &'a Tree {
        *self.path.last().unwrap_or(&&self.root)
    }

    pub fn get_transitions(&self) -> Vec<&'a Tree> {
        self.head()
            .transitions_by_prefix(self.input_buffer.as_str())
            .iter()
            .map(|(key, value)| *value)
            .collect()
    }

    /// Receives a command which changes the driver's current state
    pub fn drive<'b>(&mut self, command: DriverCommand<'b>) -> DriverSignal<'a> {
        match command {
            DriverCommand::Backtrack => self.backtrack(),
            DriverCommand::Transition(input) => self.transition(input),
        }
    }

    /// Walks up a level in the tree and clears input buffer
    fn backtrack(&mut self) -> DriverSignal<'a> {
        self.input_buffer.clear();
        match self.path.pop() {
            Some(tree) => DriverSignal::Popped,
            None => DriverSignal::NoOp,
        }
    }

    fn transition<'b>(&mut self, input: &'b str) -> DriverSignal<'a> {
        let mut result = DriverSignal::NoOp;
        for c in String::from(input).chars() { //couldn't iterate over slice for some reason
            result = self.evaluate_char(c);
        }
        result
        // only return the last transition? feels wrong.
    }

    fn evaluate_char(&mut self, c: char) -> DriverSignal<'a> {
        self.input_buffer.push(c);
        match self.head().transition(self.input_buffer.as_str()) {
            Option::Some(tree) => self.handle_pick(tree),
            Option::None => self.handle_incomplete_transition()
        }
    }

    /// Add node to list of selections and update path.
    /// If picked value is a leaf, the behavior depends on 
    /// whether the toggle flag is active or not.
    fn handle_pick(&mut self, tree: &'a Tree) -> DriverSignal<'a> {
        self.input_buffer.clear();
        if let Tree::Node(_, _) = tree {
            self.selections.push(tree);
            self.path.push(tree);
            DriverSignal::NodePicked(tree)
        }
        // FIXME legacy code. add toggle behavior to leaf
        else if self.toggle() && self.selections.contains(&tree) {
            self.selections = self.selections.iter()
                .map(|t| *t)
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
    fn handle_incomplete_transition(&mut self) -> DriverSignal<'a> {
        if self.root.transitions_by_prefix(self.input_buffer.as_str()).len() == 0 {
            let signal = DriverSignal::DeadEnd(String::from(self.input_buffer.as_str()));
            self.input_buffer.clear();
            signal
        }
        else {
            DriverSignal::NoOp
        }
    }


    fn toggle(&self) -> bool {
        false
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::tree::LeafData;

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

    // TODO add more test cases

    #[test]
    fn test_public_api() {
        let tree = build_tree();
        let root = &tree;
        // Why did I make children return a hashmap? I wanna slap myself
        let n1 = root.children()[&"n1"];
        let leaf = n1.children()[&"l"];
        let mut driver = Driver::default(&tree);
        assert_eq!(driver.drive(DriverCommand::Backtrack), DriverSignal::NoOp);
        assert_eq!(driver.drive(DriverCommand::Transition("n")), DriverSignal::NoOp);
        assert_eq!(driver.drive(DriverCommand::Transition("1")), DriverSignal::NodePicked(n1));
        assert_eq!(driver.drive(DriverCommand::Transition("l")), DriverSignal::LeafPicked(leaf));
        assert_eq!(driver.drive(DriverCommand::Transition("k")), DriverSignal::DeadEnd(String::from("k")));
        assert_eq!(driver.drive(DriverCommand::Backtrack), DriverSignal::Popped);
    }

}
