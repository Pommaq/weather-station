use std::io::BufRead;

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

pub trait DeModulator {
    fn modulate(source: impl BufRead) -> Vec<u8>;

}

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
    https://www.youtube.com/watch?v=h_7d-m1ehoY
    By convention:
    - The amplitude of the "in-phase" signal = I
        I * cos (2*pi*f*t);
    - The amplitude of the "90degree shifted" signal = Q
        Q * sin(2*pi*f*t);


    Using that knowledge with
    https://electronics.stackexchange.com/questions/132642/how-to-get-bpsk-data-from-i-q-signal
    we now know we need the signal offset to calculate the amplitudes Q and I.
    We know that NavTex use a frequency shift of 170hz, so
    - Omega = 170*2pi?

    Part 2.
    https://www.youtube.com/watch?v=5GGD99Qi1PA

    - alternatively we can calculate for non "90degree shifted" signals:
        Q * sin(2*pi*f*t);
        I * sin(2*pi*f*t + Omega)

        if my theory is correct


    Reading form: https://www.allaboutcircuits.com/textbook/radio-frequency-analysis-design/radio-frequency-demodulation/understanding-i-q-signals-and-quadrature-modulation/
    Demodulation: https://www.allaboutcircuits.com/textbook/radio-frequency-analysis-design/radio-frequency-demodulation/understanding-quadrature-demodulation/
*/
    #[cfg(test)]
    mod tests {

        use hound;
        use std::i16;

        const NAVTEX_SAMPLE_PATH: &str = "../samples/navtex_2023-02-21T16_40_30_201.wav";

        #[test]
        fn can_detect_stereo() {
            let reader = hound::WavReader::open(NAVTEX_SAMPLE_PATH).unwrap();
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
                let avg_amplitude = ((sum_i * sum_i + sum_q * sum_q) / samples_per_symbol as f64).sqrt();
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
