use num::complex::Complex;
use rand;
use {MeasuredResult, QuantumMachine, Qubit};
use gates::{SingleGate, SingleGateApplicator};
use ndarray::prelude::*;

pub struct QuantumSimulator {
    dimension: usize,
    states: Vec<Complex<f64>>,
}

impl QuantumSimulator {
    pub fn new(n: usize) -> QuantumSimulator {
        let mut states = vec![Complex::new(0., 0.); 1 << (n - 1)];
        states[0] = Complex::new(1., 0.);

        QuantumSimulator {
            dimension: n,
            states: states,
        }
    }
}

#[inline]
fn mask_pair(qubit: &Qubit) -> (usize, usize) {
    let upper_mask: usize = (0xFFFF_FFFF_FFFF_FFFFu64 << (qubit.index + 1)) as _;
    let lower_mask: usize = (!upper_mask) >> 1;
    (upper_mask, lower_mask)
}

#[inline]
fn index_pair(index: usize, qubit: &Qubit, upper_mask: usize, lower_mask: usize) -> (usize, usize) {
    let index_zero = (index & upper_mask) | (index & lower_mask);
    let index_one = index_zero | (1usize << qubit.index);
    (index_zero, index_one)
}

impl QuantumMachine for QuantumSimulator {
    fn measure(&mut self, qubit: &Qubit) -> MeasuredResult {
        if self.dimension == 1 {
            if self.states[0].norm_sqr() < rand::random::<f64>() {
                self.states[0] = Complex::new(1., 0.);
                self.states[1] = Complex::new(0., 1.);
                MeasuredResult::Zero
            } else {
                self.states[1] = Complex::new(1., 0.);
                self.states[0] = Complex::new(0., 1.);
                MeasuredResult::One
            }
        } else {
            let (upper_mask, lower_mask) = mask_pair(qubit);
            let zero_norm_sqr: f64 = (0..(self.states.len() >> 1))
                .map(|i| self.states[index_pair(i, qubit, upper_mask, lower_mask).0].norm_sqr())
                .sum();

            if zero_norm_sqr < rand::random::<f64>() {
                let norm = zero_norm_sqr.sqrt();
                for i in 0..(self.states.len() >> 1) {
                    let (iz, io) = index_pair(i, qubit, upper_mask, lower_mask);
                    self.states[iz] /= norm;
                    self.states[io] = Complex::new(0., 0.);
                }
                MeasuredResult::Zero
            } else {
                let norm = (1. - zero_norm_sqr).sqrt();
                for i in 0..(self.states.len() >> 1) {
                    let (iz, io) = index_pair(i, qubit, upper_mask, lower_mask);
                    self.states[io] /= norm;
                    self.states[iz] = Complex::new(0., 0.);
                }
                MeasuredResult::One
            }
        }
    }

    fn get_qubits(&self) -> Vec<Qubit> {
        (0..self.dimension).map(|x| Qubit { index: x }).collect()
    }
}

impl SingleGateApplicator for QuantumSimulator {
    fn apply_single(&mut self, gate: &SingleGate, qubit: &Qubit) {
        if self.dimension == 1 {
            self.states = gate.matrix.dot(&arr1(&self.states)).to_vec();
        } else {
            let (upper_mask, lower_mask) = mask_pair(qubit);
            for i in 0..(self.states.len() >> 1) {
                let (iz, io) = index_pair(i, qubit, upper_mask, lower_mask);
                let new_value = gate.matrix.dot(&array![self.states[iz], self.states[io]]);
                self.states[iz] = new_value[0];
                self.states[io] = new_value[1];
            }
        }
    }
}
