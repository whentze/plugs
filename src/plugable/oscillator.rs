use std::f32::consts::PI;

use bus::{Bus, BusReader};

use types::*;
use consts::*;

#[derive(Debug, Clone, Copy)]
pub enum Waveform {
    Sine,
    Saw,
    Pulse(f32),
}
pub use self::Waveform::*;

#[derive(Clone, Copy)]
pub enum Message {
    SetWaveform(Waveform),
    SetFrequency(f32),
    SetLevel(f32),
    Reset,
}
use self::Message::*;

pub struct Oscillator {
    phase: f32,
    freq: f32,
    waveform: Waveform,
    pub ctrlport: BusReader<Message>,
    pub lfo_in: InPort<Sample>,
    pub output: OutPort<Sample>,
    pub level: f32,
}

impl super::Plugable for Oscillator {
    type Message = self::Message;
    fn init() -> (Self, Bus<Message>) {
        let mut b = Bus::new(100);
        (
            Oscillator {
                phase: 0.0,
                freq: 440.0,
                waveform: Waveform::Sine,
                ctrlport: b.add_rx(),
                lfo_in: InPort::new(),
                output: OutPort { cable: None },
                level: 1.0,
            },
            b,
        )
    }
    fn run(&mut self) -> () {
        if let Ok(message) = self.ctrlport.try_recv() {
            match message {
                SetWaveform(w) => self.waveform = w,
                SetFrequency(f) => self.freq = f,
                SetLevel(l) => self.level = l,
                Reset => self.phase = 0.0,
            }
        }
        let freq = if let Some(Sample { left: lfo, .. }) = self.lfo_in.pull() {
            self.freq * lfo.exp2()
        } else {
            self.freq
        };
        self.phase = (self.phase + 2.0 * PI * freq / SAMPLERATE) % (2.0 * PI);

        let amp = self.level *
            match self.waveform {
                Sine => self.phase.sin(),
                Saw => self.phase / (2.0 * PI),
                Pulse(l) => if self.phase < l * 2.0 * PI { 1.0 } else { 0.0 },
            };
        self.output.push(Sample {
            left: amp,
            right: amp,
        });
    }
}
