# Spectrogram Project

Open Visual Studio Code and make a new file named `spectrogram.py`, then copy the following imports to the top of the file.

```python
from rtlsdr import RtlSdr
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
```

This project has three main parts:

1. Initializing the SDR
1. Loop once for each row of the spectrogram
   1. Read some samples
   1. Convert the samples to the frequency domain
   1. Add the frequency domain representation to a list
1. Show the spectrogram list with matplotlib

Hints are provided in most section if you get stuck, but avoid looking at them if you can.

## PyRTL-SDR

From the `pyrtlsdr` package we will be using the `RtlSdr` class, the code below will connect to the default RTL-SDR device.

```python
sdr = RtlSdr()
```

### Properties

These properties can be read or set. Floats are numbers with a decimal (like 1.2) and integers are whole numbers (like 5).

|             Field | Type  | Description                                                                                                                |
| ----------------: | ----- | -------------------------------------------------------------------------------------------------------------------------- |
| `sdr.center_freq` | float | The frequency in Hz the SDR is tuned to. Set it to 100 MHz (100,000,000 Hz) to see the audio I am transmitting.            |
| `sdr.sample_rate` | float | The number of samples taken per second. The highest sample rate that works well on these SDRs is 2.048 MHz (2,048,000 Hz). |
|        `sdr.gain` | int   | The gain of the internal amplifier. Print the `sdr.gain_values` field to see what values you can use (192 is the highest). |

<br>
<details>
<summary>Hint</summary>

```python
sdr = RtlSdr()
print(sdr.gain_values) # => [-99, -40, 71, 179, 192]

# Note that underscores (_) can be used in number
# literials to make them easier to read.

sdr.sample_rate = 2_048_000 # 2.048 MHz
sdr.center_freq = 100_000_000 # 100 Mhz
sdr.gain = 192 # Max gain from `gain_values`
```

</details>

### Methods

The only method we will be using on the `RtlSdr` object is `read_samples`.
It takes in one parameter, an integer that specifies the number of samples to read.
Only some sample counts work, so I would recommend using `1024`.

For some reason the first ~2k samples are garbage data, so those should be discarded with a call to `sdr.read_samples(2048)`.

> [note] Tip
> You will need to make a list to put each row of the spectrogram into.
> For a refresher on List operations, see the [Python Lists](#python-lists) section.
>
> Then you will need to loop a bunch of times, once for each row of the final spectrogram, read 1024 samples then process them using the numpy methods in the next section.
> If you loop 6,000 times reading 1024 samples each time, it will record for about three seconds.
>
> <details>
> <summary>Hint</summary>
>
> ```python
> waterfall = []
> while len(waterfall) < 6000:
>     samples = sdr.read_samples(FFT_SIZE)
>     # TODO: Convert to frequency domain
>     waterfall.append(frequency)
> ```
>
> </details>
>
> Try to make the duration configurable in seconds, you will need to include the sample rate in your calculations.
> You can also try adding a print statement with the progress of the recording.

## Numpy

The `read_samples` method returns a numpy list, so you can immediately use any numpy functions on it.

| Function                       | Description                                                                                  |
| ------------------------------ | -------------------------------------------------------------------------------------------- |
| `np.hanning(n)`                | Returns a list that can be multiplied with an array of length n to apply the Hanning window. |
| `np.fft.fft(samples)`          | Returns the frequency domain representation of the samples.                                  |
| `np.fft.fftshift(frequencies)` | Shifts the frequencies so that the 0 Hz frequency is in the center.                          |
| `np.abs(frequencies)`          | Returns the magnitude of each frequency component (ignoring phase shifts).                   |

<br>
<details>
<summary>Hint</summary>

In your loop you will want something like the following.
If you want you can factor the `1024` out into a constant like `FFT_SIZE`.

```python
samples = sdr.read_samples(1024) * np.hanning(1024)
fft = np.abs(np.fft.fftshift(np.fft.fft(samples)))
waterfall.append(fft)
```

</details>

## Matplotlib

Once you build up your list of frequency domain representations over time, you can show them with the matplotlib `plt.imshow` function, then use `plt.show()` to open the window.
To use up the full size of the window, you should set the aspect to 'auto' like this:

```python
plt.imshow(waterfall, aspect='auto', cmap='plasma')
plt.show()
```

At this point the spectrogram should be fully functional!
If it is continue to the next section.

### Correct Axes

With `plt.subplots()` you can get access to the `Axes` object:

```python
fig, ax = plt.subplots()
```

| Function               | Description                   |
| ---------------------- | ----------------------------- |
| `ax.set_xlabel(label)` | Sets the label of the x-axis. |
| `ax.set_ylabel(label)` | Sets the label of the y-axis. |
| `ax.set_title(label)`  | Sets the title of the plot.   |

You can also change the function used to get the tick labels on the axes with the `ax.xaxis.set_major_formatter` and `ax.yaxis.set_major_formatter` functions.

```python
def x_tick_formatter(x, pos): return 'Todo'
def y_tick_formatter(y, pos): return 'Todo'

ax.xaxis.set_major_formatter(mticker.FuncFormatter(x_tick_formatter))
ax.yaxis.set_major_formatter(mticker.FuncFormatter(y_tick_formatter))
```

> [note] Tip
> For the x-axis, remember that the Nyquist sampling theorem states that the lowest frequency will be $c-\frac{f_s}{2}$ and the highest will be $c+\frac{f_s}{2}$, where $c$ is the center frequency and $f_s$ is the sample rate.
> For the y-axis, you know how many samples are in each row (the FFT size) and with `sdr.sample_rate` you know how many samples are taken each second and therefore the time each sample represents.

## Waterfall Extension

A waterfall plot is a spectrogram that scrolls showing new data through time.
I won't walk through exactly how to make one, but ill put some info here to work with.

You will want to look into the matplotlib [animation.FuncAnimation](https://matplotlib.org/stable/api/_as_gen/matplotlib.animation.FuncAnimation.html) function to make plots that update over time.
It lets you define a function that gets called repeatedly to update a plot.
Your function should add a bunch of rows to the waterfall list then re-plot the image.

## Appendix

### Simplified Version

Instead of building up a list of frequency domain representations, you can also just record a bunch of samples and use the matplotlib function `plt.specgram` to plot a spectrogram for you!

<details>
<summary>Hint</summary>

```python
DURATION = 3 # Duration in seconds

samples = []
while len(samples) < DURATION * sdr.sample_rate:
    print(f'Recording... {len(samples) / sdr.sample_rate}s', end='\r')
    samples.extend(sdr.read_samples(1024))
```

</details>

The `specgram` function take one positional parameter, the samples buffer from the last step.
To correctly draw the axes with the time and frequency, we also need to pass the sample rate and center frequency through the `Fs` and `Fc` named properties respectively.

```python
plt.specgram(<samples>, Fs=<sample_rate>, Fc=int(<center_frequency>))
```

Finally, to open the window and show the plot, use `plt.show()`.

<details>
<summary>Hint</summary>

```python
plt.specgram(samples, Fs=sdr.sample_rate, Fc=int(sdr.center_freq))
plt.show()
```

</details>

### Python Lists

As a quick refresher, here are some basic Python List operations.

```python
# Create an empty list
a = []

# Push the value `12` to the end of the list
a.append(12)

# Add all the values from a different list
a.extend([1, 2, 3])

# Get the length of a list
len(a) # => 4
```
