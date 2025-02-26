from rtlsdr import RtlSdr
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

FFT_SIZE = 512

# Connect to the RTL-SDR device and set some parameters. Make sure that the gain
# is not set to 'auto', as the tuner was designed to be used with DVB-T signals
# and will actually really mess up the signal.
sdr = RtlSdr()
sdr.sample_rate = 2.048e6
sdr.center_freq = 99800000  # 100.0 MHz
sdr.gain = 20

# Discard the first ~2k samples as they don't contain useful data. This is just
# a quirk of the hardware.
sdr.read_samples(2048)

# The read_samples function returns an array of complex floating point numbers
# [-1, 1]. Python has native support for complex numbers btw. In the base
# librtlsdr library, samples are provided as 8-bit integers with alternating
# real and imaginary parts.
samples = sdr.read_samples(FFT_SIZE)

# Use numpy to convert the samples into the frequency domain, aka an FFT. The
# fftshift function puts the center frequency in the center of the plot, rather
# than the left edge.
fft = np.abs(np.fft.fftshift(np.fft.fft(samples)))

# Plot the frequency spectrum with matplotlib.
plt.plot(fft)
plt.xlabel("Frequency (MHz)")
plt.ylabel("Magnitude")

# Set the x-axis ticks to show the actual frequency values in MHz.
def update_ticks(x, pos):
    x_freq = (x - FFT_SIZE / 2) * sdr.sample_rate / FFT_SIZE
    freq = (x_freq + sdr.center_freq) / 1e6
    return f"{freq:.2f}"

ax = plt.gca()
ax.xaxis.set_major_formatter(mticker.FuncFormatter(update_ticks))

plt.show()
