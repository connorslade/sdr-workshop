use std::{f32::consts::TAU, iter, os::unix::ffi::OsStrExt};

use bitvec::{order::Msb0, vec::BitVec, view::AsBits};
use libhackrf::exports::num_complex::Complex;

use crate::args::{Args, BfskArgs};

use super::Modulator;

pub struct BfskModulator {
    data: BitVec<u8, Msb0>,
    config: BfskModulatorConfiguration,
    sample: u64,
}

pub struct BfskModulatorConfiguration {
    pub symbol_duration: u32,
    pub sample_rate: u32,
    pub frequency_offset: f32,
    pub repeat: bool,
}

impl BfskModulator {
    pub fn new(args: &Args, bfsk: &BfskArgs) -> Self {
        let mut data = Vec::new();

        for message in &bfsk.messages {
            if !bfsk.no_transmission_flags {
                data.push(0x02)
            }
            data.extend_from_slice(message.as_bytes());
            if !bfsk.no_transmission_flags {
                data.extend(iter::repeat_n(0x03, 2));
            }
        }

        Self {
            data: data.as_bits().to_owned(),
            config: BfskModulatorConfiguration {
                symbol_duration: args.sample_rate / bfsk.baud,
                sample_rate: args.sample_rate,
                frequency_offset: bfsk.offset,
                repeat: bfsk.repeat,
            },
            sample: 0,
        }
    }
}

impl Modulator for BfskModulator {
    fn done(&self) -> bool {
        let idx = self.sample / self.config.symbol_duration as u64;
        !self.config.repeat && idx >= self.data.len() as u64
    }

    fn sample(&mut self) -> Complex<f32> {
        let idx = (self.sample / self.config.symbol_duration as u64) as usize;
        if idx >= self.data.len() && !self.config.repeat {
            return Complex::ZERO;
        }

        let symbol = self.data[idx % self.data.len()];
        let t = (self.sample as f32 / self.config.sample_rate as f32).fract();
        self.sample += 1;

        let f = self.config.frequency_offset * (symbol as u8 as f32 * 2.0 - 1.0);
        (Complex::i() * TAU * t * f).exp()
    }
}
