use std::{cell::UnsafeCell, thread};

use anyhow::Result;
use bitvec::view::AsBits;
use libhackrf::{util::ToComplexI8, HackRf};
use modulator::Modulator;

const SAMPLE_RATE: u32 = 2_000_000;
const FREQUENCY: u64 = 100_000_000;
const GAIN: u32 = 47;
const SYMBOL_DURATION: f32 = 0.1;

mod modulator;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(SAMPLE_RATE)?;
    hackrf.set_freq(FREQUENCY)?;
    hackrf.set_txvga_gain(GAIN)?;

    let data = b"Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!".to_vec().as_bits().to_owned();
    let modulator = UnsafeCell::new(Modulator::new(SAMPLE_RATE, SYMBOL_DURATION, data));

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
