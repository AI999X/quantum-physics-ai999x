//! Quantum Physics' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: maps over two inputs, curve families, energy-level ladders, Bloch spheres, spectra, tables of coupling
//! coefficients and diagrams. Every number drawn is the model's answer or a further solve of it
//! (docs/FIGURE_PARITY.md); a figure that cannot be drawn for the inputs is left out.

use crate::figs_common as c;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::PI;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "bec_critical_temperature" => vec![c::field_map(s, "m", "n", "T_c")],
        "de_broglie_wavelength_mv" => vec![c::field_map(s, "m", "v", "lambda_nm")],
        "einstein_oscillator" => vec![c::field_map(s, "theta_E", "T", "C_molar")],
        "heisenberg_minimum_uncertainty" => vec![c::field_map(s, "dx", "m", "dv")],
        "length_contraction" => vec![c::field_map_k(s, "field", "L0", "v", "L", (0.5, 1.5), (0.25, 1.5))],
        "photoelectric_effect" => vec![c::field_map_k(s, "field", "lam", "W", "K_max_eV", (0.5, 1.0), (0.5, 1.5))],
        "planck_spectral_radiance" => vec![planck_spectrum(s), c::field_map(s, "T", "lam", "B_lambda")],
        "radioactive_dating_age" => vec![c::field_map(s, "T_half", "fraction", "t")],
        "relativistic_doppler" => vec![c::field_map_k(s, "field", "f", "v", "f_obs", (0.5, 1.5), (0.25, 1.5))],
        "relativistic_energy_from_momentum" => vec![c::field_map(s, "m", "p", "E_MeV")],
        "relativistic_momentum_energy" => vec![c::field_map_k(s, "field", "m", "v", "K_MeV", (0.5, 1.5), (0.25, 1.5))],
        "relativistic_speed_from_kinetic_energy" => vec![c::field_map(s, "m", "K", "beta")],
        "relativistic_velocity_addition" => vec![c::field_map(s, "u_prime", "v", "u")],
        "thermal_de_broglie_wavelength" => vec![c::field_map(s, "m", "T", "lambda_th_nm")],
        "time_dilation" => vec![c::field_map_k(s, "field", "t0", "v", "t", (0.5, 1.5), (0.25, 1.5))],
        "barrier_transmission" => vec![c::family_std(s, "curve-2", "E", "T", "V0", 0.1, 10.0, true), regime_map(s)],
        "bose_einstein_occupation" => vec![c::family_std(s, "curve-2", "E", "n", "T", 0.1, 10.0, true)],
        "step_potential" => vec![c::family_std(s, "curve-2", "V0", "T", "E", 0.1, 10.0, true)],
        "moseley_k_alpha" => vec![moseley(s)],
        "blackbody_band_fraction" => vec![band_spectrum(s)],
        "bohr_energy_level" => vec![bohr_orbits(s)],
        "clebsch_gordan" => vec![cg_table(s, 0.5, "table"), cg_table(s, 1.0, "table-2")],
        "coherent_state_photon_statistics" => vec![coherent_stems(s)],
        "finite_square_well" => vec![finite_levels(s)],
        "gate_rotation_axis" => vec![gate_axes(s)],
        "grover_search" => vec![grover_stems(s)],
        "harmonic_oscillator_forbidden_probability" => vec![forbidden_stems(s)],
        "harmonic_oscillator_levels" => vec![ho_levels(s)],
        "hydrogen_radial_expectations" => vec![radial_points(s)],
        "hydrogen_transition" => vec![hydrogen_levels(s), hydrogen_spectrum(s)],
        "infinite_well_probability" => vec![box_levels(s)],
        "infinite_well_transition" => vec![box_transitions(s)],
        "lande_g_zeeman" => vec![lande_bars(s), zeeman_levels(s)],
        "qubit_fidelity" => vec![fidelity_bloch(s)],
        "qubit_state_bloch" => vec![state_bloch(s)],
        "single_qubit_circuit" => vec![circuit_bloch(s)],
        "relativistic_aberration" => vec![aberration(s)],
        "semi_empirical_mass_formula" => vec![semf_bars(s)],
        "spacetime_interval" => vec![minkowski(s)],
        "spin_rotation_wigner_d" => vec![wigner_d(s)],
        "two_qubit_circuit" => vec![two_qubit_heat(s)],
        "wigner_3j_symbol" => vec![three_j(s)],
        "wkb_field_emission" => vec![triangular_barrier(s)],
        "two_body_decay" => vec![decay(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
fn oi(s: &Solved, key: &str, set: &[(&str, Input)], y: &str) -> Option<f64> {
    let a = s.solve_other(key, set)?;
    k::num_of(&a, y)
}

fn wi(s: &Solved, set: &[(&str, Input)], y: &str) -> Option<f64> {
    let a = s.with_input(set)?;
    k::num_of(&a, y)
}

fn f(x: f64) -> Input {
    Input::Float(x)
}

fn int(n: i64) -> Input {
    Input::Int(n)
}

fn txt(t: &str) -> Input {
    Input::Text(t.to_string())
}

fn fig(s: &Solved, kind: &str, title: &str, cap: &str, cv: &Canvas) -> Option<Figure> {
    Some(s.figure(kind, title, cap, cv.body(), cv.h))
}

#[allow(clippy::too_many_arguments)]
fn chart(s: &Solved, kind: &str, title: &str, cap: &str, series: Vec<(String, Vec<(f64, f64)>)>, xlab: &str,
         ylab: &str, header: &str, marker: Option<(f64, f64)>, mlabel: &str, log_x: bool) -> Option<Figure> {
    let series: Vec<(String, Vec<(f64, f64)>)> = series.into_iter().filter(|x| x.1.len() >= 2).collect();
    if series.is_empty() {
        return None;
    }
    let body = c::fit_right(&k::lines(&series, xlab, ylab, header, marker, mlabel, log_x));
    Some(s.chart(kind, title, cap, &body))
}

/// A half-whole or whole number as text: 0.5 -> "1/2", -1.5 -> "-3/2", 1 -> "1".
fn half(x: f64) -> String {
    let t = (2.0 * x).round() as i64;
    if t % 2 == 0 { format!("{}", t / 2) } else { format!("{t}/2") }
}

fn signed_half(x: f64) -> String {
    if x > 0.0 { format!("+{}", half(x)) } else { half(x) }
}

// ------------------------------------------------------------------------------------------------ the Bloch sphere
const BC: (f64, f64) = (200.0, 220.0);
const BR: f64 = 150.0;

/// A point of the Bloch sphere on the page (oblique view, x toward the viewer, y to the right, z up).
fn bp(v: (f64, f64, f64)) -> (f64, f64) {
    (BC.0 + BR * (v.1 - 0.42 * v.0), BC.1 + BR * (0.32 * v.0 - v.2))
}

fn bloch_base(cv: &mut Canvas) {
    c::ellipse(cv, BC, BR, BR, "axp-k");
    let eq: Vec<(f64, f64)> = (0..=72).map(|i| {
        let a = 2.0 * PI * i as f64 / 72.0;
        bp((a.cos(), a.sin(), 0.0))
    }).collect();
    c::poly(cv, &eq, "axp-c4", true);
    for (v, name) in [((1.15, 0.0, 0.0), "x"), ((0.0, 1.15, 0.0), "y"), ((0.0, 0.0, 1.12), "|0⟩"), ((0.0, 0.0, -1.12), "|1⟩")] {
        let p = bp(v);
        cv.dashed(BC, p);
        cv.name(p.0 + if name == "x" { -10.0 } else { 6.0 }, p.1 + if name == "|1⟩" { 16.0 } else { -4.0 }, "start", name);
    }
}

fn bloch_vec(cv: &mut Canvas, v: (f64, f64, f64), class: &str, label: &str) {
    let p = bp(v);
    c::varrow(cv, BC, p, class);
    if !label.is_empty() {
        cv.value(p.0 + 8.0, p.1 - 8.0, "start", label);
    }
}

fn triple(v: (f64, f64, f64)) -> String {
    format!("({}, {}, {})", k::num(k::snap(v.0, 1.0)), k::num(k::snap(v.1, 1.0)), k::num(k::snap(v.2, 1.0)))
}

fn gate_axes(s: &Solved) -> Option<Figure> {
    let mine = s.txt("gates")?;
    let (nx, ny, nz, ang) = (s.out("nx")?, s.out("ny")?, s.out("nz")?, s.out("angle_deg")?);
    let mut cv = Canvas::new(420.0);
    bloch_base(&mut cv);
    let mut lines = vec![];
    for g in ["X", "Y", "Z", "H", "S", "T"] {
        let a = s.with_input(&[("gates", txt(g))])?;
        let v = (k::num_of(&a, "nx")?, k::num_of(&a, "ny")?, k::num_of(&a, "nz")?);
        let d = k::num_of(&a, "angle_deg")?;
        lines.push(format!("{g}: {} by {}°", triple(v), k::num(d)));
        bloch_vec(&mut cv, v, "axp-c4", "");
    }
    bloch_vec(&mut cv, (nx, ny, nz), "axp-c", &mine);
    for (i, l) in lines.iter().enumerate() {
        cv.value(370.0, 60.0 + 20.0 * i as f64, "start", l);
    }
    cv.bold(16.0, 24.0, "start", &format!("{mine}: axis {} by {}°", triple((nx, ny, nz)), k::num(ang)));
    fig(s, "bloch", "Rotation axes of single-qubit gates on the Bloch sphere", &format!(
        "Rotation axes of single-qubit gates on the Bloch sphere (oblique view, x toward the viewer): the unit axis and \
         rotation angle the model returns for X, Y, Z, H, S and T (grey) and for the given sequence {mine} (accent): \
         {}; given: {} by {}°.", lines.join("; "), triple((nx, ny, nz)), k::num(ang)), &cv)
}

fn parse_vec(t: &str) -> Option<(f64, f64, f64)> {
    let v: Vec<f64> = t.replace(['[', ']'], "").split(',').filter_map(|x| x.trim().parse().ok()).collect();
    (v.len() == 3).then(|| (v[0], v[1], v[2]))
}

fn fidelity_bloch(s: &Solved) -> Option<Figure> {
    let r = s.vec("r").filter(|v| v.len() == 3).map(|v| (v[0], v[1], v[2])).or_else(|| parse_vec(&s.txt("r")?))?;
    let q = s.vec("s").filter(|v| v.len() == 3).map(|v| (v[0], v[1], v[2])).or_else(|| parse_vec(&s.txt("s")?))?;
    let (fid, sf, td) = (s.out("F")?, s.out("sqrt_F")?, s.out("trace_distance")?);
    let mut cv = Canvas::new(420.0);
    bloch_base(&mut cv);
    bloch_vec(&mut cv, r, "axp-c", &format!("r {}", triple(r)));
    bloch_vec(&mut cv, q, "axp-c2", &format!("s {}", triple(q)));
    cv.dashed(bp(r), bp(q));
    cv.bold(16.0, 24.0, "start", &format!("F = {}, √F = {}", k::num(fid), k::num(sf)));
    cv.value(16.0, 44.0, "start", &format!("trace distance |r - s|/2 = {}", k::num(td)));
    fig(s, "bloch", "Two qubit states on the Bloch sphere", &format!(
        "The two qubit states as Bloch vectors r = {} and s = {} (oblique view, x toward the viewer; dashed: the chord \
         between them). Marked: the model's fidelity F = {} (square root {}) and trace distance {}.", triple(r), triple(q),
        k::num(fid), k::num(sf), k::num(td)), &cv)
}

fn state_bloch(s: &Solved) -> Option<Figure> {
    let v = (s.out("x")?, s.out("y")?, s.out("z")?);
    let (th, ph, p0, p1) = (s.out("theta")?, s.out("phi")?, s.out("p0")?, s.out("p1")?);
    let mut cv = Canvas::new(420.0);
    bloch_base(&mut cv);
    cv.dashed(bp((v.0, v.1, 0.0)), bp(v));
    cv.dashed(BC, bp((v.0, v.1, 0.0)));
    bloch_vec(&mut cv, v, "axp-c", &triple(v));
    cv.bold(16.0, 24.0, "start", &format!("θ = {} rad, φ = {} rad", k::num(th), k::num(ph)));
    cv.value(16.0, 44.0, "start", &format!("P(0) = {}, P(1) = {}", k::num(p0), k::num(p1)));
    fig(s, "bloch", "A qubit state on the Bloch sphere", &format!(
        "The qubit state α|0⟩ + β|1⟩ as its Bloch vector (oblique view, x toward the viewer; dashed: its projection on \
         the equator). Marked: the model's (x, y, z) = {}, θ = {} rad, φ = {} rad, P(0) = {}, P(1) = {}.", triple(v),
        k::num(th), k::num(ph), k::num(p0), k::num(p1)), &cv)
}

fn circuit_bloch(s: &Solved) -> Option<Figure> {
    let gates = s.txt("gates")?;
    let start = s.txt("start")?;
    let words: Vec<&str> = gates.split_whitespace().collect();
    if words.is_empty() || words.len() > 8 {
        return None;
    }
    let mut cv = Canvas::new(420.0);
    bloch_base(&mut cv);
    let mut pts = vec![];
    let mut notes = vec![];
    for i in 0..=words.len() {
        let g = if i == 0 { "I".to_string() } else { words[..i].join(" ") };
        let a = s.with_input(&[("gates", txt(&g))])?;
        let v = (k::num_of(&a, "bx")?, k::num_of(&a, "by")?, k::num_of(&a, "bz")?);
        notes.push(format!("{}: {}", if i == 0 { "start" } else { words[i - 1] }, triple(v)));
        pts.push(v);
    }
    for (i, v) in pts.iter().enumerate() {
        let last = i + 1 == pts.len();
        bloch_vec(&mut cv, *v, if last { "axp-c" } else { "axp-c4" }, "");
    }
    let path: Vec<(f64, f64)> = pts.iter().map(|v| bp(*v)).collect();
    c::poly(&mut cv, &path, "axp-c3", true);
    for (i, n) in notes.iter().enumerate() {
        cv.value(370.0, 60.0 + 20.0 * i as f64, "start", n);
    }
    cv.bold(16.0, 24.0, "start", &format!("start |{start}⟩, gates {gates}"));
    fig(s, "bloch", "A single-qubit circuit on the Bloch sphere", &format!(
        "The single-qubit circuit started in |{start}⟩ on the Bloch sphere (oblique view, x toward the viewer): the \
         Bloch vector the model returns after each gate in turn (the last in the accent colour): {}.", notes.join("; ")),
        &cv)
}

// ------------------------------------------------------------------------------------------------ levels
/// An energy-level ladder: levels (label, energy) on a vertical axis, a value written beside each.
fn ladder(cv: &mut Canvas, levels: &[(String, f64)], x0: f64, x1: f64, y_top: f64, y_bot: f64, unit: &str, dashed: bool) {
    let (lo, hi) = levels.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |a, l| (a.0.min(l.1), a.1.max(l.1)));
    let py = c::lin(lo, hi, y_bot, y_top);
    for (name, e) in levels {
        let y = py(*e);
        if dashed {
            cv.dashed((x0, y), (x1, y));
        } else {
            cv.raw(&format!("<path class=\"axp-c\" d=\"M{x0:.1},{y:.1} L{x1:.1},{y:.1}\"/>"));
        }
        cv.name(x0 - 6.0, y + 4.0, "end", name);
        cv.value(x1 + 6.0, y + 4.0, "start", &format!("{} {unit}", k::num(*e)));
    }
}

fn finite_levels(s: &Solved) -> Option<Figure> {
    let (v0, a, m) = (c::inp(s, "V0")?, c::inp(s, "a")?, c::inp(s, "m")?);
    let es = s.out_list("energies_eV")?;
    if es.is_empty() || es.len() > 12 {
        return None;
    }
    let depth = v0 / 1.602176634e-19;
    let mut cv = Canvas::new(340.0);
    let py = c::lin(0.0, depth, 300.0, 60.0);
    // the well
    cv.wire(&[(60.0, 60.0), (160.0, 60.0), (160.0, 300.0), (330.0, 300.0), (330.0, 60.0), (440.0, 60.0)]);
    cv.name(440.0, 54.0, "end", &format!("V0 = {} eV", k::num(depth)));
    for (i, e) in es.iter().enumerate() {
        let y = py(*e);
        cv.raw(&format!("<path class=\"axp-c\" d=\"M162,{y:.1} L328,{y:.1}\"/>"));
        cv.value(336.0, y + 4.0, "start", &format!("E{} = {} eV", i + 1, k::num(*e)));
    }
    // dashed: an infinitely deep well of the same width (the model's particle-in-a-box levels)
    let mut inf = vec![];
    for n in 1..=4 {
        let Some(e) = oi(s, "infinite_well_levels", &[("m", f(m)), ("L", f(a)), ("n", int(n))], "E_eV") else { break };
        if e > depth {
            break;
        }
        inf.push(e);
        let y = py(e);
        cv.raw(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M170,{y:.1} L320,{y:.1}\"/>"));
        cv.name(156.0, y + 4.0, "end", &format!("{} eV", k::num(e)));
    }
    cv.bold(16.0, 24.0, "start", &format!("{} bound states in a well {} wide", es.len(), k::q(a, "m")));
    fig(s, "levels", "Bound states of a finite square well", &format!(
        "A particle of mass {} kg in a finite square well of depth V0 = {} eV and width a = {}: the model finds {} bound \
         states, {}; dashed: the levels of an infinitely deep well of the same width below V0 ({}, also from the \
         model), always higher.", k::num(m), k::num(depth), k::q(a, "m"), es.len(),
        es.iter().enumerate().map(|(i, e)| format!("E{} = {} eV", i + 1, k::num(*e))).collect::<Vec<_>>().join(", "),
        inf.iter().map(|e| format!("{} eV", k::num(*e))).collect::<Vec<_>>().join(", ")), &cv)
}

