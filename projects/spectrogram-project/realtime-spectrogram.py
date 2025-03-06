from rtlsdr import RtlSdr
import matplotlib.animation as animation
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
import threading
assert RtlSdr is not None

FFT_SIZE = 1 << 13
CHUNK_SIZE = 512
WATERFALL_SIZE = 512

sdr = RtlSdr()
sdr.sample_rate = 2.048e6
sdr.center_freq = 100e6
sdr.gain = sdr.gain_values[-1]

buffer = []
waterfall = []
running = True

def read_samples():
    global buffer, waterfall, running

    while running:
        new_samples = sdr.read_samples(CHUNK_SIZE)
        buffer.extend(new_samples)

        while len(buffer) >= FFT_SIZE:
            samples = np.array(buffer[:FFT_SIZE])
            buffer = buffer[FFT_SIZE:]

            spectrum = np.abs(np.fft.fftshift(np.fft.fft(samples)))
            waterfall.append(spectrum)
            if len(waterfall) > WATERFALL_SIZE: waterfall.pop(0)

t = threading.Thread(target=read_samples)
t.start()

fig, ax = plt.subplots()
fig.suptitle("Waterfall Plot")

def update_ticks(x, pos):
    x_freq = (x - FFT_SIZE / 2) * sdr.sample_rate / FFT_SIZE
    freq = (x_freq + sdr.center_freq) / 1e6
    return f"{freq:.2f}"

def update(frame):
    if len(waterfall) > 0:
        ax.clear()
        ax.set_xlabel("Frequency (MHz)")
        ax.set_ylabel("Time (s)")
        ax.xaxis.set_major_formatter(mticker.FuncFormatter(update_ticks))

        ax.imshow(waterfall, aspect='auto')
    return []

ani = animation.FuncAnimation(fig, update, cache_frame_data=False)
plt.show()

running = False
t.join()
