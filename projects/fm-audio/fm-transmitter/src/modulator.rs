use std::f32::consts::TAU;

use hound::WavSpec;
use num_complex::Complex;

pub struct Modulator<'a> {
    samples: &'a [f32],
    sample_idx: usize,

    audio_sample_rate: u32,
    sample_rate: u64,
    bandwidth: f32,

    i: u64,
    phase: f32,
    sample: f32,
    next_sample: f32,
}

impl<'a> Modulator<'a> {
    pub fn empty() -> Self {
        Self {
            samples: &[],
            sample_idx: 0,

            audio_sample_rate: 0,
            sample_rate: 0,
            bandwidth: 0.0,

            i: 0,
            phase: 0.0,
            sample: 0.0,
            next_sample: 0.0,
        }
    }

    pub fn new(sample_rate: u32, bandwidth: f32, spec: WavSpec, samples: &'a [f32]) -> Self {
        Self {
            samples,
            sample_idx: 0,

            audio_sample_rate: spec.sample_rate,
            sample_rate: sample_rate as _,
            bandwidth,

            i: 0,
            phase: 0.0,
            sample: 0.0,
            next_sample: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sample_idx >= self.samples.len()
    }

    pub fn sample(&mut self) -> Complex<f32> {
        let rate = self.sample_rate / self.audio_sample_rate as u64;

        let sample = self.i % rate;
        if sample == 0 {
            self.sample = self.next_sample;
            self.next_sample = self.samples[self.sample_idx];
            self.sample_idx += 1;
        }

        self.i += 1;

        let t = sample as f32 / rate as f32;
        let deviation = lerp(self.sample, self.next_sample, t) * self.bandwidth;
        self.phase += TAU * deviation / self.sample_rate as f32;

        (Complex::i() * self.phase).exp()
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
