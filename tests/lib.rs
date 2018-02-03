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

#[test]
fn operate_x() {
    let mut sim = rusq::simulator::QuantumSimulator::new(1);
    let qubit = &sim.get_qubits()[0];

    assert_eq!(sim.measure(qubit), MeasuredResult::Zero);
    sim.X(qubit);
    assert_eq!(sim.measure(qubit), MeasuredResult::One);
}

#[test]
fn operate_h() {
    let mut sim = rusq::simulator::QuantumSimulator::new(1);
    let qubit = &sim.get_qubits()[0];

    let measure = 10000;
    let mut count_zero = 0;

    for _ in 0..measure {
        sim.H(qubit);
        if sim.measure(qubit) == MeasuredResult::Zero {
            count_zero += 1;
        }
    }

    // 5 sigma - it is highly likely to be true.
    assert!(count_zero > 4500 && 5500 > count_zero);
}
