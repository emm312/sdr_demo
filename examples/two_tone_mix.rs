use plotly::{Plot, Scatter};
use sdr_demo::{collect_samples, fft_real, make_tone, mix, sum};

fn main() {
    let tone_a = make_tone(102_000_000.);
    let tone_b = make_tone(97_000_000.);
    let spectrum = sum(tone_a, tone_b);

    let lo = make_tone(100_000_000.);

    let int_freq = mix(spectrum, lo);

    let samples = collect_samples(int_freq, 100_000);

    let fft_o = fft_real(samples);

    let mut plot = Plot::new();
    let trace = Scatter::new(
        fft_o.clone().into_iter().map(|(a, _)| a).collect(),
        fft_o.into_iter().map(|(_, b)| b).collect(),
    );

    plot.add_trace(trace);
    plot.write_html("fft_two_tone_mix.html");
}