fn ho_levels(s: &Solved) -> Option<Figure> {
    let (m, w) = (c::inp(s, "m")?, c::inp(s, "omega")?);
    let n0 = c::inp(s, "n")? as i64;
    let top = (n0 + 1).max(5).min(30);
    let mut lv = vec![];
    for n in 0..=top {
        let a = s.with_input(&[("n", int(n))])?;
        lv.push((n, k::num_of(&a, "E_eV")?, k::num_of(&a, "amplitude")?));
    }
    let amax = lv.iter().map(|x| x.2).fold(0.0, f64::max);
    let emax = lv.iter().map(|x| x.1).fold(0.0, f64::max);
    if !(amax > 0.0 && emax > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(340.0);
    let px = c::lin(-amax * 1.15, amax * 1.15, 60.0, 400.0);
    let py = c::lin(0.0, emax * 1.08, 310.0, 40.0);
    // the potential through the turning points the model returns
    let mut pot: Vec<(f64, f64)> = lv.iter().rev().map(|x| (px(-x.2), py(x.1))).collect();
    pot.push((px(0.0), py(0.0)));
    pot.extend(lv.iter().map(|x| (px(x.2), py(x.1))));
    c::poly(&mut cv, &pot, "axp-c4", false);
    for (n, e, a) in &lv {
        let y = py(*e);
        let class = if *n == n0 { "axp-c2" } else { "axp-c" };
        cv.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{y:.1} L{:.1},{y:.1}\"/>", px(-a), px(*a)));
        cv.dot((px(-a), y));
        cv.dot((px(*a), y));
        if lv.len() <= 8 || n % 2 == 0 || *n == n0 {
            cv.value(410.0, y + 4.0, "start", &format!("n = {n}: {} eV", k::num(*e)));
        }
    }
    let sp = s.out("spacing_eV")?;
    cv.bold(16.0, 24.0, "start", &format!("spacing ħω = {} eV", k::num(sp)));
    fig(s, "levels", "Levels of a quantum harmonic oscillator", &format!(
        "The quantum harmonic oscillator for a particle of mass {} kg at ω = {} rad/s: the levels n = 0 to {top} the model \
         returns (spacing {} eV), each drawn between its classical turning points (dots, also returned), which trace the \
         parabola of the potential. Highlighted: the given n = {n0}, E = {} eV.", k::num(m), k::num(w), k::num(sp),
        k::num(s.out("E_eV")?)), &cv)
}

