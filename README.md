# Rusq
[![Build Status](https://travis-ci.org/hajifkd/rusq.svg?branch=master)](https://travis-ci.org/hajifkd/rusq)

Quantum computing simulator in Rust

## Overview

Rusq is a library to simulate a gate-type quantum computer written purely in Rust.

The Rusq design is highly inspired by [Q#](https://docs.microsoft.com/en-us/quantum/). Basically, only "physically safe" operation is implemented - no non-physical information like the amplitude of a wavefunction can be obtained by default.

### Feature

* Measurement and projection
* Basic quantum gates
    * $X, Y, Z$, Hadamard, identity, CNOT, CCNOT

## How to use

First, add the following to `Cargo.toml`

```toml
[dependencies]
rusq = { version = "*", git = "https://github.com/hajifkd/rusq.git" }
```

For example codes, check out the following section.

## Examples

For instance, you can make EPR pair like this:

```rust
extern crate rusq;

use rusq::prelude::*;

fn set(sim: &mut QuantumSimulator, qubit: &Qubit, r: MeasuredResult) {
    if sim.measure(qubit) != r {
        sim.X(qubit);
    }
}

fn main() {
    let mut sim = QuantumSimulator::new(2);
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
```

For other kinds of examples, check [tests](https://github.com/hajifkd/rusq/tree/master/tests) directory. Pull requests to implement another examples of quantum computation is welcome.

## License

MIT