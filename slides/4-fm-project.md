---
marp: true
paginate: true
footer: "SDR Workshop &bull; FM Project"
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

  [rounded] {
    border-radius: 6px;
  }
---

<h1 title>FM Radio Project</h1>

## Software Defined Radio Workshop

---

# Project Summary

We will be writing a Python program that records a FM radio station from the SDR for a few seconds then outputs a `.wav` file containing the demodulated audio.

---

# Frequency Modulated Audio

- Remember that FM radio modulates the instantaneous frequency of a carrier wave with an audio signal
- Therefore to demodulate it, we need to determine the frequency at each point
  - This is the same as the change in phase between samples!

---

# Avoiding DC Offset

- Because of the physical implementation of most SDRs, there will be a frequency spike at the center frequency
  - This ends up messing up FM demodulation a lot,
- We can avoid it by shifting our center frequency up a bit (100 kHz for example) then shifting our samples the other way in software

<br>

```python
t = np.arange(len(samples)) / sdr.sample_rate
samples *= np.exp(2j * np.pi * OFFSET * t)
```

---

# Ignoring Other Stations

<div two-column>
<div>

- Before we get to demodulating any signals, we should first isolate the station we actually want to listen to
- By setting the `center_freq`, we shift the center of our signal down to $0\text{ Hz}$
  - This means we can use a *low pass filter*

</div>
<div style="width:90%;margin-left:10px">
<img alt="Two FM stations on a waterfall plot" src="assets/fm-project/Screenshot.from.2025-02-25.at.21_01_41.991761932.png" rounded></img>
</div>
</div>

---

# Low Pass Filters

<div style="z-index:10;position:relative">

- Only allows frequencies lower than a defined *cutoff frequency* to pass through
- Symmetric around 0 Hz

</div>

<div center style="transform:translateY(-30px)">
<video src="assets/fm-project/low-pass.mp4" autoplay loop controls muted width="100%"></video>

</div>

---

# Scipy Low Pass Filter

- Scipy is a library that contains many algorithms for scientific computing, including signal processing
- The `signal.butter` function prepares the filter and returns the filter coefficients to be passed to `signal.lfilter` with the samples

```python
b, a = signal.butter(order, cutoff, "low", fs=sample_rate)
samples = signal.lfilter(b, a, samples)
```

- order &mdash; An integer that determines the slope of the falloff (5 is good)
- cutoff &mdash; The cutoff frequency in Hz (requires fs)

---

# Pairing Up Samples (Windowing)

- Because `numpy` works on arrays, not individual values, we need a way to compare neighboring samples
- We can take two slices, removing one element from the start of one and the end of the other

<br/>
<div style="display:grid;grid-template-columns: 1fr 2fr;">
<div>

|`[:-1]`|A|B|C|D|
|-:|-|-|-|-|
|`[1:]`|B|C|D|E|

</div>
<div style="align-self: center">

```python
a, b = samples[:-1], samples[1:]
```

</div>
</div>

---

<div two-column>
<div>

# Angle between Samples

- We can't just use `np.angle(b) - np.angle(a)` because of the discontinuity at $\pi$
- The solution is to rotate $b$ by $-a$, then measure the angle of $b$
  - Dividing rotates the opposite way as multiplication

<br>
<div center>

`np.angle(b / a)`

</div>
</div>
<video src="assets/fm-project/arg-range-issues.mp4" autoplay loop controls muted width="35%"></video>
</div>

---

# Your Turn

- Plug in your RTL-SDR device, open Visual Studio Code and make a new file named `fm-decode.py.py`
- Open the [resources page](https://connorcode.com/files/Documents/sdr-workshop) linked on Google Classroom
  - Open the ['Fm Radio Project'](https://connorcode.com/files/Documents/sdr-workshop/projects/fm-radio) link on the resources page for the starter code and a reference for the needed functions
  - You can also find this slide deck there
