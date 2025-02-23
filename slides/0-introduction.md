---
marp: true
paginate: true
footer: "SDR and DSP Workshop &bull; Introduction"
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

<h1 title>Software Defined Radio and Digital Signal Processing Workshop</h1>

## Connor Slade

---

# What is Software Defined Radio

<div two-column>
<div>

- A radio system where components that were traditionally implemented with analog hardware are implemented with software
- SDR as a thing is a device that receives and digitizes RF data for processing on a computer
- Building blocks of wireless communication protocols like Wi-Fi (QAM), Bluetooth (FSK), and Cellular (QAM)

</div>
<div style="width: 70%;margin-left: 20px;">

![RTL SDR](assets/introduction/rtl-sdr.png)

</div>
</div>

<!--
QAM is Quadrature Amplitude Modulation.

FSK is Frequency Shift Keying, which is basically FM modulation for digital information.
-->

---

# RTL2832

<div two-column>
<div>

- Integrated circuit produced by Realtek Corporation
- Demodulates DVB-T (Digital Video Broadcasting – Terrestrial) signals
- Has an additional mode for receiving FM radio

<br />

- While working on a driver for the Linux kernel, *Eric Fry* realized that the FM radio mode could be used to capture raw IQ samples

</div>
<div style="width: 70%">

![RTL2832 Pinout](assets/introduction/pinout.png)

</div>
</div>

---

# RTL-SDR

<div two-column>
<div>

- rtl-sdr is a software package that includes the librtlsdr library and various command line tools

- librtlsdr lets you configure the tuner and receive IQ samples in your own programs

- There are bindings to librtlsdr in many languages, including Python

</div>
<div style="width: 100%;margin-left: 20px;">

```python
from rtlsdr import RtlSdr

sdr = RtlSdr()
sdr.sample_rate = 2.048e6
sdr.center_freq = 100e6
sdr.freq_correction = 60
sdr.gain = 'auto'

samples = sdr.read_samples(1024)
sdr.close()
```

> Example program using the `pyrtlsdr` Python package

</div>
</div>

---

# Setting up your Environment

1. Install *Python* and *Visual Studio Code* from the Microsoft Store
2. Download setup files linked on Google Classroom
<!-- TODO: Put install script in setup:
pip install pkg_resources pyrtlsdr[lib]
-->
3. Run Zadig to fix the USB drivers
   - Select RTL-SDR device, then click "Reinstall Driver"
4. Run `example.py` to make sure everything is working

---

# Example Program Output

<div two-column>
<div>

- If everything worked, you should see a frequency spectrum like the one on the right.
- I'm transmitting FM audio data at 100Mhz, so that's what the signal is
  - Decoding it will be our first project

</div>
<div style="width: 80%;margin-left: 15px;">

<img alt="Expected spectrum" src="assets/introduction/radio-spectrum.png" rounded></img>

</div>
</div>

---

# Math Review

(also some new things probably)
This stuff will come up in the next few lessons.

---

# Complex Numbers

<div two-column>
<div>

- The complex numbers are a number system extending the real numbers
- They are expressed in the form $a+bi$, where $i$ is the imaginary unit
- A complex number can be represented as a point in a 2D coordinate system known as the *complex plane*

</div>
<div style="width: 80%;margin-left: 20px;">
<img src="assets/introduction/complex-plane.svg" alt="Complex plane" style="background: white;border-radius: 4px;width: 100%;"></img>
</div>
</div>

<!--
We will only be using the rectangular form.
-->

---

# Complex Operations

Complex numbers can be added, subtracted, multiplied, and divided as you would expect, but there are also some more intreating operations:

- The complex conjugate of a number $a+bi$ is defined as $a-bi$
- The absolute value of a complex number is it's magnitude, $\sqrt{a^2 + b^2}$
- The argument or phase of a complex number is the angle with the positive real axis, $\text{arctan2}(b, a)$

---

# Euler's formula

<div two-column>
<div style="width: 65%">

- Establishes the relationship between trig functions and the complex exponential function
- Provides a compact notation for defining *complex sinusoids* (like the one to the right)

<br />

<div center>

$$
\boxed{e^{ix} = \cos(x)+i\sin(x)}
$$

</div>

</div>
<div style="width: 35%;margin-left: 20px;">
<img src="assets/introduction/eulers-formula-plot.gif" alt="visualization of Euler's formula" style="background: white;border-radius: 4px;" />
</div>
</div>

---

# Complex Numbers in Python

<div two-column>
<div>

- Python actually has native support for complex numbers, but the imaginary unit is $j$ not $i$
- We will make use of the mathematical functions from numpy as they work on singular numbers and arrays

</div>
<div style="width: 100%;margin-left: 20px;">

```python
import numpy as np

a = 3 + 5j

assert a.real == 3
assert a.imag == 5

conjugate = np.conjugate(a)
magnitude = np.abs(a)
phase = np.angle(a)

t, freq = 0, 100
shift = np.exp(2j * np.pi * freq * t)
```

</div>
</div>

<!--
j is used over i because in electrical engineering i is used for current
-->
