use std::{fs::File, io::BufReader};

use hound::{SampleFormat, WavReader, WavSpec};
use libhackrf::exports::num_complex::Complex;

use crate::{
    args::{Args, AudioArgs},
    modulate::Modulator,
};

use super::modulator::FmModulator;

pub struct AudioPlayer {
    modulator: FmModulator<'static>,
    sample_rate: u32,
    bandwidth: f32,

    songs: Vec<(WavSpec, &'static [f32])>,
    song_idx: usize,
}

impl AudioPlayer {
    pub fn new(args: &Args, audio: &AudioArgs) -> Self {
        let mut songs = Vec::new();
        for song in &audio.songs {
            let wav = WavReader::open(song).unwrap();
            let spec = wav.spec();
            let samples: &'static _ = Box::leak(get_float_samples(wav).into_boxed_slice());

            songs.push((spec, samples));
        }

        Self {
            modulator: FmModulator::empty(),
            sample_rate: args.sample_rate,
            bandwidth: audio.bandwidth,
            songs,
            song_idx: 0,
        }
    }
}

impl Modulator for AudioPlayer {
    fn sample(&mut self) -> Complex<f32> {
        if self.modulator.is_empty() {
            let (spec, samples) = self.songs[self.song_idx];
            self.modulator = FmModulator::new(self.sample_rate, self.bandwidth, spec, samples);
            self.song_idx = (self.song_idx + 1) % self.songs.len();
        }

        self.modulator.sample()
    }
}

fn get_float_samples(wav: WavReader<BufReader<File>>) -> Vec<f32> {
    let channels = wav.spec().channels as usize;
    match wav.spec().sample_format {
        SampleFormat::Float => wav
            .into_samples::<f32>()
            .map(|x| x.unwrap())
            .step_by(channels)
            .collect::<Vec<_>>(),
        SampleFormat::Int => {
            let max = (1u32 << (wav.spec().bits_per_sample - 1)) as f32;
            wav.into_samples::<i32>()
                .map(|x| x.unwrap() as f32 / max)
                .step_by(channels)
                .collect::<Vec<_>>()
        }
    }
}
