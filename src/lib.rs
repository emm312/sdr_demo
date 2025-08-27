use std::{
    f64,
    ops::{Add, Mul},
};

use num_complex::{Complex, Complex64};
use rustfft::{
    num_traits::{ConstZero, Float, FromPrimitive, Signed}, FftNum
};

pub const SAMPLE_RATE: usize = 500_000_000;

pub fn make_tone(freq: f64) -> impl FnMut() -> f64 {
    let mut t: f64 = 0.0;

    move || {
        t += f64::consts::TAU * freq / (4. * SAMPLE_RATE as f64);
        if t >= f64::consts::TAU {
            t -= f64::consts::TAU;
        }
        t.sin()
    }
}

pub fn make_tone_complex(freq: f64) -> impl FnMut() -> Complex64 {
    let mut t: f64 = 0.0;

    move || {
        t += f64::consts::TAU * freq / (4. * SAMPLE_RATE as f64);
        if t >= f64::consts::TAU {
            t -= f64::consts::TAU
        }
        Complex64::cis(t)
    }
}

pub fn sum<O, T: Add<Output = O>>(
    mut a: impl FnMut() -> T,
    mut b: impl FnMut() -> T,
) -> impl FnMut() -> O {
    move || a() + b()
}

pub fn mix<O, T: Mul<Output = O>>(
    mut a: impl FnMut() -> T,
    mut b: impl FnMut() -> T,
) -> impl FnMut() -> O {
    move || a() * b()
}

pub fn collect_samples<T>(mut func: impl FnMut() -> T, n_samples: usize) -> Vec<T> {
    let mut ret = Vec::with_capacity(n_samples);

    for _ in 0..n_samples {
        ret.push(func());
    }

    ret
}

pub fn bin_to_freq(bin: isize, size: usize) -> f64 {
    bin as f64 * (SAMPLE_RATE as f64 / size as f64)
}

pub fn fft_real<T: FftNum + ConstZero + Float>(mut samples: Vec<T>) -> Vec<(f64, T)> {
    let mut output: Vec<Complex<T>> = vec![Complex::ZERO; samples.len() / 2 + 1];

    let mut planner: realfft::RealFftPlanner<T> = realfft::RealFftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    fft.process(&mut samples, &mut output).unwrap();
    
    let fft_o = output
        .iter()
        .enumerate()
        .map(|(p, s)| (bin_to_freq(p as isize, samples.len() / 4), s.norm()));

    fft_o.collect()
}

pub fn fft_complex<T: Copy + FromPrimitive + Signed + FftNum + Float>(
    mut samples: Vec<Complex<T>>,
) -> Vec<(f64, T)> {
    let mut planner = rustfft::FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    fft.process(&mut samples);

    let (pos, neg) = samples.split_at(samples.len() / 2);
    let mut fft_o: Vec<Complex<T>> = Vec::with_capacity(samples.len());
    fft_o.append(&mut neg.to_vec());
    fft_o.append(&mut pos.to_vec());

    let fft_o = fft_o.iter().enumerate().map(|(p, s)| {
        (
            bin_to_freq(p as isize - (fft_o.len() / 2) as isize, fft_o.len() / 4),
            s.norm(),
        )
    });

    fft_o.collect()
}
