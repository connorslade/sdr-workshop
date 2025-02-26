import math
import sys

from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

# Radio config
sdr = RtlSdr()
sdr.sample_rate = 250_000
sdr.center_freq = 99_997_500
sdr.gain = 9.9

# Decoder config
BAUD = 100
CHUNK_SIZE = 512
CHUNKS_PER_SYMBOL = sdr.sample_rate / BAUD / CHUNK_SIZE
print(f'Chunks Per Symbol: {CHUNKS_PER_SYMBOL}')

def process_samples(samples):
    b, a = signal.butter(5, 2_000, "low", fs=sdr.sample_rate)
    samples = signal.lfilter(b, a, samples)

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

    print("1" if bit else "0", end="")
    sys.stdout.flush()

    if not running and bits_to_bytes(bits[-8:]) == b'\x02':
        print("«", end="")
        running = True
        bits.clear()
    elif running and bits_to_bytes(bits[-16:]) == b'\x03\x03':
        print("»", end="")
        print(bits_to_bytes(bits[:-16]))
        running = False
        bits.clear()

last, count = 0, 0

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
