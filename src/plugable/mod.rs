pub mod pcmprinter;
pub mod identity;
pub mod silence;
pub mod pa_out;
pub mod adsr;
pub mod vca;
pub mod oscillator;

use bus::Bus;
pub trait Plugable {
    type Message: Clone;
    fn init() -> (Self, Bus<Self::Message>)
    where
        Self: Sized;
    fn run(&mut self) -> ();
}

pub use self::pcmprinter::PCMPrinter;
pub use self::identity::Identity;
pub use self::silence::Silence;
pub use self::pa_out::PAOut;
pub use self::oscillator::Oscillator;
pub use self::adsr::ADSR;
pub use self::vca::VCA;
