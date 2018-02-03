extern crate rusq;

use rusq::prelude::*;

fn set(sim: &mut QuantumSimulator, qubit: &Qubit, r: MeasuredResult) {
    if sim.measure(qubit) != r {
        sim.X(qubit);
    }
}

#[test]
fn single_qubit() {
    let sim = QuantumSimulator::new(1);
    let qubits = sim.get_qubits();

    assert_eq!(qubits.len(), 1);
}

#[test]
fn measure_qubit() {
    let mut sim = rusq::simulator::QuantumSimulator::new(1);
    let qubit = &sim.get_qubits()[0];

    set(&mut sim, qubit, MeasuredResult::Zero);
    assert_eq!(sim.measure(qubit), MeasuredResult::Zero);

    set(&mut sim, qubit, MeasuredResult::One);
    assert_eq!(sim.measure(qubit), MeasuredResult::One);
}

#[test]
fn operate_x() {
    let mut sim = rusq::simulator::QuantumSimulator::new(1);
    let qubit = &sim.get_qubits()[0];

    set(&mut sim, qubit, MeasuredResult::Zero);
    assert_eq!(sim.measure(qubit), MeasuredResult::Zero);
    sim.X(qubit);
    assert_eq!(sim.measure(qubit), MeasuredResult::One);
}

#[test]
fn operate_h() {
    let mut sim = rusq::simulator::QuantumSimulator::new(1);
    let qubit = &sim.get_qubits()[0];

    let measure_count = 10000;
    let mut count_zero = 0;

    for _ in 0..measure_count {
        set(&mut sim, qubit, MeasuredResult::Zero);
        sim.H(qubit);
        if sim.measure(qubit) == MeasuredResult::Zero {
            count_zero += 1;
        }
    }

    // 5 sigma - it is highly likely to be true.
    assert!(count_zero > 4500 && 5500 > count_zero);
}

#[test]
fn operate_cnot() {
    let mut sim = rusq::simulator::QuantumSimulator::new(2);
    let qubits = sim.get_qubits();

    set(&mut sim, &qubits[0], MeasuredResult::One);
    assert_eq!(sim.measure(&qubits[0]), MeasuredResult::One);

    set(&mut sim, &qubits[1], MeasuredResult::Zero);
    assert_eq!(sim.measure(&qubits[1]), MeasuredResult::Zero);
    sim.CNOT(&qubits[0], &qubits[1]);
    assert_eq!(sim.measure(&qubits[1]), MeasuredResult::One);
}

#[test]
fn epr_pair() {
    let mut sim = rusq::simulator::QuantumSimulator::new(2);
    let qubits = sim.get_qubits();
    let measure_count = 10000;

    for _ in 0..measure_count {
        set(&mut sim, &qubits[0], MeasuredResult::Zero);
        set(&mut sim, &qubits[1], MeasuredResult::Zero);

        sim.H(&qubits[0]);
        sim.CNOT(&qubits[0], &qubits[1]);

        assert_eq!(sim.measure(&qubits[0]), sim.measure(&qubits[1]));
    }
}

#[test]
fn operate_ccnot() {
    let mut sim = rusq::simulator::QuantumSimulator::new(3);
    let qubits = sim.get_qubits();

    set(&mut sim, &qubits[0], MeasuredResult::One);
    assert_eq!(sim.measure(&qubits[0]), MeasuredResult::One);

    set(&mut sim, &qubits[1], MeasuredResult::One);
    assert_eq!(sim.measure(&qubits[1]), MeasuredResult::One);

    set(&mut sim, &qubits[2], MeasuredResult::Zero);
    assert_eq!(sim.measure(&qubits[2]), MeasuredResult::Zero);
    sim.CCNOT(&qubits[0], &qubits[1], &qubits[2]);
    assert_eq!(sim.measure(&qubits[2]), MeasuredResult::One);

    sim.CCNOT(&qubits[0], &qubits[1], &qubits[2]);
    assert_eq!(sim.measure(&qubits[2]), MeasuredResult::Zero);

    set(&mut sim, &qubits[0], MeasuredResult::One);
    set(&mut sim, &qubits[1], MeasuredResult::Zero);
    sim.CCNOT(&qubits[0], &qubits[1], &qubits[2]);
    assert_eq!(sim.measure(&qubits[2]), MeasuredResult::Zero);

    set(&mut sim, &qubits[0], MeasuredResult::Zero);
    set(&mut sim, &qubits[1], MeasuredResult::One);
    sim.CCNOT(&qubits[0], &qubits[1], &qubits[2]);
    assert_eq!(sim.measure(&qubits[2]), MeasuredResult::Zero);

    set(&mut sim, &qubits[0], MeasuredResult::Zero);
    set(&mut sim, &qubits[1], MeasuredResult::Zero);
    sim.CCNOT(&qubits[0], &qubits[1], &qubits[2]);
    assert_eq!(sim.measure(&qubits[2]), MeasuredResult::Zero);
}
