use num::complex::Complex;
use rand;
use {MeasuredResult, QuantumMachine, Qubit};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Eigenstate {
    Zero,
    One,
}

struct QuantumState {
    coeff: Complex<f64>,
    vector: Vec<Eigenstate>,
}

impl QuantumState {
    fn norm_sqr(&self) -> f64 {
        self.coeff.norm_sqr()
    }

    fn is_orthogonal(&self, qubit: &Qubit, direction: Eigenstate) -> bool {
        self.vector[qubit.index] == direction
    }
}

pub struct QuantumSimulator {
    dimension: usize,
    states: Vec<QuantumState>,
}

impl QuantumSimulator {
    pub fn new(n: usize) -> QuantumSimulator {
        let state = QuantumState {
            coeff: Complex::new(1., 0.),
            vector: vec![Eigenstate::Zero; n],
        };

        QuantumSimulator {
            dimension: n,
            states: vec![state],
        }
    }

    fn normalize(&mut self, n: f64) {
        for state in self.states.iter_mut() {
            (*state).coeff /= n;
        }
    }
}

impl QuantumMachine for QuantumSimulator {
    fn measure(&mut self, qubit: &Qubit) -> MeasuredResult {
        let prob0: f64 = self.states
            .iter()
            .filter(|&x| x.is_orthogonal(qubit, Eigenstate::Zero))
            .map(|x| x.norm_sqr())
            .sum();

        if prob0 < rand::random::<f64>() {
            self.states
                .retain(|x| x.is_orthogonal(qubit, Eigenstate::Zero));
            self.normalize(prob0.sqrt());

            MeasuredResult::Zero
        } else {
            self.states
                .retain(|x| x.is_orthogonal(qubit, Eigenstate::One));
            self.normalize((1. - prob0).sqrt());

            MeasuredResult::One
        }
    }

    fn get_qubits(&self) -> Vec<Qubit> {
        (0..self.dimension).map(|x| Qubit { index: x }).collect()
    }
}