fn box_levels(s: &Solved) -> Option<Figure> {
    let (l, x1, x2) = (c::inp(s, "L")?, c::inp(s, "x1")?, c::inp(s, "x2")?);
    let p = s.out("P")?;
    let n0 = c::inp(s, "n")? as i64;
    let me = 9.1093837139e-31;
    let mut cv = Canvas::new(340.0);
    let nmax = n0.max(3).min(6);
    let mut es = vec![];
    for n in 1..=nmax {
        es.push(oi(s, "infinite_well_levels", &[("m", f(me)), ("L", f(l)), ("n", int(n))], "E_eV")?);
    }
    let emax = *es.last()?;
    let py = c::lin(0.0, emax * 1.25, 310.0, 50.0);
    let px = c::lin(0.0, l, 120.0, 400.0);
    cv.wire(&[(120.0, 40.0), (120.0, 310.0), (400.0, 310.0), (400.0, 40.0)]);
    let bins = 60;
    let gap = (py(es[0]) - py(es.get(1).copied().unwrap_or(emax * 1.25))).abs().min(60.0);
    for (i, e) in es.iter().enumerate() {
        let n = i as i64 + 1;
        let y = py(*e);
        cv.dashed((120.0, y), (400.0, y));
        // the probability density: the model's probability of each of 60 intervals divided by the interval
        let mut dens = vec![];
        for b in 0..bins {
            let (a0, a1) = (l * b as f64 / bins as f64, l * (b + 1) as f64 / bins as f64);
            let pb = wi(s, &[("n", int(n)), ("x1", f(a0)), ("x2", f(a1))], "P")?;
            dens.push(((a0 + a1) / 2.0, pb));
        }
        let dmax = dens.iter().map(|d| d.1).fold(0.0, f64::max).max(1e-300);
        let pts: Vec<(f64, f64)> = dens.iter().map(|(x, d)| (px(*x), y - 0.8 * gap * d / dmax)).collect();
        c::poly(&mut cv, &pts, if n == n0 { "axp-c2" } else { "axp-c" }, false);
        cv.value(406.0, y + 4.0, "start", &format!("E{n} = {} eV", k::num(*e)));
    }
    cv.raw(&format!("<rect class=\"axp-s2\" x=\"{:.1}\" y=\"40\" width=\"{:.1}\" height=\"270\"/>", px(x1), (px(x2) - px(x1)).max(1.0)));
    cv.bold(16.0, 24.0, "start", &format!("P(n = {n0}, {} to {}) = {}", k::q(x1, "m"), k::q(x2, "m"), k::num(p)));
    fig(s, "levels", "Levels and probability densities of a particle in a box", &format!(
        "An electron in an infinite square well {} wide: the lowest energy levels the model gives ({}), each with its \
         probability density drawn on the level (the model's probability of each of 60 equal intervals divided by the \
         interval); state n has n humps. Shaded: the given interval, where the probability is {} for n = {n0}.",
        k::q(l, "m"), es.iter().enumerate().map(|(i, e)| format!("E{} = {} eV", i + 1, k::num(*e))).collect::<Vec<_>>().join(", "),
        k::num(p)), &cv)
}

fn box_transitions(s: &Solved) -> Option<Figure> {
    let (m, l) = (c::inp(s, "m")?, c::inp(s, "L")?);
    let nu = c::inp(s, "n_upper")? as i64;
    let top = nu.max(4).min(7);
    let mut es = vec![];
    for n in 1..=top {
        es.push(oi(s, "infinite_well_levels", &[("m", f(m)), ("L", f(l)), ("n", int(n))], "E_eV")?);
    }
    let mut cv = Canvas::new(340.0);
    let py = c::lin(0.0, es[es.len() - 1], 310.0, 50.0);
    for (i, e) in es.iter().enumerate() {
        let y = py(*e);
        cv.line((60.0, y), (300.0, y));
        cv.name(54.0, y + 4.0, "end", &format!("n = {}", i + 1));
        cv.value(306.0, y + 4.0, "start", &format!("{} eV", k::num(*e)));
    }
    let mut notes = vec![];
    let mut col = 0;
    for up in 2..=top.min(4) {
        for lo in 1..up {
            let lam = wi(s, &[("n_upper", int(up)), ("n_lower", int(lo))], "lambda_nm")?;
            let x = 80.0 + 34.0 * col as f64;
            col += 1;
            c::varrow(&mut cv, (x, py(es[up as usize - 1])), (x, py(es[lo as usize - 1])),
                      if up == nu && lo == c::inp(s, "n_lower")? as i64 { "axp-c" } else { "axp-c3" });
            notes.push(format!("{up}→{lo}: {} nm", k::num(lam)));
        }
    }
    for (i, n) in notes.iter().enumerate() {
        cv.value(400.0, 60.0 + 20.0 * i as f64, "start", n);
    }
    cv.bold(16.0, 24.0, "start", &format!("given {nu}→{}: λ = {} nm", c::inp(s, "n_lower")? as i64, k::num(s.out("lambda_nm")?)));
    fig(s, "diagram", "Transitions of a particle in a box", &format!(
        "A particle of mass {} kg in an infinite square well of width {}: the lowest levels the model gives (E grows as \
         n²) and the photon wavelength of each downward transition it computes: {}.", k::num(m), k::q(l, "m"),
        notes.join(", ")), &cv)
}

