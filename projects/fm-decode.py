import math

from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

BANDWIDTH = 25_000
RECORD_TIME = 10 # seconds
OFFSET = 100_000
GAIN = 0.7

sdr = RtlSdr()
sdr.sample_rate = 250_000
sdr.center_freq = 200_000_000 + OFFSET
sdr.gain = 'auto'

def process_samples(samples):
    t = np.arange(len(samples)) / sdr.sample_rate
    samples *= np.exp(2j * np.pi * OFFSET * t)
    last_t = t[-1]

    b, a = signal.butter(5, BANDWIDTH, "low", fs=sdr.sample_rate)
    samples = signal.lfilter(b, a, samples)

    a, b = samples[:-1], samples[1:]
    audio = np.angle(b * np.conj(a))

    return audio * GAIN

buffer = []
while len(buffer) < RECORD_TIME * sdr.sample_rate:
    print(f"[*] Recording: {len(buffer) / sdr.sample_rate:.2f} s", end="\r")
    samples = sdr.read_samples(1 << 10)
    audio = process_samples(np.array(samples))
    buffer.extend(audio)

sdr.close()

print("\n[*] Writing to file")
wavfile.write("output.wav", int(sdr.sample_rate), np.array(buffer))
