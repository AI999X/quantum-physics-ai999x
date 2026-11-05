# quantum-physics-ai999x — offline, verified quantum physics solver with step-by-step answers

[![tests](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/quantum-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/quantum-physics-ai999x)](https://pypi.org/project/quantum-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/quantum-physics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/quantum-physics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/quantum-physics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/quantum-physics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/quantum-physics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Physical Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

quantum-physics-ai999x is an offline, deterministic Python solver that answers quantum physics questions with verified results from 75 certified capabilities, gives 376 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**quantum-physics-ai999x** is a compact, deterministic quantum physics engine for Python. It reads a question in plain English (or a typed call), picks a certified quantum physics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** quantum-physics-ai999x comes as the AI999X desktop app and as the command-line program `quantum-physics` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Quantum Physics desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-quantum-physics-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-quantum-physics-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`quantum-physics`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install quantum-physics-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Quantum Physics** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Quantum Physics among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-quantum-physics-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-quantum-physics-1.4.0-linux-x86_64.tar.gz
./ai999x-quantum-physics-1.4.0-linux-x86_64/ai999x-quantum-physics
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
quantum-physics --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
quantum-physics --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ quantum-physics ask "Lorentz factor: v = 0.8 c"
Read as: Lorentz factor
gamma = 1.666666667
beta = 0.8
gamma_minus_1 = 0.6666666667
  by: Lorentz factor  (gamma = 1/sqrt((1 - beta)(1 + beta)), beta = v/c)
  v = 239833966.4 m/s
  assumes: 0 <= v < c
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ quantum-physics info hydrogen_transition
Photon from a hydrogen-like transition (Rydberg formula)
  1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2)
  in   Z        atomic number (1 for hydrogen) (integer) []
  in   n_upper  upper level (integer) []
  in   n_lower  lower level (integer) []
  out  E_photon_eV photon energy [eV]
  out  lam      vacuum wavelength [m]
  out  lambda_nm vacuum wavelength in nm [nm]
  out  f        frequency [Hz]
  out  series   spectral series (Lyman, Balmer, ...) []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ quantum-physics solve hydrogen_transition Z=1 n_upper=3 n_lower=2
E_photon_eV = 1.8896796 eV
lam = 6.561122764e-07 m
lambda_nm = 656.1122764 nm
f = 4.569224945e+14 Hz
series = Balmer
  by: Photon from a hydrogen-like transition (Rydberg formula)  (1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2))
  assumes: n_upper > n_lower >= 1; infinitely heavy nucleus
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | quantum-physics batch

`cases.jsonl`:

```json
{"key": "hydrogen_transition", "inputs": {"Z": "1", "n_upper": "3", "n_lower": "2"}}
{"key": "lorentz_factor", "inputs": {"v": "239833966.4 m/s"}}
```

```console
$ quantum-physics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":656.112276419,"unit":"nm","solved_for":"lambda_nm","values":[["E_photon_eV",1.88967960042,"eV"],["lam",6.56112276419e-7,"m"],["lambda_nm",656.112276419,"nm"],["f",456922494479000.0,"Hz"],["series","Balmer",""]],"target":"lambda_nm","phrase":"photon energy","assumptions":["n_upper > n_lower >= 1","infinitely heavy nucleus"],"name":"Photon from a hydrogen-like transition (Rydberg formula)","equation":"1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2)"}
{"outcome":"SOLVED","message":"","value":1.66666666667,"unit":"","solved_for":"gamma","si":[["v",239833966.4]],"values":[["gamma",1.66666666667,""],["beta",0.8,""],["gamma_minus_1",0.666666666667,""]],"target":"gamma","phrase":"Lorentz factor","assumptions":["0 <= v < c"],"name":"Lorentz factor","equation":"gamma = 1/sqrt((1 - beta)(1 + beta)), beta = v/c"}
```

### In your browser

