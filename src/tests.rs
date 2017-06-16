use std::thread;
use test::Bencher;
use bus::Bus;
use types::*;
use plugable::*;


pub struct Eater {
    pub input: InPort<Sample>,
}

impl Plugable for Eater {
    type Message = ();
    fn init() -> (Self, Bus<()>) {
        (Eater {
            input: InPort::new(),
        }, Bus::new(0))
    }
    fn run(&mut self) -> () {
        self.input.pull();
    }
}

#[bench]
fn plug_vca(b: &mut Bencher) -> () {
    let (mut sil1, _) = Silence::init();
    let (mut sil2, _) = Silence::init();
    let (mut vca, _) = VCA::init();
    let (mut eat, _) = Eater::init();

    sil1.output.connect(&mut vca.signal_in);
    sil2.output.connect(&mut vca.gain_in);
    vca.output.connect(&mut eat.input);
    thread::spawn(move || loop {
        sil1.run()
    });
    thread::spawn(move || loop {
        sil2.run()
    });
    thread::spawn(move || loop {
        vca.run()
    });
    b.iter(|| { for _ in 0..48000 { eat.run(); }});
}

#[bench]
fn plug_oscillator(b: &mut Bencher) -> () {
    let (mut osci, _) = Oscillator::init();
    let (mut eat, _) = Eater::init();

    osci.output.connect(&mut eat.input);
    thread::spawn(move || loop {
        osci.run()
    });
    b.iter(|| { for _ in 0..48000 { eat.run(); }});
}

#[bench]
fn plug_adsr(b: &mut Bencher) -> () {
    use plugable::adsr::Message::*;
    let (mut adsr, mut adsr_ctl) = ADSR::init();
    let (mut eat, _) = Eater::init();
    adsr_ctl.broadcast(Start(1.0));

    adsr.output.connect(&mut eat.input);
    thread::spawn(move || loop {
        adsr.run()
    });
    b.iter(|| { for _ in 0..48000 { eat.run(); }});
}

#[bench]
fn throughput(b: &mut Bencher) -> () {
    let (mut cou, _) = Silence::init();
    let (mut ide, _) = Identity::init();
    let (mut eat, _) = Eater::init();

    cou.output.connect(&mut ide.input);
    ide.output.connect(&mut eat.input);
    thread::spawn(move || loop {
        cou.run()
    });
    thread::spawn(move || loop {
        ide.run()
    });
    b.iter(|| { for _ in 0..48000 { eat.run(); }});
}

#[bench]
fn chain_parallel(b: &mut Bencher) -> () {
    let (mut sil, _) = Silence::init();
    let (mut ide1, _) = Identity::init();
    let (mut ide2, _) = Identity::init();
    let (mut ide3, _) = Identity::init();
    let (mut ide4, _) = Identity::init();
    let (mut eat, _) = Eater::init();

    sil.output.connect(&mut ide1.input);
    ide1.output.connect(&mut ide2.input);
    ide2.output.connect(&mut ide3.input);
    ide3.output.connect(&mut ide4.input);
    ide4.output.connect(&mut eat.input);
    thread::spawn(move || loop {
        sil.run()
    });
    thread::spawn(move || loop {
        ide1.run()
    });
    thread::spawn(move || loop {
        ide2.run()
    });
    thread::spawn(move || loop {
        ide3.run()
    });
    thread::spawn(move || loop {
        ide4.run()
    });
    b.iter(|| { for _ in 0..48000 { eat.run(); }});
}

#[bench]
fn chain_single(b: &mut Bencher) -> () {
    let (mut sil, _) = Silence::init();
    let (mut ide1, _) = Identity::init();
    let (mut ide2, _) = Identity::init();
    let (mut ide3, _) = Identity::init();
    let (mut ide4, _) = Identity::init();
    let (mut eat, _) = Eater::init();

    sil.output.connect(&mut ide1.input);
    ide1.output.connect(&mut ide2.input);
    ide2.output.connect(&mut ide3.input);
    ide3.output.connect(&mut ide4.input);
    ide4.output.connect(&mut eat.input);
    thread::spawn(move || loop {
        sil.run()
    });
    thread::spawn(move || loop {
        ide1.run();
        ide2.run();
        ide3.run();
        ide4.run();
    });
    b.iter(|| { for _ in 0..48000 { eat.run(); }});
}