use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

pub struct DoubleGate {
    pub matrix: Array2<Complex<f64>>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: &Qubit, qubit2: &Qubit) {
            self.apply_double(&$mat, qubit1, qubit2);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait DoubleGateApplicator {
    fn apply_double(&mut self, gate: &DoubleGate, qubit1: &Qubit, qubit2: &Qubit);

    gen_gates!(CNOT);
}

lazy_static! {
    pub static ref CNOT: DoubleGate = {
        DoubleGate {
            matrix: array![
                [Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.),],
                [Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.),],
                [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.),],
                [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.),],
            ],
        }
    };
}
