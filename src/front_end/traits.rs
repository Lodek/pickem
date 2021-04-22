use super::driver::{Driver, DriverSignal, DriverCommand};

pub trait View {
    fn update(&mut self, signal: &DriverSignal) -> Result<()>;
}

pub trait Controller {
    /// Entrypoint for controller instance. Method should run until
    /// pickem execution is complete.
    pub fn run(&mut self) -> Result<()>;
}
