use bus::{Bus, BusReader};

use types::*;

pub struct VCA {
    pub ctrlport: BusReader<()>,
    pub signal_in: InPort<Sample>,
    pub gain_in: InPort<Sample>,
    pub output: OutPort<Sample>,
}

impl super::Plugable for VCA {
    type Message = ();
    fn init() -> (Self, Bus<()>) {
        let mut b = Bus::new(100);
        (
            VCA {
                ctrlport: b.add_rx(),
                signal_in: InPort::new(),
                gain_in: InPort::new(),
                output: OutPort { cable: None },
            },
            b,
        )
    }
    fn run(&mut self) -> () {
        if let Some(signal) = self.signal_in.pull() {
            if let Some(Sample { left: gain, .. }) = self.gain_in.pull() {
                self.output.push(Sample {
                    left: gain * signal.left,
                    right: gain * signal.right,
                });
            } else {
                self.output.push(signal);
            }
        }
    }
}
