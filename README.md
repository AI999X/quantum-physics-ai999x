# quantum-physics-ai999x — offline, verified quantum physics solver with step-by-step answers

[![tests](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/quantum-physics-ai999x)](https://pypi.org/project/quantum-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/quantum-physics-ai999x)

**[Install](https://pypi.org/project/quantum-physics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/quantum-physics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/quantum-physics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

quantum-physics-ai999x is an offline, deterministic Python solver that answers quantum physics questions with verified results from 75 certified capabilities, gives 376 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**quantum-physics-ai999x** is a compact, deterministic quantum physics engine for Python. It reads a question in plain English (or a typed call), picks a certified quantum physics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified quantum physics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 376 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** special relativity, photons and matter waves, the hydrogen atom and angular momentum, quantum wells and tunnelling, spin, qubits and quantum gates, nuclear and particle physics, statistical physics.

## Model overview

| Property | Value |
|---|---|
| Package | `quantum-physics-ai999x` 1.0.0 (import `quantum_physics_ai999x`) |
| Type | Deterministic neuro-symbolic quantum physics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 75 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 376 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install quantum-physics-ai999x
```

```python
import quantum_physics_ai999x as quantum_physics

quantum_physics.ask("Lorentz factor: v = 0.8 c").show()
# -> SOLVED — gamma = 1.66667, beta = 0.8, gamma_minus_1 = 0.666667  (lorentz_factor)
quantum_physics.ask("Hydrogen transition: Z = 1, n_upper = 3, n_lower = 2").show()
# -> SOLVED — E_photon_eV = 1.88968, lam = 6.56112e-07, lambda_nm = 656.112, f = 4.56922e+14  (hydrogen_transition)
quantum_physics.ask("Photoelectric effect: lambda = 250 nm, W = 4.3 eV").show()
# -> SOLVED — E_photon_eV = 4.95937, K_max_eV = 0.659368, V_stop = 0.659368, lambda_threshold = 2.88335e-07  (photoelectric_effect)
quantum_physics.solve("two_qubit_circuit", start="00", gates="H0 CNOT01").show()
# -> SOLVED — p00 = 0.5, p01 = 0, p10 = 0, p11 = 0.5  (two_qubit_circuit)
quantum_physics.ask("Barrier transmission: E = 1 eV, V0 = 2 eV, a = 0.5 nm, m = 9.109e-31 kg").show()
# -> SOLVED — T = 0.0235496, R = 0.97645, kappa_a = 2.56153, regime = tunnelling (E < V0)  (barrier_transmission)
quantum_physics.ask("Who created you?").message
# -> quantum-physics-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `quantum-physics-ai999x "<question>"`, `quantum-physics-ai999x --capabilities`. The outputs above were produced by running this exact code on quantum-physics-ai999x 1.0.0.

## Find the right capability, and draw it

`quantum_physics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "quantum-physics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import quantum_physics_ai999x as quantum_physics

quantum_physics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

75 certified procedures. A few of them:

- **Procedures:** Age from the remaining fraction (radioactive dating); Bloch-sphere rotation of a single-qubit gate sequence; Bose-Einstein condensation temperature; Bose-Einstein occupation of a mode; Bound states of a finite square well; CHSH Bell-test value for the singlet state; Centre-of-mass energy of a fixed-target collision; Chemical potential of an ideal Fermi gas; Clebsch-Gordan coefficient; Compton scattering; Coulomb barrier between two nuclei; Debye heat capacity of a solid; Decay of a named radionuclide; Energy and photon density of blackbody radiation; Energy level of a hydrogen-like atom (Bohr model); Energy levels of a particle in a box; Fermi energy of a free-electron gas; Fermi energy of a metal from its density; Fermi-Dirac occupation of a state; Fidelity and trace distance of two qubit states; Fraction of blackbody radiation in a wavelength band; Hydrogen-like transition with the reduced-mass correction; Invariant mass of two particles; K-alpha X-ray line (Moseley's law) …

`quantum_physics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified quantum physics capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (special relativity, photons and matter waves, the hydrogen atom and angular momentum, quantum wells and tunnelling, spin, qubits and quantum gates, nuclear and particle physics, statistical physics); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{quantum_physics_ai999x_1_0_0,
  title  = {quantum-physics-ai999x: an offline, verified quantum physics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/quantum-physics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/quantum-physics-ai999x/discussions or visit https://ai999x.com.

## Support the project

quantum-physics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/quantum-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/quantum-physics-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/quantum-physics-ai999x/tree/main/onnx).
