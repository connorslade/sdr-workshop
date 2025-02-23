use std::{fs::File, io::BufReader, path::PathBuf};

use hound::{SampleFormat, WavReader, WavSpec};
use libhackrf::exports::num_complex::Complex;

use crate::modulate::Modulator;

use super::IntoAudioModulator;

pub struct AudioPlayer<Config: IntoAudioModulator<'static>> {
    modulator: Config::Modulator,
    config: Config,

    songs: Vec<(WavSpec, &'static [f32])>,
    song_idx: usize,
}

impl<Config: IntoAudioModulator<'static>> AudioPlayer<Config> {
    pub fn new(config: Config, paths: &[PathBuf]) -> Self {
        let mut songs = Vec::new();
        for song in paths {
            let wav = WavReader::open(song).unwrap();
            let spec = wav.spec();
            let samples: &'static _ = Box::leak(get_float_samples(wav).into_boxed_slice());

            songs.push((spec, samples));
        }

        assert!(!songs.is_empty(), "You must supply at least one song.");
        let (spec, samples) = &songs[0];

        Self {
            modulator: config.create(*spec, samples),
            config,

            songs,
            song_idx: 0,
        }
    }
}

impl<Config: IntoAudioModulator<'static>> Modulator for AudioPlayer<Config> {
    fn sample(&mut self) -> Complex<f32> {
        if self.modulator.done() {
            self.song_idx = (self.song_idx + 1) % self.songs.len();
            let (spec, samples) = self.songs[self.song_idx];
            self.modulator = self.config.create(spec, samples);
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
