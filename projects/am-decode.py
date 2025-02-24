import math

from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

STATION = 499.998e6
RECORD_TIME = 3
GAIN = 1.0

SAMPLE_RATE = 250_000
RF_GAIN = 19.2

last_t = 0

def process_samples(samples):
    global last_t

    b, a = signal.butter(5, 10_000, "low", fs=sdr.sample_rate)
    t = (np.arange(len(samples)) / sdr.sample_rate) + last_t
    last_t = t[-1]

    samples *= np.exp(2j * np.pi * 100_000 * t)
    samples = signal.lfilter(b, a, samples)
    # samples = signal.decimate(samples, 5)
    return np.abs(samples) * GAIN


if __name__ == "__main__":
    sdr = RtlSdr()
    sdr.sample_rate = SAMPLE_RATE
    sdr.center_freq = STATION + 100_000
    sdr.gain = RF_GAIN

    audio_rate = int(sdr.sample_rate)# / 5)

    sdr.read_samples(2048)

    buffer = []
    while len(buffer) < RECORD_TIME * audio_rate:
        print(f"[*] Recording: {len(buffer) / audio_rate:.2f} s", end="\r")
        samples = sdr.read_samples(1 << 10)
        audio = process_samples(np.array(samples))
        buffer.extend(audio)

    sdr.close()

    print("\n[*] Writing to file")
    wavfile.write("output.wav", audio_rate, np.array(buffer))
