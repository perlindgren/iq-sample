use std::f32::consts::{PI, TAU};

fn main() {}

// const FS: u32 = 100;
// const F: u32 = 1;
// const T: u32 = 1;

// fn gen_sin() -> Vec<f32> {
//     let mut d: Vec<f32> = vec![];
//     for t in 0..(T * FS) as usize {
//         let s: f32 = (t as f32 * TAU * F as f32 / FS as f32).sin();
//         d.push(s)
//     }
//     d
// }

// #[test]
// fn sin_4() {
//     let d = gen_sin();
//     println!("d {:?}", d);
//     let f4 = (F * 4) as usize;
//     let i0 = d[F*4];
// }

fn angle(i0: f32, i1: f32, q0: f32, q1: f32) -> f32 {
    (i0 - i1).atan2(q0 - q1) / (2.0 * PI)
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
    let i1 = ((offset + e) * PI).sin();
    println!("i1 {}", i1);
    let q0 = ((offset + e) * PI / 2.0).sin();
    println!("q0 {}", q0);
    let q1 = ((offset + e) * PI * 3.0 / 2.0).sin();
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
    println!("1.1 multiplier");
    gen_sample_error(offset, 1.1); // 10% higher frequency
    println!("0.9 multiplier");
    gen_sample_error(offset, 0.9); // 10% lower frequency

    println!("-- offset pi/2 --");
    let offset = PI / 2.0;
    println!("1.0 multiplier");
    gen_sample_error(offset, 1.0);
    println!("1.1 multiplier");
    gen_sample_error(offset, 1.1); // 10% higher frequency
    println!("0.9 multiplier");
    gen_sample_error(offset, 0.9); // 10% lower frequency
}
