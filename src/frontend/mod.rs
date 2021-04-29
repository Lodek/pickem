pub mod tui;

use std::io::{Result};
use super::driver::{Driver, DriverSignal, DriverCommand};

pub trait View {
    fn update(&mut self, driver: &Driver, signal: &DriverSignal) -> Result<()>;
}

pub trait Controller {
    /// Entrypoint for controller instance. Method should run until
    /// pickem execution is complete.
    fn run(&mut self) -> Result<()>;
}
