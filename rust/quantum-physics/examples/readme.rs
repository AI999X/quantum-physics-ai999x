//! Quantum Physics: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p quantum-physics-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&quantum_physics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "clebsch_gordan", "inputs": {"j1": "0.5", "m1": "0.5", "j2": "0.5", "m2": "-0.5", "J": "1", "M": "0"}}));
    println!();
    show(json!({"key": "bohr_energy_level", "inputs": {"Z": "1", "n": "1"}}));
    println!();
    show(json!({"key": "two_body_decay", "inputs": {"M": "139.57039", "m1": "105.6583755", "m2": "0"}}));
    println!();
    show(json!({"key": "radiation_dose", "inputs": {"E": "2 mJ", "m": "500 g", "w_R": "20"}}));
    println!();
    show(json!({"key": "particle_decay_length", "inputs": {"particle": "mu-", "p": "0.1056583755"}}));
    println!();
    show(json!({"key": "compton_scattering", "inputs": {"lam": "71 pm", "theta": "90 deg"}}));
    println!();
    show(json!({"key": "gate_rotation_axis", "inputs": {"gates": "H"}}));
    println!();
    show(json!({"key": "finite_square_well", "inputs": {"V0": "10 eV", "a": "0.5 nm", "m": "9.109383714e-31 kg"}}));
    println!();
    show(json!({"key": "fixed_target_cm_energy", "inputs": {"m1": "1", "E1": "7", "m2": "1"}}));
    println!();
    show(json!({"key": "chsh_bell_test", "inputs": {"a": "0 deg", "a2": "90 deg", "b": "45 deg", "b2": "135 deg"}}));
    println!();
    show(json!({"key": "bec_critical_temperature", "inputs": {"n": "1e+20 m^-3", "m": "1.443e-25 kg"}}));
    println!();
    show(json!({"key": "wigner_3j_symbol", "inputs": {"j1": "1", "m1": "1", "j2": "1", "m2": "-1", "J": "0", "M": "0"}}));
    println!();
    show(json!({"key": "hydrogen_transition_reduced_mass", "inputs": {"Z": "1", "n_upper": "3", "n_lower": "2", "M_u": "1.007276467"}}));
    println!();
    show(json!({"key": "semi_empirical_mass_formula", "inputs": {"Z": "26", "A": "56"}}));
    println!();
    show(json!({"key": "radioactive_dating_age", "inputs": {"fraction": "0.25", "T_half": "5730 yr"}}));
}
