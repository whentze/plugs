use std::fmt;

use bus::{Bus, BusReader};

#[derive(Copy, Clone, Debug)]
pub struct Sample {
    pub left: f32,
    pub right: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum NoteOnOff {
    On,
    Off,
}
pub use self::NoteOnOff::*;

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.left, self.right)
    }
}

pub struct OutPort<T>
where
    T: Clone,
{
    pub cable: Option<Bus<T>>,
}

impl<T: Clone> OutPort<T> {
    pub fn new() -> Self {
        OutPort { cable: None }
    }

    pub fn connect(&mut self, target: &mut InPort<T>)
    where
        T: Clone,
    {
        match self.cable {
            Some(_) => (),
            None => self.cable = Some(Bus::new(480)),
        };
        target.cable = Some(self.cable.as_mut().unwrap().add_rx());
    }

    pub fn disconnect(&mut self) {
        self.cable = None;
    }

    pub fn push(&mut self, data: T) {
        if let Some(ref mut c) = self.cable {
            c.broadcast(data)
        }
    }
}

pub struct InPort<T>
where
    T: Clone,
{
    pub cable: Option<BusReader<T>>,
}

impl<T: Clone> InPort<T> {
    pub fn new() -> Self {
        InPort { cable: None }
    }
    pub fn pull(&mut self) -> Option<T> {
        match self.cable {
            Some(ref mut c) => {
                match c.recv() {
                    Ok(data) => Some(data),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}
