use types::*;
use bus::Bus;

pub struct Silence {
    pub output: OutPort<Sample>,
}

impl super::Plugable for Silence {
    type Message = ();
    fn init() -> (Self, Bus<()>) {
        (Silence { output: OutPort { cable: None } }, Bus::new(0))
    }
    fn run(&mut self) -> () {
        self.output.push(Sample {
            left: 0.0,
            right: 0.0,
        });
    }
}