use pickem::tree::Tree;
use pickem::parser;
use pickem::cli_driver::CliDriver;
use pickem::args::Config;

fn main() {

    let config = Config::from_env();
    let data = config.raw_yaml().unwrap();
    let tree = parser::parse(data.as_str());
    let mut driver = CliDriver::new(&tree).unwrap();
    driver.run().unwrap();
    driver.cleanup().unwrap();
    driver.present_result().unwrap();
}