Open [Quantum Physics on ai999x.com](https://www.ai999x.com/models/quantum-physics-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `quantum_physics_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install quantum-physics-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Clebsch-Gordan coefficient

The coefficient <j1 m1; j2 m2 | J M> for coupling two angular momenta (Condon-Shortley convention), with its exact square.

Command line:

```console
$ quantum-physics solve clebsch_gordan j1=0.5 m1=0.5 j2=0.5 m2=-0.5 J=1 M=0
cg = 0.7071067812
cg_squared = 0.5
cg_squared_exact = 1/2
  by: Clebsch-Gordan coefficient  (Racah's formula in exact rational arithmetic; zero unless m1 + m2 = M and |j1 - j2| <= J <= j1 + j2)
  j1 = 0.5
  m1 = 0.5
  j2 = 0.5
  m2 = -0.5
  J = 1
  M = 0
  assumes: each j >= 0 and whole or half-whole; |m| <= j with j - m whole
```

Python:

```python
import quantum_physics_ai999x as m

a = m.solve("clebsch_gordan", j1="0.5", m1="0.5", j2="0.5", m2="-0.5", J="1", M="0")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-clebsch_gordan) and press **Compute in your browser**.

_What this means:_ This calculation determines the Clebsch-Gordan coefficient for coupling two angular momenta with values 0.5 and 0.5 to a total angular momentum of 1, using projections 0.5 and -0.5 that sum to 0. The resulting coefficient is 0.707107, which represents the amplitude of this specific coupled state in the product basis. Its exact square, 1/2, gives the probability of finding the system in this configuration under the Condon-Shortley convention.

### Wigner 3j symbol

The Wigner 3j symbol (j1 j2 J; m1 m2 M), nonzero only when m1 + m2 + M = 0.

Command line:

```console
$ quantum-physics solve wigner_3j_symbol j1=1 m1=1 j2=1 m2=-1 J=0 M=0
w3j = 0.5773502692
w3j_squared = 0.3333333333
  by: Wigner 3j symbol  (from the exact Clebsch-Gordan coefficient: (-1)^(j1 - j2 - M) <j1 m1; j2 m2 | J, -M> / sqrt(2J + 1))
  j1 = 1
  m1 = 1
  j2 = 1
  m2 = -1
  J = 0
  M = 0
  assumes: each j >= 0 and whole or half-whole; |m| <= j with j - m whole
```

Python:

```python
import quantum_physics_ai999x as m

a = m.solve("wigner_3j_symbol", j1="1", m1="1", j2="1", m2="-1", J="0", M="0")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-wigner_3j_symbol) and press **Compute in your browser**.

_What this means:_ This calculation determines the Wigner three j symbol, a key component in quantum mechanics for describing how angular momenta combine. Using the input values of first angular momentum 1, second angular momentum 1, and total angular momentum 0, along with their respective projections 1, -1, and 0, the algorithm derives the symbol from exact Clebsch-Gordan coefficients. The resulting value 0.57735 represents the probability amplitude for this specific coupling configuration, while 0.333333 indicates the relative strength or likelihood of this interaction occurring.

### Energy level of a hydrogen-like atom (Bohr model)

Energy, ionization energy, orbit radius and orbital speed of level n of a one-electron atom or ion with nuclear charge Z.

Command line:

```console
$ quantum-physics solve bohr_energy_level Z=1 n=1
E_eV = -13.60569312 eV
E = -2.179872361e-18 J
ionization_eV = 13.60569312 eV
r = 5.291772105e-11 m
r_nm = 0.05291772105 nm
v = 2187691.262 m/s
  by: Energy level of a hydrogen-like atom (Bohr model)  (E_n = -Z^2 R_inf h c / n^2, r_n = n^2 a0/Z, v_n = Z alpha c/n)
  assumes: Z >= 1; n >= 1; infinitely heavy nucleus (no reduced-mass correction)
```

Python:

```python
import quantum_physics_ai999x as m

a = m.solve("bohr_energy_level", Z="1", n="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-bohr_energy_level) and press **Compute in your browser**.

_What this means:_ This calculation determines the properties of an electron in a hydrogen-like atom using the Bohr model. Given a nuclear charge Z of 1 and a principal quantum number n of 1, the resulting bound state energy is -13.6057 eV. The ionization energy required to remove the electron from this specific state is 13.6057 eV. The electron occupies a circular orbit with a radius of 0.0529177 nm and moves at a speed of 2.18769e+06 m/s. These values illustrate how the electron's binding strength and spatial distribution change with the atomic number and quantum level.

### Hydrogen-like transition with the reduced-mass correction

Photon energy and wavelength of a transition of a one-electron atom whose nucleus has mass M (in u): R_M = R_inf mu/m_e.

Command line:

```console
$ quantum-physics solve hydrogen_transition_reduced_mass Z=1 n_upper=3 n_lower=2 M_u=1.007276467
E_photon_eV = 1.888651009 eV
lam = 6.564696063e-07 m
lambda_nm = 656.4696063 nm
f = 4.566737822e+14 Hz
series = Balmer
mu_ratio = 0.9994556794
  by: Hydrogen-like transition with the reduced-mass correction  (1/lambda = R_inf (mu/m_e) Z^2 (1/n_lower^2 - 1/n_upper^2), mu = m_e M/(m_e + M))
  M_u = 1.007276467
  assumes: n_upper > n_lower >= 1; M > 0
```

Python:

```python
import quantum_physics_ai999x as m

