from rtlsdr import RtlSdr
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
assert RtlSdr is not None

FFT_SIZE = 1024
DURATION = 10

sdr = RtlSdr()
sdr.sample_rate = 2.048e6
sdr.center_freq = 100e6
sdr.gain = 192

waterfall_height = sdr.sample_rate * DURATION / FFT_SIZE

sdr.read_samples(2048)

waterfall = []
while len(waterfall) < waterfall_height:
    print(f'Recording... {len(waterfall) * FFT_SIZE / sdr.sample_rate}s', end='\r')

    samples = sdr.read_samples(FFT_SIZE) * np.hanning(FFT_SIZE)
    fft = np.abs(np.fft.fftshift(np.fft.fft(samples))) ** 0.25
    waterfall.append(fft)

def x_tick_formatter(x, pos):
    # Remember that the frequency ranges from -fs/2 to fs/2.
    x_freq = (x - FFT_SIZE / 2) * sdr.sample_rate / FFT_SIZE
    # Where the center frequency is what we set on the tuner.
    # The division by 1e6 is to convert Hz to MHz.
    freq = (x_freq + sdr.center_freq) / 1e6
    return f"{freq:.2f}"

def y_tick_formatter(y, pos):
    # We know how many ticks went into each row of the waterfall,
    # and we know how much time each of those samples represent.
    time = (len(waterfall) - y) * FFT_SIZE / sdr.sample_rate
    return f"{time:.2f}"

fig, ax = plt.subplots()
ax.set_xlabel('Frequency (MHz)')
ax.set_ylabel('Time (s)')
ax.set_title('Waterfall Plot')
ax.xaxis.set_major_formatter(mticker.FuncFormatter(x_tick_formatter))
ax.yaxis.set_major_formatter(mticker.FuncFormatter(y_tick_formatter))

plt.imshow(waterfall, aspect='auto', cmap='plasma')
plt.show()
