---
marp: true
paginate: true
footer: "SDR and DSP Workshop &bull; IQ Sampling"
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

  [rounded], img {
    border-radius: 6px;
  }
---

<h1 title>Software Defined Radio and Digital Signal Processing Workshop</h1>

## Connor Slade

---

# Limited Sample Rates

- Most RTL-SDRs can sample at about 2.4 MHz
  - Due to the *Nyquist Sampling Theorem*, only frequencies below 1.2 MHz can be recovered (This is not enough for FM radio!)
- Because DACs have limited sample rates, we need to *shift* the signal in the frequency domain into the sampleable range

<!-- 
It would be impractical to sample at higher rates due to limited data transfer rates, among other reasons

- HackRf can do 20 MHz
- FM radio in america: from 88.0 to 108.0 MHz
-->

---

# Mixing Signals

<div two-column>
<div>

- A *mixer* is a device that combines two input signals to produce an output signal with frequencies equal to the sum and difference of the input frequencies
- In software, we just need to multiply the samples of each signal
- See [Interactive Signal Mixing]()

</div>
<div style="width: 100%;margin-left: 20px;">

![Ideal Mixer Diagram](assets/iq-sampling/ideal-mixer.svg)

<div/>
<div/>

---

# Frequency Ambiguity

- Internally SDRs use a local oscillator (LO) and a mixer to shift incoming RF (radio frequency) signals
- Unfortunately when the RF is shifted down, some frequencies become negative, which appear the same as positive signals

---

# IQ Sampling

- By separately mixing our signal with the LO and the LO with a 90° phase shift we have enough information to distinguish positive and negative frequencies
- IQ stands for In-phase and quadrature
  - Here quadrature just means two waves 90° out of phase

---

# Hardware Implementation of the Tuner

![RTL2832 Block Diagram](assets/iq-sampling/block-diagram.png)

