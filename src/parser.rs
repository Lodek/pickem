use yaml_rust::{YamlLoader, Yaml};

use super::tree::{Tree, LeafData};

static RESERVED_KEYS: &[&str] = &[".value", ".chord", ".desc"];

///Identifies a violating node by its parent and its name, respectively
#[derive(Debug)]
pub struct Violation {
    parent_name: String,
    child_name: String,
    violation: String
}

pub enum Flags {
    LeafInheretValues
}

enum NodeType<'a> {
    Violator(Violation),
    Value(String),
    Child(NamedNode<'a>)
}

///Yaml node and its name
type NamedNode<'a> = (&'a str, &'a Yaml);


///Takes a yaml node that belongs to a parent and defines a type to it.
fn child_or_violator<'a>(parent_name: &'a str, child_name: &'a str, child: &'a Yaml) -> NodeType<'a> {
    match child {
        Yaml::Hash(_) => NodeType::Child((child_name, child)),
        Yaml::String(value) => {
            if RESERVED_KEYS.contains(&child_name) {
                NodeType::Value(String::from(value))
            }
            else {
                let violation = Violation {
                    parent_name: String::from(parent_name),
                    child_name: String::from(child_name),
                    violation: format!("{} is not a reserved keyword, hence cannot have a string value", child_name)
                };
                NodeType::Violator(violation)
            }
        },
        _ => {
                let violation = Violation {
                    parent_name: String::from(parent_name),
                    child_name: String::from(child_name),
                    violation: format!("The value of every YAML node must be a hash (asside from reserved keys)")
                };
                NodeType::Violator(violation)
        }
    }
}

fn list_of_pairs_into_pair_of_lists<T, U>(list: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
    let mut ts: Vec<T> = Vec::new();
    let mut us: Vec<U> = Vec::new();
    for (t, u) in list.into_iter() {
        ts.push(t);
        us.push(u);
    }
    (ts, us)
}


fn node_to_tree(name: &str, node: &Yaml) -> (Tree, Vec<Violation>) {
    let classified_nodes =  children(name, node);
    let mut violations: Vec<Violation> = Vec::new();
    let mut children: Vec<NamedNode> = Vec::new();
    for node in classified_nodes.into_iter() {
        match node {
            NodeType::Violator(violation) => violations.push(violation),
            NodeType::Child(named_node) => children.push(named_node),
            NodeType::Value(value) => ()
        }
    }
    let (trees, mut nested_violations) = list_of_pairs_into_pair_of_lists(children.into_iter()
        .map(uncurried_node_to_tree)
        .collect::<Vec<(Tree, Vec<Violation>)>>());
    nested_violations.push(violations);
    let violations: Vec<Violation> = nested_violations.into_iter().flatten().collect();

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
fn children<'a>(parent_name: &'a str, node: &'a Yaml) -> Vec<NodeType<'a>> {
    //should be a safe operation because the parent *should* only call this
    //for NodeType::Children values
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


// TODO Receive flag + implement value inheretance.
pub fn parse(yml: &str) -> (Tree, Vec<Violation>) {
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
