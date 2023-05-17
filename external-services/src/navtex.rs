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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
