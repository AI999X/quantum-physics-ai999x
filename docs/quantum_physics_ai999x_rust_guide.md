# quantum-physics: the Quantum Physics program, user guide

**quantum-physics-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Quantum Physics covers special relativity, photons and matter waves, the hydrogen atom, quantum wells and tunnelling, spin and qubits, quantum gates, nuclear and particle physics and statistical physics. This guide is about `quantum-physics`, the model as a self-contained program for Windows and Linux: 76 capabilities in 11 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/quantum-physics-ai999x/); the Python package (`pip install quantum-physics-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Quantum Physics | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-quantum-physics-1.5.0-windows-x86_64.zip |
| Desktop app: Quantum Physics | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-quantum-physics-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Quantum Physics in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-quantum-physics-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-quantum-physics.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-quantum-physics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-quantum-physics-1.5.0-linux-x86_64.tar.gz
./ai999x-quantum-physics-1.5.0-linux-x86_64/ai999x-quantum-physics
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `quantum-physics` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
quantum-physics --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
quantum-physics --version
```

### In your browser

Open https://www.ai999x.com/models/quantum-physics-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install quantum-physics-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ quantum-physics --version
quantum-physics-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ quantum-physics ask "Lorentz factor: v = 0.8 c"
Read as: Lorentz factor
gamma = 1.666666667
beta = 0.8
gamma_minus_1 = 0.6666666667
  by: Lorentz factor  (gamma = 1/sqrt((1 - beta)(1 + beta)), beta = v/c)
  v = 239833966.4 m/s
  assumes: 0 <= v < c
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ quantum-physics search "photon from a hydrogen-like transition (rydberg formula)"
hydrogen_transition                  Photon from a hydrogen-like transition (Rydberg formula)  (hydrogen atom / spectral lines)
hydrogen_transition_reduced_mass     Hydrogen-like transition with the reduced-mass correction  (hydrogen atom / spectral lines and isotope shift)
hydrogen_radial_expectations         Radial expectation values of a hydrogen-like state  (hydrogen atom / wavefunctions)
bohr_energy_level                    Energy level of a hydrogen-like atom (Bohr model)  (hydrogen atom / energy levels)
infinite_well_transition             Photon from a particle-in-a-box transition  (quantum wells / infinite square well)
moseley_k_alpha                      K-alpha X-ray line (Moseley's law)  (hydrogen atom / X-ray spectra)
semi_empirical_mass_formula          Semi-empirical mass formula (liquid-drop binding energy)  (nuclear and particle physics / binding energy)
coherent_state_photon_statistics     Photon-number distribution of a coherent state  (photons and matter waves / coherent states of light)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ quantum-physics list
barrier_transmission                     Transmission through a rectangular barrier
bec_critical_temperature                 Bose-Einstein condensation temperature
blackbody_band_fraction                  Fraction of blackbody radiation in a wavelength band
bohr_energy_level                        Energy level of a hydrogen-like atom (Bohr model)
bose_einstein_occupation                 Bose-Einstein occupation of a mode
chsh_bell_test                           CHSH Bell-test value for the singlet state
clebsch_gordan                           Clebsch-Gordan coefficient
coherent_state_photon_statistics         Photon-number distribution of a coherent state
… 68 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ quantum-physics solve hydrogen_transition Z=1 n_upper=3 n_lower=2
E_photon_eV = 1.8896796 eV
lam = 6.561122764e-07 m
lambda_nm = 656.1122764 nm
f = 4.569224945e+14 Hz
series = Balmer
  by: Photon from a hydrogen-like transition (Rydberg formula)  (1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2))
  assumes: n_upper > n_lower >= 1; infinitely heavy nucleus
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ quantum-physics solve hydrogen_transition Z=1 n_upper=3 n_lower=2 --json
{"outcome":"SOLVED","message":"","value":656.112276419,"unit":"nm","solved_for":"lambda_nm","values":[["E_photon_eV",1.88967960042,"eV"],["lam",6.56112276419e-7,"m"],["lambda_nm",656.112276419,"nm"],["f",456922494479000.0,"Hz"],["series","Balmer",""]],"target":"lambda_nm","phrase":"photon energy","assumptions":["n_upper > n_lower >= 1","infinitely heavy nucleus"],"name":"Photon from a hydrogen-like transition (Rydberg formula)","equation":"1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ quantum-physics solve hydrogen_transition Z=1 n_upper=3
NEEDS_INPUT: hydrogen_transition needs n_lower (lower level). Example: solve('hydrogen_transition', Z=1, n_upper=3, n_lower=2).
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | quantum-physics batch

`cases.jsonl`:

```json
{"key": "hydrogen_transition", "inputs": {"Z": "1", "n_upper": "3", "n_lower": "2"}}
{"key": "lorentz_factor", "inputs": {"v": "239833966.4 m/s"}}
```

```shell
$ quantum-physics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":656.112276419,"unit":"nm","solved_for":"lambda_nm","values":[["E_photon_eV",1.88967960042,"eV"],["lam",6.56112276419e-7,"m"],["lambda_nm",656.112276419,"nm"],["f",456922494479000.0,"Hz"],["series","Balmer",""]],"target":"lambda_nm","phrase":"photon energy","assumptions":["n_upper > n_lower >= 1","infinitely heavy nucleus"],"name":"Photon from a hydrogen-like transition (Rydberg formula)","equation":"1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2)"}
{"outcome":"SOLVED","message":"","value":1.66666666667,"unit":"","solved_for":"gamma","si":[["v",239833966.4]],"values":[["gamma",1.66666666667,""],["beta",0.8,""],["gamma_minus_1",0.666666666667,""]],"target":"gamma","phrase":"Lorentz factor","assumptions":["0 <= v < c"],"name":"Lorentz factor","equation":"gamma = 1/sqrt((1 - beta)(1 + beta)), beta = v/c"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `quantum-physics info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`quantum-physics solve ... --json` prints one answer as one line of JSON. `quantum-physics batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | quantum-physics batch`. `quantum-physics catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["quantum-physics", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`quantum-physics figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `quantum_physics_ai999x.wasm` (https://www.ai999x.com/compute/rust/quantum_physics_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("quantum_physics_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `quantum-physics info <id>` shows the same inputs and outputs in the terminal.

### Angular momentum (2)

<a id="clebsch_gordan"></a>

#### Clebsch-Gordan coefficient: `clebsch_gordan`

The coefficient <j1 m1; j2 m2 | J M> for coupling two angular momenta (Condon-Shortley convention), with its exact square.

$$
\langle j_1 m_1; j_2 m_2 | J M\rangle
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `j1` | in | first angular momentum (whole or half-whole) | - |
| `m1` | in | its projection | - |
| `j2` | in | second angular momentum | - |
| `m2` | in | its projection | - |
| `J` | in | total angular momentum | - |
| `M` | in | total projection | - |
| `cg` | out | Clebsch-Gordan coefficient | - |
| `cg_squared` | out | its square (a probability) | - |
| `cg_squared_exact` | out | the square as an exact fraction | - |

**Applies when:** each j >= 0 and whole or half-whole; |m| <= j with j - m whole.

```shell
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

**What the worked example means:** This calculation determines the Clebsch-Gordan coefficient for coupling two angular momenta with values 0.5 and 0.5 to a total angular momentum of 1, using projections 0.5 and -0.5 that sum to 0. The resulting coefficient is 0.707107, which represents the amplitude of this specific coupled state in the product basis. Its exact square, 1/2, gives the probability of finding the system in this configuration under the Condon-Shortley convention.

<a id="wigner_3j_symbol"></a>

#### Wigner 3j symbol: `wigner_3j_symbol`

The Wigner 3j symbol (j1 j2 J; m1 m2 M), nonzero only when m1 + m2 + M = 0.

$$
\begin{pmatrix} j_1 & j_2 & J \\ m_1 & m_2 & M \end{pmatrix}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `j1` | in | first angular momentum (whole or half-whole) | - |
| `m1` | in | its projection | - |
| `j2` | in | second angular momentum | - |
| `m2` | in | its projection | - |
| `J` | in | total angular momentum | - |
| `M` | in | total projection | - |
| `w3j` | out | the 3j symbol | - |
| `w3j_squared` | out | its square | - |

**Applies when:** each j >= 0 and whole or half-whole; |m| <= j with j - m whole.

```shell
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

**What the worked example means:** This calculation determines the Wigner three j symbol, a key component in quantum mechanics for describing how angular momenta combine. Using the input values of first angular momentum 1, second angular momentum 1, and total angular momentum 0, along with their respective projections 1, -1, and 0, the algorithm derives the symbol from exact Clebsch-Gordan coefficients. The resulting value 0.57735 represents the probability amplitude for this specific coupling configuration, while 0.333333 indicates the relative strength or likelihood of this interaction occurring.

### Hydrogen atom (7)

<a id="bohr_energy_level"></a>

#### Energy level of a hydrogen-like atom (Bohr model), `bohr_energy_level`

Energy, ionization energy, orbit radius and orbital speed of level n of a one-electron atom or ion with nuclear charge Z.

$$
E_n = -\frac{Z^2 R_\infty h c}{n^2} \approx -\frac{13.6\ \mathrm{eV}\,Z^2}{n^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z` | in | atomic number (nuclear charge; 1 for hydrogen, 2 for He+ ...) | - |
| `n` | in | principal quantum number | - |
| `E_eV` | out | energy of the level (negative: bound) | eV |
| `E` | out | the same in J | J |
| `ionization_eV` | out | energy needed to ionize from this level | eV |
| `r` | out | Bohr orbit radius | m |
| `r_nm` | out | orbit radius in nm | nm |
| `v` | out | orbital speed | m/s |

**Applies when:** Z >= 1; n >= 1; infinitely heavy nucleus (no reduced-mass correction).

```shell
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

**What the worked example means:** This calculation determines the properties of an electron in a hydrogen-like atom using the Bohr model. Given a nuclear charge Z of 1 and a principal quantum number n of 1, the resulting bound state energy is -13.6057 eV. The ionization energy required to remove the electron from this specific state is 13.6057 eV. The electron occupies a circular orbit with a radius of 0.0529177 nm and moves at a speed of 2.18769e+06 m/s. These values illustrate how the electron's binding strength and spatial distribution change with the atomic number and quantum level.

<a id="hydrogen_transition_reduced_mass"></a>

#### Hydrogen-like transition with the reduced-mass correction, `hydrogen_transition_reduced_mass`

Photon energy and wavelength of a transition of a one-electron atom whose nucleus has mass M (in u): R_M = R_inf mu/m_e.

$$
\frac{1}{\lambda} = \frac{\mu}{m_e} R_\infty Z^2\left(\frac{1}{n_\ell^2} - \frac{1}{n_u^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z` | in | atomic number (1 for hydrogen) | - |
| `n_upper` | in | upper level | - |
| `n_lower` | in | lower level | - |
| `M_u` | in | mass of the nucleus in u (1.007276 for the proton, 2.013553 for the deuteron) | - |
| `E_photon_eV` | out | photon energy | eV |
| `lam` | out | vacuum wavelength | m |
| `lambda_nm` | out | vacuum wavelength in nm | nm |
| `f` | out | frequency | Hz |
| `series` | out | spectral series (Lyman, Balmer, ...) | - |
| `mu_ratio` | out | mu/m_e | - |

**Applies when:** n_upper > n_lower >= 1; M > 0.

```shell
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

**What the worked example means:** This calculation determines the spectral properties of a hydrogen-like atom by adjusting the Rydberg constant for the finite mass of the nucleus. Using an atomic number of 1 and levels from 2 to 3, the reduced mass correction yields a factor of 0.999456. The resulting photon energy is 1.88865 eV, corresponding to a vacuum wavelength of 656.47 nm and a frequency of 4.56674e+14 Hz. This refinement distinguishes isotopes, such as comparing hydrogen to deuterium.

<a id="moseley_k_alpha"></a>

#### K-alpha X-ray line (Moseley's law): `moseley_k_alpha`

Energy and wavelength of the K-alpha X-ray line of element Z, with one unit of screening (Z - 1).

$$
E_{K\alpha} = \tfrac{3}{4} R_\infty h c\,(Z - 1)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z` | in | atomic number of the target element | - |
| `E_keV` | out | photon energy | keV |
| `lam` | out | wavelength | m |
| `lambda_nm` | out | wavelength in nm | nm |
| `lambda_pm` | out | wavelength in pm | pm |
| `f` | out | frequency | Hz |

**Applies when:** Z >= 2.

```shell
$ quantum-physics solve moseley_k_alpha Z=29
E_keV = 8.000147556 keV
lam = 1.549773896e-10 m
lambda_nm = 0.1549773896 nm
lambda_pm = 154.9773896 pm
f = 1.934427073e+18 Hz
  by: K-alpha X-ray line (Moseley's law)  (E = (3/4) R_inf h c (Z - 1)^2)
  assumes: Z >= 2
```

**What the worked example means:** This calculation determines the photon energy and wavelength of the K-alpha X-ray line emitted by an element with atomic number 29. Using Moseley's law, which accounts for one unit of electron screening, the model computes that the transition yields an energy of 8.00015 keV. This result translates to a frequency of 1.93443e+18 Hz and a wavelength of 0.154977 nm, providing key spectral data used for identifying elements in X-ray spectroscopy.

<a id="lande_g_zeeman"></a>

#### Landé g factor and anomalous Zeeman shift, `lande_g_zeeman`

The Landé g factor of a level L, S, J (with g_s = 2) and the energy shift g_J mu_B B mJ in a weak magnetic field.

$$
g_J = 1 + \frac{J(J+1) + S(S+1) - L(L+1)}{2J(J+1)},\quad \Delta E = g_J \mu_B B m_J
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | orbital angular momentum quantum number | - |
| `S` | in | spin quantum number | - |
| `J` | in | total angular momentum quantum number | - |
| `mJ` | in | magnetic quantum number | - |
| `B` | in | magnetic field (T) | T |
| `g_J` | out | Landé g factor | - |
| `dE` | out | Zeeman energy shift | J |
| `dE_eV` | out | the shift in eV | eV |
| `g_J_exact` | out | g_J as an exact fraction | - |

**Applies when:** |L - S| <= J <= L + S, J > 0; |mJ| <= J; weak field (LS coupling holds).

```shell
$ quantum-physics solve lande_g_zeeman L=1 S=0.5 J=1.5 mJ=0.5 "B=1 T"
g_J = 1.333333333
dE = 6.182673377e-24 J
dE_eV = 3.858921199e-05 eV
g_J_exact = 4/3
  by: Landé g factor and anomalous Zeeman shift  (g_J = 1 + [J(J+1) + S(S+1) - L(L+1)] / [2 J(J+1)], dE = g_J mu_B B mJ)
  L = 1
  S = 0.5
  J = 1.5
  mJ = 0.5
  B = 1 T
  assumes: |L - S| <= J <= L + S, J > 0; |mJ| <= J; weak field (LS coupling holds)
```

**What the worked example means:** This calculation determines the Landé g factor, which quantifies how an atomic level splits in a magnetic field. Using the orbital angular momentum quantum number L=1, spin quantum number S=0.5, and total angular momentum quantum number J=1.5, the g factor is computed as 1.33333. For a magnetic field strength B=1 T, this results in an energy shift dE of 6.18267e-24 J for the magnetic sublevel mJ=0.5. The exact fractional value of the g factor is 4/3, and the shift expressed in electron volts is 3.85892e-05 eV. This shift represents the anomalous Zeeman effect, where spectral lines split due to the interaction between the atom's magnetic moment and the external field.

<a id="hydrogen_transition"></a>

#### Photon from a hydrogen-like transition (Rydberg formula), `hydrogen_transition`

Energy, wavelength and frequency of the photon emitted when a one-electron atom drops from n_upper to n_lower.

$$
\frac{1}{\lambda} = R_\infty Z^2\left(\frac{1}{n_\ell^2} - \frac{1}{n_u^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z` | in | atomic number (1 for hydrogen) | - |
| `n_upper` | in | upper level | - |
| `n_lower` | in | lower level | - |
| `E_photon_eV` | out | photon energy | eV |
| `lam` | out | vacuum wavelength | m |
| `lambda_nm` | out | vacuum wavelength in nm | nm |
| `f` | out | frequency | Hz |
| `series` | out | spectral series (Lyman, Balmer, ...) | - |

**Applies when:** n_upper > n_lower >= 1; infinitely heavy nucleus.

```shell
$ quantum-physics solve hydrogen_transition Z=1 n_upper=3 n_lower=2
E_photon_eV = 1.8896796 eV
lam = 6.561122764e-07 m
lambda_nm = 656.1122764 nm
f = 4.569224945e+14 Hz
series = Balmer
  by: Photon from a hydrogen-like transition (Rydberg formula)  (1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2))
  assumes: n_upper > n_lower >= 1; infinitely heavy nucleus
```

**What the worked example means:** This calculation determines the properties of a photon released when an electron in a hydrogen-like atom transitions from a higher energy level to a lower one. Using the atomic number, the initial upper level, and the final lower level as inputs, the Rydberg formula computes the vacuum wavelength, frequency, and energy of the light. The result indicates that the specific transition produces radiation with an energy of 1.88968 eV and a wavelength of 656.112 nm, which corresponds to the Balmer spectral series.

<a id="hydrogen_radial_probability"></a>

#### Probability of finding the electron within a radius, `hydrogen_radial_probability`

The probability that the electron of the hydrogen-like state n, l lies within distance R of the nucleus.

$$
P(r < R) = \int_0^R |R_{nl}(r)|^2 r^2\,dr
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | principal quantum number | - |
| `l` | in | orbital quantum number (0 <= l < n) | - |
| `Z` | in | atomic number | - |
| `R` | in | radius (pm, nm, m) | m |
| `P_inside` | out | probability within R | - |
| `P_outside` | out | probability beyond R | - |
| `R_a0` | out | R in units of a0 | - |

**Applies when:** n >= 1; 0 <= l <= n - 1; R > 0; infinitely heavy nucleus.

```shell
$ quantum-physics solve hydrogen_radial_probability n=1 l=0 Z=1 "R=52.91772105 pm"
P_inside = 0.3233235838
P_outside = 0.6766764162
R_a0 = 0.9999999999
  by: Probability of finding the electron within a radius  (P = integral_0^R r^2 R_nl^2 dr / integral_0^inf r^2 R_nl^2 dr (30-digit quadrature))
  R = 5.291772105e-11 m
  assumes: n >= 1; 0 <= l <= n - 1; R > 0; infinitely heavy nucleus
```

**What the worked example means:** This calculation determines the likelihood that an electron in a hydrogen-like state defined by quantum numbers 1 and 0 resides within a distance 52.9177 pm from the nucleus. The result, expressed as 0.323324, represents the cumulative radial probability up to that radius, while 0.676676 accounts for the remaining probability beyond it. The radius is also normalized to atomic units as 1 to facilitate comparison with standard orbital scales.

<a id="hydrogen_radial_expectations"></a>

#### Radial expectation values of a hydrogen-like state, `hydrogen_radial_expectations`

<r>, <r^2>, <1/r> and the radial spread of the state n, l of a one-electron atom with nuclear charge Z (in Bohr radii).

$$
\langle r\rangle = \frac{a_0}{2Z}\left[3n^2 - l(l+1)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | principal quantum number | - |
| `l` | in | orbital quantum number (0 <= l < n) | - |
| `Z` | in | atomic number | - |
| `r_mean_a0` | out | <r> in units of a0 | - |
| `r2_mean_a0sq` | out | <r^2> in units of a0^2 | - |
| `inv_r_mean_per_a0` | out | <1/r> in units of 1/a0 | - |
| `r_mean` | out | <r> in m | m |
| `dr_a0` | out | radial spread sqrt(<r^2> - <r>^2) in a0 | - |

**Applies when:** n >= 1; 0 <= l <= n - 1; infinitely heavy nucleus.

```shell
$ quantum-physics solve hydrogen_radial_expectations n=2 l=1 Z=1
r_mean_a0 = 5
r2_mean_a0sq = 30
inv_r_mean_per_a0 = 0.25
r_mean = 2.645886053e-10 m
dr_a0 = 2.236067977
  by: Radial expectation values of a hydrogen-like state  (<r> = a0 (3n^2 - l(l+1))/(2Z), <r^2> = a0^2 n^2 (5n^2 + 1 - 3l(l+1))/(2Z^2), <1/r> = Z/(n^2 a0))
  assumes: n >= 1; 0 <= l <= n - 1; infinitely heavy nucleus
```

**What the worked example means:** This calculation determines the spatial extent of an electron in a hydrogen-like atom using the principal quantum number 2, orbital quantum number 1, and atomic number 1. It computes the expectation value of the radial distance, expressed as 5 in units of the Bohr radius, and the root-mean-square deviation from this mean, given by 2.23607. These values characterize the average size and fuzzy boundary of the electron cloud for the specified quantum state.

### Nuclear and particle physics (2)

<a id="two_body_decay"></a>

#### Kinematics of a two-body decay: `two_body_decay`

Q value, momentum and kinetic energies of the two products when a particle or nucleus at rest decays into two bodies (alpha decay, pi -> mu nu, ...).

$$
p = \frac{\sqrt{\left[M^2 - (m_1 + m_2)^2\right]\left[M^2 - (m_1 - m_2)^2\right]}}{2M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the parent | - |
| `m1` | in | mass of product 1 | - |
| `m2` | in | mass of product 2 (0 for a photon or neutrino) | - |
| `unit` | in | unit of the masses: MeV/c^2 (default) or u (optional) | - |
| `Q_MeV` | out | Q value (M - m1 - m2) c^2 | MeV |
| `p_MeV` | out | momentum of each product times c | MeV |
| `E1_MeV` | out | total energy of product 1 | MeV |
| `E2_MeV` | out | total energy of product 2 | MeV |
| `K1_MeV` | out | kinetic energy of product 1 | MeV |
| `K2_MeV` | out | kinetic energy of product 2 | MeV |

**Applies when:** M > m1 + m2; m1, m2 >= 0.

```shell
$ quantum-physics solve two_body_decay M=139.57039 m1=105.6583755 m2=0
Q_MeV = 33.9120145 MeV
p_MeV = 29.79214091 MeV
E1_MeV = 109.7782491 MeV
E2_MeV = 29.79214091 MeV
K1_MeV = 4.11987359 MeV
K2_MeV = 29.79214091 MeV
  by: Kinematics of a two-body decay  (p = sqrt[(M^2 - (m1 + m2)^2)(M^2 - (m1 - m2)^2)]/(2M), E_i = sqrt(p^2 + m_i^2), K_i = p^2/(E_i + m_i) (relativistic))
  M = 139.57039
  m1 = 105.6583755
  m2 = 0
  assumes: M > m1 + m2; m1, m2 >= 0
```

**What the worked example means:** This calculation determines the kinematics for a parent particle at rest decaying into two products. Using the mass of the parent and the masses of the two products, it computes the available energy known as the Q value, which is found to be 33.912 MeV. The momentum shared by both products in opposite directions is calculated as 29.7921 MeV. From this momentum, the total energy and kinetic energy of each individual product are derived, providing a complete picture of how the initial mass is converted into motion and rest mass.

<a id="semi_empirical_mass_formula"></a>

#### Semi-empirical mass formula (liquid-drop binding energy), `semi_empirical_mass_formula`

The liquid-drop (Bethe-Weizsaecker) binding energy of a nucleus with Z protons and mass number A, its terms, the binding energy per nucleon, the atomic mass and the most stable Z for that A.

$$
B = a_V A - a_S A^{2/3} - a_C\frac{Z(Z-1)}{A^{1/3}} - a_A\frac{(A-2Z)^2}{A} + \delta(A,Z)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z` | in | number of protons | - |
| `A` | in | mass number | - |
| `B_MeV` | out | binding energy | MeV |
| `B_per_A_MeV` | out | binding energy per nucleon | MeV |
| `mass_u` | out | atomic mass in u (Z m(1H) + N m_n - B/c^2) | - |
| `volume_MeV` | out | volume term a_V A | MeV |
| `surface_MeV` | out | surface term a_S A^(2/3) | MeV |
| `coulomb_MeV` | out | Coulomb term a_C Z(Z - 1)/A^(1/3) | MeV |
| `asymmetry_MeV` | out | asymmetry term a_A (A - 2Z)^2/A | MeV |
| `pairing_MeV` | out | pairing term (+ even-even, 0 odd A, - odd-odd) | MeV |
| `Z_stable` | out | most stable (continuous) Z for this A, where dB/dZ = 0 | - |

**Applies when:** 1 <= Z <= A; A >= 2; a liquid-drop estimate: poor for light nuclei.

```shell
$ quantum-physics solve semi_empirical_mass_formula Z=26 A=56
B_MeV = 495.3836999 MeV
B_per_A_MeV = 8.846137499 MeV
mass_u = 55.93158207
volume_MeV = 882 MeV
surface_MeV = 260.5425666 MeV
coulomb_MeV = 120.7962953 MeV
asymmetry_MeV = 6.771428571 MeV
pairing_MeV = 1.493990342 MeV
Z_stable = 25.27970457
  by: Semi-empirical mass formula (liquid-drop binding energy)  (B = a_V A - a_S A^(2/3) - a_C Z(Z - 1)/A^(1/3) - a_A (A - 2Z)^2/A + delta; a_V = 15.75, a_S = 17.8, a_C = 0.711, a_A = 23.7, a_P = 11.18 MeV, delta = +-a_P/sqrt(A))
  assumes: 1 <= Z <= A; A >= 2; a liquid-drop estimate: poor for light nuclei
```

**What the worked example means:** This calculation applies the liquid-drop model to estimate nuclear stability. It sums the volume, surface, Coulomb, asymmetry, and pairing terms using proton number 26 and mass number 56. The resulting binding energy is 495.384 MeV, while the most stable proton count for this mass number is 25.2797.

### Nuclear physics (7)

<a id="radiation_dose"></a>

#### Absorbed dose and equivalent dose: `radiation_dose`

The absorbed dose D = E/m (Gy = J/kg) of tissue that absorbs radiation energy E, and the equivalent dose H = w_R D (Sv) with the radiation weighting factor (RBE, quality factor) of the radiation.

$$
D = \frac{E}{m},\quad H = w_R\,D
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | radiation energy absorbed by the tissue (J, mJ, MeV ...) | J |
| `m` | in | mass of the absorbing tissue | kg |
| `w_R` | in | radiation weighting factor (RBE or quality factor): 1 for photons and electrons, about 2 for protons, 20 for alpha particles | - |
| `D` | out | absorbed dose (Gy = J/kg) | J/kg |
| `H` | out | equivalent dose (Sv = J/kg) | J/kg |
| `D_mGy` | out | absorbed dose in mGy | - |
| `H_mSv` | out | equivalent dose in mSv | - |
| `H_rem` | out | equivalent dose in rem (1 rem = 0.01 Sv) | - |

**Applies when:** E >= 0; m > 0; 1 <= w_R <= 20; the energy is absorbed uniformly in the mass m.

```shell
$ quantum-physics solve radiation_dose "E=2 mJ" "m=500 g" w_R=20
D = 0.004 J/kg
H = 0.08 J/kg
D_mGy = 4
H_mSv = 80
H_rem = 8
  by: Absorbed dose and equivalent dose  (D = E/m, H = w_R D)
  E = 0.002 J
  m = 0.5 kg
  w_R = 20
  assumes: E >= 0; m > 0; 1 <= w_R <= 20; the energy is absorbed uniformly in the mass m
```

In the worked example, E = 2 mJ, m = 500 g, w_R = 20 go through the equation above and give D = 0.004 J/kg, H = 0.08 J/kg, D_mGy = 4, H_mSv = 80. It applies only when E >= 0; m > 0; 1 <= w_R <= 20; the energy is absorbed uniformly in the mass m: outside that it says what is wrong instead of returning a number.

<a id="radioactive_dating_age"></a>

#### Age from the remaining fraction (radioactive dating), `radioactive_dating_age`

The time needed for a radioactive sample to decay to a given fraction of its original amount (for example carbon-14 dating).

$$
t = \frac{T_{1/2}}{\ln 2}\ln\frac{N_0}{N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fraction` | in | fraction of the original nuclei (or activity) remaining, 0 < f <= 1 (or a percentage) | - |
| `T_half` | in | half-life | s |
| `t` | out | elapsed time (age) | s |
| `half_lives` | out | number of half-lives | - |
| `decay_constant` | out | decay constant | 1/s |

**Applies when:** 0 < fraction <= 1; T_half > 0.

```shell
$ quantum-physics solve radioactive_dating_age fraction=0.25 "T_half=5730 yr"
t = 3.61650096e+11 s
half_lives = 2
decay_constant = 3.833247596e-12 1/s
  by: Age from the remaining fraction (radioactive dating)  (t = T_half log2(1/f) = ln(1/f)/lambda)
  fraction = 0.25
  T_half = 1.80825048e+11 s
  assumes: 0 < fraction <= 1; T_half > 0
```

**What the worked example means:** This calculation determines the elapsed time for a radioactive sample to decay to a specific fraction of its original amount. Using the provided half-life of 5730 yr and the remaining fraction of 0.25, the model computes an age of 3.6165e+11 s. This result indicates that 2 half-lives have occurred, with a derived decay constant of 3.83325e-12 1/s. The method relies on the exponential nature of radioactive decay where the number of nuclei decreases by half over each half-life period.

<a id="coulomb_barrier"></a>

#### Coulomb barrier between two nuclei: `coulomb_barrier`

The Coulomb potential energy of two nuclei just touching, with radii R = r0 A^(1/3).

$$
V_C = \frac{Z_1 Z_2 e^2}{4\pi\varepsilon_0\,r_0\left(A_1^{1/3} + A_2^{1/3}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z1` | in | charge number of nucleus 1 | - |
| `A1` | in | mass number of nucleus 1 | - |
| `Z2` | in | charge number of nucleus 2 | - |
| `A2` | in | mass number of nucleus 2 | - |
| `r0` | in | radius parameter (typically 1.2 fm) | m |
| `V_MeV` | out | Coulomb barrier height | MeV |
| `R` | out | touching distance r0 (A1^(1/3) + A2^(1/3)) | m |
| `R_fm` | out | the same in fm | fm |

**Applies when:** 1 <= Z <= A; r0 > 0.

```shell
$ quantum-physics solve coulomb_barrier Z1=1 A1=1 Z2=1 A2=1 "r0=1 fm"
V_MeV = 0.7199822734 MeV
R = 2e-15 m
R_fm = 2 fm
  by: Coulomb barrier between two nuclei  (V = Z1 Z2 e^2/(4 pi eps0 R) = Z1 Z2 alpha hbar c/R)
  r0 = 1e-15 m
  assumes: 1 <= Z <= A; r0 > 0
```

**What the worked example means:** This calculation determines the electrostatic potential energy required for two nuclei to touch, modeled as point charges separated by their combined radii. Using the charge numbers and mass numbers provided, the touching distance is computed as 2e-15 m, which equals 2 fm. The resulting barrier height is 0.719982 MeV, representing the energy threshold that must be overcome or tunneled through for nuclear fusion to occur.

<a id="nuclide_decay"></a>

#### Decay of a named radionuclide: `nuclide_decay`

The fraction of a named radionuclide (C-14, Co-60, I-131, U-238, ...) left after time t, with its tabulated half-life, decay constant and mean life.

$$
\frac{N}{N_0} = 2^{-t/T_{1/2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nuclide` | in | the radionuclide (e.g. C-14, Cs-137, I-131) | - |
| `t` | in | elapsed time | s |
| `fraction` | out | fraction of the parent nuclide remaining | - |
| `half_life` | out | half-life (ICRP-107) | s |
| `half_life_years` | out | half-life in Julian years | - |
| `decay_constant` | out | decay constant | 1/s |
| `mean_life` | out | mean life | s |
| `specific_activity_per_atom` | out | activity per original atom at time t (Bq) | 1/s |

**Applies when:** t >= 0; one of the tabulated nuclides.

```shell
$ quantum-physics solve nuclide_decay nuclide=I-131 "t=1385976.96 s"
fraction = 0.25
half_life = 692988.48 s
half_life_years = 0.02195947981
decay_constant = 1.000229009e-06 1/s
mean_life = 999771.0435 s
specific_activity_per_atom = 2.500572522e-07 1/s
  by: Decay of a named radionuclide  (N/N0 = 2^(-t/T_half) with the ICRP-107 half-life of the nuclide)
  t = 1385976.96 s
  nuclide = I-131
  assumes: t >= 0; one of the tabulated nuclides
```

**What the worked example means:** This calculation determines how much of the radioactive isotope I-131 remains after an elapsed time of 1.38598e+06 s. Using the standard ICRP-107 half-life of 692988 s, which corresponds to 0.0219595 Julian years, the model applies first-order decay kinetics. The resulting fraction of parent atoms left is 0.25. Additionally, the decay constant is 1.00023e-06 1/s per second, and the mean life is 999771 s seconds. This specific activity per original atom at time t is 2.50057e-07 1/s becquerels.

<a id="decay_chain_parent_daughter"></a>

#### Parent-daughter decay chain (Bateman): `decay_chain_parent_daughter`

Numbers and activities of a parent and its radioactive daughter after time t, starting from pure parent (Bateman solution).

$$
N_D(t) = N_0\frac{\lambda_P}{\lambda_D - \lambda_P}\left(e^{-\lambda_P t} - e^{-\lambda_D t}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_half_P` | in | half-life of the parent | s |
| `T_half_D` | in | half-life of the daughter | s |
| `N0` | in | initial number of parent nuclei | - |
| `t` | in | elapsed time | s |
| `N_parent` | out | parent nuclei | - |
| `N_daughter` | out | daughter nuclei | - |
| `A_parent` | out | parent activity (Bq) | 1/s |
| `A_daughter` | out | daughter activity (Bq) | 1/s |
| `activity_ratio` | out | A_daughter/A_parent (tends to 1 in secular equilibrium) | - |

**Applies when:** half-lives and N0 positive; t >= 0; no daughter at t = 0.

```shell
$ quantum-physics solve decay_chain_parent_daughter "T_half_P=2 s" "T_half_D=1 s" N0=1 "t=2 s"
N_parent = 0.5
N_daughter = 0.25
A_parent = 0.1732867951 1/s
A_daughter = 0.1732867951 1/s
activity_ratio = 1
  by: Parent-daughter decay chain (Bateman)  (N_D = N0 lambda_P (e^(-lambda_P t) - e^(-lambda_D t))/(lambda_D - lambda_P), evaluated without cancellation when the decay constants are close)
  T_half_P = 2 s
  T_half_D = 1 s
  N0 = 1
  t = 2 s
  assumes: half-lives and N0 positive; t >= 0; no daughter at t = 0
```

**What the worked example means:** This calculation determines the number of parent and daughter nuclei remaining after an elapsed time of 2 s seconds. Starting with 1 pure parent atoms, the model applies the Bateman equations to track how the parent decays into the radioactive daughter. The resulting parent population is 0.5, while the daughter population reaches 0.25. These values allow you to compute the activities, which are 0.173287 1/s for the parent and 0.173287 1/s for the daughter.

<a id="nuclear_reaction_threshold"></a>

#### Q value and threshold energy of a nuclear reaction, `nuclear_reaction_threshold`

Q value of a + X -> Y + b from the masses (in u) and, for an endothermic reaction, the minimum beam kinetic energy on a target at rest (relativistic).

$$
Q = (m_a + m_X - m_Y - m_b)c^2,\quad K_{th} = -Q\,\frac{m_a + m_X + m_Y + m_b}{2 m_X}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m_a` | in | projectile mass (u) | - |
| `m_X` | in | target mass (u) | - |
| `m_Y` | in | heavy product mass (u) | - |
| `m_b` | in | light product mass (u) | - |
| `Q_MeV` | out | Q value | MeV |
| `K_threshold_MeV` | out | threshold kinetic energy of the projectile (0 if exothermic) | MeV |
| `endothermic` | out | whether the reaction needs energy (Q < 0) | - |

**Applies when:** masses >= 0; target at rest.

```shell
$ quantum-physics solve nuclear_reaction_threshold m_a=4.002603254 m_X=14.003074 m_Y=16.99913176 m_b=1.007825032
Q_MeV = -1.191882102 MeV
K_threshold_MeV = 1.532621122 MeV
endothermic = yes
  by: Q value and threshold energy of a nuclear reaction  (Q = (m_a + m_X - m_Y - m_b) c^2; K_th = -Q (m_a + m_X + m_Y + m_b)/(2 m_X) (exact relativistic form))
  m_a = 4.002603254
  m_X = 14.003074
  m_Y = 16.99913176
  m_b = 1.007825032
  assumes: masses >= 0; target at rest
```

**What the worked example means:** This calculation determines the Q value and the minimum projectile kinetic energy required for a nuclear reaction where a projectile strikes a stationary target. The Q value, calculated as -1.19188 MeV, represents the net energy change based on the mass difference between reactants and products. If the reaction is endothermic, indicated by yes, the projectile must possess at least 1.53261 MeV to initiate it, accounting for momentum conservation in the center-of-mass frame.

<a id="radioactive_decay_law"></a>

#### Radioactive decay law: `radioactive_decay_law`

Nuclei remaining, fraction, decay constant, mean life and activity after time t, from the half-life and the initial number of nuclei.

$$
N(t) = N_0\,2^{-t/T_{1/2}} = N_0 e^{-\lambda t},\quad A = \lambda N
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_half` | in | half-life | s |
| `N0` | in | initial number of nuclei | - |
| `t` | in | elapsed time | s |
| `N` | out | nuclei remaining | - |
| `fraction` | out | fraction remaining N/N0 | - |
| `decay_constant` | out | decay constant ln2/T_half | 1/s |
| `mean_life` | out | mean life 1/lambda | s |
| `A0` | out | initial activity (Bq) | 1/s |
| `A` | out | activity at time t (Bq) | 1/s |
| `half_lives` | out | number of half-lives elapsed | - |

**Applies when:** T_half > 0; N0 > 0; t >= 0.

```shell
$ quantum-physics solve radioactive_decay_law "T_half=5730 yr" N0=1e+20 "t=11460 yr"
N = 2.5e+19
fraction = 0.25
decay_constant = 3.833247596e-12 1/s
mean_life = 2.608754e+11 s
A0 = 383324759.6 1/s
A = 95831189.9 1/s
half_lives = 2
  by: Radioactive decay law  (N = N0 2^(-t/T_half), lambda = ln 2/T_half, A = lambda N)
  T_half = 1.80825048e+11 s
  N0 = 1e+20
  t = 3.61650096e+11 s
  assumes: T_half > 0; N0 > 0; t >= 0
```

**What the worked example means:** This calculation determines the state of a radioactive sample after a specific elapsed time. Using the half-life of 5730 yr and starting with 1e+20 nuclei, it finds that 2.5e+19 nuclei are left. The decay constant is calculated as 3.83325e-12 1/s per second, which leads to a mean life of 2.60875e+11 s seconds. Consequently, the current activity is 9.58312e+07 1/s decays per second, compared to the initial activity of 3.83325e+08 1/s. This shows how quickly the material loses its radioactivity over the period of 11460 yr.

### Particle physics (1)

<a id="particle_decay_length"></a>

#### Mean decay length of a relativistic unstable particle, `particle_decay_length`

Lorentz factor, lab-frame lifetime and mean decay length beta gamma c tau of a pion, kaon, muon, hyperon, charm or bottom meson with momentum p.

$$
L = \beta\gamma c\tau = \frac{p}{mc}\,c\tau
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `particle` | in | the particle (pi+, K+, K_S0, K_L0, mu-, tau-, n, Lambda, ...) | - |
| `p` | in | momentum in GeV/c | - |
| `gamma` | out | Lorentz factor | - |
| `beta` | out | v/c | - |
| `beta_gamma` | out | beta gamma = p/(m c) | - |
| `lifetime_lab` | out | mean lifetime in the lab | s |
| `mean_decay_length` | out | mean decay length beta gamma c tau | m |
| `proper_lifetime` | out | mean proper lifetime (PDG) | s |
| `mass_MeV` | out | mass (MeV/c^2, PDG) | - |

**Applies when:** p >= 0; one of the tabulated particles.

```shell
$ quantum-physics solve particle_decay_length particle=mu- p=0.1056583755
gamma = 1.414213562
beta = 0.7071067812
beta_gamma = 1
lifetime_lab = 3.107000574e-06 s
mean_decay_length = 658.6383866 m
proper_lifetime = 2.196981175e-06 s
mass_MeV = 105.6583755
  by: Mean decay length of a relativistic unstable particle  (beta gamma = p/(m c), L = beta gamma c tau, t_lab = gamma tau)
  p = 0.1056583755
  particle = mu-
  assumes: p >= 0; one of the tabulated particles
```

**What the worked example means:** This calculation determines how far an unstable particle travels in the laboratory frame before decaying. By combining the particle's rest-frame lifetime with relativistic time dilation, it converts the input momentum 0.105658 into a mean decay length of 658.638 m. The result depends on the Lorentz factor derived from the particle's mass and velocity, showing that higher momentum leads to a longer observed path due to time dilation.

### Photons and matter waves (7)

<a id="compton_scattering"></a>

#### Compton scattering: `compton_scattering`

Wavelength and energy of a photon scattered by a free electron at rest through angle theta, and the electron's kinetic energy.

$$
\lambda' - \lambda = \frac{h}{m_e c}\left(1 - \cos\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | incident wavelength (pm, nm, m) | m |
| `theta` | in | scattering angle of the photon (deg or rad) | rad |
| `lambda_prime` | out | scattered wavelength | m |
| `lambda_prime_pm` | out | scattered wavelength in pm | pm |
| `shift_pm` | out | Compton shift lambda' - lambda in pm | pm |
| `E_in_keV` | out | incident photon energy | keV |
| `E_out_keV` | out | scattered photon energy | keV |
| `K_electron_keV` | out | kinetic energy of the recoil electron | keV |

**Applies when:** lambda > 0; 0 <= theta <= 180 deg.

```shell
$ quantum-physics solve compton_scattering "lam=71 pm" "theta=90 deg"
lambda_prime = 7.342631024e-11 m
lambda_prime_pm = 73.42631024 pm
shift_pm = 2.426310235 pm
E_in_keV = 17.46256316 keV
E_out_keV = 16.88552755 keV
K_electron_keV = 0.5770356102 keV
  by: Compton scattering  (lambda' - lambda = (h/(m_e c))(1 - cos theta) = 2 (h/(m_e c)) sin^2(theta/2))
  lam = 7.1e-11 m
  theta = 1.570796327 rad
  assumes: lambda > 0; 0 <= theta <= 180 deg
```

In the worked example, lam = 71 pm, theta = 90 deg go through the equation above and give lambda_prime = 7.342631024e-11 m, lambda_prime_pm = 73.42631024 pm, shift_pm = 2.426310235 pm, E_in_keV = 17.46256316 keV. It applies only when lambda > 0; 0 <= theta <= 180 deg: outside that it says what is wrong instead of returning a number.

<a id="de_broglie_wavelength_mv"></a>

#### de Broglie wavelength of a moving particle, `de_broglie_wavelength_mv`

The de Broglie wavelength lambda = h/(m v) of a particle of mass m moving at speed v, and the relativistic h/(gamma m v).

$$
\lambda = \frac{h}{p} = \frac{h}{m v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the particle | kg |
| `v` | in | speed (may be written as a fraction of c) | m/s |
| `p` | out | momentum m v | kg*m/s |
| `lam` | out | de Broglie wavelength h/(m v) | m |
| `lambda_nm` | out | the same in nm | nm |
| `lambda_rel_nm` | out | relativistic wavelength h/(gamma m v) in nm | nm |
| `inv_lambda` | out | 1/lambda = m v/h in 1/m (well scaled for heavy bodies, whose lambda is tiny) | - |
| `beta` | out | v/c | - |

**Applies when:** m > 0; 0 < v < c.

```shell
$ quantum-physics solve de_broglie_wavelength_mv "m=9.109383714e-31 kg" "v=1000000 m/s"
p = 9.109383714e-25 kg*m/s
lam = 7.273895093e-10 m
lambda_nm = 0.7273895093 nm
lambda_rel_nm = 0.7273854627 nm
inv_lambda = 1374779244
beta = 0.003335640952
  by: de Broglie wavelength of a moving particle  (lambda = h/(m v) (non-relativistic momentum); lambda_rel = h/(gamma m v))
  m = 9.109383714e-31 kg
  v = 1000000 m/s
  assumes: m > 0; 0 < v < c
```

**What the worked example means:** This calculation determines the wave nature of matter by computing the de Broglie wavelength, defined as Planck's constant divided by momentum. For a particle with mass 9.10938e-31 kg and speed 1e+06 m/s, the momentum is 9.10938e-25 kg*m/s, resulting in a wavelength of 7.2739e-10 m. The relativistic correction yields 0.727385 nm nanometers, while the inverse wavelength is 1.37478e+09, highlighting how quantum effects diminish for larger masses.

<a id="de_broglie_accelerated"></a>

#### de Broglie wavelength of an accelerated particle, `de_broglie_accelerated`

The de Broglie wavelength of an electron or proton accelerated from rest through a voltage V, non-relativistic and relativistic.

$$
\lambda = \frac{h}{p},\quad p = \frac{1}{c}\sqrt{K(K + 2mc^2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | accelerating voltage | V |
| `particle` | in | the accelerated particle (charge e) | - |
| `lambda_nonrel` | out | wavelength h/sqrt(2 m e V) | m |
| `lambda_nonrel_nm` | out | the same in nm | nm |
| `lambda_rel` | out | relativistic wavelength | m |
| `lambda_rel_nm` | out | the same in nm | nm |
| `p` | out | relativistic momentum | kg*m/s |
| `K_eV` | out | kinetic energy | eV |

**Applies when:** V > 0.

```shell
$ quantum-physics solve de_broglie_accelerated "V=100 V" particle=electron
lambda_nonrel = 1.226425965e-10 m
lambda_nonrel_nm = 0.1226425965 nm
lambda_rel = 1.226365968e-10 m
lambda_rel_nm = 0.1226365968 nm
p = 5.403012087e-24 kg*m/s
K_eV = 100 eV
  by: de Broglie wavelength of an accelerated particle  (lambda = h/p with p = sqrt(2 m e V) (non-relativistic) or p = sqrt(K (K + 2 m c^2))/c, K = e V)
  V = 100 V
  particle = electron
  assumes: V > 0
```

**What the worked example means:** This calculation determines the quantum wavelength of a charged particle accelerated from rest by a voltage of 100 V. It first computes the kinetic energy as 100 eV electron volts and the resulting relativistic momentum as 5.40301e-24 kg*m/s. The final wavelength, given as 0.122637 nm nanometers, represents the scale at which wave-like interference effects become significant for this specific particle.

<a id="heisenberg_minimum_uncertainty"></a>

#### Minimum momentum uncertainty (Heisenberg): `heisenberg_minimum_uncertainty`

The smallest momentum and velocity uncertainty compatible with a position uncertainty dx: dp = hbar/(2 dx).

$$
\Delta x\,\Delta p \ge \frac{\hbar}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dx` | in | position uncertainty (standard deviation) | m |
| `m` | in | mass of the particle | kg |
| `dp` | out | minimum momentum uncertainty | kg*m/s |
| `dp_eV_per_c` | out | the same in eV/c | - |
| `dv` | out | minimum velocity uncertainty | m/s |

**Applies when:** dx > 0; m > 0.

```shell
$ quantum-physics solve heisenberg_minimum_uncertainty "dx=0.1 nm" "m=9.109383714e-31 kg"
dp = 5.272859088e-25 kg*m/s
dp_eV_per_c = 986.6349023
dv = 578838.1798 m/s
  by: Minimum momentum uncertainty (Heisenberg)  (dp = hbar/(2 dx) (a minimum-uncertainty Gaussian packet), dv = dp/m)
  dx = 1e-10 m
  m = 9.109383714e-31 kg
  assumes: dx > 0; m > 0
```

**What the worked example means:** This calculation determines the smallest possible uncertainty in momentum compatible with a given position uncertainty. Applying the Heisenberg uncertainty principle for a minimum-uncertainty Gaussian wave packet shows that constraining the particle's position to within 0.1 nm results in a momentum spread of at least 5.27286e-25 kg*m/s. For a particle of mass 9.10938e-31 kg, this translates to a minimum velocity uncertainty of 578838 m/s. The result highlights the trade-off inherent in quantum mechanics: sharper position knowledge demands greater momentum indeterminacy.

<a id="photoelectric_effect"></a>

#### Photoelectric effect: `photoelectric_effect`

Maximum kinetic energy of photoelectrons, stopping potential and threshold wavelength from the light's wavelength and the metal's work function.

$$
K_{max} = \frac{hc}{\lambda} - W = e V_{stop}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength of the light (nm, m ...) | m |
| `W` | in | work function of the metal (eV, J) | J |
| `E_photon_eV` | out | photon energy | eV |
| `K_max_eV` | out | maximum kinetic energy of the photoelectrons | eV |
| `V_stop` | out | stopping potential | V |
| `lambda_threshold` | out | threshold (cut-off) wavelength | m |
| `lambda_threshold_nm` | out | threshold wavelength in nm | nm |
| `f_threshold` | out | threshold frequency | Hz |

**Applies when:** wavelength and work function positive; photon energy at least the work function.

```shell
$ quantum-physics solve photoelectric_effect "lam=250 nm" "W=4.3 eV"
E_photon_eV = 4.959367937 eV
K_max_eV = 0.6593679373 eV
V_stop = 0.6593679373 V
lambda_threshold = 2.883353452e-07 m
lambda_threshold_nm = 288.3353452 nm
f_threshold = 1.039735374e+15 Hz
  by: Photoelectric effect  (K_max = h c/lambda - W, V_stop = K_max/e, lambda_0 = h c/W)
  lam = 2.5e-07 m
  W = 6.889359526e-19 J
  assumes: wavelength and work function positive; photon energy at least the work function
```

**What the worked example means:** This calculation finds the maximum kinetic energy of photoelectrons, which is 0.659368 eV, and the corresponding stopping potential of 0.659368 V. These values are derived from the incident light wavelength of 250 nm and the metal work function of 4.3 eV. The photon energy is 4.95937 eV. If this energy exceeds the work function, electrons are ejected with the calculated kinetic energy. The threshold frequency for emission is 1.03974e+15 Hz.

<a id="photon_energy"></a>

#### Photon energy from wavelength: `photon_energy`

The energy E = h c/lambda, frequency and momentum h/lambda of a photon of vacuum wavelength lambda.

$$
E = h f = \frac{hc}{\lambda},\quad p = \frac{h}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | vacuum wavelength (nm, m ...) | m |
| `E` | out | photon energy in J | J |
| `E_eV` | out | photon energy in eV | eV |
| `f` | out | frequency c/lambda | Hz |
| `p` | out | photon momentum h/lambda | kg*m/s |

**Applies when:** lambda > 0; in vacuum.

```shell
$ quantum-physics solve photon_energy "lam=500 nm"
E = 3.972891714e-19 J
E_eV = 2.479683969 eV
f = 5.99584916e+14 Hz
p = 1.32521403e-27 kg*m/s
  by: Photon energy from wavelength  (E = h c/lambda, f = c/lambda, p = h/lambda)
  lam = 5e-07 m
  assumes: lambda > 0; in vacuum
```

**What the worked example means:** This calculation determines the physical properties of a single photon based on its vacuum wavelength. Given an input wavelength of 500 nm, the model computes the photon's frequency as 5.99585e+14 Hz, its total energy in joules as 3.97289e-19 J, and its equivalent energy in electronvolts as 2.47968 eV. Additionally, the linear momentum of the photon is calculated to be 1.32521e-27 kg*m/s. These values describe how much energy and momentum this specific light particle carries through empty space.

<a id="coherent_state_photon_statistics"></a>

#### Photon-number distribution of a coherent state, `coherent_state_photon_statistics`

The probability of n photons in a coherent (laser) state |alpha>, its mean photon number and variance (Poisson statistics).

$$
P(n) = e^{-|\alpha|^2}\frac{|\alpha|^{2n}}{n!}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | coherent amplitude \|alpha\| | - |
| `n` | in | photon number | - |
| `P_n` | out | probability of exactly n photons | - |
| `mean_n` | out | mean photon number \|alpha\|^2 | - |
| `variance` | out | photon-number variance \|alpha\|^2 | - |
| `P_0` | out | probability of no photons | - |

**Applies when:** |alpha| >= 0; n >= 0.

```shell
$ quantum-physics solve coherent_state_photon_statistics alpha=2 n=3
P_n = 0.1953668148
mean_n = 4
variance = 4
P_0 = 0.01831563889
  by: Photon-number distribution of a coherent state  (P(n) = exp(-|alpha|^2) |alpha|^(2n)/n! (computed in logarithms))
  alpha = 2
  assumes: |alpha| >= 0; n >= 0
```

**What the worked example means:** This calculation determines the photon-number statistics for a coherent state defined by the amplitude 2. The result shows that the probability of observing exactly 3 photons is 0.195367, which follows Poisson statistics where the variance equals the mean. For this specific input, the mean photon number is calculated as 4 and the variance is 4, indicating that the light source exhibits classical wave-like properties typical of laser beams.

### Quantum gates (4)

<a id="gate_rotation_axis"></a>

#### Bloch-sphere rotation of a single-qubit gate sequence, `gate_rotation_axis`

The single rotation (axis n and angle) of the Bloch sphere that a sequence of single-qubit gates performs (global phase ignored).

$$
U \simeq e^{-i\theta\,\hat n\cdot\vec\sigma/2} = \cos\tfrac{\theta}{2} I - i\sin\tfrac{\theta}{2}\,\hat n\cdot\vec\sigma
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gates` | in | gates applied left to right, e.g. 'H T Rz(pi/2) X' | - |
| `angle` | out | rotation angle in [0, pi] (rad) | rad |
| `angle_deg` | out | rotation angle in degrees | - |
| `nx` | out | axis x | - |
| `ny` | out | axis y | - |
| `nz` | out | axis z | - |

**Applies when:** gates from H, X, Y, Z, S, Sdg, T, Tdg, I, Rx(t), Ry(t), Rz(t), P(t).

```shell
$ quantum-physics solve gate_rotation_axis gates=H
angle = 3.141592654 rad
angle_deg = 180
nx = 0.7071067812
ny = 0
nz = 0.7071067812
  by: Bloch-sphere rotation of a single-qubit gate sequence  (U/sqrt(det U) = cos(t/2) I - i sin(t/2) n.sigma; angle = 2 atan2(|a|, a0), n = a/|a|)
  gates = H
  assumes: gates from H, X, Y, Z, S, Sdg, T, Tdg, I, Rx(t), Ry(t), Rz(t), P(t)
```

**What the worked example means:** This calculation determines the single equivalent rotation on the Bloch sphere for a sequence of single-qubit gates. Multiplying the unitary matrices of each gate in the provided order gives an overall transformation that ignores global phase. The result specifies the rotation angle as 3.14159 rad radians and defines the axis of rotation using the normalized vector components 0.707107, 0, and 0.707107. This simplifies complex circuit sequences into a basic geometric operation.

<a id="single_qubit_circuit"></a>

#### State of a qubit after a sequence of gates, `single_qubit_circuit`

The amplitudes, measurement probabilities and Bloch vector of a qubit after gates such as H, X, S, T and rotations act on |0>, |1>, |+>, ...

$$
|\psi\rangle = U_k \cdots U_2 U_1 |\psi_0\rangle
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `start` | in | initial state \|0>, \|1>, \|+>, \|->, \|+i> or \|-i> | - |
| `gates` | in | gates applied left to right, e.g. 'H T Rz(pi/2) X' | - |
| `alpha_re` | out | Re alpha (global phase fixed: first non-zero amplitude real positive) | - |
| `alpha_im` | out | Im alpha | - |
| `beta_re` | out | Re beta | - |
| `beta_im` | out | Im beta | - |
| `p0` | out | probability of measuring 0 | - |
| `p1` | out | probability of measuring 1 | - |
| `bx` | out | Bloch x | - |
| `by` | out | Bloch y | - |
| `bz` | out | Bloch z | - |

**Applies when:** gates from H, X, Y, Z, S, Sdg, T, Tdg, I, Rx(t), Ry(t), Rz(t), P(t); at most 500 gates.

```shell
$ quantum-physics solve single_qubit_circuit gates=H start=0
alpha_re = 0.7071067812
alpha_im = 0
beta_re = 0.7071067812
beta_im = 0
p0 = 0.5
p1 = 0.5
bx = 1
by = 0
bz = 0
  by: State of a qubit after a sequence of gates  (the state vector multiplied by each gate's 2x2 matrix in turn)
  start = 0
  gates = H
  assumes: gates from H, X, Y, Z, S, Sdg, T, Tdg, I, Rx(t), Ry(t), Rz(t), P(t); at most 500 gates
```

**What the worked example means:** This calculation determines the quantum state of a single qubit after it passes through a sequence of logic gates. Starting from the initial configuration specified as 0, each gate in the list H is applied sequentially by multiplying the current state vector by that gate's matrix representation. The resulting complex numbers define the new amplitudes, which dictate the likelihood of observing specific outcomes upon measurement. Specifically, the probability of measuring the zero state is 0.5, while the probability of measuring the one state is 0.5. These probabilities are derived directly from the squared magnitudes of the computed amplitudes, providing a complete statistical description of the qubit's behavior at the end of the circuit.

<a id="grover_search"></a>

#### Success probability of Grover's search: `grover_search`

The probability of measuring a marked item after k Grover iterations on n qubits with M marked items, and the best number of iterations.

$$
P_k = \sin^2\!\big((2k+1)\theta\big),\quad \sin\theta = \sqrt{M/N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n_qubits` | in | number of qubits (search space N = 2^n) | - |
| `M` | in | number of marked items | - |
| `k` | in | number of Grover iterations | - |
| `P_success` | out | probability of a marked item after k iterations | - |
| `k_opt` | out | best iteration count floor(pi/(4 theta)) | - |
| `P_at_k_opt` | out | success probability at k_opt | - |
| `theta` | out | rotation half-angle asin(sqrt(M/N)) | rad |

**Applies when:** 1 <= M < N/2; 0 <= k <= 10^7.

```shell
$ quantum-physics solve grover_search n_qubits=2 M=1 k=1
P_success = 1
k_opt = 1
P_at_k_opt = 1
theta = 0.5235987756 rad
  by: Success probability of Grover's search  (P_k = sin^2((2k + 1) theta), sin(theta) = sqrt(M/N); k_opt = floor(pi/(4 theta)) (the first maximum))
  assumes: 1 <= M < N/2; 0 <= k <= 10^7
```

In the worked example, n_qubits = 2, M = 1, k = 1 go through the equation above and give P_success = 1, k_opt = 1, P_at_k_opt = 1, theta = 0.5235987756 rad. It applies only when 1 <= M < N/2; 0 <= k <= 10^7: outside that it says what is wrong instead of returning a number.

<a id="two_qubit_circuit"></a>

#### Two-qubit circuit: probabilities and entanglement, `two_qubit_circuit`

Measurement probabilities of |00>, |01>, |10>, |11> and the concurrence (entanglement) after a two-qubit gate sequence such as H0 CNOT01 (a Bell state).

$$
|\Phi^+\rangle = \mathrm{CNOT}_{01}(H\otimes I)|00\rangle = \tfrac{1}{\sqrt2}(|00\rangle + |11\rangle)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `start` | in | initial basis state \|q0 q1> | - |
| `gates` | in | gates left to right: H0, X1, Rz0(t), ..., CNOT01 (control q0), CNOT10, CZ, SWAP | - |
| `p00` | out | probability of \|00> | - |
| `p01` | out | probability of \|01> | - |
| `p10` | out | probability of \|10> | - |
| `p11` | out | probability of \|11> | - |
| `concurrence` | out | concurrence 2\|a00 a11 - a01 a10\| (0 product, 1 maximally entangled) | - |
| `p_q0_is_1` | out | probability that q0 reads 1 | - |
| `p_q1_is_1` | out | probability that q1 reads 1 | - |

**Applies when:** gates from the documented set; at most 500 gates.

```shell
$ quantum-physics solve two_qubit_circuit "gates=H0 CNOT01" start=00
p00 = 0.5
p01 = 0
p10 = 0
p11 = 0.5
concurrence = 1
p_q0_is_1 = 0.5
p_q1_is_1 = 0.5
  by: Two-qubit circuit: probabilities and entanglement  (the four-amplitude state vector updated gate by gate (index 2 q0 + q1))
  start = 00
  gates = H0 CNOT01
  assumes: gates from the documented set; at most 500 gates
```

**What the worked example means:** This calculation determines the measurement outcomes for a two-qubit system starting from 00 after applying the sequence H0 CNOT01. It computes the joint probability for each basis state, resulting in 0.5 for both zeros, 0 for zero then one, 0 for one then zero, and 0.5 for both ones. The marginal probability that the first qubit reads one is 0.5, while the second qubit reads one with probability 0.5. Entanglement is quantified by the concurrence value of 1, where higher values indicate stronger quantum correlations between the two qubits.

### Quantum wells (10)

<a id="finite_square_well"></a>

#### Bound states of a finite square well: `finite_square_well`

The number and energies of the bound states of a particle of mass m in a square well of depth V0 and width a.

$$
z\tan z = \sqrt{z_0^2 - z^2},\quad -z\cot z = \sqrt{z_0^2 - z^2},\quad z_0 = \frac{a}{2\hbar}\sqrt{2mV_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V0` | in | depth of the well (eV, J) | J |
| `a` | in | width of the well | m |
| `m` | in | particle mass | kg |
| `n_bound` | out | number of bound states | - |
| `E1_eV` | out | ground-state energy above the bottom of the well | eV |
| `energies_eV` | out | all bound-state energies above the bottom (eV), ascending | - |
| `z0` | out | well strength (a/2) sqrt(2 m V0)/hbar | - |

**Applies when:** V0, a, m > 0; at most 127 bound states; no state exactly at the top of the well.

```shell
$ quantum-physics solve finite_square_well "V0=10 eV" "a=0.5 nm" "m=9.109383714e-31 kg"
n_bound = 3
E1_eV = 0.9610429053 eV
energies_eV = [0.961042905296,3.75528284703,7.95049450889]
z0 = 4.050219315
  by: Bound states of a finite square well  (even states z tan z = sqrt(z0^2 - z^2), odd states -z cot z = sqrt(z0^2 - z^2), z = (a/2) sqrt(2 m E)/hbar, solved by bisection)
  V0 = 1.602176634e-18 J
  a = 5e-10 m
  m = 9.109383714e-31 kg
  assumes: V0, a, m > 0; at most 127 bound states; no state exactly at the top of the well
```

**What the worked example means:** This calculation determines the discrete energy levels for a particle of mass 9.10938e-31 kg confined in a one-dimensional potential well of depth 10 eV and width 0.5 nm. Solving the transcendental equations for even and odd parity states using the dimensionless variable z shows that the well strength is 4.05022. The result indicates there are 3 total bound states, with the lowest energy level measured at 0.961043 eV above the bottom of the well. These values describe how the quantum mechanical confinement restricts the particle to specific allowed energies.

<a id="infinite_well_levels"></a>

#### Energy levels of a particle in a box: `infinite_well_levels`

Energy E_n = n^2 h^2/(8 m L^2), momentum and de Broglie wavelength of level n of a particle in a one-dimensional box of width L.

$$
E_n = \frac{n^2 h^2}{8 m L^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | particle mass | kg |
| `L` | in | width of the box | m |
| `n` | in | quantum number (1, 2, 3, ...) | - |
| `E` | out | energy of level n | J |
| `E_eV` | out | the same in eV | eV |
| `E1_eV` | out | ground-state energy | eV |
| `p` | out | magnitude of the momentum n pi hbar/L | kg*m/s |
| `wavelength` | out | de Broglie wavelength 2L/n | m |

**Applies when:** m > 0; L > 0; n >= 1; infinitely high walls.

```shell
$ quantum-physics solve infinite_well_levels "m=9.109383714e-31 kg" "L=1 nm" n=1
E = 6.024667394e-20 J
E_eV = 0.3760301621 eV
E1_eV = 0.3760301621 eV
p = 3.313035075e-25 kg*m/s
wavelength = 2e-09 m
  by: Energy levels of a particle in a box  (E_n = (n pi hbar/L)^2/(2m) = n^2 h^2/(8 m L^2))
  m = 9.109383714e-31 kg
  L = 1e-09 m
  assumes: m > 0; L > 0; n >= 1; infinitely high walls
```

**What the worked example means:** The calculation determines the quantized energy levels for a particle confined within a one-dimensional box. Using the input mass 9.10938e-31 kg, box width 1 nm, and quantum number 1, the model computes the total energy as 6.02467e-20 J. This result represents the discrete kinetic energy state allowed by the boundary conditions, with the ground state energy given by 0.37603 eV. The momentum magnitude is found to be 3.31304e-25 kg*m/s, corresponding to a de Broglie wavelength of 2e-09 m.

<a id="infinite_well_transition"></a>

#### Photon from a particle-in-a-box transition, `infinite_well_transition`

Energy, wavelength and frequency of the photon emitted when a particle in a box of width L drops from n_upper to n_lower.

$$
\Delta E = \frac{(n_u^2 - n_\ell^2)\,h^2}{8 m L^2} = \frac{hc}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | particle mass | kg |
| `L` | in | width of the box | m |
| `n_upper` | in | upper level | - |
| `n_lower` | in | lower level | - |
| `E_photon_eV` | out | photon energy | eV |
| `lam` | out | wavelength | m |
| `lambda_nm` | out | wavelength in nm | nm |
| `f` | out | frequency | Hz |

**Applies when:** n_upper > n_lower >= 1.

```shell
$ quantum-physics solve infinite_well_transition "m=9.109383714e-31 kg" "L=1 nm" n_upper=2 n_lower=1
E_photon_eV = 1.128090486 eV
lam = 1.09906253e-06 m
lambda_nm = 1099.06253 nm
f = 2.72771066e+14 Hz
  by: Photon from a particle-in-a-box transition  (E = (n_u^2 - n_l^2) h^2/(8 m L^2), lambda = h c/E)
  m = 9.109383714e-31 kg
  L = 1e-09 m
  assumes: n_upper > n_lower >= 1
```

**What the worked example means:** This calculation determines the properties of a photon emitted when a particle transitions from an upper energy level to a lower one within a one-dimensional box. The energy difference between these states, calculated using the particle mass 9.10938e-31 kg, box width 1 nm, and quantum numbers 2 and 1, results in a photon with energy 1.12809 eV. This energy corresponds to a specific wavelength of 1099.06 nm and a frequency of 2.72771e+14 Hz, illustrating the discrete nature of quantum emission spectra.

<a id="infinite_well_uncertainty"></a>

#### Position and momentum spread in a box: `infinite_well_uncertainty`

<x>, the position spread dx, the momentum spread dp and the product dx dp of level n of a particle in a box of width L.

$$
\Delta x = L\sqrt{\frac{1}{12} - \frac{1}{2n^2\pi^2}},\quad \Delta p = \frac{n\pi\hbar}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | quantum number | - |
| `L` | in | width of the box | m |
| `x_mean` | out | <x> (the centre) | m |
| `dx` | out | position spread | m |
| `dx_over_L` | out | dx/L | - |
| `dp` | out | momentum spread n pi hbar/L | kg*m/s |
| `product_over_hbar` | out | dx dp / hbar (at least 1/2) | - |

**Applies when:** n >= 1; L > 0.

```shell
$ quantum-physics solve infinite_well_uncertainty n=1 "L=1 m"
x_mean = 0.5 m
dx = 0.1807560276 m
dx_over_L = 0.1807560276
dp = 3.313035075e-34 kg*m/s
product_over_hbar = 0.5678618084
  by: Position and momentum spread in a box  (<x^2> = L^2 (1/3 - 1/(2 n^2 pi^2)), dx = sqrt(<x^2> - L^2/4), dp = n pi hbar/L)
  L = 1 m
  assumes: n >= 1; L > 0
```

**What the worked example means:** This calculation determines the uncertainty in position and momentum for a quantum particle confined within a one-dimensional box. Given a box width of 1 m and a quantum number of 1, the mean position is found to be 0.5 m, while the standard deviation of the position, representing how spread out the particle's location is, equals 0.180756 m. The corresponding spread in momentum, which reflects the range of possible momenta due to confinement, is calculated as 3.31304e-34 kg*m/s. The dimensionless ratio of these uncertainties scaled by Planck's constant yields 0.567862, confirming that the product satisfies the Heisenberg uncertainty principle.

<a id="harmonic_oscillator_forbidden_probability"></a>

#### Probability beyond the classical turning points (harmonic oscillator), `harmonic_oscillator_forbidden_probability`

The probability that a harmonic oscillator in level n is found in the classically forbidden region |x| > A_n.

$$
P = 1 - \int_{-\sqrt{2n+1}}^{\sqrt{2n+1}} |\psi_n(\xi)|^2\,d\xi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | quantum number (0 to 60) | - |
| `P_forbidden` | out | probability outside the turning points | - |
| `P_allowed` | out | probability inside | - |
| `xi_turning` | out | turning point in units of sqrt(hbar/(m omega)) | - |

**Applies when:** 0 <= n <= 60.

```shell
$ quantum-physics solve harmonic_oscillator_forbidden_probability n=0
P_forbidden = 0.1572992071
P_allowed = 0.8427007929
xi_turning = 1
  by: Probability beyond the classical turning points (harmonic oscillator)  (integral of H_n(xi)^2 exp(-xi^2) over |xi| < sqrt(2n + 1), term by term with the incomplete gamma function (50 digits))
  assumes: 0 <= n <= 60
```

**What the worked example means:** This calculation determines the quantum tunneling probability for a harmonic oscillator in energy level 0 by integrating the wave function squared beyond the classical limits. The result shows that there is a 0.157299 chance of detecting the particle in the classically forbidden region, which corresponds to a position magnitude greater than 1 scaled units. Conversely, the probability of finding the particle inside the classical bounds is 0.842701. This illustrates how quantum mechanics allows particles to exist where classical physics would strictly forbid them due to insufficient kinetic energy.

<a id="infinite_well_probability"></a>

#### Probability of finding a particle in part of a box, `infinite_well_probability`

The probability that a particle in level n of a box of width L is found between x1 and x2.

$$
P = \int_{x_1}^{x_2} \frac{2}{L}\sin^2\frac{n\pi x}{L}\,dx
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | quantum number | - |
| `L` | in | width of the box | m |
| `x1` | in | left end of the interval (from the wall) | m |
| `x2` | in | right end of the interval | m |
| `P` | out | probability | - |
| `classical` | out | classical (uniform) probability (x2 - x1)/L | - |

**Applies when:** 0 <= x1 < x2 <= L; n >= 1.

```shell
$ quantum-physics solve infinite_well_probability n=1 "L=1 nm" "x1=0.25 nm" "x2=0.75 nm"
P = 0.8183098862
classical = 0.5
  by: Probability of finding a particle in part of a box  (P = (x2 - x1)/L - [sin(2 n pi x2/L) - sin(2 n pi x1/L)]/(2 n pi), evaluated without cancellation)
  L = 1e-09 m
  x1 = 2.5e-10 m
  x2 = 7.5e-10 m
  assumes: 0 <= x1 < x2 <= L; n >= 1
```

**What the worked example means:** This calculation determines the likelihood of locating a particle within a specific region of an infinite potential well. Using the quantum number 1 and box width 1 nm, it integrates the squared wavefunction from the left boundary 0.25 nm to the right boundary 0.75 nm. The resulting quantum probability is 0.81831, which differs from the uniform classical expectation of 0.5, highlighting the non-uniform distribution inherent in quantum states.

<a id="harmonic_oscillator_levels"></a>

#### Quantum harmonic oscillator levels: `harmonic_oscillator_levels`

Energy (n + 1/2) hbar omega, zero-point energy, level spacing, classical amplitude and position spread of level n.

$$
E_n = \left(n + \tfrac12\right)\hbar\omega
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `omega` | in | angular frequency | rad/s |
| `n` | in | quantum number (0, 1, 2, ...) | - |
| `E` | out | energy of level n | J |
| `E_eV` | out | the same in eV | eV |
| `E0_eV` | out | zero-point energy | eV |
| `spacing_eV` | out | level spacing hbar omega | eV |
| `amplitude` | out | classical turning point | m |
| `dx` | out | position spread sqrt(<x^2>) | m |

**Applies when:** m > 0; omega > 0; n >= 0.

```shell
$ quantum-physics solve harmonic_oscillator_levels "m=1.672621926e-27 kg" "omega=1e+14 rad/s" n=0
E = 5.272859088e-21 J
E_eV = 0.03291059785 eV
E0_eV = 0.03291059785 eV
spacing_eV = 0.0658211957 eV
amplitude = 2.510956493e-11 m
dx = 1.775514363e-11 m
  by: Quantum harmonic oscillator levels  (E_n = (n + 1/2) hbar omega, A = sqrt((2n + 1) hbar/(m omega)), dx = sqrt((n + 1/2) hbar/(m omega)))
  m = 1.672621926e-27 kg
  omega = 1e+14 rad/s
  assumes: m > 0; omega > 0; n >= 0
```

In the worked example, m = 1.672621926e-27 kg, omega = 1e+14 rad/s, n = 0 go through the equation above and give E = 5.272859088e-21 J, E_eV = 0.03291059785 eV, E0_eV = 0.03291059785 eV, spacing_eV = 0.0658211957 eV. It applies only when m > 0; omega > 0; n >= 0: outside that it says what is wrong instead of returning a number.

<a id="step_potential"></a>

#### Reflection and transmission at a potential step, `step_potential`

Reflection and transmission probabilities of a particle of energy E meeting a potential step of height V0 (V = 0 for x < 0, V0 for x > 0).

$$
R = \left(\frac{k_1 - k_2}{k_1 + k_2}\right)^2,\quad T = \frac{4k_1k_2}{(k_1 + k_2)^2},\quad \frac{k_2}{k_1} = \sqrt{\frac{E - V_0}{E}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | kinetic energy of the incident particle (on the low side) | J |
| `V0` | in | height of the step | J |
| `R` | out | reflection probability | - |
| `T` | out | transmission probability (ratio of probability currents) | - |
| `k2_over_k1` | out | ratio of wavenumbers sqrt((E - V0)/E) (0 when E <= V0) | - |
| `regime` | out | partial transmission (E > V0) or total reflection | - |

**Applies when:** E > 0; V0 > 0; the particle comes from x < 0.

```shell
$ quantum-physics solve step_potential "E=4 eV" "V0=3 eV"
R = 0.1111111111
T = 0.8888888889
k2_over_k1 = 0.5
regime = partial transmission (E > V0)
  by: Reflection and transmission at a potential step  (R = ((k1 - k2)/(k1 + k2))^2, T = 4 k1 k2/(k1 + k2)^2 for E > V0; R = 1, T = 0 for E <= V0)
  E = 6.408706536e-19 J
  V0 = 4.806529902e-19 J
  assumes: E > 0; V0 > 0; the particle comes from x < 0
```

**What the worked example means:** This calculation determines the likelihood that a quantum particle with incident energy 4 eV reflects off or transmits through a potential step of height 3 eV. The resulting reflection probability is 0.111111 and the transmission probability is 0.888889, derived from the wavenumber ratio 0.5. This outcome corresponds to the physical regime described as partial transmission (E > V0), illustrating how quantum mechanics permits partial reflection even when classical physics might predict complete transmission.

<a id="barrier_transmission"></a>

#### Transmission through a rectangular barrier, `barrier_transmission`

Transmission and reflection probabilities of a particle of energy E meeting a rectangular barrier of height V0 and width a (below or above the top).

$$
T = \left[1 + \frac{V_0^2\sinh^2(\kappa a)}{4E(V_0 - E)}\right]^{-1},\quad \kappa = \frac{\sqrt{2m(V_0 - E)}}{\hbar}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | kinetic energy of the incident particle | J |
| `V0` | in | barrier height | J |
| `a` | in | barrier width | m |
| `m` | in | particle mass | kg |
| `T` | out | transmission probability | - |
| `R` | out | reflection probability | - |
| `kappa_a` | out | kappa a = a sqrt(2m(V0 - E))/hbar (0 above the barrier) | - |
| `regime` | out | tunnelling (E < V0) or over the barrier | - |

**Applies when:** E, V0, a, m > 0.

```shell
$ quantum-physics solve barrier_transmission "E=1 eV" "V0=2 eV" "a=0.5 nm" "m=9.109383714e-31 kg"
T = 0.02354711984
R = 0.9764528802
kappa_a = 2.561583612
regime = tunnelling (E < V0)
  by: Transmission through a rectangular barrier  (T = 1/(1 + V0^2 sinh^2(kappa a)/(4E(V0 - E))) for E < V0, sin for E > V0, written as sinh(x)/x to stay finite at E = V0)
  E = 1.602176634e-19 J
  V0 = 3.204353268e-19 J
  a = 5e-10 m
  m = 9.109383714e-31 kg
  assumes: E, V0, a, m > 0
```

**What the worked example means:** This calculation determines how likely a particle is to pass through or bounce off a rectangular barrier. Given an incident energy of 1 eV, a barrier height of 2 eV, and a width of 0.5 nm, the model computes the transmission probability as 0.0235471 and the reflection probability as 0.976453. The result indicates whether the particle tunnels through when its energy is lower than the barrier, or scatters over it when higher.

<a id="wkb_field_emission"></a>

#### WKB tunnelling through a triangular barrier (field emission), `wkb_field_emission`

The WKB tunnelling exponent and probability of an electron at the Fermi level through the triangular barrier W - eFx of a metal in a field F.

$$
T \approx \exp\!\left(-\frac{4\sqrt{2m_e}\,W^{3/2}}{3 e\hbar F}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | barrier height above the electron's energy (work function, eV) | J |
| `F` | in | applied electric field (V/m) | N/C |
| `exponent` | out | WKB exponent 2 integral kappa dx | - |
| `T` | out | tunnelling probability exp(-exponent) | - |
| `log10_T` | out | log10 of the probability | - |
| `width` | out | barrier width W/(eF) | m |
| `width_nm` | out | width in nm | nm |

**Applies when:** W > 0; F > 0; WKB: exponent much larger than 1.

```shell
$ quantum-physics solve wkb_field_emission "W=4.5 eV" "F=5000000000 V/m"
exponent = 13.04145462
T = 2.16854391e-06
log10_T = -5.66383178
width = 9e-10 m
width_nm = 0.9 nm
  by: WKB tunnelling through a triangular barrier (field emission)  (T = exp(-(4/3) sqrt(2 m_e) W^(3/2)/(e hbar F)) (Fowler-Nordheim exponent, no image force))
  W = 7.209794853e-19 J
  F = 5000000000 N/C
  assumes: W > 0; F > 0; WKB: exponent much larger than 1
```

**What the worked example means:** This calculation determines the quantum tunnelling probability for an electron facing a triangular potential barrier created by a metal's work function and an applied electric field. Using the Fowler-Nordheim exponent derived from the WKB approximation, the model computes that for a barrier height of 4.5 eV and a field strength of 5e+09 V/m, the resulting tunnelling probability is 2.16854e-06. The associated WKB exponent is 13.0415, which indicates how strongly the wavefunction decays across the barrier width of 0.9 nm nanometers.

### Special relativity (13)

<a id="fixed_target_cm_energy"></a>

#### Centre-of-mass energy of a fixed-target collision, `fixed_target_cm_energy`

sqrt(s) for a beam particle of mass m1 and total energy E1 striking a target of mass m2 at rest (natural units, GeV).

$$
\sqrt{s} = \sqrt{m_1^2 c^4 + m_2^2 c^4 + 2 E_1 m_2 c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | beam particle mass (GeV/c^2) | - |
| `E1` | in | beam particle total energy (GeV) | - |
| `m2` | in | target mass (GeV/c^2) | - |
| `sqrt_s` | out | centre-of-mass energy sqrt(s) (GeV) | - |
| `available` | out | energy available for new particles, sqrt(s) - m1 - m2 (GeV) | - |
| `beta_cm` | out | speed of the centre-of-mass frame / c | - |
| `gamma_cm` | out | its Lorentz factor | - |

**Applies when:** E1 >= m1 >= 0; m2 > 0.

```shell
$ quantum-physics solve fixed_target_cm_energy m1=1 E1=7 m2=1
sqrt_s = 4
available = 2
beta_cm = 0.8660254038
gamma_cm = 2
  by: Centre-of-mass energy of a fixed-target collision  (s = m1^2 + m2^2 + 2 E1 m2)
  m1 = 1
  E1 = 7
  m2 = 1
  assumes: E1 >= m1 >= 0; m2 > 0
```

**What the worked example means:** This calculation determines the effective collision energy, represented as sqrt(s), which equals 4. This value quantifies the total energy available in the frame where the net momentum is zero. From this, the usable energy for creating new particles is derived by subtracting the rest masses of the initial beam and target, resulting in 2. The motion of this reference frame relative to the laboratory is characterized by a speed fraction of 0.866025 and a Lorentz factor of 2.

<a id="invariant_mass"></a>

#### Invariant mass of two particles: `invariant_mass`

The invariant mass M = sqrt((E1 + E2)^2 - |p1 + p2|^2) of two particles, in natural units (GeV, GeV/c, GeV/c^2).

$$
M c^2 = \sqrt{(E_1 + E_2)^2 - |\vec p_1 + \vec p_2|^2 c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E1` | in | energy of particle 1 (GeV) | - |
| `p1` | in | momentum of particle 1 (px, py, pz in GeV/c) | - |
| `E2` | in | energy of particle 2 (GeV) | - |
| `p2` | in | momentum of particle 2 (px, py, pz in GeV/c) | - |
| `M` | out | invariant mass (GeV/c^2) | - |
| `P` | out | magnitude of the total momentum (GeV/c) | - |
| `beta_cm` | out | speed of the centre-of-mass frame / c | - |
| `gamma_cm` | out | Lorentz factor of the centre-of-mass frame | - |

**Applies when:** E >= |p| for each particle; a positive invariant mass.

```shell
$ quantum-physics solve invariant_mass E1=50 "p1=[30, 0, 40]" E2=50 "p2=[-30, 0, -40]"
M = 100
P = 0
beta_cm = 0
gamma_cm = 1
  by: Invariant mass of two particles  (M^2 = (E - |P|)(E + |P|) with E = E1 + E2 and P = p1 + p2)
  E1 = 50
  E2 = 50
  assumes: E >= |p| for each particle; a positive invariant mass
```

**What the worked example means:** This calculation determines the invariant mass, which represents the total rest energy of the system in its center-of-mass frame. Using the energies and momenta of particle one (50 and [30, 0, 40]) and particle two (50 and [-30, 0, -40]) shows that the combined invariant mass is 100. The resulting speed of the center-of-mass frame relative to the laboratory is characterized by beta cm equal to 0 and a Lorentz factor of 1. This value remains constant regardless of the observer's reference frame, provided the particles form an isolated system.

<a id="length_contraction"></a>

#### Length contraction: `length_contraction`

The length L = L0/gamma of a rod moving along its length at speed v, given its proper length L0.

$$
L = \frac{L_0}{\gamma} = L_0\sqrt{1 - v^2/c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L0` | in | proper length (in the rod's rest frame) | m |
| `v` | in | speed of the rod (may be written as a fraction of c) | m/s |
| `L` | out | contracted length measured in the frame where the rod moves | m |
| `gamma` | out | Lorentz factor | - |
| `shortening` | out | L0 - L | m |

**Applies when:** L0 > 0; 0 <= v < c.

```shell
$ quantum-physics solve length_contraction "L0=100 m" "v=179875474.8 m/s"
L = 80 m
gamma = 1.25
shortening = 20 m
  by: Length contraction  (L = L0 sqrt(1 - v^2/c^2))
  L0 = 100 m
  v = 179875474.8 m/s
  assumes: L0 > 0; 0 <= v < c
```

**What the worked example means:** This calculation determines the observed length of a moving rod by applying the Lorentz contraction formula. Given a proper length of 100 m meters and a velocity of 1.79875e+08 m/s, the resulting measured length is 80 m meters. The Lorentz factor for this scenario is 1.25, which quantifies the degree of time dilation and length compression experienced by the observer relative to the moving object.

<a id="lorentz_factor"></a>

#### Lorentz factor: `lorentz_factor`

The Lorentz factor gamma = 1/sqrt(1 - v^2/c^2) of a speed v.

$$
\gamma = \frac{1}{\sqrt{1 - v^2/c^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | speed (may be written as a fraction of c, e.g. 0.8 c) | m/s |
| `gamma` | out | Lorentz factor | - |
| `beta` | out | v/c | - |
| `gamma_minus_1` | out | gamma - 1 (computed without cancellation) | - |

**Applies when:** 0 <= v < c.

```shell
$ quantum-physics solve lorentz_factor "v=179875474.8 m/s"
gamma = 1.25
beta = 0.6
gamma_minus_1 = 0.25
  by: Lorentz factor  (gamma = 1/sqrt((1 - beta)(1 + beta)), beta = v/c)
  v = 179875474.8 m/s
  assumes: 0 <= v < c
```

**What the worked example means:** This calculation determines the Lorentz factor, which scales time dilation and length contraction for an object moving at speed 1.79875e+08 m/s. The result shows that gamma equals 1.25, meaning relativistic effects are significant. The ratio of the speed to the speed of light, beta, is calculated as 0.6. This value quantifies how much measurements in the moving frame differ from those in the stationary frame.

<a id="relativistic_aberration"></a>

#### Relativistic aberration of light: `relativistic_aberration`

The direction of a light ray seen from a frame moving at speed v along +x, given its direction theta (from +x) in the rest frame.

$$
\cos\theta' = \frac{\cos\theta - \beta}{1 - \beta\cos\theta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta` | in | direction of travel of the light in the rest frame, from the +x axis (rad or deg) | rad |
| `v` | in | speed of the moving frame along +x (may be a fraction of c) | m/s |
| `theta_prime` | out | direction of travel in the moving frame (rad) | rad |
| `theta_prime_deg` | out | the same angle in degrees | - |
| `doppler_factor` | out | photon energy ratio E'/E | - |

**Applies when:** 0 <= theta <= 180 deg; |v| < c.

```shell
$ quantum-physics solve relativistic_aberration "theta=90 deg" "v=179875474.8 m/s"
theta_prime = 2.214297436 rad
theta_prime_deg = 126.8698976
doppler_factor = 1.25
  by: Relativistic aberration of light  (cos(theta') = (cos(theta) - beta)/(1 - beta cos(theta)), from the boosted photon four-momentum)
  theta = 1.570796327 rad
  v = 179875474.8 m/s
  assumes: 0 <= theta <= 180 deg; |v| < c
```

**What the worked example means:** Relativistic aberration shifts the observed direction of light when moving at speed 1.79875e+08 m/s. Given an original angle of 90 deg, the new direction is calculated as 2.2143 rad. The result also includes the energy ratio, or Doppler factor, which is 1.25. This shows how both the path and frequency of the light change for the moving observer.

<a id="relativistic_doppler"></a>

#### Relativistic Doppler shift: `relativistic_doppler`

The observed frequency of light from a source approaching or receding at speed v (along the line of sight).

$$
f_{obs} = f\sqrt{\frac{1 \pm \beta}{1 \mp \beta}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | frequency in the source frame | Hz |
| `v` | in | relative speed (may be a fraction of c) | m/s |
| `motion` | in | whether source and observer approach or recede | - |
| `f_obs` | out | observed frequency | Hz |
| `ratio` | out | f_obs / f | - |
| `z` | out | redshift z = f/f_obs - 1 (negative for a blueshift) | - |

**Applies when:** f > 0; 0 <= v < c.

```shell
$ quantum-physics solve relativistic_doppler "f=1e+14 Hz" "v=179875474.8 m/s" motion=approaching
f_obs = 2e+14 Hz
ratio = 2
z = -0.5
  by: Relativistic Doppler shift  (f_obs = f sqrt((1 + beta)/(1 - beta)) approaching, f sqrt((1 - beta)/(1 + beta)) receding)
  f = 1e+14 Hz
  v = 179875474.8 m/s
  motion = approaching
  assumes: f > 0; 0 <= v < c
```

**What the worked example means:** This calculation determines the frequency of light detected by an observer when the source moves directly toward or away from them. Using the source frequency 1e+14 Hz and relative speed 1.79875e+08 m/s, it applies the relativistic Doppler formula to account for time dilation and path length changes. The resulting observed frequency is 2e+14 Hz, which corresponds to a shift ratio of 2 and a redshift value of -0.5. If the source approaches, the frequency increases; if it recedes, the frequency decreases.

<a id="relativistic_energy_from_momentum"></a>

#### Relativistic energy from momentum: `relativistic_energy_from_momentum`

Total and kinetic energy and speed of a particle of mass m with momentum p, from E^2 = (pc)^2 + (mc^2)^2.

$$
E^2 = (pc)^2 + (mc^2)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | rest mass | kg |
| `p` | in | momentum | kg*m/s |
| `E` | out | total energy | J |
| `K` | out | kinetic energy | J |
| `v` | out | speed | m/s |
| `beta` | out | v/c | - |
| `gamma` | out | Lorentz factor | - |
| `E_MeV` | out | total energy in MeV | MeV |
| `K_MeV` | out | kinetic energy in MeV | MeV |

**Applies when:** m > 0; p >= 0.

```shell
$ quantum-physics solve relativistic_energy_from_momentum "m=1 kg" "p=224844343.5 kg*m/s"
E = 1.123443973e+17 J
K = 2.246887947e+16 J
v = 179875474.8 m/s
beta = 0.6
gamma = 1.25
E_MeV = 7.011985755e+29 MeV
K_MeV = 1.402397151e+29 MeV
  by: Relativistic energy from momentum  (E = sqrt((pc)^2 + (mc^2)^2), K = (pc)^2/(E + mc^2), v = pc^2/E)
  m = 1 kg
  p = 224844343.5 kg*m/s
  assumes: m > 0; p >= 0
```

**What the worked example means:** This calculation determines the relativistic properties of a particle with rest mass 1 kg and momentum 2.24844e+08 kg*m/s. Using the energy-momentum relation, the total energy is found to be 1.12344e+17 J, while the kinetic energy, representing the energy due to motion alone, is 2.24689e+16 J. The resulting velocity of the particle is 1.79875e+08 m/s, which corresponds to a Lorentz factor of 1.25 and a normalized speed beta of 0.6. These values describe how much energy is stored in the particle's motion versus its intrinsic mass.

<a id="relativistic_momentum_energy"></a>

#### Relativistic momentum and energy: `relativistic_momentum_energy`

Momentum p = gamma m v, total energy E = gamma m c^2, kinetic energy (gamma - 1) m c^2 and rest energy of a mass m at speed v.

$$
p = \gamma m v,\quad E = \gamma m c^2,\quad K = (\gamma - 1) m c^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | rest mass | kg |
| `v` | in | speed (may be written as a fraction of c) | m/s |
| `p` | out | relativistic momentum | kg*m/s |
| `E` | out | total energy | J |
| `K` | out | kinetic energy | J |
| `E0` | out | rest energy m c^2 | J |
| `gamma` | out | Lorentz factor | - |
| `E_MeV` | out | total energy in MeV | MeV |
| `K_MeV` | out | kinetic energy in MeV | MeV |
| `pc_MeV` | out | momentum times c in MeV | MeV |

**Applies when:** m > 0; 0 <= v < c.

```shell
$ quantum-physics solve relativistic_momentum_energy "m=1 kg" "v=239833966.4 m/s"
p = 399723277.3 kg*m/s
E = 1.497925298e+17 J
K = 5.991701192e+16 J
E0 = 8.987551787e+16 J
gamma = 1.666666667
E_MeV = 9.34931434e+29 MeV
K_MeV = 3.739725736e+29 MeV
pc_MeV = 7.479451472e+29 MeV
  by: Relativistic momentum and energy  (p = gamma m v, E = gamma m c^2, K = (gamma - 1) m c^2)
  m = 1 kg
  v = 239833966.4 m/s
  assumes: m > 0; 0 <= v < c
```

**What the worked example means:** This calculation determines relativistic dynamics for a particle with rest mass 1 kg traveling at speed 2.39834e+08 m/s. The Lorentz factor, calculated as 1.66667, scales both momentum and energy. Consequently, the relativistic momentum is 3.99723e+08 kg*m/s, the total energy is 1.49793e+17 J, and the kinetic energy is 5.9917e+16 J. The rest energy alone is 8.98755e+16 J. These values are also expressed in mega-electronvolts as 9.34931e+29 MeV for total energy, 3.73973e+29 MeV for kinetic energy, and 7.47945e+29 MeV for momentum times c.

<a id="relativistic_velocity_addition"></a>

#### Relativistic velocity addition: `relativistic_velocity_addition`

The velocity u = (u' + v)/(1 + u'v/c^2) of an object moving at u' in a frame that moves at v (signed, along x).

$$
u = \frac{u' + v}{1 + u'v/c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u_prime` | in | velocity of the object in the moving frame (signed; may be a fraction of c) | m/s |
| `v` | in | velocity of the moving frame (signed; may be a fraction of c) | m/s |
| `u` | out | velocity of the object in the rest frame | m/s |
| `beta` | out | u/c | - |

**Applies when:** |u'| <= c; |v| < c.

```shell
$ quantum-physics solve relativistic_velocity_addition "u_prime=149896229 m/s" "v=149896229 m/s"
u = 239833966.4 m/s
beta = 0.8
  by: Relativistic velocity addition  (u = (u' + v)/(1 + u' v/c^2))
  u_prime = 149896229 m/s
  v = 149896229 m/s
  assumes: |u'| <= c; |v| < c
```

**What the worked example means:** This calculation determines the velocity of an object as seen by a stationary observer. It combines the object's speed within a moving reference frame, recorded here as 1.49896e+08 m/s, with the speed of that frame itself, which is 1.49896e+08 m/s. The result, 2.39834e+08 m/s, represents the final velocity along the axis of motion. This method ensures that the combined speed never exceeds the universal limit for light speed, adhering to the principles of special relativity rather than simple addition.

<a id="rest_energy"></a>

#### Rest energy (mass-energy equivalence): `rest_energy`

The rest energy E = m c^2 of a mass m, in J, MeV and kWh.

$$
E_0 = m c^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | rest mass (kg, g ...) | kg |
| `E` | out | rest energy in J | J |
| `E_MeV` | out | rest energy in MeV | MeV |
| `E_kWh` | out | rest energy in kWh | - |

**Applies when:** m > 0.

```shell
$ quantum-physics solve rest_energy "m=9.109383714e-31 kg"
E = 8.187105788e-14 J
E_MeV = 0.5109989507 MeV
E_kWh = 2.274196052e-20
  by: Rest energy (mass-energy equivalence)  (E = m c^2 (c = 299792458 m/s exactly))
  m = 9.109383714e-31 kg
  assumes: m > 0
```

**What the worked example means:** This calculation determines the intrinsic rest energy associated with a specific mass. By applying the principle of mass-energy equivalence, the input mass 9.10938e-31 kg yields a total energy of 8.18711e-14 J. This result represents the theoretical energy contained within the matter itself when it is stationary relative to the observer.

<a id="spacetime_interval"></a>

#### Spacetime interval between two events: `spacetime_interval`

The invariant interval s^2 = (c dt)^2 - dx^2 between two events, its type, and the proper time or proper distance.

$$
s^2 = (c\,\Delta t)^2 - \Delta x^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dt` | in | time between the events (signed) | s |
| `dx` | in | distance between the events along x (signed) | m |
| `s2` | out | interval s^2 = (c dt)^2 - dx^2 | m^2 |
| `nature` | out | timelike, spacelike or lightlike | - |
| `tau` | out | proper time (timelike; else 0) | s |
| `proper_length` | out | proper distance (spacelike; else 0) | m |

**Applies when:** the events are distinct.

```shell
$ quantum-physics solve spacetime_interval "dt=5 s" "dx=899377374 m"
s2 = 1.438008286e+18 m^2
nature = timelike
tau = 4 s
proper_length = 0 m
  by: Spacetime interval between two events  (s^2 = (c dt - dx)(c dt + dx); tau = sqrt(s^2)/c or L = sqrt(-s^2))
  dt = 5 s
  dx = 899377374 m
  assumes: the events are distinct
```

**What the worked example means:** This calculation determines the invariant distance between two distinct events in flat spacetime using their time difference 5 s and spatial separation 8.99377e+08 m. The resulting interval 1.43801e+18 m^2 classifies the relationship as timelike, which dictates whether causal influence is possible. If the interval is timelike, the proper time experienced by an observer moving between these events is 4 s seconds; if it is spacelike, the proper distance measured in the frame where the events are simultaneous is 0 m meters.

<a id="relativistic_speed_from_kinetic_energy"></a>

#### Speed from relativistic kinetic energy: `relativistic_speed_from_kinetic_energy`

The speed, Lorentz factor and momentum of a particle of mass m with kinetic energy K.

$$
\gamma = 1 + \frac{K}{mc^2},\quad v = c\sqrt{1 - 1/\gamma^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | rest mass | kg |
| `K` | in | kinetic energy (J, eV, MeV ...) | J |
| `v` | out | speed | m/s |
| `beta` | out | v/c | - |
| `gamma` | out | Lorentz factor | - |
| `p` | out | momentum | kg*m/s |
| `pc_MeV` | out | momentum times c in MeV | MeV |

**Applies when:** m > 0; K >= 0.

```shell
$ quantum-physics solve relativistic_speed_from_kinetic_energy "m=1 kg" "K=2.246887947e+16 J"
v = 179875474.8 m/s
beta = 0.6
gamma = 1.25
p = 224844343.5 kg*m/s
pc_MeV = 4.207191453e+29 MeV
  by: Speed from relativistic kinetic energy  (gamma = 1 + K/(m c^2), pc = sqrt(K (K + 2 m c^2)), beta = pc/(K + m c^2))
  m = 1 kg
  K = 2.246887947e+16 J
  assumes: m > 0; K >= 0
```

**What the worked example means:** This calculation determines the kinematic properties of a massive particle from its rest mass and kinetic energy. By applying relativistic mechanics, the tool computes that for a particle with mass 1 kg and kinetic energy 2.24689e+16 J, the resulting speed is 1.79875e+08 m/s, which equals 0.6 times the speed of light. The associated momentum is 2.24844e+08 kg*m/s, and the Lorentz factor governing time dilation and length contraction is 1.25. These values allow practitioners to interpret detector signals in terms of particle velocity and energy.

<a id="time_dilation"></a>

#### Time dilation: `time_dilation`

The time t = gamma t0 between two ticks of a clock that moves at speed v, given its proper time t0.

$$
\Delta t = \gamma\,\Delta t_0 = \frac{\Delta t_0}{\sqrt{1 - v^2/c^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t0` | in | proper time (measured by the moving clock) | s |
| `v` | in | speed of the clock (may be written as a fraction of c) | m/s |
| `t` | out | dilated time measured in the frame where the clock moves | s |
| `gamma` | out | Lorentz factor | - |
| `difference` | out | t - t0 | s |

**Applies when:** t0 > 0; 0 <= v < c.

```shell
$ quantum-physics solve time_dilation "t0=1 s" "v=179875474.8 m/s"
t = 1.25 s
gamma = 1.25
difference = 0.25 s
  by: Time dilation  (t = gamma t0)
  t0 = 1 s
  v = 179875474.8 m/s
  assumes: t0 > 0; 0 <= v < c
```

**What the worked example means:** This calculation determines the time interval measured by a stationary observer for a clock that moves at a constant speed. Given a proper time of 1 s and a velocity of 1.79875e+08 m/s, the Lorentz factor is computed as 1.25, resulting in an observed time of 1.25 s. The difference between the observed and proper times is 0.25 s, showing how much slower the moving clock appears to tick relative to the stationary frame.

### Spin and qubits (8)

<a id="chsh_bell_test"></a>

#### CHSH Bell-test value for the singlet state, `chsh_bell_test`

Correlations E(a, b) = -cos(a - b) of spin measurements on a singlet pair and the CHSH combination S = E(a,b) - E(a,b') + E(a',b) + E(a',b').

$$
S = E(a,b) - E(a,b') + E(a',b) + E(a',b'),\quad E(x,y) = -\cos(x - y)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | Alice's first setting (angle in the x-z plane) | rad |
| `a2` | in | Alice's second setting a' | rad |
| `b` | in | Bob's first setting | rad |
| `b2` | in | Bob's second setting b' | rad |
| `S` | out | CHSH value | - |
| `abs_S` | out | \|S\| (local hidden variables allow at most 2) | - |
| `E_ab` | out | E(a, b) | - |
| `E_ab2` | out | E(a, b') | - |
| `E_a2b` | out | E(a', b) | - |
| `E_a2b2` | out | E(a', b') | - |
| `violates` | out | whether \|S\| exceeds 2 | - |

**Applies when:** the spin singlet state; measurement directions in one plane.

```shell
$ quantum-physics solve chsh_bell_test "a=0 deg" "a2=90 deg" "b=45 deg" "b2=135 deg"
S = -2.828427125
abs_S = 2.828427125
E_ab = -0.7071067812
E_ab2 = 0.7071067812
E_a2b = -0.7071067812
E_a2b2 = -0.7071067812
violates = yes
  by: CHSH Bell-test value for the singlet state  (E(x, y) = -cos(x - y); S = E(a,b) - E(a,b') + E(a',b) + E(a',b'))
  a = 0 rad
  a2 = 1.570796327 rad
  b = 0.7853981634 rad
  b2 = 2.35619449 rad
  assumes: the spin singlet state; measurement directions in one plane
```

**What the worked example means:** This calculation determines the degree of quantum correlation for a singlet state pair based on four measurement angles. The individual correlations are computed as E(a,b) = -0.707107, E(a,b') = 0.707107, E(a',b) = -0.707107, and E(a',b') = -0.707107. These values combine into the CHSH parameter S = -2.82843. If the absolute value 2.82843 exceeds two, it indicates a violation of Bell inequalities, suggesting that no local hidden variable theory can explain the observed correlations.

<a id="qubit_fidelity"></a>

#### Fidelity and trace distance of two qubit states, `qubit_fidelity`

The (Uhlmann) fidelity and the trace distance between two qubit states given by their Bloch vectors r and s.

$$
F = \tfrac12\left[1 + \vec r\cdot\vec s + \sqrt{(1 - |\vec r|^2)(1 - |\vec s|^2)}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | Bloch vector of the first state | - |
| `s` | in | Bloch vector of the second state | - |
| `F` | out | fidelity (Tr sqrt(sqrt(rho) sigma sqrt(rho)))^2 | - |
| `sqrt_F` | out | root fidelity | - |
| `trace_distance` | out | trace distance \|r - s\|/2 | - |

**Applies when:** |r|, |s| <= 1.

```shell
$ quantum-physics solve qubit_fidelity "r=[0, 0, 1]" "s=[1, 0, 0]"
F = 0.5
sqrt_F = 0.7071067812
trace_distance = 0.7071067812
  by: Fidelity and trace distance of two qubit states  (F = [1 + r.s + sqrt((1 - |r|^2)(1 - |s|^2))]/2, D = |r - s|/2)
  assumes: |r|, |s| <= 1
```

**What the worked example means:** This calculation determines the Uhlmann fidelity and trace distance between two single-qubit states defined by their Bloch vectors. The fidelity, computed as 0.5, quantifies the similarity or overlap between the quantum states, while the root fidelity is 0.707107. The trace distance, given by 0.707107, measures how distinguishable the two states are operationally. A high fidelity suggests the states are nearly identical, whereas a large trace distance indicates they are easily distinguished from one another.

<a id="larmor_precession"></a>

#### Larmor precession of a spin-1/2: `larmor_precession`

Larmor angular frequency of an electron or proton spin in a field B, and the spin component along x after time t for a spin starting along +x.

$$
\omega_L = |\gamma| B,\quad P_{+x}(t) = \cos^2\!\left(\tfrac{\omega_L t}{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `particle` | in | the spin-1/2 particle | - |
| `B` | in | magnetic field along z | T |
| `t` | in | time since the spin pointed along +x | s |
| `omega_L` | out | Larmor angular frequency \|gamma\| B | rad/s |
| `f_L` | out | Larmor frequency | Hz |
| `Sx_mean` | out | <sigma_x> at time t (mean spin along x in units of hbar/2) | - |
| `P_plus_x` | out | probability of measuring spin up along +x at time t | - |

**Applies when:** B > 0; t >= 0.

```shell
$ quantum-physics solve larmor_precession particle=proton "B=1 T" "t=0 s"
omega_L = 267522187.1 rad/s
f_L = 42577478.46 Hz
Sx_mean = 1
P_plus_x = 1
  by: Larmor precession of a spin-1/2  (omega = |gamma| B (CODATA gyromagnetic ratios), <sigma_x> = cos(omega t), P(+x) = cos^2(omega t/2))
  B = 1 T
  t = 0 s
  particle = proton
  assumes: B > 0; t >= 0
```

**What the worked example means:** This calculation determines the precession dynamics of a spin-1/2 particle in a static magnetic field. Using the gyromagnetic ratio for the specified particle, it computes the Larmor angular frequency as 2.67522e+08 rad/s and the corresponding linear frequency as 4.25775e+07 Hz. Based on the elapsed time since the spin was aligned along the positive x-axis, it predicts the mean spin component along x to be 1 and the probability of finding the spin up along that axis to be 1. These values describe how the quantum state rotates around the magnetic field direction.

<a id="qubit_entropy_purity"></a>

#### Purity and entropy of a qubit state: `qubit_entropy_purity`

Purity Tr(rho^2), von Neumann entropy (bits) and eigenvalues of the qubit density matrix with Bloch vector r.

$$
\rho = \tfrac12(I + \vec r\cdot\vec\sigma),\quad S = -\sum_i \lambda_i\log_2\lambda_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | Bloch vector (rx, ry, rz), length at most 1 | - |
| `length` | out | \|r\| | - |
| `purity` | out | Tr(rho^2) = (1 + \|r\|^2)/2 | - |
| `S_bits` | out | von Neumann entropy in bits | - |
| `lambda_max` | out | larger eigenvalue (1 + \|r\|)/2 | - |
| `lambda_min` | out | smaller eigenvalue (1 - \|r\|)/2 | - |

**Applies when:** |r| <= 1.

```shell
$ quantum-physics solve qubit_entropy_purity "r=[0.6, 0, 0]"
length = 0.6
purity = 0.68
S_bits = 0.7219280949
lambda_max = 0.8
lambda_min = 0.2
  by: Purity and entropy of a qubit state  (eigenvalues (1 +- |r|)/2, S = -sum lambda log2 lambda)
  assumes: |r| <= 1
```

**What the worked example means:** This calculation determines the mixedness of a quantum state defined by its Bloch vector. The length of the input vector gives the eigenvalues, and from them the purity and entropy. The resulting purity is 0.68, while the entropy measures uncertainty in bits at 0.721928. These values indicate how close the state is to being pure or maximally mixed.

<a id="qubit_state_bloch"></a>

#### Qubit state on the Bloch sphere: `qubit_state_bloch`

Normalised measurement probabilities, Bloch vector, and Bloch angles of the qubit state alpha|0> + beta|1> (amplitudes need not be normalised).

$$
|\psi\rangle = \cos\tfrac{\theta}{2}|0\rangle + e^{i\phi}\sin\tfrac{\theta}{2}|1\rangle
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a_re` | in | real part of alpha | - |
| `a_im` | in | imaginary part of alpha | - |
| `b_re` | in | real part of beta | - |
| `b_im` | in | imaginary part of beta | - |
| `p0` | out | probability of 0 | - |
| `p1` | out | probability of 1 | - |
| `x` | out | Bloch x = <sigma_x> | - |
| `y` | out | Bloch y = <sigma_y> | - |
| `z` | out | Bloch z = <sigma_z> | - |
| `theta` | out | polar angle (rad) | rad |
| `phi` | out | azimuth in [0, 2 pi) (0 when one amplitude is zero) | rad |
| `norm` | out | norm of the given amplitudes | - |

**Applies when:** not both amplitudes zero.

```shell
$ quantum-physics solve qubit_state_bloch a_re=0.6 a_im=0 b_re=0 b_im=0.8
p0 = 0.36
p1 = 0.64
x = 0
y = 0.96
z = -0.28
theta = 1.854590436 rad
phi = 1.570796327 rad
norm = 1
  by: Qubit state on the Bloch sphere  (normalise; x + iy = 2 conj(alpha) beta, z = |alpha|^2 - |beta|^2, theta = 2 atan2(|beta|, |alpha|), phi = arg(beta) - arg(alpha))
  a_re = 0.6
  a_im = 0
  b_re = 0
  b_im = 0.8
  assumes: not both amplitudes zero
```

**What the worked example means:** This calculation determines the geometric representation of a qubit on the Bloch sphere. Starting from the complex amplitudes alpha and beta, the system first computes the normalization factor to ensure valid quantum probabilities. The resulting measurement probability for the zero state is 0.36 and for the one state is 0.64. These values correspond to the Cartesian coordinates of the Bloch vector, where the x component is 0, the y component is 0.96, and the z component is -0.28. The polar angle theta is calculated as 1.85459 rad radians and the azimuthal angle phi as 1.5708 rad radians, defining the specific orientation of the state vector.

<a id="rabi_oscillation"></a>

#### Rabi oscillation of a driven two-level system, `rabi_oscillation`

The probability of finding a two-level system (qubit) excited after driving it for time t with Rabi frequency Omega and detuning Delta.

$$
P_e(t) = \frac{\Omega^2}{\Omega^2 + \Delta^2}\sin^2\!\left(\frac{\sqrt{\Omega^2 + \Delta^2}\,t}{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Omega` | in | Rabi angular frequency | rad/s |
| `Delta` | in | detuning (drive minus transition angular frequency) | rad/s |
| `t` | in | driving time | s |
| `P_excited` | out | excited-state probability | - |
| `W` | out | generalised Rabi frequency sqrt(Omega^2 + Delta^2) | rad/s |
| `P_max` | out | largest excitation probability Omega^2/W^2 | - |
| `period` | out | oscillation period 2 pi/W | s |

**Applies when:** Omega > 0; t >= 0; rotating-wave approximation, starting in the ground state.

```shell
$ quantum-physics solve rabi_oscillation "Omega=1000000 rad/s" "Delta=0 rad/s" "t=1.570796327 us"
P_excited = 0.5000000001
W = 1000000 rad/s
P_max = 1
period = 6.283185307e-06 s
  by: Rabi oscillation of a driven two-level system  (P = (Omega^2/W^2) sin^2(W t/2), W = sqrt(Omega^2 + Delta^2))
  Omega = 1000000 rad/s
  Delta = 0 rad/s
  t = 1.570796327e-06 s
  assumes: Omega > 0; t >= 0; rotating-wave approximation, starting in the ground state
```

**What the worked example means:** This calculation determines the probability that a two-level system is in its excited state after being driven for time 1.5708 us. The result, 0.5, depends on the Rabi frequency 1e+06 rad/s and detuning 0 rad/s. The oscillation occurs at the generalized Rabi frequency 1e+06 rad/s, reaching a peak probability of 1 every 6.28319e-06 s seconds.

<a id="spin_rotation_wigner_d"></a>

#### Spin rotation amplitude (Wigner small-d matrix), `spin_rotation_wigner_d`

The amplitude d^j_{m'm}(beta) = <j m'| exp(-i beta J_y/hbar) |j m> and the probability of finding m' after rotating |j m> by beta about y.

$$
d^j_{m'm}(\beta) = \langle j m'|e^{-i\beta J_y/\hbar}|j m\rangle
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `j` | in | angular momentum quantum number (whole or half-whole, at most 20) | - |
| `mp` | in | final projection m' | - |
| `m` | in | initial projection m | - |
| `beta` | in | rotation angle about the y axis (deg or rad) | rad |
| `d` | out | Wigner small-d matrix element | - |
| `probability` | out | probability \|d\|^2 of measuring m' along the original axis | - |

**Applies when:** |m|, |m'| <= j; j <= 20.

```shell
$ quantum-physics solve spin_rotation_wigner_d j=0.5 mp=0.5 m=0.5 "beta=90 deg"
d = 0.7071067812
probability = 0.5
  by: Spin rotation amplitude (Wigner small-d matrix)  (Wigner's formula summed in 40-digit arithmetic)
  j = 0.5
  mp = 0.5
  m = 0.5
  beta = 1.570796327 rad
  assumes: |m|, |m'| <= j; j <= 20
```

**What the worked example means:** This calculation determines the quantum amplitude for a spin state with initial projection 0.5 to transform into final projection 0.5 after a rotation of angle 90 deg about the y-axis. The resulting Wigner small-d matrix element, denoted as 0.707107, represents this complex amplitude. Squaring its magnitude yields the probability 0.5, which indicates the likelihood of measuring the system in the new projection state along the original quantization axis.

<a id="spin_half_measurement"></a>

#### Spin-1/2 measurement along an axis: `spin_half_measurement`

Probabilities of spin up and down along a direction n (angles theta_n, phi_n) for a spin-1/2 state pointing along (theta, phi) on the Bloch sphere.

$$
P_\uparrow = |\langle +_n|\psi\rangle|^2 = \tfrac12\left(1 + \hat r\cdot\hat n\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta` | in | polar angle of the state's Bloch vector | rad |
| `phi` | in | azimuth of the state's Bloch vector | rad |
| `theta_n` | in | polar angle of the measurement axis | rad |
| `phi_n` | in | azimuth of the measurement axis | rad |
| `P_up` | out | probability of +hbar/2 along n | - |
| `P_down` | out | probability of -hbar/2 along n | - |
| `expectation` | out | <sigma_n> (the mean spin along n in units of hbar/2) | - |

**Applies when:** polar angles between 0 and 180 degrees.

```shell
$ quantum-physics solve spin_half_measurement "theta=60 deg" "phi=0 deg" "theta_n=0 deg" "phi_n=0 deg"
P_up = 0.75
P_down = 0.25
expectation = 0.5
  by: Spin-1/2 measurement along an axis  (P_up = (1 + r . n)/2 with r and n the unit vectors of the state and the axis)
  theta = 1.047197551 rad
  phi = 0 rad
  theta_n = 0 rad
  phi_n = 0 rad
  assumes: polar angles between 0 and 180 degrees
```

**What the worked example means:** This calculation determines the likelihood of observing a spin up result when measuring a quantum state along a specific direction. Given the state's polar angle 60 deg and azimuth 0 deg, and the measurement axis defined by 0 deg and 0 deg, the resulting probability for spin up is 0.75. The corresponding probability for spin down is 0.25, and the expectation value representing the average spin component in units of hbar over two is 0.5.

### Statistical physics (15)

<a id="bec_critical_temperature"></a>

#### Bose-Einstein condensation temperature: `bec_critical_temperature`

The critical temperature below which an ideal Bose gas of density n and particle mass m forms a condensate.

$$
T_c = \frac{2\pi\hbar^2}{m k_B}\left(\frac{n}{\zeta(3/2)}\right)^{2/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | particle density | 1/m^3 |
| `m` | in | particle mass | kg |
| `T_c` | out | critical temperature | K |
| `lambda_c` | out | thermal wavelength at T_c | m |
| `n_lambda3` | out | phase-space density at T_c, zeta(3/2) | - |

**Applies when:** n > 0; m > 0; ideal gas, spin 0, uniform box.

```shell
$ quantum-physics solve bec_critical_temperature "n=1e+20 m^-3" "m=1.443e-25 kg"
T_c = 3.983751061e-07 K
lambda_c = 2.967188877e-07 m
n_lambda3 = 2.612375349
  by: Bose-Einstein condensation temperature  (T_c = (2 pi hbar^2/(m k)) (n/zeta(3/2))^(2/3))
  n = 1e+20 1/m^3
  m = 1.443e-25 kg
  assumes: n > 0; m > 0; ideal gas, spin 0, uniform box
```

**What the worked example means:** This calculation determines the Bose-Einstein condensation temperature based on particle density and mass. Given a density of 1e+20 m^-3 and a mass of 1.443e-25 kg, the resulting critical temperature is 3.98375e-07 K. Below this threshold, the gas transitions into a condensate state where quantum effects become macroscopic.

<a id="bose_einstein_occupation"></a>

#### Bose-Einstein occupation of a mode: `bose_einstein_occupation`

The mean number of bosons in a single-particle state of energy E at temperature T and chemical potential mu (mu < E).

$$
\bar n(E) = \frac{1}{e^{(E - \mu)/k_BT} - 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | energy of the state | J |
| `mu` | in | chemical potential (0 for photons and phonons) | J |
| `T` | in | temperature | K |
| `n` | out | mean occupation 1/(e^x - 1) | - |
| `x` | out | (E - mu)/kT | - |
| `classical_limit` | out | Maxwell-Boltzmann value e^(-x) | - |

**Applies when:** T > 0; E > mu.

```shell
$ quantum-physics solve bose_einstein_occupation E=2.870978885078724e-21 mu=0.0 T=300.0
n = 1
x = 0.6931471806
classical_limit = 0.5
  by: Bose-Einstein occupation of a mode  (n = 1/(exp((E - mu)/kT) - 1) (expm1 for small x))
  E = 2.870978885e-21 J
  mu = 0 J
  T = 300 K
  assumes: T > 0; E > mu
```

**What the worked example means:** This calculation determines the average number of bosons occupying a single-particle state with energy 2.87098e-21 J at a given temperature 300 K and chemical potential 0 J. The result, 1, represents the Bose-Einstein distribution value. The dimensionless exponent x is calculated as 0.693147, which compares the energy difference to thermal energy. When x is large, the occupation approaches the classical Maxwell-Boltzmann limit of 0.5. This framework applies to systems like photons or phonons where particle number is not conserved, often setting the chemical potential to zero.

<a id="fermi_gas_chemical_potential"></a>

#### Chemical potential of an ideal Fermi gas: `fermi_gas_chemical_potential`

The chemical potential of an ideal spin-1/2 Fermi gas of density n and particle mass m at temperature T, from degenerate to classical.

$$
n = \frac{2}{\lambda_{th}^3}\,f_{3/2}\!\left(e^{\mu/k_BT}\right),\quad f_{3/2}(z) = -\mathrm{Li}_{3/2}(-z)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | particle density | 1/m^3 |
| `T` | in | temperature | K |
| `m` | in | particle mass | kg |
| `mu_eV` | out | chemical potential | eV |
| `eta` | out | mu/kT | - |
| `E_F_eV` | out | Fermi energy at T = 0 | eV |
| `mu_over_EF` | out | mu/E_F | - |
| `T_over_TF` | out | T/T_F | - |

**Applies when:** n, T, m > 0; T >= 0.02 T_F; |mu/kT| <= 60.

```shell
$ quantum-physics solve fermi_gas_chemical_potential n=1e+28 T=100000.0 m=9.1093837139e-31
mu_eV = -23.29160225 eV
eta = -2.702878204
E_F_eV = 1.692532188 eV
mu_over_EF = -13.76139397
T_over_TF = 5.09138516
  by: Chemical potential of an ideal Fermi gas  (n lambda_th^3 / 2 = f_{3/2}(e^(mu/kT)) solved by safeguarded Newton; the Fermi-Dirac integral by the trapezoidal rule in x = t^2)
  n = 1e+28 1/m^3
  T = 100000 K
  m = 9.109383714e-31 kg
  assumes: n, T, m > 0; T >= 0.02 T_F; |mu/kT| <= 60
```

**What the worked example means:** This calculation determines the chemical potential for an ideal spin-1/2 Fermi gas with particle density 1e+28 1/m^3 and mass 9.10938e-31 kg at temperature 100000 K. The result, -23.2916 eV, reflects the energy required to add a particle to the system. This value corresponds to a dimensionless ratio of -2.70288, indicating the balance between quantum degeneracy and thermal effects relative to the Fermi energy.

<a id="debye_heat_capacity"></a>

#### Debye heat capacity of a solid: `debye_heat_capacity`

The molar heat capacity of a crystalline solid in the Debye model at temperature T.

$$
C_V = 9R\left(\frac{T}{\Theta_D}\right)^3\int_0^{\Theta_D/T}\frac{x^4 e^x}{(e^x - 1)^2}\,dx
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `theta_D` | in | Debye temperature | K |
| `C_molar` | out | molar heat capacity C_V | J/(mol*K) |
| `C_over_3R` | out | C_V / 3R (tends to 1 at high T) | - |
| `debye_integral` | out | integral_0^(theta/T) x^4 e^x/(e^x - 1)^2 dx | - |

**Applies when:** T > 0; theta_D > 0; theta_D/T <= 600.

```shell
$ quantum-physics solve debye_heat_capacity "T=1 K" "theta_D=500 K"
C_molar = 1.555016152e-05 J/(mol*K)
C_over_3R = 6.234181826e-07
debye_integral = 25.97575761
  by: Debye heat capacity of a solid  (C_V = 9 R (T/theta_D)^3 integral_0^(theta_D/T) x^4 e^x/(e^x - 1)^2 dx (25-digit quadrature))
  T = 1 K
  theta_D = 500 K
  assumes: T > 0; theta_D > 0; theta_D/T <= 600
```

**What the worked example means:** This calculation determines the molar heat capacity of a crystalline solid using the Debye model. By evaluating the integral with temperature 1 K and Debye temperature 500 K, it yields a value of 1.55502e-05 J/(mol*K). The dimensionless ratio 6.23418e-07 indicates how close the solid is to the classical Dulong-Petit limit, while the raw integral value 25.9758 represents the core mathematical component of this physical property.

<a id="photon_gas"></a>

#### Energy and photon density of blackbody radiation, `photon_gas`

Energy density, photon number density, mean photon energy and radiation pressure of thermal radiation at temperature T.

$$
u = \frac{\pi^2 (k_BT)^4}{15(\hbar c)^3},\quad n = \frac{2\zeta(3)}{\pi^2}\left(\frac{k_BT}{\hbar c}\right)^3
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `u` | out | energy density (J/m^3) | Pa |
| `n` | out | photon number density | 1/m^3 |
| `mean_E_eV` | out | mean photon energy | eV |
| `mean_E_over_kT` | out | mean photon energy in units of kT (2.701...) | - |
| `pressure` | out | radiation pressure u/3 | Pa |

**Applies when:** T > 0.

```shell
$ quantum-physics solve photon_gas "T=2.725 K"
u = 4.171738251e-14 Pa
n = 410500842.7 1/m^3
mean_E_eV = 0.0006342969232 eV
mean_E_over_kT = 2.701178033
pressure = 1.390579417e-14 Pa
  by: Energy and photon density of blackbody radiation  (u = (pi^2/15)(kT)^4/(hbar c)^3, n = (2 zeta(3)/pi^2)(kT/(hbar c))^3)
  T = 2.725 K
  assumes: T > 0
```

**What the worked example means:** This calculation determines the thermodynamic properties of thermal radiation in equilibrium. Given a temperature of 2.725 K, it computes the total energy stored per unit volume, which is 4.17174e-14 Pa, and the number of photons present, which is 4.10501e+08 1/m^3. The mean energy of these individual photons is calculated as 0.000634297 eV, representing 2.70118 times the thermal energy scale kT. Consequently, the radiation exerts a pressure on its container equal to one-third of the energy density, resulting in 1.39058e-14 Pa.

<a id="fermi_energy_free_electrons"></a>

#### Fermi energy of a free-electron gas: `fermi_energy_free_electrons`

Fermi energy, temperature, wavenumber and speed of a free-electron gas with electron density n.

$$
E_F = \frac{\hbar^2}{2m_e}\left(3\pi^2 n\right)^{2/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | conduction-electron density (per m^3) | 1/m^3 |
| `E_F_eV` | out | Fermi energy | eV |
| `E_F` | out | Fermi energy in J | J |
| `T_F` | out | Fermi temperature E_F/k | K |
| `k_F` | out | Fermi wavenumber (3 pi^2 n)^(1/3) | 1/m |
| `v_F` | out | Fermi speed | m/s |

**Applies when:** n > 0; free electrons with spin 1/2, T = 0.

```shell
$ quantum-physics solve fermi_energy_free_electrons "n=8.47e+28 m^-3"
E_F_eV = 7.032761294 eV
E_F = 1.126772582e-18 J
T_F = 81611.80588 K
k_F = 1.358630845e+10 1/m
v_F = 1572854.811 m/s
  by: Fermi energy of a free-electron gas  (k_F = (3 pi^2 n)^(1/3), E_F = hbar^2 k_F^2/(2 m_e))
  n = 8.47e+28 1/m^3
  assumes: n > 0; free electrons with spin 1/2, T = 0
```

**What the worked example means:** This calculation determines the characteristic quantum energy scale for a free-electron gas based on its density. Given an electron density of 8.47e+28 m^-3, the resulting Fermi energy is 7.03276 eV, which translates to 1.12677e-18 J in joules. This energy level implies a Fermi temperature of 81611.8 K, indicating the thermal energy required to significantly alter the electron distribution. The associated Fermi wavenumber is 1.35863e+10 1/m, and the electrons move at a Fermi speed of 1.57285e+06 m/s. These values define the boundary between occupied and unoccupied states at absolute zero.

<a id="fermi_energy_metal"></a>

#### Fermi energy of a metal from its density: `fermi_energy_metal`

Conduction-electron density and Fermi energy of a metal from its mass density, molar mass and valence (free-electron model).

$$
n = \frac{z\rho N_A}{M},\quad E_F = \frac{\hbar^2}{2m_e}\left(3\pi^2 n\right)^{2/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | mass density of the metal | kg/m^3 |
| `M` | in | molar mass (g/mol or kg/mol) | kg/mol |
| `z` | in | conduction electrons per atom (valence) | - |
| `n` | out | conduction-electron density | 1/m^3 |
| `E_F_eV` | out | Fermi energy | eV |
| `T_F` | out | Fermi temperature | K |
| `v_F` | out | Fermi speed | m/s |

**Applies when:** rho, M > 0; z >= 1.

```shell
$ quantum-physics solve fermi_energy_metal "rho=8960 kg/m^3" "M=63.546 g/mol" z=1
n = 8.491231739e+28 1/m^3
E_F_eV = 7.044509066 eV
T_F = 81748.13311 K
v_F = 1574167.939 m/s
  by: Fermi energy of a metal from its density  (n = z rho N_A / M, E_F = hbar^2 (3 pi^2 n)^(2/3)/(2 m_e))
  rho = 8960 kg/m^3
  M = 0.063546 kg/mol
  assumes: rho, M > 0; z >= 1
```

**What the worked example means:** This calculation determines the electronic properties of a metal using its mass density, molar mass, and valence. By applying the free-electron model, it computes the number of conduction electrons per unit volume, resulting in 8.49123e+28 1/m^3. From this density, the Fermi energy is derived as 7.04451 eV, which represents the highest kinetic energy occupied by electrons at absolute zero. This value helps estimate the thermal and electrical behavior of the material without needing microscopic measurements.

<a id="fermi_dirac_occupation"></a>

#### Fermi-Dirac occupation of a state: `fermi_dirac_occupation`

The probability that a single-particle state of energy E is occupied by a fermion at temperature T and chemical potential mu.

$$
f(E) = \frac{1}{e^{(E - \mu)/k_BT} + 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | energy of the state | J |
| `mu` | in | chemical potential (Fermi level) | J |
| `T` | in | temperature | K |
| `f` | out | occupation probability 1/(e^x + 1) | - |
| `one_minus_f` | out | probability the state is empty | - |
| `x` | out | (E - mu)/kT | - |

**Applies when:** T > 0.

```shell
$ quantum-physics solve fermi_dirac_occupation "E=5 eV" "mu=5 eV" "T=300 K"
f = 0.5
one_minus_f = 0.5
x = 0
  by: Fermi-Dirac occupation of a state  (f = 1/(exp((E - mu)/kT) + 1), evaluated without overflow)
  E = 8.01088317e-19 J
  mu = 8.01088317e-19 J
  T = 300 K
  assumes: T > 0
```

**What the worked example means:** This calculation determines the likelihood that a specific quantum state is occupied by a fermion. Using the input energy of 5 eV and temperature of 300 K, the system computes an intermediate exponent value of 0. The resulting occupation probability is 0.5, while the chance that the state remains empty is 0.5. These values describe how electrons or other fermions distribute themselves across available energy levels under thermal equilibrium conditions.

<a id="blackbody_band_fraction"></a>

#### Fraction of blackbody radiation in a wavelength band, `blackbody_band_fraction`

The fraction of a blackbody's emitted power between two wavelengths at temperature T, and that band's radiant exitance.

$$
F_{0\to\lambda} = \frac{15}{\pi^4}\int_{hc/\lambda k_BT}^{\infty}\frac{t^3}{e^t - 1}\,dt
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam1` | in | shorter wavelength | m |
| `lam2` | in | longer wavelength | m |
| `T` | in | temperature | K |
| `F_band` | out | fraction of the total emission between lambda1 and lambda2 | - |
| `F_below_lam1` | out | fraction emitted below lambda1 | - |
| `F_below_lam2` | out | fraction emitted below lambda2 | - |
| `M_band` | out | radiant exitance in the band (blackbody) | W/m^2 |

**Applies when:** 0 < lambda1 < lambda2; T > 0.

```shell
$ quantum-physics solve blackbody_band_fraction lam1=1e-12 lam2=1.0 T=5778.0
F_band = 1
F_below_lam1 = 0
F_below_lam2 = 1
M_band = 63200699.74 W/m^2
  by: Fraction of blackbody radiation in a wavelength band  (F(0 -> lambda) = (15/pi^4) integral_x^inf t^3/(e^t - 1) dt, x = hc/(lambda k T), by exponential or Bernoulli series)
  lam1 = 1e-12 m
  lam2 = 1 m
  T = 5778 K
  assumes: 0 < lambda1 < lambda2; T > 0
```

**What the worked example means:** This calculation determines the portion of a blackbody's power radiated within a specific wavelength band defined by the shorter limit 1e-12 m and the longer limit 1 m at temperature 5778 K. The result, expressed as the dimensionless fraction 1, indicates how much of the total energy falls inside this window. This value is multiplied by the Stefan-Boltzmann law to find the actual radiant exitance in that band, which is reported as 6.32007e+07 W/m^2. Additionally, the cumulative fractions emitted below each boundary are provided as 0 and 1 to help visualize the distribution.

<a id="wien_peak"></a>

#### Peak of blackbody radiation (Wien's displacement law), `wien_peak`

The wavelength and frequency at which a blackbody at temperature T radiates most strongly.

$$
\lambda_{max} T = b = \frac{hc}{4.965\,k_B}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature of the blackbody | K |
| `lambda_max` | out | peak wavelength (of B_lambda) | m |
| `lambda_max_nm` | out | peak wavelength in nm | nm |
| `nu_max` | out | peak frequency (of B_nu; not c/lambda_max) | Hz |
| `b` | out | Wien's constant b = hc/(x k) | m*K |

**Applies when:** T > 0.

```shell
$ quantum-physics solve wien_peak "T=5778 K"
lambda_max = 5.015181646e-07 m
lambda_max_nm = 501.5181646 nm
nu_max = 3.396843303e+14 Hz
b = 0.002897771955 m*K
  by: Peak of blackbody radiation (Wien's displacement law)  (lambda_max = h c/(x k T) with x = 5(1 - e^-x) = 4.9651...; nu_max = x' k T/h with x' = 3(1 - e^-x') = 2.8214...)
  T = 5778 K
  assumes: T > 0
```

In the worked example, T = 5778 K go through the equation above and give lambda_max = 5.015181646e-07 m, lambda_max_nm = 501.5181646 nm, nu_max = 3.396843303e+14 Hz, b = 0.002897771955 m*K. It applies only when T > 0: outside that it says what is wrong instead of returning a number.

<a id="einstein_oscillator"></a>

#### Quantum oscillator thermodynamics (Einstein model), `einstein_oscillator`

Mean quantum number, mean energy, heat capacity and entropy of a quantum harmonic oscillator, and the molar heat capacity of an Einstein solid.

$$
C_V = 3R\left(\frac{\Theta_E}{T}\right)^2\frac{e^{\Theta_E/T}}{\left(e^{\Theta_E/T} - 1\right)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta_E` | in | Einstein temperature hbar omega/k | K |
| `T` | in | temperature | K |
| `n_mean` | out | mean number of quanta 1/(e^x - 1) | - |
| `U_eV` | out | mean energy including the zero-point energy | eV |
| `C_over_k` | out | heat capacity per oscillator in units of k | - |
| `C_molar` | out | molar heat capacity of an Einstein solid 3R C/k | J/(mol*K) |
| `S_over_k` | out | entropy per oscillator in units of k | - |

**Applies when:** theta_E > 0; T > 0.

```shell
$ quantum-physics solve einstein_oscillator "theta_E=300 K" "T=300 K"
n_mean = 0.5819767069
U_eV = 0.02797126159 eV
C_over_k = 0.9206735942
C_molar = 22.96471855 J/(mol*K)
S_over_k = 1.040651852
  by: Quantum oscillator thermodynamics (Einstein model)  (x = theta_E/T; n = 1/(e^x - 1); C/k = x^2 e^x/(e^x - 1)^2; C_molar = 3R C/k)
  theta_E = 300 K
  T = 300 K
  assumes: theta_E > 0; T > 0
```

**What the worked example means:** This calculation determines the thermodynamic properties of a quantum harmonic oscillator using the Einstein model for solids. Given an Einstein temperature of 300 K and a system temperature of 300 K, it computes the mean number of quanta, which is 0.581977, and the mean energy including zero-point contributions, found to be 0.0279713 eV. The heat capacity per oscillator in units of Boltzmann's constant is 0.920674, leading to a molar heat capacity of 22.9647 J/(mol*K). Additionally, the entropy per oscillator in units of Boltzmann's constant is calculated as 1.04065. These values describe how the solid stores thermal energy at the specified conditions.

<a id="stefan_boltzmann"></a>

#### Radiated power (Stefan-Boltzmann law): `stefan_boltzmann`

Radiant exitance and total power emitted by a surface of area A, emissivity e at temperature T.

$$
P = \varepsilon\sigma A T^4,\quad \sigma = \frac{2\pi^5 k_B^4}{15 h^3 c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | surface temperature | K |
| `emissivity` | in | emissivity (1 for a blackbody) | - |
| `A` | in | radiating area | m^2 |
| `sigma` | out | Stefan-Boltzmann constant 2 pi^5 k^4/(15 h^3 c^2) | W/(m^2*K^4) |
| `M` | out | radiant exitance e sigma T^4 | W/m^2 |
| `P` | out | total radiated power | W |

**Applies when:** T > 0; A > 0; 0 < emissivity <= 1.

```shell
$ quantum-physics solve stefan_boltzmann "T=1000 K" emissivity=1 "A=1 m^2"
sigma = 5.670374419e-08 W/(m^2*K^4)
M = 56703.74419 W/m^2
P = 56703.74419 W
  by: Radiated power (Stefan-Boltzmann law)  (P = e sigma A T^4, sigma = 2 pi^5 k^4/(15 h^3 c^2))
  T = 1000 K
  emissivity = 1
  A = 1 m^2
  assumes: T > 0; A > 0; 0 < emissivity <= 1
```

**What the worked example means:** This calculation determines the total power emitted by a surface based on its temperature, area, and emissivity. Using the Stefan-Boltzmann law, the radiant exitance is found to be 56703.7 W/m^2, which represents the power emitted per unit area. Multiplying this exitance by the given area yields the total radiated power of 56703.7 W. The underlying physical constant used in these computations is the Stefan-Boltzmann constant, evaluated here as 5.67037e-08 W/(m^2*K^4). These values describe how much energy the object loses via thermal radiation under the specified conditions.

<a id="planck_spectral_radiance"></a>

#### Spectral radiance of a blackbody (Planck's law), `planck_spectral_radiance`

The spectral radiance B_lambda of a blackbody at temperature T and wavelength lambda, and the photon occupation number of that mode.

$$
B_\lambda(T) = \frac{2hc^2}{\lambda^5}\,\frac{1}{e^{hc/(\lambda k_B T)} - 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength | m |
| `T` | in | temperature | K |
| `B_lambda` | out | spectral radiance (W per m^2 per sr per m of wavelength) | W/m^3 |
| `B_lambda_per_nm` | out | spectral radiance in W/(m^2 sr nm) | - |
| `occupation` | out | mean photon number 1/(e^x - 1) | - |
| `x` | out | h c/(lambda k T) | - |

**Applies when:** lambda > 0; T > 0.

```shell
$ quantum-physics solve planck_spectral_radiance "lam=500 nm" "T=5778 K"
B_lambda = 2.637566987e+13 W/m^3
B_lambda_per_nm = 26375.66987
occupation = 0.006920318598
x = 4.980189953
  by: Spectral radiance of a blackbody (Planck's law)  (B_lambda = (2 h c^2/lambda^5) / (exp(h c/(lambda k T)) - 1))
  lam = 5e-07 m
  T = 5778 K
  assumes: lambda > 0; T > 0
```

**What the worked example means:** This calculation determines the spectral radiance of a blackbody, which represents the power emitted per unit area, solid angle, and wavelength interval. Using Planck's law, the result depends on the input wavelength 500 nm and absolute temperature 5778 K. The computed value 2.63757e+13 W/m^3 indicates how intense the radiation is at that specific color or frequency. Additionally, the dimensionless exponent x equals 4.98019, which governs the quantum behavior of the light, while the average photon occupation number for this mode is 0.00692032.

<a id="thermal_de_broglie_wavelength"></a>

#### Thermal de Broglie wavelength: `thermal_de_broglie_wavelength`

The thermal de Broglie wavelength h/sqrt(2 pi m k T) and the quantum concentration 1/lambda^3.

$$
\lambda_{th} = \frac{h}{\sqrt{2\pi m k_B T}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | particle mass | kg |
| `T` | in | temperature | K |
| `lambda_th` | out | thermal de Broglie wavelength | m |
| `lambda_th_nm` | out | the same in nm | nm |
| `n_Q` | out | quantum concentration 1/lambda^3 | 1/m^3 |

**Applies when:** m > 0; T > 0.

```shell
$ quantum-physics solve thermal_de_broglie_wavelength "m=9.109383714e-31 kg" "T=300 K"
lambda_th = 4.303475437e-09 m
lambda_th_nm = 4.303475437 nm
n_Q = 1.254706115e+25 1/m^3
  by: Thermal de Broglie wavelength  (lambda = h/sqrt(2 pi m k T))
  m = 9.109383714e-31 kg
  T = 300 K
  assumes: m > 0; T > 0
```

**What the worked example means:** This calculation determines the thermal de Broglie wavelength, which represents the characteristic quantum size of a particle at a specific temperature. Using the particle mass 9.10938e-31 kg and temperature 300 K, the resulting wavelength is 4.30348e-09 m. This value helps assess whether quantum effects are significant; if this wavelength is comparable to the average distance between particles, classical physics may fail. The associated quantum concentration, calculated as the inverse cube of the wavelength, is 1.25471e+25 1/m^3.

<a id="two_level_thermodynamics"></a>

#### Thermodynamics of a two-level system: `two_level_thermodynamics`

Partition function, Boltzmann populations, mean energy, heat capacity (Schottky) and entropy of a two-level system with gap dE and degeneracies g0, g1.

$$
\frac{N_1}{N_0} = \frac{g_1}{g_0}e^{-\Delta E/k_BT},\quad \frac{C}{k_B} = x^2 p_0 p_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dE` | in | energy gap (eV, J) | J |
| `T` | in | temperature | K |
| `g0` | in | degeneracy of the lower level | - |
| `g1` | in | degeneracy of the upper level | - |
| `Z` | out | partition function g0 + g1 e^(-dE/kT) | - |
| `p0` | out | population of the lower level | - |
| `p1` | out | population of the upper level | - |
| `ratio` | out | population ratio N1/N0 (Boltzmann factor) | - |
| `U_eV` | out | mean energy above the lower level | eV |
| `C_over_k` | out | heat capacity per system in units of k | - |
| `S_over_k` | out | entropy per system in units of k | - |

**Applies when:** dE > 0; T > 0; g0, g1 >= 1.

```shell
$ quantum-physics solve two_level_thermodynamics "dE=0.025 eV" "T=300 K" g0=1 g1=1
Z = 1.380205578
p0 = 0.7245297481
p1 = 0.2754702519
ratio = 0.3802055784
U_eV = 0.006886756298 eV
C_over_k = 0.1866477065
S_over_k = 0.5886240855
  by: Thermodynamics of a two-level system  (Z = g0 + g1 e^(-x), x = dE/kT; p1 = g1 e^(-x)/Z; U = dE p1; C/k = x^2 p0 p1; S/k = ln Z + x p1)
  dE = 4.005441585e-21 J
  T = 300 K
  assumes: dE > 0; T > 0; g0, g1 >= 1
```

**What the worked example means:** This calculation determines the thermodynamic properties of a two-level system defined by an energy gap of 0.025 eV at temperature 300 K. With lower level degeneracy 1 and upper level degeneracy 1, the resulting partition function is 1.38021. The populations are found to be 0.72453 for the ground state and 0.27547 for the excited state, leading to a Boltzmann ratio of 0.380206. Consequently, the average internal energy above the ground state is 0.00688676 eV, while the dimensionless heat capacity and entropy are 0.186648 and 0.588624 respectively.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
