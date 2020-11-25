use yaml_rust::{YamlLoader, Yaml};

use linked_hash_map::OccupiedEntry;

use super::tree::{Tree, LeafData};

//use std::io::Read;

//Entry method for parser. Receives reader for yml file and returns parsed tree
//pub fn parse<T: Read>(reader: T) -> Tree { }
//
//
//
static RESERVED_KEYS: &[&str] = &[".value", ".chord", ".desc"];

///Identifies a violating node by its parent and its name, respectively
pub struct Violation {
    parent_name: String,
    child_name: String,
    violation: String
}

///Yaml node and its name
type NamedNode<'a> = (&'a str, &'a Yaml);


///Takes a yaml node that belongs to a parent and identify whether that node is a valid child.
///Returns a result indicating a violation or a child tuple.
fn child_or_violator<'a>(parent_name: &'a str, child_name: &'a str, node: &'a Yaml) -> Result<NamedNode<'a>, Violation> {
    if RESERVED_KEYS.contains(&child_name) {
        let result;
        match node {
            Yaml::String(_) => {
                result = Ok((child_name, node));
            }
            _ => {
                let violation = Violation {
                    parent_name: String::from(parent_name),
                    child_name: String::from(child_name),
                    violation: format!("Value of reserved keyword {} must be String", child_name)
                };
                result = Err(violation);
            }
        }
        return result;
    }
    match node {
        Yaml::Hash(_) => Ok((child_name, node)),
        _ => {
            let violation = Violation  {
                parent_name: String::from(parent_name),
                child_name: String::from(child_name),
                violation: String::from("The value of every YAML node must be a hash (asside from reserved keys)")
            };
            Err(violation)
        }
    }
}


fn separate_results(mut results: Vec<Result<NamedNode, Violation>>) -> (Vec<NamedNode>, Vec<Violation>) {
    let violations = Vec::new();
    let children = Vec::new();
    loop {
        if let Option::Some(result) = results.pop() {
            match result {
                Result::Ok(child) => children.push(child),
                Result::Err(violation) => violations.push(violation)
            }
        }
        else {
            break;
        }
    }
    (children, violations)
}

fn node_to_tree(name: &str, node: &Yaml) -> (Tree, Vec<Violation>) {
    let (children, mut violations) =  separate_results(children(name, node));
    let trees_and_violations: Vec<(Tree, Vec<Violation>)> = children.iter()
        .map(uncurried_node_to_tree)
        .collect();
    let trees: Vec<Tree> = trees_and_violations.iter().map(|(tree, _)| tree).collect();
    let mut nested_violations: Vec<Vec<Violation>> = trees_and_violations.iter().map(|(_, violations)| violations).collect();
    nested_violations.push(violations);
    let violations: Vec<Violation> = nested_violations.iter().flatten().collect();

    let tree;
    let data = build_data(node, name);
    if trees.is_empty() {
        tree = Tree::Leaf(data);
    }
    else {
        tree = Tree::Node(data, trees)
    }
    (tree, violations)
}

///Uncurried version of node_to_tree
fn uncurried_node_to_tree(named_node: NamedNode) -> (Tree, Vec<Violation>){
    let (name, node) = named_node;
    node_to_tree(name, node)
}

///Gets children for a node and calls child_or_violator on all of them
fn children<'a>(parent_name: &'a str, node: &'a Yaml) -> Vec<Result<NamedNode<'a>, Violation>> {
    //should be a safe operation because the parent 
    //is validated before calling this.
    let hash = node.as_hash().unwrap();
    let f = |(key, value): (&'a Yaml, &'a Yaml)| {
        let node_name = key.as_str().unwrap(); //also should be safe
        child_or_violator(parent_name, node_name, value)
    };
    hash.iter()
        .map(f)
        .collect()
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


pub fn parse(yml: &str) -> Tree {
    let loaded_yaml  = YamlLoader::load_from_str(yml).unwrap();
    let yaml = &loaded_yaml[0];
    node_to_tree("root", &yaml)
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
    }

    #[test]
    fn get_children_returns_children() {
    }

    #[test]
    fn node_to_tree_converts() {
    }
}
