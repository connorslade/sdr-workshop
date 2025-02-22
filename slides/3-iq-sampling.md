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

  [two-colum] {
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

# 

- Because DACs have limited sample rates, we need to shift the signal in the frequency domain into the sampleable range


---

# Representing Phase Shifts

---

# Why we need IQ Sampling

- With just real or just imaginary samples, it's impossible to determine if a frequency is negative or positive
  - Remember $\cos(x)=\cos(-x)$
- IQ samples provide a convenient way to modulate frequency, amplitude, and phase
