from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal
assert RtlSdr is not None

BANDWIDTH = 50_000 / 2
RECORD_TIME = 10 # seconds
OFFSET = 100_000
GAIN = 1.0

sdr = RtlSdr()
sdr.sample_rate = 250_000
sdr.center_freq = 100_000_000 + OFFSET
sdr.gain = sdr.gain_values[-1]

def offset(samples, frequency):
    t = np.arange(len(samples)) / sdr.sample_rate
    return samples * np.exp(2j * np.pi * frequency * t)

def process_samples(samples):
    samples = offset(samples, OFFSET)

    b, a = signal.butter(5, BANDWIDTH, "low", fs=sdr.sample_rate)
    samples = signal.lfilter(b, a, samples)

    a, b = samples[:-1], samples[1:]
    angular_frequency = np.angle(b / a)

    return angular_frequency * GAIN

buffer = []
while len(buffer) < RECORD_TIME * sdr.sample_rate:
    print(f"[*] Recording: {len(buffer) / sdr.sample_rate:.2f} s", end="\r")
    samples = sdr.read_samples(2048)
    audio = process_samples(samples)
    buffer.extend(audio)

sdr.close()

print("\n[*] Writing to file")
AUDIO_SAMPLE_RATE = 44_100
buffer = signal.resample(buffer, int(len(buffer) * AUDIO_SAMPLE_RATE / sdr.sample_rate))
wavfile.write("output.wav", AUDIO_SAMPLE_RATE, (np.array(buffer) * 32767).astype(np.int16))
