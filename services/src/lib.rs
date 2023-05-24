extern crate external_services;
extern crate persistence;

use anyhow::{Ok, Result};
use dft;
use hound;
use iq::IFIterator;

/// I and Q values must be of equal frequency for all functions in this module
/// and are assumed to be shifted 90degrees from eachother
/// as is the custom with IQ modulation
pub mod iq;


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
    
    let result = binding.into_iter().map(|x| {
        x.unwrap() as f64
    }).collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use dft::{self, Plan, Operation};
    use hound;

    use super::*;
    const SAMPLEPATH: &str = "../samples/navtex_2023-02-21T16_40_30_201.wav";

    #[test]
    fn test_dfts() {
        let mut magnitudes = get_iq_from_wav(SAMPLEPATH).unwrap();
        
        let plan = Plan::new(Operation::Backward, 512);
        dft::transform(&mut magnitudes[..512], &plan);

        print!("{:?}", magnitudes);
        

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
