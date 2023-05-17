extern crate external_services;
extern crate persistence;

use anyhow::{Ok, Result};
use hound;

pub fn get_amplitudes_from_wav(path: &str) -> Result<Vec<f64>> {
    use external_services::iq::AmplitudeIterator;

    let mut reader = hound::WavReader::open(path).unwrap();
    let binding: Vec<std::result::Result<i16, hound::Error>> =
        reader.samples::<i16>().collect::<Vec<_>>();

    let mut iterator = binding.windows(2).step_by(2).map(|x| {
        let i = x[0].as_ref().unwrap();
        let q = x[1].as_ref().unwrap();
        (i, q)
    });
    Ok(AmplitudeIterator::new(&mut iterator).collect())
}

#[cfg(test)]
mod tests {

    use super::*;
    const SAMPLEPATH: &str = "../samples/navtex_2023-02-21T16_40_30_201.wav";

    #[test]
    fn amplitude_iter_test() {
        let _ = get_amplitudes_from_wav(SAMPLEPATH).unwrap();
    }
}
