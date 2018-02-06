//!
//! A module for single quantum gates.
//!
//! The gates in this modules represents an operation for a single qubit.
//! Currently, sigma matrices, Hadamard gates and identity operator are supported.
//!

use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

#[derive(Debug)]
pub struct SingleGate {
    matrix: Array2<Complex<f64>>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit: &Qubit) {
            self.apply_single(&$mat.matrix, qubit);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

macro_rules! carray {
    ( $([$($x: expr),*]),* ) => {{
        use num::complex::Complex;
        array![
            $([$(Complex::new($x, 0.)),*]),*
        ]
    }};
}

macro_rules! carray_i {
    ( $([$($x: expr),*]),* ) => {{
        use num::complex::Complex;
        array![
            $([$(Complex::new(0., $x)),*]),*
        ]
    }};
}

///
/// An trait for the types which accept operations for a single qubit.
///
pub trait SingleGateApplicator {
    ///
    /// An operation for the given unitary matrix `matrix` to `qubit`
    ///
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit);

    gen_gates!(H, X, Y, Z, ID);

    fn phase(&mut self, phi: f64, qubit: &Qubit) {
        let mut matrix = carray![[1., 0.], [0., 0.]];
        matrix[[1, 1]] = Complex::new(phi.cos(), phi.sin());
        self.apply_single(&matrix, qubit);
    }
}

lazy_static! {
    pub static ref H: SingleGate = {
        SingleGate {
            matrix: carray![
                [1., -1.],
                [1.,  1.]
            ] / (2f64).sqrt(),
        }
    };

    pub static ref X: SingleGate = {
        SingleGate {
            matrix: carray![
                [0., 1.],
                [1., 0.]
            ],
        }
    };

    pub static ref Y: SingleGate = {
        SingleGate {
            matrix: carray_i![
                [0.,  1.],
                [-1., 0.]
            ],
        }
    };

    pub static ref Z: SingleGate = {
        SingleGate {
            matrix: carray![
                [1.,  0.],
                [0., -1.]
            ],
        }
    };

    pub static ref ID: SingleGate = {
        SingleGate {
            matrix: carray![
                [1., 0.],
                [0., 1.]
            ],
        }
    };

    pub static ref SQNOT: SingleGate = {
        SingleGate {
            matrix: carray![
                [1., 1.],
                [1., 1.]
            ] / 2. + carray_i![
                [ 1., -1.],
                [-1.,  1.]
            ] / 2.,
        }
    };
}
