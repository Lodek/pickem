
///Tree is a recursive data type with two forms: `Node` and `Leaf`.
///`Leaf`s have a name, description, chord and a value.
///`Node` is the same as `Leaf` with a list of `Tree`s.
enum Tree {
    Node(String, String, String, String, Vec<Tree>),
    Leaf(String, String, String, String)
}

///Returns map of transitions 1st level transitions for a tree.
fn getTransitions(tree: Tree) -> HashMap<String, Tree> {}

///Attempts to return a child of `Tree` whose chord is `chord`.
fn transition(tree: Tree, chord: String) -> Option<Tree> {}



///State represents a transition made by `Tree`. Selecting a leaf 
///or stepping into a node generates a new state.
///Each state contains all the data necessary to reproduce a transition.
///- `root`: the `Tree` from which the transition occured.
///- `picked`: the `Leaf` or `Node` selected in a transition.
///- `value`:  the value returned by the selected `Tree`

type Pick = (Tree, Tree);

struct Picks {
   picks: Vec<Pick>   
}

impl Picks {
    fn new() -> Pick {}
    fn pick(&picks: Picks, tree: Tree, chord: String) -> Pick {}
    fn unpick(&picks: Picks) -> Pick {}
    fn getValues(&picks: Picks) -> Vec<String> {}
    fn getTrees(&picks: Picks) -> Vec<Tree> {} 
}
