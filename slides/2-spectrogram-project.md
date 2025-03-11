---
marp: true
paginate: true
footer: "SDR Workshop &bull; Spectrogram Project"
math: katex
class: invert
style: |
  [title] {
    font-size: 2.2em;
  }

  [two-column] {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  [center] {
    display: flex;
    justify-content: center;
  }

  img, [rounded] {
    border-radius: 6px;
  }
---

<h1 title>Spectrogram Project</h1>

## Software Defined Radio Workshop

---

<div two-column>
<div style="transform: translateY(25%);">

# Project Summary

We will be writing a Python program that records and plots the RF spectrum over time.

</div>
<div style="width: 90%;margin-left: 10px">

![Spectrogram](assets/spectrogram-project/spectrogram.png)

</div>
</div>

---

# Spectrograms

<div two-column>
<div>

- A spectrogram is a plot that shows a signal's frequency over time
- This is achieved by splitting the incoming samples into chunks and stacking their frequency domain representations
- A color map is used to display the magnitude of each frequency

</div>
<div style="width: 110%">

![Waterfall Plot](assets/spectrogram-project/waterfall-plot.bmp)

</div>
</div>

---

# Discrete Fourier Transform Properties

- Operation that converts a time-domain signal to the frequency-domain
- The size of the frequency domain output is the same as time domain input
  - More samples results in a higher resolution in the frequency domain
  - Each bin corresponds to $f_s/N\text{ Hz}$
- Increasing the sample rate gives us a greater frequency range in the output
  - The output range is $-f_s/2$ to $f_s/2$
  - This makes sense if you think back to the Nyquist Sampling Theorem

<br />

> $f_s$ is the sample rate.

<!--
When givin a completely real time-domain input, the frequency-domain output's positive and negative segments will be identical in magnitude,
the negative frequencies will be complex conjugates of the positive frequency components.
-->

---

# Negative Frequency?

<div two-column>
<div>

- Negative frequency does not exist physically, it's just a mathematical construct
- It's effectively just a frequency relative to our defined center frequency
- If we tune to 100 MHz with a sample rate of 10 MHz, we will view the spectrum from 95 MHz to 105 MHz

</div>
<div style="width: 70%;margin-left: 30px;">

![Negative frequencies](assets/spectrogram-project/negative-frequencies.png)

</div>
</div>

---

# Windowing

- FTs assume that the time-domain input signals are periodic, meaning the last sample connects back to the first
- Sharp jumps between samples cause lots of unwanted frequency artifacts
- To avoid jumps, we use *windowing functions* to taper the signal's ends to zero

<div center>
<!-- <img alt="Windowing functions" src="assets/spectrogram-project/windowing-functions.bmp" style="width: 50%" /> -->
<video src="assets/spectrogram-project/hann-window.mp4" autoplay loop controls muted style="border-radius: 6px"></video>
</div>

---

# Introduction to `pyrtlsdr`

- PyRTL-SDR is used to initialize the SDR at startup then read samples
- The first ~2000 samples should be ignored

```python
sdr = RtlSdr() # Connect to RTL-SDR device
sdr.sample_rate = 2_048_000 # 2.048 MHz
sdr.center_freq = 100_000_000 # 100 Mhz
sdr.gain = 192 # Max gain
```

```python
# Ignore first 2048 samples
sdr.read_samples(2048)

while True:
  # Read 1024 samples from the device
  samples = sdr.read_samples(1024)
```

---

# Introduction to `numpy`

- Each chunk of samples needs to be processed quickly or the program won't run in real time (Python on its own is not fast enough!)
- Numpy allows efficiently performing operations on large datasets
- The PyRTL-SDR method `read_samples` actually returns a numpy array

<br>

```python
array = np.array([1, 2, 3]) # Convert a Python List to a Numpy Array
```

---

# Array Operations with `numpy`

- Basic operations like `+`, `-`, `*`, and `/` can be performed *element-wise* on same sized arrays
- Many mathematical functions are available like `np.hanning`, `np.fft.fft`, `np.fft.fftshift`, `np.abs`

<br>

```python
a = np.array([1., 2., 3.])
b = np.array([4., 5., 6.])

a + b # [5., 7., 9.]
np.mean(a) # 2.0
```

---

# What is `np.fft.fftshift`?

- Due to *math reasons*, the output of a Fourier Transform starts with the DC component, the positive frequencies, then the negative ones
- We usually want negative frequencies on the left, DC in the middle, and positive frequencies on the right
- This function just shifts the array around into the desired format

<div center>

![FFT Shift](assets/spectrogram-project/fft-shift.bmp)

</div>

---

# FFT in Python

1. Read in some samples and multiply in the windowing function

```python
FFT_SIZE = 1024
samples = sdr.read_samples(FFT_SIZE) * np.hamming(FFT_SIZE)
```

2. Perform a Fourier Transform (and FFT shift)

```python
fft = np.fft.fftshift(np.fft.fft(samples))
```

3. Get the magnitude of each frequency bin, ignoring phase shifts.

```python
freq = np.abs(fft)
```

<!--
Note that the discarding of ~2048 samples has been omitted from this example.

What is the frequency range and bin width of the freq array:

- center_freq - f_s/2 = 98.08
- center_freq + f_s/2 = 100.12

- f_s/fft_size = 4000 Hz
-->

---

# Spectrogram in Python

- After we build up a list of FFT results, we need to show it as a 2D image
- For this we can use the `plt.imshow` function from `matplotlib`
  - Setting the `aspect` to `'auto'` let it scale the image to fill the window

<br>

```python
waterfall = []
for _ in range(512):
  samples = sdr.read_samples(FFT_SIZE) * np.hanning(FFT_SIZE)
  fft = np.abs(np.fft.fftshift(np.fft.fft(samples)))
  waterfall.append(fft)

plt.imshow(waterfall, aspect='auto', cmap='plasma')
plt.show()
```

---

# Fixing the Axes

- By default `imshow` will just show the number of pixels on each axis
- We can define functions that take in x or y coordinates and return nicer labels

```python
def x_tick_formatter(x, pos): return 'todo'
def y_tick_formatter(y, pos): return 'todo'

fig, ax = plt.subplots()
ax.set_xlabel('Frequency (Hz)')
ax.set_ylabel('Time (s)')
ax.xaxis.set_major_formatter(mticker.FuncFormatter(x_tick_formatter))
ax.yaxis.set_major_formatter(mticker.FuncFormatter(y_tick_formatter))
```

---

# Python String Formatting

- F-strings or formatted strings allow you to create a string with values from variables or expressions added in
- Extra arguments can be passed to limit the number of digits after the decimal

<br>

```python
a = 'world'
b = 3.14159

f'Hello, {a}'  # => 'Hello, world!'
f'π ≈ {b}'     # => 'π ≈ 3.14159'
f'π ≈ {b:.2f}' # => 'π ≈ 3.14'
```

---

# Your Turn!

- Plug in your RTL-SDR device, open Visual Studio Code and make a new file named `spectrogram.py`
- Open the resources page linked on Google Classroom
  - Open the 'Spectrogram Project' link on the resources page for a reference sheet with all the functions you will need
  - You can also find this slide deck there
