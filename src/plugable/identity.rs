use types::*;
use bus::Bus;

pub struct Identity {
    pub input: InPort<Sample>,
    pub output: OutPort<Sample>,
}

impl super::Plugable for Identity {
    type Message = ();
    fn init() -> (Self, Bus<()>) {
        (Identity {
            input: InPort::new(),
            output: OutPort { cable: None },
        }, Bus::new(0))
    }
    fn run(&mut self) -> () {
        if let Some(d) = self.input.pull() { self.output.push(d) }
    }
}
