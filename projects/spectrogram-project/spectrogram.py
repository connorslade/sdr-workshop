from rtlsdr import RtlSdr
import matplotlib.pyplot as plt
assert RtlSdr is not None

DURATION = 10

sdr = RtlSdr()
sdr.sample_rate = 2.048e6
sdr.center_freq = 100e6
sdr.gain = 71

sdr.read_samples(2048)

samples = []
while len(samples) < DURATION * sdr.sample_rate:
    print(f'Recording... {len(samples) / sdr.sample_rate}s', end='\r')
    samples.extend(sdr.read_samples(2048))

plt.specgram(samples, Fs=sdr.sample_rate, Fc=int(sdr.center_freq))
plt.show()
