use plotly::{Plot, Scatter};
use sdr_demo::{collect_samples, fft_real, make_tone};

fn main() {
    let tone_a = make_tone(102_000_000.);

    let samples = collect_samples(tone_a, 100_000);

    let fft_o = fft_real(samples);

    let mut plot = Plot::new();
    let trace = Scatter::new(
        fft_o.clone().into_iter().map(|(a, _)| a).collect(),
        fft_o.into_iter().map(|(_, b)| b).collect(),
    );

    plot.add_trace(trace);
    plot.write_html("fft_single_tone.html");
}
