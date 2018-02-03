use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

pub struct SingleGate {
    pub matrix: Array2<Complex<f64>>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit: &Qubit) {
            self.apply_single(&$mat, qubit);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait SingleGateApplicator {
    fn apply_single(&mut self, gate: &SingleGate, qubit: &Qubit);

    gen_gates!(H, X, Y, Z, ID);
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

    pub static ref X: SingleGate = {
        SingleGate {
            matrix: array![
                [Complex::new(0., 0.), Complex::new(1., 0.)],
                [Complex::new(1., 0.), Complex::new(0., 0.)]
            ],
        }
    };

    pub static ref Y: SingleGate = {
        SingleGate {
            matrix: array![
                [Complex::new(0., 0.), Complex::new(0., 1.)],
                [Complex::new(0., -1.), Complex::new(0., 0.)]
            ],
        }
    };

    pub static ref Z: SingleGate = {
        SingleGate {
            matrix: array![
                [Complex::new(1., 0.), Complex::new(0., 0.)],
                [Complex::new(0., 0.), Complex::new(-1., 0.)]
            ],
        }
    };

    pub static ref ID: SingleGate = {
        SingleGate {
            matrix: array![
                [Complex::new(1., 0.), Complex::new(0., 0.)],
                [Complex::new(0., 0.), Complex::new(1., 0.)]
            ],
        }
    };
}
