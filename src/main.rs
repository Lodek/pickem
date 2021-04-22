use pickem::tree::Tree;
use pickem::parser;
use pickem::frontend::tui::{Controller};
use pickem::args::Config;

fn main() {
    let config = Config::from_env();
    let data = config.raw_yaml().unwrap();
    let (tree, violations) = parser::parse(data.as_str());
    if config.is_dryrun() {
        for violation in violations.iter() {
            println!("{:?}", violation);
        }
        println!("{}", tree);
    }
    else {
        //let mut controller = Controller::new(&tree).unwrap();
        //driver.run().unwrap();
        //driver.cleanup().unwrap();
        //driver.present_result().unwrap();
    }
}
