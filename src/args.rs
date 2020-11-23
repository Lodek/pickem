use clap::{App, Arg, ArgMatches};
use super::driver::{ResultMode, BreakCondition};
use std::{io, fs};
use std::io::{Result, Read};

pub struct Config {
    file: String,
    pub result_mode: ResultMode,
    pub break_condition: BreakCondition
}

fn parser() -> App<'static, 'static> {
    App::new("Pickem")
        .version("0.1")
        .author("Bruno G. <gomes.bruno.ac@gmail.com>")
        .about("Command line selection tool")
        .arg(Arg::with_name("INPUT")
             .help("Set input yaml file, '-' to read from stdin.")
             .required(true)
             .index(1))
        .arg(Arg::with_name("result_mode")
             .short("r")
             .long("result")
             .takes_value(true)
             .help("Sets how result is going to be computed")
             .possible_values(&["last", "leaves", "all"])
             .default_value("last"))
        .arg(Arg::with_name("break_condition")
             .short("b")
             .long("break")
             .takes_value(true)
             .help("Configures when pickem is going to exit.")
             .possible_values(&["dead_end", "first_leaf"])
             .default_value("dead_end"))
}

impl Config {

    ///Build Config from matches
    pub fn from_env() -> Config {
        let m = parser().get_matches();
        let result_mode = Config::enumfy_result(m.value_of("result_mode").unwrap());
        let break_condition = Config::enumfy_condition(m.value_of("break_condition").unwrap());
        //All values have defaults. As such unwrap do be safe though.
        Config {
            file: String::from(m.value_of("INPUT").unwrap()),
            result_mode: result_mode,
            break_condition: break_condition
        }
    }

    ///Returns the data to be used for pickem.
    pub fn raw_yaml(&self) -> Result<String> {
        if self.file.as_str() == "-" {
            let mut stdin = io::stdin();
            let mut data = String::new();
            stdin.read_to_string(&mut data).map(|_| data)
        }
        else {
            fs::read_to_string(self.file.as_str())
        }
    }

    ///Convert string value to ResultMode
    fn enumfy_result(result: &str) -> ResultMode {
        //TODO In Haskell I could use the Read class type to deal with this situation
        //Rust prob has something similar
        match result {
            "last" => ResultMode::Last,
            "leaves" => ResultMode::Leaves,
            "all" => ResultMode::All,
            _ => panic!("This should never happen")
        }
    }

    ///Convert string value to break condition
    fn enumfy_condition(cond: &str) -> BreakCondition {
        //TODO see above ^
        match cond {
            "dead_end" => BreakCondition::DeadEnd,
            "first_leaf" => BreakCondition::FirstLeaf,
            _ => panic!("This should never happen")
        }
    }
}
