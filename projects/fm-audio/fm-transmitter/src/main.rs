use std::{cell::UnsafeCell, env, fs::File, io::BufReader, path::Path, thread};

use anyhow::Result;
use hound::{SampleFormat, WavReader, WavSpec};
use libhackrf::{util::ToComplexI8, HackRf};
use modulator::Modulator;

const SAMPLE_RATE: u32 = 2_000_000;
const FREQUENCY: u64 = 100_000_000;
const GAIN: u32 = 47;
const TX_BANDWIDTH: f32 = 19_000.0;

mod modulator;

struct State {
    modulator: Modulator<'static>,

    songs: Vec<(WavSpec, &'static [f32])>,
    song_idx: usize,
}

fn main() -> Result<()> {
    let songs = env::args()
        .skip(1)
        .map(|x| Path::new(&x).to_path_buf())
        .filter(|x| x.exists())
        .map(|x| {
            let wav = WavReader::open(x).unwrap();
            let spec = wav.spec();
            let samples: &'static _ = Box::leak(get_float_samples(wav).into_boxed_slice());

            (spec, samples)
        })
        .collect::<Vec<_>>();

    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(SAMPLE_RATE)?;
    hackrf.set_freq(FREQUENCY)?;
    hackrf.set_txvga_gain(GAIN)?;

    let audio = UnsafeCell::new(State {
        modulator: Modulator::empty(),
        songs,
        song_idx: 0,
    });
    hackrf.start_tx(
        |_hackrf, buffer, user| {
            let data = user.downcast_ref::<UnsafeCell<State>>().unwrap();
            let data = unsafe { &mut *data.get() };

            buffer.iter_mut().for_each(|x| {
                if data.modulator.is_empty() {
                    let (spec, samples) = data.songs[data.song_idx];
                    data.modulator = Modulator::new(SAMPLE_RATE, TX_BANDWIDTH, spec, samples);
                    data.song_idx = (data.song_idx + 1) % data.songs.len();
                }

                *x = data.modulator.sample().to_i8();
            });
        },
        audio,
    )?;

    loop {
        thread::park()
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
