//!
//! A module for double quantum gates.
//!
//! The gates in this modules represents an operation for two qubits.
//! Currently, CNOT and SWAP are supported.
//!

use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

#[derive(Debug)]
pub struct DoubleGate {
    pub matrix: Array2<Complex<f64>>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: &Qubit, qubit2: &Qubit) {
            self.apply_double(&$mat.matrix, qubit1, qubit2);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

///
/// An trait for the types which accept operations for two qubits.
///
pub trait DoubleGateApplicator {
    ///
    /// An operation for the given unitary matrix `matrix` to `qubit1` and `qubit2`
    ///
    fn apply_double(&mut self, matrix: &Array2<Complex<f64>>, qubit1: &Qubit, qubit2: &Qubit);

    gen_gates!(CNOT, SWAP);
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

    pub static ref SWAP: DoubleGate = {
        DoubleGate {
            matrix: array![
                [Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.),],
                [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.),],
                [Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.),],
                [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.),],
            ],
        }
    };
}
