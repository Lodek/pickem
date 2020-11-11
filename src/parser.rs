use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use yaml_rust::yaml::Hash;

use linked_hash_map::OccupiedEntry;

use super::tree::{Tree, LeafData};

//use std::io::Read;

//Entry method for parser. Receives reader for yml file and returns parsed tree
//pub fn parse<T: Read>(reader: T) -> Tree { }
//
//
//
static EXPECTED_KEYS: &[&str] = &[".value", ".chord", ".desc"];

///Identifies a violating node by its parent and its name, respectively
type Violation<'a> = (&'a str, &'a str);

///Represents a valid child node by its name and its Yaml struct.
type Child<'a> = (&'a str, &'a Yaml);


///Returns list of violating children of `node`.
fn get_violators<'a>(parent_name: &'a str, node: &'a Yaml) -> Vec<Violation<'a>> {
    let hash = node.as_hash().unwrap();
    let mut violators = Vec::with_capacity(hash.len());
    for (key, value) in hash.iter() {
        match value {
            Yaml::Hash(_) => (),
            _ => {
                let key_name = key.as_str().unwrap();
                if !EXPECTED_KEYS.contains(&key_name) {
                    violators.push((parent_name, key_name))
                }
            }
        }
    }
    return violators;
}

fn get_children(node: &Yaml) -> Vec<Child> {
    //TODO impl
    Vec::new()
}


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


/*

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
*/


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
  violation1: whoops
  other_violation: 
    - i
    - am
    - a
    - violation
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

    #[test]
    fn get_violators_return_violations() {
        let yml = &get_test_yml()[0];
        let bar = &yml["bar"];
        let violations = get_violators(&"bar", bar);
        assert!(violations.contains(&(&"bar", &"violation1")));
        assert!(violations.contains(&(&"bar", &"other_violation")));
        assert!(!violations.contains(&(&"bar", &"barjr")));
    }


}
