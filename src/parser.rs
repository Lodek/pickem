use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use yaml_rust::yaml::Hash;

use super::tree::{Tree, LeafData};

//use std::io::Read;

//Entry method for parser. Receives reader for yml file and returns parsed tree
//pub fn parse<T: Read>(reader: T) -> Tree { }
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

//Convert a single yaml node into a tree. Recursive implementation that
//calls itself for a node's children.
//fn node_to_tree(node: &Hash, name: &str) -> Tree;


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
