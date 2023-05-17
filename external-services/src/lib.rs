pub mod navtex;
extern crate persistence;

/// I and Q values must be of equal frequency for all functions in this module
/// and are assumed to be shifted 90degrees from eachother
/// as is the custom with IQ modulation
pub mod iq {
    use std::f64::consts::PI;

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

    pub struct AmplitudeIterator<'a, T: Iterator<Item = (&'a i16, &'a i16)>> {
        inner: &'a mut T,
    }

    impl<'a, T: Iterator<Item = (&'a i16, &'a i16)>> AmplitudeIterator<'a, T> {
        pub fn new(iter: &'a mut T) -> Self {
            Self { inner: iter }
        }
    }

    impl<'a, T: Iterator<Item = (&'a i16, &'a i16)>> Iterator for AmplitudeIterator<'a, T> {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                Some((i, q)) => Some(amplitude(i.clone() as f64, q.clone() as f64)),
                None => None,
            }
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
