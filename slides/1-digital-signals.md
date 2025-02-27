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

---

# The Frequency Domain

<div two-column>
<div>

- Usually we think of signals in the *time domain*, where their value is changing through time
- All signals can be represented with an infinite sum of sine waves with varying frequency, amplitude, and phases
- A function or signal can be converted between the time and frequency domains with the *Fourier transform*

</div>
<div style="width: 90%; margin-left: 25px;">

![A signal in the time and frequency domain](assets/frequency-domain/freq-domain.gif)

</div>
</div>

---

# Time-Frequency Properties

<div two-column>
<div>

- Time-Frequency Properties or 'Fourier properties' tell us how the frequency domain signal is affected by certain modifications to time domain signal
- There are more properties than what is covered here

</div>
<div style="width: 70%;">

1. Linearity
1. Frequency Shift
1. Scaling in Time

</div>
</div>

<!--
Additional properties:

- Convolution in Time Property
- Convolution in Frequency Property
-->

---

# Time-Frequency Properties: Linearity

- If we add two signals in the time domain, the two frequency domain signals will also be added together.
- If either of the time domain signals are scaled by a scalar, the frequency domain signal will also scaled by the same amount.

<br />

$$
\boxed{ax(t)+bx(t) \leftrightarrow aX(f)+bY(f)}
$$

---

# Time-Frequency Properties: Frequency Shift

- By multiplying a time domain signal by a complex sinusoid at frequency $f_0$, we shift it by $f_0$ in the frequency domain
- This is how the tuner in the RTL-SDR is able to change the center frequency, it also allows us to focus in on specific parts of the spectrum in software

<br />

$$
\boxed{e^{i2\pi{}f_0t}x(t) \leftrightarrow X(f-f_0)}
$$

---

# Time-Frequency Properties: Scaling in Time

- Scaling in the time domain causes an inverse scaling in the frequency domain
- This means that if we want to transfer data at a faster rate, we have to use physically more of the bandwidth

<br />

$$
\boxed{x(at) \leftrightarrow X(\frac{f}{a} \times \frac{1}{\lvert a \rvert})}
$$