fn hydrogen_levels(s: &Solved) -> Option<Figure> {
    let z = c::inp(s, "Z")? as i64;
    let mut es = vec![];
    for n in 1..=6 {
        es.push(oi(s, "bohr_energy_level", &[("Z", int(z)), ("n", int(n))], "E_eV")?);
    }
    let mut cv = Canvas::new(340.0);
    // a broken axis: the levels' order, spaced by energy
    let py = c::lin(es[0], 0.0, 310.0, 50.0);
    for (i, e) in es.iter().enumerate() {
        let y = py(*e);
        cv.line((60.0, y), (330.0, y));
        if i < 3 || i == 5 {
            cv.name(54.0, y + 4.0, "end", &format!("n = {}", i + 1));
            cv.value(336.0, y + 4.0, "start", &format!("{} eV", k::num(*e)));
        }
    }
    let mut notes = vec![];
    let pairs = [(2, 1), (3, 1), (3, 2), (4, 2), (5, 2), (6, 2)];
    for (j, (u, l)) in pairs.iter().enumerate() {
        let lam = wi(s, &[("n_upper", int(*u)), ("n_lower", int(*l))], "lambda_nm")?;
        let x = 80.0 + 40.0 * j as f64;
        c::varrow(&mut cv, (x, py(es[*u as usize - 1])), (x, py(es[*l as usize - 1])), if *l == 1 { "axp-c3" } else { "axp-c" });
        notes.push(format!("{u}→{l}: {} nm", k::num(lam)));
    }
    for (i, n) in notes.iter().enumerate() {
        cv.value(420.0, 130.0 + 20.0 * i as f64, "start", n);
    }
    cv.bold(16.0, 24.0, "start", &format!("Z = {z}: E1 = {} eV", k::num(es[0])));
    cv.value(16.0, 44.0, "start", &format!("given: λ = {} nm", k::num(s.out("lambda_nm")?)));
    fig(s, "diagram", "Energy levels and transitions of a one-electron atom", &format!(
        "Energy levels of a one-electron atom with Z = {z}, E_n from the model for n = 1 to 6 (E1 = {} eV), with the \
         photon wavelengths of the Lyman (violet) and Balmer (accent) transitions it computes: {}.", k::num(es[0]),
        notes.join(", ")), &cv)
}

fn hydrogen_spectrum(s: &Solved) -> Option<Figure> {
    let mine = s.out("lambda_nm")?;
    let mut cv = Canvas::new(340.0);
    let mut lines = vec![];
    for (lo, name, class) in [(1, "Lyman", "axp-c3"), (2, "Balmer", "axp-c"), (3, "Paschen", "axp-c2")] {
        let mut l = vec![];
        for up in lo + 1..=lo + 6 {
            l.push(wi(s, &[("n_upper", int(up)), ("n_lower", int(lo))], "lambda_nm")?);
        }
        lines.push((name, class, l));
    }
    let all: Vec<f64> = lines.iter().flat_map(|x| x.2.iter().copied()).chain([mine]).collect();
    let (lo, hi) = all.iter().fold((f64::INFINITY, 0.0f64), |a, x| (a.0.min(*x), a.1.max(*x)));
    let (llo, lhi) = ((lo * 0.85).log10(), (hi * 1.15).log10());
    let px = |x: f64| 40.0 + (x.log10() - llo) / (lhi - llo) * 440.0;
    cv.line((40.0, 260.0), (480.0, 260.0));
    let mut t = 10f64.powf(llo.ceil());
    while t.log10() <= lhi {
        for m_ in [1.0, 2.0, 5.0] {
            let v = t * m_;
            if v.log10() >= llo && v.log10() <= lhi {
                cv.line((px(v), 260.0), (px(v), 266.0));
                cv.name(px(v), 282.0, "middle", &k::num(v));
            }
        }
        t *= 10.0;
    }
    cv.name(260.0, 304.0, "middle", "wavelength (nm), logarithmic axis");
    let (v0, v1) = (px(380f64.max(lo * 0.85)), px(750f64.min(hi * 1.15)));
    if v1 > v0 {
        cv.raw(&format!("<rect class=\"axp-s2\" x=\"{v0:.1}\" y=\"70\" width=\"{:.1}\" height=\"190\"/>", v1 - v0));
        cv.name((v0 + v1) / 2.0, 66.0, "middle", "visible");
    }
    for (j, (name, class, l)) in lines.iter().enumerate() {
        for x in l {
            cv.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},260\"/>", px(*x), 110.0 + 30.0 * j as f64, px(*x)));
        }
        cv.value(px(l[0]), 104.0 + 30.0 * j as f64, "middle", name);
    }
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"260\" r=\"6\"/>", px(mine)));
    cv.bold(16.0, 24.0, "start", &format!("given line: {} nm ({})", k::num(mine), s.out_text("series").unwrap_or_default()));
    let desc: Vec<String> = lines.iter().map(|(n, _, l)| format!("{n} {} to {} nm", k::num(l[0]), k::num(l[5]))).collect();
    fig(s, "spectrum", "The emission spectrum of a one-electron atom", &format!(
        "The emission spectrum: the first six lines of the Lyman, Balmer and Paschen series for the given Z, each \
         wavelength computed by the model ({}); the shaded band is the visible range 380 to 750 nm. Marked: the given \
         line at {} nm.", desc.join("; "), k::num(mine)), &cv)
}

fn zeeman_levels(s: &Solved) -> Option<Figure> {
    let (l, sp, j, mj, b) = (c::inp(s, "L")?, c::inp(s, "S")?, c::inp(s, "J")?, c::inp(s, "mJ")?, c::inp(s, "B")?);
    let de = s.out("dE_eV")?;
    let bmax = if b > 0.0 { 2.0 * b } else { 2.0 };
    let bs = k::span(0.0, bmax, 9, false);
    let mut series = vec![];
    let mut m = -j;
    while m <= j + 1e-9 {
        let p: Vec<(f64, f64)> = bs.iter().filter_map(|bb| wi(s, &[("mJ", f(m)), ("B", f(*bb))], "dE_eV").map(|y| (*bb, y))).collect();
        series.push((format!("mJ = {}", signed_half(m)), p));
        m += 1.0;
        if series.len() > 8 {
            break;
        }
    }
    let g = s.out("g_J")?;
    chart(s, "levels", "Zeeman splitting of a level in a magnetic field", &format!(
        "Zeeman splitting of the level L = {}, S = {}, J = {} in a field B from 0 to {} T: the energy shift of every mJ \
         sublevel, a solve of the model at 9 fields (gJ = {}); the shifts grow linearly with B. Marked: the given mJ = {} \
         at B = {} T, ΔE = {} eV.", half(l), half(sp), half(j), k::num(bmax), k::num(g), signed_half(mj), k::num(b),
        k::num(de)),
          series, "B, magnetic field (T)", "ΔE (eV)", &format!("gJ = {}", k::num(g)), Some((b, de)),
          &format!("{} eV", k::num(de)), false)
}

fn lande_bars(s: &Solved) -> Option<Figure> {
    let (l0, s0, j0) = (c::inp(s, "L")?, c::inp(s, "S")?, c::inp(s, "J")?);
    let terms: [(&str, f64, f64, f64); 9] = [("2S1/2", 0.0, 0.5, 0.5), ("2P1/2", 1.0, 0.5, 0.5), ("2P3/2", 1.0, 0.5, 1.5),
        ("2D3/2", 2.0, 0.5, 1.5), ("2D5/2", 2.0, 0.5, 2.5), ("1P1", 1.0, 0.0, 1.0), ("3P1", 1.0, 1.0, 1.0),
        ("3P2", 1.0, 1.0, 2.0), ("3D3", 2.0, 1.0, 3.0)];
    let mut labels = vec![];
    let mut vals = vec![];
    let mut hl = vec![];
    let mut notes = vec![];
    for (name, l, sp, j) in terms {
        let a = s.with_input(&[("L", f(l)), ("S", f(sp)), ("J", f(j)), ("mJ", f(if j.fract() == 0.0 { 0.0 } else { 0.5 }))])?;
        let g = k::num_of(&a, "g_J")?;
        let ex = a.values.iter().find(|x| x.0 == "g_J_exact").and_then(|x| x.1.as_str().map(String::from)).unwrap_or(k::num(g));
        if l == l0 && sp == s0 && j == j0 {
            hl.push(labels.len());
        }
        labels.push(name.to_string());
        vals.push(g);
        notes.push(format!("{name} {ex}"));
    }
    let body = c::fit_right(&k::bars(&labels, &vals, &hl, "term (2S+1, L, J)", "gJ, Landé g factor",
                                     &format!("given gJ = {}", s.out_text("g_J_exact").unwrap_or(k::num(s.out("g_J")?)))));
    Some(s.chart("bars", "Landé g factors of atomic terms", &format!(
        "Landé g factors of 9 atomic terms written as 2S+1, L, J, each a solve of the model (exact fractions): {}. A pure \
         spin gives 2, a pure orbit 1. Highlighted: the given term when it is one of them (L = {}, S = {}, J = {}).",
        notes.join(", "), half(l0), half(s0), half(j0)), &body))
}

