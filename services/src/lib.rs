extern crate external_services;

use anyhow::Result;
use iq::IFIterator;
use rustfft::{
    num_complex::{Complex, ComplexFloat},
    FftPlanner,
};

/// I and Q values must be of equal frequency for all functions in this module
/// and are assumed to be shifted 90degrees from eachother
/// as is the custom with IQ modulation
pub mod iq;

pub fn calculate_dft_from_wav(
    path: &str,
    samplerate: usize,
    window_size: usize,
) -> Result<Vec<f64>> {
    let iq = get_iq_from_mono_wav(path)?;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);
    let mut scratch = vec![Complex::default(); window_size];

    let mut complex_nums = iq
        .windows(2)
        .map(|win| {
            let i = win[0];
            let q = win[1];
            Complex { re: i, im: q }
        })
        .collect::<Vec<_>>();

    let leftovers = complex_nums.len() % window_size;
    let usable = complex_nums.len() - leftovers;

    let input_buffer = &mut complex_nums[0..usable];

    fft.process_with_scratch(input_buffer, &mut scratch);

    // calculate magnitudes
    let scaled = input_buffer
        .iter_mut()
        .map(|x| {
            let magnitude = x.abs();
            let power = magnitude * magnitude;
            let normalized_power = power / ((samplerate * window_size) as f64);
            normalized_power.log(10f64)
        })
        .collect::<Vec<_>>();
    Ok(scaled)
}

pub fn get_magnitudes_from_wav(path: &str) -> Result<Vec<f64>> {
    use iq::MagnitudeIterator;

    let mut reader = hound::WavReader::open(path).unwrap();
    let binding: Vec<std::result::Result<i16, hound::Error>> =
        reader.samples::<i16>().collect::<Vec<_>>();

    let mut iterator = binding.windows(2).step_by(2).map(|x| {
        let i = x[0].as_ref().unwrap();
        let q = x[1].as_ref().unwrap();
        (i, q)
    });
    Ok(MagnitudeIterator::new(&mut iterator).collect())
}

pub fn get_if_frequency_from_wav(path: &str) -> Result<Vec<f64>> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let binding: Vec<std::result::Result<i16, hound::Error>> =
        reader.samples::<i16>().collect::<Vec<_>>();

    let mut iterator = binding.windows(4).step_by(2).map(|x| {
        let i1 = x[0].as_ref().unwrap();
        let q1 = x[1].as_ref().unwrap();
        let i2 = x[2].as_ref().unwrap();
        let q2 = x[2].as_ref().unwrap();
        (i1, q1, i2, q2)
    });
    Ok(IFIterator::new(&mut iterator).collect())
}

pub fn get_iq_from_wav(path: &str) -> Result<Vec<f64>> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let binding: Vec<std::result::Result<i16, hound::Error>> =
        reader.samples::<i16>().collect::<Vec<_>>();

    let result = binding.into_iter().map(|x| x.unwrap() as f64).collect();

    Ok(result)
}

pub fn get_iq_from_mono_wav(path: &str) -> Result<Vec<f64>> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let binding: Vec<std::result::Result<i16, hound::Error>> =
        reader.samples::<i16>().collect::<Vec<_>>();

    let size = reader.len();
    let mut result = Vec::with_capacity(size as usize * 2);
    let iter = binding.into_iter();

    for i in iter {
        result.push(i? as f64);
        result.push(0.0);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;
    const SAMPLEPATH: &str = "../samples/sdrplay/SDRuno_20200904_204456Z_516kHz.wav";
    const SAMPLERATE: usize = 62500;

    #[test]
    fn test_dfts() {
        let _ = calculate_dft_from_wav(SAMPLEPATH, SAMPLERATE, 1024).unwrap();
    }

    #[test]
    fn amplitude_iter_test() {
        let _ = get_magnitudes_from_wav(SAMPLEPATH).unwrap();
    }

    #[test]
    fn if_iter_test() {
        let _ = get_if_frequency_from_wav(SAMPLEPATH).unwrap();
    }
}
