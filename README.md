# iq-sample

IQ sampling (a.k.a. complex sampling or quadrature sampling) is a technique commonly used to capture modulated signals, e.g. in wireless applications.

For our purpose we are interested in finding and tracking the carrier frequency (instead of the modulated signal).

Our goal is to come up with a robust yet computationally effective implementation suitable to running on lightweight targets.

## Approach

### Phase detection

We seek to track the signal by samples taken based on assumed frequency. Let's first look at the effect of sampling a sinusoidal signal with an offset. We start with a phase offset of zero.

![1](./figures/iq_0.drawio.svg)

The corresponding complex signal $z=a+bi$ can be represented as a vector, where $a$ is the real valued part (corresponding to $q_0-q_1$), while $b$ is the imaginary valued part (corresponding to $i_0-i_1$).

In case the signal was sampled with a phase offset of 45 degrees ($\pi/4$) we would get something like this:

![2](./figures/iq_1.drawio.svg)

As seen the vector has now turned with 45 degrees ($\pi/4$).

Similarly a larger phase offset by 90 degrees ($\pi/2$) yields.

![3](./figures/iq_2.drawio.svg)

From this we can conclude that phase can be recovered by inspecting the complex representation.

### Tracking

Our goal is to find and track the frequency of the incoming signal. We can do this by assuming a ballpark value, and inspecting the phase shift in between the successive samples.

That is, if the assumed frequency matches the incoming signal, the phase angle will remain stable, while if the assumed frequency is too high or too low we will observe a frequency shift.

## Implementation

We start by first prototyping the approach to run the host. The idea with IQ sampling is to adopt the sample rate to the frequency. In order to model this behavior we use a fixed sample frequency for the signal to pick samples from according to the assumed frequency.

```rust
const F: u32 = 100;
fn get_sample(t: f32) -> f32 {
    (t * TAU * F as f32).sin()
}

// computes the phase, nomalized to range -1..1
fn angle(i0: f32, i1: f32, q0: f32, q1: f32) -> f32 {
    (i0 - i1).atan2(q0 - q1) / TAU
}

fn sample(t: &mut f32, p_4: f32) -> f32 {
    // sample i0, q0, i1, q1
    let i0 = get_sample(*t);
    *t += p_4;
    let q0 = get_sample(*t);
    *t += p_4;
    let i1 = get_sample(*t);
    *t += p_4;
    let q1 = get_sample(*t);
    *t += p_4;
    let w = angle(i0, i1, q0, q1);
    println!("angle {}, {}", w, w * 360.0);
    w
}
```

Here we assume the input signal will be a perfect sinusoid, the function `get_sample`samples its value at time `t`.

The `angle`function takes 4 samples and computes the phase angle.

We can now implement a simple tracker.

```rust
fn tracking(f: f32) -> f32 {
    // assumed period
    let p: f32 = 1.0 / f;
    let mut p_4 = p / 4.0;

    let mut t = 0.0;
    let mut w0 = sample(&mut t, p_4);
    loop {
        let w1 = sample(&mut t, p_4);
        let diff = w1 - w0;

        if diff.abs() < 0.0001 {
            break;
        } else {
            p_4 -= 0.5 * p_4 * diff;
            w0 = w1;
        } 
    }
     1.0 / (4.0 * p_4)
}
```

We let `f` be the assumed frequency, and `p_4` a fourth of its period (thus the initial sampling frequency).

`w0` and `w1` are two successive phase angle readings. If their difference (`diff`) is less than the threshold (in the example 0.0001), we consider the frequency as found and we are done, else we adjust the sample rate according to the weighted difference (0.5 for the example). This allows us to trade adaptation speed to robustness (higher weight, more aggressive but less robust).

Now we can test our tracker!!

```rust
#[test]
fn test_tracker() {
    tracking(10.0); // 106
    tracking(50.0); // 26
    tracking(80.0); // 17
    tracking(90.0); // 12
    tracking(100.0); // 1
    tracking(110.0); // 12
    tracking(120.0); // 13
    // tracking(130.0); // unstable
}
```

The number of iterations required (shown as comment) is expected lower when assumption is closer to target. For the guitar tuner use case, adjustments are typically within the range of "cents" (hundreds of a semi tone), thus tracking.

However for this approach to work we need a way to determine the the initial assumed frequency. To this end, it seems that seeking can be done from the lowest reasonable frequency is stable, e.g. 50 Hz.

In case tracking fails (an unreasonable high frequency estimated, e.g. 400 Hz.) we can safely start from lowest frequency.








