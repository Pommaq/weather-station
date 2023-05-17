/// https://goughlui.com/2019/10/05/project-navtex-message-gallery-august-september-2019/
/// Sample source: https://www.sdrangel.org/iq-files/
/*
All NAVTEX broadcasts are made on 518 kHz, using narrow-band direct printing 7-unit forward
 error correcting (FEC or Mode B) transmission.
 This type of transmission is also used by Amateur Radio service (AMTOR).
 Broadcasts use 100 baud FSK modulation, with a frequency shift of 170 Hz.
 The center frequency of the audio spectrum applied to a single sideband transmitter is 1700 Hz.
 The receiver 6 dB bandwidth should be between 270-340 Hz.
*/
// Python implementation: https://github.com/pd0wm/navtex/blob/master/Navtex%20Decoder.ipynb

/// I and Q values must be of equal frequency for all functions in this module
/// and are assumed to be shifted 90degrees from eachother
/// as is the custom with IQ modulation
pub mod iq {
    use std::{f64::consts::PI, io::BufRead};

    pub fn phase(i: f64, q: f64) -> f64 {
        (q / i).atan()
    }
    pub fn amplitude(i: f64, q: f64) -> f64 {
        (i * i + q * q).sqrt()
    }

    /// Calculates instantaneous frequency by derivating the instantenous phase
    /// of IQ data, this works since the frequency is encoded like this when
    /// working with IQ. The returned instantenous frequency belongs to the
    /// baseband signal.
    pub fn instantaneous_frequency(i1: f64, q1: f64, i2: f64, q2: f64) -> f64 {
        let phase1 = phase(i1, q1);
        let phase2 = phase(i2, q2);
        (phase2 - phase1) / (2 as f64 * PI)
    }
}

pub mod wav {
    use hound::{self, WavReader, WavSamples};
    use std::{fs::File, io::BufReader};
    use thiserror::Error;
    use crate::navtex::iq;

    #[derive(Error, Debug)]
    enum WavError {
        
        #[error(transparent)]
        IO(#[from] hound::Error)
    }   

    /* 
    We want an iterator that takes ownership of an outside iterator,
     iterating over it and calculating what we want as we go
    */ 
    struct AmplitudeIterator<'a ,T: Iterator<Item=(&'a i16, &'a i16)>> {
        inner: &'a mut  T,
    }

    impl <'a, T:  Iterator<Item=(&'a i16,&'a i16)>> AmplitudeIterator<'a, T> {
        fn new(iter: &'a mut T) -> Self {
            Self{inner: iter}
        }
    }

    impl<'a ,T: Iterator<Item=(&'a i16, &'a i16)>> Iterator for AmplitudeIterator<'a, T> {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                Some((i, q)) => {
                    Some(iq::amplitude(i.clone() as f64, q.clone() as f64))
                },
                None => {None}
            }
        }
    }
    

    #[cfg(test)]
    mod tests {
        use crate::navtex::iq;
        use hound;
        use std::fs::File;
        use std::i16;
        use std::io::Write;

        use super::AmplitudeIterator;

        const NAVTEX_SAMPLE_PATH: &str = "../samples/navtex_2023-02-21T16_40_30_201.wav";

        #[test]
        fn can_detect_stereo() {
            let reader: hound::WavReader<std::io::BufReader<File>> = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap();
            let num_channels = reader.spec().channels;
            assert_eq!(num_channels, 2, "Unable to see that wav file is stereo");
        }

        #[test]
        fn amplitude_iter_test() {
            let mut reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap(); 
            let binding  = reader
            .samples::<i16>()
            .collect::<Vec<_>>();

            let mut iterator = binding
            .windows(2)
            .step_by(2)
            .map(|x| {
               let i = x[0].as_ref().unwrap();
               let q = x[1].as_ref().unwrap();
               (i, q)
            });

            let amplitudes =  AmplitudeIterator::new(&mut iterator);
            for _ in amplitudes {
                /* 
                    Intentionally left empty
                    We just want to see if it panics or not
                */
            }
        }

        #[test]
        fn calculate_frequency() {
            let mut output = File::create("./result.txt").unwrap();

            let mut reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap(); 
            let _ = reader
                .samples::<i16>()
                .collect::<Vec<_>>()
                .windows(4)
                .step_by(2)
                .map(move |x| {
                    // Unwrap i and q, convert to f64 to access
                    // builtin trigonometric functions
                    let i1 = x[0].as_ref().unwrap().clone() as f64;
                    let q1 = x[1].as_ref().unwrap().clone() as f64;
                    let i2 = x[2].as_ref().unwrap().clone() as f64;
                    let q2 = x[3].as_ref().unwrap().clone() as f64;
                    let phase = iq::phase(i1, q1);
                    let freq = iq::instantaneous_frequency(i1, q1, i2, q2);
                    let amplitude = iq::amplitude(i1, q1);
                    output
                        .write(
                            &format!(
                                "Found instant frequency {} with phase {} with amplitude {}\n",
                                freq, phase, amplitude
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                })
                .collect::<Vec<_>>();
        }

        #[test]
        fn calculate_peak() {
            let mut reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap();

            let largest_value = reader.samples::<i16>().fold(0, |highest, s| {
                let sample = s.unwrap();
                return if highest > sample { highest } else { sample };
            });
            println!("Highest peak was {}", largest_value);
        }
        #[test]
        fn calculate_rms() {
            let mut reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap();
            let sqr_sum = reader.samples::<i16>().fold(0.0, |sqr_sum, s| {
                let sample = s.unwrap() as f64;
                sqr_sum + sample * sample
            });
            println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
        }

        #[test]
        fn demodulate_ai() {
            const CARRIER_FREQ: f64 = 518_000.0; // in Hz
            const BAUD_RATE: f64 = 100.0; // in symbols per second
            const SAMPLE_RATE: f64 = 1953.0; // in Hz
            const PHASE_INC: f64 = 2.0 * std::f64::consts::PI * CARRIER_FREQ / SAMPLE_RATE;

            let mut reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap();
            let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();

            let samples_per_symbol = (SAMPLE_RATE / BAUD_RATE) as usize;

            let mut demodulated_bits = Vec::new();

            for i in (0..samples.len()).step_by(samples_per_symbol) {
                // Calculate the average amplitude and phase of the symbol
                let (mut sum_i, mut sum_q) = (0.0, 0.0);
                for j in i..(i + samples_per_symbol).min(samples.len()) {
                    let sample = samples[j] as f64;
                    let phase = PHASE_INC * j as f64;
                    sum_i += sample * phase.cos();
                    sum_q += sample * phase.sin();
                }
                let avg_amplitude =
                    ((sum_i * sum_i + sum_q * sum_q) / samples_per_symbol as f64).sqrt();
                let avg_phase = sum_q.atan2(sum_i);

                // Demodulate the symbol
                let bit = if avg_phase > 0.0 { 1 } else { 0 };
                demodulated_bits.push(bit);
            }
            println!("{:?}", demodulated_bits);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
