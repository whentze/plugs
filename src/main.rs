extern crate plugs;

use std::thread;

use plugs::plugable::{Plugable, Oscillator, ADSR, VCA, PAOut};

fn main() {
    use plugs::plugable::oscillator::Message::*;
    use plugs::plugable::oscillator::Waveform::*;
    use plugs::plugable::adsr::Message::*;

    let (mut lfo, mut lfo_ctl) = Oscillator::init();
    lfo_ctl.broadcast(SetWaveform(Sine));
    lfo_ctl.broadcast(SetLevel(0.0));

    let (mut osci, mut osci_ctl) = Oscillator::init();
    osci_ctl.broadcast(SetWaveform(Sine));

    let (mut adsr, mut adsr_ctl) = ADSR::init();
    let (mut vca, _) = VCA::init();
    let (mut out, _) = PAOut::init();
    lfo.output.connect(&mut osci.lfo_in);
    osci.output.connect(&mut vca.signal_in);
    adsr.output.connect(&mut vca.gain_in);
    vca.output.connect(&mut out.input);

    thread::spawn(move || loop {
        lfo.run()
    });
    thread::spawn(move || loop {
        osci.run()
    });
    thread::spawn(move || loop {
        adsr.run()
    });
    thread::spawn(move || loop {
        vca.run()
    });
    thread::spawn(move || loop {
        out.run()
    });

    // LFO demo
    osci_ctl.broadcast(SetFrequency(220.0 * f32::exp2(1.0/6.0)));
    adsr_ctl.broadcast(Start(1.0));
    lfo_ctl.broadcast(SetFrequency(0.0));
    thread::sleep(std::time::Duration::from_millis(1000));
    adsr_ctl.broadcast(Stop);
    thread::sleep(std::time::Duration::from_millis(1000));
}
