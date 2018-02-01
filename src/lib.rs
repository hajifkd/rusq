extern crate num;
extern crate rand;

use num::complex::Complex;

#[derive(Debug, PartialEq, Eq)]
enum Eigenstate {
    Zero,
    One,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MeasuredResult {
    Zero,
    One,
}

pub struct Qubit {
    index: usize,
}

impl Qubit {
    pub fn measure(&self, sim: &mut QuantumSimulator) -> MeasuredResult {
        let prob0: f64 = sim.states
            .iter()
            .filter(|&x| x.is_orthogonal(&self, Eigenstate::Zero))
            .map(|x| x.norm_sqr())
            .sum();

        if prob0 < rand::random::<f64>() {
            sim.states
                .retain(|x| x.is_orthogonal(&self, Eigenstate::Zero));
            sim.normalize(prob0.sqrt());

            MeasuredResult::Zero
        } else {
            sim.states
                .retain(|x| x.is_orthogonal(&self, Eigenstate::One));
            sim.normalize((1. - prob0).sqrt());

            MeasuredResult::One
        }
    }
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
    states: Vec<QuantumState>,
}

impl QuantumSimulator {
    fn normalize(&mut self, n: f64) {
        for state in self.states.iter_mut() {
            (*state).coeff /= n;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
