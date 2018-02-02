#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate ndarray;
extern crate num;
extern crate rand;

pub mod simulator;
pub mod gates;

#[derive(Debug, PartialEq, Eq)]
pub enum MeasuredResult {
    Zero,
    One,
}

pub struct Qubit {
    pub index: usize,
}

pub trait QuantumMachine {
    fn get_qubits(&self) -> Vec<Qubit>;
    fn measure(&mut self, qubit: &Qubit) -> MeasuredResult;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
