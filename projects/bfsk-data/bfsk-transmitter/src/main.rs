use std::{cell::UnsafeCell, thread};

use anyhow::Result;
use bitvec::view::AsBits;
use libhackrf::{util::ToComplexI8, HackRf};
use modulator::{Modulator, ModulatorConfiguration};

const SAMPLE_RATE: u32 = 2_000_000;
const FREQUENCY: u64 = 200_000_000;
const GAIN: u32 = 47;
const BAUD: u32 = 10;

mod modulator;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(SAMPLE_RATE)?;
    hackrf.set_freq(FREQUENCY)?;
    hackrf.set_txvga_gain(GAIN)?;

    let data = b"\x02Hello World!\x03".to_vec().as_bits().to_owned();
    let modulator = UnsafeCell::new(Modulator::new(
        data,
        ModulatorConfiguration {
            symbol_duration: SAMPLE_RATE / BAUD,
            sample_rate: SAMPLE_RATE,
            frequency_offset: 1000.0,
        },
    ));

    hackrf.start_tx(
        |_hackrf, samples, user| {
            let modulator = user.downcast_ref::<UnsafeCell<Modulator>>().unwrap();
            let modulator = unsafe { &mut *modulator.get() };

            samples
                .iter_mut()
                .for_each(|x| *x = modulator.sample().to_i8());
        },
        modulator,
    )?;

    loop {
        thread::park();
    }
}
