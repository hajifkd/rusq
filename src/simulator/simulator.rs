use num::complex::Complex;
use rand;
use {MeasuredResult, QuantumMachine, Qubit};
use gates::single::{SingleGate, SingleGateApplicator};
use gates::double::{DoubleGate, DoubleGateApplicator};
use ndarray::prelude::arr1;

pub struct QuantumSimulator {
    dimension: usize,
    states: Vec<Complex<f64>>,
}

impl QuantumSimulator {
    pub fn new(n: usize) -> QuantumSimulator {
        let mut states = vec![Complex::new(0., 0.); 1 << n];
        states[0] = Complex::new(1., 0.);

        QuantumSimulator {
            dimension: n,
            states: states,
        }
    }
}

fn mask_pair(qubit: &Qubit) -> (usize, usize) {
    let upper_mask = 0xFFFF_FFFF_FFFF_FFFFusize << (qubit.index + 1);
    let lower_mask = !(0xFFFF_FFFF_FFFF_FFFFusize << qubit.index);
    (upper_mask, lower_mask)
}

fn double_mask_tuple(qubit1: &Qubit, qubit2: &Qubit) -> (usize, usize, usize) {
    if qubit1.index > qubit2.index {
        double_mask_tuple(qubit2, qubit1)
    } else {
        let upper_mask = 0xFFFF_FFFF_FFFF_FFFFusize << (qubit2.index + 1);
        let middle_mask = ((0xFFFF_FFFF_FFFF_FFFFusize << (qubit1.index + 2)) | (!upper_mask)) >> 1;
        let lower_mask = !(0xFFFF_FFFF_FFFF_FFFFusize << qubit1.index);
        (upper_mask, middle_mask, lower_mask)
    }
}

#[inline]
fn index_pair(index: usize, qubit: &Qubit, upper_mask: usize, lower_mask: usize) -> (usize, usize) {
    let index_zero = ((index << 1) & upper_mask) | (index & lower_mask);
    let index_one = index_zero | (1usize << qubit.index);
    (index_zero, index_one)
}

#[inline]
fn double_indices_vec(
    index: usize,
    qubit1: &Qubit,
    qubit2: &Qubit,
    upper_mask: usize,
    middle_mask: usize,
    lower_mask: usize,
) -> Vec<usize> {
    (0..4)
        .map(|i| {
            upper_mask | ((i >> 1) << qubit1.index) | middle_mask | ((i & 0b01) << qubit2.index)
                | lower_mask
        })
        .collect()
}

impl QuantumMachine for QuantumSimulator {
    fn measure(&mut self, qubit: &Qubit) -> MeasuredResult {
        if self.dimension == 1 {
            if self.states[0].norm_sqr() > rand::random::<f64>() {
                self.states[0] = Complex::new(1., 0.);
                self.states[1] = Complex::new(0., 0.);
                MeasuredResult::Zero
            } else {
                self.states[1] = Complex::new(1., 0.);
                self.states[0] = Complex::new(0., 0.);
                MeasuredResult::One
            }
        } else {
            let (upper_mask, lower_mask) = mask_pair(qubit);
            let zero_norm_sqr: f64 = (0..(self.states.len() >> 1))
                .map(|i| self.states[index_pair(i, qubit, upper_mask, lower_mask).0].norm_sqr())
                .sum();

            if zero_norm_sqr > rand::random::<f64>() {
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

impl DoubleGateApplicator for QuantumSimulator {
    fn apply_double(&mut self, gate: &DoubleGate, qubit1: &Qubit, qubit2: &Qubit) {
        if self.dimension == 2 {
            self.states = gate.matrix.dot(&arr1(&self.states)).to_vec();
        } else {
            let (upper_mask, middle_mask, lower_mask) = double_mask_tuple(qubit1, qubit2);
            for i in 0..(self.states.len() >> 2) {
                let indices =
                    double_indices_vec(i, qubit1, qubit2, upper_mask, middle_mask, lower_mask);
                let values = indices.iter().map(|i| self.states[*i]).collect::<Vec<_>>();
                let new_value = gate.matrix.dot(&arr1(&values));
                for (i, nv) in indices.iter().zip(new_value.to_vec()) {
                    self.states[*i] = nv;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Qubit;

    #[test]
    fn test_mask_pair() {
        let qubit = Qubit { index: 12 };
        let (upper_mask, lower_mask) = mask_pair(&qubit);
        assert_eq!(
            upper_mask,
            0b11111111_11111111_11111111_11111111_11111111_11111111_11100000_00000000usize
        );

        assert_eq!(
            lower_mask,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00001111_11111111usize
        )
    }

    #[test]
    fn test_index_pair() {
        let qubit = Qubit { index: 13 };
        let (upper_mask, lower_mask) = mask_pair(&qubit);
        let (iz, io) = index_pair(
            0b01011101_11111011_11011111usize,
            &qubit,
            upper_mask,
            lower_mask,
        );
        assert_eq!(iz, 0b10111011_11011011_11011111usize);

        assert_eq!(io, 0b10111011_11111011_11011111usize);
    }
}
