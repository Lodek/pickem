# Modules
- core
- frontends

seems hard to make a common core module, unless it's used solemnly to transform the yml into data.
yeah, so core would be yml validation and conversion and traversion functions.

terminal frontend like fzf. how?
one frontend
https://github.com/fdehau/tui-rs
https://github.com/redox-os/termion

Termion seems like a good option to build the UI.
It seems light and pure rust.

Now this allows for some cool features.
So after I define the core module and functions to deal with the yml stuff, I'll write a frontend using termion which I can use to traverse the tree.
The core should have two different types of tree traversal, toggle trees and value trees.
I am not sure how to handle those two different situations yet but the tree will be the same? Will it?

Two different tree algs: short circuit and stack.
In shortcircuit, the first selected node with a value will return the value.
In stack, if a node has no children, its value is added to the stack; otherwise step into node. Repeat until signaled to stop, then return stack of values.

## Core
- Yml parsing
- structure validation
- data types
- functions for types
- short circuit traversion function
- stack traversion function

## Frontend
- Simple as possible initial frontend
