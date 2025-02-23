use std::{cell::UnsafeCell, process, thread};

use anyhow::Result;
use clap::Parser;
use libhackrf::{util::ToComplexI8, HackRf};

mod args;
mod modulate;
use args::{Args, Command};
use modulate::{audio::AudioPlayer, bfsk::BfskModulator, Modulator};

fn main() -> Result<()> {
    let args = Args::parse();

    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(args.sample_rate)?;
    hackrf.set_freq(args.frequency)?;
    hackrf.set_txvga_gain(args.gain)?;

    let modulator: Box<dyn Modulator> = match &args.command {
        Command::Audio(audio_args) => Box::new(AudioPlayer::new(&args, audio_args)),
        Command::Bfsk(bfsk_args) => Box::new(BfskModulator::new(&args, bfsk_args)),
    };
    let modulator = UnsafeCell::new(modulator);

    hackrf.start_tx(
        |_hackrf, samples, user| {
            let modulator = user
                .downcast_ref::<UnsafeCell<Box<dyn Modulator>>>()
                .unwrap();
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
