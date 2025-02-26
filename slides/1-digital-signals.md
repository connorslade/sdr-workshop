---
marp: true
paginate: true
footer: "SDR Workshop &bull; Digital Signals"
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

  [code-block] {
    background-color: #161b22;
    border: solid #30363d 1px;
    border-radius: 6px;
    padding: 10px;
  }

  hr {
    padding-top: 0.15em;
  }
---

<h1 title>Digital Signals</h1>

## Software Defined Radio Workshop

---

<div two-column>
<div>

# Radians

- A unit for angle based on the radius of a circle
- Often easier to work with mathematically
- Approximately $57.2958^\circ{}$

<br>

<div center>

|Degrees|$0^\circ{}$|$90^\circ{}$|$180^\circ{}$|$270^\circ{}$|$360^\circ{}$|
|:-:|:-:|:-:|:-:|:-:|:-:|
|Radians|$0$|$\pi/2$|$\pi$|$3\pi{}/2$|$2\pi{}$|

</div>
</div>
<div style="width: 37%">

![Radians](assets/digital-signals/radians.gif)

</div>

<!--
R=(a*pi)/180
-->

---

# Sine Waves

<div two-column>
<div>

- $f(t)=a\sin(2\pi{}ft+p)$
  - Amplitude: $a$
  - Frequency: $f$
  - Phase Shift: $p$
- $\cos(t) = \sin(t + \frac{\pi}{2})$
- $\cos(t)=\cos(-t)$

</div>
<div>

![Sine and Cosine on the Unit Circle](assets/digital-signals/sin-cos.gif)

</div>
</div>

<!--
sin(x)=-sin(-x)
-->

---

# What is a Signal?

<div two-column>
<div>

- In order to digitally process a signal, an Analog to Digital Converter (ADC) is used

- We can configure an ADC with a sample rate (number of samples per second) and obtain a series of discrete values

</div>
<div style="width: 100%;margin-left: 20px;">

![Signal sampling representation](assets/digital-signals/continous-vs-discrete.jpg)

</div>
</div>

---

# What Signal Could these Samples Represent?

<div center>

![Discrete-time samples](assets/digital-signals/ambigous-samples-top.jpg)

</div>

---

# Ambiguity in the Frequency Domain :scream:

<div two-column>
<div>

- Discrete-time samples have a frequency ambiguity that does not exist for continuous signals

- A series of discrete-time samples can represent *infinitely many* continuous signals

- This phenomenon is known as *aliasing*

</div>
<div style="width: 100%;margin-left: 20px;">

![Discrete-time samples with an ambiguous frequency](assets/digital-signals/ambigous-samples.jpg)

</div>
</div>

---

# Aliasing

<div two-column>
<div>

- The blue plots show the true signal and its frequency

- The orange plots show a signal reconstructed from the discrete samples

</div>
<div style="width: 100%;margin-left: 20px;">

![Animation showing aliasing](assets/digital-signals/fft-aliasing.gif)

</div>
</div>

<!--
Upper left: Animation depicts a sequence of sinusoids, each with a higher frequency than the previous ones.
These "true" signals are also being sampled (blue dots) at a constant frequency/rate, Upper right: The continuous Fourier transform of the sinusoid (not the samples).
The single non-zero component, depicting the actual frequency, means there is no ambiguity. Lower right: The discrete Fourier transform of just the available samples.
The presence of two components means the samples can fit at least two different sinusoids, one of which is the true frequency (upper-right).
Lower left: Using the same samples (now in orange), the default reconstruction algorithm produces the lower-frequency sinusoid.
-->

---

# The Nyquist–Shannon Sampling Theorem

If a function $x(t)$ contains no frequencies higher than $B$ hertz, then it can be completely determined from its ordinates at a sequence of points spaced less than $\frac{1}{2B}$ seconds apart.

<hr>

- The Nyquist Sampling Theorem states that your sample rate must be at least double the highest frequency in the sample to avoid aliasing

<!--
Human hearing is approx 20Hz to 20,000 Hz

- 44,100 Hz - CD audio (most common)
- 48,000 Hz - Standard sample rate for professional applications
-->
