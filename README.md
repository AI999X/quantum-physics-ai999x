# Quantum Physics (quantum-physics-ai999x)

[![tests](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/quantum-physics-ai999x)](https://pypi.org/project/quantum-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/quantum-physics-ai999x)

Quantum Physics covers special relativity, photons and matter waves, the hydrogen atom, quantum wells and tunnelling, spin and qubits, quantum gates, nuclear and particle physics and statistical physics: 76 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Quantum Physics desktop app: Debye heat capacity of a solid, with its worked example and live scene](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/hero.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `quantum-physics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#try) |
| **Python** 3.10+ | `pip install quantum-physics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 12 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import quantum_physics_ai999x as quantum_physics

a = quantum_physics.ask(
    "Wigner 3j symbol: j1 = 1, m1 = 1, j2 = 1, m2 = -1, j3 = 0, m3 = 0"
)
print(a.outcome, a.values)
# SOLVED {'w3j': 0.57735026919, 'w3j_squared': 0.333333333333}

a = quantum_physics.ask(
    "Invariant mass: E1 = 50; p1 = 30, 0, 40; E2 = 50; p2 = -30, 0, "
    "-40"
)
print(a.outcome, a.values)
# SOLVED {'M': 100.0, 'P': 0.0, 'beta_cm': 0.0, 'gamma_cm': 1.0}

a = quantum_physics.ask(
    "Coulomb barrier: Z1 = 1, A1 = 1, Z2 = 1, A2 = 1, r0 = 1 fm"
)
print(a.outcome, a.values)
# SOLVED {'V_MeV': 0.71998227343, 'R': 2e-15, 'R_fm': 2.0}

a = quantum_physics.ask("Length contraction: L0 = 100 m, v = 0.6 c")
print(a.outcome, a.values)
# SOLVED {'L': 80.0, 'gamma': 1.25, 'shortening': 20.0}

a = quantum_physics.ask("Lorentz factor: v = 0.6 c")
print(a.outcome, a.values)
# SOLVED {'gamma': 1.25, 'beta': 0.6, 'gamma_minus_1': 0.25}

a = quantum_physics.ask(
    "Particle in a box probability: n = 1, L = 1 nm, x1 = 0.25 nm, x2 "
    "= 0.75 nm"
)
print(a.outcome, a.values)
# SOLVED {'P': 0.818309886184, 'classical': 0.5}

a = quantum_physics.ask(
    "Relativistic Doppler shift: f = 1e14 Hz, v = 0.6 c, motion = "
    "approaching"
)
print(a.outcome, a.values)
# SOLVED {'f_obs': 200000000000000.0, 'ratio': 2.0, 'z': -0.5}

a = quantum_physics.ask(
    "Fermi-Dirac occupation: E = 5 eV, mu = 5 eV, T = 300 K"
)
print(a.outcome, a.values)
# SOLVED {'f': 0.5, 'one_minus_f': 0.5, 'x': 0.0}

a = quantum_physics.ask(
    "Spin rotation amplitude: j = 0.5, mp = 0.5, m = 0.5, beta = 90 "
    "deg"
)
print(a.outcome, a.values)
# SOLVED {'d': 0.707106781187, 'probability': 0.5}

a = quantum_physics.ask(
    "Spin measurement: theta = 60 deg, phi = 0 deg, theta_n = 0 deg, "
    "phi_n = 0 deg"
)
print(a.outcome, a.values)
# SOLVED {'P_up': 0.75, 'P_down': 0.25, 'expectation': 0.5}

a = quantum_physics.ask("Velocity addition: u_prime = 0.5 c, v = 0.5 c")
print(a.outcome, a.values)
# SOLVED {'u': 239833966.4, 'beta': 0.8}

a = quantum_physics.ask("Time dilation: t0 = 1 s, v = 0.6 c")
print(a.outcome, a.values)
# SOLVED {'t': 1.25, 'gamma': 1.25, 'difference': 0.25}
```

## Screenshots

[![The Quantum Physics desktop app: Debye heat capacity of a solid, with its worked example and live scene](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/hero-dark.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/)

[![Relativistic velocity addition: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/see-1.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/)

*Relativistic velocity addition*

[![Probability beyond the classical turning points (harmonic oscillator): the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/see-2.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/)

*Probability beyond the classical turning points (harmonic oscillator)*

[![Purity and entropy of a qubit state: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/see-3.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/)

*Purity and entropy of a qubit state*

Every calculation, with its formula, a worked example and its figures: **[Quantum Physics on ai999x.com](https://www.ai999x.com/models/quantum-physics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/quantum-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/quantum-physics-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `quantum-physics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/quantum-physics`) and its data (`rust/models/quantum_physics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p quantum-physics-ai999x
./target/release/quantum-physics --help
cargo test -p quantum-physics-ai999x
```
