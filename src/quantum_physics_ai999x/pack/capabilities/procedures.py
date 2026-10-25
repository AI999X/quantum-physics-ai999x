"""Quantum Physics procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded by
the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: special relativity, photons and matter waves, the hydrogen atom and angular momentum, quantum wells and
tunnelling, spin and qubits, quantum gates, nuclear and particle physics, statistical physics.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question template it parses,
worked examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are
imported inside the reference functions) and a deliberately wrong variant that the checks must reject. The shipped
code (run, verify, parse) is plain Python (math, cmath, fractions, re) plus mpmath and sympy where stated.

Conventions (fixed, so run and reference agree exactly):
- Constants: CODATA 2022 as in scipy.constants 1.18 (h, c, e, k, N_A exact; m_e = 9.1093837139e-31 kg,
  m_p = 1.67262192595e-27 kg, m_n = 1.67492750056e-27 kg, u = 1.66053906892e-27 kg = 931.49410372 MeV,
  alpha = 0.0072973525643, a0 = 5.29177210544e-11 m, R_inf = 10973731.568157 1/m, mu_B = 9.2740100657e-24 J/T).
- Energies of photons, levels and particles are reported in SI (J) and in eV, keV or MeV; wavelengths in m and nm
  (pm for Compton shifts); speeds of a frame or a particle may be written as a fraction of c ("v = 0.8 c").
- Velocities in relativity are signed along the x axis where the procedure says so; angles of a photon's direction of
  travel are measured from the +x axis (the direction of the frame's velocity).
- Qubit states: |0> is spin up along z. Gates: H, X, Y, Z, S = diag(1, i), Sdg, T = diag(1, e^{i pi/4}), Tdg,
  Rx(t) = exp(-i t X/2), Ry(t), Rz(t) = diag(e^{-it/2}, e^{it/2}), P(t) = diag(1, e^{it}); written gates act left to
  right. Amplitudes are reported with the global phase fixed so that the first non-zero amplitude is real and positive.
  Two-qubit basis states are written |q0 q1> (q0 first); CNOT01 has control q0 and target q1.
- Rotation of a single-qubit gate on the Bloch sphere: angle in [0, pi]; at an angle of pi the axis is chosen with its
  first non-zero component positive; the identity is reported with angle 0 and axis (0, 0, 1).
- Landé g factor with g_s = 2 (the textbook value); Clebsch-Gordan coefficients in the Condon-Shortley convention.
- Rectangular barrier: height V0 above the zero of potential on both sides; step potential: V = 0 for x < 0, V0 for x > 0.
- Finite square well: depth V0, width a, potential 0 inside and V0 outside; energies measured from the bottom.
- Radioactive half-lives of named nuclides: the ICRP-107 values used by radioactivedecay 0.6.1 (in seconds).
  Particle masses (MeV/c^2) and mean lifetimes (ns): the PDG values carried by particle 1.0.1.
- Results built from cos, sin, exp or log are rounded to 12 significant digits (identical on every platform);
  values below 1e-12 of their natural scale that are zero by physics are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Lorentz factor: v = 0.8 c`); physical values carry a unit; vectors are written `p1 = 1, 2, 3` with the pairs then
separated by semicolons; gate sequences follow `gates =` up to the end. Any other number or an unknown name is refused.
"""
from __future__ import annotations

import cmath
import math
import re
from fractions import Fraction

# ================================================================================================ constants (CODATA 2022)
H = 6.62607015e-34
HBAR = H / (2 * math.pi)
C = 299792458.0
QE = 1.602176634e-19
KB = 1.380649e-23
NA = 6.02214076e23
ME = 9.1093837139e-31
MP = 1.67262192595e-27
MN = 1.67492750056e-27
AMU = 1.66053906892e-27
AMU_MEV = 931.49410372
ME_MEV = 0.51099895069
ALPHA = 0.0072973525643
A0 = 5.29177210544e-11
RINF = 10973731.568157
EPS0 = 8.8541878188e-12
MUB = 9.2740100657e-24
RGAS = 8.31446261815324
EV = QE
MEV = 1e6 * QE


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if isinstance(x, complex):
        return complex(_r(x.real), _r(x.imag))
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _snap(x, scale=1.0):
    return 0.0 if abs(x) <= 1e-12 * scale else x


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, str) or isinstance(b, str) or isinstance(a, bool) or isinstance(b, bool):
        return 0.0 if a == b else math.inf
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)], default=0.0)
    try:
        a, b = complex(a), complex(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    s = max(abs(a), abs(b))
    return d / s if s > 0 else math.inf


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_NUMF = r"[-+]?\d+/\d+|" + _NUM                    # a plain number may also be a fraction (j = 3/2)
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUMF + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\S.*?)\s*$")


def _pairs(question, phrase, names, units=(), words=(), vectors=(), speeds=()):
    """{symbol: value} from '<phrase>: name = value, ...'. None unless the phrase opens the question and every part is
    a 'name = value' pair of a known name. `units`: symbols that need a unit (returned as 'value unit' strings, read
    by the runtime); `words`: symbols whose value is a word (a choice); `vectors`: symbols whose value is a list of
    numbers (then the pairs are separated by semicolons); `speeds`: symbols that may be written as a fraction of c."""
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    rest = question[m.end():].strip()
    rest = re.sub(r"[.?!]\s*$", "", rest).strip()
    if not rest:
        return {}
    parts = rest.split(";") if (";" in rest or vectors) else re.split(r"[,;]", rest)
    out = {}
    for seg in parts:
        if not seg.strip():
            continue
        mw = _WORD.match(seg)
        if mw and names.get(mw.group(1).lower()) in set(words) | set(vectors):
            key = names[mw.group(1).lower()]
            if key in out:
                return None
            val = mw.group(2).strip()
            if key in vectors:
                items = [p for p in re.split(r"[,\s]+", val.strip("[]() ")) if p]
                if not items or not all(re.fullmatch(_NUM, p) for p in items):
                    return None
                out[key] = ", ".join(items)
            else:
                out[key] = val
            continue
        ms = _SEG.match(seg)
        if not ms:
            return None
        key = names.get(ms.group(1).lower())
        if key is None or key in out:
            return None
        num, unit = ms.group(2), ms.group(3).strip()
        if "/" in num and (key in units or unit):
            return None                                 # a fraction is read only for a plain number
        if key in speeds and unit.lower() == "c":
            out[key] = f"{float(num) * C!r} m/s"
        elif key in units:
            if not unit:
                return None
            out[key] = f"{num} {unit}"
        else:
            if unit:
                return None
            out[key] = float(Fraction(num))
    return out


def _phys(sym, kind, meaning, required=True):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    PROCEDURES[key] = spec


# ------------------------------------------------------------------------------------------------ boost helpers (refs)
def _ref_boost(phi, n=2):
    """The Lorentz boost of rapidity phi along x as the matrix exponential of its generator (scipy.linalg.expm)."""
    import numpy as np
    from scipy.linalg import expm
    K = np.zeros((n, n))
    K[0, 1] = K[1, 0] = 1.0
    return expm(phi * K)


def _ref_rapidity_for(target_fn, lo=0.0, hi=40.0):
    """Rapidity at which a boosted quantity reaches its target (bisection-safe brentq on the expm boost)."""
    from scipy.optimize import brentq
    return brentq(target_fn, lo, hi, xtol=1e-15, rtol=8.9e-16, maxiter=500)


def _beta(v):
    if not (math.isfinite(v) and abs(v) < C):
        raise ValueError("the speed must be smaller than the speed of light (|v| < c)")
    return v / C


def _gamma_of_beta(b):
    return 1.0 / math.sqrt((1.0 - b) * (1.0 + b))


def _gm1(b):
    """gamma - 1 without cancellation for small beta."""
    s = math.sqrt((1.0 - b) * (1.0 + b))
    return b * b / (s * (1.0 + s))


# ================================================================================================ special relativity
def lorentz_factor(v):
    if v < 0:
        raise ValueError("give the speed as a magnitude (v >= 0)")
    b = _beta(v)
    return _out(gamma=_gamma_of_beta(b), beta=b, gamma_minus_1=_gm1(b))


def ref_lorentz_factor(v):
    L = _ref_boost(math.atanh(v / C))
    return {"gamma": float(L[0, 0]), "beta": float(L[0, 1] / L[0, 0])}


_SR_TPL = "{phrase}: v = 0.8 c"
_add("lorentz_factor",
     name="Lorentz factor", topic="special relativity / Lorentz factor",
     purpose="The Lorentz factor gamma = 1/sqrt(1 - v^2/c^2) of a speed v.",
     inputs=[_phys("v", "speed", "speed (may be written as a fraction of c, e.g. 0.8 c)")],
     outputs=[_o("gamma", "number", "Lorentz factor"), _o("beta", "number", "v/c"),
              _o("gamma_minus_1", "number", "gamma - 1 (computed without cancellation)")],
     primary="gamma", conditions=["0 <= v < c"],
     method="gamma = 1/sqrt((1 - beta)(1 + beta)), beta = v/c",
     latex=r"\gamma = \frac{1}{\sqrt{1 - v^2/c^2}}",
     run=lorentz_factor, verify=_recheck(lorentz_factor),
     parse=lambda q: _pairs(q, r"lorentz factor", {"v": "v"}, units={"v"}, speeds={"v"}),
     reference=ref_lorentz_factor,
     wrong=_safe(lambda v: _out(gamma=1 / (1 - (v / C) ** 2), beta=v / C, gamma_minus_1=1 / (1 - (v / C) ** 2) - 1)),
     random_inputs=lambda rng: {"v": rng.uniform(0.01, 0.999) * C},
     templates=["Lorentz factor: v = 0.8 c", "Lorentz factor: v = 2.0e8 m/s"],
     examples=[{"inputs": {"v": 0.6 * C}, "outputs": {"gamma": 1.25}},
               {"inputs": {"v": 0.8 * C}, "outputs": {"gamma": 5 / 3}},
               {"inputs": {"v": 0.0}, "outputs": {"gamma": 1.0, "gamma_minus_1": 0.0}}],
     questions=[{"q": "Lorentz factor: v = 0.6 c", "outputs": {"gamma": 1.25}},
                {"q": "Lorentz factor: v = 0.8 c", "outputs": {"gamma": 5 / 3}},
                {"q": "Lorentz factor: v = 1.2 c", "outcome": "INVALID_INPUT"},
                {"q": "Lorentz factor: v = 0.5", "outcome": "UNRECOGNIZED_FORMAT"}])


def time_dilation(t0, v):
    if t0 <= 0:
        raise ValueError("the proper time must be positive")
    if v < 0:
        raise ValueError("give the speed as a magnitude (v >= 0)")
    b = _beta(v)
    g = _gamma_of_beta(b)
    return _out(t=g * t0, gamma=g, difference=_gm1(b) * t0)


def ref_time_dilation(t0, v):
    L = _ref_boost(math.atanh(v / C))                    # the clock's tick (ct' = c t0, x' = 0) seen from the lab
    import numpy as np
    ct, x = L @ np.array([C * t0, 0.0])
    return {"t": float(ct / C), "gamma": float(L[0, 0])}


_add("time_dilation",
     name="Time dilation", topic="special relativity / time dilation",
     purpose="The time t = gamma t0 between two ticks of a clock that moves at speed v, given its proper time t0.",
     inputs=[_phys("t0", "time", "proper time (measured by the moving clock)"),
             _phys("v", "speed", "speed of the clock (may be written as a fraction of c)")],
     outputs=[_o("t", "time", "dilated time measured in the frame where the clock moves"), _o("gamma", "number", "Lorentz factor"),
              _o("difference", "time", "t - t0")],
     primary="t", conditions=["t0 > 0", "0 <= v < c"], method="t = gamma t0",
     latex=r"\Delta t = \gamma\,\Delta t_0 = \frac{\Delta t_0}{\sqrt{1 - v^2/c^2}}",
     run=time_dilation, verify=_recheck(time_dilation),
     parse=lambda q: _pairs(q, r"time dilation", {"t0": "t0", "v": "v"}, units={"t0", "v"}, speeds={"v"}),
     reference=ref_time_dilation,
     wrong=_safe(lambda t0, v: _out(t=t0 * math.sqrt(1 - (v / C) ** 2), gamma=_gamma_of_beta(v / C), difference=0.0)),
     random_inputs=lambda rng: {"t0": _lu(rng, 1e-9, 1e3), "v": rng.uniform(0.01, 0.995) * C},
     templates=["Time dilation: t0 = 2.2 us, v = 0.99 c"],
     examples=[{"inputs": {"t0": 1.0, "v": 0.6 * C}, "outputs": {"t": 1.25}},
               {"inputs": {"t0": 2.2e-6, "v": 0.8 * C}, "outputs": {"t": 2.2e-6 * 5 / 3}}],
     questions=[{"q": "Time dilation: t0 = 1 s, v = 0.6 c", "outputs": {"t": 1.25}},
                {"q": "Time dilation: t0 = 2.2 us, v = 0.8 c", "outputs": {"t": 2.2e-6 * 5 / 3}},
                {"q": "Time dilation: t0 = 1 s, v = 3e8 m/s", "outcome": "INVALID_INPUT"},
                {"q": "Time dilation: v = 0.6 c", "outcome": "NEEDS_INPUT"}])


def length_contraction(L0, v):
    if L0 <= 0:
        raise ValueError("the proper length must be positive")
    if v < 0:
        raise ValueError("give the speed as a magnitude (v >= 0)")
    b = _beta(v)
    s = math.sqrt((1 - b) * (1 + b))
    return _out(L=L0 * s, gamma=1 / s, shortening=L0 * _gm1(b) * s)


def ref_length_contraction(L0, v):
    # the rod rests in S' (moving at +v); its ends x' = 0 and x' = L0. Lab coordinates (ct, x) = Lambda(-v) (ct', x');
    # the far end's event that is simultaneous (ct = 0) with the near end's event at the origin gives the lab length
    import numpy as np
    M = _ref_boost(math.atanh(v / C))
    ctp = np.linalg.solve(np.array([[M[0, 0]]]), np.array([-M[0, 1] * L0]))[0]
    x = M[1, 0] * ctp + M[1, 1] * L0
    return {"L": float(x), "gamma": float(M[0, 0])}


_add("length_contraction",
     name="Length contraction", topic="special relativity / length contraction",
     purpose="The length L = L0/gamma of a rod moving along its length at speed v, given its proper length L0.",
     inputs=[_phys("L0", "length", "proper length (in the rod's rest frame)"),
             _phys("v", "speed", "speed of the rod (may be written as a fraction of c)")],
     outputs=[_o("L", "length", "contracted length measured in the frame where the rod moves"), _o("gamma", "number", "Lorentz factor"),
              _o("shortening", "length", "L0 - L")],
     primary="L", conditions=["L0 > 0", "0 <= v < c"], method="L = L0 sqrt(1 - v^2/c^2)",
     latex=r"L = \frac{L_0}{\gamma} = L_0\sqrt{1 - v^2/c^2}",
     run=length_contraction, verify=_recheck(length_contraction),
     parse=lambda q: _pairs(q, r"length contraction", {"l0": "L0", "v": "v"}, units={"L0", "v"}, speeds={"v"}),
     reference=ref_length_contraction,
     wrong=_safe(lambda L0, v: _out(L=L0 * _gamma_of_beta(v / C), gamma=_gamma_of_beta(v / C), shortening=0.0)),
     random_inputs=lambda rng: {"L0": _lu(rng, 1e-3, 1e4), "v": rng.uniform(0.01, 0.995) * C},
     templates=["Length contraction: L0 = 100 m, v = 0.6 c"],
     examples=[{"inputs": {"L0": 100.0, "v": 0.6 * C}, "outputs": {"L": 80.0}},
               {"inputs": {"L0": 5.0, "v": 0.8 * C}, "outputs": {"L": 3.0}}],
     questions=[{"q": "Length contraction: L0 = 100 m, v = 0.6 c", "outputs": {"L": 80.0}},
                {"q": "Length contraction: L0 = 5 m, v = 0.8 c", "outputs": {"L": 3.0}},
                {"q": "Length contraction: L0 = -5 m, v = 0.8 c", "outcome": "INVALID_INPUT"},
                {"q": "Length contraction: L0 = 5 m", "outcome": "NEEDS_INPUT"}])


def momentum_energy(m, v):
    if m <= 0:
        raise ValueError("the mass must be positive")
    if v < 0:
        raise ValueError("give the speed as a magnitude (v >= 0)")
    b = _beta(v)
    g = _gamma_of_beta(b)
    E0 = m * C * C
    return _out(p=g * m * v, E=g * E0, K=_gm1(b) * E0, E0=E0, gamma=g,
                E_MeV=g * E0 / MEV, K_MeV=_gm1(b) * E0 / MEV, pc_MeV=g * m * v * C / MEV)


def ref_momentum_energy(m, v):
    import numpy as np
    L = _ref_boost(math.atanh(v / C))                    # the rest four-momentum (mc^2, 0) seen from the lab
    E, pc = L @ np.array([m * C * C, 0.0])
    return {"E_MeV": float(E / MEV), "K_MeV": float((E - m * C * C) / MEV), "pc_MeV": float(pc / MEV), "gamma": float(L[0, 0])}


_add("relativistic_momentum_energy",
     name="Relativistic momentum and energy", topic="special relativity / relativistic momentum and energy",
     purpose="Momentum p = gamma m v, total energy E = gamma m c^2, kinetic energy (gamma - 1) m c^2 and rest energy of a mass m at speed v.",
     inputs=[_phys("m", "mass", "rest mass"), _phys("v", "speed", "speed (may be written as a fraction of c)")],
     outputs=[_o("p", "momentum", "relativistic momentum"), _o("E", "energy", "total energy"), _o("K", "energy", "kinetic energy"),
              _o("E0", "energy", "rest energy m c^2"), _o("gamma", "number", "Lorentz factor"),
              _o("E_MeV", "energy_mev", "total energy in MeV"), _o("K_MeV", "energy_mev", "kinetic energy in MeV"),
              _o("pc_MeV", "energy_mev", "momentum times c in MeV")],
     primary="K_MeV", conditions=["m > 0", "0 <= v < c"], method="p = gamma m v, E = gamma m c^2, K = (gamma - 1) m c^2",
     latex=r"p = \gamma m v,\quad E = \gamma m c^2,\quad K = (\gamma - 1) m c^2",
     run=momentum_energy, verify=_recheck(momentum_energy),
     parse=lambda q: _pairs(q, r"relativistic momentum(?: and energy)?", {"m": "m", "v": "v"}, units={"m", "v"}, speeds={"v"}),
     reference=ref_momentum_energy,
     wrong=_safe(lambda m, v: (lambda o: {**o, "K_MeV": _r(0.5 * m * v * v / MEV), "K": _r(0.5 * m * v * v)})(momentum_energy(m, v))),
     random_inputs=lambda rng: {"m": _lu(rng, 1e-31, 1e-25), "v": rng.uniform(0.05, 0.99) * C},
     templates=["Relativistic momentum and energy: m = 9.109e-31 kg, v = 0.6 c"],
     examples=[{"inputs": {"m": ME, "v": 0.6 * C}, "outputs": {"K_MeV": 0.25 * ME_MEV * (ME * C * C / MEV) / ME_MEV, "gamma": 1.25}},
               {"inputs": {"m": 1.0, "v": 0.8 * C}, "outputs": {"p": 1.0 * 0.8 * C * 5 / 3, "E": 5 / 3 * C * C}}],
     questions=[{"q": "Relativistic momentum and energy: m = 1 kg, v = 0.8 c", "outputs": {"p": 0.8 * C * 5 / 3, "gamma": 5 / 3}},
                {"q": "Relativistic momentum and energy: m = 1 kg, v = 1.5 c", "outcome": "INVALID_INPUT"},
                {"q": "Relativistic momentum and energy: v = 0.5 c", "outcome": "NEEDS_INPUT"}])


def energy_from_momentum(m, p):
    if m <= 0:
        raise ValueError("the mass must be positive")
    if p < 0:
        raise ValueError("give the momentum as a magnitude (p >= 0)")
    E0 = m * C * C
    pc = p * C
    E = math.hypot(pc, E0)
    K = pc * pc / (E + E0)                               # E - E0 without cancellation
    return _out(E=E, K=K, v=pc / E * C, beta=pc / E, gamma=E / E0, E_MeV=E / MEV, K_MeV=K / MEV)


def ref_energy_from_momentum(m, p):
    import numpy as np
    E0 = m * C * C
    phi = _ref_rapidity_for(lambda f: float((_ref_boost(f) @ np.array([1.0, 0.0]))[1]) - p * C / E0)
    E, pc = _ref_boost(phi) @ np.array([E0, 0.0])
    return {"E_MeV": float(E / MEV), "beta": float(pc / E), "gamma": float(E / E0)}


_add("relativistic_energy_from_momentum",
     name="Relativistic energy from momentum", topic="special relativity / energy-momentum relation",
     purpose="Total and kinetic energy and speed of a particle of mass m with momentum p, from E^2 = (pc)^2 + (mc^2)^2.",
     inputs=[_phys("m", "mass", "rest mass"), _phys("p", "momentum", "momentum")],
     outputs=[_o("E", "energy", "total energy"), _o("K", "energy", "kinetic energy"), _o("v", "speed", "speed"),
              _o("beta", "number", "v/c"), _o("gamma", "number", "Lorentz factor"), _o("E_MeV", "energy_mev", "total energy in MeV"),
              _o("K_MeV", "energy_mev", "kinetic energy in MeV")],
     primary="E_MeV", conditions=["m > 0", "p >= 0"], method="E = sqrt((pc)^2 + (mc^2)^2), K = (pc)^2/(E + mc^2), v = pc^2/E",
     latex=r"E^2 = (pc)^2 + (mc^2)^2",
     run=energy_from_momentum, verify=_recheck(energy_from_momentum),
     parse=lambda q: _pairs(q, r"relativistic energy from momentum", {"m": "m", "p": "p"}, units={"m", "p"}),
     reference=ref_energy_from_momentum,
     wrong=_safe(lambda m, p: (lambda o: {**o, "E_MeV": _r((p * C + m * C * C) / MEV)})(energy_from_momentum(m, p))),
     random_inputs=lambda rng: (lambda m: {"m": m, "p": m * C * _lu(rng, 0.05, 50)})(_lu(rng, 1e-31, 1e-25)),
     templates=["Relativistic energy from momentum: m = 1.673e-27 kg, p = 5.0e-19 kg*m/s"],
     examples=[{"inputs": {"m": 1.0, "p": 0.75 * C}, "outputs": {"E": 1.25 * C * C, "beta": 0.6}},
               {"inputs": {"m": 2.0, "p": 0.0}, "outputs": {"K": 0.0, "gamma": 1.0}}],
     questions=[{"q": "Relativistic energy from momentum: m = 1 kg, p = 224844343.5 kg*m/s", "outputs": {"beta": 0.6}},
                {"q": "Relativistic energy from momentum: m = 0 kg, p = 1 kg*m/s", "outcome": "INVALID_INPUT"},
                {"q": "Relativistic energy from momentum: m = 1 kg", "outcome": "NEEDS_INPUT"}])


def speed_from_kinetic(m, K):
    if m <= 0 or K < 0:
        raise ValueError("the mass must be positive and the kinetic energy non-negative")
    E0 = m * C * C
    g = 1 + K / E0
    pc = math.sqrt(K * (K + 2 * E0))
    b = pc / (K + E0)
    return _out(v=b * C, beta=b, gamma=g, p=pc / C, pc_MeV=pc / MEV)


def ref_speed_from_kinetic(m, K):
    import numpy as np
    E0 = m * C * C
    phi = _ref_rapidity_for(lambda f: float((_ref_boost(f) @ np.array([1.0, 0.0]))[0]) - 1.0 - K / E0)
    E, pc = _ref_boost(phi) @ np.array([E0, 0.0])
    return {"beta": float(pc / E), "gamma": float(E / E0), "pc_MeV": float(pc / MEV)}


_add("relativistic_speed_from_kinetic_energy",
     name="Speed from relativistic kinetic energy", topic="special relativity / relativistic kinetic energy",
     purpose="The speed, Lorentz factor and momentum of a particle of mass m with kinetic energy K.",
     inputs=[_phys("m", "mass", "rest mass"), _phys("K", "energy", "kinetic energy (J, eV, MeV ...)")],
     outputs=[_o("v", "speed", "speed"), _o("beta", "number", "v/c"), _o("gamma", "number", "Lorentz factor"),
              _o("p", "momentum", "momentum"), _o("pc_MeV", "energy_mev", "momentum times c in MeV")],
     primary="beta", conditions=["m > 0", "K >= 0"], method="gamma = 1 + K/(m c^2), pc = sqrt(K (K + 2 m c^2)), beta = pc/(K + m c^2)",
     latex=r"\gamma = 1 + \frac{K}{mc^2},\quad v = c\sqrt{1 - 1/\gamma^2}",
     run=speed_from_kinetic, verify=_recheck(speed_from_kinetic),
     parse=lambda q: _pairs(q, r"speed from (?:relativistic )?kinetic energy", {"m": "m", "k": "K"}, units={"m", "K"}),
     reference=ref_speed_from_kinetic,
     wrong=_safe(lambda m, K: (lambda o: {**o, "beta": _r(math.sqrt(2 * K / m) / C)})(speed_from_kinetic(m, K))),
     random_inputs=lambda rng: (lambda m: {"m": m, "K": m * C * C * _lu(rng, 0.01, 30)})(_lu(rng, 1e-31, 1e-25)),
     templates=["Speed from kinetic energy: m = 9.109e-31 kg, K = 1 MeV"],
     examples=[{"inputs": {"m": 1.0, "K": 0.25 * C * C}, "outputs": {"gamma": 1.25, "beta": 0.6}},
               {"inputs": {"m": 1.0, "K": 0.0}, "outputs": {"beta": 0.0}}],
     questions=[{"q": "Speed from kinetic energy: m = 1 kg, K = 22468879468420441 J", "outputs": {"gamma": 1.25, "beta": 0.6}},
                {"q": "Speed from kinetic energy: m = 1 kg, K = -5 J", "outcome": "INVALID_INPUT"},
                {"q": "Speed from kinetic energy: K = 1 MeV", "outcome": "NEEDS_INPUT"}])


def velocity_addition(u_prime, v):
    if not (math.isfinite(u_prime) and abs(u_prime) <= C):
        raise ValueError("the object's speed in the moving frame cannot exceed c")
    _beta(v)
    u = (u_prime + v) / (1 + u_prime * v / (C * C))
    return _out(u=u, beta=u / C)


def ref_velocity_addition(u_prime, v):
    import numpy as np
    if abs(u_prime) == C:
        return {"u": u_prime}
    M = _ref_boost(math.atanh(v / C)) @ _ref_boost(math.atanh(u_prime / C))   # two boosts composed
    return {"u": float(M[1, 0] / M[0, 0] * C)}


_add("relativistic_velocity_addition",
     name="Relativistic velocity addition", topic="special relativity / velocity addition",
     purpose="The velocity u = (u' + v)/(1 + u'v/c^2) of an object moving at u' in a frame that moves at v (signed, along x).",
     inputs=[_phys("u_prime", "speed", "velocity of the object in the moving frame (signed; may be a fraction of c)"),
             _phys("v", "speed", "velocity of the moving frame (signed; may be a fraction of c)")],
     outputs=[_o("u", "speed", "velocity of the object in the rest frame"), _o("beta", "number", "u/c")],
     primary="u", conditions=["|u'| <= c", "|v| < c"], method="u = (u' + v)/(1 + u' v/c^2)",
     latex=r"u = \frac{u' + v}{1 + u'v/c^2}",
     run=velocity_addition, verify=_recheck(velocity_addition),
     parse=lambda q: _pairs(q, r"(?:relativistic )?velocity addition", {"u_prime": "u_prime", "v": "v"},
                            units={"u_prime", "v"}, speeds={"u_prime", "v"}),
     reference=ref_velocity_addition,
     wrong=_safe(lambda u_prime, v: _out(u=u_prime + v, beta=(u_prime + v) / C)),
     random_inputs=lambda rng: {"u_prime": rng.uniform(-0.99, 0.99) * C, "v": rng.uniform(-0.99, 0.99) * C},
     templates=["Velocity addition: u_prime = 0.5 c, v = 0.5 c"],
     examples=[{"inputs": {"u_prime": 0.5 * C, "v": 0.5 * C}, "outputs": {"beta": 0.8}},
               {"inputs": {"u_prime": C, "v": 0.7 * C}, "outputs": {"u": C}},
               {"inputs": {"u_prime": -0.5 * C, "v": 0.5 * C}, "outputs": {"u": 0.0}}],
     questions=[{"q": "Velocity addition: u_prime = 0.5 c, v = 0.5 c", "outputs": {"beta": 0.8}},
                {"q": "Velocity addition: u_prime = 0.9 c, v = 0.9 c", "outputs": {"beta": 1.8 / 1.81}},
                {"q": "Velocity addition: u_prime = 0.5 c, v = 1.0 c", "outcome": "INVALID_INPUT"},
                {"q": "Velocity addition: v = 0.5 c", "outcome": "NEEDS_INPUT"}])


def relativistic_doppler(f, v, motion):
    if f <= 0:
        raise ValueError("the source frequency must be positive")
    if v < 0:
        raise ValueError("give the speed as a magnitude and the direction as approaching or receding")
    b = _beta(v)
    ratio = math.sqrt((1 + b) / (1 - b)) if motion == "approaching" else math.sqrt((1 - b) / (1 + b))
    return _out(f_obs=f * ratio, ratio=ratio, z=1 / ratio - 1)


def ref_relativistic_doppler(f, v, motion):
    import numpy as np
    s = 1.0 if motion == "approaching" else -1.0        # the photon's four-momentum (E, pc) in the source frame
    E, pc = _ref_boost(math.atanh(v / C)) @ np.array([1.0, s])
    return {"f_obs": float(f * E), "ratio": float(E)}


_MOTION = {"symbol": "motion", "type": "choice", "choices": ["approaching", "receding"],
           "meaning": "whether source and observer approach or recede", "required": True}
_add("relativistic_doppler",
     name="Relativistic Doppler shift", topic="special relativity / relativistic Doppler effect",
     purpose="The observed frequency of light from a source approaching or receding at speed v (along the line of sight).",
     inputs=[_phys("f", "frequency", "frequency in the source frame"), _phys("v", "speed", "relative speed (may be a fraction of c)"), _MOTION],
     outputs=[_o("f_obs", "frequency", "observed frequency"), _o("ratio", "number", "f_obs / f"),
              _o("z", "number", "redshift z = f/f_obs - 1 (negative for a blueshift)")],
     primary="f_obs", conditions=["f > 0", "0 <= v < c"],
     method="f_obs = f sqrt((1 + beta)/(1 - beta)) approaching, f sqrt((1 - beta)/(1 + beta)) receding",
     latex=r"f_{obs} = f\sqrt{\frac{1 \pm \beta}{1 \mp \beta}}",
     run=relativistic_doppler, verify=_recheck(relativistic_doppler),
     parse=lambda q: _pairs(q, r"relativistic doppler(?: shift)?", {"f": "f", "v": "v", "motion": "motion"}, units={"f", "v"},
                            words={"motion"}, speeds={"v"}),
     reference=ref_relativistic_doppler,
     wrong=_safe(lambda f, v, motion: (lambda r: _out(f_obs=f * r, ratio=r, z=1 / r - 1))(1 + v / C if motion == "approaching" else 1 - v / C)),
     random_inputs=lambda rng: {"f": _lu(rng, 1e6, 1e16), "v": rng.uniform(0.01, 0.95) * C, "motion": rng.choice(["approaching", "receding"])},
     templates=["Relativistic Doppler shift: f = 5e14 Hz, v = 0.6 c, motion = receding"],
     examples=[{"inputs": {"f": 1e14, "v": 0.6 * C, "motion": "approaching"}, "outputs": {"f_obs": 2e14, "ratio": 2.0}},
               {"inputs": {"f": 1e14, "v": 0.6 * C, "motion": "receding"}, "outputs": {"f_obs": 5e13, "z": 1.0}}],
     questions=[{"q": "Relativistic Doppler shift: f = 1e14 Hz, v = 0.6 c, motion = approaching", "outputs": {"f_obs": 2e14}},
                {"q": "Relativistic Doppler shift: f = 1e14 Hz, v = 0.6 c, motion = receding", "outputs": {"f_obs": 5e13}},
                {"q": "Relativistic Doppler shift: f = 1e14 Hz, v = 0.6 c, motion = sideways", "outcome": "INVALID_INPUT"},
                {"q": "Relativistic Doppler shift: f = 1e14 Hz, v = 0.6 c", "outcome": "NEEDS_INPUT"}])


def aberration(theta, v):
    if not (0 <= theta <= math.pi):
        raise ValueError("the angle must lie between 0 and 180 degrees")
    b = _beta(v)
    c0, s0 = math.cos(theta), math.sin(theta)
    g = _gamma_of_beta(b)
    kx, ky, E = g * (c0 - b), s0, g * (1 - b * c0)
    th = math.atan2(ky, kx)
    return _out(theta_prime=th, theta_prime_deg=math.degrees(th), doppler_factor=E)


def ref_aberration(theta, v):
    import numpy as np
    L = _ref_boost(-math.atanh(v / C), 3)                # into the frame that moves at +v
    E, kx, ky = L @ np.array([1.0, math.cos(theta), math.sin(theta)])
    return {"theta_prime": float(np.arctan2(ky, kx)), "doppler_factor": float(E)}


_add("relativistic_aberration",
     name="Relativistic aberration of light", topic="special relativity / aberration",
     purpose="The direction of a light ray seen from a frame moving at speed v along +x, given its direction theta (from +x) in the rest frame.",
     inputs=[_phys("theta", "angle", "direction of travel of the light in the rest frame, from the +x axis (rad or deg)"),
             _phys("v", "speed", "speed of the moving frame along +x (may be a fraction of c)")],
     outputs=[_o("theta_prime", "angle", "direction of travel in the moving frame (rad)"),
              _o("theta_prime_deg", "number", "the same angle in degrees"),
              _o("doppler_factor", "number", "photon energy ratio E'/E")],
     primary="theta_prime", conditions=["0 <= theta <= 180 deg", "|v| < c"],
     method="cos(theta') = (cos(theta) - beta)/(1 - beta cos(theta)), from the boosted photon four-momentum",
     latex=r"\cos\theta' = \frac{\cos\theta - \beta}{1 - \beta\cos\theta}",
     run=aberration, verify=_recheck(aberration),
     parse=lambda q: _pairs(q, r"(?:relativistic )?aberration(?: of light)?", {"theta": "theta", "v": "v"}, units={"theta", "v"}, speeds={"v"}),
     reference=ref_aberration,
     wrong=_safe(lambda theta, v: (lambda t: _out(theta_prime=t, theta_prime_deg=math.degrees(t), doppler_factor=1.0))(
         math.acos(max(-1.0, min(1.0, math.cos(theta) - v / C))))),
     random_inputs=lambda rng: {"theta": rng.uniform(0.1, 3.0), "v": rng.uniform(-0.9, 0.9) * C},
     templates=["Aberration of light: theta = 90 deg, v = 0.6 c"],
     examples=[{"inputs": {"theta": math.pi / 2, "v": 0.6 * C}, "outputs": {"theta_prime": math.atan2(1, -0.75), "doppler_factor": 1.25}},
               {"inputs": {"theta": 0.0, "v": 0.5 * C}, "outputs": {"theta_prime": 0.0}}],
     questions=[{"q": "Aberration of light: theta = 90 deg, v = 0.6 c", "outputs": {"doppler_factor": 1.25}},
                {"q": "Aberration of light: theta = 200 deg, v = 0.6 c", "outcome": "INVALID_INPUT"},
                {"q": "Aberration of light: theta = 90 deg", "outcome": "NEEDS_INPUT"}])


def spacetime_interval(dt, dx):
    if not (math.isfinite(dt) and math.isfinite(dx)) or (dt == 0 and dx == 0):
        raise ValueError("the two events must be distinct")
    a, b = C * abs(dt), abs(dx)
    s2 = (a - b) * (a + b)
    if abs(a - b) <= 1e-12 * (a + b):
        return _out(s2=0.0, nature="lightlike", tau=0.0, proper_length=0.0)
    if s2 > 0:
        return _out(s2=s2, nature="timelike", tau=math.sqrt(s2) / C, proper_length=0.0)
    return _out(s2=s2, nature="spacelike", tau=0.0, proper_length=math.sqrt(-s2))


def ref_spacetime_interval(dt, dx):
    import numpy as np
    a, b = C * dt, dx
    if abs(abs(a) - abs(b)) <= 1e-12 * (abs(a) + abs(b)):
        return {"nature": "lightlike"}
    if abs(a) > abs(b):                                  # boost into the frame where both events happen at one place
        ct, x = _ref_boost(-math.atanh(b / a)) @ np.array([a, b])
        return {"nature": "timelike", "tau": float(abs(ct) / C), "s2": float(ct * ct)}
    ct, x = _ref_boost(-math.atanh(a / b)) @ np.array([a, b])   # ... or at one time
    return {"nature": "spacelike", "proper_length": float(abs(x)), "s2": float(-x * x)}


_add("spacetime_interval",
     name="Spacetime interval between two events", topic="special relativity / spacetime interval",
     purpose="The invariant interval s^2 = (c dt)^2 - dx^2 between two events, its type, and the proper time or proper distance.",
     inputs=[_phys("dt", "time", "time between the events (signed)"), _phys("dx", "length", "distance between the events along x (signed)")],
     outputs=[_o("s2", "area", "interval s^2 = (c dt)^2 - dx^2"), {"symbol": "nature", "kind": "number", "meaning": "timelike, spacelike or lightlike"},
              _o("tau", "time", "proper time (timelike; else 0)"), _o("proper_length", "length", "proper distance (spacelike; else 0)")],
     primary="s2", conditions=["the events are distinct"],
     method="s^2 = (c dt - dx)(c dt + dx); tau = sqrt(s^2)/c or L = sqrt(-s^2)",
     latex=r"s^2 = (c\,\Delta t)^2 - \Delta x^2",
     run=spacetime_interval, verify=_recheck(spacetime_interval),
     parse=lambda q: _pairs(q, r"spacetime interval", {"dt": "dt", "dx": "dx"}, units={"dt", "dx"}),
     reference=ref_spacetime_interval,
     wrong=_safe(lambda dt, dx: (lambda o: {**o, "s2": _r((C * dt) ** 2 + dx * dx)})(spacetime_interval(dt, dx))),
     random_inputs=lambda rng: (lambda dt: {"dt": dt, "dx": C * abs(dt) * rng.choice([rng.uniform(0.0, 0.95), rng.uniform(1.05, 5.0)]) * rng.choice([-1, 1])})(
         rng.choice([-1, 1]) * _lu(rng, 1e-9, 1e3)),
     templates=["Spacetime interval: dt = 5 s, dx = 9e8 m"],
     examples=[{"inputs": {"dt": 5.0, "dx": 3 * C}, "outputs": {"tau": 4.0, "nature": "timelike"}},
               {"inputs": {"dt": 3.0, "dx": 5 * C}, "outputs": {"proper_length": 4 * C, "nature": "spacelike"}},
               {"inputs": {"dt": 1.0, "dx": C}, "outputs": {"s2": 0.0, "nature": "lightlike"}}],
     questions=[{"q": "Spacetime interval: dt = 5 s, dx = 899377374 m", "outputs": {"tau": 4.0}},
                {"q": "Spacetime interval: dt = 0 s, dx = 0 m", "outcome": "INVALID_INPUT"},
                {"q": "Spacetime interval: dt = 5 s", "outcome": "NEEDS_INPUT"}])


def _vec3(p, name):
    if len(p) != 3:
        raise ValueError(f"{name} needs three components")
    return [float(x) for x in p]


def invariant_mass(E1, p1, E2, p2):
    p1, p2 = _vec3(p1, "p1"), _vec3(p2, "p2")
    for E, p, n in ((E1, p1, 1), (E2, p2, 2)):
        if E <= 0 or E < math.sqrt(sum(x * x for x in p)) * (1 - 1e-12):
            raise ValueError(f"particle {n} needs E >= |p| (a real, non-negative mass)")
    E = E1 + E2
    P = [a + b for a, b in zip(p1, p2)]
    Pm = math.sqrt(sum(x * x for x in P))
    M2 = (E - Pm) * (E + Pm)
    if M2 <= 0:
        raise ValueError("the pair has no rest frame (zero invariant mass)")
    return _out(M=math.sqrt(M2), P=Pm, beta_cm=Pm / E, gamma_cm=E / math.sqrt(M2))


def ref_invariant_mass(E1, p1, E2, p2):
    import numpy as np
    E = E1 + E2
    P = np.array(p1, float) + np.array(p2, float)
    Pm = float(np.linalg.norm(P))
    L = _ref_boost(-math.atanh(Pm / E))                  # boost along the total momentum into the centre-of-mass frame
    Ecm, Pcm = L @ np.array([E, Pm])
    return {"M": float(Ecm), "beta_cm": Pm / E}


_add("invariant_mass",
     name="Invariant mass of two particles", topic="special relativity / four-vectors and invariant mass",
     purpose="The invariant mass M = sqrt((E1 + E2)^2 - |p1 + p2|^2) of two particles, in natural units (GeV, GeV/c, GeV/c^2).",
     inputs=[_numin("E1", "energy of particle 1 (GeV)"), {"symbol": "p1", "type": "vector", "min_length": 3, "max_length": 3,
                                                         "meaning": "momentum of particle 1 (px, py, pz in GeV/c)", "required": True},
             _numin("E2", "energy of particle 2 (GeV)"), {"symbol": "p2", "type": "vector", "min_length": 3, "max_length": 3,
                                                         "meaning": "momentum of particle 2 (px, py, pz in GeV/c)", "required": True}],
     outputs=[_o("M", "number", "invariant mass (GeV/c^2)"), _o("P", "number", "magnitude of the total momentum (GeV/c)"),
              _o("beta_cm", "number", "speed of the centre-of-mass frame / c"), _o("gamma_cm", "number", "Lorentz factor of the centre-of-mass frame")],
     primary="M", conditions=["E >= |p| for each particle", "a positive invariant mass"],
     method="M^2 = (E - |P|)(E + |P|) with E = E1 + E2 and P = p1 + p2",
     latex=r"M c^2 = \sqrt{(E_1 + E_2)^2 - |\vec p_1 + \vec p_2|^2 c^2}",
     run=invariant_mass, verify=_recheck(invariant_mass),
     parse=lambda q: _pairs(q, r"invariant mass", {"e1": "E1", "p1": "p1", "e2": "E2", "p2": "p2"}, vectors={"p1", "p2"}),
     reference=ref_invariant_mass,
     wrong=_safe(lambda E1, p1, E2, p2: (lambda o: {**o, "M": _r(math.sqrt(max(0.0, E1 * E1 - sum(x * x for x in p1))) +
                                                               math.sqrt(max(0.0, E2 * E2 - sum(x * x for x in p2))))})(invariant_mass(E1, p1, E2, p2))),
     random_inputs=lambda rng: (lambda m1, m2, a, b: {"E1": math.sqrt(m1 * m1 + sum(x * x for x in a)), "p1": a,
                                                      "E2": math.sqrt(m2 * m2 + sum(x * x for x in b)), "p2": b})(
         rng.uniform(0.1, 5), rng.uniform(0.1, 5), [rng.uniform(-10, 10) for _ in range(3)], [rng.uniform(-10, 10) for _ in range(3)]),
     templates=["Invariant mass: E1 = 50; p1 = 30, 0, 40; E2 = 50; p2 = -30, 0, -40"],
     examples=[{"inputs": {"E1": 50.0, "p1": [30.0, 0.0, 40.0], "E2": 50.0, "p2": [-30.0, 0.0, -40.0]}, "outputs": {"M": 100.0, "P": 0.0}},
               {"inputs": {"E1": 5.0, "p1": [3.0, 0.0, 0.0], "E2": 5.0, "p2": [0.0, 4.0, 0.0]}, "outputs": {"M": math.sqrt(75.0)}}],
     questions=[{"q": "Invariant mass: E1 = 50; p1 = 30, 0, 40; E2 = 50; p2 = -30, 0, -40", "outputs": {"M": 100.0}},
                {"q": "Invariant mass: E1 = 1; p1 = 3, 0, 4; E2 = 50; p2 = 0, 0, 0", "outcome": "INVALID_INPUT"},
                {"q": "Invariant mass: E1 = 50; p1 = 30, 0, 40", "outcome": "NEEDS_INPUT"}])


def fixed_target_cm_energy(m1, E1, m2):
    if m1 < 0 or m2 <= 0 or E1 < m1 or E1 <= 0:
        raise ValueError("masses must be non-negative (target positive) and the beam energy at least the beam mass")
    s = m1 * m1 + m2 * m2 + 2 * E1 * m2
    rs = math.sqrt(s)
    p1 = math.sqrt((E1 - m1) * (E1 + m1))
    return _out(sqrt_s=rs, available=rs - m1 - m2, beta_cm=p1 / (E1 + m2), gamma_cm=(E1 + m2) / rs)


def ref_fixed_target_cm_energy(m1, E1, m2):
    import numpy as np
    p1 = math.sqrt(E1 * E1 - m1 * m1)
    E, P = E1 + m2, p1
    Ecm, _ = _ref_boost(-math.atanh(P / E)) @ np.array([E, P])
    return {"sqrt_s": float(Ecm), "available": float(Ecm - m1 - m2), "beta_cm": P / E}


_add("fixed_target_cm_energy",
     name="Centre-of-mass energy of a fixed-target collision", topic="special relativity / collisions and available energy",
     purpose="sqrt(s) for a beam particle of mass m1 and total energy E1 striking a target of mass m2 at rest (natural units, GeV).",
     inputs=[_numin("m1", "beam particle mass (GeV/c^2)"), _numin("E1", "beam particle total energy (GeV)"), _numin("m2", "target mass (GeV/c^2)")],
     outputs=[_o("sqrt_s", "number", "centre-of-mass energy sqrt(s) (GeV)"),
              _o("available", "number", "energy available for new particles, sqrt(s) - m1 - m2 (GeV)"),
              _o("beta_cm", "number", "speed of the centre-of-mass frame / c"), _o("gamma_cm", "number", "its Lorentz factor")],
     primary="sqrt_s", conditions=["E1 >= m1 >= 0", "m2 > 0"], method="s = m1^2 + m2^2 + 2 E1 m2",
     latex=r"\sqrt{s} = \sqrt{m_1^2 c^4 + m_2^2 c^4 + 2 E_1 m_2 c^2}",
     run=fixed_target_cm_energy, verify=_recheck(fixed_target_cm_energy),
     parse=lambda q: _pairs(q, r"(?:fixed[- ]target )?centre[- ]of[- ]mass energy|(?:fixed[- ]target )?center[- ]of[- ]mass energy",
                            {"m1": "m1", "e1": "E1", "m2": "m2"}),
     reference=ref_fixed_target_cm_energy,
     wrong=_safe(lambda m1, E1, m2: (lambda o: {**o, "sqrt_s": _r(E1 + m2)})(fixed_target_cm_energy(m1, E1, m2))),
     random_inputs=lambda rng: (lambda m1: {"m1": m1, "E1": m1 + _lu(rng, 0.01, 1000), "m2": rng.uniform(0.1, 10)})(rng.uniform(0.0, 5)),
     templates=["Centre-of-mass energy: m1 = 0.938272, E1 = 7000, m2 = 0.938272"],
     examples=[{"inputs": {"m1": 1.0, "E1": 7.0, "m2": 1.0}, "outputs": {"sqrt_s": 4.0, "available": 2.0}},
               {"inputs": {"m1": 0.0, "E1": 2.0, "m2": 1.0}, "outputs": {"sqrt_s": math.sqrt(5.0)}}],
     questions=[{"q": "Centre-of-mass energy: m1 = 1, E1 = 7, m2 = 1", "outputs": {"sqrt_s": 4.0}},
                {"q": "Centre-of-mass energy: m1 = 2, E1 = 1, m2 = 1", "outcome": "INVALID_INPUT"},
                {"q": "Centre-of-mass energy: m1 = 1, E1 = 7", "outcome": "NEEDS_INPUT"}])

# ================================================================================================ photons and matter waves
def _ref_photon_energy(lam):
    """Photon energy as the level spacing of the field mode's oscillator Hamiltonian hbar w (a^dag a + 1/2), built and
    diagonalised with qutip (w = 2 pi nu, nu from scipy.constants.lambda2nu)."""
    import numpy as np
    import qutip
    from scipy import constants as sc
    w = 2 * np.pi * sc.lambda2nu(lam)
    ev = (sc.hbar * w * (qutip.num(4) + 0.5 * qutip.qeye(4))).eigenenergies()
    return float(ev[1] - ev[0])


def photoelectric(lam, W):
    if lam <= 0 or W <= 0:
        raise ValueError("the wavelength and the work function must be positive")
    Eph = H * C / lam
    K = Eph - W
    if K < -1e-12 * Eph:
        raise ValueError("no electrons are emitted: the photon energy is below the work function")
    K = max(K, 0.0)
    lt = H * C / W
    return _out(E_photon_eV=Eph / EV, K_max_eV=K / EV, V_stop=K / QE, lambda_threshold=lt, lambda_threshold_nm=lt * 1e9, f_threshold=W / H)


def ref_photoelectric(lam, W):
    from scipy import constants as sc
    from scipy.integrate import solve_ivp
    from scipy.optimize import brentq
    E = _ref_photon_energy(lam)
    K = E - W
    F = 1.0                                              # electron against a uniform retarding field of 1 V/m
    v0 = (2 * K / sc.m_e) ** 0.5
    ev = lambda t, y: y[1]                               # noqa: E731  the electron stops
    ev.terminal, ev.direction = True, -1
    T = sc.m_e * v0 / (sc.e * F)
    sol = solve_ivp(lambda t, y: [y[1], -sc.e * F / sc.m_e], (0, 2 * T), [0.0, v0], events=ev, method="DOP853", rtol=1e-13, atol=1e-30)
    V = F * float(sol.y_events[0][0][0])
    lt = brentq(lambda x: _ref_photon_energy(x) - W, 1e-12, 1e-3, xtol=1e-25, rtol=8.9e-16, maxiter=500)
    return {"E_photon_eV": E / sc.e, "K_max_eV": K / sc.e, "V_stop": V, "lambda_threshold_nm": lt * 1e9}


_add("photoelectric_effect",
     name="Photoelectric effect", topic="photons and matter waves / photoelectric effect",
     purpose="Maximum kinetic energy of photoelectrons, stopping potential and threshold wavelength from the light's wavelength and the metal's work function.",
     inputs=[_phys("lam", "length", "wavelength of the light (nm, m ...)"), _phys("W", "energy", "work function of the metal (eV, J)")],
     outputs=[_o("E_photon_eV", "energy_ev", "photon energy"), _o("K_max_eV", "energy_ev", "maximum kinetic energy of the photoelectrons"),
              _o("V_stop", "voltage", "stopping potential"), _o("lambda_threshold", "length", "threshold (cut-off) wavelength"),
              _o("lambda_threshold_nm", "length_nm", "threshold wavelength in nm"), _o("f_threshold", "frequency", "threshold frequency")],
     primary="K_max_eV", conditions=["wavelength and work function positive", "photon energy at least the work function"],
     method="K_max = h c/lambda - W, V_stop = K_max/e, lambda_0 = h c/W",
     latex=r"K_{max} = \frac{hc}{\lambda} - W = e V_{stop}",
     run=photoelectric, verify=_recheck(photoelectric),
     parse=lambda q: _pairs(q, r"photoelectric(?: effect)?", {"lambda": "lam", "lam": "lam", "w": "W"}, units={"lam", "W"}),
     reference=ref_photoelectric,
     wrong=_safe(lambda lam, W: (lambda o: {**o, "K_max_eV": _r(H * C / lam / EV), "V_stop": _r(H * C / lam / QE)})(photoelectric(lam, W))),
     random_inputs=lambda rng: (lambda W: {"W": W, "lam": H * C / (W * rng.uniform(1.1, 4.0))})(rng.uniform(1.5, 6.0) * EV),
     templates=["Photoelectric effect: lambda = 250 nm, W = 4.3 eV"],
     examples=[{"inputs": {"lam": H * C / (6.0 * EV), "W": 2.0 * EV}, "outputs": {"K_max_eV": 4.0, "V_stop": 4.0}},
               {"inputs": {"lam": 250e-9, "W": 4.3 * EV}, "outputs": {"K_max_eV": H * C / 250e-9 / EV - 4.3}}],
     questions=[{"q": "Photoelectric effect: lambda = 250 nm, W = 4.3 eV", "outputs": {"K_max_eV": H * C / 250e-9 / EV - 4.3}},
                {"q": "Photoelectric effect: lambda = 700 nm, W = 4.3 eV", "outcome": "INVALID_INPUT"},
                {"q": "Photoelectric effect: lambda = 250 nm", "outcome": "NEEDS_INPUT"}])


def compton(lam, theta):
    if lam <= 0:
        raise ValueError("the wavelength must be positive")
    if not (0 <= theta <= math.pi):
        raise ValueError("the scattering angle must lie between 0 and 180 degrees")
    lc = H / (ME * C)
    d = 2 * lc * math.sin(theta / 2) ** 2
    lp = lam + d
    E0, E1 = H * C / lam, H * C / lp
    return _out(lambda_prime=lp, lambda_prime_pm=lp * 1e12, shift_pm=d * 1e12, E_in_keV=E0 / (1e3 * EV),
                E_out_keV=E1 / (1e3 * EV), K_electron_keV=E0 * d / lp / (1e3 * EV))


def ref_compton(lam, theta):
    # energy and momentum conservation solved numerically for the scattered photon energy (electron at rest)
    from scipy import constants as sc
    from scipy.optimize import brentq
    keV = 1e3 * sc.e
    E0 = sc.h * sc.c / lam / keV
    m = sc.m_e * sc.c ** 2 / keV

    def f(E1):
        px, py = E0 - E1 * math.cos(theta), -E1 * math.sin(theta)
        return E0 + m - E1 - math.sqrt(px * px + py * py + m * m)
    E1 = brentq(f, E0 * 1e-9, E0, xtol=1e-300, rtol=8.9e-16, maxiter=500) if theta > 0 else E0
    lp = sc.h * sc.c / (E1 * keV)
    return {"lambda_prime_pm": lp * 1e12, "E_out_keV": E1, "K_electron_keV": E0 - E1}


_add("compton_scattering",
     name="Compton scattering", topic="photons and matter waves / Compton effect",
     purpose="Wavelength and energy of a photon scattered by a free electron at rest through angle theta, and the electron's kinetic energy.",
     inputs=[_phys("lam", "length", "incident wavelength (pm, nm, m)"), _phys("theta", "angle", "scattering angle of the photon (deg or rad)")],
     outputs=[_o("lambda_prime", "length", "scattered wavelength"), _o("lambda_prime_pm", "length_pm", "scattered wavelength in pm"),
              _o("shift_pm", "length_pm", "Compton shift lambda' - lambda in pm"), _o("E_in_keV", "energy_kev", "incident photon energy"),
              _o("E_out_keV", "energy_kev", "scattered photon energy"), _o("K_electron_keV", "energy_kev", "kinetic energy of the recoil electron")],
     primary="lambda_prime_pm", conditions=["lambda > 0", "0 <= theta <= 180 deg"],
     method="lambda' - lambda = (h/(m_e c))(1 - cos theta) = 2 (h/(m_e c)) sin^2(theta/2)",
     latex=r"\lambda' - \lambda = \frac{h}{m_e c}\left(1 - \cos\theta\right)",
     run=compton, verify=_recheck(compton),
     parse=lambda q: _pairs(q, r"compton(?: scattering| shift| effect)?", {"lambda": "lam", "lam": "lam", "theta": "theta"}, units={"lam", "theta"}),
     reference=ref_compton,
     wrong=_safe(lambda lam, theta: (lambda o: {**o, "lambda_prime_pm": _r((lam + H / (ME * C) * (1 + math.cos(theta))) * 1e12)})(compton(lam, theta))),
     random_inputs=lambda rng: {"lam": _lu(rng, 1e-12, 1e-10), "theta": rng.uniform(0.2, math.pi)},
     templates=["Compton scattering: lambda = 71 pm, theta = 90 deg"],
     examples=[{"inputs": {"lam": 71e-12, "theta": math.pi / 2}, "outputs": {"shift_pm": H / (ME * C) * 1e12}},
               {"inputs": {"lam": 10e-12, "theta": math.pi}, "outputs": {"lambda_prime_pm": 10 + 2 * H / (ME * C) * 1e12}}],
     questions=[{"q": "Compton scattering: lambda = 71 pm, theta = 90 deg", "outputs": {"shift_pm": H / (ME * C) * 1e12}},
                {"q": "Compton scattering: lambda = 71 pm, theta = 270 deg", "outcome": "INVALID_INPUT"},
                {"q": "Compton scattering: theta = 90 deg", "outcome": "NEEDS_INPUT"}])

_PARTICLE_EP = {"symbol": "particle", "type": "choice", "choices": ["electron", "proton"],
                "meaning": "the accelerated particle (charge e)", "required": True}


def de_broglie_voltage(V, particle):
    if V <= 0:
        raise ValueError("the accelerating voltage must be positive")
    m = ME if particle == "electron" else MP
    K = QE * V
    p_nr = math.sqrt(2 * m * K)
    p_r = math.sqrt(K * (K + 2 * m * C * C)) / C
    return _out(lambda_nonrel=H / p_nr, lambda_nonrel_nm=H / p_nr * 1e9, lambda_rel=H / p_r, lambda_rel_nm=H / p_r * 1e9,
                p=p_r, K_eV=V)


def ref_de_broglie_voltage(V, particle):
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import solve_ivp
    m = sc.m_e if particle == "electron" else sc.m_p
    d = 1.0                                              # Newtonian acceleration across a 1 m gap at field V/d
    a = sc.e * V / d / m
    ev = lambda t, y: y[0] - d                           # noqa: E731
    ev.terminal = True
    T = math.sqrt(2 * d / a)
    sol = solve_ivp(lambda t, y: [y[1], a], (0, 2 * T), [0.0, 0.0], events=ev, method="DOP853", rtol=1e-13, atol=1e-30)
    v = float(sol.y_events[0][0][1])
    # relativistic equation of motion dp/dt = F = eV/d with v = p c^2/E: the distance covered while the momentum grows
    # to p is x(p) = integral_0^p v(p')/F dp' (quadrature); the momentum at the far side solves x(p) = d (brentq)
    from scipy.integrate import quad
    from scipy.optimize import brentq
    F = sc.e * V / d
    mc = m * sc.c
    x_of = lambda P: quad(lambda q: (q / mc) * sc.c / math.sqrt(1 + (q / mc) ** 2) / F, 0, P, epsabs=0, epsrel=1e-13)[0]   # noqa: E731
    pmax = m * v * (1.000001 + sc.e * V / (m * sc.c ** 2))
    pr = brentq(lambda P: x_of(P) - d, 0.0, pmax, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"lambda_nonrel_nm": sc.h / (m * v) * 1e9, "lambda_rel_nm": sc.h / pr * 1e9}


_add("de_broglie_accelerated",
     name="de Broglie wavelength of an accelerated particle", topic="photons and matter waves / de Broglie wavelength",
     purpose="The de Broglie wavelength of an electron or proton accelerated from rest through a voltage V, non-relativistic and relativistic.",
     inputs=[_phys("V", "voltage", "accelerating voltage"), _PARTICLE_EP],
     outputs=[_o("lambda_nonrel", "length", "wavelength h/sqrt(2 m e V)"), _o("lambda_nonrel_nm", "length_nm", "the same in nm"),
              _o("lambda_rel", "length", "relativistic wavelength"), _o("lambda_rel_nm", "length_nm", "the same in nm"),
              _o("p", "momentum", "relativistic momentum"), _o("K_eV", "energy_ev", "kinetic energy")],
     primary="lambda_rel_nm", conditions=["V > 0"],
     method="lambda = h/p with p = sqrt(2 m e V) (non-relativistic) or p = sqrt(K (K + 2 m c^2))/c, K = e V",
     latex=r"\lambda = \frac{h}{p},\quad p = \frac{1}{c}\sqrt{K(K + 2mc^2)}",
     run=de_broglie_voltage, verify=_recheck(de_broglie_voltage),
     parse=lambda q: _pairs(q, r"de broglie wavelength(?: of an accelerated particle)?", {"v": "V", "particle": "particle"},
                            units={"V"}, words={"particle"}),
     reference=ref_de_broglie_voltage,
     wrong=_safe(lambda V, particle: (lambda o: {**o, "lambda_rel_nm": _r(o["lambda_nonrel_nm"] * 2 ** 0.5), "lambda_nonrel_nm": _r(o["lambda_nonrel_nm"] * 2 ** 0.5)})(
         de_broglie_voltage(V, particle))),
     random_inputs=lambda rng: {"V": _lu(rng, 1.0, 1e6), "particle": rng.choice(["electron", "proton"])},
     templates=["de Broglie wavelength: V = 100 V, particle = electron"],
     examples=[{"inputs": {"V": 100.0, "particle": "electron"}, "outputs": {"lambda_nonrel_nm": H / math.sqrt(2 * ME * QE * 100) * 1e9}},
               {"inputs": {"V": 1000.0, "particle": "proton"}, "outputs": {"lambda_nonrel_nm": H / math.sqrt(2 * MP * QE * 1000) * 1e9}}],
     questions=[{"q": "de Broglie wavelength: V = 100 V, particle = electron", "outputs": {"lambda_nonrel_nm": H / math.sqrt(2 * ME * QE * 100) * 1e9}},
                {"q": "de Broglie wavelength: V = -5 V, particle = electron", "outcome": "INVALID_INPUT"},
                {"q": "de Broglie wavelength: V = 100 V", "outcome": "NEEDS_INPUT"}])


def heisenberg(dx, m):
    if dx <= 0 or m <= 0:
        raise ValueError("the position uncertainty and the mass must be positive")
    dp = HBAR / (2 * dx)
    return _out(dp=dp, dp_eV_per_c=dp * C / EV, dv=dp / m)


def ref_heisenberg(dx, m):
    # a minimum-uncertainty (Gaussian) wave packet of width sigma_x = 1 on a grid, its spread in wavenumber by FFT
    import numpy as np
    from scipy import constants as sc
    N, Lb = 4096, 80.0
    x = np.linspace(-Lb / 2, Lb / 2, N, endpoint=False)
    psi = np.exp(-x * x / 4.0)
    phi = np.fft.fft(psi)
    k = 2 * np.pi * np.fft.fftfreq(N, d=Lb / N)
    w = np.abs(phi) ** 2
    sk = float(np.sqrt(np.sum(k * k * w) / np.sum(w)))
    dp = sc.hbar * sk / dx
    return {"dp_eV_per_c": dp * sc.c / sc.e, "dv": dp / m}


_add("heisenberg_minimum_uncertainty",
     name="Minimum momentum uncertainty (Heisenberg)", topic="photons and matter waves / uncertainty principle",
     purpose="The smallest momentum and velocity uncertainty compatible with a position uncertainty dx: dp = hbar/(2 dx).",
     inputs=[_phys("dx", "length", "position uncertainty (standard deviation)"), _phys("m", "mass", "mass of the particle")],
     outputs=[_o("dp", "momentum", "minimum momentum uncertainty"), _o("dp_eV_per_c", "number", "the same in eV/c"),
              _o("dv", "speed", "minimum velocity uncertainty")],
     primary="dv", conditions=["dx > 0", "m > 0"], method="dp = hbar/(2 dx) (a minimum-uncertainty Gaussian packet), dv = dp/m",
     latex=r"\Delta x\,\Delta p \ge \frac{\hbar}{2}",
     run=heisenberg, verify=_recheck(heisenberg),
     parse=lambda q: _pairs(q, r"(?:heisenberg )?minimum (?:momentum )?uncertainty", {"dx": "dx", "m": "m"}, units={"dx", "m"}),
     reference=ref_heisenberg,
     wrong=_safe(lambda dx, m: (lambda dp: _out(dp=dp, dp_eV_per_c=dp * C / EV, dv=dp / m))(H / (4 * math.pi * dx) * 2)),
     random_inputs=lambda rng: {"dx": _lu(rng, 1e-15, 1e-6), "m": _lu(rng, 1e-31, 1e-20)},
     templates=["Minimum uncertainty: dx = 0.1 nm, m = 9.109e-31 kg"],
     examples=[{"inputs": {"dx": 1e-10, "m": ME}, "outputs": {"dv": HBAR / 2e-10 / ME}},
               {"inputs": {"dx": 1e-15, "m": MP}, "outputs": {"dp_eV_per_c": HBAR / 2e-15 * C / EV}}],
     questions=[{"q": "Minimum uncertainty: dx = 0.1 nm, m = 9.1093837139e-31 kg", "outputs": {"dv": HBAR / 2e-10 / ME}},
                {"q": "Minimum uncertainty: dx = 0 nm, m = 1 kg", "outcome": "INVALID_INPUT"},
                {"q": "Minimum uncertainty: dx = 0.1 nm", "outcome": "NEEDS_INPUT"}])


def thermal_de_broglie(m, T):
    if m <= 0 or T <= 0:
        raise ValueError("the mass and the temperature must be positive")
    lam = H / math.sqrt(2 * math.pi * m * KB * T)
    return _out(lambda_th=lam, lambda_th_nm=lam * 1e9, n_Q=lam ** -3)


def ref_thermal_de_broglie(m, T):
    # the single-particle translational partition function per volume, Z1/V = (1/h^3) * integral of exp(-p^2/2mkT) d^3p,
    # by quadrature; the thermal wavelength is (V/Z1)^(1/3)
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    s = math.sqrt(2 * m * sc.k * T)
    I, _ = quad(lambda u: 4 * np.pi * u * u * np.exp(-u * u), 0, np.inf, epsabs=0, epsrel=1e-13)
    z = I * s ** 3 / sc.h ** 3
    return {"lambda_th_nm": z ** (-1 / 3) * 1e9, "n_Q": z}


_add("thermal_de_broglie_wavelength",
     name="Thermal de Broglie wavelength", topic="statistical physics / quantum concentration",
     purpose="The thermal de Broglie wavelength h/sqrt(2 pi m k T) and the quantum concentration 1/lambda^3.",
     inputs=[_phys("m", "mass", "particle mass"), _phys("T", "temperature", "temperature")],
     outputs=[_o("lambda_th", "length", "thermal de Broglie wavelength"), _o("lambda_th_nm", "length_nm", "the same in nm"),
              _o("n_Q", "number_density", "quantum concentration 1/lambda^3")],
     primary="lambda_th_nm", conditions=["m > 0", "T > 0"], method="lambda = h/sqrt(2 pi m k T)",
     latex=r"\lambda_{th} = \frac{h}{\sqrt{2\pi m k_B T}}",
     run=thermal_de_broglie, verify=_recheck(thermal_de_broglie),
     parse=lambda q: _pairs(q, r"thermal de broglie wavelength", {"m": "m", "t": "T"}, units={"m", "T"}),
     reference=ref_thermal_de_broglie,
     wrong=_safe(lambda m, T: (lambda lam: _out(lambda_th=lam, lambda_th_nm=lam * 1e9, n_Q=lam ** -3))(H / math.sqrt(3 * m * KB * T))),
     random_inputs=lambda rng: {"m": _lu(rng, 1e-31, 1e-24), "T": _lu(rng, 1e-6, 1e4)},
     templates=["Thermal de Broglie wavelength: m = 6.646e-27 kg, T = 300 K"],
     examples=[{"inputs": {"m": ME, "T": 300.0}, "outputs": {"lambda_th_nm": H / math.sqrt(2 * math.pi * ME * KB * 300) * 1e9}}],
     questions=[{"q": "Thermal de Broglie wavelength: m = 9.1093837139e-31 kg, T = 300 K", "outputs": {"lambda_th_nm": H / math.sqrt(2 * math.pi * ME * KB * 300) * 1e9}},
                {"q": "Thermal de Broglie wavelength: m = 1e-26 kg, T = -3 K", "outcome": "INVALID_INPUT"},
                {"q": "Thermal de Broglie wavelength: T = 300 K", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ hydrogen atom
def _ref_hydrogen(n, l, Z, mu_ratio=1.0):
    """Energy (J), <T> (J), <r>, <r^2>, <1/r> (m) of the hydrogen-like state n, l by quadrature of the exact radial
    function (scipy.special.genlaguerre) — kinetic plus potential energy integrated, not the Bohr formula. Lengths
    in units of a_mu = a0 m_e/mu; energies from scipy's Hartree energy."""
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    from scipy.special import genlaguerre
    Lp = genlaguerre(n - l - 1, 2 * l + 1)
    dLp = Lp.deriv()
    k = 2.0 * Z / n

    def u(r):
        return r ** (l + 1) * np.exp(-Z * r / n) * Lp(k * r)

    def du(r):
        return np.exp(-Z * r / n) * ((l + 1) * r ** l * Lp(k * r) - Z / n * r ** (l + 1) * Lp(k * r) + r ** (l + 1) * k * dLp(k * r))

    rmax = 60.0 * n * n / Z
    opts = dict(epsabs=0, epsrel=1e-13, limit=400)
    pts = None
    norm = quad(lambda r: u(r) ** 2, 0, rmax, **opts)[0]
    T = quad(lambda r: 0.5 * du(r) ** 2 + (0.5 * l * (l + 1) / (r * r) * u(r) ** 2 if r > 0 else 0.0), 0, rmax, **opts)[0] / norm
    V = -Z * quad(lambda r: u(r) ** 2 / r if r > 0 else 0.0, 0, rmax, **opts)[0] / norm
    r1 = quad(lambda r: r * u(r) ** 2, 0, rmax, **opts)[0] / norm
    r2 = quad(lambda r: r * r * u(r) ** 2, 0, rmax, **opts)[0] / norm
    Eh = sc.physical_constants["Hartree energy"][0] * mu_ratio
    a = sc.physical_constants["Bohr radius"][0] / mu_ratio
    return {"E": (T + V) * Eh, "T": T * Eh, "r": r1 * a, "r2": r2 * a * a, "inv_r": (-V / Z) / a, "a": a, "Lp": Lp, "dLp": dLp}


def _quantum_n(n, name="n"):
    if n < 1:
        raise ValueError(f"{name} must be a positive whole number")


def bohr_level(Z, n):
    _quantum_n(Z, "Z")
    _quantum_n(n)
    E = -Z * Z * RINF * H * C / (n * n)
    r = n * n * A0 / Z
    return _out(E_eV=E / EV, E=E, ionization_eV=-E / EV, r=r, r_nm=r * 1e9, v=Z * ALPHA * C / n)


def ref_bohr_level(Z, n):
    from scipy import constants as sc
    from scipy.optimize import brentq
    d = _ref_hydrogen(n, n - 1, Z)
    # the circular state's most probable radius: where d(u)/dr = 0, u the exact radial function (l = n - 1)
    import numpy as np
    k = 2.0 * Z / n
    du = lambda r: (n * r ** (n - 1) * d["Lp"](k * r) - Z / n * r ** n * d["Lp"](k * r) + r ** n * k * d["dLp"](k * r))   # noqa: E731
    rp = brentq(du, 0.25 * n * n / Z, 4.0 * n * n / Z, xtol=1e-300, rtol=8.9e-16) * d["a"]
    return {"E_eV": d["E"] / sc.e, "r_nm": rp * 1e9, "v": math.sqrt(2 * d["T"] / sc.m_e)}


_add("bohr_energy_level",
     name="Energy level of a hydrogen-like atom (Bohr model)", topic="hydrogen atom / energy levels",
     purpose="Energy, ionization energy, orbit radius and orbital speed of level n of a one-electron atom or ion with nuclear charge Z.",
     inputs=[_intin("Z", "atomic number (nuclear charge; 1 for hydrogen, 2 for He+ ...)", 1, 137), _intin("n", "principal quantum number", 1, 1000)],
     outputs=[_o("E_eV", "energy_ev", "energy of the level (negative: bound)"), _o("E", "energy", "the same in J"),
              _o("ionization_eV", "energy_ev", "energy needed to ionize from this level"), _o("r", "length", "Bohr orbit radius"),
              _o("r_nm", "length_nm", "orbit radius in nm"), _o("v", "speed", "orbital speed")],
     primary="E_eV", conditions=["Z >= 1", "n >= 1", "infinitely heavy nucleus (no reduced-mass correction)"],
     method="E_n = -Z^2 R_inf h c / n^2, r_n = n^2 a0/Z, v_n = Z alpha c/n",
     latex=r"E_n = -\frac{Z^2 R_\infty h c}{n^2} \approx -\frac{13.6\ \mathrm{eV}\,Z^2}{n^2}",
     run=bohr_level, verify=_recheck(bohr_level),
     parse=lambda q: _pairs(q, r"bohr (?:energy )?level|energy level of (?:a )?hydrogen(?:-like atom)?", {"z": "Z", "n": "n"}),
     reference=ref_bohr_level,
     wrong=_safe(lambda Z, n: (lambda o: {**o, "E_eV": _r(o["E_eV"] * n)})(bohr_level(Z, n))),
     random_inputs=lambda rng: {"Z": rng.randint(1, 10), "n": rng.randint(1, 7)},
     templates=["Bohr level: Z = 1, n = 2"],
     examples=[{"inputs": {"Z": 1, "n": 1}, "outputs": {"E_eV": -RINF * H * C / EV, "r": A0, "v": ALPHA * C}},
               {"inputs": {"Z": 2, "n": 3}, "outputs": {"E_eV": -4 * RINF * H * C / EV / 9, "r": 9 * A0 / 2}}],
     questions=[{"q": "Bohr level: Z = 1, n = 1", "outputs": {"E_eV": -RINF * H * C / EV}},
                {"q": "Bohr level: Z = 2, n = 3", "outputs": {"r": 4.5 * A0}},
                {"q": "Bohr level: Z = 1, n = 0", "outcome": "INVALID_INPUT"},
                {"q": "Bohr level: Z = 1", "outcome": "NEEDS_INPUT"}])

_SERIES = {1: "Lyman", 2: "Balmer", 3: "Paschen", 4: "Brackett", 5: "Pfund", 6: "Humphreys"}


def _transition(Z, n_upper, n_lower, mu_ratio):
    _quantum_n(Z, "Z")
    _quantum_n(n_lower, "n_lower")
    if n_upper <= n_lower:
        raise ValueError("the upper level must lie above the lower level (n_upper > n_lower)")
    inv = Z * Z * RINF * mu_ratio * (n_upper - n_lower) * (n_upper + n_lower) / (n_upper * n_upper * n_lower * n_lower)
    lam = 1 / inv
    return dict(E_photon_eV=H * C * inv / EV, lam=lam, lambda_nm=lam * 1e9, f=C * inv,
                series=_SERIES.get(n_lower, f"ending at n = {n_lower}"))


def hydrogen_transition(Z, n_upper, n_lower):
    return _out(**_transition(Z, n_upper, n_lower, 1.0))


def ref_hydrogen_transition(Z, n_upper, n_lower, mu_ratio=1.0):
    from scipy import constants as sc
    dE = _ref_hydrogen(n_upper, 0, Z, mu_ratio)["E"] - _ref_hydrogen(n_lower, 0, Z, mu_ratio)["E"]
    return {"E_photon_eV": dE / sc.e, "lambda_nm": sc.h * sc.c / dE * 1e9}


_NUL = [_intin("Z", "atomic number (1 for hydrogen)", 1, 137), _intin("n_upper", "upper level", 2, 1000), _intin("n_lower", "lower level", 1, 999)]
_TR_OUT = [_o("E_photon_eV", "energy_ev", "photon energy"), _o("lam", "length", "vacuum wavelength"),
           _o("lambda_nm", "length_nm", "vacuum wavelength in nm"), _o("f", "frequency", "frequency"),
           {"symbol": "series", "kind": "number", "meaning": "spectral series (Lyman, Balmer, ...)"}]
_add("hydrogen_transition",
     name="Photon from a hydrogen-like transition (Rydberg formula)", topic="hydrogen atom / spectral lines",
     purpose="Energy, wavelength and frequency of the photon emitted when a one-electron atom drops from n_upper to n_lower.",
     inputs=_NUL, outputs=_TR_OUT,
     primary="lambda_nm", conditions=["n_upper > n_lower >= 1", "infinitely heavy nucleus"],
     method="1/lambda = R_inf Z^2 (1/n_lower^2 - 1/n_upper^2)",
     latex=r"\frac{1}{\lambda} = R_\infty Z^2\left(\frac{1}{n_\ell^2} - \frac{1}{n_u^2}\right)",
     run=hydrogen_transition, verify=_recheck(hydrogen_transition),
     parse=lambda q: _pairs(q, r"hydrogen transition|rydberg formula", {"z": "Z", "n_upper": "n_upper", "n_lower": "n_lower"}),
     reference=ref_hydrogen_transition,
     wrong=_safe(lambda Z, n_upper, n_lower: (lambda o: {**o, "lambda_nm": _r(o["lambda_nm"] * Z), "E_photon_eV": _r(o["E_photon_eV"] / Z)})(
         hydrogen_transition(Z, n_upper, n_lower)) if Z > 1 else (lambda o: {**o, "lambda_nm": _r(1e9 / (RINF * (1 / n_lower - 1 / n_upper)))})(
         hydrogen_transition(Z, n_upper, n_lower))),
     random_inputs=lambda rng: (lambda lo: {"Z": rng.randint(1, 6), "n_lower": lo, "n_upper": lo + rng.randint(1, 5)})(rng.randint(1, 4)),
     templates=["Hydrogen transition: Z = 1, n_upper = 3, n_lower = 2"],
     examples=[{"inputs": {"Z": 1, "n_upper": 3, "n_lower": 2}, "outputs": {"lambda_nm": 36e9 / (5 * RINF), "series": "Balmer"}},
               {"inputs": {"Z": 1, "n_upper": 2, "n_lower": 1}, "outputs": {"E_photon_eV": 0.75 * RINF * H * C / EV, "series": "Lyman"}}],
     questions=[{"q": "Hydrogen transition: Z = 1, n_upper = 3, n_lower = 2", "outputs": {"lambda_nm": 36e9 / (5 * RINF)}},
                {"q": "Hydrogen transition: Z = 1, n_upper = 2, n_lower = 3", "outcome": "INVALID_INPUT"},
                {"q": "Hydrogen transition: Z = 1, n_upper = 3", "outcome": "NEEDS_INPUT"}])


def hydrogen_transition_reduced(Z, n_upper, n_lower, M_u):
    if M_u <= 0:
        raise ValueError("the nuclear mass must be positive")
    M = M_u * AMU
    mu_ratio = M / (M + ME)
    return _out(**_transition(Z, n_upper, n_lower, mu_ratio), mu_ratio=mu_ratio)


def ref_hydrogen_transition_reduced(Z, n_upper, n_lower, M_u):
    from scipy import constants as sc
    M = M_u * sc.physical_constants["atomic mass constant"][0]
    mu = M * sc.m_e / (M + sc.m_e)
    out = ref_hydrogen_transition(Z, n_upper, n_lower, mu / sc.m_e)
    return {**out, "mu_ratio": mu / sc.m_e}


_add("hydrogen_transition_reduced_mass",
     name="Hydrogen-like transition with the reduced-mass correction", topic="hydrogen atom / spectral lines and isotope shift",
     purpose="Photon energy and wavelength of a transition of a one-electron atom whose nucleus has mass M (in u): R_M = R_inf mu/m_e.",
     inputs=_NUL + [_numin("M_u", "mass of the nucleus in u (1.007276 for the proton, 2.013553 for the deuteron)")], outputs=_TR_OUT + [_o("mu_ratio", "number", "mu/m_e")],
     primary="lambda_nm", conditions=["n_upper > n_lower >= 1", "M > 0"],
     method="1/lambda = R_inf (mu/m_e) Z^2 (1/n_lower^2 - 1/n_upper^2), mu = m_e M/(m_e + M)",
     latex=r"\frac{1}{\lambda} = \frac{\mu}{m_e} R_\infty Z^2\left(\frac{1}{n_\ell^2} - \frac{1}{n_u^2}\right)",
     run=hydrogen_transition_reduced, verify=_recheck(hydrogen_transition_reduced),
     parse=lambda q: _pairs(q, r"reduced[- ]mass (?:hydrogen )?transition", {"z": "Z", "n_upper": "n_upper", "n_lower": "n_lower", "m_u": "M_u", "m": "M_u"}),
     reference=ref_hydrogen_transition_reduced,
     wrong=_safe(lambda Z, n_upper, n_lower, M_u: (lambda o: {**o, "lambda_nm": _r(o["lambda_nm"] * o["mu_ratio"] ** 2)})(
         hydrogen_transition_reduced(Z, n_upper, n_lower, M_u))),
     random_inputs=lambda rng: (lambda lo: {"Z": rng.randint(1, 4), "n_lower": lo, "n_upper": lo + rng.randint(1, 4),
                                            "M_u": rng.choice([1.00727646688, 2.01355321, 3.01550071, rng.uniform(0.11, 10.0)])})(rng.randint(1, 3)),
     templates=["Reduced-mass transition: Z = 1, n_upper = 3, n_lower = 2, M_u = 2.013553"],
     examples=[{"inputs": {"Z": 1, "n_upper": 3, "n_lower": 2, "M_u": 1.00727646688},
                "outputs": {"lambda_nm": 36e9 / (5 * RINF) * (1 + ME / (1.00727646688 * AMU))}}],
     questions=[{"q": "Reduced-mass transition: Z = 1, n_upper = 3, n_lower = 2, M_u = 1.00727646688",
                 "outputs": {"lambda_nm": 36e9 / (5 * RINF) * (1 + ME / (1.00727646688 * AMU))}},
                {"q": "Reduced-mass transition: Z = 1, n_upper = 3, n_lower = 2, M_u = 0", "outcome": "INVALID_INPUT"},
                {"q": "Reduced-mass transition: Z = 1, n_upper = 3, n_lower = 2", "outcome": "NEEDS_INPUT"}])


def moseley(Z):
    if Z < 2:
        raise ValueError("Moseley's law needs Z >= 2 (one electron left in the K shell screens the nucleus)")
    E = 0.75 * (Z - 1) ** 2 * RINF * H * C
    lam = H * C / E
    return _out(E_keV=E / (1e3 * EV), lam=lam, lambda_nm=lam * 1e9, lambda_pm=lam * 1e12, f=E / H)


def ref_moseley(Z):
    from scipy import constants as sc
    d = ref_hydrogen_transition(Z - 1, 2, 1)
    return {"E_keV": d["E_photon_eV"] / 1e3, "lambda_pm": d["lambda_nm"] * 1e3}


_add("moseley_k_alpha",
     name="K-alpha X-ray line (Moseley's law)", topic="hydrogen atom / X-ray spectra",
     purpose="Energy and wavelength of the K-alpha X-ray line of element Z, with one unit of screening (Z - 1).",
     inputs=[_intin("Z", "atomic number of the target element", 2, 120)],
     outputs=[_o("E_keV", "energy_kev", "photon energy"), _o("lam", "length", "wavelength"), _o("lambda_nm", "length_nm", "wavelength in nm"),
              _o("lambda_pm", "length_pm", "wavelength in pm"), _o("f", "frequency", "frequency")],
     primary="E_keV", conditions=["Z >= 2"], method="E = (3/4) R_inf h c (Z - 1)^2",
     latex=r"E_{K\alpha} = \tfrac{3}{4} R_\infty h c\,(Z - 1)^2",
     run=moseley, verify=_recheck(moseley),
     parse=lambda q: _pairs(q, r"moseley(?:'s law)?|k[- ]alpha(?: line| x-ray line)?", {"z": "Z"}),
     reference=ref_moseley,
     wrong=_safe(lambda Z: (lambda E: _out(E_keV=E / (1e3 * EV), lam=H * C / E, lambda_nm=H * C / E * 1e9, lambda_pm=H * C / E * 1e12, f=E / H))(
         0.75 * Z ** 2 * RINF * H * C)),
     random_inputs=lambda rng: {"Z": rng.randint(2, 92)},
     templates=["Moseley's law: Z = 29"],
     examples=[{"inputs": {"Z": 29}, "outputs": {"E_keV": 0.75 * 784 * RINF * H * C / (1e3 * EV)}}],
     questions=[{"q": "Moseley's law: Z = 29", "outputs": {"E_keV": 0.75 * 784 * RINF * H * C / (1e3 * EV)}},
                {"q": "Moseley's law: Z = 1", "outcome": "INVALID_INPUT"},
                {"q": "Moseley's law:", "outcome": "NEEDS_INPUT"}])


def _check_nl(n, l, Z):
    _quantum_n(n)
    _quantum_n(Z, "Z")
    if not (0 <= l < n):
        raise ValueError("the orbital quantum number must satisfy 0 <= l <= n - 1")


def radial_expectations(n, l, Z):
    _check_nl(n, l, Z)
    r = (3 * n * n - l * (l + 1)) / (2 * Z)
    r2 = n * n * (5 * n * n + 1 - 3 * l * (l + 1)) / (2 * Z * Z)
    ir = Z / (n * n)
    return _out(r_mean_a0=r, r2_mean_a0sq=r2, inv_r_mean_per_a0=ir, r_mean=r * A0, dr_a0=math.sqrt(r2 - r * r))


def ref_radial_expectations(n, l, Z):
    from scipy import constants as sc
    d = _ref_hydrogen(n, l, Z)
    a0 = sc.physical_constants["Bohr radius"][0]
    return {"r_mean_a0": d["r"] / a0, "r2_mean_a0sq": d["r2"] / a0 ** 2, "inv_r_mean_per_a0": d["inv_r"] * a0}


_add("hydrogen_radial_expectations",
     name="Radial expectation values of a hydrogen-like state", topic="hydrogen atom / wavefunctions",
     purpose="<r>, <r^2>, <1/r> and the radial spread of the state n, l of a one-electron atom with nuclear charge Z (in Bohr radii).",
     inputs=[_intin("n", "principal quantum number", 1, 1000), _intin("l", "orbital quantum number (0 <= l < n)", 0, 999),
             _intin("Z", "atomic number", 1, 137)],
     outputs=[_o("r_mean_a0", "number", "<r> in units of a0"), _o("r2_mean_a0sq", "number", "<r^2> in units of a0^2"),
              _o("inv_r_mean_per_a0", "number", "<1/r> in units of 1/a0"), _o("r_mean", "length", "<r> in m"),
              _o("dr_a0", "number", "radial spread sqrt(<r^2> - <r>^2) in a0")],
     primary="r_mean_a0", conditions=["n >= 1", "0 <= l <= n - 1", "infinitely heavy nucleus"],
     method="<r> = a0 (3n^2 - l(l+1))/(2Z), <r^2> = a0^2 n^2 (5n^2 + 1 - 3l(l+1))/(2Z^2), <1/r> = Z/(n^2 a0)",
     latex=r"\langle r\rangle = \frac{a_0}{2Z}\left[3n^2 - l(l+1)\right]",
     run=radial_expectations, verify=_recheck(radial_expectations),
     parse=lambda q: _pairs(q, r"(?:hydrogen )?radial expectation values?", {"n": "n", "l": "l", "z": "Z"}),
     reference=ref_radial_expectations,
     wrong=_safe(lambda n, l, Z: (lambda o: {**o, "r_mean_a0": _r(n * n / Z)})(radial_expectations(n, l, Z))),
     random_inputs=lambda rng: (lambda n: {"n": n, "l": rng.randint(0, n - 1), "Z": rng.randint(1, 5)})(rng.randint(1, 6)),
     templates=["Radial expectation values: n = 2, l = 1, Z = 1"],
     examples=[{"inputs": {"n": 1, "l": 0, "Z": 1}, "outputs": {"r_mean_a0": 1.5, "r2_mean_a0sq": 3.0, "inv_r_mean_per_a0": 1.0}},
               {"inputs": {"n": 2, "l": 1, "Z": 1}, "outputs": {"r_mean_a0": 5.0, "r2_mean_a0sq": 30.0}}],
     questions=[{"q": "Radial expectation values: n = 2, l = 1, Z = 1", "outputs": {"r_mean_a0": 5.0}},
                {"q": "Radial expectation values: n = 2, l = 2, Z = 1", "outcome": "INVALID_INPUT"},
                {"q": "Radial expectation values: n = 2, l = 1", "outcome": "NEEDS_INPUT"}])


def radial_probability(n, l, Z, R):
    _check_nl(n, l, Z)
    if R <= 0:
        raise ValueError("the radius must be positive")
    # r^2 R_nl^2 = e^(-x) x^(2l+2) [L_{n-l-1}^{2l+1}(x)]^2 (up to a constant) with x = 2 Z r/(n a0): an exact polynomial
    # times e^(-x), integrated term by term with the lower incomplete gamma function (40-digit arithmetic)
    import mpmath
    deg = n - l - 1
    lag = [Fraction((-1) ** i * math.comb(n + l, deg - i), math.factorial(i)) for i in range(deg + 1)]
    sq = [Fraction(0)] * (2 * deg + 1)
    for i, a in enumerate(lag):
        for j, b in enumerate(lag):
            sq[i + j] += a * b
    with mpmath.workdps(40):
        X = mpmath.mpf(2) * Z * mpmath.mpf(R) / (n * mpmath.mpf(A0))
        inner = mpmath.mpf(0)
        total = mpmath.mpf(0)
        for i, a in enumerate(sq):
            if a == 0:
                continue
            c = 2 * l + 2 + i
            coef = mpmath.mpf(a.numerator) / a.denominator
            inner += coef * mpmath.gammainc(c + 1, 0, X)
            total += coef * mpmath.factorial(c)
        P = inner / total
        Q = 1 - P
    return _out(P_inside=float(P), P_outside=float(Q), R_a0=R / A0)


def ref_radial_probability(n, l, Z, R):
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    from scipy.special import eval_genlaguerre
    k = 2.0 * Z / n
    x = R / sc.physical_constants["Bohr radius"][0]
    f = lambda r: r ** (2 * l + 2) * np.exp(-k * r) * eval_genlaguerre(n - l - 1, 2 * l + 1, k * r) ** 2   # noqa: E731
    inner = quad(f, 0, x, epsabs=0, epsrel=1e-13, limit=400)[0]
    outer = quad(f, x, np.inf, epsabs=0, epsrel=1e-13, limit=400)[0]
    return {"P_inside": inner / (inner + outer), "P_outside": outer / (inner + outer)}


_add("hydrogen_radial_probability",
     name="Probability of finding the electron within a radius", topic="hydrogen atom / wavefunctions",
     purpose="The probability that the electron of the hydrogen-like state n, l lies within distance R of the nucleus.",
     inputs=[_intin("n", "principal quantum number", 1, 30), _intin("l", "orbital quantum number (0 <= l < n)", 0, 29),
             _intin("Z", "atomic number", 1, 137), _phys("R", "length", "radius (pm, nm, m)")],
     outputs=[_o("P_inside", "number", "probability within R"), _o("P_outside", "number", "probability beyond R"),
              _o("R_a0", "number", "R in units of a0")],
     primary="P_inside", conditions=["n >= 1", "0 <= l <= n - 1", "R > 0", "infinitely heavy nucleus"],
     method="P = integral_0^R r^2 R_nl^2 dr / integral_0^inf r^2 R_nl^2 dr (30-digit quadrature)",
     latex=r"P(r < R) = \int_0^R |R_{nl}(r)|^2 r^2\,dr",
     run=radial_probability, verify=_recheck(radial_probability),
     parse=lambda q: _pairs(q, r"(?:hydrogen )?radial probability", {"n": "n", "l": "l", "z": "Z", "r": "R"}, units={"R"}),
     reference=ref_radial_probability,
     wrong=_safe(lambda n, l, Z, R: (lambda o: {**o, "P_inside": _r(1 - math.exp(-2 * Z * R / (n * A0)))})(radial_probability(n, l, Z, R))),
     random_inputs=lambda rng: (lambda n: (lambda l, Z: {"n": n, "l": l, "Z": Z, "R": rng.uniform(0.3, 2.5) * n * n * A0 / Z})(
         rng.randint(0, n - 1), rng.randint(1, 3)))(rng.randint(1, 5)),
     templates=["Radial probability: n = 1, l = 0, Z = 1, R = 52.9177210544 pm"],
     examples=[{"inputs": {"n": 1, "l": 0, "Z": 1, "R": A0}, "outputs": {"P_inside": 1 - 5 * math.exp(-2)}}],
     questions=[{"q": "Radial probability: n = 1, l = 0, Z = 1, R = 52.9177210544 pm", "outputs": {"P_inside": 1 - 5 * math.exp(-2)}},
                {"q": "Radial probability: n = 1, l = 1, Z = 1, R = 50 pm", "outcome": "INVALID_INPUT"},
                {"q": "Radial probability: n = 1, l = 0, Z = 1", "outcome": "NEEDS_INPUT"}])


# ------------------------------------------------------------------------------------------------ angular momentum
def _half(x, name):
    t = 2 * x
    if not math.isfinite(t) or abs(t - round(t)) > 1e-9:
        raise ValueError(f"{name} must be a whole or half-whole number")
    return Fraction(int(round(t)), 2)


def _fact(x):
    return math.factorial(int(x))


def _cg_exact(j1, m1, j2, m2, J, M):
    """The Clebsch-Gordan coefficient as (sign, square as a Fraction) by Racah's formula, exactly."""
    j1, m1, j2, m2, J, M = (_half(v, n) for v, n in ((j1, "j1"), (m1, "m1"), (j2, "j2"), (m2, "m2"), (J, "J"), (M, "M")))
    for j, m, n in ((j1, m1, "1"), (j2, m2, "2"), (J, M, "")):
        if j < 0 or abs(m) > j or (j - m).denominator != 1:
            raise ValueError(f"need j{n} >= 0, |m{n}| <= j{n} and j{n} - m{n} a whole number")
    if m1 + m2 != M or J < abs(j1 - j2) or J > j1 + j2 or (j1 + j2 - J).denominator != 1:
        return 1, Fraction(0)
    pref = Fraction((2 * J + 1).numerator * _fact(J + j1 - j2) * _fact(J - j1 + j2) * _fact(j1 + j2 - J), (2 * J + 1).denominator * _fact(j1 + j2 + J + 1))
    pref *= _fact(J + M) * _fact(J - M) * _fact(j1 - m1) * _fact(j1 + m1) * _fact(j2 - m2) * _fact(j2 + m2)
    s = Fraction(0)
    for k in range(0, int(j1 + j2 - J) + 1):
        args = [k, j1 + j2 - J - k, j1 - m1 - k, j2 + m2 - k, J - j2 + m1 + k, J - j1 - m2 + k]
        if any(a < 0 for a in args):
            continue
        den = 1
        for a in args:
            den *= _fact(a)
        s += Fraction((-1) ** k, den)
    return (1 if s >= 0 else -1), pref * s * s


def clebsch_gordan(j1, m1, j2, m2, J, M):
    sign, sq = _cg_exact(j1, m1, j2, m2, J, M)
    v = sign * math.sqrt(sq) if sq else 0.0
    return {"cg": _r(v), "cg_squared": _r(float(sq)), "cg_squared_exact": f"{sq.numerator}/{sq.denominator}" if sq.denominator != 1 else str(sq.numerator)}


def ref_clebsch_gordan(j1, m1, j2, m2, J, M):
    from sympy import Rational, N
    from sympy.physics.quantum.cg import CG
    R = lambda x: Rational(int(round(2 * x)), 2)          # noqa: E731
    v = CG(R(j1), R(m1), R(j2), R(m2), R(J), R(M)).doit()
    return {"cg": float(N(v, 30)), "cg_squared": float(N(v * v, 30))}


def _random_jm(rng, jmax=4):
    j1 = rng.randint(0, 2 * jmax) / 2
    j2 = rng.randint(0, 2 * jmax) / 2
    J = abs(j1 - j2) + rng.randint(0, int(round(j1 + j2 - abs(j1 - j2))))
    m1 = -j1 + rng.randint(0, int(round(2 * j1)))
    M = -J + rng.randint(0, int(round(2 * J)))
    m2 = M - m1
    if abs(m2) > j2:
        m2 = j2 if m2 > 0 else -j2
        M = m1 + m2
        if abs(M) > J:
            M, m1, m2 = J, J - j2 if abs(J - j2) <= j1 else j1, J - (J - j2 if abs(J - j2) <= j1 else j1)
    return {"j1": j1, "m1": m1, "j2": j2, "m2": m2, "J": J, "M": M}


def _rand_cg(rng):
    for _ in range(100):
        x = _random_jm(rng)
        try:
            if _cg_exact(**x)[1] != 0:
                return x
        except ValueError:
            pass
    return {"j1": 0.5, "m1": 0.5, "j2": 0.5, "m2": -0.5, "J": 1, "M": 0}


_JM_IN = [_numin("j1", "first angular momentum (whole or half-whole)"), _numin("m1", "its projection"),
          _numin("j2", "second angular momentum"), _numin("m2", "its projection"), _numin("J", "total angular momentum"), _numin("M", "total projection")]
_JM_NAMES = {"j1": "j1", "m1": "m1", "j2": "j2", "m2": "m2", "j": "J", "m": "M", "j3": "J", "m3": "M"}
_add("clebsch_gordan",
     name="Clebsch-Gordan coefficient", topic="angular momentum / addition of angular momenta",
     purpose="The coefficient <j1 m1; j2 m2 | J M> for coupling two angular momenta (Condon-Shortley convention), with its exact square.",
     inputs=_JM_IN,
     outputs=[_o("cg", "number", "Clebsch-Gordan coefficient"), _o("cg_squared", "number", "its square (a probability)"),
              {"symbol": "cg_squared_exact", "kind": "number", "meaning": "the square as an exact fraction"}],
     primary="cg", conditions=["each j >= 0 and whole or half-whole", "|m| <= j with j - m whole"],
     method="Racah's formula in exact rational arithmetic; zero unless m1 + m2 = M and |j1 - j2| <= J <= j1 + j2",
     latex=r"\langle j_1 m_1; j_2 m_2 | J M\rangle",
     run=clebsch_gordan, verify=_recheck(clebsch_gordan),
     parse=lambda q: _pairs(q, r"clebsch[- ]gordan(?: coefficient)?", {"j1": "j1", "m1": "m1", "j2": "j2", "m2": "m2", "j": "J", "m": "M"}),
     reference=ref_clebsch_gordan,
     wrong=_safe(lambda j1, m1, j2, m2, J, M: (lambda o: {**o, "cg": _r(o["cg"] / math.sqrt(2 * J + 1)) if o["cg"] else 1.0})(clebsch_gordan(j1, m1, j2, m2, J, M))),
     random_inputs=_rand_cg,
     templates=["Clebsch-Gordan coefficient: j1 = 1/2, m1 = 1/2, j2 = 1/2, m2 = -1/2, J = 1, M = 0"],
     examples=[{"inputs": {"j1": 0.5, "m1": 0.5, "j2": 0.5, "m2": -0.5, "J": 1, "M": 0}, "outputs": {"cg": math.sqrt(0.5), "cg_squared_exact": "1/2"}},
               {"inputs": {"j1": 0.5, "m1": -0.5, "j2": 0.5, "m2": 0.5, "J": 0, "M": 0}, "outputs": {"cg": -math.sqrt(0.5)}},
               {"inputs": {"j1": 1, "m1": 1, "j2": 1, "m2": -1, "J": 2, "M": 0}, "outputs": {"cg": math.sqrt(1 / 6)}}],
     questions=[{"q": "Clebsch-Gordan coefficient: j1 = 0.5, m1 = 0.5, j2 = 0.5, m2 = -0.5, J = 1, M = 0", "outputs": {"cg": math.sqrt(0.5)}},
                {"q": "Clebsch-Gordan coefficient: j1 = 1, m1 = 2, j2 = 1, m2 = 0, J = 2, M = 2", "outcome": "INVALID_INPUT"},
                {"q": "Clebsch-Gordan coefficient: j1 = 1, m1 = 1, j2 = 1", "outcome": "NEEDS_INPUT"}])


def wigner_3j(j1, m1, j2, m2, J, M):
    """(j1 j2 J; m1 m2 M) = (-1)^(j1 - j2 - M) / sqrt(2J + 1) <j1 m1; j2 m2 | J, -M>."""
    sign, sq = _cg_exact(j1, m1, j2, m2, J, -M)
    if sq == 0:
        return {"w3j": 0.0, "w3j_squared": 0.0}
    e = int(_half(j1, "j1") - _half(j2, "j2") - _half(M, "M"))
    sq3 = sq / (2 * _half(J, "J") + 1)
    return {"w3j": _r((-1) ** e * sign * math.sqrt(sq3)), "w3j_squared": _r(float(sq3))}


def ref_wigner_3j(j1, m1, j2, m2, J, M):
    from sympy import Rational, N
    from sympy.physics.wigner import wigner_3j as w3
    R = lambda x: Rational(int(round(2 * x)), 2)          # noqa: E731
    v = w3(R(j1), R(j2), R(J), R(m1), R(m2), R(M))
    return {"w3j": float(N(v, 30))}


def _rand_3j(rng):
    x = _rand_cg(rng)
    return {**x, "M": -x["M"]}


_add("wigner_3j_symbol",
     name="Wigner 3j symbol", topic="angular momentum / addition of angular momenta",
     purpose="The Wigner 3j symbol (j1 j2 J; m1 m2 M), nonzero only when m1 + m2 + M = 0.",
     inputs=_JM_IN,
     outputs=[_o("w3j", "number", "the 3j symbol"), _o("w3j_squared", "number", "its square")],
     primary="w3j", conditions=["each j >= 0 and whole or half-whole", "|m| <= j with j - m whole"],
     method="from the exact Clebsch-Gordan coefficient: (-1)^(j1 - j2 - M) <j1 m1; j2 m2 | J, -M> / sqrt(2J + 1)",
     latex=r"\begin{pmatrix} j_1 & j_2 & J \\ m_1 & m_2 & M \end{pmatrix}",
     run=wigner_3j, verify=_recheck(wigner_3j),
     parse=lambda q: _pairs(q, r"wigner 3j(?: symbol)?|3j symbol", {"j1": "j1", "m1": "m1", "j2": "j2", "m2": "m2", "j3": "J", "m3": "M", "j": "J", "m": "M"}),
     reference=ref_wigner_3j,
     wrong=_safe(lambda j1, m1, j2, m2, J, M: (lambda o: {**o, "w3j": _r(-o["w3j"]) if o["w3j"] else 1.0})(wigner_3j(j1, m1, j2, m2, J, M))),
     random_inputs=_rand_3j,
     templates=["Wigner 3j symbol: j1 = 1, m1 = 1, j2 = 1, m2 = -1, j3 = 0, m3 = 0"],
     examples=[{"inputs": {"j1": 1, "m1": 1, "j2": 1, "m2": -1, "J": 0, "M": 0}, "outputs": {"w3j": math.sqrt(1 / 3)}},
               {"inputs": {"j1": 0.5, "m1": 0.5, "j2": 0.5, "m2": -0.5, "J": 1, "M": 0}, "outputs": {"w3j": math.sqrt(1 / 6)}}],
     questions=[{"q": "Wigner 3j symbol: j1 = 1, m1 = 1, j2 = 1, m2 = -1, j3 = 0, m3 = 0", "outputs": {"w3j": math.sqrt(1 / 3)}},
                {"q": "Wigner 3j symbol: j1 = 1, m1 = 1.5, j2 = 1, m2 = -1, j3 = 0, m3 = 0", "outcome": "INVALID_INPUT"},
                {"q": "Wigner 3j symbol: j1 = 1, m1 = 1, j2 = 1", "outcome": "NEEDS_INPUT"}])


def lande_zeeman(L, S, J, mJ, B):
    L, S, J, mJ = _half(L, "L"), _half(S, "S"), _half(J, "J"), _half(mJ, "mJ")
    if L < 0 or S < 0 or L.denominator != 1:
        raise ValueError("L must be a whole number >= 0 and S >= 0")
    if J <= 0 or J < abs(L - S) or J > L + S or (L + S - J).denominator != 1:
        raise ValueError("J must lie between |L - S| and L + S in whole steps, and be positive")
    if abs(mJ) > J or (J - mJ).denominator != 1:
        raise ValueError("mJ must lie between -J and J in whole steps")
    g = 1 + (J * (J + 1) + S * (S + 1) - L * (L + 1)) / (2 * J * (J + 1))
    dE = float(g * mJ) * MUB * B
    return _out(g_J=float(g), dE=dE, dE_eV=dE / EV, g_J_exact=f"{g.numerator}/{g.denominator}" if g.denominator != 1 else str(g.numerator))


def ref_lande_zeeman(L, S, J, mJ, B):
    # <J, M=J | L_z + 2 S_z | J, M=J> / J from the coupled state built of Clebsch-Gordan coefficients (sympy)
    from sympy import Rational, N
    from sympy.physics.quantum.cg import CG
    from scipy import constants as sc
    R = lambda x: Rational(int(round(2 * x)), 2)          # noqa: E731
    Lr, Sr, Jr = R(L), R(S), R(J)
    tot = 0
    mL = -Lr
    while mL <= Lr:
        mS = Jr - mL
        if abs(mS) <= Sr:
            c = CG(Lr, mL, Sr, mS, Jr, Jr).doit()
            tot += c * c * (mL + 2 * mS)
        mL += 1
    g = float(N(tot / Jr, 30))
    dE = g * mJ * sc.physical_constants["Bohr magneton"][0] * B
    return {"g_J": g, "dE_eV": dE / sc.e}


def _rand_lande(rng):
    L = rng.randint(0, 3)
    S = rng.randint(0, 4) / 2
    Js = [abs(L - S) + k for k in range(int(round(L + S - abs(L - S))) + 1) if abs(L - S) + k > 0]
    if not Js:
        return {"L": 1, "S": 0.5, "J": 1.5, "mJ": 0.5, "B": rng.uniform(0.1, 5)}
    J = rng.choice(Js)
    mJ = -J + rng.randint(0, int(round(2 * J)))
    return {"L": L, "S": S, "J": J, "mJ": mJ, "B": rng.uniform(0.1, 5)}


_add("lande_g_zeeman",
     name="Landé g factor and anomalous Zeeman shift", topic="hydrogen atom / Zeeman effect",
     purpose="The Landé g factor of a level L, S, J (with g_s = 2) and the energy shift g_J mu_B B mJ in a weak magnetic field.",
     inputs=[_numin("L", "orbital angular momentum quantum number"), _numin("S", "spin quantum number"),
             _numin("J", "total angular momentum quantum number"), _numin("mJ", "magnetic quantum number"),
             _phys("B", "magnetic_field", "magnetic field (T)")],
     outputs=[_o("g_J", "number", "Landé g factor"), _o("dE", "energy", "Zeeman energy shift"), _o("dE_eV", "energy_ev", "the shift in eV"),
              {"symbol": "g_J_exact", "kind": "number", "meaning": "g_J as an exact fraction"}],
     primary="g_J", conditions=["|L - S| <= J <= L + S, J > 0", "|mJ| <= J", "weak field (LS coupling holds)"],
     method="g_J = 1 + [J(J+1) + S(S+1) - L(L+1)] / [2 J(J+1)], dE = g_J mu_B B mJ",
     latex=r"g_J = 1 + \frac{J(J+1) + S(S+1) - L(L+1)}{2J(J+1)},\quad \Delta E = g_J \mu_B B m_J",
     run=lande_zeeman, verify=_recheck(lande_zeeman),
     parse=lambda q: _pairs(q, r"land[eé] g factor|zeeman shift|anomalous zeeman(?: shift)?", {"l": "L", "s": "S", "j": "J", "mj": "mJ", "b": "B"}, units={"B"}),
     reference=ref_lande_zeeman,
     wrong=_safe(lambda L, S, J, mJ, B: (lambda o: {**o, "g_J": _r(1 + (J * (J + 1) - S * (S + 1) + L * (L + 1)) / (2 * J * (J + 1)))})(lande_zeeman(L, S, J, mJ, B))),
     random_inputs=_rand_lande,
     templates=["Landé g factor: L = 1, S = 1/2, J = 3/2, mJ = 1/2, B = 1 T"],
     examples=[{"inputs": {"L": 1, "S": 0.5, "J": 1.5, "mJ": 0.5, "B": 1.0}, "outputs": {"g_J": 4 / 3, "dE": 4 / 3 * 0.5 * MUB}},
               {"inputs": {"L": 0, "S": 0.5, "J": 0.5, "mJ": 0.5, "B": 2.0}, "outputs": {"g_J": 2.0}}],
     questions=[{"q": "Landé g factor: L = 1, S = 0.5, J = 1.5, mJ = 0.5, B = 1 T", "outputs": {"g_J": 4 / 3}},
                {"q": "Landé g factor: L = 1, S = 0.5, J = 2.5, mJ = 0.5, B = 1 T", "outcome": "INVALID_INPUT"},
                {"q": "Landé g factor: L = 1, S = 0.5, J = 1.5", "outcome": "NEEDS_INPUT"}])


def wigner_small_d(j, mp, m, beta):
    jj, mm, mmp = _half(j, "j"), _half(m, "m"), _half(mp, "mp")
    if jj < 0 or abs(mm) > jj or abs(mmp) > jj or (jj - mm).denominator != 1 or (jj - mmp).denominator != 1:
        raise ValueError("need j >= 0, |m|, |m'| <= j with j - m and j - m' whole")
    if jj > 20:
        raise ValueError("j must be at most 20")
    import mpmath
    with mpmath.workdps(40):
        b = mpmath.mpf(beta)
        cb, sb = mpmath.cos(b / 2), mpmath.sin(b / 2)
        jp, jm, mpp, mpm = int(jj + mmp), int(jj - mmp), int(jj + mm), int(jj - mm)
        pref = mpmath.sqrt(mpmath.factorial(jp) * mpmath.factorial(jm) * mpmath.factorial(mpp) * mpmath.factorial(mpm))
        tot = mpmath.mpf(0)
        dmm = int(mmp - mm)
        for s in range(0, 2 * int(jj) + 2):
            a = [int(jj + mm) - s, s, dmm + s, int(jj - mmp) - s]
            if any(x < 0 for x in a):
                continue
            tot += (-1) ** (dmm + s) * cb ** (int(2 * jj) + int(mm - mmp) - 2 * s) * sb ** (dmm + 2 * s) / (
                mpmath.factorial(a[0]) * mpmath.factorial(a[1]) * mpmath.factorial(a[2]) * mpmath.factorial(a[3]))
        d = float(pref * tot)
    d = _snap(d)
    return _out(d=d, probability=d * d)


def ref_wigner_small_d(j, mp, m, beta):
    # <j m'| exp(-i beta J_y) |j m> with qutip's spin-j matrices and scipy's matrix exponential
    import numpy as np
    import qutip
    from scipy.linalg import expm
    Jy = qutip.jmat(j, "y").full()
    U = expm(-1j * beta * Jy)
    idx = lambda mm: int(round(j - mm))                   # noqa: E731  qutip orders m = j, j-1, ..., -j
    d = U[idx(mp), idx(m)]
    return {"d": float(d.real), "probability": float(abs(d) ** 2)}


def _rand_wd(rng):
    j = rng.randint(1, 6) / 2
    return {"j": j, "mp": -j + rng.randint(0, int(round(2 * j))), "m": -j + rng.randint(0, int(round(2 * j))), "beta": rng.uniform(0.3, 2.8)}


_add("spin_rotation_wigner_d",
     name="Spin rotation amplitude (Wigner small-d matrix)", topic="spin and qubits / rotations of angular momentum states",
     purpose="The amplitude d^j_{m'm}(beta) = <j m'| exp(-i beta J_y/hbar) |j m> and the probability of finding m' after rotating |j m> by beta about y.",
     inputs=[_numin("j", "angular momentum quantum number (whole or half-whole, at most 20)"), _numin("mp", "final projection m'"),
             _numin("m", "initial projection m"), _phys("beta", "angle", "rotation angle about the y axis (deg or rad)")],
     outputs=[_o("d", "number", "Wigner small-d matrix element"), _o("probability", "number", "probability |d|^2 of measuring m' along the original axis")],
     primary="d", conditions=["|m|, |m'| <= j", "j <= 20"],
     method="Wigner's formula summed in 40-digit arithmetic",
     latex=r"d^j_{m'm}(\beta) = \langle j m'|e^{-i\beta J_y/\hbar}|j m\rangle",
     run=wigner_small_d, verify=_recheck(wigner_small_d),
     parse=lambda q: _pairs(q, r"(?:wigner small[- ]d|spin rotation)(?: amplitude| matrix element)?", {"j": "j", "mp": "mp", "m": "m", "beta": "beta"}, units={"beta"}),
     reference=ref_wigner_small_d,
     wrong=_safe(lambda j, mp, m, beta: (lambda o: {**o, "d": _r(wigner_small_d(j, mp, m, 2 * beta)["d"]) if abs(math.sin(beta / 2)) > 1e-6 else 2.0})(wigner_small_d(j, mp, m, beta))),
     random_inputs=_rand_wd,
     templates=["Spin rotation amplitude: j = 1/2, mp = 1/2, m = 1/2, beta = 90 deg"],
     examples=[{"inputs": {"j": 0.5, "mp": 0.5, "m": 0.5, "beta": math.pi / 2}, "outputs": {"d": math.sqrt(0.5), "probability": 0.5}},
               {"inputs": {"j": 1, "mp": 1, "m": 0, "beta": math.pi / 3}, "outputs": {"d": -math.sin(math.pi / 3) / math.sqrt(2)}},
               {"inputs": {"j": 1, "mp": 0, "m": 0, "beta": math.pi / 2}, "outputs": {"d": 0.0}}],
     questions=[{"q": "Spin rotation amplitude: j = 0.5, mp = 0.5, m = 0.5, beta = 90 deg", "outputs": {"probability": 0.5}},
                {"q": "Spin rotation amplitude: j = 1, mp = 2, m = 0, beta = 90 deg", "outcome": "INVALID_INPUT"},
                {"q": "Spin rotation amplitude: j = 1, mp = 1, m = 0", "outcome": "NEEDS_INPUT"}])

# ================================================================================================ quantum wells and tunnelling
def _ref_box_eigs(count):
    """The first eigenvalues of -d^2/dx^2 on [0, 1] with psi(0) = psi(1) = 0 by Chebyshev spectral collocation (numpy):
    no sine functions assumed."""
    import numpy as np
    N = 72
    k = np.arange(N + 1)
    y = np.cos(np.pi * k / N)
    c = np.ones(N + 1)
    c[0] = c[-1] = 2.0
    c = c * (-1.0) ** k
    X = np.tile(y, (N + 1, 1)).T
    dX = X - X.T
    D = np.outer(c, 1 / c) / (dX + np.eye(N + 1))
    D = D - np.diag(D.sum(axis=1))
    D2 = (D @ D)[1:N, 1:N]
    ev = np.sort(np.linalg.eigvals(-4.0 * D2).real)
    return ev[:count]


def infinite_well_levels(m, L, n):
    if m <= 0 or L <= 0:
        raise ValueError("the mass and the width must be positive")
    _quantum_n(n)
    E = (n * math.pi * HBAR / L) ** 2 / (2 * m)
    E1 = (math.pi * HBAR / L) ** 2 / (2 * m)
    return _out(E=E, E_eV=E / EV, E1_eV=E1 / EV, p=n * math.pi * HBAR / L, wavelength=2 * L / n)


def ref_infinite_well_levels(m, L, n):
    from scipy import constants as sc
    lam = _ref_box_eigs(n)
    E = sc.hbar ** 2 * lam[n - 1] / (2 * m * L * L)
    return {"E_eV": E / sc.e, "E1_eV": sc.hbar ** 2 * lam[0] / (2 * m * L * L) / sc.e}


_add("infinite_well_levels",
     name="Energy levels of a particle in a box", topic="quantum wells / infinite square well",
     purpose="Energy E_n = n^2 h^2/(8 m L^2), momentum and de Broglie wavelength of level n of a particle in a one-dimensional box of width L.",
     inputs=[_phys("m", "mass", "particle mass"), _phys("L", "length", "width of the box"), _intin("n", "quantum number (1, 2, 3, ...)", 1, 100000)],
     outputs=[_o("E", "energy", "energy of level n"), _o("E_eV", "energy_ev", "the same in eV"), _o("E1_eV", "energy_ev", "ground-state energy"),
              _o("p", "momentum", "magnitude of the momentum n pi hbar/L"), _o("wavelength", "length", "de Broglie wavelength 2L/n")],
     primary="E_eV", conditions=["m > 0", "L > 0", "n >= 1", "infinitely high walls"],
     method="E_n = (n pi hbar/L)^2/(2m) = n^2 h^2/(8 m L^2)",
     latex=r"E_n = \frac{n^2 h^2}{8 m L^2}",
     run=infinite_well_levels, verify=_recheck(infinite_well_levels),
     parse=lambda q: _pairs(q, r"particle in a box(?: energy)?|infinite (?:square )?well(?: energy| levels?)?", {"m": "m", "l": "L", "n": "n"}, units={"m", "L"}),
     reference=ref_infinite_well_levels,
     wrong=_safe(lambda m, L, n: (lambda o: {**o, "E_eV": _r(o["E_eV"] / n)})(infinite_well_levels(m, L, n)) if n > 1 else
                 (lambda o: {**o, "E_eV": _r(o["E_eV"] * 4), "E1_eV": _r(o["E1_eV"] * 4)})(infinite_well_levels(m, L, n))),
     random_inputs=lambda rng: {"m": _lu(rng, 1e-31, 1e-26), "L": _lu(rng, 1e-11, 1e-7), "n": rng.randint(1, 8)},
     templates=["Particle in a box: m = 9.109e-31 kg, L = 1 nm, n = 1"],
     examples=[{"inputs": {"m": ME, "L": 1e-9, "n": 1}, "outputs": {"E": H * H / (8 * ME * 1e-18)}},
               {"inputs": {"m": ME, "L": 1e-10, "n": 2}, "outputs": {"E": 4 * H * H / (8 * ME * 1e-20), "wavelength": 1e-10}}],
     questions=[{"q": "Particle in a box: m = 9.1093837139e-31 kg, L = 1 nm, n = 1", "outputs": {"E": H * H / (8 * ME * 1e-18)}},
                {"q": "Particle in a box: m = 9.1e-31 kg, L = 1 nm, n = 0", "outcome": "INVALID_INPUT"},
                {"q": "Particle in a box: m = 9.1e-31 kg, L = 1 nm", "outcome": "NEEDS_INPUT"}])


def infinite_well_transition(m, L, n_upper, n_lower):
    if m <= 0 or L <= 0:
        raise ValueError("the mass and the width must be positive")
    _quantum_n(n_lower, "n_lower")
    if n_upper <= n_lower:
        raise ValueError("the upper level must lie above the lower level")
    E = (n_upper - n_lower) * (n_upper + n_lower) * (math.pi * HBAR / L) ** 2 / (2 * m)
    return _out(E_photon_eV=E / EV, lam=H * C / E, lambda_nm=H * C / E * 1e9, f=E / H)


def ref_infinite_well_transition(m, L, n_upper, n_lower):
    from scipy import constants as sc
    lam = _ref_box_eigs(n_upper)
    E = sc.hbar ** 2 * (lam[n_upper - 1] - lam[n_lower - 1]) / (2 * m * L * L)
    return {"E_photon_eV": E / sc.e, "lambda_nm": sc.h * sc.c / E * 1e9}


_add("infinite_well_transition",
     name="Photon from a particle-in-a-box transition", topic="quantum wells / infinite square well",
     purpose="Energy, wavelength and frequency of the photon emitted when a particle in a box of width L drops from n_upper to n_lower.",
     inputs=[_phys("m", "mass", "particle mass"), _phys("L", "length", "width of the box"),
             _intin("n_upper", "upper level", 2, 100000), _intin("n_lower", "lower level", 1, 99999)],
     outputs=[_o("E_photon_eV", "energy_ev", "photon energy"), _o("lam", "length", "wavelength"), _o("lambda_nm", "length_nm", "wavelength in nm"),
              _o("f", "frequency", "frequency")],
     primary="lambda_nm", conditions=["n_upper > n_lower >= 1"], method="E = (n_u^2 - n_l^2) h^2/(8 m L^2), lambda = h c/E",
     latex=r"\Delta E = \frac{(n_u^2 - n_\ell^2)\,h^2}{8 m L^2} = \frac{hc}{\lambda}",
     run=infinite_well_transition, verify=_recheck(infinite_well_transition),
     parse=lambda q: _pairs(q, r"(?:particle in a box|infinite well) transition", {"m": "m", "l": "L", "n_upper": "n_upper", "n_lower": "n_lower"}, units={"m", "L"}),
     reference=ref_infinite_well_transition,
     wrong=_safe(lambda m, L, n_upper, n_lower: (lambda E: _out(E_photon_eV=E / EV, lam=H * C / E, lambda_nm=H * C / E * 1e9, f=E / H))(
         (n_upper - n_lower) ** 2 * (math.pi * HBAR / L) ** 2 / (2 * m))),
     random_inputs=lambda rng: (lambda lo: {"m": _lu(rng, 1e-31, 1e-26), "L": _lu(rng, 1e-10, 1e-7), "n_lower": lo, "n_upper": lo + rng.randint(1, 5)})(rng.randint(1, 4)),
     templates=["Particle in a box transition: m = 9.109e-31 kg, L = 1 nm, n_upper = 2, n_lower = 1"],
     examples=[{"inputs": {"m": ME, "L": 1e-9, "n_upper": 2, "n_lower": 1}, "outputs": {"E_photon_eV": 3 * H * H / (8 * ME * 1e-18) / EV}}],
     questions=[{"q": "Particle in a box transition: m = 9.1093837139e-31 kg, L = 1 nm, n_upper = 2, n_lower = 1", "outputs": {"E_photon_eV": 3 * H * H / (8 * ME * 1e-18) / EV}},
                {"q": "Particle in a box transition: m = 9.1e-31 kg, L = 1 nm, n_upper = 1, n_lower = 2", "outcome": "INVALID_INPUT"},
                {"q": "Particle in a box transition: m = 9.1e-31 kg, L = 1 nm, n_upper = 2", "outcome": "NEEDS_INPUT"}])


def infinite_well_probability(n, L, x1, x2):
    _quantum_n(n)
    if L <= 0 or not (0 <= x1 < x2 <= L):
        raise ValueError("need 0 <= x1 < x2 <= L")
    u = n * math.pi * (x2 - x1) / L
    cc = n * math.pi * (x1 + x2) / L
    ums = u - math.sin(u) if u > 1e-3 else u ** 3 / 6 - u ** 5 / 120 + u ** 7 / 5040
    P = (ums + 2 * math.sin(cc / 2) ** 2 * math.sin(u)) / (n * math.pi)
    return _out(P=P, classical=(x2 - x1) / L)


def ref_infinite_well_probability(n, L, x1, x2):
    import numpy as np
    from scipy.integrate import quad
    f = lambda x: 2 / L * np.sin(n * np.pi * x / L) ** 2   # noqa: E731
    pts = [x1 + (x2 - x1) * k / 8 for k in range(1, 8)]
    return {"P": quad(f, x1, x2, epsabs=0, epsrel=1e-13, limit=400, points=pts)[0]}


_add("infinite_well_probability",
     name="Probability of finding a particle in part of a box", topic="quantum wells / infinite square well",
     purpose="The probability that a particle in level n of a box of width L is found between x1 and x2.",
     inputs=[_intin("n", "quantum number", 1, 100000), _phys("L", "length", "width of the box"),
             _phys("x1", "length", "left end of the interval (from the wall)"), _phys("x2", "length", "right end of the interval")],
     outputs=[_o("P", "number", "probability"), _o("classical", "number", "classical (uniform) probability (x2 - x1)/L")],
     primary="P", conditions=["0 <= x1 < x2 <= L", "n >= 1"],
     method="P = (x2 - x1)/L - [sin(2 n pi x2/L) - sin(2 n pi x1/L)]/(2 n pi), evaluated without cancellation",
     latex=r"P = \int_{x_1}^{x_2} \frac{2}{L}\sin^2\frac{n\pi x}{L}\,dx",
     run=infinite_well_probability, verify=_recheck(infinite_well_probability),
     parse=lambda q: _pairs(q, r"(?:particle in a box|infinite well) probability", {"n": "n", "l": "L", "x1": "x1", "x2": "x2"}, units={"L", "x1", "x2"}),
     reference=ref_infinite_well_probability,
     wrong=_safe(lambda n, L, x1, x2: (lambda o: {**o, "P": _r((x2 - x1) / L)})(infinite_well_probability(n, L, x1, x2))),
     random_inputs=lambda rng: (lambda L, a, b: {"n": rng.randint(1, 6), "L": L, "x1": min(a, b) * L, "x2": max(a, b) * L})(
         _lu(rng, 1e-10, 1e-6), rng.uniform(0, 1), rng.uniform(0, 1)),
     templates=["Particle in a box probability: n = 1, L = 1 nm, x1 = 0.25 nm, x2 = 0.75 nm"],
     examples=[{"inputs": {"n": 1, "L": 1e-9, "x1": 0.25e-9, "x2": 0.75e-9}, "outputs": {"P": 0.5 + 1 / math.pi}},
               {"inputs": {"n": 2, "L": 1.0, "x1": 0.0, "x2": 0.5}, "outputs": {"P": 0.5}}],
     questions=[{"q": "Particle in a box probability: n = 1, L = 1 nm, x1 = 0.25 nm, x2 = 0.75 nm", "outputs": {"P": 0.5 + 1 / math.pi}},
                {"q": "Particle in a box probability: n = 1, L = 1 nm, x1 = 0.75 nm, x2 = 0.25 nm", "outcome": "INVALID_INPUT"},
                {"q": "Particle in a box probability: n = 1, L = 1 nm, x1 = 0.25 nm", "outcome": "NEEDS_INPUT"}])


def infinite_well_expectations(n, L):
    _quantum_n(n)
    if L <= 0:
        raise ValueError("the width must be positive")
    x2 = L * L * (1 / 3 - 1 / (2 * (n * math.pi) ** 2))
    dx = math.sqrt(x2 - L * L / 4)
    dp = n * math.pi * HBAR / L
    return _out(x_mean=L / 2, dx=dx, dx_over_L=dx / L, dp=dp, product_over_hbar=dx * dp / HBAR)


def ref_infinite_well_expectations(n, L):
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    psi = lambda x: np.sqrt(2 / L) * np.sin(n * np.pi * x / L)        # noqa: E731
    h = 1e-20 * L
    dpsi = lambda x: (np.sqrt(2 / L) * np.sin(n * np.pi * (x + 1j * h) / L)).imag / h   # noqa: E731  complex-step derivative
    o = dict(epsabs=0, epsrel=1e-13, limit=400, points=[L * k / (2 * n) for k in range(1, 2 * n)] if n > 1 else None)
    xm = quad(lambda x: x * psi(x) ** 2, 0, L, **o)[0]
    x2 = quad(lambda x: x * x * psi(x) ** 2, 0, L, **o)[0]
    p2 = sc.hbar ** 2 * quad(lambda x: dpsi(x) ** 2, 0, L, **o)[0]
    dx = np.sqrt(x2 - xm * xm)
    return {"x_mean": xm, "dx_over_L": dx / L, "product_over_hbar": dx * np.sqrt(p2) / sc.hbar}


_add("infinite_well_uncertainty",
     name="Position and momentum spread in a box", topic="quantum wells / expectation values and uncertainty",
     purpose="<x>, the position spread dx, the momentum spread dp and the product dx dp of level n of a particle in a box of width L.",
     inputs=[_intin("n", "quantum number", 1, 100000), _phys("L", "length", "width of the box")],
     outputs=[_o("x_mean", "length", "<x> (the centre)"), _o("dx", "length", "position spread"), _o("dx_over_L", "number", "dx/L"),
              _o("dp", "momentum", "momentum spread n pi hbar/L"), _o("product_over_hbar", "number", "dx dp / hbar (at least 1/2)")],
     primary="dx_over_L", conditions=["n >= 1", "L > 0"],
     method="<x^2> = L^2 (1/3 - 1/(2 n^2 pi^2)), dx = sqrt(<x^2> - L^2/4), dp = n pi hbar/L",
     latex=r"\Delta x = L\sqrt{\frac{1}{12} - \frac{1}{2n^2\pi^2}},\quad \Delta p = \frac{n\pi\hbar}{L}",
     run=infinite_well_expectations, verify=_recheck(infinite_well_expectations),
     parse=lambda q: _pairs(q, r"(?:particle in a box|infinite well) uncertainty", {"n": "n", "l": "L"}, units={"L"}),
     reference=ref_infinite_well_expectations,
     wrong=_safe(lambda n, L: (lambda o: {**o, "dx_over_L": _r(math.sqrt(1 / 12)), "product_over_hbar": _r(math.sqrt(1 / 12) * n * math.pi)})(infinite_well_expectations(n, L))),
     random_inputs=lambda rng: {"n": rng.randint(1, 8), "L": _lu(rng, 1e-10, 1e-6)},
     templates=["Particle in a box uncertainty: n = 1, L = 1 nm"],
     examples=[{"inputs": {"n": 1, "L": 1.0}, "outputs": {"dx_over_L": math.sqrt(1 / 12 - 1 / (2 * math.pi ** 2)), "x_mean": 0.5}}],
     questions=[{"q": "Particle in a box uncertainty: n = 1, L = 1 m", "outputs": {"dx_over_L": math.sqrt(1 / 12 - 1 / (2 * math.pi ** 2))}},
                {"q": "Particle in a box uncertainty: n = 0, L = 1 nm", "outcome": "INVALID_INPUT"},
                {"q": "Particle in a box uncertainty: n = 1", "outcome": "NEEDS_INPUT"}])


def _ref_ho_dvr():
    """Colbert-Miller sinc-DVR of H = -1/2 d^2/dx^2 + x^2/2 (dimensionless): eigenvalues and <x^2> of each state."""
    import numpy as np
    from scipy.linalg import eigh
    d = 0.1
    x = np.arange(-15.0, 15.0 + d / 2, d)
    i = np.arange(len(x))
    D = i[:, None] - i[None, :]
    with np.errstate(divide="ignore"):
        T = np.where(D == 0, np.pi ** 2 / 3, 2.0 * (-1.0) ** D / np.where(D == 0, 1, D) ** 2) / (2 * d * d)
    w, V = eigh(T + np.diag(0.5 * x * x))
    return w, (V ** 2 * (x ** 2)[:, None]).sum(axis=0)


def ho_levels(m, omega, n):
    if m <= 0 or omega <= 0:
        raise ValueError("the mass and the angular frequency must be positive")
    if n < 0:
        raise ValueError("n must be a whole number >= 0")
    E = (n + 0.5) * HBAR * omega
    s = math.sqrt(HBAR / (m * omega))
    return _out(E=E, E_eV=E / EV, E0_eV=0.5 * HBAR * omega / EV, spacing_eV=HBAR * omega / EV,
                amplitude=s * math.sqrt(2 * n + 1), dx=s * math.sqrt(n + 0.5))


def ref_ho_levels(m, omega, n):
    import numpy as np
    from scipy import constants as sc
    w, x2 = _ref_ho_dvr()
    E = w[n] * sc.hbar * omega
    s = np.sqrt(sc.hbar / (m * omega))
    return {"E_eV": E / sc.e, "spacing_eV": (w[n + 1] - w[n]) * sc.hbar * omega / sc.e, "amplitude": np.sqrt(2 * E / (m * omega ** 2)),
            "dx": s * np.sqrt(x2[n])}


_add("harmonic_oscillator_levels",
     name="Quantum harmonic oscillator levels", topic="quantum wells / harmonic oscillator",
     purpose="Energy (n + 1/2) hbar omega, zero-point energy, level spacing, classical amplitude and position spread of level n.",
     inputs=[_phys("m", "mass", "mass"), _phys("omega", "angular_frequency", "angular frequency"), _intin("n", "quantum number (0, 1, 2, ...)", 0, 100000)],
     outputs=[_o("E", "energy", "energy of level n"), _o("E_eV", "energy_ev", "the same in eV"), _o("E0_eV", "energy_ev", "zero-point energy"),
              _o("spacing_eV", "energy_ev", "level spacing hbar omega"), _o("amplitude", "length", "classical turning point"),
              _o("dx", "length", "position spread sqrt(<x^2>)")],
     primary="E_eV", conditions=["m > 0", "omega > 0", "n >= 0"],
     method="E_n = (n + 1/2) hbar omega, A = sqrt((2n + 1) hbar/(m omega)), dx = sqrt((n + 1/2) hbar/(m omega))",
     latex=r"E_n = \left(n + \tfrac12\right)\hbar\omega",
     run=ho_levels, verify=_recheck(ho_levels),
     parse=lambda q: _pairs(q, r"(?:quantum )?harmonic oscillator(?: levels?| energy)?", {"m": "m", "omega": "omega", "n": "n"}, units={"m", "omega"}),
     reference=ref_ho_levels,
     wrong=_safe(lambda m, omega, n: (lambda o: {**o, "E_eV": _r(n * HBAR * omega / EV) if n else 1.0})(ho_levels(m, omega, n))),
     random_inputs=lambda rng: {"m": _lu(rng, 1e-31, 1e-25), "omega": _lu(rng, 1e12, 1e16), "n": rng.randint(0, 12)},
     templates=["Harmonic oscillator: m = 1.67e-27 kg, omega = 5e14 rad/s, n = 0"],
     examples=[{"inputs": {"m": MP, "omega": 1e14, "n": 0}, "outputs": {"E": 0.5 * HBAR * 1e14}},
               {"inputs": {"m": ME, "omega": 2e15, "n": 3}, "outputs": {"E": 3.5 * HBAR * 2e15, "amplitude": math.sqrt(7 * HBAR / (ME * 2e15))}}],
     questions=[{"q": "Harmonic oscillator: m = 1.67262192595e-27 kg, omega = 1e14 rad/s, n = 0", "outputs": {"E": 0.5 * HBAR * 1e14}},
                {"q": "Harmonic oscillator: m = 1e-27 kg, omega = -1e14 rad/s, n = 0", "outcome": "INVALID_INPUT"},
                {"q": "Harmonic oscillator: m = 1e-27 kg, n = 0", "outcome": "NEEDS_INPUT"}])


def ho_forbidden(n):
    if n < 0 or n > 60:
        raise ValueError("n must be a whole number from 0 to 60")
    import mpmath
    h = [[1], [0, 2]]                                    # Hermite polynomial coefficients (physicists'), exact integers
    for k in range(1, n):
        a, b = h[k], h[k - 1]
        nxt = [0] * (k + 2)
        for i, c in enumerate(a):
            nxt[i + 1] += 2 * c
        for i, c in enumerate(b):
            nxt[i] -= 2 * k * c
        h.append(nxt)
    p = h[n]
    sq = [0] * (2 * len(p) - 1)
    for i, a in enumerate(p):
        for j, b in enumerate(p):
            sq[i + j] += a * b
    with mpmath.workdps(50):
        A2 = mpmath.mpf(2 * n + 1)                      # turning point xi_t^2 = 2n + 1
        inside = mpmath.mpf(0)
        for k in range(0, len(sq), 2):                   # only even powers survive the symmetric integral
            if sq[k]:
                inside += sq[k] * mpmath.gammainc(mpmath.mpf(k + 1) / 2, 0, A2)
        norm = mpmath.sqrt(mpmath.pi) * mpmath.mpf(2) ** n * mpmath.factorial(n)
        P_in = inside / norm
        P_out = 1 - P_in
    return _out(P_forbidden=float(P_out), P_allowed=float(P_in), xi_turning=math.sqrt(2 * n + 1))


def ref_ho_forbidden(n):
    import numpy as np
    from scipy.integrate import quad
    from scipy.special import eval_hermite
    f = lambda x: eval_hermite(n, x) ** 2 * np.exp(-x * x)   # noqa: E731
    a = math.sqrt(2 * n + 1)
    o = dict(epsabs=0, epsrel=1e-13, limit=500)
    inside = 2 * quad(f, 0, a, **o)[0]
    outside = 2 * quad(f, a, np.inf, **o)[0]
    return {"P_forbidden": outside / (inside + outside), "P_allowed": inside / (inside + outside)}


_add("harmonic_oscillator_forbidden_probability",
     name="Probability beyond the classical turning points (harmonic oscillator)", topic="quantum wells / harmonic oscillator and tunnelling",
     purpose="The probability that a harmonic oscillator in level n is found in the classically forbidden region |x| > A_n.",
     inputs=[_intin("n", "quantum number (0 to 60)", 0, 60)],
     outputs=[_o("P_forbidden", "number", "probability outside the turning points"), _o("P_allowed", "number", "probability inside"),
              _o("xi_turning", "number", "turning point in units of sqrt(hbar/(m omega))")],
     primary="P_forbidden", conditions=["0 <= n <= 60"],
     method="integral of H_n(xi)^2 exp(-xi^2) over |xi| < sqrt(2n + 1), term by term with the incomplete gamma function (50 digits)",
     latex=r"P = 1 - \int_{-\sqrt{2n+1}}^{\sqrt{2n+1}} |\psi_n(\xi)|^2\,d\xi",
     run=ho_forbidden, verify=_recheck(ho_forbidden),
     parse=lambda q: _pairs(q, r"(?:harmonic oscillator )?(?:classically )?forbidden(?: region)? probability", {"n": "n"}),
     reference=ref_ho_forbidden,
     wrong=_safe(lambda n: (lambda o: {**o, "P_forbidden": _r(math.erfc(math.sqrt(2 * n + 1)))})(ho_forbidden(n))),
     random_inputs=lambda rng: {"n": rng.randint(0, 25)},
     templates=["Forbidden region probability: n = 0"],
     examples=[{"inputs": {"n": 0}, "outputs": {"P_forbidden": math.erfc(1.0)}},
               {"inputs": {"n": 1}, "outputs": {"P_forbidden": math.erfc(math.sqrt(3)) + 2 * math.sqrt(3 / math.pi) * math.exp(-3)}}],
     questions=[{"q": "Forbidden region probability: n = 0", "outputs": {"P_forbidden": math.erfc(1.0)}},
                {"q": "Forbidden region probability: n = 100", "outcome": "INVALID_INPUT"},
                {"q": "Forbidden region probability:", "outcome": "NEEDS_INPUT"}])


def _fw_root(f, lo, hi):
    flo = f(lo)
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if mid <= lo or mid >= hi:
            break
        fm = f(mid)
        if (fm < 0) == (flo < 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return 0.5 * (lo + hi)


def finite_well(V0, a, m):
    if V0 <= 0 or a <= 0 or m <= 0:
        raise ValueError("the depth, width and mass must be positive")
    z0 = a / 2 * math.sqrt(2 * m * V0) / HBAR
    if z0 > 200:
        raise ValueError("the well holds more than 127 bound states; give a shallower or narrower well")
    k = 1
    while k * math.pi / 2 < z0:
        k += 1
    if abs(z0 - (k - 1) * math.pi / 2) <= 1e-9 * z0 and k > 1:
        raise ValueError("a bound state sits exactly at the top of the well; its energy is not determined to the stated precision")
    zs = []
    for j in range(k):
        lo, hi = j * math.pi / 2, min((j + 1) * math.pi / 2, z0)
        if lo >= z0:
            break
        if j % 2 == 0:
            f = lambda z: z * math.sin(z) - math.sqrt(max(z0 * z0 - z * z, 0.0)) * math.cos(z)   # noqa: E731  even states
        else:
            f = lambda z: z * math.cos(z) + math.sqrt(max(z0 * z0 - z * z, 0.0)) * math.sin(z)   # noqa: E731  odd states
        zs.append(_fw_root(f, lo, hi))
    En = [2 * HBAR * HBAR * z * z / (m * a * a) for z in zs]
    return _out(n_bound=float(len(En)), E1_eV=En[0] / EV, energies_eV=[E / EV for E in En], z0=z0)


def ref_finite_well(V0, a, m):
    # shooting with the numerically exponentiated transfer matrix (scipy.linalg.expm) across the well: start with the
    # decaying solution on the left, require decay on the right; roots bracketed by a scan and refined by brentq
    import numpy as np
    from scipy import constants as sc
    from scipy.linalg import expm
    from scipy.optimize import brentq
    unit = sc.hbar ** 2 / (2 * m * a * a)
    v0 = V0 / unit

    def f(e):
        kap = math.sqrt(max(v0 - e, 0.0))
        y = expm(np.array([[0.0, 1.0], [-e, 0.0]])) @ np.array([1.0, kap])
        return float((y[1] + kap * y[0]) / math.hypot(y[0], y[1]))
    grid = np.linspace(v0 * 1e-9, v0 * (1 - 1e-12), 600)
    vals = [f(e) for e in grid]
    roots = [brentq(f, grid[i], grid[i + 1], xtol=1e-300, rtol=8.9e-16) for i in range(len(grid) - 1) if vals[i] * vals[i + 1] < 0]
    En = [e * unit / sc.e for e in roots]
    return {"n_bound": float(len(En)), "energies_eV": En, "E1_eV": En[0]}


def _rand_fw(rng):
    for _ in range(100):
        z0 = rng.uniform(0.3, 7.0)
        if min(abs(z0 - k * math.pi / 2) for k in range(1, 6)) > 0.08:
            break
    m = _lu(rng, 1e-31, 1e-27)
    a = _lu(rng, 1e-10, 1e-8)
    return {"m": m, "a": a, "V0": (2 * z0 * HBAR / a) ** 2 / (2 * m)}


_add("finite_square_well",
     name="Bound states of a finite square well", topic="quantum wells / finite square well",
     purpose="The number and energies of the bound states of a particle of mass m in a square well of depth V0 and width a.",
     inputs=[_phys("V0", "energy", "depth of the well (eV, J)"), _phys("a", "length", "width of the well"), _phys("m", "mass", "particle mass")],
     outputs=[_o("n_bound", "number", "number of bound states"), _o("E1_eV", "energy_ev", "ground-state energy above the bottom of the well"),
              {"symbol": "energies_eV", "kind": "number", "meaning": "all bound-state energies above the bottom (eV), ascending"},
              _o("z0", "number", "well strength (a/2) sqrt(2 m V0)/hbar")],
     primary="E1_eV", conditions=["V0, a, m > 0", "at most 127 bound states", "no state exactly at the top of the well"],
     method="even states z tan z = sqrt(z0^2 - z^2), odd states -z cot z = sqrt(z0^2 - z^2), z = (a/2) sqrt(2 m E)/hbar, solved by bisection",
     latex=r"z\tan z = \sqrt{z_0^2 - z^2},\quad -z\cot z = \sqrt{z_0^2 - z^2},\quad z_0 = \frac{a}{2\hbar}\sqrt{2mV_0}",
     run=finite_well, verify=_recheck(finite_well),
     parse=lambda q: _pairs(q, r"finite (?:square )?well(?: bound states)?", {"v0": "V0", "a": "a", "m": "m"}, units={"V0", "a", "m"}),
     reference=ref_finite_well,
     wrong=_safe(lambda V0, a, m: (lambda o: {**o, "E1_eV": _r((math.pi * HBAR / a) ** 2 / (2 * m) / EV),
                                              "energies_eV": [_r(((k + 1) * math.pi * HBAR / a) ** 2 / (2 * m) / EV) for k in range(int(o["n_bound"]))]})(finite_well(V0, a, m))),
     random_inputs=_rand_fw,
     templates=["Finite square well: V0 = 10 eV, a = 0.5 nm, m = 9.109e-31 kg"],
     examples=[{"inputs": {"V0": 10 * EV, "a": 0.5e-9, "m": ME}, "outputs": {"n_bound": 3.0}}],
     questions=[{"q": "Finite square well: V0 = 10 eV, a = 0.5 nm, m = 9.1093837139e-31 kg", "outputs": {"n_bound": 3.0}},
                {"q": "Finite square well: V0 = -10 eV, a = 0.5 nm, m = 9.1e-31 kg", "outcome": "INVALID_INPUT"},
                {"q": "Finite square well: V0 = 10 eV, a = 0.5 nm", "outcome": "NEEDS_INPUT"}])


def barrier_transmission(E, V0, a, m):
    if E <= 0 or V0 <= 0 or a <= 0 or m <= 0:
        raise ValueError("the energy, barrier height, width and mass must be positive")
    q2 = 2 * m * (V0 - E) / (HBAR * HBAR)               # kappa^2 (E < V0) or -k2^2 (E > V0)
    x = math.sqrt(abs(q2)) * a
    if x < 1e-6:
        s = 1 + (x * x / 6 if q2 > 0 else -x * x / 6)
    else:
        s = math.sinh(x) / x if q2 > 0 else math.sin(x) / x
    if q2 > 0 and x > 30:                               # sinh^2 grows as e^(2x)/4: work with its reciprocal
        inv_B = 16 * E * (V0 - E) / (V0 * V0) * math.exp(-2 * x) / (1 - math.exp(-2 * x)) ** 2
        T = inv_B / (1 + inv_B)
        R = 1 / (1 + inv_B)
    else:
        B = V0 * V0 * (2 * m * a * a / (HBAR * HBAR)) * s * s / (4 * E)
        T, R = 1 / (1 + B), B / (1 + B)
    regime = "tunnelling (E < V0)" if E < V0 else ("E = V0" if E == V0 else "over the barrier (E > V0)")
    return _out(T=T, R=R, kappa_a=x if q2 > 0 else 0.0, regime=regime)


def ref_barrier_transmission(E, V0, a, m):
    # the time-independent Schroedinger equation integrated numerically (scipy DOP853) through the barrier, from a pure
    # transmitted wave on the right back to the left, then split into incident and reflected waves
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import solve_ivp
    unit = sc.hbar ** 2 / (2 * m * a * a)                # x in units of a
    e, v = E / unit, V0 / unit
    k = math.sqrt(e)

    def rhs(x, y):                                       # y = (Re psi, Im psi, Re psi', Im psi')
        return [y[2], y[3], (v - e) * y[0], (v - e) * y[1]]
    p1 = np.exp(1j * k)
    d1 = 1j * k * p1
    sol = solve_ivp(rhs, (1.0, 0.0), [p1.real, p1.imag, d1.real, d1.imag], method="DOP853", rtol=1e-13, atol=1e-16)
    y = sol.y[:, -1]
    psi, dpsi = y[0] + 1j * y[1], y[2] + 1j * y[3]
    A = 0.5 * (psi + dpsi / (1j * k))
    Bc = 0.5 * (psi - dpsi / (1j * k))
    return {"T": float(1 / abs(A) ** 2), "R": float(abs(Bc) ** 2 / abs(A) ** 2)}


def _rand_barrier(rng):
    m = _lu(rng, 1e-31, 1e-27)
    a = _lu(rng, 1e-11, 1e-9)
    unit = HBAR * HBAR / (2 * m * a * a)
    v = rng.uniform(0.5, 120)                           # barrier height in units of hbar^2/(2 m a^2): kappa a below ~11
    r = rng.choice([rng.uniform(0.05, 0.95), rng.uniform(1.05, 4.0)])
    return {"m": m, "a": a, "V0": v * unit, "E": r * v * unit}


_add("barrier_transmission",
     name="Transmission through a rectangular barrier", topic="quantum wells / tunnelling",
     purpose="Transmission and reflection probabilities of a particle of energy E meeting a rectangular barrier of height V0 and width a (below or above the top).",
     inputs=[_phys("E", "energy", "kinetic energy of the incident particle"), _phys("V0", "energy", "barrier height"),
             _phys("a", "length", "barrier width"), _phys("m", "mass", "particle mass")],
     outputs=[_o("T", "number", "transmission probability"), _o("R", "number", "reflection probability"),
              _o("kappa_a", "number", "kappa a = a sqrt(2m(V0 - E))/hbar (0 above the barrier)"),
              {"symbol": "regime", "kind": "number", "meaning": "tunnelling (E < V0) or over the barrier"}],
     primary="T", conditions=["E, V0, a, m > 0"],
     method="T = 1/(1 + V0^2 sinh^2(kappa a)/(4E(V0 - E))) for E < V0, sin for E > V0, written as sinh(x)/x to stay finite at E = V0",
     latex=r"T = \left[1 + \frac{V_0^2\sinh^2(\kappa a)}{4E(V_0 - E)}\right]^{-1},\quad \kappa = \frac{\sqrt{2m(V_0 - E)}}{\hbar}",
     run=barrier_transmission, verify=_recheck(barrier_transmission),
     parse=lambda q: _pairs(q, r"(?:rectangular )?barrier transmission|tunnell?ing probability", {"e": "E", "v0": "V0", "a": "a", "m": "m"},
                            units={"E", "V0", "a", "m"}),
     reference=ref_barrier_transmission,
     wrong=_safe(lambda E, V0, a, m: (lambda o: {**o, "T": _r(math.exp(-2 * o["kappa_a"])) if o["kappa_a"] else 1.0, "R": 0.0})(barrier_transmission(E, V0, a, m))),
     random_inputs=_rand_barrier,
     templates=["Barrier transmission: E = 1 eV, V0 = 2 eV, a = 0.5 nm, m = 9.109e-31 kg"],
     examples=[{"inputs": {"E": 1 * EV, "V0": 2 * EV, "a": 0.5e-9, "m": ME},
                "outputs": {"T": 1 / (1 + 4 * math.sinh(0.5e-9 * math.sqrt(2 * ME * EV) / HBAR) ** 2 / 4)}},
               {"inputs": {"E": 2 * EV, "V0": 2 * EV, "a": 1e-10, "m": ME}, "outputs": {"T": 1 / (1 + 4 * EV * EV * 2 * ME * 1e-20 / HBAR ** 2 / (8 * EV))}}],
     questions=[{"q": "Barrier transmission: E = 1 eV, V0 = 2 eV, a = 0.5 nm, m = 9.1093837139e-31 kg",
                 "outputs": {"T": 1 / (1 + 4 * math.sinh(0.5e-9 * math.sqrt(2 * ME * EV) / HBAR) ** 2 / 4)}},
                {"q": "Barrier transmission: E = -1 eV, V0 = 2 eV, a = 0.5 nm, m = 9.1e-31 kg", "outcome": "INVALID_INPUT"},
                {"q": "Barrier transmission: E = 1 eV, V0 = 2 eV, a = 0.5 nm", "outcome": "NEEDS_INPUT"}])


def wkb_triangular(W, F):
    if W <= 0 or F <= 0:
        raise ValueError("the barrier height (work function) and the field must be positive")
    b = 4 * math.sqrt(2 * ME) * W ** 1.5 / (3 * QE * HBAR * F)
    return _out(exponent=b, T=math.exp(-b), log10_T=-b / math.log(10), width=W / (QE * F), width_nm=W / (QE * F) * 1e9)


def ref_wkb_triangular(W, F):
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    d = W / (sc.e * F)
    I = quad(lambda x: np.sqrt(2 * sc.m_e * max(W - sc.e * F * x, 0.0)) / sc.hbar, 0, d, epsabs=0, epsrel=1e-13, limit=200)[0]
    return {"exponent": 2 * I, "log10_T": -2 * I / np.log(10), "width_nm": d * 1e9}


_add("wkb_field_emission",
     name="WKB tunnelling through a triangular barrier (field emission)", topic="quantum wells / tunnelling (WKB)",
     purpose="The WKB tunnelling exponent and probability of an electron at the Fermi level through the triangular barrier W - eFx of a metal in a field F.",
     inputs=[_phys("W", "energy", "barrier height above the electron's energy (work function, eV)"), _phys("F", "electric_field", "applied electric field (V/m)")],
     outputs=[_o("exponent", "number", "WKB exponent 2 integral kappa dx"), _o("T", "number", "tunnelling probability exp(-exponent)"),
              _o("log10_T", "number", "log10 of the probability"), _o("width", "length", "barrier width W/(eF)"), _o("width_nm", "length_nm", "width in nm")],
     primary="exponent", conditions=["W > 0", "F > 0", "WKB: exponent much larger than 1"],
     method="T = exp(-(4/3) sqrt(2 m_e) W^(3/2)/(e hbar F)) (Fowler-Nordheim exponent, no image force)",
     latex=r"T \approx \exp\!\left(-\frac{4\sqrt{2m_e}\,W^{3/2}}{3 e\hbar F}\right)",
     run=wkb_triangular, verify=_recheck(wkb_triangular),
     parse=lambda q: _pairs(q, r"(?:wkb )?field emission|triangular barrier(?: tunnell?ing)?", {"w": "W", "f": "F"}, units={"W", "F"}),
     reference=ref_wkb_triangular,
     wrong=_safe(lambda W, F: (lambda b: _out(exponent=b, T=math.exp(-b), log10_T=-b / math.log(10), width=W / (QE * F), width_nm=W / (QE * F) * 1e9))(
         2 * math.sqrt(2 * ME) * W ** 1.5 / (QE * HBAR * F))),
     random_inputs=lambda rng: {"W": rng.uniform(2, 6) * EV, "F": _lu(rng, 1e9, 2e10)},
     templates=["Field emission: W = 4.5 eV, F = 5e9 V/m"],
     examples=[{"inputs": {"W": 4.5 * EV, "F": 5e9}, "outputs": {"exponent": 4 * math.sqrt(2 * ME) * (4.5 * EV) ** 1.5 / (3 * QE * HBAR * 5e9)}}],
     questions=[{"q": "Field emission: W = 4.5 eV, F = 5e9 V/m", "outputs": {"exponent": 4 * math.sqrt(2 * ME) * (4.5 * EV) ** 1.5 / (3 * QE * HBAR * 5e9)}},
                {"q": "Field emission: W = 4.5 eV, F = -5e9 V/m", "outcome": "INVALID_INPUT"},
                {"q": "Field emission: W = 4.5 eV", "outcome": "NEEDS_INPUT"}])

# ================================================================================================ spin and qubits
def _bloch(theta, phi):
    return (math.sin(theta) * math.cos(phi), math.sin(theta) * math.sin(phi), math.cos(theta))


def spin_measurement(theta, phi, theta_n, phi_n):
    for t, nm in ((theta, "theta"), (theta_n, "theta_n")):
        if not (0 <= t <= math.pi):
            raise ValueError(f"{nm} (polar angle) must lie between 0 and 180 degrees")
    r, n = _bloch(theta, phi), _bloch(theta_n, phi_n)
    d = max(-1.0, min(1.0, sum(a * b for a, b in zip(r, n))))
    return _out(P_up=_snap((1 + d) / 2), P_down=_snap((1 - d) / 2), expectation=_snap(d))


def ref_spin_measurement(theta, phi, theta_n, phi_n):
    import numpy as np
    import qutip
    psi = (np.cos(theta / 2) * qutip.basis(2, 0) + np.exp(1j * phi) * np.sin(theta / 2) * qutip.basis(2, 1)).unit()
    nx, ny, nz = _bloch(theta_n, phi_n)
    op = nx * qutip.sigmax() + ny * qutip.sigmay() + nz * qutip.sigmaz()
    vals, vecs = op.eigenstates()
    up = vecs[int(np.argmax(vals))]
    p = abs(up.overlap(psi)) ** 2
    return {"P_up": float(_snap(p)), "expectation": float(_snap(qutip.expect(op, psi)))}


def _rand_spin(rng):
    return {"theta": rng.uniform(0, math.pi), "phi": rng.uniform(0, 2 * math.pi), "theta_n": rng.uniform(0, math.pi), "phi_n": rng.uniform(0, 2 * math.pi)}


_add("spin_half_measurement",
     name="Spin-1/2 measurement along an axis", topic="spin and qubits / spin measurement",
     purpose="Probabilities of spin up and down along a direction n (angles theta_n, phi_n) for a spin-1/2 state pointing along (theta, phi) on the Bloch sphere.",
     inputs=[_phys("theta", "angle", "polar angle of the state's Bloch vector"), _phys("phi", "angle", "azimuth of the state's Bloch vector"),
             _phys("theta_n", "angle", "polar angle of the measurement axis"), _phys("phi_n", "angle", "azimuth of the measurement axis")],
     outputs=[_o("P_up", "number", "probability of +hbar/2 along n"), _o("P_down", "number", "probability of -hbar/2 along n"),
              _o("expectation", "number", "<sigma_n> (the mean spin along n in units of hbar/2)")],
     primary="P_up", conditions=["polar angles between 0 and 180 degrees"],
     method="P_up = (1 + r . n)/2 with r and n the unit vectors of the state and the axis",
     latex=r"P_\uparrow = |\langle +_n|\psi\rangle|^2 = \tfrac12\left(1 + \hat r\cdot\hat n\right)",
     run=spin_measurement, verify=_recheck(spin_measurement),
     parse=lambda q: _pairs(q, r"spin(?:-1/2| half)? measurement", {"theta": "theta", "phi": "phi", "theta_n": "theta_n", "phi_n": "phi_n"},
                            units={"theta", "phi", "theta_n", "phi_n"}),
     reference=ref_spin_measurement,
     wrong=_safe(lambda theta, phi, theta_n, phi_n: (lambda o: {**o, "P_up": _r(math.cos((theta - theta_n) / 2) ** 2)})(
         spin_measurement(theta, phi, theta_n, phi_n))),
     random_inputs=_rand_spin,
     templates=["Spin measurement: theta = 90 deg, phi = 0 deg, theta_n = 0 deg, phi_n = 0 deg"],
     examples=[{"inputs": {"theta": math.pi / 2, "phi": 0.0, "theta_n": 0.0, "phi_n": 0.0}, "outputs": {"P_up": 0.5, "expectation": 0.0}},
               {"inputs": {"theta": math.pi / 3, "phi": 0.0, "theta_n": 0.0, "phi_n": 0.0}, "outputs": {"P_up": 0.75}},
               {"inputs": {"theta": 0.0, "phi": 0.0, "theta_n": math.pi, "phi_n": 0.0}, "outputs": {"P_up": 0.0, "P_down": 1.0}}],
     questions=[{"q": "Spin measurement: theta = 60 deg, phi = 0 deg, theta_n = 0 deg, phi_n = 0 deg", "outputs": {"P_up": 0.75}},
                {"q": "Spin measurement: theta = 200 deg, phi = 0 deg, theta_n = 0 deg, phi_n = 0 deg", "outcome": "INVALID_INPUT"},
                {"q": "Spin measurement: theta = 60 deg, phi = 0 deg", "outcome": "NEEDS_INPUT"}])


def qubit_state(a_re, a_im, b_re, b_im):
    a, b = complex(a_re, a_im), complex(b_re, b_im)
    nrm = math.sqrt(abs(a) ** 2 + abs(b) ** 2)
    if not math.isfinite(nrm) or nrm == 0:
        raise ValueError("the state needs at least one non-zero amplitude")
    a, b = a / nrm, b / nrm
    p0, p1 = abs(a) ** 2, abs(b) ** 2
    ab = a.conjugate() * b
    x, y, z = 2 * ab.real, 2 * ab.imag, p0 - p1
    theta = 2 * math.atan2(abs(b), abs(a))
    phi = 0.0 if (abs(a) < 1e-12 or abs(b) < 1e-12) else (cmath.phase(b) - cmath.phase(a)) % (2 * math.pi)
    if phi > 2 * math.pi - 1e-12:
        phi = 0.0
    return _out(p0=_snap(p0), p1=_snap(p1), x=_snap(x), y=_snap(y), z=_snap(z), theta=theta, phi=phi, norm=nrm)


def ref_qubit_state(a_re, a_im, b_re, b_im):
    import numpy as np
    import qutip
    psi = qutip.Qobj(np.array([[a_re + 1j * a_im], [b_re + 1j * b_im]])).unit()
    x, y, z = (float(qutip.expect(o, psi)) for o in (qutip.sigmax(), qutip.sigmay(), qutip.sigmaz()))
    return {"x": _snap(x), "y": _snap(y), "z": _snap(z), "theta": float(np.arctan2(np.hypot(x, y), z)),
            "phi": float(np.arctan2(y, x) % (2 * np.pi)), "p0": _snap((1 + z) / 2)}


_add("qubit_state_bloch",
     name="Qubit state on the Bloch sphere", topic="spin and qubits / qubit states",
     purpose="Normalised measurement probabilities, Bloch vector, and Bloch angles of the qubit state alpha|0> + beta|1> (amplitudes need not be normalised).",
     inputs=[_numin("a_re", "real part of alpha"), _numin("a_im", "imaginary part of alpha"), _numin("b_re", "real part of beta"), _numin("b_im", "imaginary part of beta")],
     outputs=[_o("p0", "number", "probability of 0"), _o("p1", "number", "probability of 1"), _o("x", "number", "Bloch x = <sigma_x>"),
              _o("y", "number", "Bloch y = <sigma_y>"), _o("z", "number", "Bloch z = <sigma_z>"), _o("theta", "angle", "polar angle (rad)"),
              _o("phi", "angle", "azimuth in [0, 2 pi) (0 when one amplitude is zero)"), _o("norm", "number", "norm of the given amplitudes")],
     primary="z", conditions=["not both amplitudes zero"],
     method="normalise; x + iy = 2 conj(alpha) beta, z = |alpha|^2 - |beta|^2, theta = 2 atan2(|beta|, |alpha|), phi = arg(beta) - arg(alpha)",
     latex=r"|\psi\rangle = \cos\tfrac{\theta}{2}|0\rangle + e^{i\phi}\sin\tfrac{\theta}{2}|1\rangle",
     run=qubit_state, verify=_recheck(qubit_state),
     parse=lambda q: _pairs(q, r"qubit state(?: on the bloch sphere)?|bloch vector", {"a_re": "a_re", "a_im": "a_im", "b_re": "b_re", "b_im": "b_im"}),
     reference=ref_qubit_state,
     wrong=_safe(lambda a_re, a_im, b_re, b_im: (lambda o: {**o, "z": _r(math.hypot(a_re, a_im) - math.hypot(b_re, b_im)), "theta": _r(math.atan2(math.hypot(b_re, b_im), math.hypot(a_re, a_im)))})(
         qubit_state(a_re, a_im, b_re, b_im))),
     random_inputs=lambda rng: {k: rng.uniform(-1, 1) for k in ("a_re", "a_im", "b_re", "b_im")},
     templates=["Qubit state: a_re = 0.6, a_im = 0, b_re = 0, b_im = 0.8"],
     examples=[{"inputs": {"a_re": 0.6, "a_im": 0.0, "b_re": 0.0, "b_im": 0.8}, "outputs": {"p0": 0.36, "y": 0.96, "x": 0.0, "z": -0.28, "phi": math.pi / 2}},
               {"inputs": {"a_re": 1.0, "a_im": 0.0, "b_re": 1.0, "b_im": 0.0}, "outputs": {"x": 1.0, "theta": math.pi / 2, "norm": math.sqrt(2)}}],
     questions=[{"q": "Qubit state: a_re = 0.6, a_im = 0, b_re = 0, b_im = 0.8", "outputs": {"p0": 0.36, "y": 0.96}},
                {"q": "Qubit state: a_re = 0, a_im = 0, b_re = 0, b_im = 0", "outcome": "INVALID_INPUT"},
                {"q": "Qubit state: a_re = 0.6, a_im = 0", "outcome": "NEEDS_INPUT"}])


# ------------------------------------------------------------------------------------------------ quantum gates
_S2 = 1 / math.sqrt(2)
_FIXED = {"I": ((1, 0), (0, 1)), "H": ((_S2, _S2), (_S2, -_S2)), "X": ((0, 1), (1, 0)), "Y": ((0, -1j), (1j, 0)), "Z": ((1, 0), (0, -1)),
          "S": ((1, 0), (0, 1j)), "SDG": ((1, 0), (0, -1j)), "T": ((1, 0), (0, cmath.exp(1j * math.pi / 4))),
          "TDG": ((1, 0), (0, cmath.exp(-1j * math.pi / 4)))}
_ANG = re.compile(r"\s*([-+]?)\s*(\d+(?:\.\d*)?|\.\d+)?\s*\*?\s*(pi|π)?\s*(?:/\s*(\d+(?:\.\d*)?))?\s*")


def _angle(text):
    m = _ANG.fullmatch(text)
    if not m or not (m.group(2) or m.group(3)):
        raise ValueError(f"cannot read the angle {text!r} (write e.g. 0.5, pi/4, -3pi/2)")
    v = float(m.group(2)) if m.group(2) else 1.0
    if m.group(3):
        v *= math.pi
    if m.group(4):
        v /= float(m.group(4))
    return -v if m.group(1) == "-" else v


def _rot(name, t):
    c, s = math.cos(t / 2), math.sin(t / 2)
    if name == "RX":
        return ((c, -1j * s), (-1j * s, c))
    if name == "RY":
        return ((c, -s), (s, c))
    if name == "RZ":
        return ((cmath.exp(-0.5j * t), 0), (0, cmath.exp(0.5j * t)))
    return ((1, 0), (0, cmath.exp(1j * t)))             # P(t)


_TOK1 = re.compile(r"\s*(SDG|TDG|RX|RY|RZ|H|X|Y|Z|S|T|I|P)(?:\s*\(([^)]*)\))?\s*(?:,|\s|$)", re.I)


def _gates_1q(text):
    """'H T Rz(pi/2), X' -> [(name, angle or None, 2x2 matrix), ...]."""
    if not isinstance(text, str) or not text.strip():
        raise ValueError("give at least one gate (H, X, Y, Z, S, Sdg, T, Tdg, Rx(t), Ry(t), Rz(t), P(t))")
    out, pos, s = [], 0, text.strip()
    while pos < len(s):
        m = _TOK1.match(s, pos)
        if not m or m.end() == pos:
            raise ValueError(f"cannot read the gate at {s[pos:pos + 12]!r}")
        name = m.group(1).upper()
        if name in ("RX", "RY", "RZ", "P"):
            if m.group(2) is None:
                raise ValueError(f"{name} needs an angle, e.g. {name}(pi/2)")
            t = _angle(m.group(2))
            out.append((name, t, _rot(name, t)))
        else:
            if m.group(2) is not None:
                raise ValueError(f"{name} takes no angle")
            out.append((name, None, _FIXED[name]))
        pos = m.end()
    if len(out) > 500:
        raise ValueError("at most 500 gates")
    return out


def _mv(M, v):
    return [M[0][0] * v[0] + M[0][1] * v[1], M[1][0] * v[0] + M[1][1] * v[1]]


_START1 = {"0": (1, 0), "1": (0, 1), "+": (_S2, _S2), "-": (_S2, -_S2), "+i": (_S2, 1j * _S2), "-i": (_S2, -1j * _S2)}


def _canon(amps):
    """Fix the global phase: the first amplitude larger than 1e-9 becomes real and positive; residue below 1e-12 is 0."""
    ref = next((a for a in amps if abs(a) > 1e-9), 1)
    ph = abs(ref) / ref
    out = []
    for a in amps:
        b = complex(a) * ph
        out.append(complex(_snap(b.real), _snap(b.imag)))
    return out


def _one_qubit_result(a, b):
    a, b = _canon([a, b])
    ab = a.conjugate() * b
    return _out(alpha_re=a.real, alpha_im=a.imag, beta_re=b.real, beta_im=b.imag, p0=_snap(abs(a) ** 2), p1=_snap(abs(b) ** 2),
                bx=_snap(2 * ab.real), by=_snap(2 * ab.imag), bz=_snap(abs(a) ** 2 - abs(b) ** 2))


def single_qubit_circuit(start, gates, reverse=False):
    seq = _gates_1q(gates)
    v = list(_START1[start])
    for _, _, M in (reversed(seq) if reverse else seq):
        v = _mv(M, v)
    return _one_qubit_result(v[0], v[1])


def ref_single_qubit_circuit(start, gates):
    from qiskit import QuantumCircuit
    from qiskit.quantum_info import Statevector
    qc = QuantumCircuit(1)
    for name, t, _ in _gates_1q(gates):
        {"H": lambda: qc.h(0), "X": lambda: qc.x(0), "Y": lambda: qc.y(0), "Z": lambda: qc.z(0), "S": lambda: qc.s(0),
         "SDG": lambda: qc.sdg(0), "T": lambda: qc.t(0), "TDG": lambda: qc.tdg(0), "I": lambda: qc.id(0),
         "RX": lambda: qc.rx(t, 0), "RY": lambda: qc.ry(t, 0), "RZ": lambda: qc.rz(t, 0), "P": lambda: qc.p(t, 0)}[name]()
    label = {"0": "0", "1": "1", "+": "+", "-": "-", "+i": "r", "-i": "l"}[start]
    sv = Statevector.from_label(label).evolve(qc).data
    r = _one_qubit_result(complex(sv[0]), complex(sv[1]))
    return {k: float(v) for k, v in r.items()}


def _rand_gates_1q(rng, n=None):
    names = ["H", "X", "Y", "Z", "S", "Sdg", "T", "Tdg", "Rx", "Ry", "Rz", "P"]
    toks = []
    for _ in range(n or rng.randint(2, 8)):
        g = rng.choice(names)
        toks.append(f"{g}({rng.uniform(-3.1, 3.1):.6f})" if g in ("Rx", "Ry", "Rz", "P") else g)
    return " ".join(toks)


_START1_IN = {"symbol": "start", "type": "choice", "choices": ["0", "1", "+", "-", "+i", "-i"], "meaning": "initial state |0>, |1>, |+>, |->, |+i> or |-i>", "required": True}
_GATES_IN = {"symbol": "gates", "type": "text", "min_length": 1, "max_length": 4000, "required": True,
             "meaning": "gates applied left to right, e.g. 'H T Rz(pi/2) X'"}


def _parse_circuit(q, phrase, starts):
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", q, re.I)
    if not m:
        return None
    rest = q[m.end():].strip().rstrip(".")
    out = {}
    mg = re.search(r"\bgates\s*=\s*(.+)$", rest, re.I)
    if mg:
        out["gates"] = mg.group(1).strip()
        rest = rest[:mg.start()]
    for seg in re.split(r"[;,]", rest):
        if not seg.strip():
            continue
        ms = re.fullmatch(r"\s*start\s*=\s*\|?([^\s|>]+)>?\s*", seg, re.I)
        if not ms:
            return None
        out["start"] = ms.group(1)
    return out


_add("single_qubit_circuit",
     name="State of a qubit after a sequence of gates", topic="quantum gates / single-qubit gates",
     purpose="The amplitudes, measurement probabilities and Bloch vector of a qubit after gates such as H, X, S, T and rotations act on |0>, |1>, |+>, ...",
     inputs=[_START1_IN, _GATES_IN],
     outputs=[_o("alpha_re", "number", "Re alpha (global phase fixed: first non-zero amplitude real positive)"), _o("alpha_im", "number", "Im alpha"),
              _o("beta_re", "number", "Re beta"), _o("beta_im", "number", "Im beta"), _o("p0", "number", "probability of measuring 0"),
              _o("p1", "number", "probability of measuring 1"), _o("bx", "number", "Bloch x"), _o("by", "number", "Bloch y"), _o("bz", "number", "Bloch z")],
     primary="p0", conditions=["gates from H, X, Y, Z, S, Sdg, T, Tdg, I, Rx(t), Ry(t), Rz(t), P(t)", "at most 500 gates"],
     method="the state vector multiplied by each gate's 2x2 matrix in turn",
     latex=r"|\psi\rangle = U_k \cdots U_2 U_1 |\psi_0\rangle",
     run=single_qubit_circuit, verify=_recheck(single_qubit_circuit),
     parse=lambda q: _parse_circuit(q, r"single[- ]qubit circuit|qubit gates", _START1),
     reference=ref_single_qubit_circuit,
     wrong=_safe(lambda start, gates: single_qubit_circuit(start, gates, reverse=True)),
     random_inputs=lambda rng: {"start": rng.choice(list(_START1)), "gates": _rand_gates_1q(rng)},
     templates=["Single-qubit circuit: start = 0; gates = H", "Single-qubit circuit: start = 0; gates = H T H"],
     examples=[{"inputs": {"start": "0", "gates": "H"}, "outputs": {"p0": 0.5, "p1": 0.5, "bx": 1.0, "alpha_re": _S2}},
               {"inputs": {"start": "0", "gates": "X"}, "outputs": {"p1": 1.0, "beta_re": 1.0, "alpha_re": 0.0}},
               {"inputs": {"start": "0", "gates": "H S"}, "outputs": {"by": 1.0, "beta_im": _S2}},
               {"inputs": {"start": "0", "gates": "Ry(pi/3)"}, "outputs": {"p0": 0.75}}],
     questions=[{"q": "Single-qubit circuit: start = 0; gates = H", "outputs": {"p0": 0.5, "bx": 1.0}},
                {"q": "Single-qubit circuit: start = 1; gates = H Z H", "outputs": {"p0": 1.0}},
                {"q": "Single-qubit circuit: start = 0; gates = H Q", "outcome": "INVALID_INPUT"},
                {"q": "Single-qubit circuit: start = 0", "outcome": "NEEDS_INPUT"}])

_TOK2 = re.compile(r"\s*(?:(CNOT|CX)(01|10)|(CZ)|(SWAP)|(SDG|TDG|RX|RY|RZ|H|X|Y|Z|S|T|I|P)([01])(?:\s*\(([^)]*)\))?)\s*(?:,|\s|$)", re.I)


def _gates_2q(text):
    if not isinstance(text, str) or not text.strip():
        raise ValueError("give at least one gate (H0, X1, Rz0(pi/2), CNOT01, CNOT10, CZ, SWAP, ...)")
    out, pos, s = [], 0, text.strip()
    while pos < len(s):
        m = _TOK2.match(s, pos)
        if not m or m.end() == pos:
            raise ValueError(f"cannot read the gate at {s[pos:pos + 12]!r}")
        if m.group(1):
            out.append(("CNOT", int(m.group(2)[0]), int(m.group(2)[1])))
        elif m.group(3):
            out.append(("CZ", 0, 1))
        elif m.group(4):
            out.append(("SWAP", 0, 1))
        else:
            name, q = m.group(5).upper(), int(m.group(6))
            if name in ("RX", "RY", "RZ", "P"):
                if m.group(7) is None:
                    raise ValueError(f"{name} needs an angle, e.g. {name}{q}(pi/2)")
                t = _angle(m.group(7))
                out.append((name, q, t, _rot(name, t)))
            else:
                if m.group(7) is not None:
                    raise ValueError(f"{name} takes no angle")
                out.append((name, q, None, _FIXED[name]))
        pos = m.end()
    if len(out) > 500:
        raise ValueError("at most 500 gates")
    return out


def _apply2(g, v):
    """v indexed by 2*q0 + q1 (q0 is the first qubit written)."""
    v = list(v)
    if g[0] == "CNOT":
        c, t = g[1], g[2]
        out = list(v)
        for i in range(4):
            bits = [(i >> 1) & 1, i & 1]
            if bits[c]:
                bits[t] ^= 1
                out[2 * bits[0] + bits[1]] = v[i]
            else:
                out[i] = v[i]
        return out
    if g[0] == "CZ":
        return [v[0], v[1], v[2], -v[3]]
    if g[0] == "SWAP":
        return [v[0], v[2], v[1], v[3]]
    q, M = g[1], g[3]
    out = [0j] * 4
    for i in range(4):
        bits = [(i >> 1) & 1, i & 1]
        b = bits[q]
        for nb in (0, 1):
            nbits = list(bits)
            nbits[q] = nb
            out[2 * nbits[0] + nbits[1]] += M[nb][b] * v[i]
    return out


def _swap_labels(g):
    if g[0] == "CNOT":
        return ("CNOT", g[2], g[1])
    if g[0] in ("CZ", "SWAP"):
        return g
    return (g[0], 1 - g[1]) + tuple(g[2:])


def two_qubit_circuit(start, gates, reverse=False):
    seq = _gates_2q(gates)
    if reverse:                                          # the wrong variant: reversed order and swapped qubit labels
        seq = [_swap_labels(g) for g in reversed(seq)]
    v = [0j] * 4
    v[int(start, 2)] = 1.0
    for g in seq:
        v = _apply2(g, v)
    p = [abs(a) ** 2 for a in v]
    conc = 2 * abs(v[0] * v[3] - v[1] * v[2])
    return _out(p00=_snap(p[0]), p01=_snap(p[1]), p10=_snap(p[2]), p11=_snap(p[3]), concurrence=_snap(conc),
                p_q0_is_1=_snap(p[2] + p[3]), p_q1_is_1=_snap(p[1] + p[3]))


def ref_two_qubit_circuit(start, gates):
    import numpy as np
    import qutip
    from qiskit import QuantumCircuit
    from qiskit.quantum_info import Statevector
    qc = QuantumCircuit(2)                               # qiskit qubit k = our qubit k; its labels are written q1 q0
    for g in _gates_2q(gates):
        if g[0] == "CNOT":
            qc.cx(g[1], g[2])
        elif g[0] == "CZ":
            qc.cz(0, 1)
        elif g[0] == "SWAP":
            qc.swap(0, 1)
        else:
            name, q, t = g[0], g[1], g[2]
            {"H": lambda: qc.h(q), "X": lambda: qc.x(q), "Y": lambda: qc.y(q), "Z": lambda: qc.z(q), "S": lambda: qc.s(q),
             "SDG": lambda: qc.sdg(q), "T": lambda: qc.t(q), "TDG": lambda: qc.tdg(q), "I": lambda: qc.id(q),
             "RX": lambda: qc.rx(t, q), "RY": lambda: qc.ry(t, q), "RZ": lambda: qc.rz(t, q), "P": lambda: qc.p(t, q)}[name]()
    sv = Statevector.from_label(start[::-1]).evolve(qc)
    pd = sv.probabilities_dict()
    p = {k[::-1]: float(v) for k, v in pd.items()}
    data = sv.data
    ours = np.array([data[0], data[2], data[1], data[3]])  # reorder to index 2*q0 + q1
    sv2 = np.linalg.svd(ours.reshape(2, 2), compute_uv=False)   # Schmidt coefficients: C = 2 s1 s2 for a pure state
    conc = float(2 * sv2[0] * sv2[1])
    return {"p00": _snap(p.get("00", 0.0)), "p01": _snap(p.get("01", 0.0)), "p10": _snap(p.get("10", 0.0)), "p11": _snap(p.get("11", 0.0)),
            "concurrence": _snap(conc)}


def _rand_gates_2q(rng):
    toks = []
    for _ in range(rng.randint(2, 9)):
        r = rng.random()
        if r < 0.25:
            toks.append(rng.choice(["CNOT01", "CNOT10", "CZ", "SWAP"]))
        else:
            g = rng.choice(["H", "X", "Y", "Z", "S", "T", "Rx", "Ry", "Rz", "P"])
            q = rng.randint(0, 1)
            toks.append(f"{g}{q}({rng.uniform(-3.1, 3.1):.6f})" if g in ("Rx", "Ry", "Rz", "P") else f"{g}{q}")
    return " ".join(toks)


_add("two_qubit_circuit",
     name="Two-qubit circuit: probabilities and entanglement", topic="quantum gates / two-qubit gates and Bell states",
     purpose="Measurement probabilities of |00>, |01>, |10>, |11> and the concurrence (entanglement) after a two-qubit gate sequence such as H0 CNOT01 (a Bell state).",
     inputs=[{"symbol": "start", "type": "choice", "choices": ["00", "01", "10", "11"], "meaning": "initial basis state |q0 q1>", "required": True},
             {**_GATES_IN, "meaning": "gates left to right: H0, X1, Rz0(t), ..., CNOT01 (control q0), CNOT10, CZ, SWAP"}],
     outputs=[_o("p00", "number", "probability of |00>"), _o("p01", "number", "probability of |01>"), _o("p10", "number", "probability of |10>"),
              _o("p11", "number", "probability of |11>"), _o("concurrence", "number", "concurrence 2|a00 a11 - a01 a10| (0 product, 1 maximally entangled)"),
              _o("p_q0_is_1", "number", "probability that q0 reads 1"), _o("p_q1_is_1", "number", "probability that q1 reads 1")],
     primary="p00", conditions=["gates from the documented set", "at most 500 gates"],
     method="the four-amplitude state vector updated gate by gate (index 2 q0 + q1)",
     latex=r"|\Phi^+\rangle = \mathrm{CNOT}_{01}(H\otimes I)|00\rangle = \tfrac{1}{\sqrt2}(|00\rangle + |11\rangle)",
     run=two_qubit_circuit, verify=_recheck(two_qubit_circuit),
     parse=lambda q: _parse_circuit(q, r"two[- ]qubit circuit", None),
     reference=ref_two_qubit_circuit,
     wrong=_safe(lambda start, gates: two_qubit_circuit(start, gates, reverse=True)),
     random_inputs=lambda rng: {"start": rng.choice(["00", "01", "10", "11"]), "gates": _rand_gates_2q(rng)},
     templates=["Two-qubit circuit: start = 00; gates = H0 CNOT01"],
     examples=[{"inputs": {"start": "00", "gates": "H0 CNOT01"}, "outputs": {"p00": 0.5, "p11": 0.5, "p01": 0.0, "concurrence": 1.0}},
               {"inputs": {"start": "00", "gates": "H0 H1"}, "outputs": {"p00": 0.25, "concurrence": 0.0}},
               {"inputs": {"start": "10", "gates": "CNOT01"}, "outputs": {"p11": 1.0}}],
     questions=[{"q": "Two-qubit circuit: start = 00; gates = H0 CNOT01", "outputs": {"p00": 0.5, "p11": 0.5, "concurrence": 1.0}},
                {"q": "Two-qubit circuit: start = 01; gates = X0 CNOT01", "outputs": {"p10": 1.0}},
                {"q": "Two-qubit circuit: start = 00; gates = H2", "outcome": "INVALID_INPUT"},
                {"q": "Two-qubit circuit: start = 00", "outcome": "NEEDS_INPUT"}])


def gate_rotation(gates):
    seq = _gates_1q(gates)
    U = ((1, 0), (0, 1))
    for _, _, M in seq:
        U = ((M[0][0] * U[0][0] + M[0][1] * U[1][0], M[0][0] * U[0][1] + M[0][1] * U[1][1]),
             (M[1][0] * U[0][0] + M[1][1] * U[1][0], M[1][0] * U[0][1] + M[1][1] * U[1][1]))
    det = U[0][0] * U[1][1] - U[0][1] * U[1][0]
    r = cmath.sqrt(det)
    V = [[U[i][j] / r for j in range(2)] for i in range(2)]
    a0 = ((V[0][0] + V[1][1]) / 2).real
    a1 = (1j * (V[0][1] + V[1][0]) / 2).real
    a2 = ((V[1][0] - V[0][1]) / 2).real
    a3 = ((V[1][1] - V[0][0]) / 2j).real
    if a0 < 0:
        a0, a1, a2, a3 = -a0, -a1, -a2, -a3
    s = math.sqrt(a1 * a1 + a2 * a2 + a3 * a3)
    if s < 1e-12:
        return _out(angle=0.0, angle_deg=0.0, nx=0.0, ny=0.0, nz=1.0)
    th = 2 * math.atan2(s, a0)
    n = [a1 / s, a2 / s, a3 / s]
    if math.pi - th < 1e-9:
        first = next(x for x in n if abs(x) > 1e-9)
        if first < 0:
            n = [-x for x in n]
    n = [_snap(x) for x in n]
    return _out(angle=th, angle_deg=math.degrees(th), nx=n[0], ny=n[1], nz=n[2])


def ref_gate_rotation(gates):
    import numpy as np
    from scipy.spatial.transform import Rotation
    U = np.eye(2, dtype=complex)
    for _, _, M in _gates_1q(gates):
        U = np.array(M, dtype=complex) @ U
    P = [np.array([[0, 1], [1, 0]], complex), np.array([[0, -1j], [1j, 0]]), np.array([[1, 0], [0, -1]], complex)]
    R = np.array([[0.5 * np.trace(P[i] @ U @ P[j] @ U.conj().T).real for j in range(3)] for i in range(3)])
    rv = Rotation.from_matrix(R).as_rotvec()
    th = float(np.linalg.norm(rv))
    if th < 1e-12:
        return {"angle": 0.0, "nz": 1.0}
    n = rv / th
    if np.pi - th < 1e-9:
        first = next(x for x in n if abs(x) > 1e-9)
        if first < 0:
            n = -n
    return {"angle": th, "nx": _snap(float(n[0])), "ny": _snap(float(n[1])), "nz": _snap(float(n[2]))}


_add("gate_rotation_axis",
     name="Bloch-sphere rotation of a single-qubit gate sequence", topic="quantum gates / gates as rotations",
     purpose="The single rotation (axis n and angle) of the Bloch sphere that a sequence of single-qubit gates performs (global phase ignored).",
     inputs=[_GATES_IN],
     outputs=[_o("angle", "angle", "rotation angle in [0, pi] (rad)"), _o("angle_deg", "number", "rotation angle in degrees"),
              _o("nx", "number", "axis x"), _o("ny", "number", "axis y"), _o("nz", "number", "axis z")],
     primary="angle", conditions=["gates from H, X, Y, Z, S, Sdg, T, Tdg, I, Rx(t), Ry(t), Rz(t), P(t)"],
     method="U/sqrt(det U) = cos(t/2) I - i sin(t/2) n.sigma; angle = 2 atan2(|a|, a0), n = a/|a|",
     latex=r"U \simeq e^{-i\theta\,\hat n\cdot\vec\sigma/2} = \cos\tfrac{\theta}{2} I - i\sin\tfrac{\theta}{2}\,\hat n\cdot\vec\sigma",
     run=gate_rotation, verify=_recheck(gate_rotation),
     parse=lambda q: (lambda m: {"gates": m.group(1).strip().rstrip(".")} if m else None)(
         re.match(r"\s*(?:gate rotation|bloch(?:-sphere)? rotation)(?: axis)?\s*:\s*gates\s*=\s*(.+)$", q, re.I)),
     reference=ref_gate_rotation,
     wrong=_safe(lambda gates: (lambda o: {**o, "angle": _r(o["angle"] / 2), "angle_deg": _r(o["angle_deg"] / 2)})(gate_rotation(gates))),
     random_inputs=lambda rng: {"gates": rng.choice(["H", "S", "T", "X"]) if rng.random() < 0.1 else _rand_gates_1q(rng)},
     templates=["Gate rotation: gates = H", "Gate rotation: gates = H S"],
     examples=[{"inputs": {"gates": "H"}, "outputs": {"angle": math.pi, "nx": _S2, "ny": 0.0, "nz": _S2}},
               {"inputs": {"gates": "T"}, "outputs": {"angle": math.pi / 4, "nz": 1.0}},
               {"inputs": {"gates": "H H"}, "outputs": {"angle": 0.0}}],
     questions=[{"q": "Gate rotation: gates = H", "outputs": {"angle": math.pi, "nx": _S2}},
                {"q": "Gate rotation: gates = S", "outputs": {"angle": math.pi / 2, "nz": 1.0}},
                {"q": "Gate rotation: gates = Rx", "outcome": "INVALID_INPUT"}])


def rabi(Omega, Delta, t):
    if Omega <= 0 or t < 0:
        raise ValueError("the Rabi frequency must be positive and the time non-negative")
    W = math.hypot(Omega, Delta)
    amp = (Omega / W) ** 2
    return _out(P_excited=_snap(amp * math.sin(W * t / 2) ** 2), W=W, P_max=amp, period=2 * math.pi / W)


def ref_rabi(Omega, Delta, t):
    import numpy as np
    from scipy.linalg import expm
    Hm = 0.5 * np.array([[-Delta, Omega], [Omega, Delta]], dtype=complex)   # rotating-wave Hamiltonian / hbar
    psi = expm(-1j * Hm * t) @ np.array([1.0, 0.0])
    return {"P_excited": float(_snap(abs(psi[1]) ** 2))}


_add("rabi_oscillation",
     name="Rabi oscillation of a driven two-level system", topic="spin and qubits / Rabi oscillations",
     purpose="The probability of finding a two-level system (qubit) excited after driving it for time t with Rabi frequency Omega and detuning Delta.",
     inputs=[_phys("Omega", "angular_frequency", "Rabi angular frequency"), _phys("Delta", "angular_frequency", "detuning (drive minus transition angular frequency)"),
             _phys("t", "time", "driving time")],
     outputs=[_o("P_excited", "number", "excited-state probability"), _o("W", "angular_frequency", "generalised Rabi frequency sqrt(Omega^2 + Delta^2)"),
              _o("P_max", "number", "largest excitation probability Omega^2/W^2"), _o("period", "time", "oscillation period 2 pi/W")],
     primary="P_excited", conditions=["Omega > 0", "t >= 0", "rotating-wave approximation, starting in the ground state"],
     method="P = (Omega^2/W^2) sin^2(W t/2), W = sqrt(Omega^2 + Delta^2)",
     latex=r"P_e(t) = \frac{\Omega^2}{\Omega^2 + \Delta^2}\sin^2\!\left(\frac{\sqrt{\Omega^2 + \Delta^2}\,t}{2}\right)",
     run=rabi, verify=_recheck(rabi),
     parse=lambda q: _pairs(q, r"rabi oscillation", {"omega": "Omega", "delta": "Delta", "t": "t"}, units={"Omega", "Delta", "t"}),
     reference=ref_rabi,
     wrong=_safe(lambda Omega, Delta, t: (lambda o: {**o, "P_excited": _r(math.sin(Omega * t / 2) ** 2)})(rabi(Omega, Delta, t))),
     random_inputs=lambda rng: (lambda Om: {"Omega": Om, "Delta": rng.uniform(-2, 2) * Om, "t": rng.uniform(0.05, 20) / Om})(_lu(rng, 1e3, 1e9)),
     templates=["Rabi oscillation: Omega = 1e6 rad/s, Delta = 0 rad/s, t = 1.5708 us"],
     examples=[{"inputs": {"Omega": 1e6, "Delta": 0.0, "t": math.pi / 1e6}, "outputs": {"P_excited": 1.0}},
               {"inputs": {"Omega": 1e6, "Delta": 1e6, "t": math.pi / (math.sqrt(2) * 1e6)}, "outputs": {"P_excited": 0.5, "P_max": 0.5}}],
     questions=[{"q": "Rabi oscillation: Omega = 1e6 rad/s, Delta = 0 rad/s, t = 1.5707963267948966 us", "outputs": {"P_excited": 0.5}},
                {"q": "Rabi oscillation: Omega = -1e6 rad/s, Delta = 0 rad/s, t = 1 us", "outcome": "INVALID_INPUT"},
                {"q": "Rabi oscillation: Omega = 1e6 rad/s, t = 1 us", "outcome": "NEEDS_INPUT"}])

_GYRO = {"electron": 1.76085962784e11, "proton": 2.6752218708e8}


def larmor(particle, B, t):
    if B <= 0 or t < 0:
        raise ValueError("the field must be positive and the time non-negative")
    w = _GYRO[particle] * B
    return _out(omega_L=w, f_L=w / (2 * math.pi), Sx_mean=_snap(math.cos(w * t)), P_plus_x=_snap(math.cos(w * t / 2) ** 2))


def ref_larmor(particle, B, t):
    import numpy as np
    import qutip
    from scipy import constants as sc
    g = sc.physical_constants["electron gyromag. ratio" if particle == "electron" else "proton gyromag. ratio"][0]
    w = g * B
    Hm = 0.5 * w * qutip.sigmaz()
    U = (-1j * Hm * t).expm()
    plus = (qutip.basis(2, 0) + qutip.basis(2, 1)).unit()
    psi = U * plus
    return {"omega_L": w, "Sx_mean": float(_snap(qutip.expect(qutip.sigmax(), psi))), "P_plus_x": float(_snap(abs(plus.overlap(psi)) ** 2))}


_add("larmor_precession",
     name="Larmor precession of a spin-1/2", topic="spin and qubits / spin precession",
     purpose="Larmor angular frequency of an electron or proton spin in a field B, and the spin component along x after time t for a spin starting along +x.",
     inputs=[{"symbol": "particle", "type": "choice", "choices": ["electron", "proton"], "meaning": "the spin-1/2 particle", "required": True},
             _phys("B", "magnetic_field", "magnetic field along z"), _phys("t", "time", "time since the spin pointed along +x")],
     outputs=[_o("omega_L", "angular_frequency", "Larmor angular frequency |gamma| B"), _o("f_L", "frequency", "Larmor frequency"),
              _o("Sx_mean", "number", "<sigma_x> at time t (mean spin along x in units of hbar/2)"),
              _o("P_plus_x", "number", "probability of measuring spin up along +x at time t")],
     primary="P_plus_x", conditions=["B > 0", "t >= 0"],
     method="omega = |gamma| B (CODATA gyromagnetic ratios), <sigma_x> = cos(omega t), P(+x) = cos^2(omega t/2)",
     latex=r"\omega_L = |\gamma| B,\quad P_{+x}(t) = \cos^2\!\left(\tfrac{\omega_L t}{2}\right)",
     run=larmor, verify=_recheck(larmor),
     parse=lambda q: _pairs(q, r"larmor precession", {"particle": "particle", "b": "B", "t": "t"}, units={"B", "t"}, words={"particle"}),
     reference=ref_larmor,
     wrong=_safe(lambda particle, B, t: (lambda o: {**o, "P_plus_x": _r(math.cos(_GYRO[particle] * B * t) ** 2)})(larmor(particle, B, t))),
     random_inputs=lambda rng: (lambda p, B: {"particle": p, "B": B, "t": rng.uniform(0.01, 60) / (_GYRO[p] * B)})(rng.choice(["electron", "proton"]), _lu(rng, 1e-4, 10)),
     templates=["Larmor precession: particle = proton, B = 1 T, t = 1 us"],
     examples=[{"inputs": {"particle": "proton", "B": 1.0, "t": 0.0}, "outputs": {"omega_L": 2.6752218708e8, "P_plus_x": 1.0}},
               {"inputs": {"particle": "electron", "B": 1.0, "t": math.pi / 1.76085962784e11}, "outputs": {"P_plus_x": 0.0, "Sx_mean": -1.0}}],
     questions=[{"q": "Larmor precession: particle = proton, B = 1 T, t = 0 s", "outputs": {"omega_L": 2.6752218708e8}},
                {"q": "Larmor precession: particle = proton, B = -1 T, t = 1 s", "outcome": "INVALID_INPUT"},
                {"q": "Larmor precession: particle = proton, B = 1 T", "outcome": "NEEDS_INPUT"}])


def _xlog2(p):
    return 0.0 if p <= 0 else -p * math.log2(p)


def qubit_entropy(r):
    r = _vec3(r, "r")
    L = math.sqrt(sum(x * x for x in r))
    if L > 1 + 1e-12:
        raise ValueError("a Bloch vector has length at most 1")
    L = min(L, 1.0)
    l1, l2 = (1 + L) / 2, (1 - L) / 2
    return _out(length=L, purity=(1 + L * L) / 2, S_bits=_snap(_xlog2(l1) + _xlog2(l2)), lambda_max=l1, lambda_min=_snap(l2))


def ref_qubit_entropy(r):
    import numpy as np
    import qutip
    rho = 0.5 * (qutip.qeye(2) + r[0] * qutip.sigmax() + r[1] * qutip.sigmay() + r[2] * qutip.sigmaz())
    ev = np.sort(rho.eigenenergies())
    return {"purity": float((rho * rho).tr().real), "S_bits": float(_snap(qutip.entropy_vn(rho, base=2))), "lambda_max": float(ev[1])}


def _rand_ball(rng, rmax=1.0):
    while True:
        v = [rng.uniform(-1, 1) for _ in range(3)]
        n = math.sqrt(sum(x * x for x in v))
        if 0.05 < n <= 1:
            s = rng.uniform(0.0, rmax) / n
            return [x * s for x in v]


_add("qubit_entropy_purity",
     name="Purity and entropy of a qubit state", topic="spin and qubits / mixed states and density matrices",
     purpose="Purity Tr(rho^2), von Neumann entropy (bits) and eigenvalues of the qubit density matrix with Bloch vector r.",
     inputs=[{"symbol": "r", "type": "vector", "min_length": 3, "max_length": 3, "meaning": "Bloch vector (rx, ry, rz), length at most 1", "required": True}],
     outputs=[_o("length", "number", "|r|"), _o("purity", "number", "Tr(rho^2) = (1 + |r|^2)/2"), _o("S_bits", "number", "von Neumann entropy in bits"),
              _o("lambda_max", "number", "larger eigenvalue (1 + |r|)/2"), _o("lambda_min", "number", "smaller eigenvalue (1 - |r|)/2")],
     primary="S_bits", conditions=["|r| <= 1"],
     method="eigenvalues (1 +- |r|)/2, S = -sum lambda log2 lambda",
     latex=r"\rho = \tfrac12(I + \vec r\cdot\vec\sigma),\quad S = -\sum_i \lambda_i\log_2\lambda_i",
     run=qubit_entropy, verify=_recheck(qubit_entropy),
     parse=lambda q: _pairs(q, r"qubit (?:entropy|purity)(?: and (?:entropy|purity))?", {"r": "r"}, vectors={"r"}),
     reference=ref_qubit_entropy,
     wrong=_safe(lambda r: (lambda o: {**o, "S_bits": _r(1 - o["length"]), "purity": _r(o["length"])})(qubit_entropy(r))),
     random_inputs=lambda rng: {"r": _rand_ball(rng)},
     templates=["Qubit entropy: r = 0.6, 0, 0"],
     examples=[{"inputs": {"r": [0.6, 0.0, 0.0]}, "outputs": {"purity": 0.68, "S_bits": _xlog2(0.8) + _xlog2(0.2)}},
               {"inputs": {"r": [0.0, 0.0, 1.0]}, "outputs": {"S_bits": 0.0, "purity": 1.0}},
               {"inputs": {"r": [0.0, 0.0, 0.0]}, "outputs": {"S_bits": 1.0, "purity": 0.5}}],
     questions=[{"q": "Qubit entropy: r = 0.6, 0, 0", "outputs": {"purity": 0.68}},
                {"q": "Qubit entropy: r = 1, 1, 0", "outcome": "INVALID_INPUT"},
                {"q": "Qubit entropy: r = 1, 0", "outcome": "INVALID_INPUT"}])


def qubit_fidelity(r, s):
    r, s = _vec3(r, "r"), _vec3(s, "s")
    lr, ls = math.sqrt(sum(x * x for x in r)), math.sqrt(sum(x * x for x in s))
    if lr > 1 + 1e-12 or ls > 1 + 1e-12:
        raise ValueError("a Bloch vector has length at most 1")
    dot = sum(a * b for a, b in zip(r, s))
    F = 0.5 * (1 + dot + math.sqrt(max(0.0, (1 - lr * lr) * (1 - ls * ls))))
    D = 0.5 * math.sqrt(sum((a - b) ** 2 for a, b in zip(r, s)))
    return _out(F=_snap(F), sqrt_F=_snap(math.sqrt(max(F, 0.0))), trace_distance=_snap(D))


def ref_qubit_fidelity(r, s):
    import qutip
    mk = lambda v: 0.5 * (qutip.qeye(2) + v[0] * qutip.sigmax() + v[1] * qutip.sigmay() + v[2] * qutip.sigmaz())   # noqa: E731
    a, b = mk(r), mk(s)
    f = float(qutip.fidelity(a, b))
    return {"F": _snap(f * f), "trace_distance": _snap(float(qutip.tracedist(a, b)))}


_add("qubit_fidelity",
     name="Fidelity and trace distance of two qubit states", topic="spin and qubits / distinguishing quantum states",
     purpose="The (Uhlmann) fidelity and the trace distance between two qubit states given by their Bloch vectors r and s.",
     inputs=[{"symbol": "r", "type": "vector", "min_length": 3, "max_length": 3, "meaning": "Bloch vector of the first state", "required": True},
             {"symbol": "s", "type": "vector", "min_length": 3, "max_length": 3, "meaning": "Bloch vector of the second state", "required": True}],
     outputs=[_o("F", "number", "fidelity (Tr sqrt(sqrt(rho) sigma sqrt(rho)))^2"), _o("sqrt_F", "number", "root fidelity"),
              _o("trace_distance", "number", "trace distance |r - s|/2")],
     primary="F", conditions=["|r|, |s| <= 1"],
     method="F = [1 + r.s + sqrt((1 - |r|^2)(1 - |s|^2))]/2, D = |r - s|/2",
     latex=r"F = \tfrac12\left[1 + \vec r\cdot\vec s + \sqrt{(1 - |\vec r|^2)(1 - |\vec s|^2)}\right]",
     run=qubit_fidelity, verify=_recheck(qubit_fidelity),
     parse=lambda q: _pairs(q, r"qubit fidelity|fidelity of two qubit states", {"r": "r", "s": "s"}, vectors={"r", "s"}),
     reference=ref_qubit_fidelity,
     wrong=_safe(lambda r, s: (lambda o: {**o, "F": _r(0.5 * (1 + sum(a * b for a, b in zip(r, s))))})(qubit_fidelity(r, s))),
     random_inputs=lambda rng: {"r": _rand_ball(rng, 0.95), "s": _rand_ball(rng, 0.95)},
     templates=["Qubit fidelity: r = 0, 0, 1; s = 1, 0, 0"],
     examples=[{"inputs": {"r": [0.0, 0.0, 1.0], "s": [1.0, 0.0, 0.0]}, "outputs": {"F": 0.5, "trace_distance": math.sqrt(2) / 2}},
               {"inputs": {"r": [0.0, 0.0, 0.0], "s": [0.0, 0.0, 0.6]}, "outputs": {"F": 0.9}}],
     questions=[{"q": "Qubit fidelity: r = 0, 0, 1; s = 1, 0, 0", "outputs": {"F": 0.5}},
                {"q": "Qubit fidelity: r = 0, 0, 2; s = 1, 0, 0", "outcome": "INVALID_INPUT"},
                {"q": "Qubit fidelity: r = 0, 0, 1", "outcome": "NEEDS_INPUT"}])


def chsh(a, a2, b, b2):
    E = lambda x, y: -math.cos(x - y)                     # noqa: E731  singlet correlation
    vals = [E(a, b), E(a, b2), E(a2, b), E(a2, b2)]
    S = vals[0] - vals[1] + vals[2] + vals[3]
    return _out(S=_snap(S), abs_S=_snap(abs(S)), E_ab=_snap(vals[0]), E_ab2=_snap(vals[1]), E_a2b=_snap(vals[2]), E_a2b2=_snap(vals[3]),
                violates=("yes" if abs(S) > 2 + 1e-9 else "no"))


def ref_chsh(a, a2, b, b2):
    import numpy as np
    import qutip
    singlet = (qutip.tensor(qutip.basis(2, 0), qutip.basis(2, 1)) - qutip.tensor(qutip.basis(2, 1), qutip.basis(2, 0))).unit()
    A = lambda t: np.cos(t) * qutip.sigmaz() + np.sin(t) * qutip.sigmax()   # noqa: E731  a spin measurement in the x-z plane
    E = lambda x, y: float(qutip.expect(qutip.tensor(A(x), A(y)), singlet))  # noqa: E731
    v = [E(a, b), E(a, b2), E(a2, b), E(a2, b2)]
    return {"S": _snap(v[0] - v[1] + v[2] + v[3]), "E_ab": _snap(v[0]), "E_a2b2": _snap(v[3])}


_add("chsh_bell_test",
     name="CHSH Bell-test value for the singlet state", topic="spin and qubits / entanglement and Bell inequalities",
     purpose="Correlations E(a, b) = -cos(a - b) of spin measurements on a singlet pair and the CHSH combination S = E(a,b) - E(a,b') + E(a',b) + E(a',b').",
     inputs=[_phys("a", "angle", "Alice's first setting (angle in the x-z plane)"), _phys("a2", "angle", "Alice's second setting a'"),
             _phys("b", "angle", "Bob's first setting"), _phys("b2", "angle", "Bob's second setting b'")],
     outputs=[_o("S", "number", "CHSH value"), _o("abs_S", "number", "|S| (local hidden variables allow at most 2)"), _o("E_ab", "number", "E(a, b)"),
              _o("E_ab2", "number", "E(a, b')"), _o("E_a2b", "number", "E(a', b)"), _o("E_a2b2", "number", "E(a', b')"),
              {"symbol": "violates", "kind": "number", "meaning": "whether |S| exceeds 2"}],
     primary="S", conditions=["the spin singlet state", "measurement directions in one plane"],
     method="E(x, y) = -cos(x - y); S = E(a,b) - E(a,b') + E(a',b) + E(a',b')",
     latex=r"S = E(a,b) - E(a,b') + E(a',b) + E(a',b'),\quad E(x,y) = -\cos(x - y)",
     run=chsh, verify=_recheck(chsh),
     parse=lambda q: _pairs(q, r"chsh(?: bell test| value)?|bell test", {"a": "a", "a2": "a2", "b": "b", "b2": "b2"}, units={"a", "a2", "b", "b2"}),
     reference=ref_chsh,
     wrong=_safe(lambda a, a2, b, b2: (lambda o: {**o, "S": _r(-math.cos(a - b) + math.cos(a - b2) - math.cos(a2 - b) + math.cos(a2 - b2))})(chsh(a, a2, b, b2))),
     random_inputs=lambda rng: {k: rng.uniform(0, 2 * math.pi) for k in ("a", "a2", "b", "b2")},
     templates=["CHSH: a = 0 deg, a2 = 90 deg, b = 45 deg, b2 = 135 deg"],
     examples=[{"inputs": {"a": 0.0, "a2": math.pi / 2, "b": math.pi / 4, "b2": 3 * math.pi / 4}, "outputs": {"S": -2 * math.sqrt(2), "violates": "yes"}},
               {"inputs": {"a": 0.0, "a2": 0.0, "b": 0.0, "b2": 0.0}, "outputs": {"S": -2.0, "violates": "no"}}],
     questions=[{"q": "CHSH: a = 0 deg, a2 = 90 deg, b = 45 deg, b2 = 135 deg", "outputs": {"abs_S": 2 * math.sqrt(2)}},
                {"q": "CHSH: a = 0 deg, a2 = 90 deg, b = 45 deg", "outcome": "NEEDS_INPUT"}])


def grover(n_qubits, M, k):
    if n_qubits < 1 or n_qubits > 60:
        raise ValueError("the register must have 1 to 60 qubits")
    N = 2 ** n_qubits
    if M < 1 or 2 * M >= N:
        raise ValueError("the number of marked items must be at least 1 and less than half the search space")
    if k < 0 or k > 10 ** 7:
        raise ValueError("the number of iterations must be between 0 and 10^7")
    th = math.asin(math.sqrt(M / N))
    kopt = math.floor(math.pi / (4 * th))
    return _out(P_success=math.sin((2 * k + 1) * th) ** 2, k_opt=float(kopt), P_at_k_opt=math.sin((2 * kopt + 1) * th) ** 2, theta=th)


def ref_grover(n_qubits, M, k):
    # explicit state-vector simulation: oracle (phase flip of the marked items) and diffusion, iterated
    import numpy as np
    N = 2 ** n_qubits
    psi = np.full(N, 1 / np.sqrt(N))
    marked = np.zeros(N, bool)
    marked[:M] = True
    probs = [float(np.sum(np.abs(psi[marked]) ** 2))]
    for _ in range(max(k, int(np.pi / 4 * np.sqrt(N / M)) + 3)):
        psi = np.where(marked, -psi, psi)
        psi = 2 * psi.mean() - psi
        probs.append(float(np.sum(np.abs(psi[marked]) ** 2)))
    kopt = next(i for i in range(len(probs) - 1) if probs[i + 1] < probs[i])
    return {"P_success": probs[k], "k_opt": float(kopt), "P_at_k_opt": probs[kopt]}


def _rand_grover(rng):
    n = rng.randint(2, 10)
    N = 2 ** n
    M = rng.randint(1, max(1, min(N // 2 - 1, 6)))
    th = math.asin(math.sqrt(M / N))
    return {"n_qubits": n, "M": M, "k": rng.randint(0, math.floor(math.pi / (4 * th)) + 2)}


_add("grover_search",
     name="Success probability of Grover's search", topic="quantum gates / quantum algorithms",
     purpose="The probability of measuring a marked item after k Grover iterations on n qubits with M marked items, and the best number of iterations.",
     inputs=[_intin("n_qubits", "number of qubits (search space N = 2^n)", 1, 60), _intin("M", "number of marked items", 1),
             _intin("k", "number of Grover iterations", 0)],
     outputs=[_o("P_success", "number", "probability of a marked item after k iterations"), _o("k_opt", "number", "best iteration count floor(pi/(4 theta))"),
              _o("P_at_k_opt", "number", "success probability at k_opt"), _o("theta", "angle", "rotation half-angle asin(sqrt(M/N))")],
     primary="P_success", conditions=["1 <= M < N/2", "0 <= k <= 10^7"],
     method="P_k = sin^2((2k + 1) theta), sin(theta) = sqrt(M/N); k_opt = floor(pi/(4 theta)) (the first maximum)",
     latex=r"P_k = \sin^2\!\big((2k+1)\theta\big),\quad \sin\theta = \sqrt{M/N}",
     run=grover, verify=_recheck(grover),
     parse=lambda q: _pairs(q, r"grover(?:'s)?(?: search)?", {"n_qubits": "n_qubits", "n": "n_qubits", "m": "M", "k": "k"}),
     reference=ref_grover,
     wrong=_safe(lambda n_qubits, M, k: (lambda o: {**o, "P_success": _r(math.sin((k + 1) * o["theta"]) ** 2)})(grover(n_qubits, M, k))),
     random_inputs=_rand_grover,
     templates=["Grover search: n_qubits = 4, M = 1, k = 3"],
     examples=[{"inputs": {"n_qubits": 2, "M": 1, "k": 1}, "outputs": {"P_success": 1.0, "k_opt": 1.0}},
               {"inputs": {"n_qubits": 4, "M": 1, "k": 3}, "outputs": {"P_success": math.sin(7 * math.asin(0.25)) ** 2, "k_opt": 3.0}}],
     questions=[{"q": "Grover search: n_qubits = 2, M = 1, k = 1", "outputs": {"P_success": 1.0}},
                {"q": "Grover search: n_qubits = 3, M = 4, k = 1", "outcome": "INVALID_INPUT"},
                {"q": "Grover search: n_qubits = 3, M = 1", "outcome": "NEEDS_INPUT"}])


def coherent_state(alpha, n):
    if alpha < 0:
        raise ValueError("give |alpha| (a non-negative number)")
    if n < 0:
        raise ValueError("n must be a whole number >= 0")
    a2 = alpha * alpha
    if alpha == 0:
        P = 1.0 if n == 0 else 0.0
    else:
        P = math.exp(-a2 + 2 * n * math.log(alpha) - math.lgamma(n + 1))
    return _out(P_n=P, mean_n=a2, variance=a2, P_0=math.exp(-a2))


def ref_coherent_state(alpha, n):
    import qutip
    N = 120
    psi = qutip.coherent(N, alpha, method="operator")
    p = abs(psi.full()[n, 0]) ** 2
    return {"P_n": float(p), "mean_n": float(qutip.expect(qutip.num(N), psi))}


_add("coherent_state_photon_statistics",
     name="Photon-number distribution of a coherent state", topic="photons and matter waves / coherent states of light",
     purpose="The probability of n photons in a coherent (laser) state |alpha>, its mean photon number and variance (Poisson statistics).",
     inputs=[_numin("alpha", "coherent amplitude |alpha|"), _intin("n", "photon number", 0)],
     outputs=[_o("P_n", "number", "probability of exactly n photons"), _o("mean_n", "number", "mean photon number |alpha|^2"),
              _o("variance", "number", "photon-number variance |alpha|^2"), _o("P_0", "number", "probability of no photons")],
     primary="P_n", conditions=["|alpha| >= 0", "n >= 0"],
     method="P(n) = exp(-|alpha|^2) |alpha|^(2n)/n! (computed in logarithms)",
     latex=r"P(n) = e^{-|\alpha|^2}\frac{|\alpha|^{2n}}{n!}",
     run=coherent_state, verify=_recheck(coherent_state),
     parse=lambda q: _pairs(q, r"coherent state(?: photon statistics)?", {"alpha": "alpha", "n": "n"}),
     reference=ref_coherent_state,
     wrong=_safe(lambda alpha, n: (lambda o: {**o, "P_n": _r(math.exp(-alpha) * alpha ** n / math.factorial(n))})(coherent_state(alpha, n))),
     random_inputs=lambda rng: (lambda al: {"alpha": al, "n": max(0, int(round(al * al + rng.uniform(-1.5, 1.5) * al)))})(rng.uniform(0.5, 4.0)),
     templates=["Coherent state: alpha = 2, n = 3"],
     examples=[{"inputs": {"alpha": 2.0, "n": 3}, "outputs": {"P_n": math.exp(-4) * 64 / 6, "mean_n": 4.0}},
               {"inputs": {"alpha": 0.0, "n": 0}, "outputs": {"P_n": 1.0}}],
     questions=[{"q": "Coherent state: alpha = 2, n = 3", "outputs": {"P_n": math.exp(-4) * 64 / 6}},
                {"q": "Coherent state: alpha = -2, n = 3", "outcome": "INVALID_INPUT"},
                {"q": "Coherent state: alpha = 2", "outcome": "NEEDS_INPUT"}])

# ================================================================================================ nuclear and particle physics
def _ref_decay_ode(s_end, target=None):
    """y' = -y integrated with scipy DOP853 (no exponential function assumed); y at s_end, or the s at which y = target."""
    from scipy.integrate import solve_ivp
    if target is None:
        sol = solve_ivp(lambda s, y: [-y[0]], (0.0, s_end), [1.0], method="DOP853", rtol=1e-13, atol=1e-300)
        return float(sol.y[0, -1])
    ev = lambda s, y: y[0] - target                      # noqa: E731
    ev.terminal = True
    sol = solve_ivp(lambda s, y: [-y[0]], (0.0, s_end), [1.0], method="DOP853", rtol=1e-13, atol=1e-300, events=ev)
    return float(sol.t_events[0][0])


def decay_law(T_half, N0, t):
    if T_half <= 0 or N0 <= 0 or t < 0:
        raise ValueError("the half-life and N0 must be positive and the time non-negative")
    lam = math.log(2) / T_half
    f = 2.0 ** (-t / T_half)
    return _out(N=N0 * f, fraction=f, decay_constant=lam, mean_life=1 / lam, A0=lam * N0, A=lam * N0 * f, half_lives=t / T_half)


def ref_decay_law(T_half, N0, t):
    import numpy as np
    lam = np.log(2) / T_half
    f = _ref_decay_ode(lam * t)
    return {"N": N0 * f, "fraction": f, "decay_constant": float(lam), "A": float(lam * N0 * f)}


_add("radioactive_decay_law",
     name="Radioactive decay law", topic="nuclear physics / radioactive decay",
     purpose="Nuclei remaining, fraction, decay constant, mean life and activity after time t, from the half-life and the initial number of nuclei.",
     inputs=[_phys("T_half", "time", "half-life"), _numin("N0", "initial number of nuclei"), _phys("t", "time", "elapsed time")],
     outputs=[_o("N", "number", "nuclei remaining"), _o("fraction", "number", "fraction remaining N/N0"), _o("decay_constant", "decay_constant", "decay constant ln2/T_half"),
              _o("mean_life", "time", "mean life 1/lambda"), _o("A0", "radioactivity", "initial activity (Bq)"), _o("A", "radioactivity", "activity at time t (Bq)"),
              _o("half_lives", "number", "number of half-lives elapsed")],
     primary="fraction", conditions=["T_half > 0", "N0 > 0", "t >= 0"],
     method="N = N0 2^(-t/T_half), lambda = ln 2/T_half, A = lambda N",
     latex=r"N(t) = N_0\,2^{-t/T_{1/2}} = N_0 e^{-\lambda t},\quad A = \lambda N",
     run=decay_law, verify=_recheck(decay_law),
     parse=lambda q: _pairs(q, r"(?:radioactive )?decay law", {"t_half": "T_half", "n0": "N0", "t": "t"}, units={"T_half", "t"}),
     reference=ref_decay_law,
     wrong=_safe(lambda T_half, N0, t: (lambda f: _out(N=N0 * f, fraction=f, decay_constant=1 / T_half, mean_life=T_half, A0=N0 / T_half, A=N0 * f / T_half,
                                                       half_lives=t / T_half))(math.exp(-t / T_half))),
     random_inputs=lambda rng: (lambda T: {"T_half": T, "N0": _lu(rng, 1e3, 1e24), "t": T * rng.uniform(0.01, 10)})(_lu(rng, 1e-3, 1e17)),
     templates=["Decay law: T_half = 5730 yr, N0 = 1e20, t = 11460 yr"],
     examples=[{"inputs": {"T_half": 10.0, "N0": 1000.0, "t": 20.0}, "outputs": {"N": 250.0, "fraction": 0.25, "decay_constant": math.log(2) / 10}},
               {"inputs": {"T_half": 3600.0, "N0": 1e20, "t": 0.0}, "outputs": {"A": 1e20 * math.log(2) / 3600}}],
     questions=[{"q": "Decay law: T_half = 5730 yr, N0 = 1e20, t = 11460 yr", "outputs": {"N": 2.5e19}},
                {"q": "Decay law: T_half = 0 s, N0 = 1e20, t = 5 s", "outcome": "INVALID_INPUT"},
                {"q": "Decay law: T_half = 5730 yr, N0 = 1e20", "outcome": "NEEDS_INPUT"}])


def decay_age(fraction, T_half):
    if T_half <= 0 or not (0 < fraction <= 1):
        raise ValueError("the remaining fraction must lie in (0, 1] and the half-life must be positive")
    t = -T_half * math.log2(fraction)
    return _out(t=_snap(t, T_half), half_lives=_snap(t / T_half), decay_constant=math.log(2) / T_half)


def ref_decay_age(fraction, T_half):
    import numpy as np
    lam = np.log(2) / T_half
    s = _ref_decay_ode(-np.log(fraction) * 2 + 5, target=fraction) if fraction < 1 else 0.0
    return {"t": float(s / lam), "half_lives": float(s / lam / T_half)}


_add("radioactive_dating_age",
     name="Age from the remaining fraction (radioactive dating)", topic="nuclear physics / radioactive dating",
     purpose="The time needed for a radioactive sample to decay to a given fraction of its original amount (for example carbon-14 dating).",
     inputs=[_numin("fraction", "fraction of the original nuclei (or activity) remaining, 0 < f <= 1 (or a percentage)"), _phys("T_half", "time", "half-life")],
     outputs=[_o("t", "time", "elapsed time (age)"), _o("half_lives", "number", "number of half-lives"), _o("decay_constant", "decay_constant", "decay constant")],
     primary="t", conditions=["0 < fraction <= 1", "T_half > 0"],
     method="t = T_half log2(1/f) = ln(1/f)/lambda",
     latex=r"t = \frac{T_{1/2}}{\ln 2}\ln\frac{N_0}{N}",
     run=decay_age, verify=_recheck(decay_age),
     parse=lambda q: (lambda o: o if o is None or not isinstance(o.get("fraction"), str) else {**o, "fraction": float(o["fraction"].rstrip("%").strip()) / 100})(
         _pairs(q, r"(?:radioactive )?dating(?: age)?|age from (?:the )?remaining fraction", {"fraction": "fraction", "f": "fraction", "t_half": "T_half"},
                units={"T_half"}, words=())) if "%" not in q else _parse_pct_age(q),
     reference=ref_decay_age,
     wrong=_safe(lambda fraction, T_half: _out(t=T_half * (1 - fraction) * 2, half_lives=(1 - fraction) * 2, decay_constant=math.log(2) / T_half)),
     random_inputs=lambda rng: {"fraction": rng.uniform(0.001, 0.95), "T_half": _lu(rng, 1e-3, 1e17)},
     templates=["Radioactive dating: fraction = 0.25, T_half = 5730 yr"],
     examples=[{"inputs": {"fraction": 0.25, "T_half": 5730.0}, "outputs": {"t": 11460.0, "half_lives": 2.0}},
               {"inputs": {"fraction": 1.0, "T_half": 5730.0}, "outputs": {"t": 0.0}}],
     questions=[{"q": "Radioactive dating: fraction = 0.25, T_half = 5730 yr", "outputs": {"half_lives": 2.0}},
                {"q": "Radioactive dating: fraction = 12.5%, T_half = 5730 yr", "outputs": {"half_lives": 3.0}},
                {"q": "Radioactive dating: fraction = 1.5, T_half = 5730 yr", "outcome": "INVALID_INPUT"},
                {"q": "Radioactive dating: fraction = 0.25", "outcome": "NEEDS_INPUT"}])


def _parse_pct_age(q):
    m = re.search(r"\bf(?:raction)?\s*=\s*(" + _NUM + r")\s*%", q, re.I)
    if not m:
        return None
    q2 = q[:m.start()] + f"fraction = {float(m.group(1)) / 100!r}" + q[m.end():]
    return _pairs(q2, r"(?:radioactive )?dating(?: age)?|age from (?:the )?remaining fraction", {"fraction": "fraction", "f": "fraction", "t_half": "T_half"}, units={"T_half"})


_NUCLIDES = {  # half-lives in seconds (ICRP-107, as used by radioactivedecay 0.6.1)
    "C-14": 179874478656.0, "H-3": 388781329.30560005, "Co-60": 166346024.445504, "Cs-137": 951980944.7479681,
    "Sr-90": 908523901.8432001, "I-131": 692988.48, "Rn-222": 330350.4, "Ra-226": 50491081728.0, "U-238": 1.4099634572544002e+17,
    "U-235": 2.221607596032e+16, "Pu-239": 760837487788.8, "K-40": 3.947771452608e+16, "Am-241": 13638903451.776001,
    "Tc-99m": 21654.0, "F-18": 6586.2, "P-32": 1232323.2, "Po-210": 11955686.4, "Th-232": 4.43374811424e+17, "Na-22": 82107965.967552,
    "Ir-192": 6378652.8, "Kr-85": 339426296.91648, "Xe-133": 452995.2, "Tl-201": 262483.2, "Ga-67": 281767.68, "In-111": 242326.08,
    "Pb-210": 700563758.9760001, "Be-7": 4598208.0, "Ni-63": 3158848300.608, "Fe-55": 86371306.68096001, "Cl-36": 9498634750080.0}


def nuclide_decay(nuclide, t):
    if t < 0:
        raise ValueError("the elapsed time must be non-negative")
    T = _NUCLIDES[nuclide]
    lam = math.log(2) / T
    return _out(fraction=2.0 ** (-t / T), half_life=T, half_life_years=T / 3.15576e7, decay_constant=lam, mean_life=1 / lam,
                specific_activity_per_atom=lam * 2.0 ** (-t / T))


def ref_nuclide_decay(nuclide, t):
    import radioactivedecay as rd
    inv = rd.Inventory({nuclide: 1.0}, "num")
    left = inv.decay(t, "s").numbers().get(nuclide, 0.0)
    return {"fraction": float(left), "half_life": float(rd.Nuclide(nuclide).half_life("s"))}


_add("nuclide_decay",
     name="Decay of a named radionuclide", topic="nuclear physics / radioactive decay of real nuclides",
     purpose="The fraction of a named radionuclide (C-14, Co-60, I-131, U-238, ...) left after time t, with its tabulated half-life, decay constant and mean life.",
     inputs=[{"symbol": "nuclide", "type": "choice", "choices": list(_NUCLIDES), "meaning": "the radionuclide (e.g. C-14, Cs-137, I-131)", "required": True},
             _phys("t", "time", "elapsed time")],
     outputs=[_o("fraction", "number", "fraction of the parent nuclide remaining"), _o("half_life", "time", "half-life (ICRP-107)"),
              _o("half_life_years", "number", "half-life in Julian years"), _o("decay_constant", "decay_constant", "decay constant"),
              _o("mean_life", "time", "mean life"), _o("specific_activity_per_atom", "radioactivity", "activity per original atom at time t (Bq)")],
     primary="fraction", conditions=["t >= 0", "one of the tabulated nuclides"],
     method="N/N0 = 2^(-t/T_half) with the ICRP-107 half-life of the nuclide",
     latex=r"\frac{N}{N_0} = 2^{-t/T_{1/2}}",
     run=nuclide_decay, verify=_recheck(nuclide_decay),
     parse=lambda q: _pairs(q, r"(?:nuclide|isotope) decay|decay of (?:a )?(?:named )?(?:radio)?nuclide", {"nuclide": "nuclide", "isotope": "nuclide", "t": "t"},
                            units={"t"}, words={"nuclide"}),
     reference=ref_nuclide_decay,
     wrong=_safe(lambda nuclide, t: (lambda o: {**o, "fraction": _r(math.exp(-t / _NUCLIDES[nuclide]))})(nuclide_decay(nuclide, t))),
     random_inputs=lambda rng: (lambda nu: {"nuclide": nu, "t": _NUCLIDES[nu] * rng.uniform(0.05, 6)})(rng.choice(list(_NUCLIDES))),
     templates=["Nuclide decay: nuclide = C-14, t = 1000 yr"],
     examples=[{"inputs": {"nuclide": "I-131", "t": 2 * 692988.48}, "outputs": {"fraction": 0.25}},
               {"inputs": {"nuclide": "C-14", "t": 0.0}, "outputs": {"fraction": 1.0, "half_life": 179874478656.0}}],
     questions=[{"q": "Nuclide decay: nuclide = I-131, t = 1385976.96 s", "outputs": {"fraction": 0.25}},
                {"q": "Nuclide decay: nuclide = Xx-999, t = 1 s", "outcome": "INVALID_INPUT"},
                {"q": "Nuclide decay: nuclide = C-14", "outcome": "NEEDS_INPUT"}])


def decay_chain(T_half_P, T_half_D, N0, t):
    if T_half_P <= 0 or T_half_D <= 0 or N0 <= 0 or t < 0:
        raise ValueError("half-lives and N0 must be positive and the time non-negative")
    lp, ld = math.log(2) / T_half_P, math.log(2) / T_half_D
    NP = N0 * math.exp(-lp * t)
    d = (ld - lp) * t
    if abs(d) > 1:
        ND = N0 * lp / (ld - lp) * (math.exp(-lp * t) - math.exp(-ld * t))
    else:
        phi = 1 - d / 2 + d * d / 6 - d ** 3 / 24 if abs(d) < 1e-5 else -math.expm1(-d) / d
        ND = N0 * lp * t * math.exp(-lp * t) * phi
    AP, AD = lp * NP, ld * ND
    return _out(N_parent=NP, N_daughter=_snap(ND, N0), A_parent=AP, A_daughter=_snap(AD, lp * N0 + ld * N0), activity_ratio=_snap(AD / AP))


def ref_decay_chain(T_half_P, T_half_D, N0, t):
    import numpy as np
    from scipy.integrate import solve_ivp
    lp, ld = np.log(2) / T_half_P, np.log(2) / T_half_D
    s = lp * t
    r = ld / lp
    sol = solve_ivp(lambda x, y: [-y[0], y[0] - r * y[1]], (0.0, s), [1.0, 0.0], method="DOP853", rtol=1e-13, atol=1e-20) if s > 0 else None
    yP, yD = (sol.y[0, -1], sol.y[1, -1]) if sol is not None else (1.0, 0.0)
    return {"N_parent": float(N0 * yP), "N_daughter": float(N0 * yD), "activity_ratio": float(r * yD / yP)}


_add("decay_chain_parent_daughter",
     name="Parent-daughter decay chain (Bateman)", topic="nuclear physics / decay chains and equilibrium",
     purpose="Numbers and activities of a parent and its radioactive daughter after time t, starting from pure parent (Bateman solution).",
     inputs=[_phys("T_half_P", "time", "half-life of the parent"), _phys("T_half_D", "time", "half-life of the daughter"),
             _numin("N0", "initial number of parent nuclei"), _phys("t", "time", "elapsed time")],
     outputs=[_o("N_parent", "number", "parent nuclei"), _o("N_daughter", "number", "daughter nuclei"), _o("A_parent", "radioactivity", "parent activity (Bq)"),
              _o("A_daughter", "radioactivity", "daughter activity (Bq)"), _o("activity_ratio", "number", "A_daughter/A_parent (tends to 1 in secular equilibrium)")],
     primary="N_daughter", conditions=["half-lives and N0 positive", "t >= 0", "no daughter at t = 0"],
     method="N_D = N0 lambda_P (e^(-lambda_P t) - e^(-lambda_D t))/(lambda_D - lambda_P), evaluated without cancellation when the decay constants are close",
     latex=r"N_D(t) = N_0\frac{\lambda_P}{\lambda_D - \lambda_P}\left(e^{-\lambda_P t} - e^{-\lambda_D t}\right)",
     run=decay_chain, verify=_recheck(decay_chain),
     parse=lambda q: _pairs(q, r"(?:parent[- ]daughter )?decay chain|bateman", {"t_half_p": "T_half_P", "t_half_d": "T_half_D", "n0": "N0", "t": "t"},
                            units={"T_half_P", "T_half_D", "t"}),
     reference=ref_decay_chain,
     wrong=_safe(lambda T_half_P, T_half_D, N0, t: (lambda o: {**o, "N_daughter": _r(N0 - o["N_parent"]), "activity_ratio": _r((N0 - o["N_parent"]) * T_half_P / (T_half_D * o["N_parent"]))})(
         decay_chain(T_half_P, T_half_D, N0, t))),
     random_inputs=lambda rng: (lambda TP: {"T_half_P": TP, "T_half_D": TP * _lu(rng, 0.05, 20), "N0": _lu(rng, 1e3, 1e22), "t": TP * rng.uniform(0.02, 5)})(_lu(rng, 1, 1e12)),
     templates=["Decay chain: T_half_P = 10 h, T_half_D = 1 h, N0 = 1e12, t = 5 h"],
     examples=[{"inputs": {"T_half_P": 2.0, "T_half_D": 1.0, "N0": 1.0, "t": 2.0}, "outputs": {"N_parent": 0.5, "N_daughter": 0.25}},
               {"inputs": {"T_half_P": 1.0, "T_half_D": 1.0, "N0": 1.0, "t": 1.0}, "outputs": {"N_daughter": 0.5 * math.log(2)}}],
     questions=[{"q": "Decay chain: T_half_P = 2 s, T_half_D = 1 s, N0 = 1, t = 2 s", "outputs": {"N_daughter": 0.25}},
                {"q": "Decay chain: T_half_P = 2 s, T_half_D = -1 s, N0 = 1, t = 2 s", "outcome": "INVALID_INPUT"},
                {"q": "Decay chain: T_half_P = 2 s, T_half_D = 1 s, N0 = 1", "outcome": "NEEDS_INPUT"}])


def two_body_decay(M, m1, m2, unit="MeV"):
    f = AMU_MEV if unit == "u" else 1.0
    M, m1, m2 = M * f, m1 * f, m2 * f
    if m1 < 0 or m2 < 0:
        raise ValueError("the product masses must be non-negative")
    Q = M - m1 - m2
    if Q <= 0:
        raise ValueError("the decay is forbidden: the parent is not heavier than the products")
    p = math.sqrt(Q * (M + m1 + m2) * (M - m1 + m2) * (M + m1 - m2)) / (2 * M)
    E1, E2 = math.hypot(p, m1), math.hypot(p, m2)
    K1 = p * p / (E1 + m1)
    K2 = p * p / (E2 + m2)
    return _out(Q_MeV=Q, p_MeV=p, E1_MeV=E1, E2_MeV=E2, K1_MeV=K1, K2_MeV=K2)


def ref_two_body_decay(M, m1, m2, unit="MeV"):
    # momentum balance in the parent's rest frame solved numerically for p (scipy brentq); Q from the kinetic energies
    from scipy import constants as sc
    from scipy.optimize import brentq
    f = sc.physical_constants["atomic mass constant energy equivalent in MeV"][0] if unit == "u" else 1.0
    M, m1, m2 = M * f, m1 * f, m2 * f
    p = brentq(lambda q: math.sqrt(q * q + m1 * m1) + math.sqrt(q * q + m2 * m2) - M, 0.0, M, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    E1, E2 = math.sqrt(p * p + m1 * m1), math.sqrt(p * p + m2 * m2)
    return {"p_MeV": p, "K1_MeV": E1 - m1, "K2_MeV": E2 - m2, "Q_MeV": (E1 - m1) + (E2 - m2)}


def _rand_two_body(rng):
    unit = rng.choice(["MeV", "u"])
    M = rng.uniform(100, 5000) if unit == "MeV" else rng.uniform(4, 240)
    a = rng.uniform(0.0, 0.9)
    Qf = _lu(rng, 1e-3, 0.5)
    m1 = a * M * (1 - Qf)
    m2 = (M * (1 - Qf) - m1) * rng.choice([1.0, 1.0, 0.0 + 1.0])
    return {"M": M, "m1": m1, "m2": m2, "unit": unit}


_add("two_body_decay",
     name="Kinematics of a two-body decay", topic="nuclear and particle physics / decay kinematics and Q value",
     purpose="Q value, momentum and kinetic energies of the two products when a particle or nucleus at rest decays into two bodies (alpha decay, pi -> mu nu, ...).",
     inputs=[_numin("M", "mass of the parent"), _numin("m1", "mass of product 1"), _numin("m2", "mass of product 2 (0 for a photon or neutrino)"),
             {"symbol": "unit", "type": "choice", "choices": ["MeV", "u"], "meaning": "unit of the masses: MeV/c^2 (default) or u", "required": False}],
     outputs=[_o("Q_MeV", "energy_mev", "Q value (M - m1 - m2) c^2"), _o("p_MeV", "energy_mev", "momentum of each product times c"),
              _o("E1_MeV", "energy_mev", "total energy of product 1"), _o("E2_MeV", "energy_mev", "total energy of product 2"),
              _o("K1_MeV", "energy_mev", "kinetic energy of product 1"), _o("K2_MeV", "energy_mev", "kinetic energy of product 2")],
     primary="K1_MeV", conditions=["M > m1 + m2", "m1, m2 >= 0"],
     method="p = sqrt[(M^2 - (m1 + m2)^2)(M^2 - (m1 - m2)^2)]/(2M), E_i = sqrt(p^2 + m_i^2), K_i = p^2/(E_i + m_i) (relativistic)",
     latex=r"p = \frac{\sqrt{\left[M^2 - (m_1 + m_2)^2\right]\left[M^2 - (m_1 - m_2)^2\right]}}{2M}",
     run=two_body_decay, verify=_recheck(two_body_decay),
     parse=lambda q: _pairs(q, r"two[- ]body decay", {"m": "M", "m1": "m1", "m2": "m2", "unit": "unit"}, words={"unit"}),
     reference=ref_two_body_decay,
     wrong=_safe(lambda M, m1, m2, unit="MeV": (lambda o: {**o, "K1_MeV": _r(o["Q_MeV"] * m2 / (m1 + m2)) if m1 + m2 > 0 else 0.0,
                                                           "K2_MeV": _r(o["Q_MeV"] * m1 / (m1 + m2)) if m1 + m2 > 0 else 0.0, "p_MeV": 0.0})(two_body_decay(M, m1, m2, unit))),
     random_inputs=_rand_two_body,
     templates=["Two-body decay: M = 139.57039, m1 = 105.6583755, m2 = 0"],
     examples=[{"inputs": {"M": 10.0, "m1": 3.0, "m2": 4.0}, "outputs": {"Q_MeV": 3.0, "p_MeV": math.sqrt(3 * 17 * 11 * 9) / 20}},
               {"inputs": {"M": 139.57039, "m1": 105.6583755, "m2": 0.0}, "outputs": {"p_MeV": (139.57039 ** 2 - 105.6583755 ** 2) / (2 * 139.57039)}}],
     questions=[{"q": "Two-body decay: M = 139.57039, m1 = 105.6583755, m2 = 0", "outputs": {"p_MeV": (139.57039 ** 2 - 105.6583755 ** 2) / (2 * 139.57039)}},
                {"q": "Two-body decay: M = 10, m1 = 6, m2 = 5", "outcome": "INVALID_INPUT"},
                {"q": "Two-body decay: M = 10, m1 = 6", "outcome": "NEEDS_INPUT"}])


def reaction_threshold(m_a, m_X, m_Y, m_b):
    if min(m_a, m_X, m_Y, m_b) < 0 or m_X <= 0:
        raise ValueError("masses must be non-negative and the target mass positive")
    mi = (m_a + m_X) * AMU_MEV
    mf = (m_Y + m_b) * AMU_MEV
    Q = ((m_a + m_X) - (m_Y + m_b)) * AMU_MEV
    K = 0.0 if Q >= 0 else (-Q) * (mf + mi) / (2 * m_X * AMU_MEV)
    return _out(Q_MeV=Q, K_threshold_MeV=K, endothermic=("yes" if Q < 0 else "no"))


def ref_reaction_threshold(m_a, m_X, m_Y, m_b):
    # the beam kinetic energy at which sqrt(s), built from the four-momenta, just equals the final rest energy (brentq)
    from scipy import constants as sc
    from scipy.optimize import brentq
    u = sc.physical_constants["atomic mass constant energy equivalent in MeV"][0]
    ma, mX, mf = m_a * u, m_X * u, (m_Y + m_b) * u

    def rs(K):
        Ea = K + ma
        pa = math.sqrt(max(Ea * Ea - ma * ma, 0.0))
        return math.sqrt((Ea + mX) ** 2 - pa * pa)
    Kbig = 10 * (mf + ma + mX)
    Q = (rs(Kbig) - m_Y * u - m_b * u) - (rs(Kbig) - ma - mX)
    K = 0.0 if mf <= ma + mX else brentq(lambda K: rs(K) - mf, 0.0, Kbig, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"Q_MeV": Q, "K_threshold_MeV": K}


def _rand_reaction(rng):
    ma, mX = rng.uniform(1, 20), rng.uniform(1, 60)
    tot = ma + mX
    Qu = rng.uniform(-0.02, 0.02)
    mY = rng.uniform(0.2, 0.9) * (tot - Qu)
    return {"m_a": ma, "m_X": mX, "m_Y": mY, "m_b": tot - Qu - mY}


_add("nuclear_reaction_threshold",
     name="Q value and threshold energy of a nuclear reaction", topic="nuclear physics / reactions, Q value and threshold",
     purpose="Q value of a + X -> Y + b from the masses (in u) and, for an endothermic reaction, the minimum beam kinetic energy on a target at rest (relativistic).",
     inputs=[_numin("m_a", "projectile mass (u)"), _numin("m_X", "target mass (u)"), _numin("m_Y", "heavy product mass (u)"), _numin("m_b", "light product mass (u)")],
     outputs=[_o("Q_MeV", "energy_mev", "Q value"), _o("K_threshold_MeV", "energy_mev", "threshold kinetic energy of the projectile (0 if exothermic)"),
              {"symbol": "endothermic", "kind": "number", "meaning": "whether the reaction needs energy (Q < 0)"}],
     primary="Q_MeV", conditions=["masses >= 0", "target at rest"],
     method="Q = (m_a + m_X - m_Y - m_b) c^2; K_th = -Q (m_a + m_X + m_Y + m_b)/(2 m_X) (exact relativistic form)",
     latex=r"Q = (m_a + m_X - m_Y - m_b)c^2,\quad K_{th} = -Q\,\frac{m_a + m_X + m_Y + m_b}{2 m_X}",
     run=reaction_threshold, verify=_recheck(reaction_threshold),
     parse=lambda q: _pairs(q, r"(?:nuclear )?reaction (?:q value|threshold)(?: and threshold| energy)?", {"m_a": "m_a", "m_x": "m_X", "m_y": "m_Y", "m_b": "m_b"}),
     reference=ref_reaction_threshold,
     wrong=_safe(lambda m_a, m_X, m_Y, m_b: (lambda o: {**o, "K_threshold_MeV": _r(max(0.0, -o["Q_MeV"])), "Q_MeV": _r(-o["Q_MeV"])})(reaction_threshold(m_a, m_X, m_Y, m_b))),
     random_inputs=_rand_reaction,
     templates=["Reaction threshold: m_a = 4.002603, m_X = 14.003074, m_Y = 16.999131, m_b = 1.007825"],
     examples=[{"inputs": {"m_a": 4.002603254, "m_X": 14.003074004, "m_Y": 16.999131757, "m_b": 1.00782503207},
                "outputs": {"Q_MeV": (4.002603254 + 14.003074004 - 16.999131757 - 1.00782503207) * AMU_MEV}}],
     questions=[{"q": "Reaction threshold: m_a = 4.002603254, m_X = 14.003074004, m_Y = 16.999131757, m_b = 1.00782503207",
                 "outputs": {"Q_MeV": (4.002603254 + 14.003074004 - 16.999131757 - 1.00782503207) * AMU_MEV}},
                {"q": "Reaction threshold: m_a = 4, m_X = 0, m_Y = 3, m_b = 1", "outcome": "INVALID_INPUT"},
                {"q": "Reaction threshold: m_a = 4, m_X = 14", "outcome": "NEEDS_INPUT"}])


def coulomb_barrier(Z1, A1, Z2, A2, r0):
    for v, n in ((Z1, "Z1"), (Z2, "Z2"), (A1, "A1"), (A2, "A2")):
        if v < 1:
            raise ValueError(f"{n} must be a positive whole number")
    if Z1 > A1 or Z2 > A2:
        raise ValueError("Z cannot exceed A")
    if r0 <= 0:
        raise ValueError("the radius parameter must be positive")
    R = r0 * (A1 ** (1 / 3) + A2 ** (1 / 3))
    hbarc = HBAR * C
    V = Z1 * Z2 * ALPHA * hbarc / R
    return _out(V_MeV=V / MEV, R=R, R_fm=R * 1e15)


def ref_coulomb_barrier(Z1, A1, Z2, A2, r0):
    # the work done against the Coulomb force from infinity to the touching distance (scipy quadrature, k = 1/(4 pi eps0))
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    R = r0 * (np.cbrt(A1) + np.cbrt(A2))
    k = 1 / (4 * np.pi * sc.epsilon_0)
    W = quad(lambda x: k * Z1 * Z2 * sc.e ** 2 / (R / x) ** 2 * (R / x ** 2), 0.0, 1.0, epsabs=0, epsrel=1e-13)[0]   # r = R/x
    return {"V_MeV": W / (1e6 * sc.e), "R_fm": R * 1e15}


_add("coulomb_barrier",
     name="Coulomb barrier between two nuclei", topic="nuclear physics / nuclear size and the Coulomb barrier",
     purpose="The Coulomb potential energy of two nuclei just touching, with radii R = r0 A^(1/3).",
     inputs=[_intin("Z1", "charge number of nucleus 1", 1), _intin("A1", "mass number of nucleus 1", 1), _intin("Z2", "charge number of nucleus 2", 1),
             _intin("A2", "mass number of nucleus 2", 1), _phys("r0", "length", "radius parameter (typically 1.2 fm)")],
     outputs=[_o("V_MeV", "energy_mev", "Coulomb barrier height"), _o("R", "length", "touching distance r0 (A1^(1/3) + A2^(1/3))"), _o("R_fm", "length_fm", "the same in fm")],
     primary="V_MeV", conditions=["1 <= Z <= A", "r0 > 0"],
     method="V = Z1 Z2 e^2/(4 pi eps0 R) = Z1 Z2 alpha hbar c/R",
     latex=r"V_C = \frac{Z_1 Z_2 e^2}{4\pi\varepsilon_0\,r_0\left(A_1^{1/3} + A_2^{1/3}\right)}",
     run=coulomb_barrier, verify=_recheck(coulomb_barrier),
     parse=lambda q: _pairs(q, r"coulomb barrier", {"z1": "Z1", "a1": "A1", "z2": "Z2", "a2": "A2", "r0": "r0"}, units={"r0"}),
     reference=ref_coulomb_barrier,
     wrong=_safe(lambda Z1, A1, Z2, A2, r0: (lambda o: {**o, "V_MeV": _r(o["V_MeV"] * (A1 ** (1 / 3) + A2 ** (1 / 3)) / ((A1 + A2) ** (1 / 3)))})(coulomb_barrier(Z1, A1, Z2, A2, r0))),
     random_inputs=lambda rng: (lambda A1, A2: {"A1": A1, "Z1": max(1, A1 // 2 - rng.randint(0, A1 // 8)), "A2": A2, "Z2": max(1, A2 // 2 - rng.randint(0, A2 // 8)),
                                                "r0": rng.uniform(1.1, 1.5) * 1e-15})(rng.randint(1, 240), rng.randint(1, 240)),
     templates=["Coulomb barrier: Z1 = 2, A1 = 4, Z2 = 79, A2 = 197, r0 = 1.2 fm"],
     examples=[{"inputs": {"Z1": 1, "A1": 1, "Z2": 1, "A2": 1, "r0": 1e-15}, "outputs": {"R_fm": 2.0, "V_MeV": ALPHA * HBAR * C / 2e-15 / MEV}}],
     questions=[{"q": "Coulomb barrier: Z1 = 1, A1 = 1, Z2 = 1, A2 = 1, r0 = 1 fm", "outputs": {"V_MeV": ALPHA * HBAR * C / 2e-15 / MEV}},
                {"q": "Coulomb barrier: Z1 = 5, A1 = 4, Z2 = 79, A2 = 197, r0 = 1.2 fm", "outcome": "INVALID_INPUT"},
                {"q": "Coulomb barrier: Z1 = 2, A1 = 4, Z2 = 79, A2 = 197", "outcome": "NEEDS_INPUT"}])

_PARTICLES = {  # name: (particle-library name, mass in MeV/c^2, mean lifetime in ns), PDG values carried by particle 1.0.1
    "pi+": ("pi+", 139.57039, 26.032746280292145), "K+": ("K+", 493.677, 12.379386062646352),
    "K_S0": ("K(S)0", 497.611, 0.0895429010381056), "K_L0": ("K(L)0", 497.611, 51.143120198205644),
    "mu-": ("mu-", 105.6583755, 2196.981174899978), "tau-": ("tau-", 1776.93, 0.0002903449302827113),
    "n": ("n", 939.5654219, 878318597479.1921), "Lambda": ("Lambda", 1115.683, 0.26171449580552947),
    "Sigma+": ("Sigma+", 1189.37, 0.08018174649176595), "Xi-": ("Xi-", 1321.71, 0.16373431764947924),
    "Omega-": ("Omega-", 1672.45, 0.08207131632804322), "D+": ("D+", 1869.66, 0.0010332997754331345),
    "D0": ("D0", 1864.84, 0.0004103565816402161), "B+": ("B+", 5279.41, 0.0016369359784901931), "B0": ("B0", 5279.72, 0.0015113937013798086)}


def decay_length(particle, p):
    if p < 0:
        raise ValueError("the momentum must be non-negative")
    _, m, tau_ns = _PARTICLES[particle]
    pm = p * 1000.0                                      # GeV/c -> MeV/c
    bg = pm / m
    g = math.hypot(1.0, bg)
    tau = tau_ns * 1e-9
    return _out(gamma=g, beta=bg / g, beta_gamma=bg, lifetime_lab=g * tau, mean_decay_length=bg * C * tau, proper_lifetime=tau, mass_MeV=m)


def ref_decay_length(particle, p):
    import numpy as np
    from particle import Particle
    from scipy import constants as sc
    P = Particle.from_name(_PARTICLES[particle][0])
    m, tau = float(P.mass), float(P.lifetime) * 1e-9
    E, pz = _ref_boost(float(np.arcsinh(p * 1000.0 / m))) @ np.array([m, 0.0])   # the boosted rest four-momentum
    return {"gamma": float(E / m), "beta_gamma": float(pz / m), "mean_decay_length": float(pz / m * sc.c * tau), "mass_MeV": m}


_add("particle_decay_length",
     name="Mean decay length of a relativistic unstable particle", topic="particle physics / lifetimes and time dilation",
     purpose="Lorentz factor, lab-frame lifetime and mean decay length beta gamma c tau of a pion, kaon, muon, hyperon, charm or bottom meson with momentum p.",
     inputs=[{"symbol": "particle", "type": "choice", "choices": list(_PARTICLES), "meaning": "the particle (pi+, K+, K_S0, K_L0, mu-, tau-, n, Lambda, ...)", "required": True},
             _numin("p", "momentum in GeV/c")],
     outputs=[_o("gamma", "number", "Lorentz factor"), _o("beta", "number", "v/c"), _o("beta_gamma", "number", "beta gamma = p/(m c)"),
              _o("lifetime_lab", "time", "mean lifetime in the lab"), _o("mean_decay_length", "length", "mean decay length beta gamma c tau"),
              _o("proper_lifetime", "time", "mean proper lifetime (PDG)"), _o("mass_MeV", "number", "mass (MeV/c^2, PDG)")],
     primary="mean_decay_length", conditions=["p >= 0", "one of the tabulated particles"],
     method="beta gamma = p/(m c), L = beta gamma c tau, t_lab = gamma tau",
     latex=r"L = \beta\gamma c\tau = \frac{p}{mc}\,c\tau",
     run=decay_length, verify=_recheck(decay_length),
     parse=lambda q: _pairs(q, r"(?:particle )?decay length", {"particle": "particle", "p": "p"}, words={"particle"}),
     reference=ref_decay_length,
     wrong=_safe(lambda particle, p: (lambda o: {**o, "mean_decay_length": _r(o["gamma"] * C * o["proper_lifetime"])})(decay_length(particle, p))),
     random_inputs=lambda rng: {"particle": rng.choice(list(_PARTICLES)), "p": _lu(rng, 0.01, 500)},
     templates=["Decay length: particle = K_S0, p = 10"],
     examples=[{"inputs": {"particle": "mu-", "p": 0.1056583755}, "outputs": {"beta_gamma": 1.0, "mean_decay_length": C * 2196.981174899978e-9}}],
     questions=[{"q": "Decay length: particle = mu-, p = 0.1056583755", "outputs": {"beta_gamma": 1.0}},
                {"q": "Decay length: particle = quark, p = 10", "outcome": "INVALID_INPUT"},
                {"q": "Decay length: particle = K_S0", "outcome": "NEEDS_INPUT"}])

# ================================================================================================ statistical physics
def _newton_root(g, dg, x):
    for _ in range(100):
        step = g(x) / dg(x)
        x -= step
        if abs(step) <= 1e-16 * abs(x):
            break
    return x


X_WIEN_L = _newton_root(lambda x: x - 5 + 5 * math.exp(-x), lambda x: 1 - 5 * math.exp(-x), 5.0)   # x = 5(1 - e^-x)
X_WIEN_F = _newton_root(lambda x: x - 3 + 3 * math.exp(-x), lambda x: 1 - 3 * math.exp(-x), 3.0)   # x = 3(1 - e^-x)
SIGMA_SB = 2 * math.pi ** 5 * KB ** 4 / (15 * H ** 3 * C ** 2)


def wien(T):
    if T <= 0:
        raise ValueError("the temperature must be positive")
    b = H * C / (X_WIEN_L * KB)
    return _out(lambda_max=b / T, lambda_max_nm=b / T * 1e9, nu_max=X_WIEN_F * KB * T / H, b=b)


def ref_wien(T):
    # the maximum of Planck's law located numerically: brentq on the complex-step derivative of ln B (no Wien equation)
    from scipy import constants as sc
    from scipy.optimize import brentq
    a = sc.h * sc.c / (sc.k * T)

    def dlnB_l(lam):
        z = complex(lam, lam * 1e-30)
        return (-5 * cmath.log(z) - cmath.log(cmath.exp(a / z) - 1)).imag / (lam * 1e-30)
    lam = brentq(dlnB_l, 0.2 * 2.9e-3 / T, 5 * 2.9e-3 / T, xtol=1e-300, rtol=8.9e-16)
    bnu = sc.h / (sc.k * T)

    def dlnB_f(nu):
        z = complex(nu, nu * 1e-30)
        return (3 * cmath.log(z) - cmath.log(cmath.exp(bnu * z) - 1)).imag / (nu * 1e-30)
    nu = brentq(dlnB_f, 0.2 * 5.88e10 * T, 5 * 5.88e10 * T, xtol=1e-300, rtol=8.9e-16)
    return {"lambda_max_nm": lam * 1e9, "nu_max": nu, "b": sc.physical_constants["Wien wavelength displacement law constant"][0]}


_add("wien_peak",
     name="Peak of blackbody radiation (Wien's displacement law)", topic="statistical physics / blackbody radiation",
     purpose="The wavelength and frequency at which a blackbody at temperature T radiates most strongly.",
     inputs=[_phys("T", "temperature", "temperature of the blackbody")],
     outputs=[_o("lambda_max", "length", "peak wavelength (of B_lambda)"), _o("lambda_max_nm", "length_nm", "peak wavelength in nm"),
              _o("nu_max", "frequency", "peak frequency (of B_nu; not c/lambda_max)"), _o("b", "wien_constant", "Wien's constant b = hc/(x k)")],
     primary="lambda_max_nm", conditions=["T > 0"],
     method="lambda_max = h c/(x k T) with x = 5(1 - e^-x) = 4.9651...; nu_max = x' k T/h with x' = 3(1 - e^-x') = 2.8214...",
     latex=r"\lambda_{max} T = b = \frac{hc}{4.965\,k_B}",
     run=wien, verify=_recheck(wien),
     parse=lambda q: _pairs(q, r"wien(?:'s)?(?: displacement law| peak)?|blackbody peak", {"t": "T"}, units={"T"}),
     reference=ref_wien,
     wrong=_safe(lambda T: (lambda o: {**o, "lambda_max_nm": _r(H * C / (5 * KB * T) * 1e9), "nu_max": _r(C / (H * C / (X_WIEN_L * KB * T)))})(wien(T))),
     random_inputs=lambda rng: {"T": _lu(rng, 3, 1e5)},
     templates=["Wien's displacement law: T = 5778 K"],
     examples=[{"inputs": {"T": 5778.0}, "outputs": {"lambda_max_nm": 0.0028977719551851727 / 5778 * 1e9}},
               {"inputs": {"T": 300.0}, "outputs": {"b": 0.0028977719551851727}}],
     questions=[{"q": "Wien's displacement law: T = 5778 K", "outputs": {"lambda_max_nm": 0.0028977719551851727 / 5778 * 1e9}},
                {"q": "Wien's displacement law: T = -5 K", "outcome": "INVALID_INPUT"},
                {"q": "Wien's displacement law:", "outcome": "NEEDS_INPUT"}])


def _ref_planck_integral(lo=0.0, hi=None, power=3):
    import numpy as np
    from scipy.integrate import quad
    f = lambda t: t ** power / np.expm1(t) if t > 0 else 0.0   # noqa: E731
    return quad(f, lo, np.inf if hi is None else hi, epsabs=0, epsrel=1e-13, limit=400)[0]


def stefan_boltzmann(T, emissivity, A):
    if T <= 0 or A <= 0 or not (0 < emissivity <= 1):
        raise ValueError("T and A must be positive and the emissivity between 0 and 1")
    M = emissivity * SIGMA_SB * T ** 4
    return _out(sigma=SIGMA_SB, M=M, P=M * A)


def ref_stefan_boltzmann(T, emissivity, A):
    # pi times Planck's law integrated over all wavelengths (scipy quadrature in x = hc/(lambda k T))
    import numpy as np
    from scipy import constants as sc
    I = _ref_planck_integral()
    M = emissivity * np.pi * 2 * sc.k ** 4 * T ** 4 / (sc.h ** 3 * sc.c ** 2) * I
    return {"M": M, "P": M * A, "sigma": sc.physical_constants["Stefan-Boltzmann constant"][0]}


_add("stefan_boltzmann",
     name="Radiated power (Stefan-Boltzmann law)", topic="statistical physics / blackbody radiation",
     purpose="Radiant exitance and total power emitted by a surface of area A, emissivity e at temperature T.",
     inputs=[_phys("T", "temperature", "surface temperature"), _numin("emissivity", "emissivity (1 for a blackbody)"), _phys("A", "area", "radiating area")],
     outputs=[_o("sigma", "stefan_boltzmann_constant", "Stefan-Boltzmann constant 2 pi^5 k^4/(15 h^3 c^2)"), _o("M", "intensity", "radiant exitance e sigma T^4"),
              _o("P", "power", "total radiated power")],
     primary="P", conditions=["T > 0", "A > 0", "0 < emissivity <= 1"],
     method="P = e sigma A T^4, sigma = 2 pi^5 k^4/(15 h^3 c^2)",
     latex=r"P = \varepsilon\sigma A T^4,\quad \sigma = \frac{2\pi^5 k_B^4}{15 h^3 c^2}",
     run=stefan_boltzmann, verify=_recheck(stefan_boltzmann),
     parse=lambda q: _pairs(q, r"stefan[- ]boltzmann(?: law)?|radiated power", {"t": "T", "emissivity": "emissivity", "e": "emissivity", "a": "A"}, units={"T", "A"}),
     reference=ref_stefan_boltzmann,
     wrong=_safe(lambda T, emissivity, A: (lambda o: {**o, "P": _r(emissivity * SIGMA_SB * T ** 3 * A), "M": _r(emissivity * SIGMA_SB * T ** 3)})(stefan_boltzmann(T, emissivity, A))),
     random_inputs=lambda rng: {"T": _lu(rng, 10, 1e5), "emissivity": rng.uniform(0.05, 1.0), "A": _lu(rng, 1e-6, 1e3)},
     templates=["Stefan-Boltzmann: T = 5778 K, emissivity = 1, A = 6.09e18 m^2"],
     examples=[{"inputs": {"T": 1000.0, "emissivity": 1.0, "A": 1.0}, "outputs": {"M": 5.6703744191844314e-08 * 1e12}},
               {"inputs": {"T": 300.0, "emissivity": 0.5, "A": 2.0}, "outputs": {"P": 5.6703744191844314e-08 * 300.0 ** 4}}],
     questions=[{"q": "Stefan-Boltzmann: T = 1000 K, emissivity = 1, A = 1 m^2", "outputs": {"P": 5.6703744191844314e-08 * 1e12}},
                {"q": "Stefan-Boltzmann: T = 1000 K, emissivity = 1.5, A = 1 m^2", "outcome": "INVALID_INPUT"},
                {"q": "Stefan-Boltzmann: T = 1000 K, emissivity = 1", "outcome": "NEEDS_INPUT"}])


def planck_radiance(lam, T):
    if lam <= 0 or T <= 0:
        raise ValueError("the wavelength and the temperature must be positive")
    x = H * C / (lam * KB * T)
    if x > 700:
        nbar = math.exp(-x)
    else:
        nbar = 1 / math.expm1(x)
    B = 2 * H * C * C / lam ** 5 * nbar
    return _out(B_lambda=B, B_lambda_per_nm=B * 1e-9, occupation=nbar, x=x)


def ref_planck_radiance(lam, T):
    import astropy.units as u
    from astropy.modeling.models import BlackBody
    from astropy import constants as const
    bb = BlackBody(temperature=T * u.K)
    wl = lam * u.m
    Bl = (bb(wl) * const.c / wl ** 2).to(u.W / (u.m ** 3 * u.sr)).value     # astropy's own blackbody, B_nu -> B_lambda
    return {"B_lambda": float(Bl), "B_lambda_per_nm": float(Bl) * 1e-9}


_add("planck_spectral_radiance",
     name="Spectral radiance of a blackbody (Planck's law)", topic="statistical physics / blackbody radiation",
     purpose="The spectral radiance B_lambda of a blackbody at temperature T and wavelength lambda, and the photon occupation number of that mode.",
     inputs=[_phys("lam", "length", "wavelength"), _phys("T", "temperature", "temperature")],
     outputs=[_o("B_lambda", "spectral_radiance_per_wavelength", "spectral radiance (W per m^2 per sr per m of wavelength)"),
              _o("B_lambda_per_nm", "number", "spectral radiance in W/(m^2 sr nm)"), _o("occupation", "number", "mean photon number 1/(e^x - 1)"),
              _o("x", "number", "h c/(lambda k T)")],
     primary="B_lambda", conditions=["lambda > 0", "T > 0"],
     method="B_lambda = (2 h c^2/lambda^5) / (exp(h c/(lambda k T)) - 1)",
     latex=r"B_\lambda(T) = \frac{2hc^2}{\lambda^5}\,\frac{1}{e^{hc/(\lambda k_B T)} - 1}",
     run=planck_radiance, verify=_recheck(planck_radiance),
     parse=lambda q: _pairs(q, r"planck(?:'s law| spectral radiance)?|spectral radiance", {"lambda": "lam", "lam": "lam", "t": "T"}, units={"lam", "T"}),
     reference=ref_planck_radiance,
     wrong=_safe(lambda lam, T: (lambda o: {**o, "B_lambda": _r(2 * C * KB * T / lam ** 4), "B_lambda_per_nm": _r(2 * C * KB * T / lam ** 4 * 1e-9)})(planck_radiance(lam, T))),
     random_inputs=lambda rng: (lambda T: {"T": T, "lam": 2.9e-3 / T * _lu(rng, 0.15, 30)})(_lu(rng, 10, 1e5)),
     templates=["Planck's law: lambda = 500 nm, T = 5778 K"],
     examples=[{"inputs": {"lam": 500e-9, "T": 5778.0}, "outputs": {"B_lambda": 2 * H * C * C / (500e-9) ** 5 / math.expm1(H * C / (500e-9 * KB * 5778))}}],
     questions=[{"q": "Planck's law: lambda = 500 nm, T = 5778 K", "outputs": {"B_lambda": 2 * H * C * C / (500e-9) ** 5 / math.expm1(H * C / (500e-9 * KB * 5778))}},
                {"q": "Planck's law: lambda = 500 nm, T = 0 K", "outcome": "INVALID_INPUT"},
                {"q": "Planck's law: lambda = 500 nm", "outcome": "NEEDS_INPUT"}])


def _bernoulli(n):
    B = [Fraction(1)]
    for m in range(1, n + 1):
        B.append(-sum(math.comb(m + 1, k) * B[k] for k in range(m)) / (m + 1))
    return B


_BERN = _bernoulli(60)
_PI4_15 = math.pi ** 4 / 15


def _planck_tail(x):
    """integral from x to infinity of t^3/(e^t - 1) dt: exponential series for x >= 1, pi^4/15 minus the Bernoulli
    series of the head for x < 1 (both converge to full double precision)."""
    if x >= 1:
        s, n = 0.0, 1
        while True:
            term = math.exp(-n * x) * (x ** 3 / n + 3 * x * x / n ** 2 + 6 * x / n ** 3 + 6 / n ** 4)
            s += term
            if term < 1e-18 * s or n > 2000:
                return s
            n += 1
    head = 0.0
    for k, b in enumerate(_BERN):
        if b:
            head += float(b) * x ** (k + 3) / (math.factorial(k) * (k + 3))
    return _PI4_15 - head


def band_fraction(lam1, lam2, T):
    if not (0 < lam1 < lam2) or T <= 0:
        raise ValueError("need 0 < lambda1 < lambda2 and T > 0")
    x1, x2 = H * C / (lam1 * KB * T), H * C / (lam2 * KB * T)
    t1, t2 = _planck_tail(x1), _planck_tail(x2)
    F = (t2 - t1) / _PI4_15
    return _out(F_band=F, F_below_lam1=t1 / _PI4_15, F_below_lam2=t2 / _PI4_15, M_band=F * SIGMA_SB * T ** 4)


def ref_band_fraction(lam1, lam2, T):
    from scipy import constants as sc
    x1, x2 = sc.h * sc.c / (lam1 * sc.k * T), sc.h * sc.c / (lam2 * sc.k * T)
    tot = _ref_planck_integral()
    band = _ref_planck_integral(x2, x1)
    return {"F_band": band / tot, "F_below_lam1": _ref_planck_integral(x1) / tot, "F_below_lam2": _ref_planck_integral(x2) / tot}


_add("blackbody_band_fraction",
     name="Fraction of blackbody radiation in a wavelength band", topic="statistical physics / blackbody radiation",
     purpose="The fraction of a blackbody's emitted power between two wavelengths at temperature T, and that band's radiant exitance.",
     inputs=[_phys("lam1", "length", "shorter wavelength"), _phys("lam2", "length", "longer wavelength"), _phys("T", "temperature", "temperature")],
     outputs=[_o("F_band", "number", "fraction of the total emission between lambda1 and lambda2"), _o("F_below_lam1", "number", "fraction emitted below lambda1"),
              _o("F_below_lam2", "number", "fraction emitted below lambda2"), _o("M_band", "intensity", "radiant exitance in the band (blackbody)")],
     primary="F_band", conditions=["0 < lambda1 < lambda2", "T > 0"],
     method="F(0 -> lambda) = (15/pi^4) integral_x^inf t^3/(e^t - 1) dt, x = hc/(lambda k T), by exponential or Bernoulli series",
     latex=r"F_{0\to\lambda} = \frac{15}{\pi^4}\int_{hc/\lambda k_BT}^{\infty}\frac{t^3}{e^t - 1}\,dt",
     run=band_fraction, verify=_recheck(band_fraction),
     parse=lambda q: _pairs(q, r"(?:blackbody )?band fraction|fraction of blackbody radiation", {"lam1": "lam1", "lambda1": "lam1", "lam2": "lam2", "lambda2": "lam2", "t": "T"},
                            units={"lam1", "lam2", "T"}),
     reference=ref_band_fraction,
     wrong=_safe(lambda lam1, lam2, T: (lambda o: {**o, "F_band": _r((lam2 - lam1) / lam2)})(band_fraction(lam1, lam2, T))),
     random_inputs=lambda rng: (lambda T, a, b: {"T": T, "lam1": 2.9e-3 / T * min(a, b), "lam2": 2.9e-3 / T * max(a, b) * 1.05})(_lu(rng, 10, 1e5), _lu(rng, 0.1, 20), _lu(rng, 0.1, 20)),
     templates=["Band fraction: lam1 = 400 nm, lam2 = 700 nm, T = 5778 K"],
     examples=[{"inputs": {"lam1": 1e-12, "lam2": 1.0, "T": 5778.0}, "outputs": {"F_below_lam1": 0.0}}],
     questions=[{"q": "Band fraction: lam1 = 700 nm, lam2 = 400 nm, T = 5778 K", "outcome": "INVALID_INPUT"},
                {"q": "Band fraction: lam1 = 400 nm, lam2 = 700 nm", "outcome": "NEEDS_INPUT"}])


ZETA3 = 1.2020569031595942853997                        # Apéry's constant zeta(3)


def photon_gas(T):
    if T <= 0:
        raise ValueError("the temperature must be positive")
    u = math.pi ** 2 / 15 * (KB * T) ** 4 / (HBAR * C) ** 3
    n = 2 * ZETA3 / math.pi ** 2 * (KB * T / (HBAR * C)) ** 3
    return _out(u=u, n=n, mean_E_eV=u / n / EV, mean_E_over_kT=u / (n * KB * T), pressure=u / 3)


def ref_photon_gas(T):
    import numpy as np
    from scipy import constants as sc
    s = sc.k * T / (sc.h * sc.c)
    u = 8 * np.pi * sc.k * T * s ** 3 * _ref_planck_integral()          # mode density 8 pi nu^2/c^3 times mean energy, integrated
    n = 8 * np.pi * s ** 3 * _ref_planck_integral(power=2)
    return {"u": u, "n": n, "mean_E_over_kT": u / (n * sc.k * T)}


_add("photon_gas",
     name="Energy and photon density of blackbody radiation", topic="statistical physics / photon gas",
     purpose="Energy density, photon number density, mean photon energy and radiation pressure of thermal radiation at temperature T.",
     inputs=[_phys("T", "temperature", "temperature")],
     outputs=[_o("u", "pressure", "energy density (J/m^3)"), _o("n", "number_density", "photon number density"), _o("mean_E_eV", "energy_ev", "mean photon energy"),
              _o("mean_E_over_kT", "number", "mean photon energy in units of kT (2.701...)"), _o("pressure", "pressure", "radiation pressure u/3")],
     primary="n", conditions=["T > 0"],
     method="u = (pi^2/15)(kT)^4/(hbar c)^3, n = (2 zeta(3)/pi^2)(kT/(hbar c))^3",
     latex=r"u = \frac{\pi^2 (k_BT)^4}{15(\hbar c)^3},\quad n = \frac{2\zeta(3)}{\pi^2}\left(\frac{k_BT}{\hbar c}\right)^3",
     run=photon_gas, verify=_recheck(photon_gas),
     parse=lambda q: _pairs(q, r"photon gas|blackbody radiation density", {"t": "T"}, units={"T"}),
     reference=ref_photon_gas,
     wrong=_safe(lambda T: (lambda o: {**o, "n": _r(o["u"] / (3 * KB * T)), "mean_E_over_kT": 3.0})(photon_gas(T))),
     random_inputs=lambda rng: {"T": _lu(rng, 1, 1e8)},
     templates=["Photon gas: T = 2.725 K"],
     examples=[{"inputs": {"T": 2.725}, "outputs": {"mean_E_over_kT": math.pi ** 4 / (30 * ZETA3)}}],
     questions=[{"q": "Photon gas: T = 2.725 K", "outputs": {"mean_E_over_kT": math.pi ** 4 / (30 * ZETA3)}},
                {"q": "Photon gas: T = 0 K", "outcome": "INVALID_INPUT"},
                {"q": "Photon gas:", "outcome": "NEEDS_INPUT"}])


def two_level(dE, T, g0, g1):
    if dE <= 0 or T <= 0 or g0 < 1 or g1 < 1:
        raise ValueError("the gap and temperature must be positive and the degeneracies at least 1")
    x = dE / (KB * T)
    e = math.exp(-x)
    Z = g0 + g1 * e
    p1 = g1 * e / Z
    p0 = g0 / Z
    return _out(Z=Z, p0=p0, p1=p1, ratio=g1 / g0 * e, U_eV=dE * p1 / EV, C_over_k=x * x * p0 * p1, S_over_k=math.log(Z) + x * p1)


def ref_two_level(dE, T, g0, g1):
    # thermodynamics from the partition function by symbolic differentiation (sympy): U = -d lnZ/d beta, C = beta^2 d2lnZ,
    # S = ln Z + beta U, p1 = -(1/beta) d lnZ/d eps
    import sympy as sp
    from scipy import constants as sc
    b, eps = sp.symbols("beta epsilon", positive=True)
    lnZ = sp.log(g0 + g1 * sp.exp(-b * eps))
    x = dE / (sc.k * T)
    sub = {b: sp.Float(x, 30), eps: 1}
    U = -sp.diff(lnZ, b)
    vals = {"U": U.subs(sub), "C": (b ** 2 * sp.diff(lnZ, b, 2)).subs(sub), "S": (lnZ + b * U).subs(sub),
            "p1": (-sp.diff(lnZ, eps) / b).subs(sub)}
    v = {k: float(sp.N(e, 25)) for k, e in vals.items()}
    return {"U_eV": v["U"] * dE / sc.e, "C_over_k": v["C"], "S_over_k": v["S"], "p1": v["p1"], "p0": 1 - v["p1"]}


_add("two_level_thermodynamics",
     name="Thermodynamics of a two-level system", topic="statistical physics / partition functions and Boltzmann populations",
     purpose="Partition function, Boltzmann populations, mean energy, heat capacity (Schottky) and entropy of a two-level system with gap dE and degeneracies g0, g1.",
     inputs=[_phys("dE", "energy", "energy gap (eV, J)"), _phys("T", "temperature", "temperature"), _intin("g0", "degeneracy of the lower level", 1), _intin("g1", "degeneracy of the upper level", 1)],
     outputs=[_o("Z", "number", "partition function g0 + g1 e^(-dE/kT)"), _o("p0", "number", "population of the lower level"), _o("p1", "number", "population of the upper level"),
              _o("ratio", "number", "population ratio N1/N0 (Boltzmann factor)"), _o("U_eV", "energy_ev", "mean energy above the lower level"),
              _o("C_over_k", "number", "heat capacity per system in units of k"), _o("S_over_k", "number", "entropy per system in units of k")],
     primary="ratio", conditions=["dE > 0", "T > 0", "g0, g1 >= 1"],
     method="Z = g0 + g1 e^(-x), x = dE/kT; p1 = g1 e^(-x)/Z; U = dE p1; C/k = x^2 p0 p1; S/k = ln Z + x p1",
     latex=r"\frac{N_1}{N_0} = \frac{g_1}{g_0}e^{-\Delta E/k_BT},\quad \frac{C}{k_B} = x^2 p_0 p_1",
     run=two_level, verify=_recheck(two_level),
     parse=lambda q: _pairs(q, r"two[- ]level (?:system|thermodynamics)|boltzmann (?:population )?ratio", {"de": "dE", "t": "T", "g0": "g0", "g1": "g1"}, units={"dE", "T"}),
     reference=ref_two_level,
     wrong=_safe(lambda dE, T, g0, g1: (lambda o: {**o, "p1": _r(g1 * math.exp(-dE / (KB * T)) / g0), "U_eV": _r(dE / EV * g1 * math.exp(-dE / (KB * T)) / g0),
                                                   "C_over_k": _r((dE / (KB * T)) ** 2 * math.exp(-dE / (KB * T)))})(two_level(dE, T, g0, g1))),
     random_inputs=lambda rng: (lambda T: {"T": T, "dE": KB * T * _lu(rng, 0.05, 15), "g0": rng.randint(1, 4), "g1": rng.randint(1, 4)})(_lu(rng, 1, 1e4)),
     templates=["Two-level system: dE = 0.025 eV, T = 300 K, g0 = 1, g1 = 1"],
     examples=[{"inputs": {"dE": KB * 300 * math.log(2), "T": 300.0, "g0": 1, "g1": 1}, "outputs": {"ratio": 0.5, "p1": 1 / 3}},
               {"inputs": {"dE": KB * 100, "T": 100.0, "g0": 1, "g1": 3}, "outputs": {"ratio": 3 * math.exp(-1)}}],
     questions=[{"q": "Two-level system: dE = 0.025 eV, T = 300 K, g0 = 1, g1 = 1", "outputs": {"ratio": math.exp(-0.025 * EV / (KB * 300))}},
                {"q": "Two-level system: dE = 0.025 eV, T = 300 K, g0 = 0, g1 = 1", "outcome": "INVALID_INPUT"},
                {"q": "Two-level system: dE = 0.025 eV, T = 300 K", "outcome": "NEEDS_INPUT"}])


def einstein_oscillator(theta_E, T):
    if theta_E <= 0 or T <= 0:
        raise ValueError("the Einstein temperature and the temperature must be positive")
    x = theta_E / T
    em = math.expm1(x)
    nbar = 1 / em
    c = x * x * math.exp(x) / (em * em) if x < 300 else x * x * math.exp(-x)
    return _out(n_mean=nbar, U_eV=KB * theta_E * (nbar + 0.5) / EV, C_over_k=c, C_molar=3 * RGAS * c,
                S_over_k=x * nbar - math.log(-math.expm1(-x)))


def ref_einstein_oscillator(theta_E, T):
    # Boltzmann-weighted sums over the oscillator levels (numpy): <n>, and C from the energy fluctuations Var(E)/(k T^2)
    import numpy as np
    from scipy import constants as sc
    x = theta_E / T
    n = np.arange(0, int(60 / x) + 50)
    w = np.exp(-n * x)
    Z = w.sum()
    m1 = (n * w).sum() / Z
    m2 = (n * n * w).sum() / Z
    c = x * x * (m2 - m1 * m1)
    return {"n_mean": float(m1), "C_over_k": float(c), "C_molar": float(3 * sc.R * c), "S_over_k": float(x * m1 + np.log(Z))}


_add("einstein_oscillator",
     name="Quantum oscillator thermodynamics (Einstein model)", topic="statistical physics / Einstein model of a solid",
     purpose="Mean quantum number, mean energy, heat capacity and entropy of a quantum harmonic oscillator, and the molar heat capacity of an Einstein solid.",
     inputs=[_phys("theta_E", "temperature", "Einstein temperature hbar omega/k"), _phys("T", "temperature", "temperature")],
     outputs=[_o("n_mean", "number", "mean number of quanta 1/(e^x - 1)"), _o("U_eV", "energy_ev", "mean energy including the zero-point energy"),
              _o("C_over_k", "number", "heat capacity per oscillator in units of k"), _o("C_molar", "molar_heat_capacity", "molar heat capacity of an Einstein solid 3R C/k"),
              _o("S_over_k", "number", "entropy per oscillator in units of k")],
     primary="C_molar", conditions=["theta_E > 0", "T > 0"],
     method="x = theta_E/T; n = 1/(e^x - 1); C/k = x^2 e^x/(e^x - 1)^2; C_molar = 3R C/k",
     latex=r"C_V = 3R\left(\frac{\Theta_E}{T}\right)^2\frac{e^{\Theta_E/T}}{\left(e^{\Theta_E/T} - 1\right)^2}",
     run=einstein_oscillator, verify=_recheck(einstein_oscillator),
     parse=lambda q: _pairs(q, r"einstein (?:oscillator|solid|model|heat capacity)", {"theta_e": "theta_E", "t": "T"}, units={"theta_E", "T"}),
     reference=ref_einstein_oscillator,
     wrong=_safe(lambda theta_E, T: (lambda o: {**o, "C_molar": _r(3 * RGAS * math.exp(-theta_E / T)), "C_over_k": _r(math.exp(-theta_E / T))})(einstein_oscillator(theta_E, T))),
     random_inputs=lambda rng: (lambda th: {"theta_E": th, "T": th / _lu(rng, 0.05, 20)})(_lu(rng, 10, 3000)),
     templates=["Einstein solid: theta_E = 240 K, T = 300 K"],
     examples=[{"inputs": {"theta_E": 300.0, "T": 300.0}, "outputs": {"C_over_k": math.e / (math.e - 1) ** 2, "n_mean": 1 / (math.e - 1)}}],
     questions=[{"q": "Einstein solid: theta_E = 300 K, T = 300 K", "outputs": {"C_over_k": math.e / (math.e - 1) ** 2}},
                {"q": "Einstein solid: theta_E = 300 K, T = -300 K", "outcome": "INVALID_INPUT"},
                {"q": "Einstein solid: theta_E = 300 K", "outcome": "NEEDS_INPUT"}])


def debye(T, theta_D):
    if T <= 0 or theta_D <= 0:
        raise ValueError("the temperature and the Debye temperature must be positive")
    import mpmath
    xd = theta_D / T
    if xd > 600:
        raise ValueError("T is below theta_D/600: use the T^3 law (the integral equals 4 pi^4/15 to double precision)")
    with mpmath.workdps(25):
        I = mpmath.quad(lambda x: x ** 4 * mpmath.exp(x) / mpmath.expm1(x) ** 2 if x != 0 else mpmath.mpf(0), [0, min(xd, 1), xd])
        I = float(I)
    c = 3 * (T / theta_D) ** 3 * I                       # C / (3 N k) = 3 (T/theta)^3 integral
    return _out(C_molar=3 * RGAS * c, C_over_3R=c, debye_integral=I)


def ref_debye(T, theta_D):
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    xd = theta_D / T
    f = lambda x: x ** 4 * np.exp(x) / np.expm1(x) ** 2 if x > 0 else 0.0   # noqa: E731
    I = quad(f, 0, xd, epsabs=0, epsrel=1e-13, limit=400)[0]
    return {"C_molar": 9 * sc.R * (T / theta_D) ** 3 * I, "debye_integral": I}


_add("debye_heat_capacity",
     name="Debye heat capacity of a solid", topic="statistical physics / phonons and the Debye model",
     purpose="The molar heat capacity of a crystalline solid in the Debye model at temperature T.",
     inputs=[_phys("T", "temperature", "temperature"), _phys("theta_D", "temperature", "Debye temperature")],
     outputs=[_o("C_molar", "molar_heat_capacity", "molar heat capacity C_V"), _o("C_over_3R", "number", "C_V / 3R (tends to 1 at high T)"),
              _o("debye_integral", "number", "integral_0^(theta/T) x^4 e^x/(e^x - 1)^2 dx")],
     primary="C_molar", conditions=["T > 0", "theta_D > 0", "theta_D/T <= 600"],
     method="C_V = 9 R (T/theta_D)^3 integral_0^(theta_D/T) x^4 e^x/(e^x - 1)^2 dx (25-digit quadrature)",
     latex=r"C_V = 9R\left(\frac{T}{\Theta_D}\right)^3\int_0^{\Theta_D/T}\frac{x^4 e^x}{(e^x - 1)^2}\,dx",
     run=debye, verify=_recheck(debye),
     parse=lambda q: _pairs(q, r"debye (?:heat capacity|model)", {"t": "T", "theta_d": "theta_D"}, units={"T", "theta_D"}),
     reference=ref_debye,
     wrong=_safe(lambda T, theta_D: (lambda o: {**o, "C_molar": _r(12 * math.pi ** 4 / 5 * RGAS * (T / theta_D) ** 3)})(debye(T, theta_D))),
     random_inputs=lambda rng: (lambda th: {"theta_D": th, "T": th / _lu(rng, 0.1, 6)})(_lu(rng, 50, 2000)),
     templates=["Debye heat capacity: T = 300 K, theta_D = 343 K"],
     examples=[{"inputs": {"T": 1.0, "theta_D": 500.0}, "outputs": {"C_molar": 12 * math.pi ** 4 / 5 * RGAS * (1 / 500) ** 3}}],
     questions=[{"q": "Debye heat capacity: T = 1 K, theta_D = 500 K", "outputs": {"C_molar": 12 * math.pi ** 4 / 5 * RGAS * (1 / 500) ** 3}},
                {"q": "Debye heat capacity: T = 300 K, theta_D = 0 K", "outcome": "INVALID_INPUT"},
                {"q": "Debye heat capacity: T = 300 K", "outcome": "NEEDS_INPUT"}])


def _fermi_from_n(n):
    kF = (3 * math.pi ** 2 * n) ** (1 / 3)
    EF = HBAR ** 2 * kF ** 2 / (2 * ME)
    return kF, EF


def fermi_energy(n):
    if n <= 0:
        raise ValueError("the electron density must be positive")
    kF, EF = _fermi_from_n(n)
    return _out(E_F_eV=EF / EV, E_F=EF, T_F=EF / KB, k_F=kF, v_F=HBAR * kF / ME)


def _ref_fermi_level(n):
    # the energy up to which the free-electron density of states g(E) = (1/2 pi^2)(2m/hbar^2)^(3/2) sqrt(E) holds n
    # electrons per volume: quadrature of g and brentq on the count
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    from scipy.optimize import brentq
    g0 = (2 * sc.m_e / sc.hbar ** 2) ** 1.5 / (2 * np.pi ** 2)
    count = lambda E: quad(lambda e: g0 * np.sqrt(e), 0, E, epsabs=0, epsrel=1e-13)[0]   # noqa: E731
    guess = sc.hbar ** 2 / (2 * sc.m_e) * (3 * np.pi ** 2 * n) ** (2 / 3)
    return brentq(lambda E: count(E) / n - 1, 0.25 * guess, 4 * guess, xtol=1e-300, rtol=8.9e-16)


def ref_fermi_energy(n):
    from scipy import constants as sc
    EF = _ref_fermi_level(n)
    return {"E_F_eV": EF / sc.e, "T_F": EF / sc.k, "v_F": (2 * EF / sc.m_e) ** 0.5}


_add("fermi_energy_free_electrons",
     name="Fermi energy of a free-electron gas", topic="statistical physics / free-electron model of metals",
     purpose="Fermi energy, temperature, wavenumber and speed of a free-electron gas with electron density n.",
     inputs=[_phys("n", "number_density", "conduction-electron density (per m^3)")],
     outputs=[_o("E_F_eV", "energy_ev", "Fermi energy"), _o("E_F", "energy", "Fermi energy in J"), _o("T_F", "temperature", "Fermi temperature E_F/k"),
              _o("k_F", "wavenumber", "Fermi wavenumber (3 pi^2 n)^(1/3)"), _o("v_F", "speed", "Fermi speed")],
     primary="E_F_eV", conditions=["n > 0", "free electrons with spin 1/2, T = 0"],
     method="k_F = (3 pi^2 n)^(1/3), E_F = hbar^2 k_F^2/(2 m_e)",
     latex=r"E_F = \frac{\hbar^2}{2m_e}\left(3\pi^2 n\right)^{2/3}",
     run=fermi_energy, verify=_recheck(fermi_energy),
     parse=lambda q: _pairs(q, r"fermi energy(?: of a free[- ]electron gas)?", {"n": "n"}, units={"n"}),
     reference=ref_fermi_energy,
     wrong=_safe(lambda n: (lambda o: {**o, "E_F_eV": _r(HBAR ** 2 * (3 * math.pi ** 2 * n) ** (2 / 3) / ME / EV)})(fermi_energy(n))),
     random_inputs=lambda rng: {"n": _lu(rng, 1e20, 1e30)},
     templates=["Fermi energy: n = 8.47e28 m^-3"],
     examples=[{"inputs": {"n": 8.47e28}, "outputs": {"E_F_eV": HBAR ** 2 * (3 * math.pi ** 2 * 8.47e28) ** (2 / 3) / (2 * ME) / EV}}],
     questions=[{"q": "Fermi energy: n = 8.47e28 m^-3", "outputs": {"E_F_eV": HBAR ** 2 * (3 * math.pi ** 2 * 8.47e28) ** (2 / 3) / (2 * ME) / EV}},
                {"q": "Fermi energy: n = -1 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Fermi energy:", "outcome": "NEEDS_INPUT"}])


def fermi_energy_metal(rho, M, z):
    if rho <= 0 or M <= 0 or z < 1:
        raise ValueError("density and molar mass must be positive and the valence at least 1")
    n = z * rho * NA / M
    kF, EF = _fermi_from_n(n)
    return _out(n=n, E_F_eV=EF / EV, T_F=EF / KB, v_F=HBAR * kF / ME)


def ref_fermi_energy_metal(rho, M, z):
    from scipy import constants as sc
    n = z * rho * sc.N_A / M
    EF = _ref_fermi_level(n)
    return {"n": n, "E_F_eV": EF / sc.e, "T_F": EF / sc.k}


_add("fermi_energy_metal",
     name="Fermi energy of a metal from its density", topic="statistical physics / free-electron model of metals",
     purpose="Conduction-electron density and Fermi energy of a metal from its mass density, molar mass and valence (free-electron model).",
     inputs=[_phys("rho", "density", "mass density of the metal"), _phys("M", "molar_mass", "molar mass (g/mol or kg/mol)"),
             _intin("z", "conduction electrons per atom (valence)", 1, 8)],
     outputs=[_o("n", "number_density", "conduction-electron density"), _o("E_F_eV", "energy_ev", "Fermi energy"),
              _o("T_F", "temperature", "Fermi temperature"), _o("v_F", "speed", "Fermi speed")],
     primary="E_F_eV", conditions=["rho, M > 0", "z >= 1"],
     method="n = z rho N_A / M, E_F = hbar^2 (3 pi^2 n)^(2/3)/(2 m_e)",
     latex=r"n = \frac{z\rho N_A}{M},\quad E_F = \frac{\hbar^2}{2m_e}\left(3\pi^2 n\right)^{2/3}",
     run=fermi_energy_metal, verify=_recheck(fermi_energy_metal),
     parse=lambda q: _pairs(q, r"fermi energy of a metal|metal fermi energy", {"rho": "rho", "m": "M", "z": "z"}, units={"rho", "M"}),
     reference=ref_fermi_energy_metal,
     wrong=_safe(lambda rho, M, z: (lambda o: {**o, "E_F_eV": _r(HBAR ** 2 * (3 * math.pi ** 2 * rho * NA / M) ** (2 / 3) / (2 * ME) / EV) if z > 1 else _r(o["E_F_eV"] * 2)})(
         fermi_energy_metal(rho, M, z))),
     random_inputs=lambda rng: {"rho": rng.uniform(500, 22000), "M": rng.uniform(0.006, 0.25), "z": rng.randint(1, 3)},
     templates=["Fermi energy of a metal: rho = 8960 kg/m^3, M = 63.546 g/mol, z = 1"],
     examples=[{"inputs": {"rho": 8960.0, "M": 0.063546, "z": 1}, "outputs": {"n": 8960 * NA / 0.063546}}],
     questions=[{"q": "Fermi energy of a metal: rho = 8960 kg/m^3, M = 63.546 g/mol, z = 1", "outputs": {"n": 8960 * NA / 0.063546}},
                {"q": "Fermi energy of a metal: rho = 8960 kg/m^3, M = 63.546 g/mol, z = 0", "outcome": "INVALID_INPUT"},
                {"q": "Fermi energy of a metal: rho = 8960 kg/m^3, M = 63.546 g/mol", "outcome": "NEEDS_INPUT"}])


def fermi_dirac(E, mu, T):
    if T <= 0:
        raise ValueError("the temperature must be positive")
    x = (E - mu) / (KB * T)
    if x > 0:
        e = math.exp(-x)
        f, g = e / (1 + e), 1 / (1 + e)
    else:
        e = math.exp(x)
        f, g = 1 / (1 + e), e / (1 + e)
    return _out(f=f, one_minus_f=g, x=x)


def ref_fermi_dirac(E, mu, T):
    # grand-canonical average over the two occupations of the state (n = 0, 1), weights exp(-n x)
    import numpy as np
    from scipy import constants as sc
    x = (E - mu) / (sc.k * T)
    n = np.array([0.0, 1.0])
    lw = -n * x
    w = np.exp(lw - lw.max())
    f = float((n * w).sum() / w.sum())
    return {"f": f, "one_minus_f": float(w[0] / w.sum())}


_add("fermi_dirac_occupation",
     name="Fermi-Dirac occupation of a state", topic="statistical physics / quantum statistics",
     purpose="The probability that a single-particle state of energy E is occupied by a fermion at temperature T and chemical potential mu.",
     inputs=[_phys("E", "energy", "energy of the state"), _phys("mu", "energy", "chemical potential (Fermi level)"), _phys("T", "temperature", "temperature")],
     outputs=[_o("f", "number", "occupation probability 1/(e^x + 1)"), _o("one_minus_f", "number", "probability the state is empty"), _o("x", "number", "(E - mu)/kT")],
     primary="f", conditions=["T > 0"],
     method="f = 1/(exp((E - mu)/kT) + 1), evaluated without overflow",
     latex=r"f(E) = \frac{1}{e^{(E - \mu)/k_BT} + 1}",
     run=fermi_dirac, verify=_recheck(fermi_dirac),
     parse=lambda q: _pairs(q, r"fermi[- ]dirac(?: occupation| distribution)?", {"e": "E", "mu": "mu", "t": "T"}, units={"E", "mu", "T"}),
     reference=ref_fermi_dirac,
     wrong=_safe(lambda E, mu, T: (lambda o: {**o, "f": _r(math.exp(-o["x"]) if o["x"] > -700 else 1e300)})(fermi_dirac(E, mu, T))),
     templates=["Fermi-Dirac occupation: E = 5.1 eV, mu = 5.0 eV, T = 300 K"],
     examples=[{"inputs": {"E": 5 * EV, "mu": 5 * EV, "T": 300.0}, "outputs": {"f": 0.5}},
               {"inputs": {"E": 5 * EV + KB * 300 * math.log(3), "mu": 5 * EV, "T": 300.0}, "outputs": {"f": 0.25}}],
     questions=[{"q": "Fermi-Dirac occupation: E = 5 eV, mu = 5 eV, T = 300 K", "outputs": {"f": 0.5}},
                {"q": "Fermi-Dirac occupation: E = 5 eV, mu = 5 eV, T = 0 K", "outcome": "INVALID_INPUT"},
                {"q": "Fermi-Dirac occupation: E = 5 eV, mu = 5 eV", "outcome": "NEEDS_INPUT"}])


def _rand_fd(rng):
    T = _lu(rng, 1, 1e4)
    mu = rng.uniform(0, 10) * EV
    return {"T": T, "mu": mu, "E": mu + rng.uniform(-15, 15) * KB * T}


PROCEDURES["fermi_dirac_occupation"]["random_inputs"] = _rand_fd


def bose_einstein(E, mu, T):
    if T <= 0:
        raise ValueError("the temperature must be positive")
    x = (E - mu) / (KB * T)
    if x <= 0:
        raise ValueError("a Bose-Einstein occupation needs E > mu")
    n = 1 / math.expm1(x) if x < 700 else math.exp(-x)
    return _out(n=n, x=x, classical_limit=math.exp(-x))


def ref_bose_einstein(E, mu, T):
    # grand-canonical average over the occupations n = 0, 1, 2, ... of the mode (numpy sum, weights exp(-n x))
    import numpy as np
    from scipy import constants as sc
    x = (E - mu) / (sc.k * T)
    n = np.arange(0, int(60 / x) + 60, dtype=float)
    w = np.exp(-n * x)
    return {"n": float((n * w).sum() / w.sum())}


_add("bose_einstein_occupation",
     name="Bose-Einstein occupation of a mode", topic="statistical physics / quantum statistics",
     purpose="The mean number of bosons in a single-particle state of energy E at temperature T and chemical potential mu (mu < E).",
     inputs=[_phys("E", "energy", "energy of the state"), _phys("mu", "energy", "chemical potential (0 for photons and phonons)"), _phys("T", "temperature", "temperature")],
     outputs=[_o("n", "number", "mean occupation 1/(e^x - 1)"), _o("x", "number", "(E - mu)/kT"), _o("classical_limit", "number", "Maxwell-Boltzmann value e^(-x)")],
     primary="n", conditions=["T > 0", "E > mu"],
     method="n = 1/(exp((E - mu)/kT) - 1) (expm1 for small x)",
     latex=r"\bar n(E) = \frac{1}{e^{(E - \mu)/k_BT} - 1}",
     run=bose_einstein, verify=_recheck(bose_einstein),
     parse=lambda q: _pairs(q, r"bose[- ]einstein(?: occupation| distribution)?", {"e": "E", "mu": "mu", "t": "T"}, units={"E", "mu", "T"}),
     reference=ref_bose_einstein,
     wrong=_safe(lambda E, mu, T: (lambda o: {**o, "n": _r(1 / (math.exp(o["x"]) + 1))})(bose_einstein(E, mu, T))),
     random_inputs=lambda rng: (lambda T: {"T": T, "mu": 0.0, "E": KB * T * _lu(rng, 0.01, 20)})(_lu(rng, 1, 1e4)),
     templates=["Bose-Einstein occupation: E = 0.01 eV, mu = 0 eV, T = 300 K"],
     examples=[{"inputs": {"E": KB * 300 * math.log(2), "mu": 0.0, "T": 300.0}, "outputs": {"n": 1.0}},
               {"inputs": {"E": KB * 300 * math.log(3), "mu": 0.0, "T": 300.0}, "outputs": {"n": 0.5}}],
     questions=[{"q": "Bose-Einstein occupation: E = 0.01 eV, mu = 0.02 eV, T = 300 K", "outcome": "INVALID_INPUT"},
                {"q": "Bose-Einstein occupation: E = 0.01 eV, T = 300 K", "outcome": "NEEDS_INPUT"}])


def _fd_half(eta):
    """I(eta) = integral_0^inf sqrt(x)/(e^(x - eta) + 1) dx and dI/deta, with x = t^2 and the trapezoidal rule on the
    even, analytic integrand 2 t^2/(e^(t^2 - eta) + 1) (geometric convergence; the step resolves the Fermi edge)."""
    h = 0.2 / math.sqrt(max(eta, 1.0))
    tmax = math.sqrt(max(eta, 0.0) + 46.0)
    I = dI = 0.0
    k = 1
    while k * h <= tmax:
        t = k * h
        a = t * t - eta
        if a > 0:
            e = math.exp(-a)
            f, df = e / (1 + e), e / (1 + e) ** 2
        else:
            e = math.exp(a)
            f, df = 1 / (1 + e), e / (1 + e) ** 2
        I += 2 * t * t * f
        dI += 2 * t * t * df
        k += 1
    return I * h, dI * h


def fermi_chemical_potential(n, T, m):
    if n <= 0 or T <= 0 or m <= 0:
        raise ValueError("density, temperature and mass must be positive")
    lam = H / math.sqrt(2 * math.pi * m * KB * T)
    y = n * lam ** 3 / 2                                 # = f_{3/2}(z), spin 1/2
    EF = HBAR ** 2 * (3 * math.pi ** 2 * n) ** (2 / 3) / (2 * m)
    if KB * T < 0.02 * EF:
        raise ValueError("T is below 0.02 T_F: the gas is fully degenerate (mu = E_F to within the Sommerfeld correction); give a higher temperature")
    Y = y * math.sqrt(math.pi) / 2                       # target for I(eta) = Gamma(3/2) f_{3/2}(z)
    lo, hi = -60.0, 60.0
    eta = math.log(y) if y < 1 else (3 * math.sqrt(math.pi) * y / 4) ** (2 / 3)
    eta = min(max(eta, lo + 1), hi - 1)
    for _ in range(200):
        I, dI = _fd_half(eta)
        fv = I - Y
        if fv > 0:
            hi = min(hi, eta)
        else:
            lo = max(lo, eta)
        new = eta - fv / dI if dI > 0 else (lo + hi) / 2
        if not (lo < new < hi):
            new = (lo + hi) / 2
        if abs(new - eta) <= 1e-15 * max(1.0, abs(new)) or hi - lo <= 1e-15 * max(1.0, abs(new)):
            eta = new
            break
        eta = new
    if not (-59.0 < eta < 59.0):
        raise ValueError("mu/kT lies outside -59 ... 59: the gas is either classical (use the Boltzmann limit) or fully degenerate")
    mu = eta * KB * T
    return _out(mu_eV=mu / EV, eta=eta, E_F_eV=EF / EV, mu_over_EF=mu / EF, T_over_TF=KB * T / EF)


def ref_fermi_chemical_potential(n, T, m):
    # mu such that the Fermi-Dirac-filled free-particle density of states holds n per volume (scipy quad + brentq)
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    from scipy.optimize import brentq
    kT = sc.k * T
    pref = (2 * m * kT / sc.hbar ** 2) ** 1.5 / (2 * np.pi ** 2)

    def dens(eta):
        f = lambda x: np.sqrt(x) / (np.exp(x - eta) + 1) if x - eta < 700 else 0.0   # noqa: E731
        up = max(eta, 0.0) + 80
        pts = [eta] if 0 < eta < up else None
        return pref * quad(f, 0, up, epsabs=0, epsrel=1e-13, limit=400, points=pts)[0]
    eta = brentq(lambda e: dens(e) / n - 1, -60, 60, xtol=1e-14, rtol=8.9e-16)
    return {"eta": eta, "mu_eV": eta * kT / sc.e}


_add("fermi_gas_chemical_potential",
     name="Chemical potential of an ideal Fermi gas", topic="statistical physics / quantum statistics of gases",
     purpose="The chemical potential of an ideal spin-1/2 Fermi gas of density n and particle mass m at temperature T, from degenerate to classical.",
     inputs=[_phys("n", "number_density", "particle density"), _phys("T", "temperature", "temperature"), _phys("m", "mass", "particle mass")],
     outputs=[_o("mu_eV", "energy_ev", "chemical potential"), _o("eta", "number", "mu/kT"), _o("E_F_eV", "energy_ev", "Fermi energy at T = 0"),
              _o("mu_over_EF", "number", "mu/E_F"), _o("T_over_TF", "number", "T/T_F")],
     primary="eta", conditions=["n, T, m > 0", "T >= 0.02 T_F", "|mu/kT| <= 60"],
     method="n lambda_th^3 / 2 = f_{3/2}(e^(mu/kT)) solved by safeguarded Newton; the Fermi-Dirac integral by the trapezoidal rule in x = t^2",
     latex=r"n = \frac{2}{\lambda_{th}^3}\,f_{3/2}\!\left(e^{\mu/k_BT}\right),\quad f_{3/2}(z) = -\mathrm{Li}_{3/2}(-z)",
     run=fermi_chemical_potential, verify=_recheck(fermi_chemical_potential),
     parse=lambda q: _pairs(q, r"fermi gas chemical potential|chemical potential of an ideal fermi gas", {"n": "n", "t": "T", "m": "m"}, units={"n", "T", "m"}),
     reference=ref_fermi_chemical_potential,
     wrong=_safe(lambda n, T, m: (lambda o: {**o, "eta": _r(o["E_F_eV"] * EV / (KB * T)), "mu_eV": o["E_F_eV"]})(fermi_chemical_potential(n, T, m))),
     random_inputs=lambda rng: (lambda n, m: (lambda EF: {"n": n, "m": m, "T": EF / KB * _lu(rng, 0.05, 3)})(HBAR ** 2 * (3 * math.pi ** 2 * n) ** (2 / 3) / (2 * m)))(
         _lu(rng, 1e18, 1e29), rng.choice([ME, 6.64e-27, 1.0e-25])),
     templates=["Fermi gas chemical potential: n = 1e28 m^-3, T = 50000 K, m = 9.109e-31 kg"],
     examples=[{"inputs": {"n": 1e28, "T": 1e5, "m": ME}, "outputs": {"E_F_eV": HBAR ** 2 * (3 * math.pi ** 2 * 1e28) ** (2 / 3) / (2 * ME) / EV}}],
     questions=[{"q": "Fermi gas chemical potential: n = 1e28 m^-3, T = 1 K, m = 9.1093837139e-31 kg", "outcome": "INVALID_INPUT"},
                {"q": "Fermi gas chemical potential: n = 1e28 m^-3, T = 50000 K", "outcome": "NEEDS_INPUT"}])

ZETA32 = 2.6123753486854883433                          # zeta(3/2)


def bec_temperature(n, m):
    if n <= 0 or m <= 0:
        raise ValueError("the density and the mass must be positive")
    Tc = 2 * math.pi * HBAR ** 2 / (m * KB) * (n / ZETA32) ** (2 / 3)
    return _out(T_c=Tc, lambda_c=(ZETA32 / n) ** (1 / 3), n_lambda3=ZETA32)


def ref_bec_temperature(n, m):
    # the temperature at which the excited states (mu = 0) can just hold all n particles per volume: quadrature of the
    # Bose-Einstein-filled density of states (x = u^2 removes the 1/sqrt(x) singularity) and brentq
    import numpy as np
    from scipy import constants as sc
    from scipy.integrate import quad
    from scipy.optimize import brentq
    I = quad(lambda u: 2 * u * u * np.exp(-u * u) / -np.expm1(-u * u) if u > 0 else 0.0, 0, np.inf, epsabs=0, epsrel=1e-13, limit=400)[0]   # = integral sqrt(x)/(e^x - 1)

    def excited(T):
        return (2 * m * sc.k * T / sc.hbar ** 2) ** 1.5 / (4 * np.pi ** 2) * I   # spin 0: g(E) = (2m/hbar^2)^(3/2) sqrt(E)/(4 pi^2)
    guess = 2 * np.pi * sc.hbar ** 2 / (m * sc.k) * (n / 2.6) ** (2 / 3)
    Tc = brentq(lambda T: excited(T) / n - 1, 0.2 * guess, 5 * guess, xtol=1e-300, rtol=8.9e-16)
    return {"T_c": Tc}


_add("bec_critical_temperature",
     name="Bose-Einstein condensation temperature", topic="statistical physics / Bose-Einstein condensation",
     purpose="The critical temperature below which an ideal Bose gas of density n and particle mass m forms a condensate.",
     inputs=[_phys("n", "number_density", "particle density"), _phys("m", "mass", "particle mass")],
     outputs=[_o("T_c", "temperature", "critical temperature"), _o("lambda_c", "length", "thermal wavelength at T_c"), _o("n_lambda3", "number", "phase-space density at T_c, zeta(3/2)")],
     primary="T_c", conditions=["n > 0", "m > 0", "ideal gas, spin 0, uniform box"],
     method="T_c = (2 pi hbar^2/(m k)) (n/zeta(3/2))^(2/3)",
     latex=r"T_c = \frac{2\pi\hbar^2}{m k_B}\left(\frac{n}{\zeta(3/2)}\right)^{2/3}",
     run=bec_temperature, verify=_recheck(bec_temperature),
     parse=lambda q: _pairs(q, r"(?:bose[- ]einstein )?condensation temperature|bec (?:critical )?temperature", {"n": "n", "m": "m"}, units={"n", "m"}),
     reference=ref_bec_temperature,
     wrong=_safe(lambda n, m: (lambda o: {**o, "T_c": _r(2 * math.pi * HBAR ** 2 / (m * KB) * n ** (2 / 3))})(bec_temperature(n, m))),
     random_inputs=lambda rng: {"n": _lu(rng, 1e17, 1e28), "m": _lu(rng, 1e-27, 3e-25)},
     templates=["Condensation temperature: n = 1e20 m^-3, m = 1.443e-25 kg"],
     examples=[{"inputs": {"n": 1e20, "m": 1.443e-25}, "outputs": {"T_c": 2 * math.pi * HBAR ** 2 / (1.443e-25 * KB) * (1e20 / ZETA32) ** (2 / 3)}}],
     questions=[{"q": "Condensation temperature: n = 1e20 m^-3, m = 1.443e-25 kg", "outputs": {"T_c": 2 * math.pi * HBAR ** 2 / (1.443e-25 * KB) * (1e20 / ZETA32) ** (2 / 3)}},
                {"q": "Condensation temperature: n = 0 m^-3, m = 1.443e-25 kg", "outcome": "INVALID_INPUT"},
                {"q": "Condensation temperature: n = 1e20 m^-3", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented, not a reason to
# leave it out). Photon energy, rest energy, de Broglie wavelength from m and v, the step potential and the
# semi-empirical mass formula.

def photon_energy(lam):
    if not (lam > 0 and math.isfinite(lam)):
        raise ValueError("the wavelength must be positive")
    E = H * C / lam
    return _out(E=E, E_eV=E / EV, f=C / lam, p=H / lam)


def ref_photon_energy(lam):
    # the level spacing of the field mode's oscillator Hamiltonian (qutip), frequency from scipy.constants.lambda2nu
    from scipy import constants as sc
    E = _ref_photon_energy(lam)
    return {"E_eV": E / sc.e, "f": float(sc.lambda2nu(lam)), "p": E / sc.c}


_add("photon_energy",
     name="Photon energy from wavelength", topic="photons and matter waves / photon energy",
     purpose="The energy E = h c/lambda, frequency and momentum h/lambda of a photon of vacuum wavelength lambda.",
     inputs=[_phys("lam", "length", "vacuum wavelength (nm, m ...)")],
     outputs=[_o("E", "energy", "photon energy in J"), _o("E_eV", "energy_ev", "photon energy in eV"),
              _o("f", "frequency", "frequency c/lambda"), _o("p", "momentum", "photon momentum h/lambda")],
     primary="E_eV", conditions=["lambda > 0", "in vacuum"],
     method="E = h c/lambda, f = c/lambda, p = h/lambda",
     latex=r"E = h f = \frac{hc}{\lambda},\quad p = \frac{h}{\lambda}",
     run=photon_energy, verify=_recheck(photon_energy),
     parse=lambda q: _pairs(q, r"photon energy(?: from wavelength)?", {"lambda": "lam", "lam": "lam"}, units={"lam"}),
     reference=ref_photon_energy,
     wrong=_safe(lambda lam: (lambda E: _out(E=E, E_eV=E / EV, f=C / lam, p=HBAR / lam))(HBAR * C / lam)),
     random_inputs=lambda rng: {"lam": _lu(rng, 1e-12, 1e-2)},
     templates=["Photon energy: lambda = 500 nm"],
     examples=[{"inputs": {"lam": 500e-9}, "outputs": {"E_eV": H * C / 500e-9 / EV, "f": C / 500e-9}},
               {"inputs": {"lam": H * C / (2.0 * EV)}, "outputs": {"E_eV": 2.0}}],
     questions=[{"q": "Photon energy: lambda = 500 nm", "outputs": {"E_eV": H * C / 500e-9 / EV}},
                {"q": "Photon energy: lambda = 0.1 m", "outputs": {"f": C / 0.1}},
                {"q": "Photon energy: lambda = -500 nm", "outcome": "INVALID_INPUT"},
                {"q": "Photon energy:", "outcome": "NEEDS_INPUT"}])


def rest_energy(m):
    if not (m > 0 and math.isfinite(m)):
        raise ValueError("the mass must be positive")
    E = m * C * C
    return _out(E=E, E_MeV=E / MEV, E_kWh=E / 3.6e6)


def ref_rest_energy(m):
    # astropy's mass-energy equivalency (units.mass_energy), a library's own conversion
    import astropy.units as u
    return {"E_MeV": float((m * u.kg).to(u.MeV, equivalencies=u.mass_energy()).value),
            "E_kWh": float((m * u.kg).to(u.kW * u.h, equivalencies=u.mass_energy()).value)}


_add("rest_energy",
     name="Rest energy (mass-energy equivalence)", topic="special relativity / rest energy",
     purpose="The rest energy E = m c^2 of a mass m, in J, MeV and kWh.",
     inputs=[_phys("m", "mass", "rest mass (kg, g ...)")],
     outputs=[_o("E", "energy", "rest energy in J"), _o("E_MeV", "energy_mev", "rest energy in MeV"),
              _o("E_kWh", "number", "rest energy in kWh")],
     primary="E_MeV", conditions=["m > 0"],
     method="E = m c^2 (c = 299792458 m/s exactly)",
     latex=r"E_0 = m c^2",
     run=rest_energy, verify=_recheck(rest_energy),
     parse=lambda q: _pairs(q, r"rest energy|mass[- ]energy equivalence", {"m": "m"}, units={"m"}),
     reference=ref_rest_energy,
     wrong=_safe(lambda m: (lambda E: _out(E=E, E_MeV=E / MEV, E_kWh=E / 3.6e6))(0.5 * m * C * C)),
     random_inputs=lambda rng: {"m": _lu(rng, 1e-31, 1e3)},
     templates=["Rest energy: m = 9.1093837139e-31 kg", "Rest energy: m = 1 g"],
     examples=[{"inputs": {"m": ME}, "outputs": {"E_MeV": ME * C * C / MEV}},
               {"inputs": {"m": 1.0}, "outputs": {"E": C * C, "E_kWh": C * C / 3.6e6}}],
     questions=[{"q": "Rest energy: m = 9.1093837139e-31 kg", "outputs": {"E_MeV": ME * C * C / MEV}},
                {"q": "Rest energy: m = 1 g", "outputs": {"E": 1e-3 * C * C}},
                {"q": "Rest energy: m = 0 kg", "outcome": "INVALID_INPUT"},
                {"q": "Rest energy:", "outcome": "NEEDS_INPUT"}])


def de_broglie_mv(m, v):
    if not (m > 0 and math.isfinite(m)):
        raise ValueError("the mass must be positive")
    if not (v > 0):
        raise ValueError("the speed must be positive")
    b = _beta(v)
    p = m * v
    pr = p * _gamma_of_beta(b)
    return _out(p=p, lam=H / p, lambda_nm=H / p * 1e9, lambda_rel_nm=H / pr * 1e9, inv_lambda=p / H, beta=b)


def ref_de_broglie_mv(m, v):
    # non-relativistic lambda = h/(m v) from scipy.constants; relativistic momentum m c sinh(rapidity) from the
    # expm Lorentz boost of the rest four-momentum
    import numpy as np
    from scipy import constants as sc
    L = _ref_boost(math.atanh(v / C))
    E, pc = L @ np.array([m * sc.c ** 2, 0.0])
    return {"lambda_nm": sc.h / (m * v) * 1e9, "lambda_rel_nm": sc.h * sc.c / pc * 1e9, "inv_lambda": m * v / sc.h}


_add("de_broglie_wavelength_mv",
     name="de Broglie wavelength of a moving particle", topic="photons and matter waves / de Broglie wavelength",
     purpose="The de Broglie wavelength lambda = h/(m v) of a particle of mass m moving at speed v, and the relativistic h/(gamma m v).",
     inputs=[_phys("m", "mass", "mass of the particle"), _phys("v", "speed", "speed (may be written as a fraction of c)")],
     outputs=[_o("p", "momentum", "momentum m v"), _o("lam", "length", "de Broglie wavelength h/(m v)"),
              _o("lambda_nm", "length_nm", "the same in nm"), _o("lambda_rel_nm", "length_nm", "relativistic wavelength h/(gamma m v) in nm"),
              _o("inv_lambda", "number", "1/lambda = m v/h in 1/m (well scaled for heavy bodies, whose lambda is tiny)"),
              _o("beta", "number", "v/c")],
     primary="lambda_nm", conditions=["m > 0", "0 < v < c"],
     method="lambda = h/(m v) (non-relativistic momentum); lambda_rel = h/(gamma m v)",
     latex=r"\lambda = \frac{h}{p} = \frac{h}{m v}",
     run=de_broglie_mv, verify=_recheck(de_broglie_mv),
     parse=lambda q: _pairs(q, r"de broglie wavelength of a moving particle|de broglie wavelength from mass and speed",
                            {"m": "m", "v": "v"}, units={"m", "v"}, speeds={"v"}),
     reference=ref_de_broglie_mv,
     wrong=_safe(lambda m, v: (lambda o: {**o, "lam": _r(HBAR / (m * v)), "lambda_nm": _r(HBAR / (m * v) * 1e9),
                                          "lambda_rel_nm": _r(HBAR / (m * v * _gamma_of_beta(v / C)) * 1e9),
                                          "inv_lambda": _r(m * v / HBAR)})(de_broglie_mv(m, v))),
     random_inputs=lambda rng: {"m": _lu(rng, 1e-31, 1e-3), "v": _lu(rng, 1e-2, 0.9 * C)},
     templates=["de Broglie wavelength of a moving particle: m = 9.109e-31 kg, v = 2.0e6 m/s"],
     examples=[{"inputs": {"m": ME, "v": 1e6}, "outputs": {"lambda_nm": H / (ME * 1e6) * 1e9}},
               {"inputs": {"m": 0.145, "v": 40.0}, "outputs": {"lam": H / (0.145 * 40.0)}},
               {"inputs": {"m": ME, "v": 0.6 * C}, "outputs": {"lambda_rel_nm": H / (1.25 * ME * 0.6 * C) * 1e9}}],
     questions=[{"q": "de Broglie wavelength of a moving particle: m = 9.1093837139e-31 kg, v = 1e6 m/s", "outputs": {"lambda_nm": H / (ME * 1e6) * 1e9}},
                {"q": "de Broglie wavelength of a moving particle: m = 0.145 kg, v = 40 m/s", "outputs": {"lam": H / (0.145 * 40.0)}},
                {"q": "de Broglie wavelength of a moving particle: m = 9.1e-31 kg, v = 1.2 c", "outcome": "INVALID_INPUT"},
                {"q": "de Broglie wavelength of a moving particle: m = 9.1e-31 kg", "outcome": "NEEDS_INPUT"}])


def step_potential(E, V0):
    if not (E > 0 and V0 > 0):
        raise ValueError("the energy and the step height must be positive")
    if E <= V0:
        return _out(R=1.0, T=0.0, k2_over_k1=0.0, regime="total reflection (E <= V0)")
    r = math.sqrt((E - V0) / E)
    return _out(R=((1 - r) / (1 + r)) ** 2, T=4 * r / (1 + r) ** 2, k2_over_k1=r, regime="partial transmission (E > V0)")


def ref_step_potential(E, V0):
    # the Schroedinger equation integrated numerically (scipy DOP853) from a pure transmitted wave on the step back to
    # x = 0 and on through the free region to x = -1; incident and reflected amplitudes are read from psi and psi'
    # there, and T is the ratio of probability currents
    import numpy as np
    from scipy.integrate import solve_ivp
    v = V0 / E                                           # units: hbar^2/(2m) = 1, E = 1, so k1 = 1
    if v < 1:
        k2 = math.sqrt(1 - v)
        p1 = np.exp(1j * k2)
        d1 = 1j * k2 * p1
    else:
        kap = math.sqrt(v - 1)
        p1 = np.exp(-kap) + 0j
        d1 = -kap * p1

    def rhs(c):
        return lambda x, y: [y[2], y[3], c * y[0], c * y[1]]
    o = dict(method="DOP853", rtol=1e-13, atol=1e-16)
    s2 = solve_ivp(rhs(v - 1), (1.0, 0.0), [p1.real, p1.imag, d1.real, d1.imag], **o).y[:, -1]
    s1 = solve_ivp(rhs(-1.0), (0.0, -1.0), list(s2), **o).y[:, -1]
    psi, dpsi = s1[0] + 1j * s1[1], s1[2] + 1j * s1[3]
    A = 0.5 * (psi + dpsi / 1j) * np.exp(1j)            # psi = A e^{ix} + B e^{-ix} at x = -1
    B = 0.5 * (psi - dpsi / 1j) * np.exp(-1j)
    jt = float(np.imag(np.conj(p1) * d1))               # transmitted current (0 for the decaying wave)
    return {"R": float(abs(B) ** 2 / abs(A) ** 2), "T": jt / float(abs(A) ** 2)}


def _rand_step(rng):
    V0 = rng.uniform(0.5, 50) * EV
    return {"V0": V0, "E": V0 * (rng.uniform(1.02, 25.0) if rng.random() < 0.8 else rng.uniform(0.05, 0.98))}


_add("step_potential",
     name="Reflection and transmission at a potential step", topic="quantum wells / potential step",
     purpose="Reflection and transmission probabilities of a particle of energy E meeting a potential step of height V0 (V = 0 for x < 0, V0 for x > 0).",
     inputs=[_phys("E", "energy", "kinetic energy of the incident particle (on the low side)"), _phys("V0", "energy", "height of the step")],
     outputs=[_o("R", "number", "reflection probability"), _o("T", "number", "transmission probability (ratio of probability currents)"),
              _o("k2_over_k1", "number", "ratio of wavenumbers sqrt((E - V0)/E) (0 when E <= V0)"),
              {"symbol": "regime", "kind": "number", "meaning": "partial transmission (E > V0) or total reflection"}],
     primary="T", conditions=["E > 0", "V0 > 0", "the particle comes from x < 0"],
     method="R = ((k1 - k2)/(k1 + k2))^2, T = 4 k1 k2/(k1 + k2)^2 for E > V0; R = 1, T = 0 for E <= V0",
     latex=r"R = \left(\frac{k_1 - k_2}{k_1 + k_2}\right)^2,\quad T = \frac{4k_1k_2}{(k_1 + k_2)^2},\quad \frac{k_2}{k_1} = \sqrt{\frac{E - V_0}{E}}",
     run=step_potential, verify=_recheck(step_potential),
     parse=lambda q: _pairs(q, r"(?:potential )?step(?: potential)? (?:reflection|transmission)(?: and (?:reflection|transmission))?|step potential",
                            {"e": "E", "v0": "V0"}, units={"E", "V0"}),
     reference=ref_step_potential,
     wrong=_safe(lambda E, V0: (lambda o: {**o, "T": _r(4 / (1 + o["k2_over_k1"]) ** 2) if E > V0 else 0.0})(step_potential(E, V0))),
     random_inputs=_rand_step,
     templates=["Step potential: E = 5 eV, V0 = 3 eV"],
     examples=[{"inputs": {"E": 4 * EV, "V0": 3 * EV}, "outputs": {"R": 1 / 9, "T": 8 / 9, "k2_over_k1": 0.5}},
               {"inputs": {"E": 1 * EV, "V0": 3 * EV}, "outputs": {"R": 1.0, "T": 0.0}}],
     questions=[{"q": "Step potential: E = 4 eV, V0 = 3 eV", "outputs": {"T": 8 / 9, "R": 1 / 9}},
                {"q": "Step potential: E = 1 eV, V0 = 3 eV", "outputs": {"T": 0.0}},
                {"q": "Step potential: E = -1 eV, V0 = 3 eV", "outcome": "INVALID_INPUT"},
                {"q": "Step potential: E = 4 eV", "outcome": "NEEDS_INPUT"}])

# liquid-drop coefficients (MeV): a_V = 15.75, a_S = 17.8, a_C = 0.711, a_A = 23.7, a_P = 11.18 (Wapstra's set as
# quoted in Krane); Coulomb term a_C Z(Z - 1)/A^(1/3); pairing +a_P/sqrt(A) even-even, 0 odd A, -a_P/sqrt(A) odd-odd.
# Atomic mass M = Z m(1H) + N m_n - B/c^2 with m(1H) = 1.00782503223 u, m_n = 1.00866491606 u (AME2020).
SEMF = {"aV": 15.75, "aS": 17.8, "aC": 0.711, "aA": 23.7, "aP": 11.18}
M_H1_U, M_N_U = 1.00782503223, 1.00866491606


def _semf(Z, A, zz1=True):
    Z, A = int(Z), int(A)
    if A < 2 or Z < 1 or Z > A:
        raise ValueError("give 1 <= Z <= A and A >= 2")
    a = SEMF
    N = A - Z
    c3 = A ** (1 / 3)
    vol, surf = a["aV"] * A, a["aS"] * c3 * c3
    coul = a["aC"] * (Z * (Z - 1) if zz1 else Z * Z) / c3
    asym = a["aA"] * (A - 2 * Z) ** 2 / A
    pair = 0.0 if A % 2 else (a["aP"] / math.sqrt(A) if Z % 2 == 0 else -a["aP"] / math.sqrt(A))
    B = vol - surf - coul - asym + pair
    zs = (4 * a["aA"] + a["aC"] / c3) / (8 * a["aA"] / A + 2 * a["aC"] / c3)
    return _out(B_MeV=B, B_per_A_MeV=B / A, mass_u=Z * M_H1_U + N * M_N_U - B / AMU_MEV,
                volume_MeV=vol, surface_MeV=surf, coulomb_MeV=coul, asymmetry_MeV=asym, pairing_MeV=pair, Z_stable=zs)


def semf(Z, A):
    return _semf(Z, A)


def ref_semf(Z, A):
    # the terms in exact rational arithmetic (sympy, 30 digits); the most stable Z as the root of dB/dZ (symbolic
    # derivative, scipy brentq) instead of the closed form
    import sympy as sp
    from scipy.optimize import brentq
    q = {k: sp.Rational(str(v)) for k, v in SEMF.items()}
    z = sp.Symbol("z")
    Ai = sp.Integer(A)
    c3 = sp.root(Ai, 3)
    pair = 0 if A % 2 else (q["aP"] / sp.sqrt(Ai) if Z % 2 == 0 else -q["aP"] / sp.sqrt(Ai))
    Bz = q["aV"] * Ai - q["aS"] * c3 ** 2 - q["aC"] * z * (z - 1) / c3 - q["aA"] * (Ai - 2 * z) ** 2 / Ai
    B = sp.N(Bz.subs(z, Z) + pair, 30)
    d = sp.lambdify(z, sp.diff(Bz, z))
    zs = brentq(d, 0.0, A, xtol=1e-14, rtol=8.9e-16)
    mass = sp.N(Z * sp.Rational(str(M_H1_U)) + (A - Z) * sp.Rational(str(M_N_U)) - B / sp.Rational(str(AMU_MEV)), 30)
    return {"B_MeV": float(B), "B_per_A_MeV": float(B / A), "mass_u": float(mass), "Z_stable": zs}


def _rand_semf(rng):
    A = rng.randint(12, 260)
    zs = A / (1.98 + 0.0155 * A ** (2 / 3))
    return {"A": A, "Z": max(1, min(A, int(round(zs)) + rng.randint(-4, 4)))}


_add("semi_empirical_mass_formula",
     name="Semi-empirical mass formula (liquid-drop binding energy)", topic="nuclear and particle physics / binding energy",
     purpose="The liquid-drop (Bethe-Weizsaecker) binding energy of a nucleus with Z protons and mass number A, its terms, the binding energy per nucleon, the atomic mass and the most stable Z for that A.",
     inputs=[_intin("Z", "number of protons", 1, 130), _intin("A", "mass number", 2, 400)],
     outputs=[_o("B_MeV", "energy_mev", "binding energy"), _o("B_per_A_MeV", "energy_mev", "binding energy per nucleon"),
              _o("mass_u", "number", "atomic mass in u (Z m(1H) + N m_n - B/c^2)"),
              _o("volume_MeV", "energy_mev", "volume term a_V A"), _o("surface_MeV", "energy_mev", "surface term a_S A^(2/3)"),
              _o("coulomb_MeV", "energy_mev", "Coulomb term a_C Z(Z - 1)/A^(1/3)"), _o("asymmetry_MeV", "energy_mev", "asymmetry term a_A (A - 2Z)^2/A"),
              _o("pairing_MeV", "energy_mev", "pairing term (+ even-even, 0 odd A, - odd-odd)"),
              _o("Z_stable", "number", "most stable (continuous) Z for this A, where dB/dZ = 0")],
     primary="B_MeV", conditions=["1 <= Z <= A", "A >= 2", "a liquid-drop estimate: poor for light nuclei"],
     method="B = a_V A - a_S A^(2/3) - a_C Z(Z - 1)/A^(1/3) - a_A (A - 2Z)^2/A + delta; a_V = 15.75, a_S = 17.8, a_C = 0.711, a_A = 23.7, a_P = 11.18 MeV, delta = +-a_P/sqrt(A)",
     latex=r"B = a_V A - a_S A^{2/3} - a_C\frac{Z(Z-1)}{A^{1/3}} - a_A\frac{(A-2Z)^2}{A} + \delta(A,Z)",
     run=semf, verify=_recheck(semf),
     parse=lambda q: _pairs(q, r"semi[- ]empirical mass formula|liquid[- ]drop binding energy", {"z": "Z", "a": "A"}),
     reference=ref_semf,
     wrong=_safe(lambda Z, A: _semf(Z, A, zz1=False)),
     random_inputs=_rand_semf,
     templates=["Semi-empirical mass formula: Z = 26, A = 56"],
     examples=[{"inputs": {"Z": 26, "A": 56},
                "outputs": {"B_MeV": 15.75 * 56 - 17.8 * 56 ** (2 / 3) - 0.711 * 26 * 25 / 56 ** (1 / 3) - 23.7 * 16 / 56 + 11.18 / 56 ** 0.5}},
               {"inputs": {"Z": 92, "A": 235},
                "outputs": {"B_MeV": 15.75 * 235 - 17.8 * 235 ** (2 / 3) - 0.711 * 92 * 91 / 235 ** (1 / 3) - 23.7 * 51 ** 2 / 235}}],
     questions=[{"q": "Semi-empirical mass formula: Z = 26, A = 56",
                 "outputs": {"B_MeV": 15.75 * 56 - 17.8 * 56 ** (2 / 3) - 0.711 * 26 * 25 / 56 ** (1 / 3) - 23.7 * 16 / 56 + 11.18 / 56 ** 0.5}},
                {"q": "Semi-empirical mass formula: Z = 30, A = 20", "outcome": "INVALID_INPUT"},
                {"q": "Semi-empirical mass formula: Z = 26", "outcome": "NEEDS_INPUT"}])
