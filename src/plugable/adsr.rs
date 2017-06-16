use bus::{Bus, BusReader};

use types::*;
use consts::SAMPLERATE;
use envelope::{Envelope, Point, InterpolationMode};

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Start(f32),
    Stop,
    SetAttack(f32),
    SetDecay(f32),
    SetSustain(f32),
    SetRelease(f32),
}
use self::Message::*;

pub struct ADSR {
    envelope_on: Envelope,
    envelope_off: Envelope,
    state: NoteOnOff,
    level: f32,
    time_since_change: f32,
    pub ctrlport: BusReader<Message>,
    pub cv_in: InPort<Sample>,
    pub output: OutPort<Sample>,
}

impl super::Plugable for ADSR {
    type Message = self::Message;
    fn init() -> (Self, Bus<Message>) {
        let mut b = Bus::new(100);
        let env1 = Envelope::new(
            &[
                Point {
                    x: 0.0,
                    y: 0.0,
                    interpolation_mode: InterpolationMode::Flat,
                },
                Point {
                    x: 0.2,
                    y: 1.0,
                    interpolation_mode: InterpolationMode::Quadratic,
                },
                Point {
                    x: 0.4,
                    y: 0.6,
                    interpolation_mode: InterpolationMode::Quadratic,
                },
            ],
        );
        let env2 = Envelope::new(
            &[
                Point {
                    x: 0.0,
                    y: 0.6,
                    interpolation_mode: InterpolationMode::Flat,
                },
                Point {
                    x: 0.7,
                    y: 0.0,
                    interpolation_mode: InterpolationMode::Quadratic,
                },
            ],
        );

        (
            ADSR {
                envelope_on: env1,
                envelope_off: env2,
                state: Off,
                level: 0.0,
                time_since_change: 0.0,
                ctrlport: b.add_rx(),
                cv_in: InPort::new(),
                output: OutPort { cable: None },
            },
            b,
        )
    }
    fn run(&mut self) -> () {
        if let Ok(message) = self.ctrlport.try_recv() {
            match message {
                Start(level) => {
                    self.state = On;
                    self.level = level;
                    self.time_since_change = 0.0;
                }
                Stop => {
                    self.state = Off;
                    self.time_since_change = 0.0;
                }
                SetAttack(a) => self.envelope_on.points[1].x = a,
                SetDecay(d) => self.envelope_on.points[2].x = d + self.envelope_on.points[1].x,
                SetSustain(s) => {
                    self.envelope_on.points[2].y = s;
                    self.envelope_off.points[0].y = s
                }
                SetRelease(r) => self.envelope_off.points[1].x = r,
            }
        }

        self.output.push(Sample {
            left: self.level *
                (match self.state {
                     On => &self.envelope_on,
                     Off => &self.envelope_off,
                 }).sample(self.time_since_change),
            right: 0.0,
        });

        self.time_since_change += 1.0 / SAMPLERATE;
    }
}
