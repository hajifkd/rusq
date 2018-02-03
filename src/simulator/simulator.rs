use num::complex::Complex;
use rand;
use {MeasuredResult, QuantumMachine, Qubit};
use gates::single::SingleGateApplicator;
use gates::double::DoubleGateApplicator;
use gates::triple::TripleGateApplicator;
use ndarray::prelude::*;

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

    fn apply(&mut self, qubits: &[&Qubit], matrix: &Array2<Complex<f64>>) {
        let dim = qubits.len();

        let masks = mask_vec(&mut qubits.iter().map(|&x| x).collect::<Vec<_>>());
        for i in 0..(self.states.len() >> dim) {
            let indices = indices_vec(i, qubits, &masks, dim);
            let values = indices.iter().map(|&i| self.states[i]).collect::<Vec<_>>();
            let new_values = matrix.dot(&arr1(&values));
            for (&i, nv) in indices.iter().zip(new_values.to_vec()) {
                self.states[i] = nv;
            }
        }
    }
}

fn mask_pair(qubit: &Qubit) -> (usize, usize) {
    let upper_mask = 0xFFFF_FFFF_FFFF_FFFFusize << (qubit.index + 1);
    let lower_mask = !(0xFFFF_FFFF_FFFF_FFFFusize << qubit.index);
    (upper_mask, lower_mask)
}

#[inline]
fn index_pair(index: usize, qubit: &Qubit, upper_mask: usize, lower_mask: usize) -> (usize, usize) {
    let index_zero = ((index << 1) & upper_mask) | (index & lower_mask);
    let index_one = index_zero | (1usize << qubit.index);
    (index_zero, index_one)
}

fn mask_vec(qubits: &mut [&Qubit]) -> Vec<usize> {
    qubits.sort_by(|a, b| a.index.cmp(&b.index));
    let mut res = vec![0; qubits.len() + 1];

    res[0] = 0xFFFF_FFFF_FFFF_FFFFusize << (qubits[qubits.len() - 1].index + 1);

    for i in 1..qubits.len() {
        res[i] = (0xFFFF_FFFF_FFFF_FFFFusize << (qubits[qubits.len() - i - 1].index + 1))
            | (!(0xFFFF_FFFF_FFFF_FFFFusize << (qubits[qubits.len() - i].index)));
    }

    res[qubits.len()] = !(0xFFFF_FFFF_FFFF_FFFFusize << qubits[0].index);

    res
}

fn indices_vec(index: usize, qubits: &[&Qubit], mask: &[usize], dim: usize) -> Vec<usize> {
    let ims = (0..dim + 1)
        .map(|s| (index << (dim - s)) & mask[s])
        .collect::<Vec<_>>();
    (0..1 << dim)
        .map(|i| {
            (0..dim).fold(0, |acc, j| {
                acc | ims[j] | ((i >> (dim - 1 - j) & 0b1) << qubits[j].index)
            }) | ims[dim]
        })
        .collect()
}

impl QuantumMachine for QuantumSimulator {
    fn measure(&mut self, qubit: &Qubit) -> MeasuredResult {
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

    fn get_qubits(&self) -> Vec<Qubit> {
        (0..self.dimension).map(|x| Qubit { index: x }).collect()
    }
}

impl SingleGateApplicator for QuantumSimulator {
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit) {
        self.apply(&[qubit], matrix);
    }
}

impl DoubleGateApplicator for QuantumSimulator {
    fn apply_double(&mut self, matrix: &Array2<Complex<f64>>, qubit1: &Qubit, qubit2: &Qubit) {
        self.apply(&[qubit1, qubit2], matrix);
    }
}

impl TripleGateApplicator for QuantumSimulator {
    fn apply_triple(
        &mut self,
        matrix: &Array2<Complex<f64>>,
        qubit1: &Qubit,
        qubit2: &Qubit,
        qubit3: &Qubit,
    ) {
        self.apply(&[qubit1, qubit2, qubit3], matrix);
    }
}

#[cfg(test)]
mod tests {}
