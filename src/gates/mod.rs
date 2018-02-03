use ndarray::prelude::*;
use num::complex::Complex;

pub struct SingleGate {
    pub matrix: Array2<Complex<f64>>,
}

pub trait SingleGateApplicator {
    fn apply_single(&mut self, gate: &SingleGate, qubit: &::Qubit);
}

lazy_static! {
    static ref H: SingleGate = {
        SingleGate {
            matrix: array![
                [Complex::new(1., 0.), Complex::new(-1., 0.)],
                [Complex::new(1., 0.), Complex::new(1., 0.)]
            ] / (2f64).sqrt(),
        }
    };
}
