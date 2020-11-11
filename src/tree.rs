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
        match self {
            Tree::Leaf(_) => HashMap::new(),
            Tree::Node(_, children) => {
                let mut map = HashMap::new();
                for child in children {
                    let data = child.data();
                    map.insert(data.chord.as_str(), child);
                }
                map
            }
        }
    }


    ///Attempts to return a child of `Tree` whose chord is `chord`.
    pub fn transition(&self, chord: &str) -> Option<&Tree> {
        let transitions = self.children();
        match transitions.get(chord) {
            Option::Some(tree) => Option::Some(*tree),
            Option::None => Option::None
        }
    }
}


///Internal for a tree transition
///Stores the previous root and the picked tree
struct Pick<'a> {
    root: &'a Tree,
    picked: &'a Tree
}

///Abstract data type for traversal through a tree. Each transition made using `pick` is added
///to `Picks`' internal state. 
///Provides methods to manage its state.
pub struct Picks<'a> {
    picks: Vec<Pick<'a>>   
}

impl<'a> Picks<'a> {

    /// Constructor
    pub fn new() -> Picks<'a> {
        return Picks {
            picks: Vec::new()
        }
    }

    ///Picks a child from `tree` based on the chord property.
    ///Option is empty if no child was found for the given chord.
    pub fn pick(&mut self, tree: &'a Tree, chord: &str) -> Option<&'a Tree> {
        match tree.transition(chord) {
            Option::None => Option::None,
            Option::Some(child) => {
                let pick = Pick {
                    root: tree, 
                    picked: child
                };
                self.picks.push(pick);
                Option::Some(child)
            }
        }
    }

    ///Undo the previous pick operation
    pub fn unpick(&mut self) -> Option<&'a Tree> {
        match self.picks.pop() {
            Option::None => Option::None,
            Option::Some(Pick {root, picked: _}) => {
                Option::Some(root)
            }
        }
    }


    ///Returns list of values from all picked trees.
    ///Result is sorted chronologically
    pub fn get_values(&self) -> Vec<&str> {
        self.picks.iter()
            .map(|pick| pick.picked.data().value.as_str())
            .collect::<Vec<&str>>()
    }

    ///Return list of trees pick. Result is sorted chronologically
    pub fn get_trees(&self) -> Vec<&Tree> {
        self.picks.iter()
            .map(|pick| pick.picked)
            .collect::<Vec<&Tree>>()
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

    #[test]
    fn pick_from_tree_returns_leaf() {
        let leaf = Tree::Leaf(data_builder(String::from("leaf")));
        let root = Tree::Node(data_builder(String::from("root")), vec![leaf]);
        let mut picks = Picks::new();
        match picks.pick(&root, &"leaf") {
            Option::Some(_) => {
                assert_eq!(true, true)
            }
            Option::None => panic!("pick didn't return new root")
        }
    }

    #[test]
    fn unpick_from_tree_returns_old_root() {
        let leaf = Tree::Leaf(data_builder(String::from("leaf")));
        let root = Tree::Node(data_builder(String::from("root")), vec![leaf]);
        let mut picks = Picks::new();
        let _new_root;
        match picks.pick(&root, &"leaf") {
            Option::Some(body) => _new_root = body,
            _ => panic!("this feels wrong") //this does feel wrong. there has to be a better way to do this assignment
        }
        match picks.unpick() {
            Option::None => panic!("unpick didn't return old root"),
            Option::Some(tree) => {
                assert_eq!(tree.data().name, String::from("root"))
            }
        }
    }

}
