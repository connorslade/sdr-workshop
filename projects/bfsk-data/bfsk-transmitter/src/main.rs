use std::{cell::UnsafeCell, iter, os::unix::ffi::OsStrExt, process, thread};

use anyhow::Result;
use bitvec::view::AsBits;
use clap::Parser;
use libhackrf::{util::ToComplexI8, HackRf};
use modulator::Modulator;

mod args;
mod modulator;
use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Transmitting {:?}", args.messages);

    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(args.sample_rate)?;
    hackrf.set_freq(args.frequency)?;
    hackrf.set_txvga_gain(args.gain)?;

    let mut data = Vec::new();

    for message in &args.messages {
        if !args.no_transmission_flags {
            data.push(0x02)
        }
        data.extend_from_slice(message.as_bytes());
        if !args.no_transmission_flags {
            data.extend(iter::repeat_n(0x03, 2));
        }
    }

    let modulator = UnsafeCell::new(Modulator::new(data.as_bits().to_owned(), args.get_config()));

    hackrf.start_tx(
        |_hackrf, samples, user| {
            let modulator = user.downcast_ref::<UnsafeCell<Modulator>>().unwrap();
            let modulator = unsafe { &mut *modulator.get() };

            if modulator.done() {
                process::abort();
            }

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
