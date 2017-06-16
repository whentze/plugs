use types::*;
use consts::*;
use portaudio as pa;
use bus::Bus;

pub struct PAOut {
    pub input: InPort<Sample>,
    outstream: pa::Stream<pa::Blocking<pa::stream::Buffer>, pa::Output<f32>>,
}

impl super::Plugable for PAOut {
    type Message = ();
    fn init() -> (Self, Bus<()>) {
        let pa = pa::PortAudio::new().unwrap();
        let def_output = pa.default_output_device().unwrap();

        let output_params = pa::StreamParameters::<f32>::new(def_output, 2, true, 0.02);
        let settings = pa::stream::OutputSettings::new(output_params, SAMPLERATE as f64, 256);

        let mut stream = pa.open_blocking_stream(settings).unwrap();
        stream.start().unwrap();
        (
            PAOut {
                input: InPort::new(),
                outstream: stream,
            },
            Bus::new(0),
        )
    }

    fn run(&mut self) -> () {
        let inp = &mut self.input;
        let num_frames = 32;
        let res = self.outstream.write(
            num_frames as u32,
            |output| for i in 0..num_frames {
                let s = inp.pull().unwrap();
                output[2 * i] = s.left;
                output[2 * i + 1] = s.right;
            },
        );
        match res {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}

unsafe impl Send for PAOut {}
