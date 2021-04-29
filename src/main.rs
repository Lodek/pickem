use pickem::tree::Tree;
use pickem::parser;
use pickem::frontend::View;
use pickem::frontend::tui::{Controller, OutputView, TUI, OutputFormat, Flags};
use pickem::driver::{Driver, DriverFlag};
use pickem::args::Config;
use pickem::frontend::Controller as ControllerTrait;


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
        let mut driver = Driver::default(&tree);
        let mut tui = TUI::new().unwrap();
        let mut output_view = OutputView::new(OutputFormat::Value).unwrap();
        let views: Vec<&mut dyn View> = vec![&mut tui, &mut output_view];
        let flags = vec![Flags::LoopMode];
        let mut controller = Controller::new(&mut driver, views, flags).unwrap();
        controller.run().unwrap();
    }
}
