# Quantum Physics (quantum-physics-ai999x)

[![tests](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/quantum-physics-ai999x)](https://pypi.org/project/quantum-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/quantum-physics-ai999x)

Quantum Physics covers special relativity, photons and matter waves, the hydrogen atom, quantum wells and tunnelling, spin and qubits, quantum gates, nuclear and particle physics and statistical physics: 76 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Quantum Physics desktop app: Lorentz factor, with its worked example and live scene](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/hero.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-quantum-physics-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-quantum-physics-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `quantum-physics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install quantum-physics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Quantum Physics desktop app: every scene and chart is drawn from the computed answer.

[![Quantum Physics desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/clip.mp4)*

| | |
|---|---|
| [![Lorentz factor: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-1.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-lorentz_factor)<br>Lorentz factor | [![Photon from a hydrogen-like transition (Rydberg formula): the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-2.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-hydrogen_transition)<br>Photon from a hydrogen-like transition (Rydberg formula) |
| [![Photoelectric effect: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-3.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-photoelectric_effect)<br>Photoelectric effect | [![Two-qubit circuit: probabilities and entanglement: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-4.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-two_qubit_circuit)<br>Two-qubit circuit: probabilities and entanglement |
| [![Transmission through a rectangular barrier: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-5.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-barrier_transmission)<br>Transmission through a rectangular barrier | [![Clebsch-Gordan coefficient: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-6.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-clebsch_gordan)<br>Clebsch-Gordan coefficient |
| [![Semi-empirical mass formula (liquid-drop binding energy): the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-7.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-semi_empirical_mass_formula)<br>Semi-empirical mass formula (liquid-drop binding energy) | [![Absorbed dose and equivalent dose: the live scene in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-8.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-radiation_dose)<br>Absorbed dose and equivalent dose |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Lorentz factor: a chart in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/chart-1.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-lorentz_factor)<br>Lorentz factor | [![Photon from a hydrogen-like transition (Rydberg formula): a chart in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/chart-2.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-hydrogen_transition)<br>Photon from a hydrogen-like transition (Rydberg formula) | [![Photoelectric effect: a chart in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/chart-3.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-photoelectric_effect)<br>Photoelectric effect | [![Two-qubit circuit: probabilities and entanglement: a chart in the Quantum Physics desktop app](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/chart-4.webp)](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-two_qubit_circuit)<br>Two-qubit circuit: probabilities and entanglement |

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

## Command line: 15 examples

Each command below was run with `quantum-physics` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ quantum-physics solve clebsch_gordan j1=0.5 m1=0.5 j2=0.5 m2=-0.5 J=1 M=0
cg = 0.7071067812
cg_squared = 0.5
cg_squared_exact = 1/2

$ quantum-physics solve bohr_energy_level Z=1 n=1
E_eV = -13.60569312 eV
E = -2.179872361e-18 J
ionization_eV = 13.60569312 eV
r = 5.291772105e-11 m
r_nm = 0.05291772105 nm
v = 2187691.262 m/s

$ quantum-physics solve two_body_decay M=139.57039 m1=105.6583755 m2=0
Q_MeV = 33.9120145 MeV
p_MeV = 29.79214091 MeV
E1_MeV = 109.7782491 MeV
E2_MeV = 29.79214091 MeV
K1_MeV = 4.11987359 MeV
K2_MeV = 29.79214091 MeV

$ quantum-physics solve radiation_dose "E=2 mJ" "m=500 g" w_R=20
D = 0.004 J/kg
H = 0.08 J/kg
D_mGy = 4
H_mSv = 80
H_rem = 8

$ quantum-physics solve particle_decay_length particle=mu- p=0.1056583755
gamma = 1.414213562
beta = 0.7071067812
beta_gamma = 1
lifetime_lab = 3.107000574e-06 s
mean_decay_length = 658.6383866 m
proper_lifetime = 2.196981175e-06 s

$ quantum-physics solve compton_scattering "lam=71 pm" "theta=90 deg"
lambda_prime = 7.342631024e-11 m
lambda_prime_pm = 73.42631024 pm
shift_pm = 2.426310235 pm
E_in_keV = 17.46256316 keV
E_out_keV = 16.88552755 keV
K_electron_keV = 0.5770356102 keV

$ quantum-physics solve gate_rotation_axis gates=H
angle = 3.141592654 rad
angle_deg = 180
nx = 0.7071067812
ny = 0
nz = 0.7071067812

$ quantum-physics solve finite_square_well "V0=10 eV" "a=0.5 nm" "m=9.109383714e-31 kg"
n_bound = 3
E1_eV = 0.9610429053 eV
energies_eV = [0.961042905296,3.75528284703,7.95049450889]
z0 = 4.050219315

$ quantum-physics solve fixed_target_cm_energy m1=1 E1=7 m2=1
sqrt_s = 4
available = 2
beta_cm = 0.8660254038
gamma_cm = 2

$ quantum-physics solve chsh_bell_test "a=0 deg" "a2=90 deg" "b=45 deg" "b2=135 deg"
S = -2.828427125
abs_S = 2.828427125
E_ab = -0.7071067812
E_ab2 = 0.7071067812
E_a2b = -0.7071067812
E_a2b2 = -0.7071067812

$ quantum-physics solve bec_critical_temperature "n=1e+20 m^-3" "m=1.443e-25 kg"
T_c = 3.983751061e-07 K
lambda_c = 2.967188877e-07 m
n_lambda3 = 2.612375349

$ quantum-physics solve wigner_3j_symbol j1=1 m1=1 j2=1 m2=-1 J=0 M=0
w3j = 0.5773502692
w3j_squared = 0.3333333333

$ quantum-physics solve hydrogen_transition_reduced_mass Z=1 n_upper=3 n_lower=2 M_u=1.007276467
E_photon_eV = 1.888651009 eV
lam = 6.564696063e-07 m
lambda_nm = 656.4696063 nm
f = 4.566737822e+14 Hz
series = Balmer
mu_ratio = 0.9994556794

$ quantum-physics solve semi_empirical_mass_formula Z=26 A=56
B_MeV = 495.3836999 MeV
B_per_A_MeV = 8.846137499 MeV
mass_u = 55.93158207
volume_MeV = 882 MeV
surface_MeV = 260.5425666 MeV
coulomb_MeV = 120.7962953 MeV

$ quantum-physics solve radioactive_dating_age fraction=0.25 "T_half=5730 yr"
t = 3.61650096e+11 s
half_lives = 2
decay_constant = 3.833247596e-12 1/s
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Quantum Physics on ai999x.com](https://www.ai999x.com/models/quantum-physics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/quantum-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/quantum-physics-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `quantum-physics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/quantum-physics`) and its data (`rust/models/quantum_physics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p quantum-physics-ai999x
./target/release/quantum-physics --help
cargo test -p quantum-physics-ai999x
```