a = m.solve("hydrogen_transition_reduced_mass", Z="1", n_upper="3", n_lower="2", M_u="1.007276467")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/quantum-physics-ai999x/#cap-hydrogen_transition_reduced_mass) and press **Compute in your browser**.

_What this means:_ This calculation determines the spectral properties of a hydrogen-like atom by adjusting the Rydberg constant for the finite mass of the nucleus. Using an atomic number of 1 and levels from 2 to 3, the reduced mass correction yields a factor of 0.999456. The resulting photon energy is 1.88865 eV, corresponding to a vacuum wavelength of 656.47 nm and a frequency of 4.56674e+14 Hz. This refinement distinguishes isotopes, such as comparing hydrogen to deuterium.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/quantum-physics-ai999x/blob/main/docs/quantum_physics_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/quantum-physics-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified quantum physics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 376 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `quantum-physics` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** special relativity, photons and matter waves, the hydrogen atom and angular momentum, quantum wells and tunnelling, spin, qubits and quantum gates, nuclear and particle physics, statistical physics.

## Model overview

| Property | Value |
|---|---|
| Package | `quantum-physics-ai999x` 1.1.0 (import `quantum_physics_ai999x`) |
| Type | Deterministic neuro-symbolic quantum physics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 75 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 376 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `quantum-physics-ai999x "<question>"`; for the native program use `quantum-physics` (above). The outputs above were produced by running this exact code on quantum-physics-ai999x 1.1.0.

## Find the right capability, and draw it

