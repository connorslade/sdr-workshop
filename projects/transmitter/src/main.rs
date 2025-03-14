use std::{cell::UnsafeCell, process, thread};

use anyhow::Result;
use clap::Parser;
use libhackrf::{util::ToComplexI8, HackRf};

mod args;
mod modulate;
use args::{Args, Command};
use modulate::{
    am::AmModulatorConfiguration, bfsk::BfskModulator, fm::FmModulatorConfiguration,
    player::AudioPlayer, Modulator,
};

fn main() -> Result<()> {
    let args = Args::parse();

    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(args.sample_rate)?;
    hackrf.set_freq(args.frequency)?;
    hackrf.set_txvga_gain(args.gain)?;

    let modulator: Box<dyn Modulator> = match &args.command {
        Command::Fm(fm) => Box::new(AudioPlayer::new(
            FmModulatorConfiguration {
                sample_rate: args.sample_rate as _,
                bandwidth: fm.bandwidth,
            },
            &fm.songs,
        )),
        Command::Am(am) => Box::new(AudioPlayer::new(
            AmModulatorConfiguration {
                sample_rate: args.sample_rate as _,
                modulation: am.modulation,
            },
            &am.songs,
        )),
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
                process::exit(0);
            }

            samples
                .iter_mut()
                .for_each(|x| *x = modulator.sample().to_i8());
        },
        modulator,
    )?;

    ctrlc::set_handler(move || {
        let _ = hackrf.stop_tx();
        process::exit(0);
    })?;

    loop {
        thread::park();
    }
}
