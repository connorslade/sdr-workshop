use hound::WavSpec;
use libhackrf::exports::num_complex::Complex;

use super::{IntoAudioModulator, Modulator};

pub struct AmModulator<'a> {
    samples: &'a [f32],
    audio_sample_rate: u32,
    config: AmModulatorConfiguration,
    i: u64,
}

#[derive(Clone, Copy)]
pub struct AmModulatorConfiguration {
    pub sample_rate: u64,
}

impl<'a> IntoAudioModulator<'a> for AmModulatorConfiguration {
    type Modulator = AmModulator<'a>;

    fn create(&self, spec: WavSpec, samples: &'a [f32]) -> Self::Modulator {
        AmModulator {
            samples,
            audio_sample_rate: spec.sample_rate,
            config: *self,
            i: 0,
        }
    }
}

impl Modulator for AmModulator<'_> {
    fn done(&self) -> bool {
        let rate = self.config.sample_rate as f64 / self.audio_sample_rate as f64;
        (self.i as f64 / rate).round() as usize >= self.samples.len()
    }

    fn sample(&mut self) -> Complex<f32> {
        let rate = self.config.sample_rate as f64 / self.audio_sample_rate as f64;
        let sample = self.samples[(self.i as f64 / rate).round() as usize];
        self.i += 1;

        sample.into()
    }
}
