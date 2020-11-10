use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use yaml_rust::yaml::Hash;

use super::tree::{Tree, LeafData};

//use std::io::Read;

//Entry method for parser. Receives reader for yml file and returns parsed tree
//pub fn parse<T: Read>(reader: T) -> Tree { }
//
//


fn attr_getter<'a>(node: &'a Yaml, attr: &'a str, default: &'a str) -> &'a str {
        node[attr].as_str().unwrap_or(default)

}

///Builder method to convert fields in an yaml node to `TreeData`
fn build_data(node: &Yaml, name: &str) -> LeafData { 
    LeafData {
        name: String::from(name),
        value: String::from(attr_getter(node, &".value", name)),
        chord: String::from(attr_getter(node, &".chord", name)),
        desc: String::from(attr_getter(node, &".desc", name))
    }
}

///Warns about missing expected fields in a node.
fn validate_node(node: &Yaml, name: &str) {
    let EXPECTED_KEYS = vec![".chord", ".value", ".desc"];
    for key in EXPECTED_KEYS {
        if node[key].as_str().is_none() {
            println!("WARN: Node {} does not contain {} key. Defaulting to {}", name, key, name);
        }
    }
}


///Get keys of a node that correspond to nodes. Return list of valid keys and list of offenders
fn get_valid_children<'a>(node: &'a Yaml, name: &'a str) -> (Vec<&'a Yaml>, Vec<String>) {
    let hash = node.as_hash().unwrap();
    let len = hash.len();
    let mut node_children = Vec::with_capacity(len);
    let mut offenders = Vec::with_capacity(len);
    for entry in hash.entries() {
        match entry.key()

    }
    //node.as_hash()
}

///Convert a single yaml node into a tree. Recursive implementation that
///calls itself for a node's children.
fn node_to_tree(name: &str, node: &Yaml) -> Tree {
    validate_node(node);
    let data = build_data(node, name);
    let mut children = get_valid_children().iter().map(node_to_tree)
        .collect::<Vec<Tree>>();
    if children.is_empty() {
        Tree::Leaf(data)
    }
    else {
        Tree::Node(data, children)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_yml<'a>() -> Vec<Yaml> {
        let raw = 
"
foo:
  .chord: chord
  .desc: desc
bar:
  barjr:
    .chord: jr
";
        YamlLoader::load_from_str(raw).unwrap()
    }

    #[test]
    fn test_attr_getter_returns_default_value_if_empty() {
        let yml = &get_test_yml()[0];
        let foo = &yml["foo"];
        assert_eq!(attr_getter(foo, ".chord", "foo"), "chord");
        assert_eq!(attr_getter(foo, ".value", "foo"), "foo");
    }


}
