extern crate anyhow;
extern crate inquire;
extern crate plotters;
extern crate services;

use inquire::validator::Validation;
use inquire::Text;
use plotters::prelude::*;
use services::get_if_frequency_from_wav;

fn draw_chart(output_path: &str, amplitudes: &[f64], sample_size: usize) -> anyhow::Result<()> {
    std::fs::File::create(output_path)?;

    let data = (0..sample_size)
        .map(|x| (x as f64, amplitudes[x]))
        .collect::<Vec<(f64, f64)>>();

    // Set number of pixels on drawing
    let drawing_area = BitMapBackend::new(output_path, (4000 as u32, 1000)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder
        .margin(7)
        .set_left_and_bottom_label_area_size(20);

    let mut chart_context = chart_builder
        .build_cartesian_2d(0.0..sample_size as f64, -1.0..255.0)
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

    draw_chart(&output_path, &amplitudes, 10000)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use services::{get_if_frequency_from_wav, get_magnitudes_from_wav, get_iq_from_wav};

    use crate::draw_chart;
    // 44100hz sample rate.
    const SAMPLE_PATH: &str = "../samples/mediacollege/440Hz_44100Hz_16bit_05sec.wav";

    #[test]
    fn is_drawable() {
        let amplitudes = get_if_frequency_from_wav(SAMPLE_PATH).unwrap();
        let output = "aaaa.png";
        let sample = &amplitudes[4000..4000 + 3000];

        draw_chart(output, sample, sample.len()).unwrap();
    }

    #[test]
    fn dft_drawable() {

        let mut magnitudes = get_magnitudes_from_wav(SAMPLE_PATH).unwrap();
        let output = "bbbbbb2.png";

        let plan = dft::Plan::new(dft::Operation::Backward, 512);
        dft::transform(&mut magnitudes[..512], &plan);
        // The magnitudes are now encoded as i/q data. index(i)=frequency, q=phase
        let mag : Vec<f64>= magnitudes[..512].iter().step_by(2).cloned().collect();

        draw_chart(output, &mag, 512/2).unwrap();
    }
}