// ------------------------------------------------------------------------------------------------ charts and stems
fn moseley(s: &Solved) -> Option<Figure> {
    let (z, e) = (c::inp(s, "Z")?, s.out("E_keV")?);
    let zmax = (z * 2.0).max(30.0).min(100.0) as i64;
    let p: Vec<(f64, f64)> = (2..=zmax).step_by(((zmax / 30).max(1)) as usize)
        .filter_map(|zz| wi(s, &[("Z", int(zz))], "E_keV").map(|y| (zz as f64, y))).collect();
    chart(s, "curve-2", "K-alpha X-ray line: photon energy against atomic number", &format!(
        "E (photon energy) against Z (atomic number of the target element) from 2 to {zmax}, each point a solve of the \
         model: the energy grows as (Z - 1)², Moseley's law. Marked: the given Z = {}, E = {} keV.", z, k::num(e)),
          vec![(String::new(), p)], "Z, atomic number", "E, photon energy (keV)", &format!("E = {} keV", k::num(e)),
          Some((z, e)), &format!("{} keV", k::num(e)), false)
}

fn coherent_stems(s: &Solved) -> Option<Figure> {
    let (al, n0, p0) = (c::inp(s, "alpha")?, c::inp(s, "n")? as i64, s.out("P_n")?);
    let alphas = [al / 2.0, al, al * 1.5];
    let nmax = ((al * 1.5).powi(2) * 1.6 + 4.0).ceil().clamp(8.0, 30.0) as i64;
    let labels: Vec<String> = (0..=nmax).map(|n| n.to_string()).collect();
    let mut groups = vec![];
    for a in alphas {
        // probabilities below a thousandth of the largest left out (they would force a logarithmic axis)
        let v: Vec<Option<f64>> = (0..=nmax).map(|n| wi(s, &[("alpha", f(a)), ("n", int(n))], "P_n")).collect();
        let top = v.iter().flatten().fold(0.0f64, |x, y| x.max(*y));
        let v: Vec<Option<f64>> = v.into_iter().map(|x| x.filter(|p| *p >= top * 1e-3)).collect();
        groups.push((format!("|α| = {}", k::num(a)), v));
    }
    let body = c::fit_right(&k::bar_groups(&labels, &groups, "n, photon number", "P(n)",
                                           &format!("P({n0}) = {} at |α| = {}", k::num(p0), k::num(al))));
    Some(s.chart("stems", "Photon-number distributions of coherent light", &format!(
        "Photon-number distribution of coherent (laser) light for |α| = {}, {} and {}, P(n) for n = 0 to {nmax}, each a \
         solve of the model (side by side): Poisson distributions whose variance equals the mean |α|². Marked: the given \
         |α| = {}, P({n0}) = {}.", k::num(alphas[0]), k::num(alphas[1]), k::num(alphas[2]), k::num(al), k::num(p0)),
                 &body))
}

fn grover_stems(s: &Solved) -> Option<Figure> {
    let (q0, m0, k0, p0) = (c::inp(s, "n_qubits")? as i64, c::inp(s, "M")? as i64, c::inp(s, "k")? as i64, s.out("P_success")?);
    let qs: Vec<i64> = vec![q0, q0 + 1, q0 + 2].into_iter().filter(|q| *q <= 16).collect();
    let kmax = 12.max(k0);
    let labels: Vec<String> = (0..=kmax).map(|x| x.to_string()).collect();
    let mut groups = vec![];
    let mut notes = vec![];
    for q in &qs {
        let v: Vec<Option<f64>> = (0..=kmax).map(|kk| wi(s, &[("n_qubits", int(*q)), ("M", int(m0)), ("k", int(kk))], "P_success")).collect();
        if let Some(a) = s.with_input(&[("n_qubits", int(*q))]) {
            notes.push(format!("{q} qubits: best k = {}, P = {}", k::num(k::num_of(&a, "k_opt")?), k::num(k::num_of(&a, "P_at_k_opt")?)));
        }
        groups.push((format!("{q} qubits"), v));
    }
    let body = c::fit_right(&k::bar_groups(&labels, &groups, "k, Grover iterations", "success probability",
                                           &format!("P = {} after k = {k0}", k::num(p0))));
    Some(s.chart("stems", "Grover search: success probability against iterations", &format!(
        "Grover search for {m0} marked item(s) among 2ⁿ: the probability of success after k = 0 to {kmax} iterations for \
         {} qubits, each a solve of the model (side by side). The probability rises to nearly 1 at the best k and then \
         falls again: {}. Marked: the given {q0} qubits, k = {k0}, P = {}.",
        qs.iter().map(|q| q.to_string()).collect::<Vec<_>>().join(", "), notes.join("; "), k::num(p0)), &body))
}

fn forbidden_stems(s: &Solved) -> Option<Figure> {
    let (n0, p0) = (c::inp(s, "n")? as i64, s.out("P_forbidden")?);
    let nmax = 20.max(n0).min(60);
    let mut labels = vec![];
    let mut vals = vec![];
    for n in 0..=nmax {
        labels.push(n.to_string());
        vals.push(wi(s, &[("n", int(n))], "P_forbidden")?);
    }
    let body = c::fit_right(&k::bars(&labels, &vals, &[n0 as usize], "n, quantum number", "probability outside",
                                     &format!("P = {} at n = {n0}", k::num(p0))));
    Some(s.chart("stems", "Probability beyond the classical turning points", &format!(
        "Probability of finding a quantum harmonic oscillator outside its classical turning points for levels n = 0 to \
         {nmax}, each a solve of the model: {} in the ground state, {} at n = {nmax}, falling slowly as the quantum \
         oscillator approaches the classical one. Highlighted: the given n = {n0}, P = {}.", k::num(vals[0]),
        k::num(vals[vals.len() - 1]), k::num(p0)), &body))
}

fn radial_points(s: &Solved) -> Option<Figure> {
    let (n0, l0, z) = (c::inp(s, "n")? as i64, c::inp(s, "l")? as i64, c::inp(s, "Z")? as i64);
    let nmax = n0.max(4).min(8);
    let mut series = vec![];
    let mut notes = vec![];
    let letters = ["s", "p", "d", "f", "g", "h", "i", "k"];
    for l in 0..nmax.min(4) {
        let mut p = vec![];
        for n in (l + 1)..=nmax {
            let a = s.with_input(&[("n", int(n)), ("l", int(l))])?;
            let r = k::num_of(&a, "r_mean_a0")?;
            p.push((n as f64, r));
            notes.push(format!("{n}{} {}", letters[l as usize], k::num(r)));
        }
        series.push((format!("l = {l} ({})", letters[l as usize]), p));
    }
    let r0 = s.out("r_mean_a0")?;
    let dr = s.out("dr_a0")?;
    chart(s, "points", "Mean radius of the electron in a one-electron atom", &format!(
        "Mean radius ⟨r⟩ of the electron of a one-electron atom with Z = {z} for every state n = 1 to {nmax} and l = 0 to \
         n - 1 (up to f), each a solve of the model (in units of a0): {}. ⟨r⟩ grows as n²/Z; at fixed n a larger l sits \
         slightly closer in. Marked: the given state n = {n0}, l = {l0}, ⟨r⟩ = {} a0 with spread {} a0.",
        notes.join(", "), k::num(r0), k::num(dr)),
          series, "n, principal quantum number", "⟨r⟩ (units of a0)", &format!("⟨r⟩ = {} a0", k::num(r0)),
          Some((n0 as f64, r0)), &format!("{} a0", k::num(r0)), false)
}

fn planck_spectrum(s: &Solved) -> Option<Figure> {
    let (lam, t) = (c::inp(s, "lam")?, c::inp(s, "T")?);
    let ts = [t * 0.52, t * 0.78, t];
    let peak = oi(s, "wien_peak", &[("T", f(t))], "lambda_max")?;
    let ls = k::span(peak * 0.15, peak * 6.0, 61, true);
    let mut series = vec![];
    let mut tops = vec![];
    let mut notes = vec![];
    for tt in ts {
        let p: Vec<(f64, f64)> = ls.iter().filter_map(|l| c::at(s, &[("lam", *l), ("T", tt)], "B_lambda").map(|y| (*l, y))).collect();
        let lm = oi(s, "wien_peak", &[("T", f(tt))], "lambda_max")?;
        let bm = c::at(s, &[("lam", lm), ("T", tt)], "B_lambda")?;
        tops.push((lm, bm));
        notes.push(format!("T = {} K: peak at {}, {} W/m³", k::num(tt), k::q(lm, "m"), k::num(bm)));
        series.push((format!("T = {} K", k::num(tt)), p));
    }
    series.push(("peaks (Wien)".into(), tops));
    let b = s.out("B_lambda")?;
    chart(s, "spectrum", "Planck spectra at three temperatures", &format!(
        "Planck spectral radiance at three temperatures, 61 wavelengths each a solve of the model; the dots are the \
         peaks from its Wien's-law capability: {}. Marked: the given λ = {}, T = {} K, B = {} W/m³.", notes.join("; "),
        k::q(lam, "m"), k::num(t), k::num(b)),
          series, "λ, wavelength (m)", "B, spectral radiance (W/m³)", &format!("B = {} W/m³", k::num(b)),
          Some((lam, b)), "given", true)
}

