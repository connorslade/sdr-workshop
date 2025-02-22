import math

from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

STATION = 100.0e6
RECORD_TIME = 10 # seconds
GAIN = 0.2
RF_GAIN = 300

def normalize_angle(angle):
    return np.arctan2(np.sin(angle), np.cos(angle))


def process_samples(samples):
    b, a = signal.butter(5, 100_000, "low", fs=sdr.sample_rate)
    filtered_samples = signal.lfilter(b, a, samples)

    a, b = samples[:-1], samples[1:]
    angle = np.angle(b) - np.angle(a)
    # range of [-2pi, 2pi]?
    angle = normalize_angle(angle)
    return angle * GAIN


if __name__ == "__main__":
    sdr = RtlSdr()
    sdr.sample_rate = 250_000
    sdr.center_freq = STATION
    sdr.gain = RF_GAIN

    sdr.read_samples(2048)

    buffer = []
    while len(buffer) < RECORD_TIME * sdr.sample_rate:
        print(f"[*] Recording: {len(buffer) / sdr.sample_rate:.2f} s", end="\r")
        samples = sdr.read_samples(1 << 10)
        audio = process_samples(np.array(samples))
        buffer.extend(audio)

    sdr.close()

    print("\n[*] Writing to file")
    wavfile.write("output.wav", int(sdr.sample_rate), np.array(buffer))
