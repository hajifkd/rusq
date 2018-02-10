//!
//! The main crate for Rusq.
//!
//! Rusq is a simulator for gate-type quantum computer in Rust.
//!
//! For a quick start, please see [README](https://github.com/hajifkd/rusq/blob/master/README.md) and
//! [some example codes](https://github.com/hajifkd/rusq/blob/master/tests/).
//!
//!

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate ndarray;
extern crate num;
extern crate rand;

pub mod simulator;
pub mod gates;
pub mod prelude;

///
/// A type for the result of the measurement of a qubit.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MeasuredResult {
    /// The qubit is measured as $|0\rangle$
    Zero,
    /// The qubit is measured as $|1\rangle$
    One,
}

///
/// A type for a qubit.
///
/// This qubit type just represents the index in a given quantum machine.
/// All "states" are carried by a type implementing [QuantumMachine](trait.QuantumMachine.html) trait.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Qubit {
    /// The index in a given quantum machine.
    pub index: usize,
}

///
/// A trait for types representing quantum machines.
///
/// The type implementing this must be responsible for the states - namely, the wavefunction.
///
/// Currently, only the supporting type is [QuantumSimulator](simulator/simulator/struct.QuantumSimulator.html).
///
pub trait QuantumMachine {
    /// Returns all the qubits in the machine.
    fn get_qubits(&self) -> Vec<Qubit>;

    /// Measures the given qubit.
    /// Note that the qubit is expected to be projected to the corresponding state.
    fn measure(&mut self, qubit: &Qubit) -> MeasuredResult;

    fn measure_x(&mut self, qubit: &Qubit) -> MeasuredResult;
}
