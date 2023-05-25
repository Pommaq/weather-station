use std::f64::consts::PI;

pub fn phase(i: f64, q: f64) -> f64 {
    (q / i).atan()
}
pub fn magnitude(i: f64, q: f64) -> f64 {
    (i * i + q * q).sqrt()
}

/// Calculates instantaneous frequency by derivating the instantenous phase
/// of IQ data, this works since the frequency is encoded like this when
/// working with IQ. The returned instantenous frequency belongs to the
/// baseband signal.
pub fn instantaneous_frequency(i1: f64, q1: f64, i2: f64, q2: f64) -> f64 {
    let phase1 = phase(i1, q1);
    let phase2 = phase(i2, q2);
    (phase2 - phase1) / (2f64 * PI)
}

pub struct IFIterator<'a, T: Iterator<Item = (&'a i16, &'a i16, &'a i16, &'a i16)>> {
    inner: &'a mut T,
}

impl<'a, T: Iterator<Item = (&'a i16, &'a i16, &'a i16, &'a i16)>> IFIterator<'a, T> {
    pub fn new(iter: &'a mut T) -> Self {
        Self { inner: iter }
    }
}
impl<'a, T: Iterator<Item = (&'a i16, &'a i16, &'a i16, &'a i16)>> Iterator for IFIterator<'a, T> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(i1, q1, i2, q2)| {
            instantaneous_frequency(*i1 as f64, *q1 as f64, *i2 as f64, *q2 as f64)
        })
    }
}

pub struct MagnitudeIterator<'a, T: Iterator<Item = (&'a i16, &'a i16)>> {
    inner: &'a mut T,
}

impl<'a, T: Iterator<Item = (&'a i16, &'a i16)>> MagnitudeIterator<'a, T> {
    pub fn new(iter: &'a mut T) -> Self {
        Self { inner: iter }
    }
}

impl<'a, T: Iterator<Item = (&'a i16, &'a i16)>> Iterator for MagnitudeIterator<'a, T> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(i, q)| magnitude(*i as f64, *q as f64))
    }
}
