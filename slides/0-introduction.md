---
marp: true
paginate: true
footer: "SDR Workshop &bull; Introduction"
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

<h1 title>Introduction</h1>

## Software Defined Radio Workshop

<!--
We use radio technology like FM radio, Bluetooth, Wi-Fi, and Cellular daily, but how do they actually work?
In this workshop you will learn some of how these systems work and implement your own simplified versions.
-->

---

# What is Software Defined Radio

<div two-column>
<div>

- A radio system where components that were traditionally implemented with analog hardware are implemented with software
- A device that receives and digitizes radio frequency data for processing on a computer
- Used for research and development of new communication protocols

</div>
<div style="width: 70%;margin-left: 20px;">

![RTL SDR](assets/introduction/rtl-sdr.png)

</div>
</div>

<!--
The BFSK project is very similar to the way bluetooth 1.0 works, although like 700x slower...
-->

---

# RTL2832

<div two-column>
<div>

- Integrated circuit produced by Realtek Corporation
- Demodulates DVB-T (Digital Video Broadcasting – Terrestrial) signals
- Has an additional mode for receiving FM radio

<br />

- While working on a driver for the Linux kernel, *Eric Fry* realized that the FM radio mode could be used to capture raw samples

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

- librtlsdr lets you configure the tuner and receive samples in your own programs

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

- Open the [resources page](https://connorcode.com/files/Documents/sdr-workshop) for this workshop on Google Classroom
- *Plug in your RTL-SDR device* and go through the steps in the setup guide

<hr/>

1. Install *Python* and *Visual Studio Code* from the Microsoft Store
2. Download setup files linked on the resources page
3. Run Zadig to fix the USB drivers
   - Select RTL-SDR device, then click "Reinstall Driver"
4. Run `example.py` to make sure everything is working

<!--
Zadig will need the administrator password, so wait for someone to put that in.
-->

---

<div two-column>
<div>

# Example Program Output

- If everything worked, you should see a frequency spectrum like the one here
- I'm transmitting FM audio data at 100Mhz, so that's what the signal is

</div>
<div style="width: 80%;margin-left: 15px;">

<img alt="Expected spectrum" src="assets/introduction/radio-spectrum.png" rounded></img>

</div>
</div>

---

# Workshop Outline

1. Introduction
1. Digital Signals
   - Spectrogram Project
1. IQ Sampling
   - FM Radio Project
1. Data Modulation
   - Binary Data Transmission Project
