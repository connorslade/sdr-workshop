use std::ffi::OsString;

use clap::Parser;

use crate::modulator::ModulatorConfiguration;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Sample rate in Hz.
    #[arg(long, short, default_value_t = 2_000_000)]
    pub sample_rate: u32,
    /// Center frequency in Hz.
    #[arg(long, short, default_value_t = 200_000_000)]
    pub frequency: u64,
    /// Offset from the center freqnency for the two symbols, in Hz.
    #[arg(long, short, default_value_t = 1000.0)]
    pub offset: f32,
    /// Gain in dB, ranges from 0-47.
    #[arg(long, short, default_value_t = 47)]
    pub gain: u32,
    /// Baud (bits per second).
    #[arg(long, short, default_value_t = 10)]
    pub baud: u32,
    /// After transmission of all messages ends, loop through again.
    #[arg(long, short, default_value_t = true)]
    pub repeat: bool,
    #[arg(long, short, default_value_t = false)]
    pub no_transmission_flags: bool,

    /// Message to send. Multiple can be supplied.
    #[arg(default_value = "Hello World!")]
    pub messages: Vec<OsString>,
}

impl Args {
    pub fn get_config(&self) -> ModulatorConfiguration {
        ModulatorConfiguration {
            symbol_duration: self.sample_rate / self.baud,
            sample_rate: self.sample_rate,
            frequency_offset: self.offset,
            repeat: self.repeat,
        }
    }
}
