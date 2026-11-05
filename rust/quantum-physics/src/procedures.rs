//! quantum_physics: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/quantum_physics/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "barrier_transmission",
    "bec_critical_temperature",
    "blackbody_band_fraction",
    "bohr_energy_level",
    "bose_einstein_occupation",
    "chsh_bell_test",
    "clebsch_gordan",
    "coherent_state_photon_statistics",
    "compton_scattering",
    "coulomb_barrier",
    "de_broglie_accelerated",
    "de_broglie_wavelength_mv",
    "debye_heat_capacity",
    "decay_chain_parent_daughter",
    "einstein_oscillator",
    "fermi_dirac_occupation",
    "fermi_energy_free_electrons",
    "fermi_energy_metal",
    "fermi_gas_chemical_potential",
    "finite_square_well",
    "fixed_target_cm_energy",
    "gate_rotation_axis",
    "grover_search",
    "harmonic_oscillator_forbidden_probability",
    "harmonic_oscillator_levels",
    "heisenberg_minimum_uncertainty",
    "hydrogen_radial_expectations",
    "hydrogen_radial_probability",
    "hydrogen_transition",
    "hydrogen_transition_reduced_mass",
    "infinite_well_levels",
    "infinite_well_probability",
    "infinite_well_transition",
    "infinite_well_uncertainty",
    "invariant_mass",
    "lande_g_zeeman",
    "larmor_precession",
    "length_contraction",
    "lorentz_factor",
    "moseley_k_alpha",
    "nuclear_reaction_threshold",
    "nuclide_decay",
    "particle_decay_length",
    "photoelectric_effect",
    "photon_energy",
    "photon_gas",
    "planck_spectral_radiance",
    "qubit_entropy_purity",
    "qubit_fidelity",
    "qubit_state_bloch",
    "rabi_oscillation",
    "radioactive_dating_age",
    "radioactive_decay_law",
    "relativistic_aberration",
    "relativistic_doppler",
    "relativistic_energy_from_momentum",
    "relativistic_momentum_energy",
    "relativistic_speed_from_kinetic_energy",
    "relativistic_velocity_addition",
    "rest_energy",
    "semi_empirical_mass_formula",
    "single_qubit_circuit",
    "spacetime_interval",
    "spin_half_measurement",
    "spin_rotation_wigner_d",
    "stefan_boltzmann",
    "step_potential",
    "thermal_de_broglie_wavelength",
    "time_dilation",
    "two_body_decay",
    "two_level_thermodynamics",
    "two_qubit_circuit",
    "wien_peak",
    "wigner_3j_symbol",
    "wkb_field_emission",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "barrier_transmission" => {
            let r = f_barrier_transmission(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_39(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bec_critical_temperature" => {
            let r = f_bec_temperature(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_78(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "blackbody_band_fraction" => {
            let r = f_band_fraction(match v.get("lam1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bohr_energy_level" => {
            let r = f_bohr_level(match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_168(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bose_einstein_occupation" => {
            let r = f_bose_einstein(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_208(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chsh_bell_test" => {
            let r = f_chsh(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clebsch_gordan" => {
            let r = f_clebsch_gordan(match v.get("j1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("j2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_315(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coherent_state_photon_statistics" => {
            let r = f_coherent_state(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_357(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "compton_scattering" => {
            let r = f_compton(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_397(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coulomb_barrier" => {
            let r = f_coulomb_barrier(match v.get("Z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_442(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "de_broglie_accelerated" => {
            let r = f_de_broglie_voltage(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("particle") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_480(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "de_broglie_wavelength_mv" => {
            let r = f_de_broglie_mv(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_522(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "debye_heat_capacity" => {
            let r = f_debye(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_565(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "decay_chain_parent_daughter" => {
            let r = f_decay_chain(match v.get("T_half_P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_618(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "einstein_oscillator" => {
            let r = f_einstein_oscillator(match v.get("theta_E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_659(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fermi_dirac_occupation" => {
            let r = f_fermi_dirac(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_699(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fermi_energy_free_electrons" => {
            let r = f_fermi_energy(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_736(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fermi_energy_metal" => {
            let r = f_fermi_energy_metal(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_779(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fermi_gas_chemical_potential" => {
            let r = f_fermi_chemical_potential(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_850(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "finite_square_well" => {
            let r = f_finite_well(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_922(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fixed_target_cm_energy" => {
            let r = f_fixed_target_cm_energy(match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_967(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gate_rotation_axis" => {
            let r = f_gate_rotation(match v.get("gates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1048(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "grover_search" => {
            let r = f_grover(match v.get("n_qubits") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1097(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "harmonic_oscillator_forbidden_probability" => {
            let r = f_ho_forbidden(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "harmonic_oscillator_levels" => {
            let r = f_ho_levels(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1194(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heisenberg_minimum_uncertainty" => {
            let r = f_heisenberg(match v.get("dx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydrogen_radial_expectations" => {
            let r = f_radial_expectations(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydrogen_radial_probability" => {
            let r = f_radial_probability(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1324(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydrogen_transition" => {
            let r = f_hydrogen_transition(match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_upper") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_lower") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1363(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydrogen_transition_reduced_mass" => {
            let r = f_hydrogen_transition_reduced(match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_upper") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_lower") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M_u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1403(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "infinite_well_levels" => {
            let r = f_infinite_well_levels(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1444(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "infinite_well_probability" => {
            let r = f_infinite_well_probability(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1489(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "infinite_well_transition" => {
            let r = f_infinite_well_transition(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_upper") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_lower") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1532(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "infinite_well_uncertainty" => {
            let r = f_infinite_well_expectations(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1570(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "invariant_mass" => {
            let r = f_invariant_mass(match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1621(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lande_g_zeeman" => {
            let r = f_lande_zeeman(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mJ") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1685(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "larmor_precession" => {
            let r = f_larmor(match v.get("particle") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "length_contraction" => {
            let r = f_length_contraction(match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1767(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lorentz_factor" => {
            let r = f_lorentz_factor(match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moseley_k_alpha" => {
            let r = f_moseley(match v.get("Z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1842(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuclear_reaction_threshold" => {
            let r = f_reaction_threshold(match v.get("m_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuclide_decay" => {
            let r = f_nuclide_decay(match v.get("nuclide") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1921(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "particle_decay_length" => {
            let r = f_decay_length(match v.get("particle") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1958(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photoelectric_effect" => {
            let r = f_photoelectric(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1999(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photon_energy" => {
            let r = f_photon_energy(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2036(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photon_gas" => {
            let r = f_photon_gas(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2072(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "planck_spectral_radiance" => {
            let r = f_planck_radiance(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "qubit_entropy_purity" => {
            let r = f_qubit_entropy(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2158(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "qubit_fidelity" => {
            let r = f_qubit_fidelity(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2210(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "qubit_state_bloch" => {
            let r = f_qubit_state(match v.get("a_re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_im") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b_re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b_im") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rabi_oscillation" => {
            let r = f_rabi(match v.get("Omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2301(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radioactive_dating_age" => {
            let r = f_decay_age(match v.get("fraction") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2343(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radioactive_decay_law" => {
            let r = f_decay_law(match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2385(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "relativistic_aberration" => {
            let r = f_aberration(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2427(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "relativistic_doppler" => {
            let r = f_relativistic_doppler(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("motion") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "relativistic_energy_from_momentum" => {
            let r = f_energy_from_momentum(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2508(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "relativistic_momentum_energy" => {
            let r = f_momentum_energy(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2549(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "relativistic_speed_from_kinetic_energy" => {
            let r = f_speed_from_kinetic(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2588(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "relativistic_velocity_addition" => {
            let r = f_velocity_addition(match v.get("u_prime") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2628(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rest_energy" => {
            let r = f_rest_energy(match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2665(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "semi_empirical_mass_formula" => {
            let r = f_semf(match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2711(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "single_qubit_circuit" => {
            let r = f_single_qubit_circuit(match v.get("start") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gates") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2781(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spacetime_interval" => {
            let r = f_spacetime_interval(match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2827(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spin_half_measurement" => {
            let r = f_spin_measurement(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi_n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2871(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spin_rotation_wigner_d" => {
            let r = f_wigner_small_d(match v.get("j") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2933(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stefan_boltzmann" => {
            let r = f_stefan_boltzmann(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("emissivity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2976(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "step_potential" => {
            let r = f_step_potential(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thermal_de_broglie_wavelength" => {
            let r = f_thermal_de_broglie(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3056(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "time_dilation" => {
            let r = f_time_dilation(match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3097(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_body_decay" => {
            let r = f_two_body_decay(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("MeV")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3142(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_level_thermodynamics" => {
            let r = f_two_level(match v.get("dE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3187(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_qubit_circuit" => {
            let r = f_two_qubit_circuit(match v.get("start") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gates") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wien_peak" => {
            let r = f_wien(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3308(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wigner_3j_symbol" => {
            let r = f_wigner_3j(match v.get("j1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("j2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3377(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wkb_field_emission" => {
            let r = f_wkb_triangular(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3416(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_A0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(5.29177210544e-11)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ALPHA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.0072973525643)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_AMU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.66053906892e-27)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_AMU_MEV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(931.49410372)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(299792458.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_H() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.62607015e-34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HBAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.0545718176461565e-34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.380649e-23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ME() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.1093837139e-31)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MEV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.6021766339999998e-13)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.67262192595e-27)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MUB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.2740100657e-24)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_H1_U() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.00782503223)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_N_U() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.00866491606)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_NA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.02214076e+23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_QE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RGAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RINF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(10973731.568157)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SEMF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("aV"), Py::Float(15.75)), (Py::str("aS"), Py::Float(17.8)), (Py::str("aC"), Py::Float(0.711)), (Py::str("aA"), Py::Float(23.7)), (Py::str("aP"), Py::Float(11.18))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SIGMA_SB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(5.6703744191844314e-08)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_X_WIEN_F() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.8214393721220787)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_X_WIEN_L() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(4.965114231744276)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ZETA3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.2020569031595942)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ZETA32() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.612375348685488)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ANG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*([-+]?)\\s*(\\d+(?:\\.\\d*)?|\\.\\d+)?\\s*\\*?\\s*(pi|π)?\\s*(?:/\\s*(\\d+(?:\\.\\d*)?))?\\s*"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BERN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(f__bernoulli(Py::Int(60i64))?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FIXED() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("I"), Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])])), (Py::str("H"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.7071067811865475), Py::Float(0.7071067811865475)]), Py::tuple(vec![Py::Float(0.7071067811865475), Py::Float(-0.7071067811865475)])])), (Py::str("X"), Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)])])), (Py::str("Y"), Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Complex(-0.0, -1.0)]), Py::tuple(vec![Py::Complex(0.0, 1.0), Py::Int(0i64)])])), (Py::str("Z"), Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(-1i64)])])), (Py::str("S"), Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Complex(0.0, 1.0)])])), (Py::str("SDG"), Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Complex(-0.0, -1.0)])])), (Py::str("T"), Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Complex(0.7071067811865476, 0.7071067811865475)])])), (Py::str("TDG"), Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Complex(0.7071067811865476, -0.7071067811865475)])]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GYRO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("electron"), Py::Float(176085962784.0)), (Py::str("proton"), Py::Float(267522187.08))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NUCLIDES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("C-14"), Py::Float(179874478656.0)), (Py::str("H-3"), Py::Float(388781329.30560005)), (Py::str("Co-60"), Py::Float(166346024.445504)), (Py::str("Cs-137"), Py::Float(951980944.7479681)), (Py::str("Sr-90"), Py::Float(908523901.8432001)), (Py::str("I-131"), Py::Float(692988.48)), (Py::str("Rn-222"), Py::Float(330350.4)), (Py::str("Ra-226"), Py::Float(50491081728.0)), (Py::str("U-238"), Py::Float(1.4099634572544002e+17)), (Py::str("U-235"), Py::Float(2.221607596032e+16)), (Py::str("Pu-239"), Py::Float(760837487788.8)), (Py::str("K-40"), Py::Float(3.947771452608e+16)), (Py::str("Am-241"), Py::Float(13638903451.776001)), (Py::str("Tc-99m"), Py::Float(21654.0)), (Py::str("F-18"), Py::Float(6586.2)), (Py::str("P-32"), Py::Float(1232323.2)), (Py::str("Po-210"), Py::Float(11955686.4)), (Py::str("Th-232"), Py::Float(4.43374811424e+17)), (Py::str("Na-22"), Py::Float(82107965.967552)), (Py::str("Ir-192"), Py::Float(6378652.8)), (Py::str("Kr-85"), Py::Float(339426296.91648)), (Py::str("Xe-133"), Py::Float(452995.2)), (Py::str("Tl-201"), Py::Float(262483.2)), (Py::str("Ga-67"), Py::Float(281767.68)), (Py::str("In-111"), Py::Float(242326.08)), (Py::str("Pb-210"), Py::Float(700563758.9760001)), (Py::str("Be-7"), Py::Float(4598208.0)), (Py::str("Ni-63"), Py::Float(3158848300.608)), (Py::str("Fe-55"), Py::Float(86371306.68096001)), (Py::str("Cl-36"), Py::Float(9498634750080.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PARTICLES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("pi+"), Py::tuple(vec![Py::str("pi+"), Py::Float(139.57039), Py::Float(26.032746280292145)])), (Py::str("K+"), Py::tuple(vec![Py::str("K+"), Py::Float(493.677), Py::Float(12.379386062646352)])), (Py::str("K_S0"), Py::tuple(vec![Py::str("K(S)0"), Py::Float(497.611), Py::Float(0.0895429010381056)])), (Py::str("K_L0"), Py::tuple(vec![Py::str("K(L)0"), Py::Float(497.611), Py::Float(51.143120198205644)])), (Py::str("mu-"), Py::tuple(vec![Py::str("mu-"), Py::Float(105.6583755), Py::Float(2196.981174899978)])), (Py::str("tau-"), Py::tuple(vec![Py::str("tau-"), Py::Float(1776.93), Py::Float(0.0002903449302827113)])), (Py::str("n"), Py::tuple(vec![Py::str("n"), Py::Float(939.5654219), Py::Float(878318597479.1921)])), (Py::str("Lambda"), Py::tuple(vec![Py::str("Lambda"), Py::Float(1115.683), Py::Float(0.26171449580552947)])), (Py::str("Sigma+"), Py::tuple(vec![Py::str("Sigma+"), Py::Float(1189.37), Py::Float(0.08018174649176595)])), (Py::str("Xi-"), Py::tuple(vec![Py::str("Xi-"), Py::Float(1321.71), Py::Float(0.16373431764947924)])), (Py::str("Omega-"), Py::tuple(vec![Py::str("Omega-"), Py::Float(1672.45), Py::Float(0.08207131632804322)])), (Py::str("D+"), Py::tuple(vec![Py::str("D+"), Py::Float(1869.66), Py::Float(0.0010332997754331345)])), (Py::str("D0"), Py::tuple(vec![Py::str("D0"), Py::Float(1864.84), Py::Float(0.0004103565816402161)])), (Py::str("B+"), Py::tuple(vec![Py::str("B+"), Py::Float(5279.41), Py::Float(0.0016369359784901931)])), (Py::str("B0"), Py::tuple(vec![Py::str("B0"), Py::Float(5279.72), Py::Float(0.0015113937013798086)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PI4_15() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.493939402266828)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SERIES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(1i64), Py::str("Lyman")), (Py::Int(2i64), Py::str("Balmer")), (Py::Int(3i64), Py::str("Paschen")), (Py::Int(4i64), Py::str("Brackett")), (Py::Int(5i64), Py::str("Pfund")), (Py::Int(6i64), Py::str("Humphreys"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__START1() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("0"), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)])), (Py::str("1"), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])), (Py::str("+"), Py::tuple(vec![Py::Float(0.7071067811865475), Py::Float(0.7071067811865475)])), (Py::str("-"), Py::tuple(vec![Py::Float(0.7071067811865475), Py::Float(-0.7071067811865475)])), (Py::str("+i"), Py::tuple(vec![Py::Float(0.7071067811865475), Py::Complex(0.0, 0.7071067811865475)])), (Py::str("-i"), Py::tuple(vec![Py::Float(0.7071067811865475), Py::Complex(0.0, -0.7071067811865475)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TOK1() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*(SDG|TDG|RX|RY|RZ|H|X|Y|Z|S|T|I|P)(?:\\s*\\(([^)]*)\\))?\\s*(?:,|\\s|$)"), &[], (Py::Int(2i64)).i()?)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TOK2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*(?:(CNOT|CX)(01|10)|(CZ)|(SWAP)|(SDG|TDG|RX|RY|RZ|H|X|Y|Z|S|T|I|P)([01])(?:\\s*\\(([^)]*)\\))?)\\s*(?:,|\\s|$)"), &[], (Py::Int(2i64)).i()?)?) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__angle(mut v_text: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_m = py::method(&c__ANG()?, "fullmatch", &[v_text.clone()])?;
    if ({ let t3223 = py::not(&v_m.clone()); if t3223.truthy() { t3223 } else { py::not(&{ let t3222 = py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?; if t3222.truthy() { t3222 } else { py::method(&v_m.clone(), "group", &[Py::Int(3i64)])? } }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["cannot read the angle ".to_string(), py::format(&Py::str(&py::repr(&v_text.clone())), "")?, " (write e.g. 0.5, pi/4, -3pi/2)".to_string()].concat()))));
    }
    v_v = (if (py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?).truthy() { py::float(&py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)? } else { Py::Float(1.0) });
    if (py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?).truthy() {
        v_v = py::mul(&v_v, &Py::Float(std::f64::consts::PI))?;
    }
    if (py::method(&v_m.clone(), "group", &[Py::Int(4i64)])?).truthy() {
        v_v = py::truediv(&v_v, &py::float(&py::method(&v_m.clone(), "group", &[Py::Int(4i64)])?)?)?;
    }
    return Ok((if ({ let t3224 = py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?; Py::Bool({ let t3225 = Py::str("-"); if py::eq(&t3224, &t3225) { true } else { false } }) }).truthy() { py::neg(&v_v.clone())? } else { v_v.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__apply2(mut v_g: Py, mut v_v: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bits: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_nbits: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_v = Py::list((v_v.clone()).items()?);
    if ({ let t3245 = py::getitem(&v_g.clone(), &Py::Int(0i64))?; Py::Bool({ let t3246 = Py::str("CNOT"); if py::eq(&t3245, &t3246) { true } else { false } }) }).truthy() {
        let t3247 = (Py::tuple(vec![py::getitem(&v_g.clone(), &Py::Int(1i64))?, py::getitem(&v_g.clone(), &Py::Int(2i64))?])).items()?;
        if t3247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t3247[0].clone();
        v_t = t3247[1].clone();
        v_out = Py::list((v_v.clone()).items()?);
        for t3248 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
            v_i = t3248;
            v_bits = Py::list(vec![py::bitand(&py::rshift(&v_i.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, py::bitand(&v_i.clone(), &Py::Int(1i64))?]);
            if (py::getitem(&v_bits.clone(), &v_c.clone())?).truthy() {
                let t3249 = v_bits.clone(); let t3250 = v_t.clone();
                py::setitem(&t3249, &t3250, py::bitxor(&py::getitem(&t3249, &t3250)?, &Py::Int(1i64))?)?;
                py::setitem(&v_out.clone(), &py::add(&py::mul(&Py::Int(2i64), &py::getitem(&v_bits.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_bits.clone(), &Py::Int(1i64))?)?, py::getitem(&v_v.clone(), &v_i.clone())?)?;
            } else {
                py::setitem(&v_out.clone(), &v_i.clone(), py::getitem(&v_v.clone(), &v_i.clone())?)?;
            }
        }
        return Ok(v_out.clone());
    }
    if ({ let t3251 = py::getitem(&v_g.clone(), &Py::Int(0i64))?; Py::Bool({ let t3252 = Py::str("CZ"); if py::eq(&t3251, &t3252) { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?, py::neg(&py::getitem(&v_v.clone(), &Py::Int(3i64))?)?]));
    }
    if ({ let t3253 = py::getitem(&v_g.clone(), &Py::Int(0i64))?; Py::Bool({ let t3254 = Py::str("SWAP"); if py::eq(&t3253, &t3254) { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?, py::getitem(&v_v.clone(), &Py::Int(3i64))?]));
    }
    let t3255 = (Py::tuple(vec![py::getitem(&v_g.clone(), &Py::Int(1i64))?, py::getitem(&v_g.clone(), &Py::Int(3i64))?])).items()?;
    if t3255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3255[0].clone();
    v_M = t3255[1].clone();
    v_out = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &Py::Int(4i64))?;
    for t3256 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
        v_i = t3256;
        v_bits = Py::list(vec![py::bitand(&py::rshift(&v_i.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, py::bitand(&v_i.clone(), &Py::Int(1i64))?]);
        v_b = py::getitem(&v_bits.clone(), &v_q.clone())?;
        for t3257 in py::iter(&(Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])))? {
            v_nb = t3257;
            v_nbits = Py::list((v_bits.clone()).items()?);
            py::setitem(&v_nbits.clone(), &v_q.clone(), v_nb.clone())?;
            let t3258 = v_out.clone(); let t3259 = py::add(&py::mul(&Py::Int(2i64), &py::getitem(&v_nbits.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_nbits.clone(), &Py::Int(1i64))?)?;
            py::setitem(&t3258, &t3259, py::iadd(&py::getitem(&t3258, &t3259)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_nb.clone())?, &v_b.clone())?, &py::getitem(&v_v.clone(), &v_i.clone())?)?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bernoulli(mut v_n: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_B = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    for t118 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t118;
        let _ = py::method(&v_B.clone(), "append", &[py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t119 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t119;
out.push(py::mul(&py::comb(&py::add(&v_m.clone(), &Py::Int(1i64))?, &v_k.clone())?, &py::getitem(&v_B.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::add(&v_m.clone(), &Py::Int(1i64))?)?])?;
    }
    return Ok(v_B.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta(mut v_v: Py) -> R {
    if (py::not(&{ let t3087 = py::math1("isfinite", &v_v.clone())?; if !t3087.truthy() { t3087 } else { { let t3085 = py::abs(&v_v.clone())?; Py::Bool({ let t3086 = c_C()?; if py::lt(&t3085, &t3086)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the speed must be smaller than the speed of light (|v| < c)"))));
    }
    return Ok(py::truediv(&v_v.clone(), &c_C()?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bloch(mut v_theta: Py, mut v_phi: Py) -> R {
    return Ok(Py::tuple(vec![py::mul(&py::math1("sin", &v_theta.clone())?, &py::math1("cos", &v_phi.clone())?)?, py::mul(&py::math1("sin", &v_theta.clone())?, &py::math1("sin", &v_phi.clone())?)?, py::math1("cos", &v_theta.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canon(mut v_amps: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_ref: Py = Py::None;
    v_ref = (match ('g2764: { let mut out = vec![]; for t2765 in py::iter(&(v_amps.clone()))? { let mut v_a: Py; 
v_a = t2765;
if ({ let t2766 = py::abs(&v_a.clone())?; Py::Bool({ let t2767 = Py::Float(1e-09); if py::gt(&t2766, &t2767)? { true } else { false } }) }).truthy() { out.push(v_a.clone()); break 'g2764 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::Int(1i64) });
    v_ph = py::truediv(&py::abs(&v_ref.clone())?, &v_ref.clone())?;
    v_out = Py::list(vec![]);
    for t2768 in py::iter(&(v_amps.clone()))? {
        v_a = t2768;
        v_b = py::mul(&py::complex(&[v_a.clone()])?, &v_ph.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[py::complex(&[f__snap(py::real(&v_b.clone())?, Py::Float(1.0))?, f__snap(py::imag(&v_b.clone())?, Py::Float(1.0))?])?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_exact(mut v_j1: Py, mut v_m1: Py, mut v_j2: Py, mut v_m2: Py, mut v_J: Py, mut v_M: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_args: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pref: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3338 = ({ let mut out = vec![]; for t3333 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_j1.clone(), Py::str("j1")]), Py::tuple(vec![v_m1.clone(), Py::str("m1")]), Py::tuple(vec![v_j2.clone(), Py::str("j2")]), Py::tuple(vec![v_m2.clone(), Py::str("m2")]), Py::tuple(vec![v_J.clone(), Py::str("J")]), Py::tuple(vec![v_M.clone(), Py::str("M")])])))? { let mut v_n: Py; let mut v_v: Py; 
let t3334 = (t3333).items()?;
if t3334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3334[0].clone();
v_n = t3334[1].clone();
out.push(f__half(v_v.clone(), v_n.clone())?); } Py::list(out) }).items()?;
    if t3338.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_j1 = t3338[0].clone();
    v_m1 = t3338[1].clone();
    v_j2 = t3338[2].clone();
    v_m2 = t3338[3].clone();
    v_J = t3338[4].clone();
    v_M = t3338[5].clone();
    for t3339 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_j1.clone(), v_m1.clone(), Py::str("1")]), Py::tuple(vec![v_j2.clone(), v_m2.clone(), Py::str("2")]), Py::tuple(vec![v_J.clone(), v_M.clone(), Py::str("")])])))? {
        let t3348 = (t3339).items()?;
        if t3348.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_j = t3348[0].clone();
        v_m = t3348[1].clone();
        v_n = t3348[2].clone();
        if ({ let t3347 = { let t3340 = v_j.clone(); Py::Bool({ let t3341 = Py::Int(0i64); if py::lt(&t3340, &t3341)? { true } else { false } }) }; if t3347.truthy() { t3347 } else { { let t3346 = { let t3342 = py::abs(&v_m.clone())?; Py::Bool({ let t3343 = v_j.clone(); if py::gt(&t3342, &t3343)? { true } else { false } }) }; if t3346.truthy() { t3346 } else { { let t3344 = py::denominator(&py::sub(&v_j.clone(), &v_m.clone())?)?; Py::Bool({ let t3345 = Py::Int(1i64); if py::ne(&t3344, &t3345) { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["need j".to_string(), py::format(&v_n.clone(), "")?, " >= 0, |m".to_string(), py::format(&v_n.clone(), "")?, "| <= j".to_string(), py::format(&v_n.clone(), "")?, " and j".to_string(), py::format(&v_n.clone(), "")?, " - m".to_string(), py::format(&v_n.clone(), "")?, " a whole number".to_string()].concat()))));
        }
    }
    if ({ let t3359 = { let t3349 = py::add(&v_m1.clone(), &v_m2.clone())?; Py::Bool({ let t3350 = v_M.clone(); if py::ne(&t3349, &t3350) { true } else { false } }) }; if t3359.truthy() { t3359 } else { { let t3358 = { let t3351 = v_J.clone(); Py::Bool({ let t3352 = py::abs(&py::sub(&v_j1.clone(), &v_j2.clone())?)?; if py::lt(&t3351, &t3352)? { true } else { false } }) }; if t3358.truthy() { t3358 } else { { let t3357 = { let t3353 = v_J.clone(); Py::Bool({ let t3354 = py::add(&v_j1.clone(), &v_j2.clone())?; if py::gt(&t3353, &t3354)? { true } else { false } }) }; if t3357.truthy() { t3357 } else { { let t3355 = py::denominator(&py::sub(&py::add(&v_j1.clone(), &v_j2.clone())?, &v_J.clone())?)?; Py::Bool({ let t3356 = Py::Int(1i64); if py::ne(&t3355, &t3356) { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::tuple(vec![Py::Int(1i64), py::fraction(&[Py::Int(0i64)])?]));
    }
    v_pref = py::fraction(&[py::mul(&py::mul(&py::mul(&py::numerator(&py::add(&py::mul(&Py::Int(2i64), &v_J.clone())?, &Py::Int(1i64))?)?, &f__fact(py::sub(&py::add(&v_J.clone(), &v_j1.clone())?, &v_j2.clone())?)?)?, &f__fact(py::add(&py::sub(&v_J.clone(), &v_j1.clone())?, &v_j2.clone())?)?)?, &f__fact(py::sub(&py::add(&v_j1.clone(), &v_j2.clone())?, &v_J.clone())?)?)?, py::mul(&py::denominator(&py::add(&py::mul(&Py::Int(2i64), &v_J.clone())?, &Py::Int(1i64))?)?, &f__fact(py::add(&py::add(&py::add(&v_j1.clone(), &v_j2.clone())?, &v_J.clone())?, &Py::Int(1i64))?)?)?])?;
    v_pref = py::mul(&v_pref, &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&f__fact(py::add(&v_J.clone(), &v_M.clone())?)?, &f__fact(py::sub(&v_J.clone(), &v_M.clone())?)?)?, &f__fact(py::sub(&v_j1.clone(), &v_m1.clone())?)?)?, &f__fact(py::add(&v_j1.clone(), &v_m1.clone())?)?)?, &f__fact(py::sub(&v_j2.clone(), &v_m2.clone())?)?)?, &f__fact(py::add(&v_j2.clone(), &v_m2.clone())?)?)?)?;
    v_s = py::fraction(&[Py::Int(0i64)])?;
    for t3360 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&py::int(&py::sub(&py::add(&v_j1.clone(), &v_j2.clone())?, &v_J.clone())?)?, &Py::Int(1i64))?).i()?]) {
        v_k = t3360;
        v_args = Py::list(vec![v_k.clone(), py::sub(&py::sub(&py::add(&v_j1.clone(), &v_j2.clone())?, &v_J.clone())?, &v_k.clone())?, py::sub(&py::sub(&v_j1.clone(), &v_m1.clone())?, &v_k.clone())?, py::sub(&py::add(&v_j2.clone(), &v_m2.clone())?, &v_k.clone())?, py::add(&py::add(&py::sub(&v_J.clone(), &v_j2.clone())?, &v_m1.clone())?, &v_k.clone())?, py::add(&py::sub(&py::sub(&v_J.clone(), &v_j1.clone())?, &v_m2.clone())?, &v_k.clone())?]);
        if (Py::Bool(({ let mut out = vec![]; for t3361 in py::iter(&(v_args.clone()))? { let mut v_a: Py; 
v_a = t3361;
out.push({ let t3362 = v_a.clone(); Py::Bool({ let t3363 = Py::Int(0i64); if py::lt(&t3362, &t3363)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
            continue;
        }
        v_den = Py::Int(1i64);
        for t3364 in py::iter(&(v_args.clone()))? {
            v_a = t3364;
            v_den = py::mul(&v_den, &f__fact(v_a.clone())?)?;
        }
        v_s = py::iadd(&v_s, &py::fraction(&[py::pow(&py::neg(&Py::Int(1i64))?, &v_k.clone())?, v_den.clone()])?)?;
    }
    return Ok(Py::tuple(vec![(if ({ let t3365 = v_s.clone(); Py::Bool({ let t3366 = Py::Int(0i64); if py::ge(&t3365, &t3366)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }), py::mul(&py::mul(&v_pref.clone(), &v_s.clone())?, &v_s.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_nl(mut v_n: Py, mut v_l: Py, mut v_Z: Py) -> R {
    let _ = f__quantum_n(v_n.clone(), Py::str("n"))?;
    let _ = f__quantum_n(v_Z.clone(), Py::str("Z"))?;
    if (py::not(&{ let t1299 = Py::Int(0i64); Py::Bool({ let t1300 = v_l.clone(); if py::le(&t1299, &t1300)? { { let t1301 = v_n.clone(); if py::lt(&t1300, &t1301)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the orbital quantum number must satisfy 0 <= l <= n - 1"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fact(mut v_x: Py) -> R {
    return Ok(py::factorial(&py::int(&v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fd_half(mut v_eta: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tmax: Py = Py::None;
    v_h = py::truediv(&Py::Float(0.2), &py::math1("sqrt", &py::minmax(&[v_eta.clone(), Py::Float(1.0)], true)?)?)?;
    v_tmax = py::math1("sqrt", &py::add(&py::minmax(&[v_eta.clone(), Py::Float(0.0)], true)?, &Py::Float(46.0))?)?;
    let t818 = Py::Float(0.0);
    v_I = t818.clone();
    v_dI = t818.clone();
    v_k = Py::Int(1i64);
    while ({ let t823 = py::mul(&v_k.clone(), &v_h.clone())?; Py::Bool({ let t824 = v_tmax.clone(); if py::le(&t823, &t824)? { true } else { false } }) }).truthy() {
        v_t = py::mul(&v_k.clone(), &v_h.clone())?;
        v_a = py::sub(&py::mul(&v_t.clone(), &v_t.clone())?, &v_eta.clone())?;
        if ({ let t819 = v_a.clone(); Py::Bool({ let t820 = Py::Int(0i64); if py::gt(&t819, &t820)? { true } else { false } }) }).truthy() {
            v_e = py::math1("exp", &py::neg(&v_a.clone())?)?;
            let t821 = (Py::tuple(vec![py::truediv(&v_e.clone(), &py::add(&Py::Int(1i64), &v_e.clone())?)?, py::truediv(&v_e.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_e.clone())?, &Py::Int(2i64))?)?])).items()?;
            if t821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t821[0].clone();
            v_df = t821[1].clone();
        } else {
            v_e = py::math1("exp", &v_a.clone())?;
            let t822 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_e.clone())?)?, py::truediv(&v_e.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_e.clone())?, &Py::Int(2i64))?)?])).items()?;
            if t822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t822[0].clone();
            v_df = t822[1].clone();
        }
        v_I = py::iadd(&v_I, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_t.clone())?, &v_t.clone())?, &v_f.clone())?)?;
        v_dI = py::iadd(&v_dI, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_t.clone())?, &v_t.clone())?, &v_df.clone())?)?;
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
    }
    return Ok(Py::tuple(vec![py::mul(&v_I.clone(), &v_h.clone())?, py::mul(&v_dI.clone(), &v_h.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fermi_from_n(mut v_n: Py) -> R {
    let mut v_EF: Py = Py::None;
    let mut v_kF: Py = Py::None;
    v_kF = py::pow(&py::mul(&py::mul(&Py::Int(3i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_n.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    v_EF = py::truediv(&py::mul(&py::pow(&c_HBAR()?, &Py::Int(2i64))?, &py::pow(&v_kF.clone(), &Py::Int(2i64))?)?, &py::mul(&Py::Int(2i64), &c_ME()?)?)?;
    return Ok(Py::tuple(vec![v_kF.clone(), v_EF.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fw_root(mut v_f: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mid: Py = Py::None;
    v_flo = py::call(&v_f.clone(), &[v_lo.clone()])?;
    for t898 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t898;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t903 = { let t899 = v_mid.clone(); Py::Bool({ let t900 = v_lo.clone(); if py::le(&t899, &t900)? { true } else { false } }) }; if t903.truthy() { t903 } else { { let t901 = v_mid.clone(); Py::Bool({ let t902 = v_hi.clone(); if py::ge(&t901, &t902)? { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_mid.clone()])?;
        if ({ let t904 = { let t906 = v_fm.clone(); Py::Bool({ let t907 = Py::Int(0i64); if py::lt(&t906, &t907)? { true } else { false } }) }; Py::Bool({ let t905 = { let t908 = v_flo.clone(); Py::Bool({ let t909 = Py::Int(0i64); if py::lt(&t908, &t909)? { true } else { false } }) }; if py::eq(&t904, &t905) { true } else { false } }) }).truthy() {
            let t910 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t910[0].clone();
            v_flo = t910[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gamma_of_beta(mut v_b: Py) -> R {
    return Ok(py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::mul(&py::sub(&Py::Float(1.0), &v_b.clone())?, &py::add(&Py::Float(1.0), &v_b.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gates_1q(mut v_text: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    if ({ let t2736 = py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_)))); if t2736.truthy() { t2736 } else { py::not(&py::method(&v_text.clone(), "strip", &[])?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one gate (H, X, Y, Z, S, Sdg, T, Tdg, Rx(t), Ry(t), Rz(t), P(t))"))));
    }
    let t2737 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64), py::method(&v_text.clone(), "strip", &[])?])).items()?;
    if t2737.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_out = t2737[0].clone();
    v_pos = t2737[1].clone();
    v_s = t2737[2].clone();
    while ({ let t2758 = v_pos.clone(); Py::Bool({ let t2759 = py::len(&v_s.clone())?; if py::lt(&t2758, &t2759)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__TOK1()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if ({ let t2740 = py::not(&v_m.clone()); if t2740.truthy() { t2740 } else { { let t2738 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t2739 = v_pos.clone(); if py::eq(&t2738, &t2739) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the gate at ".to_string(), py::format(&Py::str(&py::repr(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(12i64))?).i()?), None)?)), "")?].concat()))));
        }
        v_name = py::method(&py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?, "upper", &[])?;
        if ({ let t2741 = v_name.clone(); Py::Bool({ let t2742 = Py::tuple(vec![Py::str("RX"), Py::str("RY"), Py::str("RZ"), Py::str("P")]); if py::contains(&t2742, &t2741)? { true } else { false } }) }).truthy() {
            if ({ let t2743 = py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?; Py::Bool({ let t2744 = Py::None; if py::is(&t2743, &t2744) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs an angle, e.g. ".to_string(), py::format(&v_name.clone(), "")?, "(pi/2)".to_string()].concat()))));
            }
            v_t = f__angle(py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)?;
            let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_name.clone(), v_t.clone(), f__rot(v_name.clone(), v_t.clone())?])])?;
        } else {
            if ({ let t2756 = py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?; Py::Bool({ let t2757 = Py::None; if !py::is(&t2756, &t2757) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " takes no angle".to_string()].concat()))));
            }
            let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_name.clone(), Py::None, py::getitem(&c__FIXED()?, &v_name.clone())?])])?;
        }
        v_pos = py::method(&v_m.clone(), "end", &[])?;
    }
    if ({ let t2760 = py::len(&v_out.clone())?; Py::Bool({ let t2761 = Py::Int(500i64); if py::gt(&t2760, &t2761)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 500 gates"))));
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gates_2q(mut v_text: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    if ({ let t3212 = py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_)))); if t3212.truthy() { t3212 } else { py::not(&py::method(&v_text.clone(), "strip", &[])?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one gate (H0, X1, Rz0(pi/2), CNOT01, CNOT10, CZ, SWAP, ...)"))));
    }
    let t3213 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64), py::method(&v_text.clone(), "strip", &[])?])).items()?;
    if t3213.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_out = t3213[0].clone();
    v_pos = t3213[1].clone();
    v_s = t3213[2].clone();
    while ({ let t3235 = v_pos.clone(); Py::Bool({ let t3236 = py::len(&v_s.clone())?; if py::lt(&t3235, &t3236)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__TOK2()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if ({ let t3216 = py::not(&v_m.clone()); if t3216.truthy() { t3216 } else { { let t3214 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t3215 = v_pos.clone(); if py::eq(&t3214, &t3215) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the gate at ".to_string(), py::format(&Py::str(&py::repr(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(12i64))?).i()?), None)?)), "")?].concat()))));
        }
        if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![Py::str("CNOT"), py::int(&py::getitem(&py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?, &Py::Int(0i64))?)?, py::int(&py::getitem(&py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?, &Py::Int(1i64))?)?])])?;
        } else {
            if (py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?).truthy() {
                let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![Py::str("CZ"), Py::Int(0i64), Py::Int(1i64)])])?;
            } else {
                if (py::method(&v_m.clone(), "group", &[Py::Int(4i64)])?).truthy() {
                    let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![Py::str("SWAP"), Py::Int(0i64), Py::Int(1i64)])])?;
                } else {
                    let t3217 = (Py::tuple(vec![py::method(&py::method(&v_m.clone(), "group", &[Py::Int(5i64)])?, "upper", &[])?, py::int(&py::method(&v_m.clone(), "group", &[Py::Int(6i64)])?)?])).items()?;
                    if t3217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_name = t3217[0].clone();
                    v_q = t3217[1].clone();
                    if ({ let t3218 = v_name.clone(); Py::Bool({ let t3219 = Py::tuple(vec![Py::str("RX"), Py::str("RY"), Py::str("RZ"), Py::str("P")]); if py::contains(&t3219, &t3218)? { true } else { false } }) }).truthy() {
                        if ({ let t3220 = py::method(&v_m.clone(), "group", &[Py::Int(7i64)])?; Py::Bool({ let t3221 = Py::None; if py::is(&t3220, &t3221) { true } else { false } }) }).truthy() {
                            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs an angle, e.g. ".to_string(), py::format(&v_name.clone(), "")?, py::format(&v_q.clone(), "")?, "(pi/2)".to_string()].concat()))));
                        }
                        v_t = f__angle(py::method(&v_m.clone(), "group", &[Py::Int(7i64)])?)?;
                        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_name.clone(), v_q.clone(), v_t.clone(), f__rot(v_name.clone(), v_t.clone())?])])?;
                    } else {
                        if ({ let t3233 = py::method(&v_m.clone(), "group", &[Py::Int(7i64)])?; Py::Bool({ let t3234 = Py::None; if !py::is(&t3233, &t3234) { true } else { false } }) }).truthy() {
                            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " takes no angle".to_string()].concat()))));
                        }
                        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_name.clone(), v_q.clone(), Py::None, py::getitem(&c__FIXED()?, &v_name.clone())?])])?;
                    }
                }
            }
        }
        v_pos = py::method(&v_m.clone(), "end", &[])?;
    }
    if ({ let t3237 = py::len(&v_out.clone())?; Py::Bool({ let t3238 = Py::Int(500i64); if py::gt(&t3237, &t3238)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 500 gates"))));
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gm1(mut v_b: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::math1("sqrt", &py::mul(&py::sub(&Py::Float(1.0), &v_b.clone())?, &py::add(&Py::Float(1.0), &v_b.clone())?)?)?;
    return Ok(py::truediv(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_s.clone(), &py::add(&Py::Float(1.0), &v_s.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__half(mut v_x: Py, mut v_name: Py) -> R {
    let mut v_t: Py = Py::None;
    v_t = py::mul(&Py::Int(2i64), &v_x.clone())?;
    if ({ let t3337 = py::not(&py::math1("isfinite", &v_t.clone())?); if t3337.truthy() { t3337 } else { { let t3335 = py::abs(&py::sub(&v_t.clone(), &py::round(&v_t.clone(), None)?)?)?; Py::Bool({ let t3336 = Py::Float(1e-09); if py::gt(&t3335, &t3336)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole or half-whole number".to_string()].concat()))));
    }
    return Ok(py::fraction(&[py::int(&py::round(&v_t.clone(), None)?)?, Py::Int(2i64)])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mv(mut v_M: Py, mut v_v: Py) -> R {
    return Ok(Py::list(vec![py::add(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?, py::add(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__one_qubit_result(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_ab: Py = Py::None;
    let t2771 = (f__canon(Py::list(vec![v_a.clone(), v_b.clone()]))?).items()?;
    if t2771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2771[0].clone();
    v_b = t2771[1].clone();
    v_ab = py::mul(&py::method(&v_a.clone(), "conjugate", &[])?, &v_b.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("alpha_re"), py::real(&v_a.clone())?), (Py::str("alpha_im"), py::imag(&v_a.clone())?), (Py::str("beta_re"), py::real(&v_b.clone())?), (Py::str("beta_im"), py::imag(&v_b.clone())?), (Py::str("p0"), f__snap(py::pow(&py::abs(&v_a.clone())?, &Py::Int(2i64))?, Py::Float(1.0))?), (Py::str("p1"), f__snap(py::pow(&py::abs(&v_b.clone())?, &Py::Int(2i64))?, Py::Float(1.0))?), (Py::str("bx"), f__snap(py::mul(&Py::Int(2i64), &py::real(&v_ab.clone())?)?, Py::Float(1.0))?), (Py::str("by"), f__snap(py::mul(&Py::Int(2i64), &py::imag(&v_ab.clone())?)?, Py::Float(1.0))?), (Py::str("bz"), f__snap(py::sub(&py::pow(&py::abs(&v_a.clone())?, &Py::Int(2i64))?, &py::pow(&py::abs(&v_b.clone())?, &Py::Int(2i64))?)?, Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t3407 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3408 = (t3407).items()?;
if t3408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3408[0].clone();
v_v = t3408[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__planck_tail(mut v_x: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_head: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    if ({ let t110 = v_x.clone(); Py::Bool({ let t111 = Py::Int(1i64); if py::ge(&t110, &t111)? { true } else { false } }) }).truthy() {
        let t112 = (Py::tuple(vec![Py::Float(0.0), Py::Int(1i64)])).items()?;
        if t112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t112[0].clone();
        v_n = t112[1].clone();
        while (Py::Bool(true)).truthy() {
            v_term = py::mul(&py::math1("exp", &py::mul(&py::neg(&v_n.clone())?, &v_x.clone())?)?, &py::add(&py::add(&py::add(&py::truediv(&py::pow(&v_x.clone(), &Py::Int(3i64))?, &v_n.clone())?, &py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &v_x.clone())?, &v_x.clone())?, &py::pow(&v_n.clone(), &Py::Int(2i64))?)?)?, &py::truediv(&py::mul(&Py::Int(6i64), &v_x.clone())?, &py::pow(&v_n.clone(), &Py::Int(3i64))?)?)?, &py::truediv(&Py::Int(6i64), &py::pow(&v_n.clone(), &Py::Int(4i64))?)?)?)?;
            v_s = py::iadd(&v_s, &v_term.clone())?;
            if ({ let t117 = { let t113 = v_term.clone(); Py::Bool({ let t114 = py::mul(&Py::Float(1e-18), &v_s.clone())?; if py::lt(&t113, &t114)? { true } else { false } }) }; if t117.truthy() { t117 } else { { let t115 = v_n.clone(); Py::Bool({ let t116 = Py::Int(2000i64); if py::gt(&t115, &t116)? { true } else { false } }) } } }).truthy() {
                return Ok(v_s.clone());
            }
            v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        }
    }
    v_head = Py::Float(0.0);
    for t120 in py::iter(&(py::enumerate(&c__BERN()?, 0)?))? {
        let t121 = (t120).items()?;
        if t121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t121[0].clone();
        v_b = t121[1].clone();
        if (v_b.clone()).truthy() {
            v_head = py::iadd(&v_head, &py::truediv(&py::mul(&py::float(&v_b.clone())?, &py::pow(&v_x.clone(), &py::add(&v_k.clone(), &Py::Int(3i64))?)?)?, &py::mul(&py::factorial(&v_k.clone())?, &py::add(&v_k.clone(), &Py::Int(3i64))?)?)?)?;
        }
    }
    return Ok(py::sub(&c__PI4_15()?, &v_head.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__quantum_n(mut v_n: Py, mut v_name: Py) -> R {
    if ({ let t1557 = v_n.clone(); Py::Bool({ let t1558 = Py::Int(1i64); if py::lt(&t1557, &t1558)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive whole number".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t3409 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t3409.truthy() { t3409 } else { Py::Bool(matches!(v_x.clone(), Py::Str(_))) } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t3410 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3410;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() {
        return Ok(py::complex(&[f__r(py::real(&v_x.clone())?)?, f__r(py::imag(&v_x.clone())?)?])?);
    }
    if ({ let t3413 = { let t3411 = v_x.clone(); Py::Bool({ let t3412 = Py::Int(0i64); if py::eq(&t3411, &t3412) { true } else { false } }) }; if t3413.truthy() { t3413 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t3414 = v_x.clone(); Py::Bool({ let t3415 = Py::Int(0i64); if py::eq(&t3414, &t3415) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t3424 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t3424.truthy() { t3424 } else { { let t3423 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t3423.truthy() { t3423 } else { { let t3422 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t3422.truthy() { t3422 } else { Py::Bool(matches!(v_b.clone(), Py::Bool(_))) } } } } } }).truthy() {
        return Ok((if ({ let t3425 = v_a.clone(); Py::Bool({ let t3426 = v_b.clone(); if py::eq(&t3425, &t3426) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    if ({ let t3427 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t3427.truthy() { t3427 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t3431 = py::not(&{ let t3428 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t3428.truthy() { t3428 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t3431.truthy() { t3431 } else { { let t3429 = py::len(&v_a.clone())?; Py::Bool({ let t3430 = py::len(&v_b.clone())?; if py::ne(&t3429, &t3430) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok({ let t = { let mut out = vec![]; for t3432 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3433 = (t3432).items()?;
if t3433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3433[0].clone();
v_y = t3433[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    let t3435: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t3434 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
            if t3434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t3434[0].clone();
            v_b = t3434[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3435 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t3436 = v_d.clone(); Py::Bool({ let t3437 = Py::Int(0i64); if py::eq(&t3436, &t3437) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t3438 = v_s.clone(); Py::Bool({ let t3439 = Py::Int(0i64); if py::gt(&t3438, &t3439)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rot(mut v_name: Py, mut v_t: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3226 = (Py::tuple(vec![py::math1("cos", &py::truediv(&v_t.clone(), &Py::Int(2i64))?)?, py::math1("sin", &py::truediv(&v_t.clone(), &Py::Int(2i64))?)?])).items()?;
    if t3226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3226[0].clone();
    v_s = t3226[1].clone();
    if ({ let t3227 = v_name.clone(); Py::Bool({ let t3228 = Py::str("RX"); if py::eq(&t3227, &t3228) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::tuple(vec![v_c.clone(), py::mul(&py::neg(&Py::Complex(0.0, 1.0))?, &v_s.clone())?]), Py::tuple(vec![py::mul(&py::neg(&Py::Complex(0.0, 1.0))?, &v_s.clone())?, v_c.clone()])]));
    }
    if ({ let t3229 = v_name.clone(); Py::Bool({ let t3230 = Py::str("RY"); if py::eq(&t3229, &t3230) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::tuple(vec![v_c.clone(), py::neg(&v_s.clone())?]), Py::tuple(vec![v_s.clone(), v_c.clone()])]));
    }
    if ({ let t3231 = v_name.clone(); Py::Bool({ let t3232 = Py::str("RZ"); if py::eq(&t3231, &t3232) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::tuple(vec![py::cmath("exp", &[py::mul(&py::neg(&Py::Complex(0.0, 0.5))?, &v_t.clone())?])?, Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), py::cmath("exp", &[py::mul(&Py::Complex(0.0, 0.5), &v_t.clone())?])?])]));
    }
    return Ok(Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &v_t.clone())?])?])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__semf(mut v_Z: Py, mut v_A: Py, mut v_zz1: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_asym: Py = Py::None;
    let mut v_c3: Py = Py::None;
    let mut v_coul: Py = Py::None;
    let mut v_pair: Py = Py::None;
    let mut v_surf: Py = Py::None;
    let mut v_vol: Py = Py::None;
    let mut v_zs: Py = Py::None;
    let t2690 = (Py::tuple(vec![py::int(&v_Z.clone())?, py::int(&v_A.clone())?])).items()?;
    if t2690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t2690[0].clone();
    v_A = t2690[1].clone();
    if ({ let t2698 = { let t2691 = v_A.clone(); Py::Bool({ let t2692 = Py::Int(2i64); if py::lt(&t2691, &t2692)? { true } else { false } }) }; if t2698.truthy() { t2698 } else { { let t2697 = { let t2693 = v_Z.clone(); Py::Bool({ let t2694 = Py::Int(1i64); if py::lt(&t2693, &t2694)? { true } else { false } }) }; if t2697.truthy() { t2697 } else { { let t2695 = v_Z.clone(); Py::Bool({ let t2696 = v_A.clone(); if py::gt(&t2695, &t2696)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 <= Z <= A and A >= 2"))));
    }
    v_a = c_SEMF()?;
    v_N = py::sub(&v_A.clone(), &v_Z.clone())?;
    v_c3 = py::pow(&v_A.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    let t2699 = (Py::tuple(vec![py::mul(&py::getitem(&v_a.clone(), &Py::str("aV"))?, &v_A.clone())?, py::mul(&py::mul(&py::getitem(&v_a.clone(), &Py::str("aS"))?, &v_c3.clone())?, &v_c3.clone())?])).items()?;
    if t2699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vol = t2699[0].clone();
    v_surf = t2699[1].clone();
    v_coul = py::truediv(&py::mul(&py::getitem(&v_a.clone(), &Py::str("aC"))?, &(if (v_zz1.clone()).truthy() { py::mul(&v_Z.clone(), &py::sub(&v_Z.clone(), &Py::Int(1i64))?)? } else { py::mul(&v_Z.clone(), &v_Z.clone())? }))?, &v_c3.clone())?;
    v_asym = py::truediv(&py::mul(&py::getitem(&v_a.clone(), &Py::str("aA"))?, &py::pow(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_Z.clone())?)?, &Py::Int(2i64))?)?, &v_A.clone())?;
    v_pair = (if (py::modulo(&v_A.clone(), &Py::Int(2i64))?).truthy() { Py::Float(0.0) } else { (if ({ let t2700 = py::modulo(&v_Z.clone(), &Py::Int(2i64))?; Py::Bool({ let t2701 = Py::Int(0i64); if py::eq(&t2700, &t2701) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_a.clone(), &Py::str("aP"))?, &py::math1("sqrt", &v_A.clone())?)? } else { py::truediv(&py::neg(&py::getitem(&v_a.clone(), &Py::str("aP"))?)?, &py::math1("sqrt", &v_A.clone())?)? }) });
    v_B = py::add(&py::sub(&py::sub(&py::sub(&v_vol.clone(), &v_surf.clone())?, &v_coul.clone())?, &v_asym.clone())?, &v_pair.clone())?;
    v_zs = py::truediv(&py::add(&py::mul(&Py::Int(4i64), &py::getitem(&v_a.clone(), &Py::str("aA"))?)?, &py::truediv(&py::getitem(&v_a.clone(), &Py::str("aC"))?, &v_c3.clone())?)?, &py::add(&py::truediv(&py::mul(&Py::Int(8i64), &py::getitem(&v_a.clone(), &Py::str("aA"))?)?, &v_A.clone())?, &py::truediv(&py::mul(&Py::Int(2i64), &py::getitem(&v_a.clone(), &Py::str("aC"))?)?, &v_c3.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("B_MeV"), v_B.clone()), (Py::str("B_per_A_MeV"), py::truediv(&v_B.clone(), &v_A.clone())?), (Py::str("mass_u"), py::sub(&py::add(&py::mul(&v_Z.clone(), &c_M_H1_U()?)?, &py::mul(&v_N.clone(), &c_M_N_U()?)?)?, &py::truediv(&v_B.clone(), &c_AMU_MEV()?)?)?), (Py::str("volume_MeV"), v_vol.clone()), (Py::str("surface_MeV"), v_surf.clone()), (Py::str("coulomb_MeV"), v_coul.clone()), (Py::str("asymmetry_MeV"), v_asym.clone()), (Py::str("pairing_MeV"), v_pair.clone()), (Py::str("Z_stable"), v_zs.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_x: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t3270 = py::abs(&v_x.clone())?; Py::Bool({ let t3271 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t3270, &t3271)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__swap_labels(mut v_g: Py) -> R {
    if ({ let t3240 = py::getitem(&v_g.clone(), &Py::Int(0i64))?; Py::Bool({ let t3241 = Py::str("CNOT"); if py::eq(&t3240, &t3241) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("CNOT"), py::getitem(&v_g.clone(), &Py::Int(2i64))?, py::getitem(&v_g.clone(), &Py::Int(1i64))?]));
    }
    if ({ let t3242 = py::getitem(&v_g.clone(), &Py::Int(0i64))?; Py::Bool({ let t3243 = Py::tuple(vec![Py::str("CZ"), Py::str("SWAP")]); if py::contains(&t3243, &t3242)? { true } else { false } }) }).truthy() {
        return Ok(v_g.clone());
    }
    return Ok(py::add(&Py::tuple(vec![py::getitem(&v_g.clone(), &Py::Int(0i64))?, py::sub(&Py::Int(1i64), &py::getitem(&v_g.clone(), &Py::Int(1i64))?)?]), &Py::tuple((py::slice(&v_g.clone(), Some((Py::Int(2i64)).i()?), None, None)?).items()?))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__transition(mut v_Z: Py, mut v_n_upper: Py, mut v_n_lower: Py, mut v_mu_ratio: Py) -> R {
    let mut v_inv: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__quantum_n(v_Z.clone(), Py::str("Z"))?;
    let _ = f__quantum_n(v_n_lower.clone(), Py::str("n_lower"))?;
    if ({ let t1401 = v_n_upper.clone(); Py::Bool({ let t1402 = v_n_lower.clone(); if py::le(&t1401, &t1402)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the upper level must lie above the lower level (n_upper > n_lower)"))));
    }
    v_inv = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&v_Z.clone(), &v_Z.clone())?, &c_RINF()?)?, &v_mu_ratio.clone())?, &py::sub(&v_n_upper.clone(), &v_n_lower.clone())?)?, &py::add(&v_n_upper.clone(), &v_n_lower.clone())?)?, &py::mul(&py::mul(&py::mul(&v_n_upper.clone(), &v_n_upper.clone())?, &v_n_lower.clone())?, &v_n_lower.clone())?)?;
    v_lam = py::truediv(&Py::Int(1i64), &v_inv.clone())?;
    return Ok(py::dict_from(&Py::dict(vec![]), vec![(Py::str("E_photon_eV"), py::truediv(&py::mul(&py::mul(&c_H()?, &c_C()?)?, &v_inv.clone())?, &c_EV()?)?), (Py::str("lam"), v_lam.clone()), (Py::str("lambda_nm"), py::mul(&v_lam.clone(), &Py::Float(1000000000.0))?), (Py::str("f"), py::mul(&c_C()?, &v_inv.clone())?), (Py::str("series"), py::method(&c__SERIES()?, "get", &[v_n_lower.clone(), Py::str(&["ending at n = ".to_string(), py::format(&v_n_lower.clone(), "")?].concat())])?)])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec3(mut v_p: Py, mut v_name: Py) -> R {
    if ({ let t2183 = py::len(&v_p.clone())?; Py::Bool({ let t2184 = Py::Int(3i64); if py::ne(&t2183, &t2184) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs three components".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t2185 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t2185;
out.push(py::float(&v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xlog2(mut v_p: Py) -> R {
    return Ok((if ({ let t2156 = v_p.clone(); Py::Bool({ let t2157 = Py::Int(0i64); if py::le(&t2156, &t2157)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::neg(&v_p.clone())?, &py::math1("log2", &v_p.clone())?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_aberration(mut v_theta: Py, mut v_v: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c0: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_kx: Py = Py::None;
    let mut v_ky: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_th: Py = Py::None;
    if (py::not(&{ let t2410 = Py::Int(0i64); Py::Bool({ let t2411 = v_theta.clone(); if py::le(&t2410, &t2411)? { { let t2412 = Py::Float(std::f64::consts::PI); if py::le(&t2411, &t2412)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle must lie between 0 and 180 degrees"))));
    }
    v_b = f__beta(v_v.clone())?;
    let t2416 = (Py::tuple(vec![py::math1("cos", &v_theta.clone())?, py::math1("sin", &v_theta.clone())?])).items()?;
    if t2416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c0 = t2416[0].clone();
    v_s0 = t2416[1].clone();
    v_g = f__gamma_of_beta(v_b.clone())?;
    let t2417 = (Py::tuple(vec![py::mul(&v_g.clone(), &py::sub(&v_c0.clone(), &v_b.clone())?)?, v_s0.clone(), py::mul(&v_g.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_b.clone(), &v_c0.clone())?)?)?])).items()?;
    if t2417.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_kx = t2417[0].clone();
    v_ky = t2417[1].clone();
    v_E = t2417[2].clone();
    v_th = py::atan2(&v_ky.clone(), &v_kx.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_prime"), v_th.clone()), (Py::str("theta_prime_deg"), py::math1("degrees", &v_th.clone())?), (Py::str("doppler_factor"), v_E.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_band_fraction(mut v_lam1: Py, mut v_lam2: Py, mut v_T: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let mut v_x2: Py = Py::None;
    if ({ let t108 = py::not(&{ let t103 = Py::Int(0i64); Py::Bool({ let t104 = v_lam1.clone(); if py::lt(&t103, &t104)? { { let t105 = v_lam2.clone(); if py::lt(&t104, &t105)? { true } else { false } } } else { false } }) }); if t108.truthy() { t108 } else { { let t106 = v_T.clone(); Py::Bool({ let t107 = Py::Int(0i64); if py::le(&t106, &t107)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 0 < lambda1 < lambda2 and T > 0"))));
    }
    let t109 = (Py::tuple(vec![py::truediv(&py::mul(&c_H()?, &c_C()?)?, &py::mul(&py::mul(&v_lam1.clone(), &c_KB()?)?, &v_T.clone())?)?, py::truediv(&py::mul(&c_H()?, &c_C()?)?, &py::mul(&py::mul(&v_lam2.clone(), &c_KB()?)?, &v_T.clone())?)?])).items()?;
    if t109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x1 = t109[0].clone();
    v_x2 = t109[1].clone();
    let t122 = (Py::tuple(vec![f__planck_tail(v_x1.clone())?, f__planck_tail(v_x2.clone())?])).items()?;
    if t122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t122[0].clone();
    v_t2 = t122[1].clone();
    v_F = py::truediv(&py::sub(&v_t2.clone(), &v_t1.clone())?, &c__PI4_15()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("F_band"), v_F.clone()), (Py::str("F_below_lam1"), py::truediv(&v_t1.clone(), &c__PI4_15()?)?), (Py::str("F_below_lam2"), py::truediv(&v_t2.clone(), &c__PI4_15()?)?), (Py::str("M_band"), py::mul(&py::mul(&v_F.clone(), &c_SIGMA_SB()?)?, &py::pow(&v_T.clone(), &Py::Int(4i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_barrier_transmission(mut v_E: Py, mut v_V0: Py, mut v_a: Py, mut v_m: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_inv_B: Py = Py::None;
    let mut v_q2: Py = Py::None;
    let mut v_regime: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t11 = { let t1 = v_E.clone(); Py::Bool({ let t2 = Py::Int(0i64); if py::le(&t1, &t2)? { true } else { false } }) }; if t11.truthy() { t11 } else { { let t10 = { let t3 = v_V0.clone(); Py::Bool({ let t4 = Py::Int(0i64); if py::le(&t3, &t4)? { true } else { false } }) }; if t10.truthy() { t10 } else { { let t9 = { let t5 = v_a.clone(); Py::Bool({ let t6 = Py::Int(0i64); if py::le(&t5, &t6)? { true } else { false } }) }; if t9.truthy() { t9 } else { { let t7 = v_m.clone(); Py::Bool({ let t8 = Py::Int(0i64); if py::le(&t7, &t8)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the energy, barrier height, width and mass must be positive"))));
    }
    v_q2 = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &py::sub(&v_V0.clone(), &v_E.clone())?)?, &py::mul(&c_HBAR()?, &c_HBAR()?)?)?;
    v_x = py::mul(&py::math1("sqrt", &py::abs(&v_q2.clone())?)?, &v_a.clone())?;
    if ({ let t12 = v_x.clone(); Py::Bool({ let t13 = Py::Float(1e-06); if py::lt(&t12, &t13)? { true } else { false } }) }).truthy() {
        v_s = py::add(&Py::Int(1i64), &(if ({ let t14 = v_q2.clone(); Py::Bool({ let t15 = Py::Int(0i64); if py::gt(&t14, &t15)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(6i64))? } else { py::truediv(&py::mul(&py::neg(&v_x.clone())?, &v_x.clone())?, &Py::Int(6i64))? }))?;
    } else {
        v_s = (if ({ let t16 = v_q2.clone(); Py::Bool({ let t17 = Py::Int(0i64); if py::gt(&t16, &t17)? { true } else { false } }) }).truthy() { py::truediv(&py::math1("sinh", &v_x.clone())?, &v_x.clone())? } else { py::truediv(&py::math1("sin", &v_x.clone())?, &v_x.clone())? });
    }
    if ({ let t22 = { let t18 = v_q2.clone(); Py::Bool({ let t19 = Py::Int(0i64); if py::gt(&t18, &t19)? { true } else { false } }) }; if !t22.truthy() { t22 } else { { let t20 = v_x.clone(); Py::Bool({ let t21 = Py::Int(30i64); if py::gt(&t20, &t21)? { true } else { false } }) } } }).truthy() {
        v_inv_B = py::truediv(&py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(16i64), &v_E.clone())?, &py::sub(&v_V0.clone(), &v_E.clone())?)?, &py::mul(&v_V0.clone(), &v_V0.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&Py::Int(2i64))?, &v_x.clone())?)?)?, &py::pow(&py::sub(&Py::Int(1i64), &py::math1("exp", &py::mul(&py::neg(&Py::Int(2i64))?, &v_x.clone())?)?)?, &Py::Int(2i64))?)?;
        v_T = py::truediv(&v_inv_B.clone(), &py::add(&Py::Int(1i64), &v_inv_B.clone())?)?;
        v_R = py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_inv_B.clone())?)?;
    } else {
        v_B = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_V0.clone(), &v_V0.clone())?, &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_a.clone())?, &v_a.clone())?, &py::mul(&c_HBAR()?, &c_HBAR()?)?)?)?, &v_s.clone())?, &v_s.clone())?, &py::mul(&Py::Int(4i64), &v_E.clone())?)?;
        let t23 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_B.clone())?)?, py::truediv(&v_B.clone(), &py::add(&Py::Int(1i64), &v_B.clone())?)?])).items()?;
        if t23.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_T = t23[0].clone();
        v_R = t23[1].clone();
    }
    v_regime = (if ({ let t24 = v_E.clone(); Py::Bool({ let t25 = v_V0.clone(); if py::lt(&t24, &t25)? { true } else { false } }) }).truthy() { Py::str("tunnelling (E < V0)") } else { (if ({ let t26 = v_E.clone(); Py::Bool({ let t27 = v_V0.clone(); if py::eq(&t26, &t27) { true } else { false } }) }).truthy() { Py::str("E = V0") } else { Py::str("over the barrier (E > V0)") }) });
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("R"), v_R.clone()), (Py::str("kappa_a"), (if ({ let t37 = v_q2.clone(); Py::Bool({ let t38 = Py::Int(0i64); if py::gt(&t37, &t38)? { true } else { false } }) }).truthy() { v_x.clone() } else { Py::Float(0.0) })), (Py::str("regime"), v_regime.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bec_temperature(mut v_n: Py, mut v_m: Py) -> R {
    let mut v_Tc: Py = Py::None;
    if ({ let t68 = { let t64 = v_n.clone(); Py::Bool({ let t65 = Py::Int(0i64); if py::le(&t64, &t65)? { true } else { false } }) }; if t68.truthy() { t68 } else { { let t66 = v_m.clone(); Py::Bool({ let t67 = Py::Int(0i64); if py::le(&t66, &t67)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the density and the mass must be positive"))));
    }
    v_Tc = py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&c_HBAR()?, &Py::Int(2i64))?)?, &py::mul(&v_m.clone(), &c_KB()?)?)?, &py::pow(&py::truediv(&v_n.clone(), &c_ZETA32()?)?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_c"), v_Tc.clone()), (Py::str("lambda_c"), py::pow(&py::truediv(&c_ZETA32()?, &v_n.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?), (Py::str("n_lambda3"), c_ZETA32()?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bohr_level(mut v_Z: Py, mut v_n: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__quantum_n(v_Z.clone(), Py::str("Z"))?;
    let _ = f__quantum_n(v_n.clone(), Py::str("n"))?;
    v_E = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::neg(&v_Z.clone())?, &v_Z.clone())?, &c_RINF()?)?, &c_H()?)?, &c_C()?)?, &py::mul(&v_n.clone(), &v_n.clone())?)?;
    v_r = py::truediv(&py::mul(&py::mul(&v_n.clone(), &v_n.clone())?, &c_A0()?)?, &v_Z.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_eV"), py::truediv(&v_E.clone(), &c_EV()?)?), (Py::str("E"), v_E.clone()), (Py::str("ionization_eV"), py::truediv(&py::neg(&v_E.clone())?, &c_EV()?)?), (Py::str("r"), v_r.clone()), (Py::str("r_nm"), py::mul(&v_r.clone(), &Py::Float(1000000000.0))?), (Py::str("v"), py::truediv(&py::mul(&py::mul(&v_Z.clone(), &c_ALPHA()?)?, &c_C()?)?, &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bose_einstein(mut v_E: Py, mut v_mu: Py, mut v_T: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t193 = v_T.clone(); Py::Bool({ let t194 = Py::Int(0i64); if py::le(&t193, &t194)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature must be positive"))));
    }
    v_x = py::truediv(&py::sub(&v_E.clone(), &v_mu.clone())?, &py::mul(&c_KB()?, &v_T.clone())?)?;
    if ({ let t195 = v_x.clone(); Py::Bool({ let t196 = Py::Int(0i64); if py::le(&t195, &t196)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a Bose-Einstein occupation needs E > mu"))));
    }
    v_n = (if ({ let t197 = v_x.clone(); Py::Bool({ let t198 = Py::Int(700i64); if py::lt(&t197, &t198)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::math1("expm1", &v_x.clone())?)? } else { py::math1("exp", &py::neg(&v_x.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("x"), v_x.clone()), (Py::str("classical_limit"), py::math1("exp", &py::neg(&v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chsh(mut v_a: Py, mut v_a2: Py, mut v_b: Py, mut v_b2: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_E = { let v_a = v_a.clone(); let v_a2 = v_a2.clone(); let v_b = v_b.clone(); let v_b2 = v_b2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::neg(&py::math1("cos", &py::sub(&v_x.clone(), &v_y.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_vals = Py::list(vec![py::call(&v_E.clone(), &[v_a.clone(), v_b.clone()])?, py::call(&v_E.clone(), &[v_a.clone(), v_b2.clone()])?, py::call(&v_E.clone(), &[v_a2.clone(), v_b.clone()])?, py::call(&v_E.clone(), &[v_a2.clone(), v_b2.clone()])?]);
    v_S = py::add(&py::add(&py::sub(&py::getitem(&v_vals.clone(), &Py::Int(0i64))?, &py::getitem(&v_vals.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_vals.clone(), &Py::Int(2i64))?)?, &py::getitem(&v_vals.clone(), &Py::Int(3i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), f__snap(v_S.clone(), Py::Float(1.0))?), (Py::str("abs_S"), f__snap(py::abs(&v_S.clone())?, Py::Float(1.0))?), (Py::str("E_ab"), f__snap(py::getitem(&v_vals.clone(), &Py::Int(0i64))?, Py::Float(1.0))?), (Py::str("E_ab2"), f__snap(py::getitem(&v_vals.clone(), &Py::Int(1i64))?, Py::Float(1.0))?), (Py::str("E_a2b"), f__snap(py::getitem(&v_vals.clone(), &Py::Int(2i64))?, Py::Float(1.0))?), (Py::str("E_a2b2"), f__snap(py::getitem(&v_vals.clone(), &Py::Int(3i64))?, Py::Float(1.0))?), (Py::str("violates"), (if ({ let t244 = py::abs(&v_S.clone())?; Py::Bool({ let t245 = py::add(&Py::Int(2i64), &Py::Float(1e-09))?; if py::gt(&t244, &t245)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_clebsch_gordan(mut v_j1: Py, mut v_m1: Py, mut v_j2: Py, mut v_m2: Py, mut v_J: Py, mut v_M: Py) -> R {
    let mut v_sign: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t305 = (f__cg_exact(v_j1.clone(), v_m1.clone(), v_j2.clone(), v_m2.clone(), v_J.clone(), v_M.clone())?).items()?;
    if t305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sign = t305[0].clone();
    v_sq = t305[1].clone();
    v_v = (if (v_sq.clone()).truthy() { py::mul(&v_sign.clone(), &py::math1("sqrt", &v_sq.clone())?)? } else { Py::Float(0.0) });
    return Ok(Py::dict(vec![(Py::str("cg"), f__r(v_v.clone())?), (Py::str("cg_squared"), f__r(py::float(&v_sq.clone())?)?), (Py::str("cg_squared_exact"), (if ({ let t313 = py::denominator(&v_sq.clone())?; Py::Bool({ let t314 = Py::Int(1i64); if py::ne(&t313, &t314) { true } else { false } }) }).truthy() { Py::str(&[py::format(&py::numerator(&v_sq.clone())?, "")?, "/".to_string(), py::format(&py::denominator(&v_sq.clone())?, "")?].concat()) } else { Py::str(&py::to_str(&py::numerator(&v_sq.clone())?)) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coherent_state(mut v_alpha: Py, mut v_n: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_a2: Py = Py::None;
    if ({ let t340 = v_alpha.clone(); Py::Bool({ let t341 = Py::Int(0i64); if py::lt(&t340, &t341)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give |alpha| (a non-negative number)"))));
    }
    if ({ let t342 = v_n.clone(); Py::Bool({ let t343 = Py::Int(0i64); if py::lt(&t342, &t343)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number >= 0"))));
    }
    v_a2 = py::mul(&v_alpha.clone(), &v_alpha.clone())?;
    if ({ let t344 = v_alpha.clone(); Py::Bool({ let t345 = Py::Int(0i64); if py::eq(&t344, &t345) { true } else { false } }) }).truthy() {
        v_P = (if ({ let t346 = v_n.clone(); Py::Bool({ let t347 = Py::Int(0i64); if py::eq(&t346, &t347) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) });
    } else {
        v_P = py::math1("exp", &py::sub(&py::add(&py::neg(&v_a2.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_n.clone())?, &py::math1("log", &v_alpha.clone())?)?)?, &py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("P_n"), v_P.clone()), (Py::str("mean_n"), v_a2.clone()), (Py::str("variance"), v_a2.clone()), (Py::str("P_0"), py::math1("exp", &py::neg(&v_a2.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compton(mut v_lam: Py, mut v_theta: Py) -> R {
    let mut v_E0: Py = Py::None;
    let mut v_E1: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_lc: Py = Py::None;
    let mut v_lp: Py = Py::None;
    if ({ let t382 = v_lam.clone(); Py::Bool({ let t383 = Py::Int(0i64); if py::le(&t382, &t383)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wavelength must be positive"))));
    }
    if (py::not(&{ let t384 = Py::Int(0i64); Py::Bool({ let t385 = v_theta.clone(); if py::le(&t384, &t385)? { { let t386 = Py::Float(std::f64::consts::PI); if py::le(&t385, &t386)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the scattering angle must lie between 0 and 180 degrees"))));
    }
    v_lc = py::truediv(&c_H()?, &py::mul(&c_ME()?, &c_C()?)?)?;
    v_d = py::mul(&py::mul(&Py::Int(2i64), &v_lc.clone())?, &py::pow(&py::math1("sin", &py::truediv(&v_theta.clone(), &Py::Int(2i64))?)?, &Py::Int(2i64))?)?;
    v_lp = py::add(&v_lam.clone(), &v_d.clone())?;
    let t387 = (Py::tuple(vec![py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_lam.clone())?, py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_lp.clone())?])).items()?;
    if t387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_E0 = t387[0].clone();
    v_E1 = t387[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_prime"), v_lp.clone()), (Py::str("lambda_prime_pm"), py::mul(&v_lp.clone(), &Py::Float(1000000000000.0))?), (Py::str("shift_pm"), py::mul(&v_d.clone(), &Py::Float(1000000000000.0))?), (Py::str("E_in_keV"), py::truediv(&v_E0.clone(), &py::mul(&Py::Float(1000.0), &c_EV()?)?)?), (Py::str("E_out_keV"), py::truediv(&v_E1.clone(), &py::mul(&Py::Float(1000.0), &c_EV()?)?)?), (Py::str("K_electron_keV"), py::truediv(&py::truediv(&py::mul(&v_E0.clone(), &v_d.clone())?, &v_lp.clone())?, &py::mul(&Py::Float(1000.0), &c_EV()?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coulomb_barrier(mut v_Z1: Py, mut v_A1: Py, mut v_Z2: Py, mut v_A2: Py, mut v_r0: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_hbarc: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t422 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_Z1.clone(), Py::str("Z1")]), Py::tuple(vec![v_Z2.clone(), Py::str("Z2")]), Py::tuple(vec![v_A1.clone(), Py::str("A1")]), Py::tuple(vec![v_A2.clone(), Py::str("A2")])])))? {
        let t425 = (t422).items()?;
        if t425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t425[0].clone();
        v_n = t425[1].clone();
        if ({ let t423 = v_v.clone(); Py::Bool({ let t424 = Py::Int(1i64); if py::lt(&t423, &t424)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_n.clone(), "")?, " must be a positive whole number".to_string()].concat()))));
        }
    }
    if ({ let t430 = { let t426 = v_Z1.clone(); Py::Bool({ let t427 = v_A1.clone(); if py::gt(&t426, &t427)? { true } else { false } }) }; if t430.truthy() { t430 } else { { let t428 = v_Z2.clone(); Py::Bool({ let t429 = v_A2.clone(); if py::gt(&t428, &t429)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Z cannot exceed A"))));
    }
    if ({ let t431 = v_r0.clone(); Py::Bool({ let t432 = Py::Int(0i64); if py::le(&t431, &t432)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the radius parameter must be positive"))));
    }
    v_R = py::mul(&v_r0.clone(), &py::add(&py::pow(&v_A1.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?, &py::pow(&v_A2.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?)?;
    v_hbarc = py::mul(&c_HBAR()?, &c_C()?)?;
    v_V = py::truediv(&py::mul(&py::mul(&py::mul(&v_Z1.clone(), &v_Z2.clone())?, &c_ALPHA()?)?, &v_hbarc.clone())?, &v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_MeV"), py::truediv(&v_V.clone(), &c_MEV()?)?), (Py::str("R"), v_R.clone()), (Py::str("R_fm"), py::mul(&v_R.clone(), &Py::Float(1000000000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_de_broglie_mv(mut v_m: Py, mut v_v: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pr: Py = Py::None;
    if (py::not(&{ let t507 = { let t505 = v_m.clone(); Py::Bool({ let t506 = Py::Int(0i64); if py::gt(&t505, &t506)? { true } else { false } }) }; if !t507.truthy() { t507 } else { py::math1("isfinite", &v_m.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass must be positive"))));
    }
    if (py::not(&{ let t508 = v_v.clone(); Py::Bool({ let t509 = Py::Int(0i64); if py::gt(&t508, &t509)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the speed must be positive"))));
    }
    v_b = f__beta(v_v.clone())?;
    v_p = py::mul(&v_m.clone(), &v_v.clone())?;
    v_pr = py::mul(&v_p.clone(), &f__gamma_of_beta(v_b.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("lam"), py::truediv(&c_H()?, &v_p.clone())?), (Py::str("lambda_nm"), py::mul(&py::truediv(&c_H()?, &v_p.clone())?, &Py::Float(1000000000.0))?), (Py::str("lambda_rel_nm"), py::mul(&py::truediv(&c_H()?, &v_pr.clone())?, &Py::Float(1000000000.0))?), (Py::str("inv_lambda"), py::truediv(&v_p.clone(), &c_H()?)?), (Py::str("beta"), v_b.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_de_broglie_voltage(mut v_V: Py, mut v_particle: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p_nr: Py = Py::None;
    let mut v_p_r: Py = Py::None;
    if ({ let t467 = v_V.clone(); Py::Bool({ let t468 = Py::Int(0i64); if py::le(&t467, &t468)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the accelerating voltage must be positive"))));
    }
    v_m = (if ({ let t469 = v_particle.clone(); Py::Bool({ let t470 = Py::str("electron"); if py::eq(&t469, &t470) { true } else { false } }) }).truthy() { c_ME()? } else { c_MP()? });
    v_K = py::mul(&c_QE()?, &v_V.clone())?;
    v_p_nr = py::math1("sqrt", &py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_K.clone())?)?;
    v_p_r = py::truediv(&py::math1("sqrt", &py::mul(&v_K.clone(), &py::add(&v_K.clone(), &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &c_C()?)?, &c_C()?)?)?)?)?, &c_C()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_nonrel"), py::truediv(&c_H()?, &v_p_nr.clone())?), (Py::str("lambda_nonrel_nm"), py::mul(&py::truediv(&c_H()?, &v_p_nr.clone())?, &Py::Float(1000000000.0))?), (Py::str("lambda_rel"), py::truediv(&c_H()?, &v_p_r.clone())?), (Py::str("lambda_rel_nm"), py::mul(&py::truediv(&c_H()?, &v_p_r.clone())?, &Py::Float(1000000000.0))?), (Py::str("p"), v_p_r.clone()), (Py::str("K_eV"), v_V.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_debye(mut v_T: Py, mut v_theta_D: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_xd: Py = Py::None;
    if ({ let t551 = { let t547 = v_T.clone(); Py::Bool({ let t548 = Py::Int(0i64); if py::le(&t547, &t548)? { true } else { false } }) }; if t551.truthy() { t551 } else { { let t549 = v_theta_D.clone(); Py::Bool({ let t550 = Py::Int(0i64); if py::le(&t549, &t550)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature and the Debye temperature must be positive"))));
    }
    v_xd = py::truediv(&v_theta_D.clone(), &v_T.clone())?;
    if ({ let t552 = v_xd.clone(); Py::Bool({ let t553 = Py::Int(600i64); if py::gt(&t552, &t553)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T is below theta_D/600: use the T^3 law (the integral equals 4 pi^4/15 to double precision)"))));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
        v_I = ax_engine::pymp::call("quad", &[{ let v_T = v_T.clone(); let v_theta_D = v_theta_D.clone(); let v_xd = v_xd.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t554 = v_x.clone(); Py::Bool({ let t555 = Py::Int(0i64); if py::ne(&t554, &t555) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::pow(&v_x.clone(), &Py::Int(4i64))?, &ax_engine::pymp::call("exp", &[v_x.clone()], &[])?)?, &py::pow(&ax_engine::pymp::call("expm1", &[v_x.clone()], &[])?, &Py::Int(2i64))?)? } else { ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![Py::Int(0i64), py::minmax(&[v_xd.clone(), Py::Int(1i64)], false)?, v_xd.clone()])], &[])?;
        v_I = py::float(&v_I.clone())?;
    }
    v_c = py::mul(&py::mul(&Py::Int(3i64), &py::pow(&py::truediv(&v_T.clone(), &v_theta_D.clone())?, &Py::Int(3i64))?)?, &v_I.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_molar"), py::mul(&py::mul(&Py::Int(3i64), &c_RGAS()?)?, &v_c.clone())?), (Py::str("C_over_3R"), v_c.clone()), (Py::str("debye_integral"), v_I.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_age(mut v_fraction: Py, mut v_T_half: Py) -> R {
    let mut v_t: Py = Py::None;
    if ({ let t2331 = { let t2326 = v_T_half.clone(); Py::Bool({ let t2327 = Py::Int(0i64); if py::le(&t2326, &t2327)? { true } else { false } }) }; if t2331.truthy() { t2331 } else { py::not(&{ let t2328 = Py::Int(0i64); Py::Bool({ let t2329 = v_fraction.clone(); if py::lt(&t2328, &t2329)? { { let t2330 = Py::Int(1i64); if py::le(&t2329, &t2330)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the remaining fraction must lie in (0, 1] and the half-life must be positive"))));
    }
    v_t = py::mul(&py::neg(&v_T_half.clone())?, &py::math1("log2", &v_fraction.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), f__snap(v_t.clone(), v_T_half.clone())?), (Py::str("half_lives"), f__snap(py::truediv(&v_t.clone(), &v_T_half.clone())?, Py::Float(1.0))?), (Py::str("decay_constant"), py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T_half.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_chain(mut v_T_half_P: Py, mut v_T_half_D: Py, mut v_N0: Py, mut v_t: Py) -> R {
    let mut v_AD: Py = Py::None;
    let mut v_AP: Py = Py::None;
    let mut v_ND: Py = Py::None;
    let mut v_NP: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_phi: Py = Py::None;
    if ({ let t600 = { let t590 = v_T_half_P.clone(); Py::Bool({ let t591 = Py::Int(0i64); if py::le(&t590, &t591)? { true } else { false } }) }; if t600.truthy() { t600 } else { { let t599 = { let t592 = v_T_half_D.clone(); Py::Bool({ let t593 = Py::Int(0i64); if py::le(&t592, &t593)? { true } else { false } }) }; if t599.truthy() { t599 } else { { let t598 = { let t594 = v_N0.clone(); Py::Bool({ let t595 = Py::Int(0i64); if py::le(&t594, &t595)? { true } else { false } }) }; if t598.truthy() { t598 } else { { let t596 = v_t.clone(); Py::Bool({ let t597 = Py::Int(0i64); if py::lt(&t596, &t597)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("half-lives and N0 must be positive and the time non-negative"))));
    }
    let t601 = (Py::tuple(vec![py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T_half_P.clone())?, py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T_half_D.clone())?])).items()?;
    if t601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lp = t601[0].clone();
    v_ld = t601[1].clone();
    v_NP = py::mul(&v_N0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_lp.clone())?, &v_t.clone())?)?)?;
    v_d = py::mul(&py::sub(&v_ld.clone(), &v_lp.clone())?, &v_t.clone())?;
    if ({ let t602 = py::abs(&v_d.clone())?; Py::Bool({ let t603 = Py::Int(1i64); if py::gt(&t602, &t603)? { true } else { false } }) }).truthy() {
        v_ND = py::mul(&py::truediv(&py::mul(&v_N0.clone(), &v_lp.clone())?, &py::sub(&v_ld.clone(), &v_lp.clone())?)?, &py::sub(&py::math1("exp", &py::mul(&py::neg(&v_lp.clone())?, &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_ld.clone())?, &v_t.clone())?)?)?)?;
    } else {
        v_phi = (if ({ let t604 = py::abs(&v_d.clone())?; Py::Bool({ let t605 = Py::Float(1e-05); if py::lt(&t604, &t605)? { true } else { false } }) }).truthy() { py::sub(&py::add(&py::sub(&Py::Int(1i64), &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?, &py::truediv(&py::mul(&v_d.clone(), &v_d.clone())?, &Py::Int(6i64))?)?, &py::truediv(&py::pow(&v_d.clone(), &Py::Int(3i64))?, &Py::Int(24i64))?)? } else { py::truediv(&py::neg(&py::math1("expm1", &py::neg(&v_d.clone())?)?)?, &v_d.clone())? });
        v_ND = py::mul(&py::mul(&py::mul(&py::mul(&v_N0.clone(), &v_lp.clone())?, &v_t.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_lp.clone())?, &v_t.clone())?)?)?, &v_phi.clone())?;
    }
    let t606 = (Py::tuple(vec![py::mul(&v_lp.clone(), &v_NP.clone())?, py::mul(&v_ld.clone(), &v_ND.clone())?])).items()?;
    if t606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_AP = t606[0].clone();
    v_AD = t606[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("N_parent"), v_NP.clone()), (Py::str("N_daughter"), f__snap(v_ND.clone(), v_N0.clone())?), (Py::str("A_parent"), v_AP.clone()), (Py::str("A_daughter"), f__snap(v_AD.clone(), py::add(&py::mul(&v_lp.clone(), &v_N0.clone())?, &py::mul(&v_ld.clone(), &v_N0.clone())?)?)?), (Py::str("activity_ratio"), f__snap(py::truediv(&v_AD.clone(), &v_AP.clone())?, Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_law(mut v_T_half: Py, mut v_N0: Py, mut v_t: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_lam: Py = Py::None;
    if ({ let t2375 = { let t2368 = v_T_half.clone(); Py::Bool({ let t2369 = Py::Int(0i64); if py::le(&t2368, &t2369)? { true } else { false } }) }; if t2375.truthy() { t2375 } else { { let t2374 = { let t2370 = v_N0.clone(); Py::Bool({ let t2371 = Py::Int(0i64); if py::le(&t2370, &t2371)? { true } else { false } }) }; if t2374.truthy() { t2374 } else { { let t2372 = v_t.clone(); Py::Bool({ let t2373 = Py::Int(0i64); if py::lt(&t2372, &t2373)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the half-life and N0 must be positive and the time non-negative"))));
    }
    v_lam = py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T_half.clone())?;
    v_f = py::pow(&Py::Float(2.0), &py::truediv(&py::neg(&v_t.clone())?, &v_T_half.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), py::mul(&v_N0.clone(), &v_f.clone())?), (Py::str("fraction"), v_f.clone()), (Py::str("decay_constant"), v_lam.clone()), (Py::str("mean_life"), py::truediv(&Py::Int(1i64), &v_lam.clone())?), (Py::str("A0"), py::mul(&v_lam.clone(), &v_N0.clone())?), (Py::str("A"), py::mul(&py::mul(&v_lam.clone(), &v_N0.clone())?, &v_f.clone())?), (Py::str("half_lives"), py::truediv(&v_t.clone(), &v_T_half.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_length(mut v_particle: Py, mut v_p: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_bg: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_tau_ns: Py = Py::None;
    if ({ let t1946 = v_p.clone(); Py::Bool({ let t1947 = Py::Int(0i64); if py::lt(&t1946, &t1947)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the momentum must be non-negative"))));
    }
    let t1948 = (py::getitem(&c__PARTICLES()?, &v_particle.clone())?).items()?;
    if t1948.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__ = t1948[0].clone();
    v_m = t1948[1].clone();
    v_tau_ns = t1948[2].clone();
    v_pm = py::mul(&v_p.clone(), &Py::Float(1000.0))?;
    v_bg = py::truediv(&v_pm.clone(), &v_m.clone())?;
    v_g = py::hypot(&[Py::Float(1.0), v_bg.clone()])?;
    v_tau = py::mul(&v_tau_ns.clone(), &Py::Float(1e-09))?;
    return Ok(f__out(Py::dict(vec![(Py::str("gamma"), v_g.clone()), (Py::str("beta"), py::truediv(&v_bg.clone(), &v_g.clone())?), (Py::str("beta_gamma"), v_bg.clone()), (Py::str("lifetime_lab"), py::mul(&v_g.clone(), &v_tau.clone())?), (Py::str("mean_decay_length"), py::mul(&py::mul(&v_bg.clone(), &c_C()?)?, &v_tau.clone())?), (Py::str("proper_lifetime"), v_tau.clone()), (Py::str("mass_MeV"), v_m.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_einstein_oscillator(mut v_theta_E: Py, mut v_T: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_em: Py = Py::None;
    let mut v_nbar: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t647 = { let t643 = v_theta_E.clone(); Py::Bool({ let t644 = Py::Int(0i64); if py::le(&t643, &t644)? { true } else { false } }) }; if t647.truthy() { t647 } else { { let t645 = v_T.clone(); Py::Bool({ let t646 = Py::Int(0i64); if py::le(&t645, &t646)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Einstein temperature and the temperature must be positive"))));
    }
    v_x = py::truediv(&v_theta_E.clone(), &v_T.clone())?;
    v_em = py::math1("expm1", &v_x.clone())?;
    v_nbar = py::truediv(&Py::Int(1i64), &v_em.clone())?;
    v_c = (if ({ let t648 = v_x.clone(); Py::Bool({ let t649 = Py::Int(300i64); if py::lt(&t648, &t649)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&v_x.clone(), &v_x.clone())?, &py::math1("exp", &v_x.clone())?)?, &py::mul(&v_em.clone(), &v_em.clone())?)? } else { py::mul(&py::mul(&v_x.clone(), &v_x.clone())?, &py::math1("exp", &py::neg(&v_x.clone())?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("n_mean"), v_nbar.clone()), (Py::str("U_eV"), py::truediv(&py::mul(&py::mul(&c_KB()?, &v_theta_E.clone())?, &py::add(&v_nbar.clone(), &Py::Float(0.5))?)?, &c_EV()?)?), (Py::str("C_over_k"), v_c.clone()), (Py::str("C_molar"), py::mul(&py::mul(&Py::Int(3i64), &c_RGAS()?)?, &v_c.clone())?), (Py::str("S_over_k"), py::sub(&py::mul(&v_x.clone(), &v_nbar.clone())?, &py::math1("log", &py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_energy_from_momentum(mut v_m: Py, mut v_p: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_E0: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_pc: Py = Py::None;
    if ({ let t2495 = v_m.clone(); Py::Bool({ let t2496 = Py::Int(0i64); if py::le(&t2495, &t2496)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass must be positive"))));
    }
    if ({ let t2497 = v_p.clone(); Py::Bool({ let t2498 = Py::Int(0i64); if py::lt(&t2497, &t2498)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the momentum as a magnitude (p >= 0)"))));
    }
    v_E0 = py::mul(&py::mul(&v_m.clone(), &c_C()?)?, &c_C()?)?;
    v_pc = py::mul(&v_p.clone(), &c_C()?)?;
    v_E = py::hypot(&[v_pc.clone(), v_E0.clone()])?;
    v_K = py::truediv(&py::mul(&v_pc.clone(), &v_pc.clone())?, &py::add(&v_E.clone(), &v_E0.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("K"), v_K.clone()), (Py::str("v"), py::mul(&py::truediv(&v_pc.clone(), &v_E.clone())?, &c_C()?)?), (Py::str("beta"), py::truediv(&v_pc.clone(), &v_E.clone())?), (Py::str("gamma"), py::truediv(&v_E.clone(), &v_E0.clone())?), (Py::str("E_MeV"), py::truediv(&v_E.clone(), &c_MEV()?)?), (Py::str("K_MeV"), py::truediv(&v_K.clone(), &c_MEV()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fermi_chemical_potential(mut v_n: Py, mut v_T: Py, mut v_m: Py) -> R {
    let mut v_EF: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_fv: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_y: Py = Py::None;
    if ({ let t811 = { let t804 = v_n.clone(); Py::Bool({ let t805 = Py::Int(0i64); if py::le(&t804, &t805)? { true } else { false } }) }; if t811.truthy() { t811 } else { { let t810 = { let t806 = v_T.clone(); Py::Bool({ let t807 = Py::Int(0i64); if py::le(&t806, &t807)? { true } else { false } }) }; if t810.truthy() { t810 } else { { let t808 = v_m.clone(); Py::Bool({ let t809 = Py::Int(0i64); if py::le(&t808, &t809)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("density, temperature and mass must be positive"))));
    }
    v_lam = py::truediv(&c_H()?, &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &c_KB()?)?, &v_T.clone())?)?)?;
    v_y = py::truediv(&py::mul(&v_n.clone(), &py::pow(&v_lam.clone(), &Py::Int(3i64))?)?, &Py::Int(2i64))?;
    v_EF = py::truediv(&py::mul(&py::pow(&c_HBAR()?, &Py::Int(2i64))?, &py::pow(&py::mul(&py::mul(&Py::Int(3i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_n.clone())?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?, &py::mul(&Py::Int(2i64), &v_m.clone())?)?;
    if ({ let t812 = py::mul(&c_KB()?, &v_T.clone())?; Py::Bool({ let t813 = py::mul(&Py::Float(0.02), &v_EF.clone())?; if py::lt(&t812, &t813)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T is below 0.02 T_F: the gas is fully degenerate (mu = E_F to within the Sommerfeld correction); give a higher temperature"))));
    }
    v_Y = py::truediv(&py::mul(&v_y.clone(), &py::math1("sqrt", &Py::Float(std::f64::consts::PI))?)?, &Py::Int(2i64))?;
    let t814 = (Py::tuple(vec![py::neg(&Py::Float(60.0))?, Py::Float(60.0)])).items()?;
    if t814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t814[0].clone();
    v_hi = t814[1].clone();
    v_eta = (if ({ let t815 = v_y.clone(); Py::Bool({ let t816 = Py::Int(1i64); if py::lt(&t815, &t816)? { true } else { false } }) }).truthy() { py::math1("log", &v_y.clone())? } else { py::pow(&py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &py::math1("sqrt", &Py::Float(std::f64::consts::PI))?)?, &v_y.clone())?, &Py::Int(4i64))?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)? });
    v_eta = py::minmax(&[py::minmax(&[v_eta.clone(), py::add(&v_lo.clone(), &Py::Int(1i64))?], true)?, py::sub(&v_hi.clone(), &Py::Int(1i64))?], false)?;
    for t817 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t817;
        let t825 = (f__fd_half(v_eta.clone())?).items()?;
        if t825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_I = t825[0].clone();
        v_dI = t825[1].clone();
        v_fv = py::sub(&v_I.clone(), &v_Y.clone())?;
        if ({ let t826 = v_fv.clone(); Py::Bool({ let t827 = Py::Int(0i64); if py::gt(&t826, &t827)? { true } else { false } }) }).truthy() {
            v_hi = py::minmax(&[v_hi.clone(), v_eta.clone()], false)?;
        } else {
            v_lo = py::minmax(&[v_lo.clone(), v_eta.clone()], true)?;
        }
        v_new = (if ({ let t828 = v_dI.clone(); Py::Bool({ let t829 = Py::Int(0i64); if py::gt(&t828, &t829)? { true } else { false } }) }).truthy() { py::sub(&v_eta.clone(), &py::truediv(&v_fv.clone(), &v_dI.clone())?)? } else { py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))? });
        if (py::not(&{ let t830 = v_lo.clone(); Py::Bool({ let t831 = v_new.clone(); if py::lt(&t830, &t831)? { { let t832 = v_hi.clone(); if py::lt(&t831, &t832)? { true } else { false } } } else { false } }) })).truthy() {
            v_new = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        }
        if ({ let t837 = { let t833 = py::abs(&py::sub(&v_new.clone(), &v_eta.clone())?)?; Py::Bool({ let t834 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&v_new.clone())?], true)?)?; if py::le(&t833, &t834)? { true } else { false } }) }; if t837.truthy() { t837 } else { { let t835 = py::sub(&v_hi.clone(), &v_lo.clone())?; Py::Bool({ let t836 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&v_new.clone())?], true)?)?; if py::le(&t835, &t836)? { true } else { false } }) } } }).truthy() {
            v_eta = v_new.clone();
            break;
        }
        v_eta = v_new.clone();
    }
    if (py::not(&{ let t838 = py::neg(&Py::Float(59.0))?; Py::Bool({ let t839 = v_eta.clone(); if py::lt(&t838, &t839)? { { let t840 = Py::Float(59.0); if py::lt(&t839, &t840)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("mu/kT lies outside -59 ... 59: the gas is either classical (use the Boltzmann limit) or fully degenerate"))));
    }
    v_mu = py::mul(&py::mul(&v_eta.clone(), &c_KB()?)?, &v_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("mu_eV"), py::truediv(&v_mu.clone(), &c_EV()?)?), (Py::str("eta"), v_eta.clone()), (Py::str("E_F_eV"), py::truediv(&v_EF.clone(), &c_EV()?)?), (Py::str("mu_over_EF"), py::truediv(&v_mu.clone(), &v_EF.clone())?), (Py::str("T_over_TF"), py::truediv(&py::mul(&c_KB()?, &v_T.clone())?, &v_EF.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fermi_dirac(mut v_E: Py, mut v_mu: Py, mut v_T: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t684 = v_T.clone(); Py::Bool({ let t685 = Py::Int(0i64); if py::le(&t684, &t685)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature must be positive"))));
    }
    v_x = py::truediv(&py::sub(&v_E.clone(), &v_mu.clone())?, &py::mul(&c_KB()?, &v_T.clone())?)?;
    if ({ let t686 = v_x.clone(); Py::Bool({ let t687 = Py::Int(0i64); if py::gt(&t686, &t687)? { true } else { false } }) }).truthy() {
        v_e = py::math1("exp", &py::neg(&v_x.clone())?)?;
        let t688 = (Py::tuple(vec![py::truediv(&v_e.clone(), &py::add(&Py::Int(1i64), &v_e.clone())?)?, py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_e.clone())?)?])).items()?;
        if t688.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t688[0].clone();
        v_g = t688[1].clone();
    } else {
        v_e = py::math1("exp", &v_x.clone())?;
        let t689 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_e.clone())?)?, py::truediv(&v_e.clone(), &py::add(&Py::Int(1i64), &v_e.clone())?)?])).items()?;
        if t689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t689[0].clone();
        v_g = t689[1].clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("f"), v_f.clone()), (Py::str("one_minus_f"), v_g.clone()), (Py::str("x"), v_x.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fermi_energy(mut v_n: Py) -> R {
    let mut v_EF: Py = Py::None;
    let mut v_kF: Py = Py::None;
    if ({ let t724 = v_n.clone(); Py::Bool({ let t725 = Py::Int(0i64); if py::le(&t724, &t725)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the electron density must be positive"))));
    }
    let t726 = (f__fermi_from_n(v_n.clone())?).items()?;
    if t726.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kF = t726[0].clone();
    v_EF = t726[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("E_F_eV"), py::truediv(&v_EF.clone(), &c_EV()?)?), (Py::str("E_F"), v_EF.clone()), (Py::str("T_F"), py::truediv(&v_EF.clone(), &c_KB()?)?), (Py::str("k_F"), v_kF.clone()), (Py::str("v_F"), py::truediv(&py::mul(&c_HBAR()?, &v_kF.clone())?, &c_ME()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fermi_energy_metal(mut v_rho: Py, mut v_M: Py, mut v_z: Py) -> R {
    let mut v_EF: Py = Py::None;
    let mut v_kF: Py = Py::None;
    let mut v_n: Py = Py::None;
    if ({ let t768 = { let t761 = v_rho.clone(); Py::Bool({ let t762 = Py::Int(0i64); if py::le(&t761, &t762)? { true } else { false } }) }; if t768.truthy() { t768 } else { { let t767 = { let t763 = v_M.clone(); Py::Bool({ let t764 = Py::Int(0i64); if py::le(&t763, &t764)? { true } else { false } }) }; if t767.truthy() { t767 } else { { let t765 = v_z.clone(); Py::Bool({ let t766 = Py::Int(1i64); if py::lt(&t765, &t766)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("density and molar mass must be positive and the valence at least 1"))));
    }
    v_n = py::truediv(&py::mul(&py::mul(&v_z.clone(), &v_rho.clone())?, &c_NA()?)?, &v_M.clone())?;
    let t769 = (f__fermi_from_n(v_n.clone())?).items()?;
    if t769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kF = t769[0].clone();
    v_EF = t769[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("E_F_eV"), py::truediv(&v_EF.clone(), &c_EV()?)?), (Py::str("T_F"), py::truediv(&v_EF.clone(), &c_KB()?)?), (Py::str("v_F"), py::truediv(&py::mul(&c_HBAR()?, &v_kF.clone())?, &c_ME()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_finite_well(mut v_V0: Py, mut v_a: Py, mut v_m: Py) -> R {
    let mut v_En: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_z0: Py = Py::None;
    let mut v_zs: Py = Py::None;
    if ({ let t882 = { let t875 = v_V0.clone(); Py::Bool({ let t876 = Py::Int(0i64); if py::le(&t875, &t876)? { true } else { false } }) }; if t882.truthy() { t882 } else { { let t881 = { let t877 = v_a.clone(); Py::Bool({ let t878 = Py::Int(0i64); if py::le(&t877, &t878)? { true } else { false } }) }; if t881.truthy() { t881 } else { { let t879 = v_m.clone(); Py::Bool({ let t880 = Py::Int(0i64); if py::le(&t879, &t880)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the depth, width and mass must be positive"))));
    }
    v_z0 = py::truediv(&py::mul(&py::truediv(&v_a.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_V0.clone())?)?)?, &c_HBAR()?)?;
    if ({ let t883 = v_z0.clone(); Py::Bool({ let t884 = Py::Int(200i64); if py::gt(&t883, &t884)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the well holds more than 127 bound states; give a shallower or narrower well"))));
    }
    v_k = Py::Int(1i64);
    while ({ let t885 = py::truediv(&py::mul(&v_k.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t886 = v_z0.clone(); if py::lt(&t885, &t886)? { true } else { false } }) }).truthy() {
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
    }
    if ({ let t891 = { let t887 = py::abs(&py::sub(&v_z0.clone(), &py::truediv(&py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?)?)?; Py::Bool({ let t888 = py::mul(&Py::Float(1e-09), &v_z0.clone())?; if py::le(&t887, &t888)? { true } else { false } }) }; if !t891.truthy() { t891 } else { { let t889 = v_k.clone(); Py::Bool({ let t890 = Py::Int(1i64); if py::gt(&t889, &t890)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a bound state sits exactly at the top of the well; its energy is not determined to the stated precision"))));
    }
    v_zs = Py::list(vec![]);
    for t892 in py::range_iter(&[(v_k.clone()).i()?]) {
        v_j = t892;
        let t893 = (Py::tuple(vec![py::truediv(&py::mul(&v_j.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?, py::minmax(&[py::truediv(&py::mul(&py::add(&v_j.clone(), &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?, v_z0.clone()], false)?])).items()?;
        if t893.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t893[0].clone();
        v_hi = t893[1].clone();
        if ({ let t894 = v_lo.clone(); Py::Bool({ let t895 = v_z0.clone(); if py::ge(&t894, &t895)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t896 = py::modulo(&v_j.clone(), &Py::Int(2i64))?; Py::Bool({ let t897 = Py::Int(0i64); if py::eq(&t896, &t897) { true } else { false } }) }).truthy() {
            v_f = { let v_V0 = v_V0.clone(); let v_a = v_a.clone(); let v_hi = v_hi.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_z0 = v_z0.clone(); let v_zs = v_zs.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&v_z.clone(), &py::math1("sin", &v_z.clone())?)?, &py::mul(&py::math1("sqrt", &py::minmax(&[py::sub(&py::mul(&v_z0.clone(), &v_z0.clone())?, &py::mul(&v_z.clone(), &v_z.clone())?)?, Py::Float(0.0)], true)?)?, &py::math1("cos", &v_z.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        } else {
            v_f = { let v_V0 = v_V0.clone(); let v_a = v_a.clone(); let v_f = v_f.clone(); let v_hi = v_hi.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_z0 = v_z0.clone(); let v_zs = v_zs.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::mul(&v_z.clone(), &py::math1("cos", &v_z.clone())?)?, &py::mul(&py::math1("sqrt", &py::minmax(&[py::sub(&py::mul(&v_z0.clone(), &v_z0.clone())?, &py::mul(&v_z.clone(), &v_z.clone())?)?, Py::Float(0.0)], true)?)?, &py::math1("sin", &v_z.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        }
        let _ = py::method(&v_zs.clone(), "append", &[f__fw_root(v_f.clone(), v_lo.clone(), v_hi.clone())?])?;
    }
    v_En = { let mut out = vec![]; for t911 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t911;
out.push(py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &c_HBAR()?)?, &c_HBAR()?)?, &v_z.clone())?, &v_z.clone())?, &py::mul(&py::mul(&v_m.clone(), &v_a.clone())?, &v_a.clone())?)?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("n_bound"), py::float(&py::len(&v_En.clone())?)?), (Py::str("E1_eV"), py::truediv(&py::getitem(&v_En.clone(), &Py::Int(0i64))?, &c_EV()?)?), (Py::str("energies_eV"), { let mut out = vec![]; for t921 in py::iter(&(v_En.clone()))? { let mut v_E: Py; 
v_E = t921;
out.push(py::truediv(&v_E.clone(), &c_EV()?)?); } Py::list(out) }), (Py::str("z0"), v_z0.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fixed_target_cm_energy(mut v_m1: Py, mut v_E1: Py, mut v_m2: Py) -> R {
    let mut v_p1: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t957 = { let t947 = v_m1.clone(); Py::Bool({ let t948 = Py::Int(0i64); if py::lt(&t947, &t948)? { true } else { false } }) }; if t957.truthy() { t957 } else { { let t956 = { let t949 = v_m2.clone(); Py::Bool({ let t950 = Py::Int(0i64); if py::le(&t949, &t950)? { true } else { false } }) }; if t956.truthy() { t956 } else { { let t955 = { let t951 = v_E1.clone(); Py::Bool({ let t952 = v_m1.clone(); if py::lt(&t951, &t952)? { true } else { false } }) }; if t955.truthy() { t955 } else { { let t953 = v_E1.clone(); Py::Bool({ let t954 = Py::Int(0i64); if py::le(&t953, &t954)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("masses must be non-negative (target positive) and the beam energy at least the beam mass"))));
    }
    v_s = py::add(&py::add(&py::mul(&v_m1.clone(), &v_m1.clone())?, &py::mul(&v_m2.clone(), &v_m2.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_E1.clone())?, &v_m2.clone())?)?;
    v_rs = py::math1("sqrt", &v_s.clone())?;
    v_p1 = py::math1("sqrt", &py::mul(&py::sub(&v_E1.clone(), &v_m1.clone())?, &py::add(&v_E1.clone(), &v_m1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sqrt_s"), v_rs.clone()), (Py::str("available"), py::sub(&py::sub(&v_rs.clone(), &v_m1.clone())?, &v_m2.clone())?), (Py::str("beta_cm"), py::truediv(&v_p1.clone(), &py::add(&v_E1.clone(), &v_m2.clone())?)?), (Py::str("gamma_cm"), py::truediv(&py::add(&v_E1.clone(), &v_m2.clone())?, &v_rs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gate_rotation(mut v_gates: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_a3: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_seq = f__gates_1q(v_gates.clone())?;
    v_U = Py::tuple(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])]);
    for t1018 in py::iter(&(v_seq.clone()))? {
        let t1019 = (t1018).items()?;
        if t1019.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v__ = t1019[0].clone();
        v__ = t1019[1].clone();
        v_M = t1019[2].clone();
        v_U = Py::tuple(vec![Py::tuple(vec![py::add(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, py::add(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?)?]), Py::tuple(vec![py::add(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, py::add(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?)?])]);
    }
    v_det = py::sub(&py::mul(&py::getitem(&py::getitem(&v_U.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_U.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_U.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?;
    v_r = py::cmath("sqrt", &[v_det.clone()])?;
    v_V = { let mut out = vec![]; for t1020 in py::range_iter(&[(Py::Int(2i64)).i()?]) { let mut v_i: Py; 
v_i = t1020;
out.push({ let mut out = vec![]; for t1021 in py::range_iter(&[(Py::Int(2i64)).i()?]) { let mut v_j: Py; 
v_j = t1021;
out.push(py::truediv(&py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_j.clone())?, &v_r.clone())?); } Py::list(out) }); } Py::list(out) };
    v_a0 = py::real(&py::truediv(&py::add(&py::getitem(&py::getitem(&v_V.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_V.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &Py::Int(2i64))?)?;
    v_a1 = py::real(&py::truediv(&py::mul(&Py::Complex(0.0, 1.0), &py::add(&py::getitem(&py::getitem(&v_V.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_V.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, &Py::Int(2i64))?)?;
    v_a2 = py::real(&py::truediv(&py::sub(&py::getitem(&py::getitem(&v_V.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_V.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, &Py::Int(2i64))?)?;
    v_a3 = py::real(&py::truediv(&py::sub(&py::getitem(&py::getitem(&v_V.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_V.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, &Py::Complex(0.0, 2.0))?)?;
    if ({ let t1022 = v_a0.clone(); Py::Bool({ let t1023 = Py::Int(0i64); if py::lt(&t1022, &t1023)? { true } else { false } }) }).truthy() {
        let t1024 = (Py::tuple(vec![py::neg(&v_a0.clone())?, py::neg(&v_a1.clone())?, py::neg(&v_a2.clone())?, py::neg(&v_a3.clone())?])).items()?;
        if t1024.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_a0 = t1024[0].clone();
        v_a1 = t1024[1].clone();
        v_a2 = t1024[2].clone();
        v_a3 = t1024[3].clone();
    }
    v_s = py::math1("sqrt", &py::add(&py::add(&py::mul(&v_a1.clone(), &v_a1.clone())?, &py::mul(&v_a2.clone(), &v_a2.clone())?)?, &py::mul(&v_a3.clone(), &v_a3.clone())?)?)?;
    if ({ let t1025 = v_s.clone(); Py::Bool({ let t1026 = Py::Float(1e-12); if py::lt(&t1025, &t1026)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("angle"), Py::Float(0.0)), (Py::str("angle_deg"), Py::Float(0.0)), (Py::str("nx"), Py::Float(0.0)), (Py::str("ny"), Py::Float(0.0)), (Py::str("nz"), Py::Float(1.0))]))?);
    }
    v_th = py::mul(&Py::Int(2i64), &py::atan2(&v_s.clone(), &v_a0.clone())?)?;
    v_n = Py::list(vec![py::truediv(&v_a1.clone(), &v_s.clone())?, py::truediv(&v_a2.clone(), &v_s.clone())?, py::truediv(&v_a3.clone(), &v_s.clone())?]);
    if ({ let t1036 = py::sub(&Py::Float(std::f64::consts::PI), &v_th.clone())?; Py::Bool({ let t1037 = Py::Float(1e-09); if py::lt(&t1036, &t1037)? { true } else { false } }) }).truthy() {
        v_first = (match ('g1038: { let mut out = vec![]; for t1039 in py::iter(&(v_n.clone()))? { let mut v_x: Py; 
v_x = t1039;
if ({ let t1040 = py::abs(&v_x.clone())?; Py::Bool({ let t1041 = Py::Float(1e-09); if py::gt(&t1040, &t1041)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); break 'g1038 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
        if ({ let t1042 = v_first.clone(); Py::Bool({ let t1043 = Py::Int(0i64); if py::lt(&t1042, &t1043)? { true } else { false } }) }).truthy() {
            v_n = { let mut out = vec![]; for t1044 in py::iter(&(v_n.clone()))? { let mut v_x: Py; 
v_x = t1044;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
        }
    }
    v_n = { let mut out = vec![]; for t1045 in py::iter(&(v_n.clone()))? { let mut v_x: Py; 
v_x = t1045;
out.push(f__snap(v_x.clone(), Py::Float(1.0))?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("angle"), v_th.clone()), (Py::str("angle_deg"), py::math1("degrees", &v_th.clone())?), (Py::str("nx"), py::getitem(&v_n.clone(), &Py::Int(0i64))?), (Py::str("ny"), py::getitem(&v_n.clone(), &Py::Int(1i64))?), (Py::str("nz"), py::getitem(&v_n.clone(), &Py::Int(2i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_grover(mut v_n_qubits: Py, mut v_M: Py, mut v_k: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_kopt: Py = Py::None;
    let mut v_th: Py = Py::None;
    if ({ let t1077 = { let t1073 = v_n_qubits.clone(); Py::Bool({ let t1074 = Py::Int(1i64); if py::lt(&t1073, &t1074)? { true } else { false } }) }; if t1077.truthy() { t1077 } else { { let t1075 = v_n_qubits.clone(); Py::Bool({ let t1076 = Py::Int(60i64); if py::gt(&t1075, &t1076)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the register must have 1 to 60 qubits"))));
    }
    v_N = py::pow(&Py::Int(2i64), &v_n_qubits.clone())?;
    if ({ let t1082 = { let t1078 = v_M.clone(); Py::Bool({ let t1079 = Py::Int(1i64); if py::lt(&t1078, &t1079)? { true } else { false } }) }; if t1082.truthy() { t1082 } else { { let t1080 = py::mul(&Py::Int(2i64), &v_M.clone())?; Py::Bool({ let t1081 = v_N.clone(); if py::ge(&t1080, &t1081)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of marked items must be at least 1 and less than half the search space"))));
    }
    if ({ let t1087 = { let t1083 = v_k.clone(); Py::Bool({ let t1084 = Py::Int(0i64); if py::lt(&t1083, &t1084)? { true } else { false } }) }; if t1087.truthy() { t1087 } else { { let t1085 = v_k.clone(); Py::Bool({ let t1086 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::gt(&t1085, &t1086)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of iterations must be between 0 and 10^7"))));
    }
    v_th = py::math1("asin", &py::math1("sqrt", &py::truediv(&v_M.clone(), &v_N.clone())?)?)?;
    v_kopt = py::math1("floor", &py::truediv(&Py::Float(std::f64::consts::PI), &py::mul(&Py::Int(4i64), &v_th.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_success"), py::pow(&py::math1("sin", &py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_th.clone())?)?, &Py::Int(2i64))?), (Py::str("k_opt"), py::float(&v_kopt.clone())?), (Py::str("P_at_k_opt"), py::pow(&py::math1("sin", &py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_kopt.clone())?, &Py::Int(1i64))?, &v_th.clone())?)?, &Py::Int(2i64))?), (Py::str("theta"), v_th.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heisenberg(mut v_dx: Py, mut v_m: Py) -> R {
    let mut v_dp: Py = Py::None;
    if ({ let t1223 = { let t1219 = v_dx.clone(); Py::Bool({ let t1220 = Py::Int(0i64); if py::le(&t1219, &t1220)? { true } else { false } }) }; if t1223.truthy() { t1223 } else { { let t1221 = v_m.clone(); Py::Bool({ let t1222 = Py::Int(0i64); if py::le(&t1221, &t1222)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the position uncertainty and the mass must be positive"))));
    }
    v_dp = py::truediv(&c_HBAR()?, &py::mul(&Py::Int(2i64), &v_dx.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dp"), v_dp.clone()), (Py::str("dp_eV_per_c"), py::truediv(&py::mul(&v_dp.clone(), &c_C()?)?, &c_EV()?)?), (Py::str("dv"), py::truediv(&v_dp.clone(), &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ho_forbidden(mut v_n: Py) -> R {
    let mut v_A2: Py = Py::None;
    let mut v_P_in: Py = Py::None;
    let mut v_P_out: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_inside: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_norm: Py = Py::None;
    let mut v_nxt: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sq: Py = Py::None;
    if ({ let t1126 = { let t1122 = v_n.clone(); Py::Bool({ let t1123 = Py::Int(0i64); if py::lt(&t1122, &t1123)? { true } else { false } }) }; if t1126.truthy() { t1126 } else { { let t1124 = v_n.clone(); Py::Bool({ let t1125 = Py::Int(60i64); if py::gt(&t1124, &t1125)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number from 0 to 60"))));
    }
    v_h = Py::list(vec![Py::list(vec![Py::Int(1i64)]), Py::list(vec![Py::Int(0i64), Py::Int(2i64)])]);
    for t1127 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t1127;
        let t1128 = (Py::tuple(vec![py::getitem(&v_h.clone(), &v_k.clone())?, py::getitem(&v_h.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?])).items()?;
        if t1128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1128[0].clone();
        v_b = t1128[1].clone();
        v_nxt = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(2i64))?)?;
        for t1129 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? {
            let t1132 = (t1129).items()?;
            if t1132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_i = t1132[0].clone();
            v_c = t1132[1].clone();
            let t1130 = v_nxt.clone(); let t1131 = py::add(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t1130, &t1131, py::iadd(&py::getitem(&t1130, &t1131)?, &py::mul(&Py::Int(2i64), &v_c.clone())?)?)?;
        }
        for t1133 in py::iter(&(py::enumerate(&v_b.clone(), 0)?))? {
            let t1136 = (t1133).items()?;
            if t1136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_i = t1136[0].clone();
            v_c = t1136[1].clone();
            let t1134 = v_nxt.clone(); let t1135 = v_i.clone();
            py::setitem(&t1134, &t1135, py::sub(&py::getitem(&t1134, &t1135)?, &py::mul(&py::mul(&Py::Int(2i64), &v_k.clone())?, &v_c.clone())?)?)?;
        }
        let _ = py::method(&v_h.clone(), "append", &[v_nxt.clone()])?;
    }
    v_p = py::getitem(&v_h.clone(), &v_n.clone())?;
    v_sq = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::sub(&py::mul(&Py::Int(2i64), &py::len(&v_p.clone())?)?, &Py::Int(1i64))?)?;
    for t1137 in py::iter(&(py::enumerate(&v_p.clone(), 0)?))? {
        let t1142 = (t1137).items()?;
        if t1142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1142[0].clone();
        v_a = t1142[1].clone();
        for t1138 in py::iter(&(py::enumerate(&v_p.clone(), 0)?))? {
            let t1141 = (t1138).items()?;
            if t1141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t1141[0].clone();
            v_b = t1141[1].clone();
            let t1139 = v_sq.clone(); let t1140 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t1139, &t1140, py::iadd(&py::getitem(&t1139, &t1140)?, &py::mul(&v_a.clone(), &v_b.clone())?)?)?;
        }
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(50i64))?;
        v_A2 = ax_engine::pymp::call("mpf", &[py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?], &[])?;
        v_inside = ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])?;
        for t1143 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::len(&v_sq.clone())?).i()?, (Py::Int(2i64)).i()?]) {
            v_k = t1143;
            if (py::getitem(&v_sq.clone(), &v_k.clone())?).truthy() {
                v_inside = py::iadd(&v_inside, &py::mul(&py::getitem(&v_sq.clone(), &v_k.clone())?, &ax_engine::pymp::call("gammainc", &[py::truediv(&ax_engine::pymp::call("mpf", &[py::add(&v_k.clone(), &Py::Int(1i64))?], &[])?, &Py::Int(2i64))?, Py::Int(0i64), v_A2.clone()], &[])?)?)?;
            }
        }
        v_norm = py::mul(&py::mul(&ax_engine::pymp::call("sqrt", &[ax_engine::pymp::constant("pi")?], &[])?, &py::pow(&ax_engine::pymp::call("mpf", &[Py::Int(2i64)], &[])?, &v_n.clone())?)?, &ax_engine::pymp::call("factorial", &[v_n.clone()], &[])?)?;
        v_P_in = py::truediv(&v_inside.clone(), &v_norm.clone())?;
        v_P_out = py::sub(&Py::Int(1i64), &v_P_in.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("P_forbidden"), py::float(&v_P_out.clone())?), (Py::str("P_allowed"), py::float(&v_P_in.clone())?), (Py::str("xi_turning"), py::math1("sqrt", &py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ho_levels(mut v_m: Py, mut v_omega: Py, mut v_n: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t1182 = { let t1178 = v_m.clone(); Py::Bool({ let t1179 = Py::Int(0i64); if py::le(&t1178, &t1179)? { true } else { false } }) }; if t1182.truthy() { t1182 } else { { let t1180 = v_omega.clone(); Py::Bool({ let t1181 = Py::Int(0i64); if py::le(&t1180, &t1181)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass and the angular frequency must be positive"))));
    }
    if ({ let t1183 = v_n.clone(); Py::Bool({ let t1184 = Py::Int(0i64); if py::lt(&t1183, &t1184)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number >= 0"))));
    }
    v_E = py::mul(&py::mul(&py::add(&v_n.clone(), &Py::Float(0.5))?, &c_HBAR()?)?, &v_omega.clone())?;
    v_s = py::math1("sqrt", &py::truediv(&c_HBAR()?, &py::mul(&v_m.clone(), &v_omega.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_eV"), py::truediv(&v_E.clone(), &c_EV()?)?), (Py::str("E0_eV"), py::truediv(&py::mul(&py::mul(&Py::Float(0.5), &c_HBAR()?)?, &v_omega.clone())?, &c_EV()?)?), (Py::str("spacing_eV"), py::truediv(&py::mul(&c_HBAR()?, &v_omega.clone())?, &c_EV()?)?), (Py::str("amplitude"), py::mul(&v_s.clone(), &py::math1("sqrt", &py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?)?)?), (Py::str("dx"), py::mul(&v_s.clone(), &py::math1("sqrt", &py::add(&v_n.clone(), &Py::Float(0.5))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrogen_transition(mut v_Z: Py, mut v_n_upper: Py, mut v_n_lower: Py) -> R {
    return Ok(f__out(f__transition(v_Z.clone(), v_n_upper.clone(), v_n_lower.clone(), Py::Float(1.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrogen_transition_reduced(mut v_Z: Py, mut v_n_upper: Py, mut v_n_lower: Py, mut v_M_u: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_mu_ratio: Py = Py::None;
    if ({ let t1388 = v_M_u.clone(); Py::Bool({ let t1389 = Py::Int(0i64); if py::le(&t1388, &t1389)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the nuclear mass must be positive"))));
    }
    v_M = py::mul(&v_M_u.clone(), &c_AMU()?)?;
    v_mu_ratio = py::truediv(&v_M.clone(), &py::add(&v_M.clone(), &c_ME()?)?)?;
    return Ok({ let mut v: Vec<Py> = vec![];  let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &f__transition(v_Z.clone(), v_n_upper.clone(), v_n_lower.clone(), v_mu_ratio.clone())?)?; d.push((Py::str("mu_ratio"), v_mu_ratio.clone()));  if v.len() > 0 { return Err(PyErr::Type("_out() takes fewer positional arguments".into())) } f__out(Py::dict(d))? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_well_expectations(mut v_n: Py, mut v_L: Py) -> R {
    let mut v_dp: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_x2: Py = Py::None;
    let _ = f__quantum_n(v_n.clone(), Py::str("n"))?;
    if ({ let t1559 = v_L.clone(); Py::Bool({ let t1560 = Py::Int(0i64); if py::le(&t1559, &t1560)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the width must be positive"))));
    }
    v_x2 = py::mul(&py::mul(&v_L.clone(), &v_L.clone())?, &py::sub(&py::truediv(&Py::Int(1i64), &Py::Int(3i64))?, &py::truediv(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &py::pow(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?)?)?)?)?;
    v_dx = py::math1("sqrt", &py::sub(&v_x2.clone(), &py::truediv(&py::mul(&v_L.clone(), &v_L.clone())?, &Py::Int(4i64))?)?)?;
    v_dp = py::truediv(&py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &c_HBAR()?)?, &v_L.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("x_mean"), py::truediv(&v_L.clone(), &Py::Int(2i64))?), (Py::str("dx"), v_dx.clone()), (Py::str("dx_over_L"), py::truediv(&v_dx.clone(), &v_L.clone())?), (Py::str("dp"), v_dp.clone()), (Py::str("product_over_hbar"), py::truediv(&py::mul(&v_dx.clone(), &v_dp.clone())?, &c_HBAR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_well_levels(mut v_m: Py, mut v_L: Py, mut v_n: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_E1: Py = Py::None;
    if ({ let t1432 = { let t1428 = v_m.clone(); Py::Bool({ let t1429 = Py::Int(0i64); if py::le(&t1428, &t1429)? { true } else { false } }) }; if t1432.truthy() { t1432 } else { { let t1430 = v_L.clone(); Py::Bool({ let t1431 = Py::Int(0i64); if py::le(&t1430, &t1431)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass and the width must be positive"))));
    }
    let _ = f__quantum_n(v_n.clone(), Py::str("n"))?;
    v_E = py::truediv(&py::pow(&py::truediv(&py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &c_HBAR()?)?, &v_L.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &v_m.clone())?)?;
    v_E1 = py::truediv(&py::pow(&py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &c_HBAR()?)?, &v_L.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_eV"), py::truediv(&v_E.clone(), &c_EV()?)?), (Py::str("E1_eV"), py::truediv(&v_E1.clone(), &c_EV()?)?), (Py::str("p"), py::truediv(&py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &c_HBAR()?)?, &v_L.clone())?), (Py::str("wavelength"), py::truediv(&py::mul(&Py::Int(2i64), &v_L.clone())?, &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_well_probability(mut v_n: Py, mut v_L: Py, mut v_x1: Py, mut v_x2: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_cc: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_ums: Py = Py::None;
    let _ = f__quantum_n(v_n.clone(), Py::str("n"))?;
    if ({ let t1477 = { let t1471 = v_L.clone(); Py::Bool({ let t1472 = Py::Int(0i64); if py::le(&t1471, &t1472)? { true } else { false } }) }; if t1477.truthy() { t1477 } else { py::not(&{ let t1473 = Py::Int(0i64); Py::Bool({ let t1474 = v_x1.clone(); if py::le(&t1473, &t1474)? { { let t1475 = v_x2.clone(); if py::lt(&t1474, &t1475)? { { let t1476 = v_L.clone(); if py::le(&t1475, &t1476)? { true } else { false } } } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 0 <= x1 < x2 <= L"))));
    }
    v_u = py::truediv(&py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &py::sub(&v_x2.clone(), &v_x1.clone())?)?, &v_L.clone())?;
    v_cc = py::truediv(&py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &py::add(&v_x1.clone(), &v_x2.clone())?)?, &v_L.clone())?;
    v_ums = (if ({ let t1478 = v_u.clone(); Py::Bool({ let t1479 = Py::Float(0.001); if py::gt(&t1478, &t1479)? { true } else { false } }) }).truthy() { py::sub(&v_u.clone(), &py::math1("sin", &v_u.clone())?)? } else { py::add(&py::sub(&py::truediv(&py::pow(&v_u.clone(), &Py::Int(3i64))?, &Py::Int(6i64))?, &py::truediv(&py::pow(&v_u.clone(), &Py::Int(5i64))?, &Py::Int(120i64))?)?, &py::truediv(&py::pow(&v_u.clone(), &Py::Int(7i64))?, &Py::Int(5040i64))?)? });
    v_P = py::truediv(&py::add(&v_ums.clone(), &py::mul(&py::mul(&Py::Int(2i64), &py::pow(&py::math1("sin", &py::truediv(&v_cc.clone(), &Py::Int(2i64))?)?, &Py::Int(2i64))?)?, &py::math1("sin", &v_u.clone())?)?)?, &py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("classical"), py::truediv(&py::sub(&v_x2.clone(), &v_x1.clone())?, &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_well_transition(mut v_m: Py, mut v_L: Py, mut v_n_upper: Py, mut v_n_lower: Py) -> R {
    let mut v_E: Py = Py::None;
    if ({ let t1518 = { let t1514 = v_m.clone(); Py::Bool({ let t1515 = Py::Int(0i64); if py::le(&t1514, &t1515)? { true } else { false } }) }; if t1518.truthy() { t1518 } else { { let t1516 = v_L.clone(); Py::Bool({ let t1517 = Py::Int(0i64); if py::le(&t1516, &t1517)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass and the width must be positive"))));
    }
    let _ = f__quantum_n(v_n_lower.clone(), Py::str("n_lower"))?;
    if ({ let t1521 = v_n_upper.clone(); Py::Bool({ let t1522 = v_n_lower.clone(); if py::le(&t1521, &t1522)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the upper level must lie above the lower level"))));
    }
    v_E = py::truediv(&py::mul(&py::mul(&py::sub(&v_n_upper.clone(), &v_n_lower.clone())?, &py::add(&v_n_upper.clone(), &v_n_lower.clone())?)?, &py::pow(&py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &c_HBAR()?)?, &v_L.clone())?, &Py::Int(2i64))?)?, &py::mul(&Py::Int(2i64), &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_photon_eV"), py::truediv(&v_E.clone(), &c_EV()?)?), (Py::str("lam"), py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_E.clone())?), (Py::str("lambda_nm"), py::mul(&py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_E.clone())?, &Py::Float(1000000000.0))?), (Py::str("f"), py::truediv(&v_E.clone(), &c_H()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_invariant_mass(mut v_E1: Py, mut v_p1: Py, mut v_E2: Py, mut v_p2: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_M2: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Pm: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t1598 = (Py::tuple(vec![f__vec3(v_p1.clone(), Py::str("p1"))?, f__vec3(v_p2.clone(), Py::str("p2"))?])).items()?;
    if t1598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t1598[0].clone();
    v_p2 = t1598[1].clone();
    for t1599 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_E1.clone(), v_p1.clone(), Py::Int(1i64)]), Py::tuple(vec![v_E2.clone(), v_p2.clone(), Py::Int(2i64)])])))? {
        let t1606 = (t1599).items()?;
        if t1606.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_E = t1606[0].clone();
        v_p = t1606[1].clone();
        v_n = t1606[2].clone();
        if ({ let t1605 = { let t1600 = v_E.clone(); Py::Bool({ let t1601 = Py::Int(0i64); if py::le(&t1600, &t1601)? { true } else { false } }) }; if t1605.truthy() { t1605 } else { { let t1602 = v_E.clone(); Py::Bool({ let t1603 = py::mul(&py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t1604 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t1604;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?, &py::sub(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::lt(&t1602, &t1603)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["particle ".to_string(), py::format(&v_n.clone(), "")?, " needs E >= |p| (a real, non-negative mass)".to_string()].concat()))));
        }
    }
    v_E = py::add(&v_E1.clone(), &v_E2.clone())?;
    v_P = { let mut out = vec![]; for t1607 in py::iter(&(py::zip(&[v_p1.clone(), v_p2.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1608 = (t1607).items()?;
if t1608.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1608[0].clone();
v_b = t1608[1].clone();
out.push(py::add(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_Pm = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t1609 in py::iter(&(v_P.clone()))? { let mut v_x: Py; 
v_x = t1609;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?;
    v_M2 = py::mul(&py::sub(&v_E.clone(), &v_Pm.clone())?, &py::add(&v_E.clone(), &v_Pm.clone())?)?;
    if ({ let t1610 = v_M2.clone(); Py::Bool({ let t1611 = Py::Int(0i64); if py::le(&t1610, &t1611)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pair has no rest frame (zero invariant mass)"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("M"), py::math1("sqrt", &v_M2.clone())?), (Py::str("P"), v_Pm.clone()), (Py::str("beta_cm"), py::truediv(&v_Pm.clone(), &v_E.clone())?), (Py::str("gamma_cm"), py::truediv(&v_E.clone(), &py::math1("sqrt", &v_M2.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lande_zeeman(mut v_L: Py, mut v_S: Py, mut v_J: Py, mut v_mJ: Py, mut v_B: Py) -> R {
    let mut v_dE: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t1649 = (Py::tuple(vec![f__half(v_L.clone(), Py::str("L"))?, f__half(v_S.clone(), Py::str("S"))?, f__half(v_J.clone(), Py::str("J"))?, f__half(v_mJ.clone(), Py::str("mJ"))?])).items()?;
    if t1649.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_L = t1649[0].clone();
    v_S = t1649[1].clone();
    v_J = t1649[2].clone();
    v_mJ = t1649[3].clone();
    if ({ let t1657 = { let t1650 = v_L.clone(); Py::Bool({ let t1651 = Py::Int(0i64); if py::lt(&t1650, &t1651)? { true } else { false } }) }; if t1657.truthy() { t1657 } else { { let t1656 = { let t1652 = v_S.clone(); Py::Bool({ let t1653 = Py::Int(0i64); if py::lt(&t1652, &t1653)? { true } else { false } }) }; if t1656.truthy() { t1656 } else { { let t1654 = py::denominator(&v_L.clone())?; Py::Bool({ let t1655 = Py::Int(1i64); if py::ne(&t1654, &t1655) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("L must be a whole number >= 0 and S >= 0"))));
    }
    if ({ let t1668 = { let t1658 = v_J.clone(); Py::Bool({ let t1659 = Py::Int(0i64); if py::le(&t1658, &t1659)? { true } else { false } }) }; if t1668.truthy() { t1668 } else { { let t1667 = { let t1660 = v_J.clone(); Py::Bool({ let t1661 = py::abs(&py::sub(&v_L.clone(), &v_S.clone())?)?; if py::lt(&t1660, &t1661)? { true } else { false } }) }; if t1667.truthy() { t1667 } else { { let t1666 = { let t1662 = v_J.clone(); Py::Bool({ let t1663 = py::add(&v_L.clone(), &v_S.clone())?; if py::gt(&t1662, &t1663)? { true } else { false } }) }; if t1666.truthy() { t1666 } else { { let t1664 = py::denominator(&py::sub(&py::add(&v_L.clone(), &v_S.clone())?, &v_J.clone())?)?; Py::Bool({ let t1665 = Py::Int(1i64); if py::ne(&t1664, &t1665) { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("J must lie between |L - S| and L + S in whole steps, and be positive"))));
    }
    if ({ let t1673 = { let t1669 = py::abs(&v_mJ.clone())?; Py::Bool({ let t1670 = v_J.clone(); if py::gt(&t1669, &t1670)? { true } else { false } }) }; if t1673.truthy() { t1673 } else { { let t1671 = py::denominator(&py::sub(&v_J.clone(), &v_mJ.clone())?)?; Py::Bool({ let t1672 = Py::Int(1i64); if py::ne(&t1671, &t1672) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("mJ must lie between -J and J in whole steps"))));
    }
    v_g = py::add(&Py::Int(1i64), &py::truediv(&py::sub(&py::add(&py::mul(&v_J.clone(), &py::add(&v_J.clone(), &Py::Int(1i64))?)?, &py::mul(&v_S.clone(), &py::add(&v_S.clone(), &Py::Int(1i64))?)?)?, &py::mul(&v_L.clone(), &py::add(&v_L.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_J.clone())?, &py::add(&v_J.clone(), &Py::Int(1i64))?)?)?)?;
    v_dE = py::mul(&py::mul(&py::float(&py::mul(&v_g.clone(), &v_mJ.clone())?)?, &c_MUB()?)?, &v_B.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("g_J"), py::float(&v_g.clone())?), (Py::str("dE"), v_dE.clone()), (Py::str("dE_eV"), py::truediv(&v_dE.clone(), &c_EV()?)?), (Py::str("g_J_exact"), (if ({ let t1683 = py::denominator(&v_g.clone())?; Py::Bool({ let t1684 = Py::Int(1i64); if py::ne(&t1683, &t1684) { true } else { false } }) }).truthy() { Py::str(&[py::format(&py::numerator(&v_g.clone())?, "")?, "/".to_string(), py::format(&py::denominator(&v_g.clone())?, "")?].concat()) } else { Py::str(&py::to_str(&py::numerator(&v_g.clone())?)) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_larmor(mut v_particle: Py, mut v_B: Py, mut v_t: Py) -> R {
    let mut v_w: Py = Py::None;
    if ({ let t1714 = { let t1710 = v_B.clone(); Py::Bool({ let t1711 = Py::Int(0i64); if py::le(&t1710, &t1711)? { true } else { false } }) }; if t1714.truthy() { t1714 } else { { let t1712 = v_t.clone(); Py::Bool({ let t1713 = Py::Int(0i64); if py::lt(&t1712, &t1713)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the field must be positive and the time non-negative"))));
    }
    v_w = py::mul(&py::getitem(&c__GYRO()?, &v_particle.clone())?, &v_B.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_L"), v_w.clone()), (Py::str("f_L"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("Sx_mean"), f__snap(py::math1("cos", &py::mul(&v_w.clone(), &v_t.clone())?)?, Py::Float(1.0))?), (Py::str("P_plus_x"), f__snap(py::pow(&py::math1("cos", &py::truediv(&py::mul(&v_w.clone(), &v_t.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?, Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_length_contraction(mut v_L0: Py, mut v_v: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t1751 = v_L0.clone(); Py::Bool({ let t1752 = Py::Int(0i64); if py::le(&t1751, &t1752)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the proper length must be positive"))));
    }
    if ({ let t1753 = v_v.clone(); Py::Bool({ let t1754 = Py::Int(0i64); if py::lt(&t1753, &t1754)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the speed as a magnitude (v >= 0)"))));
    }
    v_b = f__beta(v_v.clone())?;
    v_s = py::math1("sqrt", &py::mul(&py::sub(&Py::Int(1i64), &v_b.clone())?, &py::add(&Py::Int(1i64), &v_b.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L"), py::mul(&v_L0.clone(), &v_s.clone())?), (Py::str("gamma"), py::truediv(&Py::Int(1i64), &v_s.clone())?), (Py::str("shortening"), py::mul(&py::mul(&v_L0.clone(), &f__gm1(v_b.clone())?)?, &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lorentz_factor(mut v_v: Py) -> R {
    let mut v_b: Py = Py::None;
    if ({ let t1792 = v_v.clone(); Py::Bool({ let t1793 = Py::Int(0i64); if py::lt(&t1792, &t1793)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the speed as a magnitude (v >= 0)"))));
    }
    v_b = f__beta(v_v.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gamma"), f__gamma_of_beta(v_b.clone())?), (Py::str("beta"), v_b.clone()), (Py::str("gamma_minus_1"), f__gm1(v_b.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_momentum_energy(mut v_m: Py, mut v_v: Py) -> R {
    let mut v_E0: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    if ({ let t2533 = v_m.clone(); Py::Bool({ let t2534 = Py::Int(0i64); if py::le(&t2533, &t2534)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass must be positive"))));
    }
    if ({ let t2535 = v_v.clone(); Py::Bool({ let t2536 = Py::Int(0i64); if py::lt(&t2535, &t2536)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the speed as a magnitude (v >= 0)"))));
    }
    v_b = f__beta(v_v.clone())?;
    v_g = f__gamma_of_beta(v_b.clone())?;
    v_E0 = py::mul(&py::mul(&v_m.clone(), &c_C()?)?, &c_C()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("p"), py::mul(&py::mul(&v_g.clone(), &v_m.clone())?, &v_v.clone())?), (Py::str("E"), py::mul(&v_g.clone(), &v_E0.clone())?), (Py::str("K"), py::mul(&f__gm1(v_b.clone())?, &v_E0.clone())?), (Py::str("E0"), v_E0.clone()), (Py::str("gamma"), v_g.clone()), (Py::str("E_MeV"), py::truediv(&py::mul(&v_g.clone(), &v_E0.clone())?, &c_MEV()?)?), (Py::str("K_MeV"), py::truediv(&py::mul(&f__gm1(v_b.clone())?, &v_E0.clone())?, &c_MEV()?)?), (Py::str("pc_MeV"), py::truediv(&py::mul(&py::mul(&py::mul(&v_g.clone(), &v_m.clone())?, &v_v.clone())?, &c_C()?)?, &c_MEV()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moseley(mut v_Z: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_lam: Py = Py::None;
    if ({ let t1831 = v_Z.clone(); Py::Bool({ let t1832 = Py::Int(2i64); if py::lt(&t1831, &t1832)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Moseley's law needs Z >= 2 (one electron left in the K shell screens the nucleus)"))));
    }
    v_E = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.75), &py::pow(&py::sub(&v_Z.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &c_RINF()?)?, &c_H()?)?, &c_C()?)?;
    v_lam = py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_E.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_keV"), py::truediv(&v_E.clone(), &py::mul(&Py::Float(1000.0), &c_EV()?)?)?), (Py::str("lam"), v_lam.clone()), (Py::str("lambda_nm"), py::mul(&v_lam.clone(), &Py::Float(1000000000.0))?), (Py::str("lambda_pm"), py::mul(&v_lam.clone(), &Py::Float(1000000000000.0))?), (Py::str("f"), py::truediv(&v_E.clone(), &c_H()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nuclide_decay(mut v_nuclide: Py, mut v_t: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_lam: Py = Py::None;
    if ({ let t1910 = v_t.clone(); Py::Bool({ let t1911 = Py::Int(0i64); if py::lt(&t1910, &t1911)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the elapsed time must be non-negative"))));
    }
    v_T = py::getitem(&c__NUCLIDES()?, &v_nuclide.clone())?;
    v_lam = py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("fraction"), py::pow(&Py::Float(2.0), &py::truediv(&py::neg(&v_t.clone())?, &v_T.clone())?)?), (Py::str("half_life"), v_T.clone()), (Py::str("half_life_years"), py::truediv(&v_T.clone(), &Py::Float(31557600.0))?), (Py::str("decay_constant"), v_lam.clone()), (Py::str("mean_life"), py::truediv(&Py::Int(1i64), &v_lam.clone())?), (Py::str("specific_activity_per_atom"), py::mul(&v_lam.clone(), &py::pow(&Py::Float(2.0), &py::truediv(&py::neg(&v_t.clone())?, &v_T.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_photoelectric(mut v_lam: Py, mut v_W: Py) -> R {
    let mut v_Eph: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_lt: Py = Py::None;
    if ({ let t1987 = { let t1983 = v_lam.clone(); Py::Bool({ let t1984 = Py::Int(0i64); if py::le(&t1983, &t1984)? { true } else { false } }) }; if t1987.truthy() { t1987 } else { { let t1985 = v_W.clone(); Py::Bool({ let t1986 = Py::Int(0i64); if py::le(&t1985, &t1986)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wavelength and the work function must be positive"))));
    }
    v_Eph = py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_lam.clone())?;
    v_K = py::sub(&v_Eph.clone(), &v_W.clone())?;
    if ({ let t1988 = v_K.clone(); Py::Bool({ let t1989 = py::mul(&py::neg(&Py::Float(1e-12))?, &v_Eph.clone())?; if py::lt(&t1988, &t1989)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no electrons are emitted: the photon energy is below the work function"))));
    }
    v_K = py::minmax(&[v_K.clone(), Py::Float(0.0)], true)?;
    v_lt = py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_W.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_photon_eV"), py::truediv(&v_Eph.clone(), &c_EV()?)?), (Py::str("K_max_eV"), py::truediv(&v_K.clone(), &c_EV()?)?), (Py::str("V_stop"), py::truediv(&v_K.clone(), &c_QE()?)?), (Py::str("lambda_threshold"), v_lt.clone()), (Py::str("lambda_threshold_nm"), py::mul(&v_lt.clone(), &Py::Float(1000000000.0))?), (Py::str("f_threshold"), py::truediv(&v_W.clone(), &c_H()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_photon_energy(mut v_lam: Py) -> R {
    let mut v_E: Py = Py::None;
    if (py::not(&{ let t2026 = { let t2024 = v_lam.clone(); Py::Bool({ let t2025 = Py::Int(0i64); if py::gt(&t2024, &t2025)? { true } else { false } }) }; if !t2026.truthy() { t2026 } else { py::math1("isfinite", &v_lam.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wavelength must be positive"))));
    }
    v_E = py::truediv(&py::mul(&c_H()?, &c_C()?)?, &v_lam.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_eV"), py::truediv(&v_E.clone(), &c_EV()?)?), (Py::str("f"), py::truediv(&c_C()?, &v_lam.clone())?), (Py::str("p"), py::truediv(&c_H()?, &v_lam.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_photon_gas(mut v_T: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_u: Py = Py::None;
    if ({ let t2061 = v_T.clone(); Py::Bool({ let t2062 = Py::Int(0i64); if py::le(&t2061, &t2062)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature must be positive"))));
    }
    v_u = py::truediv(&py::mul(&py::truediv(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &Py::Int(15i64))?, &py::pow(&py::mul(&c_KB()?, &v_T.clone())?, &Py::Int(4i64))?)?, &py::pow(&py::mul(&c_HBAR()?, &c_C()?)?, &Py::Int(3i64))?)?;
    v_n = py::mul(&py::truediv(&py::mul(&Py::Int(2i64), &c_ZETA3()?)?, &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &py::pow(&py::truediv(&py::mul(&c_KB()?, &v_T.clone())?, &py::mul(&c_HBAR()?, &c_C()?)?)?, &Py::Int(3i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("u"), v_u.clone()), (Py::str("n"), v_n.clone()), (Py::str("mean_E_eV"), py::truediv(&py::truediv(&v_u.clone(), &v_n.clone())?, &c_EV()?)?), (Py::str("mean_E_over_kT"), py::truediv(&v_u.clone(), &py::mul(&py::mul(&v_n.clone(), &c_KB()?)?, &v_T.clone())?)?), (Py::str("pressure"), py::truediv(&v_u.clone(), &Py::Int(3i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_planck_radiance(mut v_lam: Py, mut v_T: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_nbar: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t2101 = { let t2097 = v_lam.clone(); Py::Bool({ let t2098 = Py::Int(0i64); if py::le(&t2097, &t2098)? { true } else { false } }) }; if t2101.truthy() { t2101 } else { { let t2099 = v_T.clone(); Py::Bool({ let t2100 = Py::Int(0i64); if py::le(&t2099, &t2100)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wavelength and the temperature must be positive"))));
    }
    v_x = py::truediv(&py::mul(&c_H()?, &c_C()?)?, &py::mul(&py::mul(&v_lam.clone(), &c_KB()?)?, &v_T.clone())?)?;
    if ({ let t2102 = v_x.clone(); Py::Bool({ let t2103 = Py::Int(700i64); if py::gt(&t2102, &t2103)? { true } else { false } }) }).truthy() {
        v_nbar = py::math1("exp", &py::neg(&v_x.clone())?)?;
    } else {
        v_nbar = py::truediv(&Py::Int(1i64), &py::math1("expm1", &v_x.clone())?)?;
    }
    v_B = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &c_H()?)?, &c_C()?)?, &c_C()?)?, &py::pow(&v_lam.clone(), &Py::Int(5i64))?)?, &v_nbar.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("B_lambda"), v_B.clone()), (Py::str("B_lambda_per_nm"), py::mul(&v_B.clone(), &Py::Float(1e-09))?), (Py::str("occupation"), v_nbar.clone()), (Py::str("x"), v_x.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qubit_entropy(mut v_r: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    v_r = f__vec3(v_r.clone(), Py::str("r"))?;
    v_L = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t2141 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t2141;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?;
    if ({ let t2142 = v_L.clone(); Py::Bool({ let t2143 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::gt(&t2142, &t2143)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a Bloch vector has length at most 1"))));
    }
    v_L = py::minmax(&[v_L.clone(), Py::Float(1.0)], false)?;
    let t2144 = (Py::tuple(vec![py::truediv(&py::add(&Py::Int(1i64), &v_L.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&Py::Int(1i64), &v_L.clone())?, &Py::Int(2i64))?])).items()?;
    if t2144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l1 = t2144[0].clone();
    v_l2 = t2144[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("length"), v_L.clone()), (Py::str("purity"), py::truediv(&py::add(&Py::Int(1i64), &py::mul(&v_L.clone(), &v_L.clone())?)?, &Py::Int(2i64))?), (Py::str("S_bits"), f__snap(py::add(&f__xlog2(v_l1.clone())?, &f__xlog2(v_l2.clone())?)?, Py::Float(1.0))?), (Py::str("lambda_max"), v_l1.clone()), (Py::str("lambda_min"), f__snap(v_l2.clone(), Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qubit_fidelity(mut v_r: Py, mut v_s: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_dot: Py = Py::None;
    let mut v_lr: Py = Py::None;
    let mut v_ls: Py = Py::None;
    let t2186 = (Py::tuple(vec![f__vec3(v_r.clone(), Py::str("r"))?, f__vec3(v_s.clone(), Py::str("s"))?])).items()?;
    if t2186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2186[0].clone();
    v_s = t2186[1].clone();
    let t2189 = (Py::tuple(vec![py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t2187 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t2187;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?, py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t2188 in py::iter(&(v_s.clone()))? { let mut v_x: Py; 
v_x = t2188;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?])).items()?;
    if t2189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lr = t2189[0].clone();
    v_ls = t2189[1].clone();
    if ({ let t2194 = { let t2190 = v_lr.clone(); Py::Bool({ let t2191 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::gt(&t2190, &t2191)? { true } else { false } }) }; if t2194.truthy() { t2194 } else { { let t2192 = v_ls.clone(); Py::Bool({ let t2193 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::gt(&t2192, &t2193)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a Bloch vector has length at most 1"))));
    }
    v_dot = py::sum(&{ let mut out = vec![]; for t2195 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2196 = (t2195).items()?;
if t2196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2196[0].clone();
v_b = t2196[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_F = py::mul(&Py::Float(0.5), &py::add(&py::add(&Py::Int(1i64), &v_dot.clone())?, &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::mul(&py::sub(&Py::Int(1i64), &py::mul(&v_lr.clone(), &v_lr.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_ls.clone(), &v_ls.clone())?)?)?], true)?)?)?)?;
    v_D = py::mul(&Py::Float(0.5), &py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t2197 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2198 = (t2197).items()?;
if t2198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2198[0].clone();
v_b = t2198[1].clone();
out.push(py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), f__snap(v_F.clone(), Py::Float(1.0))?), (Py::str("sqrt_F"), f__snap(py::math1("sqrt", &py::minmax(&[v_F.clone(), Py::Float(0.0)], true)?)?, Py::Float(1.0))?), (Py::str("trace_distance"), f__snap(v_D.clone(), Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qubit_state(mut v_a_re: Py, mut v_a_im: Py, mut v_b_re: Py, mut v_b_im: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_ab: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_nrm: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_theta: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2235 = (Py::tuple(vec![py::complex(&[v_a_re.clone(), v_a_im.clone()])?, py::complex(&[v_b_re.clone(), v_b_im.clone()])?])).items()?;
    if t2235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2235[0].clone();
    v_b = t2235[1].clone();
    v_nrm = py::math1("sqrt", &py::add(&py::pow(&py::abs(&v_a.clone())?, &Py::Int(2i64))?, &py::pow(&py::abs(&v_b.clone())?, &Py::Int(2i64))?)?)?;
    if ({ let t2238 = py::not(&py::math1("isfinite", &v_nrm.clone())?); if t2238.truthy() { t2238 } else { { let t2236 = v_nrm.clone(); Py::Bool({ let t2237 = Py::Int(0i64); if py::eq(&t2236, &t2237) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the state needs at least one non-zero amplitude"))));
    }
    let t2239 = (Py::tuple(vec![py::truediv(&v_a.clone(), &v_nrm.clone())?, py::truediv(&v_b.clone(), &v_nrm.clone())?])).items()?;
    if t2239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2239[0].clone();
    v_b = t2239[1].clone();
    let t2240 = (Py::tuple(vec![py::pow(&py::abs(&v_a.clone())?, &Py::Int(2i64))?, py::pow(&py::abs(&v_b.clone())?, &Py::Int(2i64))?])).items()?;
    if t2240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t2240[0].clone();
    v_p1 = t2240[1].clone();
    v_ab = py::mul(&py::method(&v_a.clone(), "conjugate", &[])?, &v_b.clone())?;
    let t2241 = (Py::tuple(vec![py::mul(&Py::Int(2i64), &py::real(&v_ab.clone())?)?, py::mul(&Py::Int(2i64), &py::imag(&v_ab.clone())?)?, py::sub(&v_p0.clone(), &v_p1.clone())?])).items()?;
    if t2241.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2241[0].clone();
    v_y = t2241[1].clone();
    v_z = t2241[2].clone();
    v_theta = py::mul(&Py::Int(2i64), &py::atan2(&py::abs(&v_b.clone())?, &py::abs(&v_a.clone())?)?)?;
    v_phi = (if ({ let t2246 = { let t2242 = py::abs(&v_a.clone())?; Py::Bool({ let t2243 = Py::Float(1e-12); if py::lt(&t2242, &t2243)? { true } else { false } }) }; if t2246.truthy() { t2246 } else { { let t2244 = py::abs(&v_b.clone())?; Py::Bool({ let t2245 = Py::Float(1e-12); if py::lt(&t2244, &t2245)? { true } else { false } }) } } }).truthy() { Py::Float(0.0) } else { py::modulo(&py::sub(&py::cmath("phase", &[v_b.clone()])?, &py::cmath("phase", &[v_a.clone()])?)?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)? });
    if ({ let t2247 = v_phi.clone(); Py::Bool({ let t2248 = py::sub(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-12))?; if py::gt(&t2247, &t2248)? { true } else { false } }) }).truthy() {
        v_phi = Py::Float(0.0);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("p0"), f__snap(v_p0.clone(), Py::Float(1.0))?), (Py::str("p1"), f__snap(v_p1.clone(), Py::Float(1.0))?), (Py::str("x"), f__snap(v_x.clone(), Py::Float(1.0))?), (Py::str("y"), f__snap(v_y.clone(), Py::Float(1.0))?), (Py::str("z"), f__snap(v_z.clone(), Py::Float(1.0))?), (Py::str("theta"), v_theta.clone()), (Py::str("phi"), v_phi.clone()), (Py::str("norm"), v_nrm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rabi(mut v_Omega: Py, mut v_Delta: Py, mut v_t: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_amp: Py = Py::None;
    if ({ let t2289 = { let t2285 = v_Omega.clone(); Py::Bool({ let t2286 = Py::Int(0i64); if py::le(&t2285, &t2286)? { true } else { false } }) }; if t2289.truthy() { t2289 } else { { let t2287 = v_t.clone(); Py::Bool({ let t2288 = Py::Int(0i64); if py::lt(&t2287, &t2288)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Rabi frequency must be positive and the time non-negative"))));
    }
    v_W = py::hypot(&[v_Omega.clone(), v_Delta.clone()])?;
    v_amp = py::pow(&py::truediv(&v_Omega.clone(), &v_W.clone())?, &Py::Int(2i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_excited"), f__snap(py::mul(&v_amp.clone(), &py::pow(&py::math1("sin", &py::truediv(&py::mul(&v_W.clone(), &v_t.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?)?, Py::Float(1.0))?), (Py::str("W"), v_W.clone()), (Py::str("P_max"), v_amp.clone()), (Py::str("period"), py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_W.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radial_expectations(mut v_n: Py, mut v_l: Py, mut v_Z: Py) -> R {
    let mut v_ir: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let _ = f__check_nl(v_n.clone(), v_l.clone(), v_Z.clone())?;
    v_r = py::truediv(&py::sub(&py::mul(&py::mul(&Py::Int(3i64), &v_n.clone())?, &v_n.clone())?, &py::mul(&v_l.clone(), &py::add(&v_l.clone(), &Py::Int(1i64))?)?)?, &py::mul(&Py::Int(2i64), &v_Z.clone())?)?;
    v_r2 = py::truediv(&py::mul(&py::mul(&v_n.clone(), &v_n.clone())?, &py::sub(&py::add(&py::mul(&py::mul(&Py::Int(5i64), &v_n.clone())?, &v_n.clone())?, &Py::Int(1i64))?, &py::mul(&py::mul(&Py::Int(3i64), &v_l.clone())?, &py::add(&v_l.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_Z.clone())?, &v_Z.clone())?)?;
    v_ir = py::truediv(&v_Z.clone(), &py::mul(&v_n.clone(), &v_n.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("r_mean_a0"), v_r.clone()), (Py::str("r2_mean_a0sq"), v_r2.clone()), (Py::str("inv_r_mean_per_a0"), v_ir.clone()), (Py::str("r_mean"), py::mul(&v_r.clone(), &c_A0()?)?), (Py::str("dr_a0"), py::math1("sqrt", &py::sub(&v_r2.clone(), &py::mul(&v_r.clone(), &v_r.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radial_probability(mut v_n: Py, mut v_l: Py, mut v_Z: Py, mut v_R: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_coef: Py = Py::None;
    let mut v_deg: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lag: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let mut v_total: Py = Py::None;
    let _ = f__check_nl(v_n.clone(), v_l.clone(), v_Z.clone())?;
    if ({ let t1302 = v_R.clone(); Py::Bool({ let t1303 = Py::Int(0i64); if py::le(&t1302, &t1303)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the radius must be positive"))));
    }
    v_deg = py::sub(&py::sub(&v_n.clone(), &v_l.clone())?, &Py::Int(1i64))?;
    v_lag = { let mut out = vec![]; for t1304 in py::range_iter(&[(py::add(&v_deg.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t1304;
out.push(py::fraction(&[py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_i.clone())?, &py::comb(&py::add(&v_n.clone(), &v_l.clone())?, &py::sub(&v_deg.clone(), &v_i.clone())?)?)?, py::factorial(&v_i.clone())?])?); } Py::list(out) };
    v_sq = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&py::mul(&Py::Int(2i64), &v_deg.clone())?, &Py::Int(1i64))?)?;
    for t1305 in py::iter(&(py::enumerate(&v_lag.clone(), 0)?))? {
        let t1310 = (t1305).items()?;
        if t1310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1310[0].clone();
        v_a = t1310[1].clone();
        for t1306 in py::iter(&(py::enumerate(&v_lag.clone(), 0)?))? {
            let t1309 = (t1306).items()?;
            if t1309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t1309[0].clone();
            v_b = t1309[1].clone();
            let t1307 = v_sq.clone(); let t1308 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t1307, &t1308, py::iadd(&py::getitem(&t1307, &t1308)?, &py::mul(&v_a.clone(), &v_b.clone())?)?)?;
        }
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_X = py::truediv(&py::mul(&py::mul(&ax_engine::pymp::call("mpf", &[Py::Int(2i64)], &[])?, &v_Z.clone())?, &ax_engine::pymp::call("mpf", &[v_R.clone()], &[])?)?, &py::mul(&v_n.clone(), &ax_engine::pymp::call("mpf", &[c_A0()?], &[])?)?)?;
        v_inner = ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])?;
        v_total = ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])?;
        for t1311 in py::iter(&(py::enumerate(&v_sq.clone(), 0)?))? {
            let t1314 = (t1311).items()?;
            if t1314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_i = t1314[0].clone();
            v_a = t1314[1].clone();
            if ({ let t1312 = v_a.clone(); Py::Bool({ let t1313 = Py::Int(0i64); if py::eq(&t1312, &t1313) { true } else { false } }) }).truthy() {
                continue;
            }
            v_c = py::add(&py::add(&py::mul(&Py::Int(2i64), &v_l.clone())?, &Py::Int(2i64))?, &v_i.clone())?;
            v_coef = py::truediv(&ax_engine::pymp::call("mpf", &[py::numerator(&v_a.clone())?], &[])?, &py::denominator(&v_a.clone())?)?;
            v_inner = py::iadd(&v_inner, &py::mul(&v_coef.clone(), &ax_engine::pymp::call("gammainc", &[py::add(&v_c.clone(), &Py::Int(1i64))?, Py::Int(0i64), v_X.clone()], &[])?)?)?;
            v_total = py::iadd(&v_total, &py::mul(&v_coef.clone(), &ax_engine::pymp::call("factorial", &[v_c.clone()], &[])?)?)?;
        }
        v_P = py::truediv(&v_inner.clone(), &v_total.clone())?;
        v_Q = py::sub(&Py::Int(1i64), &v_P.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("P_inside"), py::float(&v_P.clone())?), (Py::str("P_outside"), py::float(&v_Q.clone())?), (Py::str("R_a0"), py::truediv(&v_R.clone(), &c_A0()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_threshold(mut v_m_a: Py, mut v_m_X: Py, mut v_m_Y: Py, mut v_m_b: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_mf: Py = Py::None;
    let mut v_mi: Py = Py::None;
    if ({ let t1871 = { let t1867 = py::minmax(&[v_m_a.clone(), v_m_X.clone(), v_m_Y.clone(), v_m_b.clone()], false)?; Py::Bool({ let t1868 = Py::Int(0i64); if py::lt(&t1867, &t1868)? { true } else { false } }) }; if t1871.truthy() { t1871 } else { { let t1869 = v_m_X.clone(); Py::Bool({ let t1870 = Py::Int(0i64); if py::le(&t1869, &t1870)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("masses must be non-negative and the target mass positive"))));
    }
    v_mi = py::mul(&py::add(&v_m_a.clone(), &v_m_X.clone())?, &c_AMU_MEV()?)?;
    v_mf = py::mul(&py::add(&v_m_Y.clone(), &v_m_b.clone())?, &c_AMU_MEV()?)?;
    v_Q = py::mul(&py::sub(&py::add(&v_m_a.clone(), &v_m_X.clone())?, &py::add(&v_m_Y.clone(), &v_m_b.clone())?)?, &c_AMU_MEV()?)?;
    v_K = (if ({ let t1872 = v_Q.clone(); Py::Bool({ let t1873 = Py::Int(0i64); if py::ge(&t1872, &t1873)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&py::mul(&py::neg(&v_Q.clone())?, &py::add(&v_mf.clone(), &v_mi.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_m_X.clone())?, &c_AMU_MEV()?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("Q_MeV"), v_Q.clone()), (Py::str("K_threshold_MeV"), v_K.clone()), (Py::str("endothermic"), (if ({ let t1883 = v_Q.clone(); Py::Bool({ let t1884 = Py::Int(0i64); if py::lt(&t1883, &t1884)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_relativistic_doppler(mut v_f: Py, mut v_v: Py, mut v_motion: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    if ({ let t2452 = v_f.clone(); Py::Bool({ let t2453 = Py::Int(0i64); if py::le(&t2452, &t2453)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source frequency must be positive"))));
    }
    if ({ let t2454 = v_v.clone(); Py::Bool({ let t2455 = Py::Int(0i64); if py::lt(&t2454, &t2455)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the speed as a magnitude and the direction as approaching or receding"))));
    }
    v_b = f__beta(v_v.clone())?;
    v_ratio = (if ({ let t2459 = v_motion.clone(); Py::Bool({ let t2460 = Py::str("approaching"); if py::eq(&t2459, &t2460) { true } else { false } }) }).truthy() { py::math1("sqrt", &py::truediv(&py::add(&Py::Int(1i64), &v_b.clone())?, &py::sub(&Py::Int(1i64), &v_b.clone())?)?)? } else { py::math1("sqrt", &py::truediv(&py::sub(&Py::Int(1i64), &v_b.clone())?, &py::add(&Py::Int(1i64), &v_b.clone())?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("f_obs"), py::mul(&v_f.clone(), &v_ratio.clone())?), (Py::str("ratio"), v_ratio.clone()), (Py::str("z"), py::sub(&py::truediv(&Py::Int(1i64), &v_ratio.clone())?, &Py::Int(1i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rest_energy(mut v_m: Py) -> R {
    let mut v_E: Py = Py::None;
    if (py::not(&{ let t2655 = { let t2653 = v_m.clone(); Py::Bool({ let t2654 = Py::Int(0i64); if py::gt(&t2653, &t2654)? { true } else { false } }) }; if !t2655.truthy() { t2655 } else { py::math1("isfinite", &v_m.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass must be positive"))));
    }
    v_E = py::mul(&py::mul(&v_m.clone(), &c_C()?)?, &c_C()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_MeV"), py::truediv(&v_E.clone(), &c_MEV()?)?), (Py::str("E_kWh"), py::truediv(&v_E.clone(), &Py::Float(3600000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_semf(mut v_Z: Py, mut v_A: Py) -> R {
    return Ok(f__semf(v_Z.clone(), v_A.clone(), Py::Bool(true))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_qubit_circuit(mut v_start: Py, mut v_gates: Py, mut v_reverse: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_seq = f__gates_1q(v_gates.clone())?;
    v_v = Py::list((py::getitem(&c__START1()?, &v_start.clone())?).items()?);
    for t2762 in py::iter(&((if (v_reverse.clone()).truthy() { py::reversed(&v_seq.clone())? } else { v_seq.clone() })))? {
        let t2763 = (t2762).items()?;
        if t2763.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v__ = t2763[0].clone();
        v__ = t2763[1].clone();
        v_M = t2763[2].clone();
        v_v = f__mv(v_M.clone(), v_v.clone())?;
    }
    return Ok(f__one_qubit_result(py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spacetime_interval(mut v_dt: Py, mut v_dx: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_s2: Py = Py::None;
    if ({ let t2812 = py::not(&{ let t2806 = py::math1("isfinite", &v_dt.clone())?; if !t2806.truthy() { t2806 } else { py::math1("isfinite", &v_dx.clone())? } }); if t2812.truthy() { t2812 } else { { let t2811 = { let t2807 = v_dt.clone(); Py::Bool({ let t2808 = Py::Int(0i64); if py::eq(&t2807, &t2808) { true } else { false } }) }; if !t2811.truthy() { t2811 } else { { let t2809 = v_dx.clone(); Py::Bool({ let t2810 = Py::Int(0i64); if py::eq(&t2809, &t2810) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two events must be distinct"))));
    }
    let t2813 = (Py::tuple(vec![py::mul(&c_C()?, &py::abs(&v_dt.clone())?)?, py::abs(&v_dx.clone())?])).items()?;
    if t2813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2813[0].clone();
    v_b = t2813[1].clone();
    v_s2 = py::mul(&py::sub(&v_a.clone(), &v_b.clone())?, &py::add(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t2814 = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?; Py::Bool({ let t2815 = py::mul(&Py::Float(1e-12), &py::add(&v_a.clone(), &v_b.clone())?)?; if py::le(&t2814, &t2815)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("s2"), Py::Float(0.0)), (Py::str("nature"), Py::str("lightlike")), (Py::str("tau"), Py::Float(0.0)), (Py::str("proper_length"), Py::Float(0.0))]))?);
    }
    if ({ let t2825 = v_s2.clone(); Py::Bool({ let t2826 = Py::Int(0i64); if py::gt(&t2825, &t2826)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("s2"), v_s2.clone()), (Py::str("nature"), Py::str("timelike")), (Py::str("tau"), py::truediv(&py::math1("sqrt", &v_s2.clone())?, &c_C()?)?), (Py::str("proper_length"), Py::Float(0.0))]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("s2"), v_s2.clone()), (Py::str("nature"), Py::str("spacelike")), (Py::str("tau"), Py::Float(0.0)), (Py::str("proper_length"), py::math1("sqrt", &py::neg(&v_s2.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_speed_from_kinetic(mut v_m: Py, mut v_K: Py) -> R {
    let mut v_E0: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pc: Py = Py::None;
    if ({ let t2578 = { let t2574 = v_m.clone(); Py::Bool({ let t2575 = Py::Int(0i64); if py::le(&t2574, &t2575)? { true } else { false } }) }; if t2578.truthy() { t2578 } else { { let t2576 = v_K.clone(); Py::Bool({ let t2577 = Py::Int(0i64); if py::lt(&t2576, &t2577)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass must be positive and the kinetic energy non-negative"))));
    }
    v_E0 = py::mul(&py::mul(&v_m.clone(), &c_C()?)?, &c_C()?)?;
    v_g = py::add(&Py::Int(1i64), &py::truediv(&v_K.clone(), &v_E0.clone())?)?;
    v_pc = py::math1("sqrt", &py::mul(&v_K.clone(), &py::add(&v_K.clone(), &py::mul(&Py::Int(2i64), &v_E0.clone())?)?)?)?;
    v_b = py::truediv(&v_pc.clone(), &py::add(&v_K.clone(), &v_E0.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), py::mul(&v_b.clone(), &c_C()?)?), (Py::str("beta"), v_b.clone()), (Py::str("gamma"), v_g.clone()), (Py::str("p"), py::truediv(&v_pc.clone(), &c_C()?)?), (Py::str("pc_MeV"), py::truediv(&v_pc.clone(), &c_MEV()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spin_measurement(mut v_theta: Py, mut v_phi: Py, mut v_theta_n: Py, mut v_phi_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    for t2852 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_theta.clone(), Py::str("theta")]), Py::tuple(vec![v_theta_n.clone(), Py::str("theta_n")])])))? {
        let t2856 = (t2852).items()?;
        if t2856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t = t2856[0].clone();
        v_nm = t2856[1].clone();
        if (py::not(&{ let t2853 = Py::Int(0i64); Py::Bool({ let t2854 = v_t.clone(); if py::le(&t2853, &t2854)? { { let t2855 = Py::Float(std::f64::consts::PI); if py::le(&t2854, &t2855)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_nm.clone(), "")?, " (polar angle) must lie between 0 and 180 degrees".to_string()].concat()))));
        }
    }
    let t2857 = (Py::tuple(vec![f__bloch(v_theta.clone(), v_phi.clone())?, f__bloch(v_theta_n.clone(), v_phi_n.clone())?])).items()?;
    if t2857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2857[0].clone();
    v_n = t2857[1].clone();
    v_d = py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::sum(&{ let mut out = vec![]; for t2858 in py::iter(&(py::zip(&[v_r.clone(), v_n.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2859 = (t2858).items()?;
if t2859.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2859[0].clone();
v_b = t2859[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?], false)?], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_up"), f__snap(py::truediv(&py::add(&Py::Int(1i64), &v_d.clone())?, &Py::Int(2i64))?, Py::Float(1.0))?), (Py::str("P_down"), f__snap(py::truediv(&py::sub(&Py::Int(1i64), &v_d.clone())?, &Py::Int(2i64))?, Py::Float(1.0))?), (Py::str("expectation"), f__snap(v_d.clone(), Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stefan_boltzmann(mut v_T: Py, mut v_emissivity: Py, mut v_A: Py) -> R {
    let mut v_M: Py = Py::None;
    if ({ let t2966 = { let t2958 = v_T.clone(); Py::Bool({ let t2959 = Py::Int(0i64); if py::le(&t2958, &t2959)? { true } else { false } }) }; if t2966.truthy() { t2966 } else { { let t2965 = { let t2960 = v_A.clone(); Py::Bool({ let t2961 = Py::Int(0i64); if py::le(&t2960, &t2961)? { true } else { false } }) }; if t2965.truthy() { t2965 } else { py::not(&{ let t2962 = Py::Int(0i64); Py::Bool({ let t2963 = v_emissivity.clone(); if py::lt(&t2962, &t2963)? { { let t2964 = Py::Int(1i64); if py::le(&t2963, &t2964)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T and A must be positive and the emissivity between 0 and 1"))));
    }
    v_M = py::mul(&py::mul(&v_emissivity.clone(), &c_SIGMA_SB()?)?, &py::pow(&v_T.clone(), &Py::Int(4i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma"), c_SIGMA_SB()?), (Py::str("M"), v_M.clone()), (Py::str("P"), py::mul(&v_M.clone(), &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_step_potential(mut v_E: Py, mut v_V0: Py) -> R {
    let mut v_r: Py = Py::None;
    if (py::not(&{ let t3005 = { let t3001 = v_E.clone(); Py::Bool({ let t3002 = Py::Int(0i64); if py::gt(&t3001, &t3002)? { true } else { false } }) }; if !t3005.truthy() { t3005 } else { { let t3003 = v_V0.clone(); Py::Bool({ let t3004 = Py::Int(0i64); if py::gt(&t3003, &t3004)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the energy and the step height must be positive"))));
    }
    if ({ let t3006 = v_E.clone(); Py::Bool({ let t3007 = v_V0.clone(); if py::le(&t3006, &t3007)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("R"), Py::Float(1.0)), (Py::str("T"), Py::Float(0.0)), (Py::str("k2_over_k1"), Py::Float(0.0)), (Py::str("regime"), Py::str("total reflection (E <= V0)"))]))?);
    }
    v_r = py::math1("sqrt", &py::truediv(&py::sub(&v_E.clone(), &v_V0.clone())?, &v_E.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::pow(&py::truediv(&py::sub(&Py::Int(1i64), &v_r.clone())?, &py::add(&Py::Int(1i64), &v_r.clone())?)?, &Py::Int(2i64))?), (Py::str("T"), py::truediv(&py::mul(&Py::Int(4i64), &v_r.clone())?, &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &Py::Int(2i64))?)?), (Py::str("k2_over_k1"), v_r.clone()), (Py::str("regime"), Py::str("partial transmission (E > V0)"))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thermal_de_broglie(mut v_m: Py, mut v_T: Py) -> R {
    let mut v_lam: Py = Py::None;
    if ({ let t3046 = { let t3042 = v_m.clone(); Py::Bool({ let t3043 = Py::Int(0i64); if py::le(&t3042, &t3043)? { true } else { false } }) }; if t3046.truthy() { t3046 } else { { let t3044 = v_T.clone(); Py::Bool({ let t3045 = Py::Int(0i64); if py::le(&t3044, &t3045)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mass and the temperature must be positive"))));
    }
    v_lam = py::truediv(&c_H()?, &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &c_KB()?)?, &v_T.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_th"), v_lam.clone()), (Py::str("lambda_th_nm"), py::mul(&v_lam.clone(), &Py::Float(1000000000.0))?), (Py::str("n_Q"), py::pow(&v_lam.clone(), &py::neg(&Py::Int(3i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_time_dilation(mut v_t0: Py, mut v_v: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    if ({ let t3081 = v_t0.clone(); Py::Bool({ let t3082 = Py::Int(0i64); if py::le(&t3081, &t3082)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the proper time must be positive"))));
    }
    if ({ let t3083 = v_v.clone(); Py::Bool({ let t3084 = Py::Int(0i64); if py::lt(&t3083, &t3084)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the speed as a magnitude (v >= 0)"))));
    }
    v_b = f__beta(v_v.clone())?;
    v_g = f__gamma_of_beta(v_b.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), py::mul(&v_g.clone(), &v_t0.clone())?), (Py::str("gamma"), v_g.clone()), (Py::str("difference"), py::mul(&f__gm1(v_b.clone())?, &v_t0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_body_decay(mut v_M: Py, mut v_m1: Py, mut v_m2: Py, mut v_unit: Py) -> R {
    let mut v_E1: Py = Py::None;
    let mut v_E2: Py = Py::None;
    let mut v_K1: Py = Py::None;
    let mut v_K2: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_f = (if ({ let t3122 = v_unit.clone(); Py::Bool({ let t3123 = Py::str("u"); if py::eq(&t3122, &t3123) { true } else { false } }) }).truthy() { c_AMU_MEV()? } else { Py::Float(1.0) });
    let t3124 = (Py::tuple(vec![py::mul(&v_M.clone(), &v_f.clone())?, py::mul(&v_m1.clone(), &v_f.clone())?, py::mul(&v_m2.clone(), &v_f.clone())?])).items()?;
    if t3124.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_M = t3124[0].clone();
    v_m1 = t3124[1].clone();
    v_m2 = t3124[2].clone();
    if ({ let t3129 = { let t3125 = v_m1.clone(); Py::Bool({ let t3126 = Py::Int(0i64); if py::lt(&t3125, &t3126)? { true } else { false } }) }; if t3129.truthy() { t3129 } else { { let t3127 = v_m2.clone(); Py::Bool({ let t3128 = Py::Int(0i64); if py::lt(&t3127, &t3128)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the product masses must be non-negative"))));
    }
    v_Q = py::sub(&py::sub(&v_M.clone(), &v_m1.clone())?, &v_m2.clone())?;
    if ({ let t3130 = v_Q.clone(); Py::Bool({ let t3131 = Py::Int(0i64); if py::le(&t3130, &t3131)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the decay is forbidden: the parent is not heavier than the products"))));
    }
    v_p = py::truediv(&py::math1("sqrt", &py::mul(&py::mul(&py::mul(&v_Q.clone(), &py::add(&py::add(&v_M.clone(), &v_m1.clone())?, &v_m2.clone())?)?, &py::add(&py::sub(&v_M.clone(), &v_m1.clone())?, &v_m2.clone())?)?, &py::sub(&py::add(&v_M.clone(), &v_m1.clone())?, &v_m2.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_M.clone())?)?;
    let t3132 = (Py::tuple(vec![py::hypot(&[v_p.clone(), v_m1.clone()])?, py::hypot(&[v_p.clone(), v_m2.clone()])?])).items()?;
    if t3132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_E1 = t3132[0].clone();
    v_E2 = t3132[1].clone();
    v_K1 = py::truediv(&py::mul(&v_p.clone(), &v_p.clone())?, &py::add(&v_E1.clone(), &v_m1.clone())?)?;
    v_K2 = py::truediv(&py::mul(&v_p.clone(), &v_p.clone())?, &py::add(&v_E2.clone(), &v_m2.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q_MeV"), v_Q.clone()), (Py::str("p_MeV"), v_p.clone()), (Py::str("E1_MeV"), v_E1.clone()), (Py::str("E2_MeV"), v_E2.clone()), (Py::str("K1_MeV"), v_K1.clone()), (Py::str("K2_MeV"), v_K2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_level(mut v_dE: Py, mut v_T: Py, mut v_g0: Py, mut v_g1: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t3177 = { let t3167 = v_dE.clone(); Py::Bool({ let t3168 = Py::Int(0i64); if py::le(&t3167, &t3168)? { true } else { false } }) }; if t3177.truthy() { t3177 } else { { let t3176 = { let t3169 = v_T.clone(); Py::Bool({ let t3170 = Py::Int(0i64); if py::le(&t3169, &t3170)? { true } else { false } }) }; if t3176.truthy() { t3176 } else { { let t3175 = { let t3171 = v_g0.clone(); Py::Bool({ let t3172 = Py::Int(1i64); if py::lt(&t3171, &t3172)? { true } else { false } }) }; if t3175.truthy() { t3175 } else { { let t3173 = v_g1.clone(); Py::Bool({ let t3174 = Py::Int(1i64); if py::lt(&t3173, &t3174)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the gap and temperature must be positive and the degeneracies at least 1"))));
    }
    v_x = py::truediv(&v_dE.clone(), &py::mul(&c_KB()?, &v_T.clone())?)?;
    v_e = py::math1("exp", &py::neg(&v_x.clone())?)?;
    v_Z = py::add(&v_g0.clone(), &py::mul(&v_g1.clone(), &v_e.clone())?)?;
    v_p1 = py::truediv(&py::mul(&v_g1.clone(), &v_e.clone())?, &v_Z.clone())?;
    v_p0 = py::truediv(&v_g0.clone(), &v_Z.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Z"), v_Z.clone()), (Py::str("p0"), v_p0.clone()), (Py::str("p1"), v_p1.clone()), (Py::str("ratio"), py::mul(&py::truediv(&v_g1.clone(), &v_g0.clone())?, &v_e.clone())?), (Py::str("U_eV"), py::truediv(&py::mul(&v_dE.clone(), &v_p1.clone())?, &c_EV()?)?), (Py::str("C_over_k"), py::mul(&py::mul(&py::mul(&v_x.clone(), &v_x.clone())?, &v_p0.clone())?, &v_p1.clone())?), (Py::str("S_over_k"), py::add(&py::math1("log", &v_Z.clone())?, &py::mul(&v_x.clone(), &v_p1.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_qubit_circuit(mut v_start: Py, mut v_gates: Py, mut v_reverse: Py) -> R {
    let mut v_conc: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_seq = f__gates_2q(v_gates.clone())?;
    if (v_reverse.clone()).truthy() {
        v_seq = { let mut out = vec![]; for t3239 in py::iter(&(py::reversed(&v_seq.clone())?))? { let mut v_g: Py; 
v_g = t3239;
out.push(f__swap_labels(v_g.clone())?); } Py::list(out) };
    }
    v_v = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &Py::Int(4i64))?;
    py::setitem(&v_v.clone(), &py::int_base(&v_start.clone(), &Py::Int(2i64))?, Py::Float(1.0))?;
    for t3244 in py::iter(&(v_seq.clone()))? {
        v_g = t3244;
        v_v = f__apply2(v_g.clone(), v_v.clone())?;
    }
    v_p = { let mut out = vec![]; for t3260 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t3260;
out.push(py::pow(&py::abs(&v_a.clone())?, &Py::Int(2i64))?); } Py::list(out) };
    v_conc = py::mul(&Py::Int(2i64), &py::abs(&py::sub(&py::mul(&py::getitem(&v_v.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(3i64))?)?, &py::mul(&py::getitem(&v_v.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(2i64))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("p00"), f__snap(py::getitem(&v_p.clone(), &Py::Int(0i64))?, Py::Float(1.0))?), (Py::str("p01"), f__snap(py::getitem(&v_p.clone(), &Py::Int(1i64))?, Py::Float(1.0))?), (Py::str("p10"), f__snap(py::getitem(&v_p.clone(), &Py::Int(2i64))?, Py::Float(1.0))?), (Py::str("p11"), f__snap(py::getitem(&v_p.clone(), &Py::Int(3i64))?, Py::Float(1.0))?), (Py::str("concurrence"), f__snap(v_conc.clone(), Py::Float(1.0))?), (Py::str("p_q0_is_1"), f__snap(py::add(&py::getitem(&v_p.clone(), &Py::Int(2i64))?, &py::getitem(&v_p.clone(), &Py::Int(3i64))?)?, Py::Float(1.0))?), (Py::str("p_q1_is_1"), f__snap(py::add(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_p.clone(), &Py::Int(3i64))?)?, Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_velocity_addition(mut v_u_prime: Py, mut v_v: Py) -> R {
    let mut v_u: Py = Py::None;
    if (py::not(&{ let t2615 = py::math1("isfinite", &v_u_prime.clone())?; if !t2615.truthy() { t2615 } else { { let t2613 = py::abs(&v_u_prime.clone())?; Py::Bool({ let t2614 = c_C()?; if py::le(&t2613, &t2614)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the object's speed in the moving frame cannot exceed c"))));
    }
    let _ = f__beta(v_v.clone())?;
    v_u = py::truediv(&py::add(&v_u_prime.clone(), &v_v.clone())?, &py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_u_prime.clone(), &v_v.clone())?, &py::mul(&c_C()?, &c_C()?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("u"), v_u.clone()), (Py::str("beta"), py::truediv(&v_u.clone(), &c_C()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wien(mut v_T: Py) -> R {
    let mut v_b: Py = Py::None;
    if ({ let t3297 = v_T.clone(); Py::Bool({ let t3298 = Py::Int(0i64); if py::le(&t3297, &t3298)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature must be positive"))));
    }
    v_b = py::truediv(&py::mul(&c_H()?, &c_C()?)?, &py::mul(&c_X_WIEN_L()?, &c_KB()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_max"), py::truediv(&v_b.clone(), &v_T.clone())?), (Py::str("lambda_max_nm"), py::mul(&py::truediv(&v_b.clone(), &v_T.clone())?, &Py::Float(1000000000.0))?), (Py::str("nu_max"), py::truediv(&py::mul(&py::mul(&c_X_WIEN_F()?, &c_KB()?)?, &v_T.clone())?, &c_H()?)?), (Py::str("b"), v_b.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wigner_3j(mut v_j1: Py, mut v_m1: Py, mut v_j2: Py, mut v_m2: Py, mut v_J: Py, mut v_M: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let mut v_sq3: Py = Py::None;
    let t3367 = (f__cg_exact(v_j1.clone(), v_m1.clone(), v_j2.clone(), v_m2.clone(), v_J.clone(), py::neg(&v_M.clone())?)?).items()?;
    if t3367.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sign = t3367[0].clone();
    v_sq = t3367[1].clone();
    if ({ let t3368 = v_sq.clone(); Py::Bool({ let t3369 = Py::Int(0i64); if py::eq(&t3368, &t3369) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("w3j"), Py::Float(0.0)), (Py::str("w3j_squared"), Py::Float(0.0))]));
    }
    v_e = py::int(&py::sub(&py::sub(&f__half(v_j1.clone(), Py::str("j1"))?, &f__half(v_j2.clone(), Py::str("j2"))?)?, &f__half(v_M.clone(), Py::str("M"))?)?)?;
    v_sq3 = py::truediv(&v_sq.clone(), &py::add(&py::mul(&Py::Int(2i64), &f__half(v_J.clone(), Py::str("J"))?)?, &Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("w3j"), f__r(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_e.clone())?, &v_sign.clone())?, &py::math1("sqrt", &v_sq3.clone())?)?)?), (Py::str("w3j_squared"), f__r(py::float(&v_sq3.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wigner_small_d(mut v_j: Py, mut v_mp: Py, mut v_m: Py, mut v_beta: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dmm: Py = Py::None;
    let mut v_jj: Py = Py::None;
    let mut v_jm: Py = Py::None;
    let mut v_jp: Py = Py::None;
    let mut v_mm: Py = Py::None;
    let mut v_mmp: Py = Py::None;
    let mut v_mpm: Py = Py::None;
    let mut v_mpp: Py = Py::None;
    let mut v_pref: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sb: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let t2899 = (Py::tuple(vec![f__half(v_j.clone(), Py::str("j"))?, f__half(v_m.clone(), Py::str("m"))?, f__half(v_mp.clone(), Py::str("mp"))?])).items()?;
    if t2899.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_jj = t2899[0].clone();
    v_mm = t2899[1].clone();
    v_mmp = t2899[2].clone();
    if ({ let t2913 = { let t2900 = v_jj.clone(); Py::Bool({ let t2901 = Py::Int(0i64); if py::lt(&t2900, &t2901)? { true } else { false } }) }; if t2913.truthy() { t2913 } else { { let t2912 = { let t2902 = py::abs(&v_mm.clone())?; Py::Bool({ let t2903 = v_jj.clone(); if py::gt(&t2902, &t2903)? { true } else { false } }) }; if t2912.truthy() { t2912 } else { { let t2911 = { let t2904 = py::abs(&v_mmp.clone())?; Py::Bool({ let t2905 = v_jj.clone(); if py::gt(&t2904, &t2905)? { true } else { false } }) }; if t2911.truthy() { t2911 } else { { let t2910 = { let t2906 = py::denominator(&py::sub(&v_jj.clone(), &v_mm.clone())?)?; Py::Bool({ let t2907 = Py::Int(1i64); if py::ne(&t2906, &t2907) { true } else { false } }) }; if t2910.truthy() { t2910 } else { { let t2908 = py::denominator(&py::sub(&v_jj.clone(), &v_mmp.clone())?)?; Py::Bool({ let t2909 = Py::Int(1i64); if py::ne(&t2908, &t2909) { true } else { false } }) } } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need j >= 0, |m|, |m'| <= j with j - m and j - m' whole"))));
    }
    if ({ let t2914 = v_jj.clone(); Py::Bool({ let t2915 = Py::Int(20i64); if py::gt(&t2914, &t2915)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("j must be at most 20"))));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_b = ax_engine::pymp::call("mpf", &[v_beta.clone()], &[])?;
        let t2916 = (Py::tuple(vec![ax_engine::pymp::call("cos", &[py::truediv(&v_b.clone(), &Py::Int(2i64))?], &[])?, ax_engine::pymp::call("sin", &[py::truediv(&v_b.clone(), &Py::Int(2i64))?], &[])?])).items()?;
        if t2916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cb = t2916[0].clone();
        v_sb = t2916[1].clone();
        let t2917 = (Py::tuple(vec![py::int(&py::add(&v_jj.clone(), &v_mmp.clone())?)?, py::int(&py::sub(&v_jj.clone(), &v_mmp.clone())?)?, py::int(&py::add(&v_jj.clone(), &v_mm.clone())?)?, py::int(&py::sub(&v_jj.clone(), &v_mm.clone())?)?])).items()?;
        if t2917.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_jp = t2917[0].clone();
        v_jm = t2917[1].clone();
        v_mpp = t2917[2].clone();
        v_mpm = t2917[3].clone();
        v_pref = ax_engine::pymp::call("sqrt", &[py::mul(&py::mul(&py::mul(&ax_engine::pymp::call("factorial", &[v_jp.clone()], &[])?, &ax_engine::pymp::call("factorial", &[v_jm.clone()], &[])?)?, &ax_engine::pymp::call("factorial", &[v_mpp.clone()], &[])?)?, &ax_engine::pymp::call("factorial", &[v_mpm.clone()], &[])?)?], &[])?;
        v_tot = ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])?;
        v_dmm = py::int(&py::sub(&v_mmp.clone(), &v_mm.clone())?)?;
        for t2918 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&py::mul(&Py::Int(2i64), &py::int(&v_jj.clone())?)?, &Py::Int(2i64))?).i()?]) {
            v_s = t2918;
            v_a = Py::list(vec![py::sub(&py::int(&py::add(&v_jj.clone(), &v_mm.clone())?)?, &v_s.clone())?, v_s.clone(), py::add(&v_dmm.clone(), &v_s.clone())?, py::sub(&py::int(&py::sub(&v_jj.clone(), &v_mmp.clone())?)?, &v_s.clone())?]);
            if (Py::Bool(({ let mut out = vec![]; for t2919 in py::iter(&(v_a.clone()))? { let mut v_x: Py; 
v_x = t2919;
out.push({ let t2920 = v_x.clone(); Py::Bool({ let t2921 = Py::Int(0i64); if py::lt(&t2920, &t2921)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
                continue;
            }
            v_tot = py::iadd(&v_tot, &py::truediv(&py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::add(&v_dmm.clone(), &v_s.clone())?)?, &py::pow(&v_cb.clone(), &py::sub(&py::add(&py::int(&py::mul(&Py::Int(2i64), &v_jj.clone())?)?, &py::int(&py::sub(&v_mm.clone(), &v_mmp.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_s.clone())?)?)?)?, &py::pow(&v_sb.clone(), &py::add(&v_dmm.clone(), &py::mul(&Py::Int(2i64), &v_s.clone())?)?)?)?, &py::mul(&py::mul(&py::mul(&ax_engine::pymp::call("factorial", &[py::getitem(&v_a.clone(), &Py::Int(0i64))?], &[])?, &ax_engine::pymp::call("factorial", &[py::getitem(&v_a.clone(), &Py::Int(1i64))?], &[])?)?, &ax_engine::pymp::call("factorial", &[py::getitem(&v_a.clone(), &Py::Int(2i64))?], &[])?)?, &ax_engine::pymp::call("factorial", &[py::getitem(&v_a.clone(), &Py::Int(3i64))?], &[])?)?)?)?;
        }
        v_d = py::float(&py::mul(&v_pref.clone(), &v_tot.clone())?)?;
    }
    v_d = f__snap(v_d.clone(), Py::Float(1.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("probability"), py::mul(&v_d.clone(), &v_d.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wkb_triangular(mut v_W: Py, mut v_F: Py) -> R {
    let mut v_b: Py = Py::None;
    if ({ let t3406 = { let t3402 = v_W.clone(); Py::Bool({ let t3403 = Py::Int(0i64); if py::le(&t3402, &t3403)? { true } else { false } }) }; if t3406.truthy() { t3406 } else { { let t3404 = v_F.clone(); Py::Bool({ let t3405 = Py::Int(0i64); if py::le(&t3404, &t3405)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the barrier height (work function) and the field must be positive"))));
    }
    v_b = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &py::math1("sqrt", &py::mul(&Py::Int(2i64), &c_ME()?)?)?)?, &py::pow(&v_W.clone(), &Py::Float(1.5))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(3i64), &c_QE()?)?, &c_HBAR()?)?, &v_F.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("exponent"), v_b.clone()), (Py::str("T"), py::math1("exp", &py::neg(&v_b.clone())?)?), (Py::str("log10_T"), py::truediv(&py::neg(&v_b.clone())?, &py::math1("log", &Py::Int(10i64))?)?), (Py::str("width"), py::truediv(&v_W.clone(), &py::mul(&c_QE()?, &v_F.clone())?)?), (Py::str("width_nm"), py::mul(&py::truediv(&v_W.clone(), &py::mul(&c_QE()?, &v_F.clone())?)?, &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1048(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1050: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1049 = v_inputs.clone(); f_gate_rotation(py::getitem(&t1049, &Py::str("gates"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1050 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1051 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1072 = (t1051).items()?;
        if t1072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1072[0].clone();
        v_v = t1072[1].clone();
        if ({ let t1052 = v_k.clone(); Py::Bool({ let t1053 = v_outputs.clone(); if !py::contains(&t1053, &t1052)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1097(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1099: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1098 = v_inputs.clone(); f_grover(py::getitem(&t1098, &Py::str("n_qubits"))?, py::getitem(&t1098, &Py::str("M"))?, py::getitem(&t1098, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1099 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1100 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1121 = (t1100).items()?;
        if t1121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1121[0].clone();
        v_v = t1121[1].clone();
        if ({ let t1101 = v_k.clone(); Py::Bool({ let t1102 = v_outputs.clone(); if !py::contains(&t1102, &t1101)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1154 = v_inputs.clone(); f_ho_forbidden(py::getitem(&t1154, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1156 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1177 = (t1156).items()?;
        if t1177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1177[0].clone();
        v_v = t1177[1].clone();
        if ({ let t1157 = v_k.clone(); Py::Bool({ let t1158 = v_outputs.clone(); if !py::contains(&t1158, &t1157)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1194(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1196: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1195 = v_inputs.clone(); f_ho_levels(py::getitem(&t1195, &Py::str("m"))?, py::getitem(&t1195, &Py::str("omega"))?, py::getitem(&t1195, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1196 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1197 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1218 = (t1197).items()?;
        if t1218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1218[0].clone();
        v_v = t1218[1].clone();
        if ({ let t1198 = v_k.clone(); Py::Bool({ let t1199 = v_outputs.clone(); if !py::contains(&t1199, &t1198)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1234 = v_inputs.clone(); f_heisenberg(py::getitem(&t1234, &Py::str("dx"))?, py::getitem(&t1234, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1236 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1257 = (t1236).items()?;
        if t1257.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1257[0].clone();
        v_v = t1257[1].clone();
        if ({ let t1237 = v_k.clone(); Py::Bool({ let t1238 = v_outputs.clone(); if !py::contains(&t1238, &t1237)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1272(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1274: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1273 = v_inputs.clone(); f_radial_expectations(py::getitem(&t1273, &Py::str("n"))?, py::getitem(&t1273, &Py::str("l"))?, py::getitem(&t1273, &Py::str("Z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1274 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1275 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1296 = (t1275).items()?;
        if t1296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1296[0].clone();
        v_v = t1296[1].clone();
        if ({ let t1276 = v_k.clone(); Py::Bool({ let t1277 = v_outputs.clone(); if !py::contains(&t1277, &t1276)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t134: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t133 = v_inputs.clone(); f_band_fraction(py::getitem(&t133, &Py::str("lam1"))?, py::getitem(&t133, &Py::str("lam2"))?, py::getitem(&t133, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t134 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t135 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t156 = (t135).items()?;
        if t156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t156[0].clone();
        v_v = t156[1].clone();
        if ({ let t136 = v_k.clone(); Py::Bool({ let t137 = v_outputs.clone(); if !py::contains(&t137, &t136)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1324(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1326: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1325 = v_inputs.clone(); f_radial_probability(py::getitem(&t1325, &Py::str("n"))?, py::getitem(&t1325, &Py::str("l"))?, py::getitem(&t1325, &Py::str("Z"))?, py::getitem(&t1325, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1326 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1327 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1348 = (t1327).items()?;
        if t1348.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1348[0].clone();
        v_v = t1348[1].clone();
        if ({ let t1328 = v_k.clone(); Py::Bool({ let t1329 = v_outputs.clone(); if !py::contains(&t1329, &t1328)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1363(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1365: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1364 = v_inputs.clone(); f_hydrogen_transition(py::getitem(&t1364, &Py::str("Z"))?, py::getitem(&t1364, &Py::str("n_upper"))?, py::getitem(&t1364, &Py::str("n_lower"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1365 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1366 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1387 = (t1366).items()?;
        if t1387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1387[0].clone();
        v_v = t1387[1].clone();
        if ({ let t1367 = v_k.clone(); Py::Bool({ let t1368 = v_outputs.clone(); if !py::contains(&t1368, &t1367)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1403(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1405: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1404 = v_inputs.clone(); f_hydrogen_transition_reduced(py::getitem(&t1404, &Py::str("Z"))?, py::getitem(&t1404, &Py::str("n_upper"))?, py::getitem(&t1404, &Py::str("n_lower"))?, py::getitem(&t1404, &Py::str("M_u"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1405 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1406 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1427 = (t1406).items()?;
        if t1427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1427[0].clone();
        v_v = t1427[1].clone();
        if ({ let t1407 = v_k.clone(); Py::Bool({ let t1408 = v_outputs.clone(); if !py::contains(&t1408, &t1407)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1444(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1446: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1445 = v_inputs.clone(); f_infinite_well_levels(py::getitem(&t1445, &Py::str("m"))?, py::getitem(&t1445, &Py::str("L"))?, py::getitem(&t1445, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1446 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1447 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1468 = (t1447).items()?;
        if t1468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1468[0].clone();
        v_v = t1468[1].clone();
        if ({ let t1448 = v_k.clone(); Py::Bool({ let t1449 = v_outputs.clone(); if !py::contains(&t1449, &t1448)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1489(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1490 = v_inputs.clone(); f_infinite_well_probability(py::getitem(&t1490, &Py::str("n"))?, py::getitem(&t1490, &Py::str("L"))?, py::getitem(&t1490, &Py::str("x1"))?, py::getitem(&t1490, &Py::str("x2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1491 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1492 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1513 = (t1492).items()?;
        if t1513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1513[0].clone();
        v_v = t1513[1].clone();
        if ({ let t1493 = v_k.clone(); Py::Bool({ let t1494 = v_outputs.clone(); if !py::contains(&t1494, &t1493)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1532(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1533 = v_inputs.clone(); f_infinite_well_transition(py::getitem(&t1533, &Py::str("m"))?, py::getitem(&t1533, &Py::str("L"))?, py::getitem(&t1533, &Py::str("n_upper"))?, py::getitem(&t1533, &Py::str("n_lower"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1534 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1535 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1556 = (t1535).items()?;
        if t1556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1556[0].clone();
        v_v = t1556[1].clone();
        if ({ let t1536 = v_k.clone(); Py::Bool({ let t1537 = v_outputs.clone(); if !py::contains(&t1537, &t1536)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1570(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1572: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1571 = v_inputs.clone(); f_infinite_well_expectations(py::getitem(&t1571, &Py::str("n"))?, py::getitem(&t1571, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1572 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1573 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1594 = (t1573).items()?;
        if t1594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1594[0].clone();
        v_v = t1594[1].clone();
        if ({ let t1574 = v_k.clone(); Py::Bool({ let t1575 = v_outputs.clone(); if !py::contains(&t1575, &t1574)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1621(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1622 = v_inputs.clone(); f_invariant_mass(py::getitem(&t1622, &Py::str("E1"))?, py::getitem(&t1622, &Py::str("p1"))?, py::getitem(&t1622, &Py::str("E2"))?, py::getitem(&t1622, &Py::str("p2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1624 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1645 = (t1624).items()?;
        if t1645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1645[0].clone();
        v_v = t1645[1].clone();
        if ({ let t1625 = v_k.clone(); Py::Bool({ let t1626 = v_outputs.clone(); if !py::contains(&t1626, &t1625)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_168(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t169 = v_inputs.clone(); f_bohr_level(py::getitem(&t169, &Py::str("Z"))?, py::getitem(&t169, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t170 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t171 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t192 = (t171).items()?;
        if t192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t192[0].clone();
        v_v = t192[1].clone();
        if ({ let t172 = v_k.clone(); Py::Bool({ let t173 = v_outputs.clone(); if !py::contains(&t173, &t172)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1685(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1687: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1686 = v_inputs.clone(); f_lande_zeeman(py::getitem(&t1686, &Py::str("L"))?, py::getitem(&t1686, &Py::str("S"))?, py::getitem(&t1686, &Py::str("J"))?, py::getitem(&t1686, &Py::str("mJ"))?, py::getitem(&t1686, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1687 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1688 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1709 = (t1688).items()?;
        if t1709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1709[0].clone();
        v_v = t1709[1].clone();
        if ({ let t1689 = v_k.clone(); Py::Bool({ let t1690 = v_outputs.clone(); if !py::contains(&t1690, &t1689)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1728: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1727 = v_inputs.clone(); f_larmor(py::getitem(&t1727, &Py::str("particle"))?, py::getitem(&t1727, &Py::str("B"))?, py::getitem(&t1727, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1728 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1729 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1750 = (t1729).items()?;
        if t1750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1750[0].clone();
        v_v = t1750[1].clone();
        if ({ let t1730 = v_k.clone(); Py::Bool({ let t1731 = v_outputs.clone(); if !py::contains(&t1731, &t1730)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1767(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1769: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1768 = v_inputs.clone(); f_length_contraction(py::getitem(&t1768, &Py::str("L0"))?, py::getitem(&t1768, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1769 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1770 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1791 = (t1770).items()?;
        if t1791.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1791[0].clone();
        v_v = t1791[1].clone();
        if ({ let t1771 = v_k.clone(); Py::Bool({ let t1772 = v_outputs.clone(); if !py::contains(&t1772, &t1771)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1808: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1807 = v_inputs.clone(); f_lorentz_factor(py::getitem(&t1807, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1808 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1809 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1830 = (t1809).items()?;
        if t1830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1830[0].clone();
        v_v = t1830[1].clone();
        if ({ let t1810 = v_k.clone(); Py::Bool({ let t1811 = v_outputs.clone(); if !py::contains(&t1811, &t1810)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1842(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1844: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1843 = v_inputs.clone(); f_moseley(py::getitem(&t1843, &Py::str("Z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1844 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1845 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1866 = (t1845).items()?;
        if t1866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1866[0].clone();
        v_v = t1866[1].clone();
        if ({ let t1846 = v_k.clone(); Py::Bool({ let t1847 = v_outputs.clone(); if !py::contains(&t1847, &t1846)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1885(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1887: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1886 = v_inputs.clone(); f_reaction_threshold(py::getitem(&t1886, &Py::str("m_a"))?, py::getitem(&t1886, &Py::str("m_X"))?, py::getitem(&t1886, &Py::str("m_Y"))?, py::getitem(&t1886, &Py::str("m_b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1887 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1909 = (t1888).items()?;
        if t1909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1909[0].clone();
        v_v = t1909[1].clone();
        if ({ let t1889 = v_k.clone(); Py::Bool({ let t1890 = v_outputs.clone(); if !py::contains(&t1890, &t1889)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1921(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1923: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1922 = v_inputs.clone(); f_nuclide_decay(py::getitem(&t1922, &Py::str("nuclide"))?, py::getitem(&t1922, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1923 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1924 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1945 = (t1924).items()?;
        if t1945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1945[0].clone();
        v_v = t1945[1].clone();
        if ({ let t1925 = v_k.clone(); Py::Bool({ let t1926 = v_outputs.clone(); if !py::contains(&t1926, &t1925)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1958(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1959 = v_inputs.clone(); f_decay_length(py::getitem(&t1959, &Py::str("particle"))?, py::getitem(&t1959, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1960 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1961 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1982 = (t1961).items()?;
        if t1982.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1982[0].clone();
        v_v = t1982[1].clone();
        if ({ let t1962 = v_k.clone(); Py::Bool({ let t1963 = v_outputs.clone(); if !py::contains(&t1963, &t1962)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1999(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2001: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2000 = v_inputs.clone(); f_photoelectric(py::getitem(&t2000, &Py::str("lam"))?, py::getitem(&t2000, &Py::str("W"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2001 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2002 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2023 = (t2002).items()?;
        if t2023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2023[0].clone();
        v_v = t2023[1].clone();
        if ({ let t2003 = v_k.clone(); Py::Bool({ let t2004 = v_outputs.clone(); if !py::contains(&t2004, &t2003)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2036(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2038: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2037 = v_inputs.clone(); f_photon_energy(py::getitem(&t2037, &Py::str("lam"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2038 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2039 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2060 = (t2039).items()?;
        if t2060.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2060[0].clone();
        v_v = t2060[1].clone();
        if ({ let t2040 = v_k.clone(); Py::Bool({ let t2041 = v_outputs.clone(); if !py::contains(&t2041, &t2040)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2072(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2074: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2073 = v_inputs.clone(); f_photon_gas(py::getitem(&t2073, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2074 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2075 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2096 = (t2075).items()?;
        if t2096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2096[0].clone();
        v_v = t2096[1].clone();
        if ({ let t2076 = v_k.clone(); Py::Bool({ let t2077 = v_outputs.clone(); if !py::contains(&t2077, &t2076)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_208(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t210: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t209 = v_inputs.clone(); f_bose_einstein(py::getitem(&t209, &Py::str("E"))?, py::getitem(&t209, &Py::str("mu"))?, py::getitem(&t209, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t210 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t211 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t232 = (t211).items()?;
        if t232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t232[0].clone();
        v_v = t232[1].clone();
        if ({ let t212 = v_k.clone(); Py::Bool({ let t213 = v_outputs.clone(); if !py::contains(&t213, &t212)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2115: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2114 = v_inputs.clone(); f_planck_radiance(py::getitem(&t2114, &Py::str("lam"))?, py::getitem(&t2114, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2115 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2116 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2137 = (t2116).items()?;
        if t2137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2137[0].clone();
        v_v = t2137[1].clone();
        if ({ let t2117 = v_k.clone(); Py::Bool({ let t2118 = v_outputs.clone(); if !py::contains(&t2118, &t2117)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2158(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2160: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2159 = v_inputs.clone(); f_qubit_entropy(py::getitem(&t2159, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2160 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2161 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2182 = (t2161).items()?;
        if t2182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2182[0].clone();
        v_v = t2182[1].clone();
        if ({ let t2162 = v_k.clone(); Py::Bool({ let t2163 = v_outputs.clone(); if !py::contains(&t2163, &t2162)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2210(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2212: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2211 = v_inputs.clone(); f_qubit_fidelity(py::getitem(&t2211, &Py::str("r"))?, py::getitem(&t2211, &Py::str("s"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2212 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2213 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2234 = (t2213).items()?;
        if t2234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2234[0].clone();
        v_v = t2234[1].clone();
        if ({ let t2214 = v_k.clone(); Py::Bool({ let t2215 = v_outputs.clone(); if !py::contains(&t2215, &t2214)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2262: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2261 = v_inputs.clone(); f_qubit_state(py::getitem(&t2261, &Py::str("a_re"))?, py::getitem(&t2261, &Py::str("a_im"))?, py::getitem(&t2261, &Py::str("b_re"))?, py::getitem(&t2261, &Py::str("b_im"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2262 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2263 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2284 = (t2263).items()?;
        if t2284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2284[0].clone();
        v_v = t2284[1].clone();
        if ({ let t2264 = v_k.clone(); Py::Bool({ let t2265 = v_outputs.clone(); if !py::contains(&t2265, &t2264)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2301(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2303: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2302 = v_inputs.clone(); f_rabi(py::getitem(&t2302, &Py::str("Omega"))?, py::getitem(&t2302, &Py::str("Delta"))?, py::getitem(&t2302, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2303 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2304 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2325 = (t2304).items()?;
        if t2325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2325[0].clone();
        v_v = t2325[1].clone();
        if ({ let t2305 = v_k.clone(); Py::Bool({ let t2306 = v_outputs.clone(); if !py::contains(&t2306, &t2305)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2343(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2345: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2344 = v_inputs.clone(); f_decay_age(py::getitem(&t2344, &Py::str("fraction"))?, py::getitem(&t2344, &Py::str("T_half"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2345 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2346 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2367 = (t2346).items()?;
        if t2367.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2367[0].clone();
        v_v = t2367[1].clone();
        if ({ let t2347 = v_k.clone(); Py::Bool({ let t2348 = v_outputs.clone(); if !py::contains(&t2348, &t2347)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2385(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2387: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2386 = v_inputs.clone(); f_decay_law(py::getitem(&t2386, &Py::str("T_half"))?, py::getitem(&t2386, &Py::str("N0"))?, py::getitem(&t2386, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2387 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2388 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2409 = (t2388).items()?;
        if t2409.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2409[0].clone();
        v_v = t2409[1].clone();
        if ({ let t2389 = v_k.clone(); Py::Bool({ let t2390 = v_outputs.clone(); if !py::contains(&t2390, &t2389)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2427(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2429: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2428 = v_inputs.clone(); f_aberration(py::getitem(&t2428, &Py::str("theta"))?, py::getitem(&t2428, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2429 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2430 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2451 = (t2430).items()?;
        if t2451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2451[0].clone();
        v_v = t2451[1].clone();
        if ({ let t2431 = v_k.clone(); Py::Bool({ let t2432 = v_outputs.clone(); if !py::contains(&t2432, &t2431)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t248: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t247 = v_inputs.clone(); f_chsh(py::getitem(&t247, &Py::str("a"))?, py::getitem(&t247, &Py::str("a2"))?, py::getitem(&t247, &Py::str("b"))?, py::getitem(&t247, &Py::str("b2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t248 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t249 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t270 = (t249).items()?;
        if t270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t270[0].clone();
        v_v = t270[1].clone();
        if ({ let t250 = v_k.clone(); Py::Bool({ let t251 = v_outputs.clone(); if !py::contains(&t251, &t250)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2471 = v_inputs.clone(); f_relativistic_doppler(py::getitem(&t2471, &Py::str("f"))?, py::getitem(&t2471, &Py::str("v"))?, py::getitem(&t2471, &Py::str("motion"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2472 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2473 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2494 = (t2473).items()?;
        if t2494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2494[0].clone();
        v_v = t2494[1].clone();
        if ({ let t2474 = v_k.clone(); Py::Bool({ let t2475 = v_outputs.clone(); if !py::contains(&t2475, &t2474)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2508(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2510: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2509 = v_inputs.clone(); f_energy_from_momentum(py::getitem(&t2509, &Py::str("m"))?, py::getitem(&t2509, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2510 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2511 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2532 = (t2511).items()?;
        if t2532.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2532[0].clone();
        v_v = t2532[1].clone();
        if ({ let t2512 = v_k.clone(); Py::Bool({ let t2513 = v_outputs.clone(); if !py::contains(&t2513, &t2512)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2549(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2551: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2550 = v_inputs.clone(); f_momentum_energy(py::getitem(&t2550, &Py::str("m"))?, py::getitem(&t2550, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2551 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2552 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2573 = (t2552).items()?;
        if t2573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2573[0].clone();
        v_v = t2573[1].clone();
        if ({ let t2553 = v_k.clone(); Py::Bool({ let t2554 = v_outputs.clone(); if !py::contains(&t2554, &t2553)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2588(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2590: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2589 = v_inputs.clone(); f_speed_from_kinetic(py::getitem(&t2589, &Py::str("m"))?, py::getitem(&t2589, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2590 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2591 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2612 = (t2591).items()?;
        if t2612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2612[0].clone();
        v_v = t2612[1].clone();
        if ({ let t2592 = v_k.clone(); Py::Bool({ let t2593 = v_outputs.clone(); if !py::contains(&t2593, &t2592)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2628(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2630: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2629 = v_inputs.clone(); f_velocity_addition(py::getitem(&t2629, &Py::str("u_prime"))?, py::getitem(&t2629, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2630 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2631 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2652 = (t2631).items()?;
        if t2652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2652[0].clone();
        v_v = t2652[1].clone();
        if ({ let t2632 = v_k.clone(); Py::Bool({ let t2633 = v_outputs.clone(); if !py::contains(&t2633, &t2632)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2665(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2667: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2666 = v_inputs.clone(); f_rest_energy(py::getitem(&t2666, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2667 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2668 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2689 = (t2668).items()?;
        if t2689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2689[0].clone();
        v_v = t2689[1].clone();
        if ({ let t2669 = v_k.clone(); Py::Bool({ let t2670 = v_outputs.clone(); if !py::contains(&t2670, &t2669)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2711(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2713: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2712 = v_inputs.clone(); f_semf(py::getitem(&t2712, &Py::str("Z"))?, py::getitem(&t2712, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2713 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2714 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2735 = (t2714).items()?;
        if t2735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2735[0].clone();
        v_v = t2735[1].clone();
        if ({ let t2715 = v_k.clone(); Py::Bool({ let t2716 = v_outputs.clone(); if !py::contains(&t2716, &t2715)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2781(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2783: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2782 = v_inputs.clone(); f_single_qubit_circuit(py::getitem(&t2782, &Py::str("start"))?, py::getitem(&t2782, &Py::str("gates"))?, py::method(&t2782, "get", &[Py::str("reverse"), Py::Bool(false)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2783 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2784 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2805 = (t2784).items()?;
        if t2805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2805[0].clone();
        v_v = t2805[1].clone();
        if ({ let t2785 = v_k.clone(); Py::Bool({ let t2786 = v_outputs.clone(); if !py::contains(&t2786, &t2785)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2827(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2829: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2828 = v_inputs.clone(); f_spacetime_interval(py::getitem(&t2828, &Py::str("dt"))?, py::getitem(&t2828, &Py::str("dx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2829 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2830 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2851 = (t2830).items()?;
        if t2851.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2851[0].clone();
        v_v = t2851[1].clone();
        if ({ let t2831 = v_k.clone(); Py::Bool({ let t2832 = v_outputs.clone(); if !py::contains(&t2832, &t2831)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2871(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2873: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2872 = v_inputs.clone(); f_spin_measurement(py::getitem(&t2872, &Py::str("theta"))?, py::getitem(&t2872, &Py::str("phi"))?, py::getitem(&t2872, &Py::str("theta_n"))?, py::getitem(&t2872, &Py::str("phi_n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2873 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2874 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2895 = (t2874).items()?;
        if t2895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2895[0].clone();
        v_v = t2895[1].clone();
        if ({ let t2875 = v_k.clone(); Py::Bool({ let t2876 = v_outputs.clone(); if !py::contains(&t2876, &t2875)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2933(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2935: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2934 = v_inputs.clone(); f_wigner_small_d(py::getitem(&t2934, &Py::str("j"))?, py::getitem(&t2934, &Py::str("mp"))?, py::getitem(&t2934, &Py::str("m"))?, py::getitem(&t2934, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2935 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2936 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2957 = (t2936).items()?;
        if t2957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2957[0].clone();
        v_v = t2957[1].clone();
        if ({ let t2937 = v_k.clone(); Py::Bool({ let t2938 = v_outputs.clone(); if !py::contains(&t2938, &t2937)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2976(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2978: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2977 = v_inputs.clone(); f_stefan_boltzmann(py::getitem(&t2977, &Py::str("T"))?, py::getitem(&t2977, &Py::str("emissivity"))?, py::getitem(&t2977, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2978 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2979 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3000 = (t2979).items()?;
        if t3000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3000[0].clone();
        v_v = t3000[1].clone();
        if ({ let t2980 = v_k.clone(); Py::Bool({ let t2981 = v_outputs.clone(); if !py::contains(&t2981, &t2980)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3019: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3018 = v_inputs.clone(); f_step_potential(py::getitem(&t3018, &Py::str("E"))?, py::getitem(&t3018, &Py::str("V0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3019 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3020 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3041 = (t3020).items()?;
        if t3041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3041[0].clone();
        v_v = t3041[1].clone();
        if ({ let t3021 = v_k.clone(); Py::Bool({ let t3022 = v_outputs.clone(); if !py::contains(&t3022, &t3021)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3056(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3058: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3057 = v_inputs.clone(); f_thermal_de_broglie(py::getitem(&t3057, &Py::str("m"))?, py::getitem(&t3057, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3058 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3059 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3080 = (t3059).items()?;
        if t3080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3080[0].clone();
        v_v = t3080[1].clone();
        if ({ let t3060 = v_k.clone(); Py::Bool({ let t3061 = v_outputs.clone(); if !py::contains(&t3061, &t3060)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3097(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3099: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3098 = v_inputs.clone(); f_time_dilation(py::getitem(&t3098, &Py::str("t0"))?, py::getitem(&t3098, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3099 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3100 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3121 = (t3100).items()?;
        if t3121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3121[0].clone();
        v_v = t3121[1].clone();
        if ({ let t3101 = v_k.clone(); Py::Bool({ let t3102 = v_outputs.clone(); if !py::contains(&t3102, &t3101)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3142(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3144: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3143 = v_inputs.clone(); f_two_body_decay(py::getitem(&t3143, &Py::str("M"))?, py::getitem(&t3143, &Py::str("m1"))?, py::getitem(&t3143, &Py::str("m2"))?, py::method(&t3143, "get", &[Py::str("unit"), Py::str("MeV")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3144 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3145 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3166 = (t3145).items()?;
        if t3166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3166[0].clone();
        v_v = t3166[1].clone();
        if ({ let t3146 = v_k.clone(); Py::Bool({ let t3147 = v_outputs.clone(); if !py::contains(&t3147, &t3146)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_315(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t317: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t316 = v_inputs.clone(); f_clebsch_gordan(py::getitem(&t316, &Py::str("j1"))?, py::getitem(&t316, &Py::str("m1"))?, py::getitem(&t316, &Py::str("j2"))?, py::getitem(&t316, &Py::str("m2"))?, py::getitem(&t316, &Py::str("J"))?, py::getitem(&t316, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t317 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t318 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t339 = (t318).items()?;
        if t339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t339[0].clone();
        v_v = t339[1].clone();
        if ({ let t319 = v_k.clone(); Py::Bool({ let t320 = v_outputs.clone(); if !py::contains(&t320, &t319)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3187(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3189: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3188 = v_inputs.clone(); f_two_level(py::getitem(&t3188, &Py::str("dE"))?, py::getitem(&t3188, &Py::str("T"))?, py::getitem(&t3188, &Py::str("g0"))?, py::getitem(&t3188, &Py::str("g1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3189 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3190 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3211 = (t3190).items()?;
        if t3211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3211[0].clone();
        v_v = t3211[1].clone();
        if ({ let t3191 = v_k.clone(); Py::Bool({ let t3192 = v_outputs.clone(); if !py::contains(&t3192, &t3191)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3272(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3274: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3273 = v_inputs.clone(); f_two_qubit_circuit(py::getitem(&t3273, &Py::str("start"))?, py::getitem(&t3273, &Py::str("gates"))?, py::method(&t3273, "get", &[Py::str("reverse"), Py::Bool(false)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3274 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3275 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3296 = (t3275).items()?;
        if t3296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3296[0].clone();
        v_v = t3296[1].clone();
        if ({ let t3276 = v_k.clone(); Py::Bool({ let t3277 = v_outputs.clone(); if !py::contains(&t3277, &t3276)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3308(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3310: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3309 = v_inputs.clone(); f_wien(py::getitem(&t3309, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3310 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3311 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3332 = (t3311).items()?;
        if t3332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3332[0].clone();
        v_v = t3332[1].clone();
        if ({ let t3312 = v_k.clone(); Py::Bool({ let t3313 = v_outputs.clone(); if !py::contains(&t3313, &t3312)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3377(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3379: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3378 = v_inputs.clone(); f_wigner_3j(py::getitem(&t3378, &Py::str("j1"))?, py::getitem(&t3378, &Py::str("m1"))?, py::getitem(&t3378, &Py::str("j2"))?, py::getitem(&t3378, &Py::str("m2"))?, py::getitem(&t3378, &Py::str("J"))?, py::getitem(&t3378, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3379 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3380 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3401 = (t3380).items()?;
        if t3401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3401[0].clone();
        v_v = t3401[1].clone();
        if ({ let t3381 = v_k.clone(); Py::Bool({ let t3382 = v_outputs.clone(); if !py::contains(&t3382, &t3381)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3416(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3418: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3417 = v_inputs.clone(); f_wkb_triangular(py::getitem(&t3417, &Py::str("W"))?, py::getitem(&t3417, &Py::str("F"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3418 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3419 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3440 = (t3419).items()?;
        if t3440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3440[0].clone();
        v_v = t3440[1].clone();
        if ({ let t3420 = v_k.clone(); Py::Bool({ let t3421 = v_outputs.clone(); if !py::contains(&t3421, &t3420)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_357(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t359: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t358 = v_inputs.clone(); f_coherent_state(py::getitem(&t358, &Py::str("alpha"))?, py::getitem(&t358, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t359 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t360 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t381 = (t360).items()?;
        if t381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t381[0].clone();
        v_v = t381[1].clone();
        if ({ let t361 = v_k.clone(); Py::Bool({ let t362 = v_outputs.clone(); if !py::contains(&t362, &t361)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_39(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t41: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t40 = v_inputs.clone(); f_barrier_transmission(py::getitem(&t40, &Py::str("E"))?, py::getitem(&t40, &Py::str("V0"))?, py::getitem(&t40, &Py::str("a"))?, py::getitem(&t40, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t41 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t42 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t63 = (t42).items()?;
        if t63.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t63[0].clone();
        v_v = t63[1].clone();
        if ({ let t43 = v_k.clone(); Py::Bool({ let t44 = v_outputs.clone(); if !py::contains(&t44, &t43)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_397(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t399: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t398 = v_inputs.clone(); f_compton(py::getitem(&t398, &Py::str("lam"))?, py::getitem(&t398, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t399 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t400 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t421 = (t400).items()?;
        if t421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t421[0].clone();
        v_v = t421[1].clone();
        if ({ let t401 = v_k.clone(); Py::Bool({ let t402 = v_outputs.clone(); if !py::contains(&t402, &t401)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_442(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t444: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t443 = v_inputs.clone(); f_coulomb_barrier(py::getitem(&t443, &Py::str("Z1"))?, py::getitem(&t443, &Py::str("A1"))?, py::getitem(&t443, &Py::str("Z2"))?, py::getitem(&t443, &Py::str("A2"))?, py::getitem(&t443, &Py::str("r0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t444 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t445 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t466 = (t445).items()?;
        if t466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t466[0].clone();
        v_v = t466[1].clone();
        if ({ let t446 = v_k.clone(); Py::Bool({ let t447 = v_outputs.clone(); if !py::contains(&t447, &t446)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_480(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t482: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t481 = v_inputs.clone(); f_de_broglie_voltage(py::getitem(&t481, &Py::str("V"))?, py::getitem(&t481, &Py::str("particle"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t482 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t483 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t504 = (t483).items()?;
        if t504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t504[0].clone();
        v_v = t504[1].clone();
        if ({ let t484 = v_k.clone(); Py::Bool({ let t485 = v_outputs.clone(); if !py::contains(&t485, &t484)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_522(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t524: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t523 = v_inputs.clone(); f_de_broglie_mv(py::getitem(&t523, &Py::str("m"))?, py::getitem(&t523, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t524 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t525 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t546 = (t525).items()?;
        if t546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t546[0].clone();
        v_v = t546[1].clone();
        if ({ let t526 = v_k.clone(); Py::Bool({ let t527 = v_outputs.clone(); if !py::contains(&t527, &t526)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_565(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t566 = v_inputs.clone(); f_debye(py::getitem(&t566, &Py::str("T"))?, py::getitem(&t566, &Py::str("theta_D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t568 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t589 = (t568).items()?;
        if t589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t589[0].clone();
        v_v = t589[1].clone();
        if ({ let t569 = v_k.clone(); Py::Bool({ let t570 = v_outputs.clone(); if !py::contains(&t570, &t569)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_618(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t620: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t619 = v_inputs.clone(); f_decay_chain(py::getitem(&t619, &Py::str("T_half_P"))?, py::getitem(&t619, &Py::str("T_half_D"))?, py::getitem(&t619, &Py::str("N0"))?, py::getitem(&t619, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t620 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t621 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t642 = (t621).items()?;
        if t642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t642[0].clone();
        v_v = t642[1].clone();
        if ({ let t622 = v_k.clone(); Py::Bool({ let t623 = v_outputs.clone(); if !py::contains(&t623, &t622)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_659(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t661: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t660 = v_inputs.clone(); f_einstein_oscillator(py::getitem(&t660, &Py::str("theta_E"))?, py::getitem(&t660, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t661 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t662 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t683 = (t662).items()?;
        if t683.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t683[0].clone();
        v_v = t683[1].clone();
        if ({ let t663 = v_k.clone(); Py::Bool({ let t664 = v_outputs.clone(); if !py::contains(&t664, &t663)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_699(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t701: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t700 = v_inputs.clone(); f_fermi_dirac(py::getitem(&t700, &Py::str("E"))?, py::getitem(&t700, &Py::str("mu"))?, py::getitem(&t700, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t701 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t702 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t723 = (t702).items()?;
        if t723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t723[0].clone();
        v_v = t723[1].clone();
        if ({ let t703 = v_k.clone(); Py::Bool({ let t704 = v_outputs.clone(); if !py::contains(&t704, &t703)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_736(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t738: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t737 = v_inputs.clone(); f_fermi_energy(py::getitem(&t737, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t738 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t739 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t760 = (t739).items()?;
        if t760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t760[0].clone();
        v_v = t760[1].clone();
        if ({ let t740 = v_k.clone(); Py::Bool({ let t741 = v_outputs.clone(); if !py::contains(&t741, &t740)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_779(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t781: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t780 = v_inputs.clone(); f_fermi_energy_metal(py::getitem(&t780, &Py::str("rho"))?, py::getitem(&t780, &Py::str("M"))?, py::getitem(&t780, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t781 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t782 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t803 = (t782).items()?;
        if t803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t803[0].clone();
        v_v = t803[1].clone();
        if ({ let t783 = v_k.clone(); Py::Bool({ let t784 = v_outputs.clone(); if !py::contains(&t784, &t783)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_78(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t80: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t79 = v_inputs.clone(); f_bec_temperature(py::getitem(&t79, &Py::str("n"))?, py::getitem(&t79, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t80 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t81 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t102 = (t81).items()?;
        if t102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t102[0].clone();
        v_v = t102[1].clone();
        if ({ let t82 = v_k.clone(); Py::Bool({ let t83 = v_outputs.clone(); if !py::contains(&t83, &t82)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_850(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t852: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t851 = v_inputs.clone(); f_fermi_chemical_potential(py::getitem(&t851, &Py::str("n"))?, py::getitem(&t851, &Py::str("T"))?, py::getitem(&t851, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t852 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t853 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t874 = (t853).items()?;
        if t874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t874[0].clone();
        v_v = t874[1].clone();
        if ({ let t854 = v_k.clone(); Py::Bool({ let t855 = v_outputs.clone(); if !py::contains(&t855, &t854)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_922(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t924: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t923 = v_inputs.clone(); f_finite_well(py::getitem(&t923, &Py::str("V0"))?, py::getitem(&t923, &Py::str("a"))?, py::getitem(&t923, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t924 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t925 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t946 = (t925).items()?;
        if t946.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t946[0].clone();
        v_v = t946[1].clone();
        if ({ let t926 = v_k.clone(); Py::Bool({ let t927 = v_outputs.clone(); if !py::contains(&t927, &t926)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_967(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t969: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t968 = v_inputs.clone(); f_fixed_target_cm_energy(py::getitem(&t968, &Py::str("m1"))?, py::getitem(&t968, &Py::str("E1"))?, py::getitem(&t968, &Py::str("m2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t969 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t970 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t991 = (t970).items()?;
        if t991.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t991[0].clone();
        v_v = t991[1].clone();
        if ({ let t971 = v_k.clone(); Py::Bool({ let t972 = v_outputs.clone(); if !py::contains(&t972, &t971)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
