use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use yaml_rust::yaml::Hash;

use super::tree::{Tree, LeafData};

//use std::io::Read;

//Entry method for parser. Receives reader for yml file and returns parsed tree
//pub fn parse<T: Read>(reader: T) -> Tree { }

///Builder method to convert fields in an yaml node to `TreeData`
fn build_data(node: &Hash, name: &str) -> LeafData { 

    let to_string_with_default = |opt: Option<Yaml>, default: &str| {
        let ref_str = opt.map(|yml: &Yaml| yml.as_str()).unwrap_or(name);
        String::from(ref_str)
    }

    LeafData {
        name: String::from(name),
        value: to_string_with_default(node.get(&".value"), name),
        chord: to_string_with_default(node.get(&".chord"), name),
        desc: to_string_with_default(node.get(&".desc"), name),
    }
}

//Convert a single yaml node into a tree. Recursive implementation that
//calls itself for a node's children.
//fn node_to_tree(node: &Hash, name: &str) -> Tree;


#[cfg(test)]
mod tests {
    use super::*;

}
