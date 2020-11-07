use std::collections::HashMap;
use std::str;

///Encapsulates the data stored by a leaf
struct LeafData {
    name: String,
    desc: String,
    chord: String,
    value: String
}

///Tree is a recursive data type with two forms: `Node` and `Leaf`.
///`Leaf` contains data.
///`Node` contains data and a list of `Tree`
enum Tree {
    Node(LeafData, Vec<Tree>),
    Leaf(LeafData)
}

///Returns map of children 1st level transitions for a tree.
fn children(tree: &Tree) -> HashMap<String, Tree> {
    match tree {
        Tree::Leaf(_) => HashMap::new(),
        Tree::Node(_, children) => {
            let map = HashMap::new();
            for child in children {
                map.insert(child.chord, child);
            }
        }
    }
}

///Attempts to return a child of `Tree` whose chord is `chord`.
fn transition(tree: &Tree, chord: &String) -> Option<Tree> {
    let children_map = children(tree);
    children_map.get(chord);
}


///Internal for a tree transition
///Stores the previous root and the picked tree
type Pick = (Tree, Tree);

///Abstracts the traversal through a tree. Each transition made using `pick` is added
///to `Picks`' internal state. Provides methods to manage its state.
struct Picks {
   picks: Vec<Pick>   
}

impl Picks {

    // Constructor
    //fn new() -> Picks { }

    //Picks a child from `tree` based on the chord property.
    //Option is empty if no child was found for the given chord.
    //fn pick(&picks: Picks, tree: Tree, chord: String) -> Option<Tree> {}


    //Undo the previous pick operation
    //fn unpick(&picks: Picks) -> Option<Tree> {}


    //Returns list of values from all picked trees.
    //Result is sorted chronologically
    //fn get_values(&picks: Picks) -> Vec<String> {}

    //Return list of trees pick. Result is sorted chronologically
    //fn get_trees(&picks: Picks) -> Vec<Tree> {} 

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn children_from_node_returns_transitions() {
        let c1 = Tree::Leaf(LeafData {
            name: String::from("c1")
            desc: String::from("c1"),
            chord: String::from("c1"),
            value: String::from("c1")
        });

        let c2 = Tree::Leaf(LeafData {
            name: String::from("c2"),
            desc: String::from("c2"),
            chord: String::from("c2"),
            value: String::from("c2")
        });

        let parent = Tree::Node(LeafData {
            name: String::from("p"),
            desc: String("p"),
            chord: String::from("p"),
            value: String::from("p")
        }, vec![c1, c2]);

        let children_map = children(&parent);
        assert_eq!(children_map.get(String::from("c1")), c1);
        assert_eq!(children_map.get(String::from("c2")), c2);
    }

    #[test]
    fn children_from_node_returns_transitions() {
        let leaf = Tree::Leaf(LeafData {
            name: String::from("c1"),
            desc: String::from("c1"),
            chord: String::from("c1"),
            value: String::from("c1")
        });
        let children_map = children(&leaf);
        assert_eq!(children_map.len(), 0);
    }


}
