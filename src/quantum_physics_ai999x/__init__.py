"""quantum-physics-ai999x — offline quantum physics answers with their formula, steps and verification.

    import quantum_physics_ai999x as m
    m.ask("Lorentz factor: v = 0.8 c").show()                        # gamma = 1.6667
    m.ask("Hydrogen transition: Z = 1, n_upper = 3, n_lower = 2").show()   # H-alpha, 656.1 nm (vacuum, infinite nuclear mass)
    m.solve("two_qubit_circuit", start="00", gates="H0 CNOT01").show()  # Bell state: p00 = p11 = 0.5, concurrence 1
    m.capabilities()                   # every certified capability

Covers special relativity (Lorentz factor, time dilation, length contraction, momentum and energy, velocity addition,
Doppler shift, aberration, intervals, invariant mass, collision energies), photons and matter waves (photoelectric and
Compton effects, de Broglie wavelengths, the uncertainty principle, coherent states), the hydrogen atom and angular
momentum (Bohr levels, spectral lines with the reduced mass, Moseley's law, radial expectation values and
probabilities, Landé g factor, Clebsch-Gordan coefficients, 3j symbols, Wigner rotation matrices), quantum wells and
tunnelling (particle in a box, harmonic oscillator, finite square well, rectangular barrier, WKB field emission),
spin, qubits and quantum gates (measurement along an axis, Bloch vectors, one- and two-qubit circuits, Bell states and
concurrence, gate rotations, Rabi and Larmor dynamics, entropy, fidelity, CHSH, Grover search), nuclear and particle
physics (decay law, dating, named nuclides, decay chains, two-body decays, reaction thresholds, Coulomb barrier,
decay lengths) and statistical physics (Wien, Stefan-Boltzmann, Planck, band fractions, photon gas, two-level,
Einstein and Debye models, Fermi energy, Fermi-Dirac and Bose-Einstein statistics, Fermi gas chemical potential, BEC).
Questions use the documented form "<capability phrase>: name = value, ..." (see m.templates()). Constants are CODATA
2022. Built by the AI999X model machine from the same engine as every domain; a question that is not determined is
refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("quantum_physics", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
