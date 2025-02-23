use std::f32::consts::TAU;

use bitvec::{order::Msb0, vec::BitVec};
use libhackrf::exports::num_complex::Complex;

pub struct Modulator {
    data: BitVec<u8, Msb0>,
    config: ModulatorConfiguration,
    sample: u64,
}

pub struct ModulatorConfiguration {
    pub symbol_duration: u32,
    pub sample_rate: u32,
    pub frequency_offset: f32,
    pub repeat: bool,
}

impl Modulator {
    pub fn new(data: BitVec<u8, Msb0>, config: ModulatorConfiguration) -> Self {
        Self {
            data,
            config,
            sample: 0,
        }
    }

    pub fn done(&self) -> bool {
        let idx = self.sample / self.config.symbol_duration as u64;
        !self.config.repeat && idx >= self.data.len() as u64
    }

    pub fn sample(&mut self) -> Complex<f32> {
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
