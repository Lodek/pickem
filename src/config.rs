use clap::{Arg, App};

#[derive(PartialEq)]
///Configures how the result will be presented
pub enum ResultMode {
    Last,
    Leaves,
    All
}

#[derive(PartialEq)]
///Possible conditions for program termination
pub enum BreakCondition {
    DeadEnd,
    FirstLeaf,
}

pub struct Config {
    pub file: String,
    pub result_mode: ResultMode,
    pub break_condition: BreakCondition,
}

impl Config {

    pub fn from_args(args: &[String]) -> Config {
        Config {
            file: "",
            result_mode: ResultMode::AllNodes,
            break_condition: BreakCondition::DeadEnd
        }
    }
}

fn build_parser() -> App {
    let matches = App::new("Pickem")
        .version("0.1")
        .author("Bruno G. <gomes.bruno.ac@gmail.com>")
        .about("Command line selection tool")
        .arg(Arg::new("INPUT")
             .about("Set input yaml file. Leave blank to read from stdin.")
             .required(false)
             .default("")
             .index(1))
        .arg(Arg::with_name("result_mode")
             .short("r")
             .long("result")
             .takes_value(true)
             .help("Sets how result is going to be computed")
             .possible_values(&["last, leaves, all"])
             .default("last"))
        .arg(Arg::with_name("break_condition")
             .short("b")
             .long("break")
             .takes_value(true)
             .help("Configures when pickem is going to exit.")
             .possible_values(&["dead_end, first_leaf"])
             .default("dead_end"))
}
