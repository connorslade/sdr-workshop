use libhackrf::exports::num_complex::Complex;

pub mod audio;
pub mod bfsk;

pub trait Modulator {
    fn sample(&mut self) -> Complex<f32>;
    fn done(&self) -> bool {
        false
    }
}
