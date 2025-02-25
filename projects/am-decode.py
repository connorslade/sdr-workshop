import math

from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

STATION = 499.996e6
OFFSET = 100_000
RECORD_TIME = 3
GAIN = 1.0

sdr = RtlSdr()
sdr.sample_rate = 250_000
sdr.center_freq = STATION + OFFSET
sdr.gain = 192

last_t = 0

def process_samples(samples):
    global last_t

    t = np.arange(len(samples)) / sdr.sample_rate + last_t
    samples *= np.exp(2j * np.pi * OFFSET * t)
    last_t = t[-1]

    b, a = signal.butter(5, 5_000, "low", fs=sdr.sample_rate)
    samples = signal.lfilter(b, a, samples)

    return np.abs(samples) * GAIN

buffer = []
audio_rate = int(sdr.sample_rate)
while len(buffer) < RECORD_TIME * audio_rate:
    print(f"[*] Recording: {len(buffer) / audio_rate:.2f} s", end="\r")
    samples = sdr.read_samples(1 << 10)
    audio = process_samples(np.array(samples))
    buffer.extend(audio)

sdr.close()

print("\n[*] Writing to file")
wavfile.write("output.wav", audio_rate, np.array(buffer))
