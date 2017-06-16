use types::*;
use byteorder::{WriteBytesExt, LittleEndian};
use bus::Bus;
use std::io;

pub struct PCMPrinter {
    pub input: InPort<Sample>,
}

impl super::Plugable for PCMPrinter {
    type Message = ();
    fn init() -> (Self, Bus<()>) {
        (PCMPrinter { input: InPort::new() }, Bus::new(0))
    }
    fn run(&mut self) -> () {
        if let Some(d) = self.input.pull() {
            io::stdout().write_f32::<LittleEndian>(d.left).unwrap();
            io::stdout().write_f32::<LittleEndian>(d.right).unwrap();
        };
    }
}