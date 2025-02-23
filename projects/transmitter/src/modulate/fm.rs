use std::f32::consts::TAU;

use hound::WavSpec;
use libhackrf::exports::num_complex::Complex;

use super::{IntoAudioModulator, Modulator};

pub struct FmModulator<'a> {
    samples: &'a [f32],
    sample_idx: usize,

    audio_sample_rate: u32,
    config: FmModulatorConfiguration,

    i: u64,
    phase: f32,
    sample: f32,
    next_sample: f32,
}

#[derive(Clone, Copy)]
pub struct FmModulatorConfiguration {
    pub sample_rate: u64,
    pub bandwidth: f32,
}

impl<'a> IntoAudioModulator<'a> for FmModulatorConfiguration {
    type Modulator = FmModulator<'a>;

    fn create(&self, spec: WavSpec, samples: &'a [f32]) -> Self::Modulator {
        FmModulator {
            samples,
            sample_idx: 0,
            audio_sample_rate: spec.sample_rate,
            config: *self,
            i: 0,
            phase: 0.0,
            sample: 0.0,
            next_sample: 0.0,
        }
    }
}

impl Modulator for FmModulator<'_> {
    fn done(&self) -> bool {
        self.sample_idx >= self.samples.len()
    }

    fn sample(&mut self) -> Complex<f32> {
        let rate = self.config.sample_rate / self.audio_sample_rate as u64;

        let sample = self.i % rate;
        if sample == 0 {
            self.sample = self.next_sample;
            self.next_sample = self.samples[self.sample_idx];
            self.sample_idx += 1;
        }

        self.i += 1;

        let t = sample as f32 / rate as f32;
        let deviation = lerp(self.sample, self.next_sample, t) * self.config.bandwidth;
        self.phase += TAU * deviation / self.config.sample_rate as f32;

        (Complex::i() * self.phase).exp()
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
