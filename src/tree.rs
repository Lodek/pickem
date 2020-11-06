
///Encapsulates the data stored by a leaf
struct LeafData {
    name: String,
    desc: String,
    chords: String,
    value: String
}

///Tree is a recursive data type with two forms: `Node` and `Leaf`.
///`Leaf` contains data.
///`Node` contains data and a list of `Tree`
enum Tree {
    Node(LeafData, Vec<Tree>),
    Leaf(LeafData)
}

///Returns map of transitions 1st level transitions for a tree.
fn getTransitions(tree: Tree) -> HashMap<String, Tree> {}

///Attempts to return a child of `Tree` whose chord is `chord`.
fn transition(tree: Tree, chord: String) -> Option<Tree> {}


///Internal for a tree transition
///Stores the previous root and the picked tree
type Pick = (Tree, Tree);

///Abstracts the traversal through a tree. Each transition made using `pick` is added
///to `Picks`' internal state. Provides methods to manage its state.
struct Picks {
   picks: Vec<Pick>   
}

impl Picks {

    /// Constructor
    fn new() -> Picks {

    }

    ///Picks a child from `tree` based on the chord property.
    ///Option is empty if no child was found for the given chord.
    fn pick(&picks: Picks, tree: Tree, chord: String) -> Option<Tree> {}


    ///Undo the previous pick operation
    fn unpick(&picks: Picks) -> Option<Tree> {}


    ///Returns list of values from all picked trees.
    ///Result is sorted chronologically
    fn getValues(&picks: Picks) -> Vec<String> {}

    ///Return list of trees pick. Result is sorted chronologically
    fn getTrees(&picks: Picks) -> Vec<Tree> {} 

}