fn band_spectrum(s: &Solved) -> Option<Figure> {
    let (l1, l2, t) = (c::inp(s, "lam1")?, c::inp(s, "lam2")?, c::inp(s, "T")?);
    let (fb, f1, f2) = (s.out("F_band")?, s.out("F_below_lam1")?, s.out("F_below_lam2")?);
    let peak = oi(s, "wien_peak", &[("T", f(t))], "lambda_max")?;
    let ls = k::span(peak * 0.2, peak * 8.0, 121, true);
    let mut full = vec![];
    let mut band = vec![];
    for l in &ls {
        let Some(y) = oi(s, "planck_spectral_radiance", &[("lam", f(*l)), ("T", f(t))], "B_lambda") else { continue };
        full.push((*l, y));
        band.push((*l, if *l >= l1 && *l <= l2 { y } else { f64::NAN }));
    }
    chart(s, "spectrum", "A wavelength band of a blackbody spectrum", &format!(
        "The band from {} to {} (highlighted where it falls in the drawn range) of the Planck spectrum of a T = {} K \
         blackbody (121 wavelengths from the model's spectral radiance, drawn around the peak at {}). Marked: the \
         model's band fraction F = {} of all radiated power, with {} below the band and {} above it.", k::q(l1, "m"),
        k::q(l2, "m"), k::num(t), k::q(peak, "m"), k::num(fb), k::num(f1), k::num(k::snap(1.0 - f2, 1.0))),
          vec![("spectrum".into(), full), ("in the band".into(), band)], "λ, wavelength (m)",
          "B, spectral radiance (W/m³)", &format!("F = {}", k::num(fb)), None, "", true)
}

fn semf_bars(s: &Solved) -> Option<Figure> {
    let (z, a) = (c::inp(s, "Z")? as i64, c::inp(s, "A")? as i64);
    let (b, vol, sur, cou, asy) = (s.out("B_MeV")?, s.out("volume_MeV")?, s.out("surface_MeV")?, s.out("coulomb_MeV")?,
                                   s.out("asymmetry_MeV")?);
    let pair = b - (vol - sur - cou - asy);
    let labels: Vec<String> = ["volume", "surface", "Coulomb", "asymmetry", "pairing", "total B"].iter().map(|x| x.to_string()).collect();
    let vals = vec![vol, -sur, -cou, -asy, k::snap(pair, b), b];
    let body = c::fit_right(&k::bars(&labels, &vals, &[5], "term", "contribution (MeV)",
                                     &format!("B = {} MeV, {} MeV per nucleon", k::num(b), k::num(s.out("B_per_A_MeV")?))));
    Some(s.chart("bars", "The terms of the semi-empirical mass formula", &format!(
        "The five terms of the semi-empirical mass formula for Z = {z}, A = {a}, each from the model with the signs the \
         formula combines them with: volume +{}, surface -{}, Coulomb -{}, asymmetry -{}, pairing {} MeV (the remainder of \
         the binding energy, derived); total binding energy B = {} MeV.", k::num(vol), k::num(sur), k::num(cou),
        k::num(asy), k::num(k::snap(pair, b)), k::num(b)), &body))
}

fn wigner_d(s: &Solved) -> Option<Figure> {
    let beta = c::inp(s, "beta")?;
    let d0 = s.out("d")?;
    let mut rows: Vec<Vec<f64>> = vec![vec![f64::NAN; 6]; 3];
    for (jj, off) in [(0.5f64, 0usize), (1.0, 3)] {
        let ms: Vec<f64> = if jj == 0.5 { vec![0.5, -0.5] } else { vec![1.0, 0.0, -1.0] };
        for (i, mp) in ms.iter().enumerate() {
            for (jx, m) in ms.iter().enumerate() {
                rows[i][off + jx] = wi(s, &[("j", f(jj)), ("mp", f(*mp)), ("m", f(*m)), ("beta", f(beta))], "d")?;
            }
        }
    }
    let rl: Vec<String> = vec!["m' = +1/2, +1".into(), "m' = -1/2, 0".into(), "m' = -1".into()];
    let cl: Vec<String> = vec!["+1/2".into(), "-1/2".into(), "".into(), "+1".into(), "0".into(), "-1".into()];
    let body = k::heat(&rows, &rl, &cl, &format!("j = 1/2 (left), j = 1 (right), β = {}°", k::num(beta * 180.0 / PI)));
    if body.is_empty() {
        return None;
    }
    Some(s.chart("matrix", "Wigner small-d matrices for a rotation", &format!(
        "Wigner small-d matrices d(β) for a rotation by β = {} rad about the y axis, j = 1/2 (left) and j = 1 (right): \
         rows m', columns m, every element a solve of the model (negative ones in the second colour). The squares are \
         the probabilities of finding m' after rotating the state m. Given element: d = {}.", k::num(beta), k::num(d0)),
                 &c::fit_right(&body)))
}

fn two_qubit_heat(s: &Solved) -> Option<Figure> {
    let mine = s.txt("gates")?;
    let mut circuits: Vec<String> = ["X0", "H0", "H0 H1", "H0 CNOT01", "X1 H0 CNOT01", "H0 CNOT01 H0"].iter().map(|x| x.to_string()).collect();
    if !circuits.contains(&mine) {
        circuits.push(mine.clone());
    }
    let outs = ["p00", "p01", "p10", "p11", "concurrence"];
    let mut m = vec![];
    for g in &circuits {
        let a = s.with_input(&[("gates", txt(g))])?;
        m.push(outs.iter().map(|o| k::num_of(&a, o).unwrap_or(f64::NAN)).collect::<Vec<f64>>());
    }
    let rl: Vec<String> = circuits.iter().map(|g| if *g == mine { format!("{g} (given)") } else { g.clone() }).collect();
    let cl: Vec<String> = vec!["P(00)".into(), "P(01)".into(), "P(10)".into(), "P(11)".into(), "C".into()];
    let body = k::heat(&m, &rl, &cl, &format!("start |{}⟩", s.txt("start")?));
    if body.is_empty() {
        return None;
    }
    let cpos = circuits.iter().position(|g| *g == mine)?;
    Some(s.chart("heatmap", "Two-qubit circuits: outcome probabilities and entanglement", &format!(
        "Two-qubit circuits started in |{}⟩, rows top to bottom: {}, with the probability of each measurement outcome \
         and the concurrence C (1 = maximally entangled), each a solve of the model. The given circuit {mine}: P(00) = {}, \
         P(11) = {}, concurrence {}.", s.txt("start")?, circuits.join("; "), k::num(m[cpos][0]), k::num(m[cpos][3]),
        k::num(m[cpos][4])), &c::fit_right(&body)))
}

/// The regime (tunnelling or over the barrier) over the particle's energy and the barrier height, both from a tenth
/// to ten times the given values on logarithmic axes; each cell one solve of the model.
fn regime_map(s: &Solved) -> Option<Figure> {
    let (e0, v0) = (c::inp(s, "E")?, c::inp(s, "V0")?);
    if !(e0 > 0.0 && v0 > 0.0) {
        return None;
    }
    let (nx, ny) = (17, 13);
    if s.left() < nx * ny {
        return None;
    }
    let xs = k::span(e0 / 10.0, e0 * 10.0, nx, true);
    let ys = k::span(v0 / 10.0, v0 * 10.0, ny, true);
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let Some(a) = s.with(&[("E", *x), ("V0", *y)]) else { continue };
            let Some(r) = a.values.iter().find(|v| v.0 == "regime").and_then(|v| v.1.as_str().map(String::from)) else { continue };
            if !names.contains(&r) {
                names.push(r.clone());
            }
            cells.push((i, j, r));
        }
    }
    if names.len() < 2 {
        return None;
    }
    let mine = s.out_text("regime")?;
    let body = c::fit_right(&k::decision(&xs, &ys, &cells, &names, (e0, v0), "E, kinetic energy (J)", "V0, barrier height (J)",
                                         &format!("given: {mine}")));
    let counts: Vec<String> = names.iter().map(|n| format!("{n}: {} cells", cells.iter().filter(|c_| c_.2 == *n).count())).collect();
    Some(s.chart("decision", "Tunnelling or over the barrier", &format!(
        "The regime over E (kinetic energy of the incident particle) and V0 (barrier height), each from a tenth to ten \
         times the given value on logarithmic axes: each of the {nx} × {ny} cells is one solve of the model, coloured by \
         the regime it returns ({}). The boundary is E = V0. Marked: the given point, {mine}.", counts.join(", ")), &body))
}

