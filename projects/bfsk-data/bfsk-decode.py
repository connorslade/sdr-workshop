import math
import sys

from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

# Radio config
SAMPLE_RATE = 250_000
STATION = 199.995e6
RF_GAIN = 9.9

# Decoder config
BAUD = 10
CHUNK_SIZE = 256
CHUNKS_PER_SYMBOL = SAMPLE_RATE / BAUD / CHUNK_SIZE


def process_samples(samples):
    b, a = signal.butter(5, 2_000, "low", fs=sdr.sample_rate)
    filtered_samples = signal.lfilter(b, a, samples)

    a, b = samples[:-1], samples[1:]
    angle = np.angle(b * np.conj(a))
    return angle


def bits_to_bytes(bits):
    out = []

    for bits in zip(*[iter(bits)] * 8):
        byte = 0
        for bit in bits:
            byte = (byte << 1) | bit
        out.append(byte)

    return bytes(out)

bits = []
running = False

def got_bit(bit):
    global bits, running

    bits.append(bit)

    print(("1" if bit else "0"), end="")
    sys.stdout.flush()

    last_byte = bits_to_bytes(bits[-8:])
    if not running and last_byte == b'\x02':
        print("«", end="")
        running = True
        bits.clear()
    elif running and last_byte == b'\x03':
        print("»", end="")
        if running: print(bits_to_bytes(bits[:-8]))
        running = False
        bits.clear()

if __name__ == "__main__":
    sdr = RtlSdr()
    sdr.sample_rate = SAMPLE_RATE
    sdr.center_freq = STATION
    sdr.gain = RF_GAIN

    last = 0
    count = 0

    while True:
        samples = np.array(sdr.read_samples(CHUNK_SIZE))
        angles = process_samples(samples)
        bit = np.mean(angles) > 0.0

        if bit != last:
            count = round(count / CHUNKS_PER_SYMBOL)
            for _ in range(count): got_bit(last)

            last = bit
            count = 0
        count += 1

    sdr.close()
