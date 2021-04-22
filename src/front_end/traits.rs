use super::driver::{Driver, DriverSignal, DriverCommand};

pub trait View {
    fn update(&mut self, signal: DriverSignal) -> Result<()>;
}

pub trait Controller {
    pub fn run(&mut self) -> Result<()>;
}
