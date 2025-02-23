use std::{ffi::OsString, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Sample rate in Hz.
    #[arg(long, short, default_value_t = 2_000_000)]
    pub sample_rate: u32,
    /// Center frequency in Hz.
    #[arg(long, short, default_value_t = 200_000_000)]
    pub frequency: u64,
    /// Gain in dB, ranges from 0-47.
    #[arg(long, short, default_value_t = 47)]
    pub gain: u32,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Transmit audio with AM.
    Am(AmArgs),
    /// Transmit audio with FM.
    Fm(FmArgs),
    /// Transmit binary data with frequency shift keying.
    Bfsk(BfskArgs),
}

#[derive(Parser)]
pub struct AmArgs {
    /// Path to .wav files
    pub songs: Vec<PathBuf>,
}

#[derive(Parser)]
pub struct FmArgs {
    /// Bandwidth, in Hz.
    #[arg(long, short, default_value_t = 19_000.0)]
    pub bandwidth: f32,
    /// Path to .wav files
    pub songs: Vec<PathBuf>,
}

#[derive(Parser)]
pub struct BfskArgs {
    /// Offset from the center freqnency for the two symbols, in Hz.
    #[arg(long, short, default_value_t = 1000.0)]
    pub offset: f32,
    /// Baud (bits per second).
    #[arg(long, short, default_value_t = 10)]
    pub baud: u32,
    /// After transmission of all messages ends, loop through again.
    #[arg(long, short, default_value_t = true)]
    pub repeat: bool,
    /// Don't transmit the 0x02 and 0x03 bytes denoting the start and end of a message
    #[arg(long, short, default_value_t = false)]
    pub no_transmission_flags: bool,

    /// Message to send. Multiple can be supplied.
    #[arg(default_value = "Hello World!")]
    pub messages: Vec<OsString>,
}