// ------------------------------------------------------------------------------------------------ tables
fn cg_table(s: &Solved, j1: f64, kind: &str) -> Option<Figure> {
    let j2 = 0.5;
    let ms = |j: f64| -> Vec<f64> { let mut v = vec![]; let mut m = j; while m >= -j - 1e-9 { v.push(m); m -= 1.0; } v };
    let (m1s, m2s) = (ms(j1), ms(j2));
    let mut coupled = vec![];
    for jj in [j1 + j2, (j1 - j2).abs()] {
        for mm in ms(jj) {
            coupled.push((jj, mm));
        }
    }
    let mut head = vec!["m1, m2".to_string()];
    head.extend(coupled.iter().map(|(jj, mm)| format!("{},{}", half(*jj), signed_half(*mm))));
    let mut rows = vec![];
    let given = (c::inp(s, "j1")?, c::inp(s, "m1")?, c::inp(s, "j2")?, c::inp(s, "m2")?, c::inp(s, "J")?, c::inp(s, "M")?);
    let mut mark = None;
    for m1 in &m1s {
        for m2 in &m2s {
            let mut r = vec![format!("{}, {}", signed_half(*m1), signed_half(*m2))];
            for (jj, mm) in &coupled {
                if (m1 + m2 - mm).abs() > 1e-9 {
                    r.push("·".into());
                    continue;
                }
                let v = wi(s, &[("j1", f(j1)), ("m1", f(*m1)), ("j2", f(j2)), ("m2", f(*m2)), ("J", f(*jj)), ("M", f(*mm))], "cg")?;
                r.push(k::num(k::snap(v, 1.0)));
            }
            if (given.0 - j1).abs() < 1e-9 && (given.2 - j2).abs() < 1e-9 && (given.1 - m1).abs() < 1e-9 && (given.3 - m2).abs() < 1e-9 {
                mark = Some(rows.len());
            }
            rows.push(r);
        }
    }
    let (body, h) = k::table(&head, &rows, mark, &format!("columns: J, M for j1 = {}, j2 = 1/2", half(j1)));
    let cg = s.out("cg")?;
    let title = if j1 == 0.5 { "Clebsch-Gordan coefficients of two spins 1/2" } else { "Clebsch-Gordan coefficients for j1 = 1 and j2 = 1/2" };
    Some(s.figure(kind, title, &format!(
        "Clebsch-Gordan coefficients for coupling j1 = {} with j2 = 1/2: the model's value for each uncoupled state (rows, \
         m1 and m2) and each coupled state (columns, J and M); a dot where M is not m1 + m2. The given coefficient: \
         ⟨{} {}; {} {} | {} {}⟩ = {}{}.", half(j1), half(given.0), signed_half(given.1), half(given.2), signed_half(given.3),
        half(given.4), signed_half(given.5), k::num(cg), if mark.is_some() { " (its row outlined)" } else { "" }), &body, h))
}

fn three_j(s: &Solved) -> Option<Figure> {
    let (j1, j2) = (c::inp(s, "j1")?, c::inp(s, "j2")?);
    if j1 > 1.0 || j2 > 1.0 || (2.0 * j1).fract() != 0.0 || (2.0 * j2).fract() != 0.0 {
        return None;
    }
    let ms = |j: f64| -> Vec<f64> { let mut v = vec![]; let mut m = j; while m >= -j - 1e-9 { v.push(m); m -= 1.0; } v };
    let mut head = vec!["J, m1".to_string()];
    head.extend(ms(j2).iter().map(|m| format!("m2 = {}", signed_half(*m))));
    let mut rows = vec![];
    let given = (c::inp(s, "m1")?, c::inp(s, "m2")?, c::inp(s, "J")?);
    let mut mark = None;
    let mut jj = (j1 - j2).abs();
    while jj <= j1 + j2 + 1e-9 {
        for m1 in ms(j1) {
            let mut r = vec![format!("{}, {}", half(jj), signed_half(m1))];
            for m2 in ms(j2) {
                let mm = -(m1 + m2);
                if mm.abs() > jj + 1e-9 {
                    r.push("·".into());
                    continue;
                }
                let v = wi(s, &[("m1", f(m1)), ("m2", f(m2)), ("J", f(jj)), ("M", f(mm))], "w3j")?;
                r.push(k::num(k::snap(v, 1.0)));
            }
            if (given.0 - m1).abs() < 1e-9 && (given.2 - jj).abs() < 1e-9 {
                mark = Some(rows.len());
            }
            rows.push(r);
        }
        jj += 1.0;
    }
    if rows.len() > 14 {
        return None;
    }
    let (body, h) = k::table(&head, &rows, mark, &format!("({} {} J; m1 m2 M) with M = -(m1 + m2)", half(j1), half(j2)));
    let w = s.out("w3j")?;
    Some(s.figure("table", "Wigner 3j symbols", &format!(
        "Wigner 3j symbols ({} {} J; m1 m2 M) with M = -(m1 + m2), for every allowed J: rows J and m1, columns m2, every \
         value a solve of the model; a dot where |M| > J. The given symbol (m1 = {}, m2 = {}, J = {}) = {}, squared {}.",
        half(j1), half(j2), signed_half(given.0), signed_half(given.1), half(given.2), k::num(w),
        k::num(s.out("w3j_squared")?)), &body, h))
}

// ------------------------------------------------------------------------------------------------ diagrams
fn bohr_orbits(s: &Solved) -> Option<Figure> {
    let (z, n0) = (c::inp(s, "Z")? as i64, c::inp(s, "n")? as i64);
    let top = n0.max(4).min(8);
    let mut lv = vec![];
    for n in 1..=top {
        let a = s.with_input(&[("n", int(n))])?;
        lv.push((n, k::num_of(&a, "r_nm")?, k::num_of(&a, "E_eV")?, k::num_of(&a, "v")?));
    }
    let rmax = lv.last()?.1;
    let mut cv = Canvas::new(420.0);
    let ctr = (175.0, 225.0);
    let pr = 160.0 / rmax;
    c::disc(&mut cv, ctr, 5.0, true);
    for (n, r, e, v) in &lv {
        c::ellipse(&mut cv, ctr, (r * pr).max(2.0), (r * pr).max(2.0), if *n == n0 { "axp-c" } else { "axp-c4" });
        let row = *n as f64 - 1.0;
        cv.value(355.0, 60.0 + 44.0 * row, "start", &format!("n = {n}: {} nm", k::num(*r)));
        cv.name(355.0, 76.0 + 44.0 * row, "start", &format!("{} eV, {} km/s", k::num(*e), k::num(v / 1000.0)));
    }
    cv.bold(16.0, 24.0, "start", &format!("Z = {z}, orbits to scale"));
    fig(s, "orbit", "Bohr orbits of a one-electron atom", &format!(
        "Bohr orbits of a one-electron atom with Z = {z} for n = 1 to {top} drawn to scale, the radius, energy and speed \
         of each from the model: {}. Highlighted: the given n = {n0}.", lv.iter().map(|(n, r, e, v)| format!(
            "n = {n}: r = {} nm, E = {} eV, v = {} m/s", k::num(*r), k::num(*e), k::num(*v))).collect::<Vec<_>>().join("; ")), &cv)
}

fn aberration(s: &Solved) -> Option<Figure> {
    let (th, v, tp) = (c::inp(s, "theta")?, c::inp(s, "v")?, s.out("theta_prime")?);
    let mut cv = Canvas::new(300.0);
    let o = (260.0, 270.0);
    cv.line((40.0, o.1), (480.0, o.1));
    let mut notes = vec![];
    for i in 1..12 {
        let a = PI * i as f64 / 12.0;
        let Some(b) = c::at(s, &[("theta", a)], "theta_prime") else { continue };
        cv.raw(&format!("<path class=\"axp-c4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", o.0, o.1, o.0 + 200.0 * a.cos(), o.1 - 200.0 * a.sin()));
        c::varrow(&mut cv, o, (o.0 + 230.0 * b.cos(), o.1 - 230.0 * b.sin()), "axp-c3");
        notes.push(format!("{}°→{}°", k::num(a * 180.0 / PI), k::num(b * 180.0 / PI)));
    }
    c::varrow(&mut cv, o, (o.0 + 250.0 * tp.cos(), o.1 - 250.0 * tp.sin()), "axp-c");
    cv.bold(16.0, 24.0, "start", &format!("v = {} c", k::num(v / 299792458.0)));
    cv.value(16.0, 44.0, "start", &format!("given: {}° → {}°", k::num(th * 180.0 / PI), k::num(tp * 180.0 / PI)));
    cv.name(480.0, o.1 + 18.0, "end", "+x, the direction of motion");
    fig(s, "diagram", "Relativistic aberration of light", &format!(
        "Relativistic aberration at relative speed v = {} c: rays at 15° steps from the x axis in one frame (grey) and the \
         directions the model gives in the other (violet arrows): {}. Marked: the given direction {}° becomes {}°.",
        k::num(v / 299792458.0), notes.join(", "), k::num(th * 180.0 / PI), k::num(tp * 180.0 / PI)), &cv)
}

