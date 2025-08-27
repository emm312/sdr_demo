use plotly::{Plot, Scatter};
use sdr_demo::{collect_samples, fft_complex, make_tone_complex, mix};

fn main() {
    let tone_a = make_tone_complex(102_000_000.);

    let lo = make_tone_complex(-100_000_000.);

    let int_freq = mix(tone_a, lo);

    let samples = collect_samples(int_freq, 50_000);

    let mut plot = Plot::new();
    let i = Scatter::new(
        (0..samples.len()).collect(),
        samples.iter().map(|n| n.re).collect(),
    );
    let q = Scatter::new(
        (0..samples.len()).collect(),
        samples.iter().map(|n| n.im).collect(),
    );

    plot.add_trace(i);
    plot.add_trace(q);
    plot.write_html("td_iq_pos_freq.html");

    let fft_o = fft_complex(samples);

    let mut plot = Plot::new();
    let trace = Scatter::new(
        fft_o.clone().into_iter().map(|(a, _)| a).collect(),
        fft_o.into_iter().map(|(_, b)| b).collect(),
    );

    plot.add_trace(trace);
    plot.write_html("fft_pos_freq.html");
}
