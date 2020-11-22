use std::collections::HashMap;

///Encapsulates the data stored by a leaf
#[derive(Debug, PartialEq)]
pub struct LeafData {
    pub name: String,
    pub desc: String,
    pub chord: String,
    pub value: String
}


///Tree is a recursive data type with two forms: `Node` and `Leaf`.
///`Leaf` contains data.
///`Node` contains data and a list of `Tree`
#[derive(Debug)]
pub enum Tree {
    Node(LeafData, Vec<Tree>),
    Leaf(LeafData)
}

impl Tree {

    pub fn data(&self) -> &LeafData {
        match self {
            Tree::Leaf(d) => d,
            Tree::Node(d, _) => d
        }
    }

    ///Returns map of children 1st level transitions for a tree.
    pub fn children(&self) -> HashMap<&str, &Tree> {
        self.transitions_by_prefix(&"")
    }


    ///Attempts to return a child of `Tree` whose chord is `chord`.
    pub fn transition(&self, chord: &str) -> Option<&Tree> {
        let transitions = self.children();
        match transitions.get(chord) {
            Option::Some(tree) => Option::Some(*tree),
            Option::None => Option::None
        }
    }

    pub fn transitions_by_prefix(&self, prefix: &str) -> HashMap<&str, &Tree> {
        match self {
            Tree::Leaf(_) => HashMap::new(),
            Tree::Node(_, children) => {
                let mut map = HashMap::new();
                for child in children {
                    let data = child.data();
                    let chord = data.chord.as_str();
                    if chord.starts_with(prefix) {
                        map.insert(chord, child);
                    }

                }
                map
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn data_builder(param: String) -> LeafData {
        let data = LeafData {
            name: param.clone(),
            desc: param.clone(),
            chord: param.clone(),
            value: param.clone(),
        };
        return data;
    }

    #[test]
    fn data_from_node_returns_data() {
        let tree = Tree::Leaf(data_builder(String::from("test")));
        let data = tree.data();
        assert_eq!(data.name, String::from("test"));
        assert_eq!(data.desc, String::from("test"));
        assert_eq!(data.chord, String::from("test"));
        assert_eq!(data.value, String::from("test"));
    }

    #[test]
    fn children_from_node_returns_transitions() {
        let c1 = Tree::Leaf(data_builder(String::from("c1")));
        let c2 = Tree::Leaf(data_builder(String::from("c2")));
        let parent = Tree::Node(
            data_builder(String::from("p")),
            vec![c1, c2]);

        let children_map = parent.children();

        match children_map.get(&"c1") {
            Some(tree) => assert_eq!(tree.data().name, String::from("c1")),
            None       => panic!("Map doesn't contain child!")
        }

        match children_map.get(&"c2") {
            Some(tree) => assert_eq!(tree.data().name, String::from("c2")),
            None       => panic!("Map doesn't contain child!")
        }

    }

    #[test]
    fn children_from_leaf_returns_no_transitions() {
        let leaf = Tree::Leaf(data_builder(String::from("c1")));
        let children_map = leaf.children();
        assert_eq!(children_map.len(), 0);
    }


}
