# iq-sample

IQ sampling (a.k.a. complex sampling or quadrature sampling) is a technique commonly used to capture modulated signals, e.g. in wireless applications.

For our purpose we are interested in finding and tracking the carrier frequency (instead of the modulated signal).

Our goal is to come up with a robust yet computationally effective implementation suitable to running on lightweight targets.

## Approach

### Phase detection

We seek to track the signal by samples taken based on assumed frequency. Let's first look at the effect of sampling a sinusoidal signal with an offset. We start with a phase offset of zero.

![1](./figures/iq_0.drawio.svg)
*Figure 1: sine wave at $i_0$, $q_0$, $i_1$, $q_1$*

The corresponding complex signal $z=a+bi$ can be represented as a vector, where $a$ is the real valued part (corresponding to $q_0-q_1$), while $b$ is the imaginary valued part (corresponding to $i_0-i_1$).

![2](./figures/iq_0_phase.drawio.svg)
*Figure 2: sine wave at $i_0$, $q_0$, $i_1$, $q_1$*

In case the signal was sampled with a phase offset of 45 degrees ($\pi/4$) we would get something like this:

![3](./figures/iq_1.drawio.svg)
*Figure 1: sine with 45 degree offset*

with the corresponding (complex) representation:

![4](./figures/iq_1_phase.drawio.svg)
*Figure 3: phase with 45 degree offset*

As seen, in Figure 3, the vector has now turned with 45 degrees ($\pi/4$).

Similarly a larger phase offset by 90 degrees ($\pi/2$) yields.

![3](./figures/iq_2.drawio.svg)
*Figure 4: sine with 90 degree offset*

with the corresponding (complex) representation:

![4](./figures/iq_2_phase.drawio.svg)
*Figure 5: phase with 90 degree offset*

As seen the vector has now turned 90 degrees ($\pi/4$).

From this we can conclude that phase can be recovered by inspecting the complex representation.

### Tracking

Our goal is to find and track the frequency of the incoming signal. We can do this by assuming a ballpark value, and inspecting the phase shift in between the successive samples.

That is, if the assumed frequency matches the incoming signal, the phase angle will remain stable, while if the assumed frequency is too high or too low we will observe a frequency shift.

