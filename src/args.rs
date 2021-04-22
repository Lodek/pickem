use clap::{App, Arg, ArgMatches};
use std::{io, fs};
use std::io::{Result, Read};

pub struct Config<'a> {
    file: String,
    matches: ArgMatches<'a>
}

fn parser() -> App<'static, 'static> {
    App::new("Pickem")
        .version("0.2.0")
        .author("Bruno G. <gomes.bruno.ac@gmail.com>")
        .about("Command line selection tool")
        .arg(Arg::with_name("INPUT")
             .help("Set input yaml file, '-' to read from stdin.")
             .required(true)
             .index(1))
        .arg(Arg::with_name("dryrun")
             .short("d")
             .long("dryrun")
             .required(false)
             .help("Performs a dry run by parsing the input yaml file, diplaying warnings and the final configuration"))
}

impl Config<'_> {

    ///Build Config from matches
    pub fn from_env<'a>() -> Config<'a> {
        let m = parser().get_matches();
        //All values have defaults. As such unwrap do be safe though.
        Config {
            file: String::from(m.value_of("INPUT").unwrap()),
            matches: m
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

    pub fn is_dryrun(&self) -> bool {
        self.matches.is_present("dryrun")
    }
}
