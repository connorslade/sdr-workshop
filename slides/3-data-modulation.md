---
marp: true
paginate: true
footer: "SDR Workshop &bull; Data Modulation"
math: katex
class: invert
style: |
  [title] {
    font-size: 2.2em;
  }

  [two-column] {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }

  [two-column] > div {
    align-self: center;
  }

  [center] {
    display: flex;
    justify-content: center;
  }

  [rounded], img {
    border-radius: 6px;
  }
---

<h1 title>Data Modulation</h1>

## Software Defined Radio Workshop

---

# Basic Data Modulation

- There are three parameters you can change on a sine wave: amplitude, frequency, and phase shift
- As such there are three main type of modulation schemes
  - Amplitude Modulation (AM)
  - Frequency Modulation (FM)
  - Phase Modulation (PM)
- More complex modulation schemes can be a combination of these

---

# <!-- fit --> Analog Modulation

I'm only covering AM and FM here because they are a lot more common than PM.

<!--
I don't think phase modulation was historically used in radio transmission. At least not commonly.

It was used for other applications like analog synthesizers.
-->

---

<div two-column>
<div>

## Amplitude Modulation (AM)

- Simplest analog modulation scheme
- Popularized in the 1900s for wirelessly transmitting morse code and later audio (AM Radio)
- Very susceptible to noise and interference

</div>
<div>

![Amplitude Modulation](assets/data-modulation/amplitude-modulation.svg)

</div>

<!--
Due to the high susceptibility to noise / interference *much* higher broadcast power is needed to overcome interference
-->

---

### Amplitude Modulation Index

- To reduce distortion, the modulation percent can be decreased
- Modulation over 100% is called over-modulation

<div center>
<img alt="" src="assets/data-modulation/am-modulation-percent.gif" width="70%"></img>
</div>

---

<div two-column>
<div>

## Frequency Modulation (FM)

- The instantaneous frequency of the carrier is modulated by the signal
- Much less susceptible to noise and interference (Thats why we still use FM radio)

</div>
<div>

![Frequency Modulation](assets/data-modulation/frequency-modulation.svg)

</div>

---

### Frequency Modulation Bandwidth

- The bandwidth of a FM signal is the difference between the lowest and highest instantaneous frequencies
- Large bandwidths give higher quality audio, but use more radio spectrum

<br>
<div center>
<img alt="" src="assets/data-modulation/fm-modulation-index.gif" width="85%"></img>
</div>

---

# <!-- fit --> Digital Modulation

Instead of just being referred to as modulations, digital schemes are called shift keyings.

---

# Binary Shift Keying

<img alt="Digital Modulations" src="assets/data-modulation/digital-modulations.svg" width="100%" />

---

# Binary Amplitude Shift Keying Constellation

<div center>
<img alt="BASK" src="assets/data-modulation/bask.gif" width="90%" />
</div>

---

# Binary Phase Shift Keying Constellation

<div center>
<img alt="BASK" src="assets/data-modulation/bpsk.gif" width="90%" />
</div>
