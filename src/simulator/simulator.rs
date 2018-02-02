use num::complex::Complex;
use rand;
use {MeasuredResult, QuantumMachine, Qubit};
use gates::{SingleGate, SingleGateApplicator};

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

    fn encode(&self, absent: &Qubit) -> usize {
        self.vector.iter().enumerate().fold(0usize, |acc, (i, x)| {
            if i == absent.index {
                acc
            } else {
                (acc << 1) + if *x == Eigenstate::Zero { 0 } else { 1 }
            }
        })
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

impl SingleGateApplicator for QuantumSimulator {
    fn apply_single(&mut self, gate: &SingleGate, qubit: &Qubit) {
        let mut index_table = vec![-1isize; 1 << (self.dimension - 1)];

        for (i, state) in self.states.iter_mut().enumerate() {
            let code = state.encode(qubit);
            if index_table[code] >= 0 {
                //self.states[index_table[code] as usize].coeff = Complex::new(1., 0.);
            } else {
                index_table[code] = i as isize;
            }
        }
    }
}