fn minkowski(s: &Solved) -> Option<Figure> {
    let (dt, dx) = (c::inp(s, "dt")?, c::inp(s, "dx")?);
    let cl = 299792458.0;
    let evs = [(dt, dx), (dt, dx * 2.5), (dt * 0.5, dx * 0.2)];
    let xs: Vec<f64> = evs.iter().map(|e| (e.1 / cl).abs()).chain(evs.iter().map(|e| e.0.abs())).collect();
    let ext = xs.iter().cloned().fold(0.0, f64::max) * 1.2;
    if !(ext > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(420.0);
    let o = (260.0, 380.0);
    let sc = 210.0 / ext;
    cv.line((40.0, o.1), (480.0, o.1));
    cv.line((o.0, o.1), (o.0, 40.0));
    cv.name(480.0, o.1 + 18.0, "end", "x (light-seconds)");
    cv.name(o.0 + 6.0, 50.0, "start", "c t (s)");
    c::poly(&mut cv, &[(o.0 - ext * sc, o.1 - ext * sc), o, (o.0 + ext * sc, o.1 - ext * sc)], "axp-c4", true);
    c::area(&mut cv, &[(o.0 - ext * sc, o.1 - ext * sc), o, (o.0 + ext * sc, o.1 - ext * sc)], "axp-s4");
    let mut notes = vec![];
    for (i, (t, x)) in evs.iter().enumerate() {
        let a = s.with(&[("dt", *t), ("dx", *x)])?;
        let nat = a.values.iter().find(|v| v.0 == "nature").and_then(|v| v.1.as_str().map(String::from))?;
        let p = (o.0 + x / cl * sc, o.1 - t * sc);
        c::disc(&mut cv, p, 6.0, true);
        let extra = if nat == "timelike" { format!("τ = {}", k::q(k::num_of(&a, "tau")?, "s")) }
                    else if nat == "spacelike" { format!("length {} ls", k::num(k::num_of(&a, "proper_length")? / cl)) }
                    else { String::new() };
        cv.value(p.0 + 9.0, p.1 - 6.0, "start", &format!("{}{nat}", if i == 0 { "given: " } else { "" }));
        cv.name(p.0 + 9.0, p.1 + 10.0, "start", &extra);
        notes.push(format!("Δt = {}, Δx = {} ls: {nat} {extra}", k::q(*t, "s"), k::num(x / cl)));
    }
    cv.bold(16.0, 24.0, "start", "light cone: |x| = c t (dashed)");
    fig(s, "diagram", "Events on a Minkowski diagram", &format!(
        "A Minkowski diagram with the origin event and three others (the given one, one 2.5 times farther in space, one \
         at half the time and a fifth of the distance; distances in light-seconds = Δx / c): events inside the light cone \
         are timelike (a proper time τ), outside spacelike (a proper length), each classification and value from the \
         model: {}.", notes.join("; ")), &cv)
}

fn triangular_barrier(s: &Solved) -> Option<Figure> {
    let (w, fld) = (c::inp(s, "W")?, c::inp(s, "F")?);
    let ev = 1.602176634e-19;
    let mut cv = Canvas::new(340.0);
    let mut cases = vec![];
    for ff in [fld, 2.0 * fld] {
        let a = s.with(&[("F", ff)])?;
        cases.push((ff, k::num_of(&a, "width")?, k::num_of(&a, "T")?));
    }
    let wmax = cases[0].1 * 1.3;
    let px = c::lin(0.0, wmax, 120.0, 490.0);
    let py = c::lin(0.0, w / ev * 1.3, 270.0, 50.0);
    cv.raw("<rect class=\"axp-s4\" x=\"40\" y=\"50\" width=\"80\" height=\"220\"/>");
    cv.name(80.0, 290.0, "middle", "metal");
    cv.line((40.0, 270.0), (500.0, 270.0));
    cv.name(500.0, 286.0, "end", "Fermi level");
    for (i, (ff, wd, t)) in cases.iter().enumerate() {
        let class = if i == 0 { "axp-c" } else { "axp-c3" };
        c::poly(&mut cv, &[(120.0, py(w / ev)), (px(*wd), 270.0)], class, false);
        cv.value(px(*wd) + 4.0, 258.0 - 18.0 * i as f64, "start", &format!("{} nm", k::num(wd * 1e9)));
        cv.value(300.0, 80.0 + 20.0 * i as f64, "start", &format!("F = {} V/m: T = {}", k::num(*ff), k::num(*t)));
        c::varrow(&mut cv, (120.0, 270.0 - 10.0 - 8.0 * i as f64), (px(*wd), 270.0 - 10.0 - 8.0 * i as f64), class);
    }
    cv.line((120.0, py(w / ev)), (120.0, 270.0));
    cv.name(116.0, py(w / ev) - 6.0, "end", &format!("W = {} eV", k::num(w / ev)));
    cv.bold(16.0, 24.0, "start", "a triangular barrier: the field tilts the vacuum level");
    fig(s, "diagram", "Field emission through a triangular barrier", &format!(
        "Field emission through the triangular barrier of a metal with work function W = {} eV at F = {} V/m and at twice \
         that field: the barrier falls from W at the surface and crosses the Fermi level at the width the model returns \
         ({} nm and {} nm; the sloped lines are drawn through those two points); arrows: the tunnelling path. Tunnelling \
         probabilities {} and {}.", k::num(w / ev), k::num(fld), k::num(cases[0].1 * 1e9), k::num(cases[1].1 * 1e9),
        k::num(cases[0].2), k::num(cases[1].2)), &cv)
}

fn decay(s: &Solved) -> Option<Figure> {
    let (m, m1, m2) = (c::inp(s, "M")?, c::inp(s, "m1")?, c::inp(s, "m2")?);
    let (q, p, e1, e2, k1, k2) = (s.out("Q_MeV")?, s.out("p_MeV")?, s.out("E1_MeV")?, s.out("E2_MeV")?, s.out("K1_MeV")?,
                                  s.out("K2_MeV")?);
    let mut cv = Canvas::new(200.0);
    let ctr = (260.0, 110.0);
    c::disc(&mut cv, ctr, 16.0, true);
    cv.name(ctr.0, ctr.1 - 24.0, "middle", &format!("M = {}", k::num(m)));
    c::varrow(&mut cv, (ctr.0 - 20.0, ctr.1), (ctr.0 - 160.0, ctr.1), "axp-c");
    c::varrow(&mut cv, (ctr.0 + 20.0, ctr.1), (ctr.0 + 160.0, ctr.1), "axp-c2");
    cv.value(ctr.0 - 90.0, ctr.1 - 10.0, "middle", &format!("p = {} MeV/c", k::num(p)));
    cv.value(ctr.0 + 90.0, ctr.1 - 10.0, "middle", &format!("p = {} MeV/c", k::num(p)));
    cv.name(ctr.0 - 170.0, ctr.1 + 26.0, "middle", &format!("m1 = {}: E {}, K {} MeV", k::num(m1), k::num(e1), k::num(k1)));
    cv.name(ctr.0 + 170.0, ctr.1 + 46.0, "middle", &format!("m2 = {}: E {}, K {} MeV", k::num(m2), k::num(e2), k::num(k2)));
    cv.bold(16.0, 24.0, "start", &format!("Q = {} MeV", k::num(q)));
    fig(s, "diagram", "A two-body decay at rest", &format!(
        "A particle of mass M = {} at rest decaying into m1 = {} and m2 = {}: the products fly apart back to back with equal \
         momentum. Marked: the model's p = {} MeV/c, energies E1 = {} and E2 = {} MeV, kinetic energies K1 = {} and K2 = {} \
         MeV, Q = {} MeV.", k::num(m), k::num(m1), k::num(m2), k::num(p), k::num(e1), k::num(e2), k::num(k1), k::num(k2),
        k::num(q)), &cv)
}

