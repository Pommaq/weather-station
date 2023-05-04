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

pub mod wav {
    use hound;
    /*
        https://www.allaboutcircuits.com/textbook/radio-frequency-analysis-design/radio-frequency-demodulation/how-to-demodulate-digital-phase-modulation/
        https://thinkrf.com/solutions/signal-analysis-demodulation/
        https://www.embedded.com/the-goertzel-algorithm/

        we need to do a bit of math. We know Navtex transmit at 100 baud,
        so every 1/100th second there is a "bit". We expect to read at
        518Khz and our sample file is at 1953hz sample rate.
        Thus assuming we start at 0 we expect to see a bit at
        sample number x by calculating 1953/100 = 195.3*x

        I will, for now, assume that if we see 518khz + 170 hz it's a 1.
        if it's 518khz -170hz it's a 0.

        It appears we can decode this FSK modulation using Goertzel algorithm.

        Keyword for the "decoding" we want to do is "RF demodulation".


    */
    #[cfg(test)]
    mod tests {

        use hound;
        use hound::SampleFormat;
        use std::f32::consts::PI;
        use std::i16;

        const NAVTEX_SAMPLE_PATH: &str = "../samples/navtex_2023-02-21T16_40_30_201.wav";

        #[test]
        fn can_detect_stereo() {
            let mut reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap();
            let num_channels = reader.spec().channels;
            assert_eq!(num_channels, 2, "Unable to see that wav file is stereo");
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
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
