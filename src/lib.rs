use std::f32::consts::{PI, TAU};

const F: f32 = 100.0;
const E2: f32 = 85.0;

fn get_sample(t: f32) -> f32 {
    (t * TAU * F).sin()
}

// fn get_sample(t: f32) -> f32 {
//     (t * TAU * E2).sin() + (t * TAU * E2 * 2.0).sin()
// }

// fn get_sample(t: f32) -> f32 {
//     (t * TAU * E2).sin() + (t * TAU * E2 * 2.0).sin() + (t * TAU * E2 * 3.0).sin()
// }

// fn get_sample(t: f32) -> f32 {
//     (1..=8).fold(0.0, |acc, k| {
//         acc + (t * TAU * E2 * k as f32).sin() / k as f32
//     })
// }

// computes the phase, normalized to range -1..1
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
    w
}

// f is assumed frequency
fn tracking(f: f32) {
    // assumed period
    let p: f32 = 1.0 / f;
    let mut p_4 = p / 4.0;

    let mut t = 0.0;

    let mut it = 0;
    let mut f_now;

    loop {
        it += 1;
        let w0 = sample(&mut t, p_4);
        let w1 = sample(&mut t, p_4);
        let diff = w1 - w0;
        f_now = 1.0 / (4.0 * p_4);

        println!(
            "f_now {} p_4 {} w0 {}, w1 {}, diff {}",
            f_now, p_4, w0, w1, diff
        );

        if diff.abs() < 0.0001 || f_now > 400.0 {
            break;
        } else {
            p_4 -= 0.5 * p_4 * diff;
        }
    }
    println!("initial f {}, it {}, f estimated {}", f, it, f_now);
}

#[test]
fn test_tracker() {
    // tracking(10.0);
    // tracking(50.0);
    // tracking(80.0);
    // tracking(90.0);
    // tracking(100.0);
    // tracking(110.0);
    tracking(114.0);
    //tracking(120.0);
    //tracking(130.0); // unstable
}

fn gen_sample(offset: f32) {
    let i0 = offset.sin();
    // println!("i0 {}", i0);
    let i1 = (offset + PI).sin();
    // println!("i1 {}", i1);
    let q0 = (offset + PI / 2.0).sin();
    // println!("q0 {}", q0);
    let q1 = (offset + PI * 3.0 / 2.0).sin();
    // println!("q1 {}", q1);
    let phase = angle(i0, i1, q0, q1);
    println!("angle {} {}", phase, phase * 360.0);
}

#[test]
fn iq_simple() {
    for i in 0..8 {
        let offset = i as f32 * 2.0 * PI / 8.0;
        gen_sample(offset)
    }
}

fn gen_sample_error(offset: f32, e: f32) -> f32 {
    let i0 = 0.0f32.sin();
    println!("i0 {}", i0);
    let q0 = (offset + e * PI / 2.0).sin();
    println!("q0 {}", q0);
    let i1 = (offset + e * PI).sin();
    println!("i1 {}", i1);
    let q1 = (offset + e * PI * 3.0 / 2.0).sin();
    println!("q1 {}", q1);
    let phase = angle(i0, i1, q0, q1);
    println!("angle {} {}", phase, phase * 360.0);
    phase
}
#[test]
fn iq_error() {
    let offset = 0.0;
    println!("-- offset 0 --");
    println!("1.0 multiplier");
    gen_sample_error(offset, 1.0);
    // println!("1.1 multiplier");
    // gen_sample_error(offset, 1.1); // 10% higher frequency
    // println!("0.9 multiplier");
    // gen_sample_error(offset, 0.9); // 10% lower frequency

    println!("-- offset pi/2 --");
    let offset = PI / 2.0;
    println!("1.0 multiplier");
    gen_sample_error(offset, 1.0);
    // println!("1.1 multiplier");
    // gen_sample_error(offset, 1.1); // 10% higher frequency
    // println!("0.9 multiplier");
    // gen_sample_error(offset, 0.9); // 10% lower frequency
}
