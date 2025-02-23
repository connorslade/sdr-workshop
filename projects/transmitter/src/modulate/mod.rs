use hound::WavSpec;
use libhackrf::exports::num_complex::Complex;

pub mod am;
pub mod bfsk;
pub mod fm;
pub mod player;

pub trait Modulator {
    fn sample(&mut self) -> Complex<f32>;
    fn done(&self) -> bool {
        false
    }
}

pub trait IntoAudioModulator<'a> {
    type Modulator: Modulator;

    fn create(&self, spec: WavSpec, samples: &'a [f32]) -> Self::Modulator;
}
