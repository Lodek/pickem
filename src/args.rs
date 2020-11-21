use clap::{ArgMatches};
use super::driver::{ResultMode, BreakCondition};

pub struct Config {
    pub file: String,
    pub result_mode: ResultMode,
    pub break_condition: BreakCondition
}

impl Config {

    ///Build Config from matches
    pub fn new(m: ArgMatches) -> Config {
        let result_mode = Config::enumfy_result(m.value_of("result_mode").unwrap());
        let break_condition = Config::enumfy_condition(m.value_of("break_condition").unwrap());
        //All values have defaults. As such unwrap do be safe though.
        Config {
            file: String::from(m.value_of("INPUT").unwrap()),
            result_mode: result_mode,
            break_condition: break_condition
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
