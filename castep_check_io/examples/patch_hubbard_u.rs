//! Patch one Hubbard U entry of a `.check` file.
//!
//! The `CELL%HUBBARD_U` record of a continuation `.check` holds the U of
//! the model cell that wrote it. A continued run can use a different U in
//! its live cell, and the record is not updated on convergence. This
//! example sets one entry (species, ion, orbital) in eV, so a follow-up
//! run whose `.cell` U differs detects the change and forces a real SCF.
//!
//! Usage: `cargo run -p castep-check-io --example patch_hubbard_u
//!   [--devel-code-len N] <in.check> <out.check> <species> <ion> <orbital> <value_ev>`
//!
//! The live cell section and the original cell section are patched.
//! Every other record is preserved byte-exact.

use castep_check_io::{cell::RawCell, CheckDocument, CheckKind, ParamsLayout};

/// eV per Hartree, recovered from the target binary encoding of 5.0 eV
/// as 0.18374662193776414 Ha.
const EV_PER_HA: f64 = 27.21138460816724;

/// The species index (0-based) of one symbol in the cell section.
fn resolve_species(section: &RawCell, species: &str, ns: i32) -> usize {
    let raw = section.get("CELL%SPECIES_SYMBOL").expect("read CELL%SPECIES_SYMBOL");
    let symbols: Vec<String> = raw
        .chunks_exact(8)
        .map(|c| std::str::from_utf8(c).expect("the symbol is ascii").trim().to_string())
        .collect();
    assert_eq!(symbols.len(), ns as usize, "the symbol count does not match");
    symbols
        .iter()
        .position(|s| s == species)
        .unwrap_or_else(|| die(&format!("species {species} not in {symbols:?}")))
}

/// Patches one U entry in place and prints the old and new value.
fn patch_entry(section: &mut RawCell, species: &str, ion: usize, orbital: &str, value_ev: f64) {
    let counts = section.counts();
    let sp = resolve_species(section, species, counts.ns);
    if ion < 1 || ion > counts.mi as usize {
        die(&format!("ion {ion} is out of range 1..{}", counts.mi));
    }
    let orb = match orbital {
        "s" => 0,
        "p" => 1,
        "d" => 2,
        "f" => 3,
        other => die(&format!("orbital must be s, p, d or f, got {other}")),
    };
    // Fortran column-major (orbital, ion, species):
    // index = orb + 4*(ion-1) + 4*mi*(sp-1).
    let mut vals = section.f64s("CELL%HUBBARD_U").expect("read CELL%HUBBARD_U");
    assert_eq!(
        vals.len(),
        4 * counts.mi as usize * counts.ns as usize,
        "the U array size does not match the counts"
    );
    let idx = orb + 4 * (ion - 1) + 4 * counts.mi as usize * sp;
    let old = vals[idx];
    vals[idx] = value_ev / EV_PER_HA;
    let mut payload = Vec::with_capacity(vals.len() * 8);
    for v in &vals {
        payload.extend_from_slice(&v.to_be_bytes());
    }
    section.set_payload("CELL%HUBBARD_U", payload);
    println!(
        "{species} ion {ion} {orbital}: {old} Ha ({old_ev:.4} eV) -> {new} Ha ({new_ev:.4} eV)",
        old = old,
        old_ev = old * EV_PER_HA,
        new = vals[idx],
        new_ev = value_ev
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    let mut layout = ParamsLayout::default();
    while i < args.len() && args[i].starts_with("--") {
        match args[i].as_str() {
            "--devel-code-len" => {
                i += 1;
                layout.devel_code_len = args
                    .get(i)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| die("--devel-code-len needs a value"));
            }
            other => die(&format!("unknown flag {other}")),
        }
        i += 1;
    }
    let rest: Vec<&String> = args.iter().skip(i).collect();
    if rest.len() != 6 {
        die("usage: [--devel-code-len N] <in.check> <out.check> <species> <ion> <orbital> <value_ev>");
    }
    let (in_path, out_path, species, ion, orbital, value_ev) = (
        rest[0].as_str(),
        rest[1].as_str(),
        rest[2].as_str(),
        rest[3].parse::<usize>().expect("ion must be a number"),
        rest[4].as_str(),
        rest[5].parse::<f64>().expect("value must be a number"),
    );

    let mut doc =
        CheckDocument::from_check_file_with(in_path, &layout).expect("read the .check file");
    assert!(
        matches!(doc.kind, CheckKind::Check),
        "the input must be a .check file"
    );

    patch_entry(&mut doc.cell, species, ion, orbital, value_ev);
    patch_entry(&mut doc.orig_cell, species, ion, orbital, value_ev);
    doc.to_check_file(out_path).expect("write the patched .check file");

    // Verify by re-reading the written file and checking the slot.
    let check =
        CheckDocument::from_check_file_with(out_path, &layout).expect("re-read the written file");
    for section in [&check.cell, &check.orig_cell] {
        let counts = section.counts();
        let sp = resolve_species(section, species, counts.ns);
        let orb = match orbital {
            "s" => 0,
            "p" => 1,
            "d" => 2,
            _ => 3,
        };
        let vals = section.f64s("CELL%HUBBARD_U").expect("read CELL%HUBBARD_U");
        let idx = orb + 4 * (ion - 1) + 4 * counts.mi as usize * sp;
        assert_eq!(vals[idx], value_ev / EV_PER_HA, "the written slot did not stick");
    }
    println!(
        "wrote {out_path} (from {in_path}, layout.devel_code_len={}, verified)",
        layout.devel_code_len
    );
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}
