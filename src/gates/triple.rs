//!
//! A module for triple quantum gates.
//!
//! The gates in this modules represents an operation for three qubits.
//! Currently, CCNOT are supported.
//!

use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

#[derive(Debug)]
pub struct TripleGate {
    pub matrix: Array2<Complex<f64>>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: &Qubit, qubit2: &Qubit, qubit3: &Qubit) {
            self.apply_triple(&$mat.matrix, qubit1, qubit2, qubit3);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

///
/// An trait for the types which accept operations for theree qubits.
///
pub trait TripleGateApplicator {
    ///
    /// An operation for the given unitary matrix `matrix` to `qubit1`, `qubit2` and `qubit3`
    ///
    fn apply_triple(
        &mut self,
        matrix: &Array2<Complex<f64>>,
        qubit1: &Qubit,
        qubit2: &Qubit,
        qubit3: &Qubit,
    );

    gen_gates!(CCNOT);
}

lazy_static! {
    pub static ref CCNOT: TripleGate = {
        let mut vec = vec![Complex::new(0., 0.,); 64];
        for i in 0..6 {
            vec[i * 8 + i] = Complex::new(1., 0.);
        }

        vec[6 * 8 + 7] = Complex::new(1., 0.);
        vec[7 * 8 + 6] = Complex::new(1., 0.);

        TripleGate {
            matrix: Array::from_shape_vec((8, 8), vec).unwrap()
        }
    };
}
