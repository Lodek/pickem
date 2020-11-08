use std::collections::HashMap;

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

fn tree_data(tree: &Tree) -> &LeafData {
    match tree {
        Tree::Leaf(d) => d,
        Tree::Node(d, _) => d
    }
}


///Returns map of children 1st level transitions for a tree.
fn children(tree: &Tree) -> HashMap<String, &Tree> {
    match tree {
        Tree::Leaf(_) => HashMap::new(),
        Tree::Node(_, children) => {
            let mut map = HashMap::new();
            for child in children {
                let data = tree_data(child);
                map.insert(data.chord.clone(), child);
            }
            map
        }
    }
}


///Attempts to return a child of `Tree` whose chord is `chord`.
fn transition<'a>(tree: &'a Tree, chord: &String) -> Option<&'a Tree> {
    //This is weird, I wanted to use `children` but wasn't able to make it work.
    match tree {
        Tree::Leaf(_) => Option::None,
        Tree::Node(_, children) => {
            let mut opt: Option<&Tree> = None;
            for child in children {
                let data = tree_data(child);
                if &data.chord == chord {
                    opt = Some(child);
                }
            }
            opt
        }
    }
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
        let data = tree_data(&tree);
        assert_eq!(data.name, String::from("test"));
        assert_eq!(data.desc, String::from("test"));
        assert_eq!(data.chord, String::from("test"));
        assert_eq!(data.value, String::from("test"));
    }

    #[test]
    fn children_from_node_returns_transitions() {
        let c1 = Tree::Leaf(LeafData {
            name: String::from("c1"),
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
            desc: String::from("p"),
            chord: String::from("p"),
            value: String::from("p")
        }, vec![c1, c2]);

        let children_map = children(&parent);

        match children_map.get(&String::from("c1")) {
            Some(tree) => assert_eq!(tree_data(tree).name, String::from("c1")),
            None       => panic!("Map doesn't contain child!")
        }

        match children_map.get(&String::from("c2")) {
            Some(tree) => assert_eq!(tree_data(tree).name, String::from("c2")),
            None       => panic!("Map doesn't contain child!")
        }

    }

    #[test]
    fn children_from_leaf_returns_no_transitions() {
        let leaf = Tree::Leaf(data_builder(String::from("c1")));
        let children_map = children(&leaf);
        assert_eq!(children_map.len(), 0);
    }


}
