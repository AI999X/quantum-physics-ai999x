# Quantum Physics: the desktop app, user guide

**Quantum Physics**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Quantum Physics covers special relativity, photons and matter waves, the hydrogen atom, quantum wells and tunnelling, spin and qubits, quantum gates, nuclear and particle physics and statistical physics. Its desktop app holds all 76 calculations in 11 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-quantum-physics-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-quantum-physics-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-quantum-physics-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-quantum-physics.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-quantum-physics-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-quantum-physics-1.6.5-linux-x86_64.tar.gz
./ai999x-quantum-physics-1.6.5-linux-x86_64/ai999x-quantum-physics
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Quantum Physics app, opened on Lorentz factor with its worked example.](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/hero.webp)

*The Quantum Physics app, opened on Lorentz factor with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Special relativity › Lorentz factor (or type part of its name in the search box). It opens with a worked example filled in: “Lorentz factor: v = 0.6 c” The answer: gamma = 1.25, beta = 0.6, gamma_minus_1 = 0.25.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Lorentz factor: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-1.webp)

*Lorentz factor: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Lorentz factor: v = 0.8 c”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Lorentz factor
gamma = 1.666666667
beta = 0.8
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Photon from a hydrogen-like transition (Rydberg formula)](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-2.webp)

*Photon from a hydrogen-like transition (Rydberg formula)*

![Photoelectric effect](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-3.webp)

*Photoelectric effect*

![Two-qubit circuit: probabilities and entanglement](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-4.webp)

*Two-qubit circuit: probabilities and entanglement*

![Transmission through a rectangular barrier](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-5.webp)

*Transmission through a rectangular barrier*

![Clebsch-Gordan coefficient](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-6.webp)

*Clebsch-Gordan coefficient*

![Semi-empirical mass formula (liquid-drop binding energy)](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-7.webp)

*Semi-empirical mass formula (liquid-drop binding energy)*

![Absorbed dose and equivalent dose](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/scene-8.webp)

*Absorbed dose and equivalent dose*

![Lorentz factor: a chart, enlarged.](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/chart-1.webp)

*Lorentz factor: a chart, enlarged.*

![Photon from a hydrogen-like transition (Rydberg formula): a chart, enlarged.](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/chart-2.webp)

*Photon from a hydrogen-like transition (Rydberg formula): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Quantum Physics app in the dark theme.](https://www.ai999x.com/?ax_shot=quantum-physics-ai999x/hero-dark.webp)

*The Quantum Physics app in the dark theme.*

## 7. Every calculation

76 calculations in 11 topics; each has its own entry on the model's page (https://www.ai999x.com/models/quantum-physics-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Angular momentum | Clebsch-Gordan coefficient, Wigner 3j symbol |
| Hydrogen atom | Energy level of a hydrogen-like atom (Bohr model), Hydrogen-like transition with the reduced-mass correction, K-alpha X-ray line (Moseley's law), Landé g factor and anomalous Zeeman shift, Photon from a hydrogen-like transition (Rydberg formula), Probability of finding the electron within a radius, Radial expectation values of a hydrogen-like state |
| Nuclear and particle physics | Kinematics of a two-body decay, Semi-empirical mass formula (liquid-drop binding energy) |
| Nuclear physics | Absorbed dose and equivalent dose, Age from the remaining fraction (radioactive dating), Coulomb barrier between two nuclei, Decay of a named radionuclide, Parent-daughter decay chain (Bateman), Q value and threshold energy of a nuclear reaction, Radioactive decay law |
| Particle physics | Mean decay length of a relativistic unstable particle |
| Photons and matter waves | Compton scattering, de Broglie wavelength of a moving particle, de Broglie wavelength of an accelerated particle, Minimum momentum uncertainty (Heisenberg), Photoelectric effect, Photon energy from wavelength, Photon-number distribution of a coherent state |
| Quantum gates | Bloch-sphere rotation of a single-qubit gate sequence, State of a qubit after a sequence of gates, Success probability of Grover's search, Two-qubit circuit: probabilities and entanglement |
| Quantum wells | Bound states of a finite square well, Energy levels of a particle in a box, Photon from a particle-in-a-box transition, Position and momentum spread in a box, Probability beyond the classical turning points (harmonic oscillator), Probability of finding a particle in part of a box, Quantum harmonic oscillator levels, Reflection and transmission at a potential step, Transmission through a rectangular barrier, WKB tunnelling through a triangular barrier (field emission) |
| Special relativity | Centre-of-mass energy of a fixed-target collision, Invariant mass of two particles, Length contraction, Lorentz factor, Relativistic aberration of light, Relativistic Doppler shift, Relativistic energy from momentum, Relativistic momentum and energy, Relativistic velocity addition, Rest energy (mass-energy equivalence), Spacetime interval between two events, Speed from relativistic kinetic energy, Time dilation |
| Spin and qubits | CHSH Bell-test value for the singlet state, Fidelity and trace distance of two qubit states, Larmor precession of a spin-1/2, Purity and entropy of a qubit state, Qubit state on the Bloch sphere, Rabi oscillation of a driven two-level system, Spin rotation amplitude (Wigner small-d matrix), Spin-1/2 measurement along an axis |
| Statistical physics | Bose-Einstein condensation temperature, Bose-Einstein occupation of a mode, Chemical potential of an ideal Fermi gas, Debye heat capacity of a solid, Energy and photon density of blackbody radiation, Fermi energy of a free-electron gas, Fermi energy of a metal from its density, Fermi-Dirac occupation of a state, Fraction of blackbody radiation in a wavelength band, Peak of blackbody radiation (Wien's displacement law), Quantum oscillator thermodynamics (Einstein model), Radiated power (Stefan-Boltzmann law), Spectral radiance of a blackbody (Planck's law), Thermal de Broglie wavelength, Thermodynamics of a two-level system |

## 8. Other ways to use it

The same engine runs on the command line (`quantum-physics`, its own guide), in Python (`pip install quantum-physics-ai999x`, `import quantum_physics_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/quantum-physics-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/quantum-physics-ai999x/
