use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

pub struct SingleGate {
    pub matrix: Array2<Complex<f64>>,
}

pub trait SingleGateApplicator {
    fn apply_single(&mut self, gate: &SingleGate, qubit: &Qubit);

    #[allow(non_snake_case)]
    fn H(&mut self, qubit: &Qubit) {
        self.apply_single(&H, qubit);
    }
}

lazy_static! {
    pub static ref H: SingleGate = {
        SingleGate {
            matrix: array![
                [Complex::new(1., 0.), Complex::new(-1., 0.)],
                [Complex::new(1., 0.), Complex::new(1., 0.)]
            ] / (2f64).sqrt(),
        }
    };
}
