# Spectrogram Project

Open Visual Studio Code and make a new file named `spectrogram.py`.

Hints are provided in each section if you get stuck, but avoid looking at them if you can.

## PyRTL-SDR

From the `pyrtlsdr` package we will be using the `RtlSdr` class, which is imported and instantiated as follows.
This will connect to the default RTL-SDR device.

```python
from rtlsdr import RtlSdr

sdr = RtlSdr()
```

### Properties

These properties can be read or set. Floats are numbers with a decimal and integers are whole numbers.

|Field|Type|Description|
|-:|-|-|
|`center_freq`|float|The frequency in Hz the SDR is tuned to|
|`sample_rate`|float|The number of samples taken per second|
|`gain`|int|The gain of the internial amplifier|

You should set the `center_freq` to 100MHz (100,000,000 Hz), as that is the frequency I am brodcasting on.
Only some sample rates are valid, highest that works on these SDRs is 2.048 MHz (2,048,000 Hz).
Only a few values for gain work, the options can be queried with the `sdr.gain_values` field.

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
Only some sample counts work, so I would recommend using `2048`.

For some reason the first ~2k samples are garbage data, so those should be disgaurded with a call to `sdr.read_samples(2048)`.

## Recording

To show the radio spectrum over time, we will need to record samples for a few seconds before plotting the spectrogram.
Because we can only read certain numbers of samples at a time, we will need to keep adding to a buffer in chunks of 2048 samples until we have enough.

Because `sdr.sample_rate` tells us how many samples are taken each second, to get the number of samples in a longer period, just multiply the sample rate by the duration in seconds.
This can be used to create a loop condition that keeps reading samples and adding them to a list for a set duration.

For a refresher on List operations, see the [Python Lists](#python-lists) section.

<details>
<summary>Hint</summary>

```python
# How log to record (in seconds)
DURATION = 3

# Create an empty list to hold samples
samples = []

# While we have less that DURATION worth of samples stored, read more
while len(samples) < DURATION * sdr.sample_rate:
    # Print how long we have been recording for (this is optional btw).
    # `end='\r'` is used to print over the last message so each update
    # dosn't print a new line.
    print(f'Recording... {len(samples) / sdr.sample_rate}s', end='\r')

    # Read some samples from the SDR and add them all to the list
    samples.extend(sdr.read_samples(2048))
```

</details>

## Matplotlib

Matplotlib nicely provides a function specifically for plotting spectrograms, `plt.specgram`, assuming `plt` has been imported like this:

```python
import matplotlib.pyplot as plt
```

The `specgram` function take one positional parameter, the samples buffer from the last step.
To correctly draw the axies with the time and frequency, we also need to pass the sample rate and center frequency through the `Fs` and `Fc` named properties respectively.

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

## Waterfall Extension

A waterfall plot is a spectrogram that scrolls showing new data through time.
I won't walk through exactly how to make one, but ill put some info here to work with.

- See [animation.FuncAnimation](https://matplotlib.org/stable/api/_as_gen/matplotlib.animation.FuncAnimation.html) to make plots that update over time

## Appendix

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
