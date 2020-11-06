
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
struct State {
    root: Tree,
    picked: Tree,
    value: String
}

///Initialize the stack of states for a tree.
fn initStates(root: Tree) -> Vec<State> {

}

///From a list of states and a chord, performs a transition based on the lastest state
fn transition(states: Vec<State>, chord: String) -> Vec<State> {

}

///Undos the last state
fn backstep(states: Vec<State>) -> Vec<State> {

}

///Transforms list of states into a list of values
fn getValues(states: Vec<State>) -> Vec<String> {

}
