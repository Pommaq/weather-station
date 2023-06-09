extern crate anyhow;
extern crate inquire;
extern crate plotters;
extern crate services;

use inquire::validator::Validation;
use inquire::Text;
use plotters::prelude::*;
use services::get_if_frequency_from_wav;

fn draw_chart(
    output_path: &str,
    amplitudes: &[f64],
    sample_size: usize,
    max_y: u32,
    max_x: u32,
) -> anyhow::Result<()> {
    std::fs::File::create(output_path)?;

    let data = (0..sample_size)
        .map(|x| (x as f64, amplitudes[x]))
        .collect::<Vec<(f64, f64)>>();

    // Set number of pixels on drawing
    let drawing_area = BitMapBackend::new(output_path, (1024, 1024)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder
        .margin(7)
        .set_left_and_bottom_label_area_size(20);

    let mut chart_context = chart_builder
        .build_cartesian_2d(0.0..max_x as f64, 0.0..max_y as f64)
        .unwrap();

    chart_context.configure_mesh().draw().unwrap();

    chart_context
        .draw_series(LineSeries::new(data, BLACK))
        .unwrap()
        .label("IF")
        .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], BLACK));

    chart_context
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .margin(20)
        .legend_area_size(5)
        .border_style(BLUE)
        .background_style(BLUE.mix(0.1))
        .label_font(("Calibri", 20))
        .draw()
        .unwrap();

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let validator = |input: &str| {
        use std::path::Path;
        if Path::exists(Path::new(input)) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("No such file".into()))
        }
    };

    let input_path = Text::new("Enter path to a file")
        .with_validator(validator)
        .prompt()?;

    let amplitudes = get_if_frequency_from_wav(&input_path)?;

    let output_path = Text::new("Enter output filename").prompt()?;

    //draw_chart(&output_path, &amplitudes, 10000)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use services::calculate_dft_from_wav;

    use crate::draw_chart;
    // 44100hz sample rate.
    const SAMPLE_PATH: &str = "../samples/sdrplay/SDRuno_20200904_204456Z_516kHz.wav";
    const SAMPLERATE: usize = 62500;

    #[test]
    fn is_drawable() {
        let window_size = 1024 * 2;

        let amplitudes = calculate_dft_from_wav(SAMPLE_PATH, SAMPLERATE, window_size).unwrap();
        let output = "aaaa.png";
        let entry = 52;

        let sample = &amplitudes[entry * window_size..(entry + 1) * window_size];

        draw_chart(
            output,
            sample,
            sample.len(),
            2,
            window_size.try_into().unwrap(),
        )
        .unwrap();
    }
}
