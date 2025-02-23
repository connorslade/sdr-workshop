use bitvec::vec::BitVec;
use libhackrf::exports::num_complex::Complex;

pub struct Modulator {
    data: BitVec<u8>,
    symbol_duration: u32,
    sample: u64,
}

impl Modulator {
    pub fn new(sample_rate: u32, symbol_duration: f32, data: BitVec<u8>) -> Self {
        Self {
            data,
            symbol_duration: (sample_rate as f32 * symbol_duration) as u32,
            sample: 0,
        }
    }

    pub fn sample(&mut self) -> Complex<f32> {
        let symbol = self.data[(self.sample / self.symbol_duration as u64) as usize];
        self.sample += 1;

        let in_phase = if symbol { 1.0 } else { -1.0 };
        Complex::new(in_phase, 0.0)
    }
}
