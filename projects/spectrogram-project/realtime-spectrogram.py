from rtlsdr import RtlSdr
import matplotlib.animation as animation
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
assert RtlSdr is not None

FFT_SIZE = 1024
WATERFALL_SIZE = 512
CHUNK_SIZE = 1024
CHUNKS_PER_FRAME = 100

sdr = RtlSdr()
sdr.sample_rate = 250_000
sdr.center_freq = 100e6
sdr.gain = 192

buffer = []
waterfall = []

fig, ax = plt.subplots()
fig.suptitle("Waterfall Plot")

def x_tick_formatter(x, pos):
    x_freq = (x - FFT_SIZE / 2) * sdr.sample_rate / FFT_SIZE
    freq = (x_freq + sdr.center_freq) / 1e6
    return f"{freq:.2f}"

def y_tick_formatter(y, pos):
    time = (len(waterfall) - y) * FFT_SIZE / sdr.sample_rate
    return f"{time:.2f}"

def update(frame):
    global buffer, waterfall

    for _ in range(CHUNKS_PER_FRAME):
        buffer.extend(sdr.read_samples(CHUNK_SIZE))

    while len(buffer) >= FFT_SIZE:
        samples = np.array(buffer[:FFT_SIZE])
        buffer = buffer[FFT_SIZE:]

        spectrum = np.abs(np.fft.fftshift(np.fft.fft(samples)))
        waterfall.append(spectrum)
        if len(waterfall) > WATERFALL_SIZE: waterfall.pop(0)

    if len(waterfall) > 0:
        ax.clear()
        ax.set_xlabel("Frequency (MHz)")
        ax.set_ylabel("Time (s)")
        ax.xaxis.set_major_formatter(mticker.FuncFormatter(x_tick_formatter))
        ax.yaxis.set_major_formatter(mticker.FuncFormatter(y_tick_formatter))

        ax.imshow(waterfall, aspect='auto', cmap='plasma')

    return []

ani = animation.FuncAnimation(fig, update, cache_frame_data=False, interval=0)
plt.show()
