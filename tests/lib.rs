extern crate rusq;

use rusq::prelude::*;

#[test]
fn single_qubit() {
    let sim = rusq::simulator::QuantumSimulator::new(1);
    let qubits = sim.get_qubits();

    assert_eq!(qubits.len(), 1);
}

#[test]
fn measure_qubit() {
    let mut sim = rusq::simulator::QuantumSimulator::new(1);
    let qubit = &sim.get_qubits()[0];

    assert_eq!(sim.measure(qubit), MeasuredResult::Zero);
}