`quantum_physics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "quantum-physics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import quantum_physics_ai999x as quantum_physics

quantum_physics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `quantum-physics-ai999x`; the answer is the one it returned, with the steps it took.

**Lorentz factor** · _Special relativity_

> Lorentz factor: v = 0.8 c

→ **gamma = 1.666666667, beta = 0.8, gamma_minus_1 = 0.6666666667**

<details><summary>Steps</summary>

- Lorentz factor: v = 239833966.4 m/s → gamma = 1.666666667, beta = 0.8, gamma_minus_1 = 0.6666666667

</details>

**Photon from a hydrogen-like transition (Rydberg formula)** · _Hydrogen atom_

> Hydrogen transition: Z = 1, n_upper = 3, n_lower = 2

→ **E_photon_eV = 1.8896796 eV, lam = 6.561122764e-07 m, lambda_nm = 656.1122764 nm, f = 4.569224945e+14 Hz, series = Balmer**

<details><summary>Steps</summary>

- Photon from a hydrogen-like transition (Rydberg formula): Z = 1, n_upper = 3, n_lower = 2 → E_photon_eV = 1.8896796 eV, lam = 6.561122764e-07 m, lambda_nm = 656.1122764 nm, f = 4.569224945e+14 Hz, series = Balmer nm

</details>

**Photoelectric effect** · _Photons and matter waves_

> Photoelectric effect: lambda = 250 nm, W = 4.3 eV

→ **E_photon_eV = 4.959367937 eV, K_max_eV = 0.6593679373 eV, V_stop = 0.6593679373 V, lambda_threshold = 2.883353452e-07 m, lambda_threshold_nm = 288.3353452 nm, f_threshold = 1.039735374e+15 Hz**

<details><summary>Steps</summary>

- Photoelectric effect: lam = 250 nm, W = 4.3 eV → E_photon_eV = 4.959367937 eV, K_max_eV = 0.6593679373 eV, V_stop = 0.6593679373 V, lambda_threshold = 2.883353452e-07 m, lambda_threshold_nm = 288.3353452 nm, f_threshold = 1.039735374e+15 Hz

</details>

**Two-qubit circuit: probabilities and entanglement** · _Quantum gates_

`m.solve("two_qubit_circuit", start="00", gates="H0 CNOT01")`

→ **p00 = 0.5, p01 = 0, p10 = 0, p11 = 0.5, concurrence = 1, p_q0_is_1 = 0.5, p_q1_is_1 = 0.5**

<details><summary>Steps</summary>

- Two-qubit circuit: probabilities and entanglement: start = 00, gates = H0 CNOT01 → p00 = 0.5, p01 = 0, p10 = 0, p11 = 0.5, concurrence = 1, p_q0_is_1 = 0.5, p_q1_is_1 = 0.5

</details>

**Transmission through a rectangular barrier** · _Quantum wells_

> Barrier transmission: E = 1 eV, V0 = 2 eV, a = 0.5 nm, m = 9.109e-31 kg

→ **T = 0.02354963067, R = 0.9764503693, kappa_a = 2.56152966, regime = tunnelling (E < V0)**

<details><summary>Steps</summary>

- Transmission through a rectangular barrier: E = 1 eV, V0 = 2 eV, a = 0.5 nm, m = 9.109e-31 kg → T = 0.02354963067, R = 0.9764503693, kappa_a = 2.56152966, regime = tunnelling (E < V0)

</details>

**Clebsch-Gordan coefficient** · _Angular momentum_

> Clebsch-Gordan coefficient: j1 = 0.5, m1 = 0.5, j2 = 0.5, m2 = -0.5, J = 1, M = 0

→ **cg = 0.7071067812, cg_squared = 0.5, cg_squared_exact = 1/2**

<details><summary>Steps</summary>

- Clebsch-Gordan coefficient: j1 = 0.5, m1 = 0.5, j2 = 0.5, m2 = -0.5, J = 1, M = 0 → cg = 0.7071067812, cg_squared = 0.5, cg_squared_exact = 1/2

</details>

## Figures

Every point was computed by `quantum-physics-ai999x`.

**Hydrogen energy levels and spectral lines**

![Hydrogen energy levels and spectral lines — computed by quantum-physics-ai999x](https://huggingface.co/AI999X/quantum-physics-ai999x/resolve/main/figures/hydrogen-transition-diagram.svg)

_Energy levels of hydrogen, E_n from the package for n = 1 … 6 (E₁ = -13.6057 eV), with the photon wavelengths of Lyman and Balmer transitions it computes: 2→1: 121.502 nm, 3→1: 102.518 nm, 3→2: 656.112 nm, 4→2: 486.009 nm, 5→2: 433.937 nm, 6→2: 410.07 nm._

**The hydrogen line spectrum**

![The hydrogen line spectrum — computed by quantum-physics-ai999x](https://huggingface.co/AI999X/quantum-physics-ai999x/resolve/main/figures/hydrogen-transition-spectrum.svg)

_The hydrogen emission spectrum: the first six lines of the Lyman, Balmer and Paschen series, each wavelength and photon energy computed by the package (Lyman 121.502–93.0252 nm; Balmer 656.112–388.807 nm; Paschen 1874.61–922.658 nm); the dashed lines bound the visible range 380–750 nm, where only Balmer lines fall._

**Particle in a box: levels and probability densities**

![Particle in a box: levels and probability densities — computed by quantum-physics-ai999x](https://huggingface.co/AI999X/quantum-physics-ai999x/resolve/main/figures/infinite-well-probability-levels.svg)

_An electron in a 1 nm infinite square well: the three lowest energy levels the package gives (E1 = 0.37603 eV, E2 = 1.50412 eV, E3 = 3.38427 eV), each with its probability density drawn on the level (the probability of each of 60 intervals of 0.0166667 nm divided by the interval, 0.4 eV per 1/nm); state n has n humps and n − 1 nodes._

**Two-qubit circuits: outcome probabilities and entanglement**

![Two-qubit circuits: outcome probabilities and entanglement — computed by quantum-physics-ai999x](https://huggingface.co/AI999X/quantum-physics-ai999x/resolve/main/figures/two-qubit-circuit-heatmap.svg)

_Two-qubit circuits started in |00⟩ — rows, top to bottom: X0; H0; H0 H1; H0 CNOT01; X1 H0 CNOT01; H0 CNOT01 H0 — with the probability of each measurement outcome and the concurrence (1 = maximally entangled) the package computes; H0 CNOT01 makes the Bell state, P(00) = P(11) = 0.5 with concurrence 1._

**Blackbody spectra and their peaks**

![Blackbody spectra and their peaks — computed by quantum-physics-ai999x](https://huggingface.co/AI999X/quantum-physics-ai999x/resolve/main/figures/planck-spectral-radiance-spectrum.svg)

_Planck spectral radiance at three temperatures, 117 wavelengths each from the package; the dots are the peaks given by its Wien's-law capability: T = 3000 K: λ_max = 965.924 nm, B_λ = 995.249 W/(m² sr nm); T = 4500 K: λ_max = 643.949 nm, B_λ = 7557.67 W/(m² sr nm); T = 5778 K: λ_max = 501.518 nm, B_λ = 26376.3 W/(m² sr nm)._

**Tunnelling through a rectangular barrier**

![Tunnelling through a rectangular barrier — computed by quantum-physics-ai999x](https://huggingface.co/AI999X/quantum-physics-ai999x/resolve/main/figures/barrier-transmission-curve.svg)

_Transmission and reflection of an electron (m = 9.1094e-31 kg) at a rectangular barrier of height V₀ = 3.2044e-19 J (2 eV) and width a = 0.5 nm, for 60 energies computed by the package; tunnelling below E = V₀, transmission resonances above it; marked: E/V₀ = 0.5, T = 0.0235471._

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
@software{quantum_physics_ai999x_1_1_0,
  title  = {quantum-physics-ai999x: an offline, verified quantum physics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
