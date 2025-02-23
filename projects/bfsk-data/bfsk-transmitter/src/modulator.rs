use std::f32::consts::TAU;

use bitvec::vec::BitVec;
use libhackrf::exports::num_complex::Complex;

pub struct Modulator {
    data: BitVec<u8>,
    config: ModulatorConfiguration,
    sample: u64,
}

pub struct ModulatorConfiguration {
    pub symbol_duration: u32,
    pub sample_rate: u32,
    pub frequency_offset: f32,
}

impl Modulator {
    pub fn new(data: BitVec<u8>, config: ModulatorConfiguration) -> Self {
        Self {
            data,
            config,
            sample: 0,
        }
    }

    pub fn sample(&mut self) -> Complex<f32> {
        let idx = (self.sample / self.config.symbol_duration as u64) as usize;
        let symbol = self.data[idx % self.data.len()];
        self.sample += 1;

        let t = (self.sample as f32 / self.config.sample_rate as f32).fract();
        let f = self.config.frequency_offset * (symbol as u8 as f32 * 2.0 - 1.0);

        (Complex::i() * TAU * t * f).exp()
    }
}
