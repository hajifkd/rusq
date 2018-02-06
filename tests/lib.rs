extern crate permutohedron;
extern crate rusq;

use rusq::prelude::*;
use permutohedron::LexicalPermutation;

macro_rules! count {
    () => { 0 };

    ($e1: expr) => { 1 };

    ($e1: expr, $($e: expr),+) => {
        1 + count!($($e),*)
    };
}

// Well, is this too complicated as test?
// Maybe we need "examples" in addition.
macro_rules! logic {
    ( ($sim: ident, $qubits: ident, $gate: ident) => {} ) => {};

    ( ($sim: ident, $qubits: ident, $gate: ident) => { $($input: expr),+ => $($result: expr),+ ; $($e: tt)* } ) => {
        {
            let mut indices = (0..count!($($input),*)).collect::<Vec<usize>>();

            loop {
                let iperm = indices.to_vec();
                if !indices.next_permutation() {
                    break;
                }

                let mut index = -1;
                $({
                    index += 1;
                    set(&mut $sim, &$qubits[iperm[index as usize]],
                        if $input == 0 { MeasuredResult::Zero } else { MeasuredResult::One });
                });*

                let mut index = -1;
                $sim.$gate($({
                    index += 1;
                    let _ = $input;
                    &$qubits[iperm[index as usize]]
                }),*);

                let mut index = -1;
                $({
                    index += 1;
                    assert_eq!(if $result == 0 { MeasuredResult::Zero } else { MeasuredResult::One },
                               $sim.measure(&$qubits[iperm[index as usize]]));
                });*
            }
        }

        logic!(($sim, $qubits, $gate) => { $($e)* })
    };
}

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
fn operate_cnot() {
    let mut sim = rusq::simulator::QuantumSimulator::new(2);
    let qubits = sim.get_qubits();

    logic!((sim, qubits, CNOT) => {
        0, 0 => 0, 0;
        0, 1 => 0, 1;
        1, 0 => 1, 1;
        1, 1 => 1, 0;
    });
}

#[test]
fn operate_swap() {
    let mut sim = rusq::simulator::QuantumSimulator::new(2);
    let qubits = sim.get_qubits();

    logic!((sim, qubits, SWAP) => {
        0, 0 => 0, 0;
        0, 1 => 1, 0;
        1, 0 => 0, 1;
        1, 1 => 1, 1;
    });
}

#[test]
fn operate_ccnot() {
    let mut sim = rusq::simulator::QuantumSimulator::new(3);
    let qubits = sim.get_qubits();

    logic!((sim, qubits, CCNOT) => {
        0, 0, 0 => 0, 0, 0;
        0, 0, 1 => 0, 0, 1;
        0, 1, 0 => 0, 1, 0;
        0, 1, 1 => 0, 1, 1;
        1, 0, 0 => 1, 0, 0;
        1, 0, 1 => 1, 0, 1;
        1, 1, 0 => 1, 1, 1;
        1, 1, 1 => 1, 1, 0;
    });
}
