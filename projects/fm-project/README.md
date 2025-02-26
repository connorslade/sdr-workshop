# FM Radio Project

Make a new file named `fm-decode.py` in Visual Studio Code and copy in the following template.
'Hints' provided in each section if you get stuck, but avoid looking at them if you can.

## Project Template

Copy the following project template into your project file.

```python
from rtlsdr import RtlSdr
import numpy as np
import scipy.io.wavfile as wavfile
import scipy.signal as signal

RECORD_TIME = 10

sdr = RtlSdr()
# TODO: Initialize RTL-SDR

# Shifts the frequencies in samples up by `frequency`
def offset(samples, frequency):
    t = np.arange(len(samples)) / sdr.sample_rate
    return samples * np.exp(2j * np.pi * frequency * t)

def process_samples(samples):
    # TODO: FM Demodulation
    pass

# Record and demodulate audio for `RECORD_TIME` seconds
buffer = []
while len(buffer) < RECORD_TIME * sdr.sample_rate:
    print(f"[*] Recording: {len(buffer) / sdr.sample_rate:.2f} s", end="\r")
    samples = sdr.read_samples(2048)
    audio = process_samples(samples)
    buffer.extend(audio)

sdr.close()

# Save audio to a `.wav` file
print("\n[*] Writing to file")
wavfile.write("output.wav", int(sdr.sample_rate), np.array(buffer))
```

## Initialize RTL-SDR

After creating the `RtlSdr` object, we need to configure the sample rate, center frequency, and gain.
This can be achieved by setting the `sample_rate`, `center_freq`, and `gain` properties.
These properties can also be read later on.

For our use cases, we can just keep the sample rate at 250 kHz (`250_000`) and gain at `'auto'`.
Because we need to offset the center frequency from the actual frequency of our signal, we can set it to the signal frequency of 200 MHz (`200_000_000`) plus an offset like 100 kHz (`100_000`).

I would recommend storing the offset you pick in a variable, as we will need it later for undoing the offset in software.

Finally, because the first approximately 2000 samples don't contain useful information, those can just be ignored with a call to `sdr.read_samples(2048)`.

<details>
<summary>Solution</summary>

```python
OFFSET = 100_000

sdr = RtlSdr()
sdr.sample_rate = 250_000
sdr.center_freq = 200_000_000 + OFFSET
sdr.gain = 'auto'

sdr.read_samples(2048)
```

</details>

## FM Demodulation

All of the code you write for this section should be in the `process_samples` function.

### Converting to a Numpy Array

While recording, the `process_samples` function gets called over and over with a List of samples to demodulate.
In order to make computation more efficient and make use of premade signal processing functions, we first need to convert the incomming sample list to a Numpy array with the `numpy.array` function.

<details>
<summary>Hint</summary>

In the `process_samples` function, we can use `np.array(samples)` to convert the List of samples to a Numpy array, like this:

```python
def process_samples(samples):
    samples = np.array(samples)
```

</details>

### Removing Our Offset

Because we set our center frequency 100 kHz above our signal of interest, we need to shift the frequencies of our samples by the same amount before any other processing steps.
To help with this I have provided the following function:

```python
def offset(samples, frequency):
    t = np.arange(len(samples)) / sdr.sample_rate
    return samples * np.exp(2j * np.pi * frequency * t)
```

You pass it a `numpy` sample array and a frequency and it will retuen a new array that has been frequency shifted.

<details>
<summary>Hint</summary>

```python
samples = offset(samples, 100_000)
```

</details>

### Ignoring Other Stations

Now that the signal we are interested in is centred at 0 Hz, we can make use of a *Low Pass Filter* to filter out other nearby stations or interference.
The Scipy `signal.butter` function is used to design a filter with a given order and cutoff.
It returns two lists, b and a, that represent the filter.

From my testing an order of `5` and a cutoff at 25 kHz (`25_000`) seams to work well.

```python
b, a = signal.butter(order, cutoff, "low", fs=sdr.sample_rate)
```

Now we can use the `signal.lfilter` to apply the filter to our samples.

```python
samples = signal.lfilter(b, a, samples)
```

### Pairing Up Samples

Python list slicing lets you isolate a section of a bigger list using the following syntax: `[start:end]`.
Start is the index of the first element to be included and end is the last.
If you want to start with the first element or end with the last element, you can omit that bound.

```python
a = [1, 2, 3, 4]

a[2:]   # [      3, 4]
a[:2]   # [1, 2      ]
a[:-1]  # [1, 2, 3   ]
a[1:-1] # [   2, 3   ]
```

Because `numpy` works on arrays, not individual values, in order to compare neighboring samples, we need to make two slices that have one sample of offset.
We can do this by taking one slice that skips the first element and another slice that skips the last.

<details>
<summary>Hint</summary>

```python
a = samples[:-1] # Skips the last element
b = samples[1:] # Skips the first element
```

If you want, this can also be written more concisely as the following:

```python
a, b = samples[:-1], samples[1:]
```

</details>

### Determining the Instantaneous Frequency

Finally, to actually demodulate the FM signal back into audio, we need to figure out what frequency each sample represents.
The frequency (technically the [angular frequency](https://en.wikipedia.org/wiki/Angular_frequency)) is the same as the change in phase at a point.
So we just need to find the change in phase element-wise between `a` and `b`.

The `np.angle` function lets us get the phase of a IQ sample, but we can't just subtract the angle between two to samples to get the change in angle because the output `np.angle` is not continuous.

Instead what we can do is rotate sample b by in inverse of sample `a`.
This brings the angle of sample `a` to $(a-a)=0$, and the angle to sample `b` to ($b - a$).

<details>
<summary>Hint</summary>

```python
frequency = np.angle(b / a)
```

</details>
<br>

The decoded samples can be so loud that you hear clipping in the audio playback, so I would recommend multiplying your frequency array by a constant like $0.3$ before returning it.

<details>
<summary>Hint</summary>

```python
return frequency * 0.3
```

</details>
<br>

## Testing it Out

Now, if you run your script by pressing the play button VS Code, you should see it count op for `RECORD_TIME` seconds then exit, leaving an `output.wav`.
If you play the WAV file, you *should* be able to hear the music being broadcast.

If something is not working, you can take a look at [my implementation](fm-decode.py) or ask someone for help.
