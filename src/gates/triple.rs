use ndarray::prelude::*;
use num::complex::Complex;
use Qubit;

pub struct TripleGate {
    pub matrix: Array2<Complex<f64>>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: &Qubit, qubit2: &Qubit, qubit3: &Qubit) {
            self.apply_triple(&$mat, qubit1, qubit2, qubit3);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait TripleGateApplicator {
    fn apply_triple(&mut self, gate: &TripleGate, qubit1: &Qubit, qubit2: &Qubit, qubit3: &Qubit);

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
