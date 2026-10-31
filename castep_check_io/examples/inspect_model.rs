//! Inspect a CASTEP `.check` / `.castep_bin` model file.
//!
//! Prints the document summary, selected parameters, the cell, the
//! electronic section, and a charge-density audit.
//!
//! Usage:
//!   cargo run -p castep-check-io --example inspect_model -- <file>

use castep_check_io::{CheckDocument, ParamsLayout};
use std::path::Path;

fn parse_with_fallback(path: &Path) -> CheckDocument {
    let data = std::fs::read(path).expect("read model file");
    match CheckDocument::from_bytes(&data) {
        Ok(doc) => {
            eprintln!("parsed with the standard layout (devel_code_len=80)");
            doc
        }
        Err(e) => {
            eprintln!("standard layout failed: {e}; retrying with devel_code_len=256");
            let wide = ParamsLayout { devel_code_len: 256 };
            CheckDocument::from_bytes_with(&data, &wide)
                .unwrap_or_else(|e2| panic!("wide layout also failed: {e2}"))
        }
    }
}

fn f64s(payload: &[u8]) -> Vec<f64> {
    payload
        .chunks_exact(8)
        .map(|c| f64::from_be_bytes(c.try_into().unwrap()))
        .collect()
}

fn i32s(payload: &[u8]) -> Vec<i32> {
    payload
        .chunks_exact(4)
        .map(|c| i32::from_be_bytes(c.try_into().unwrap()))
        .collect()
}

fn char_str(payload: &[u8]) -> String {
    let end = payload.iter().position(|&b| b == 0).unwrap_or(payload.len());
    String::from_utf8_lossy(&payload[..end]).into_owned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: inspect_model <file.check|file.castep_bin>");
        std::process::exit(2);
    }
    let doc = parse_with_fallback(Path::new(&args[1]));

    println!("=== document ===");
    println!(
        "kind={:?}  layout.devel_code_len={}",
        doc.kind, doc.layout.devel_code_len
    );

    println!("\n=== parameters (selected) ===");
    for key in [
        "task",
        "xc_functional",
        "cutoff_energy",
        "smearing_width",
        "kpoint_mp_spacing",
        "kpoint_mp_density",
        "fixed_cell",
        "elecenergytol",
        "mixing_scheme",
        "metal_method",
    ] {
        let val = if let Ok(Some(v)) = doc.params.f64(key) {
            format!("{v}")
        } else if let Ok(Some(v)) = doc.params.boolean(key) {
            v.to_string()
        } else if let Some(v) = doc.params.text(key) {
            v.to_string()
        } else {
            "(absent)".to_string()
        };
        println!("  {key:<20} = {val}");
    }

    let cell = &doc.cell;
    let global = &doc.cell_global;
    let counts = cell.counts();

    println!("\n=== cell ===");
    if let Some(l) = cell.get("CELL%REAL_LATTICE") {
        let l = f64s(l);
        println!(
            "real_lattice (bohr): [{}  {}  {}] [{}  {}  {}] [{}  {}  {}]",
            l[0], l[1], l[2], l[3], l[4], l[5], l[6], l[7], l[8]
        );
        let lx = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]).sqrt();
        println!(
            "  |lat1| = {lx:.4} bohr = {:.4} Ang",
            lx * 0.529177210903
        );
    }
    if let Some(v) = cell.get("CELL%VOLUME") {
        println!("volume: {:.6e} bohr^3", f64s(v)[0]);
    }
    println!(
        "ns={} mi={} nkpts={} nso={}",
        counts.ns, counts.mi, global.counts().nkpts, counts.nso
    );
    if let Some(sp) = cell.get("CELL%SPECIES_SYMBOL") {
        let syms: Vec<String> = sp
            .chunks_exact(8)
            .map(|c| char_str(c).trim().to_string())
            .collect();
        println!("species: {syms:?}");
    }
    if let Some(nis) = cell.get("CELL%NUM_IONS_IN_SPECIES") {
        println!("ions per species: {:?}", i32s(nis));
    }
    if let Some(pos) = cell.get("CELL%IONIC_POSITIONS") {
        let p = f64s(pos);
        let n = p.len() / 3;
        println!("ionic_positions (frac, {n} slots):");
        for i in 0..n.min(8) {
            println!("  atom {}: {} {} {}", i, p[3 * i], p[3 * i + 1], p[3 * i + 2]);
        }
    }
    if let Some(kg) = global.get("KPOINT_MP_GRID") {
        println!("kpoint_mp_grid: {:?}", i32s(kg));
    }

    let elec = &doc.electronic;
    println!("\n=== electronic ===");
    println!(
        "found_gs_wvfn={} found_gs_den={} nbands_max={} nspins={}",
        elec.found_gs_wvfn, elec.found_gs_den, elec.nbands_max, elec.nspins
    );
    println!("total_energy = {:.6e} Ha", elec.total_energy);
    println!("fermi = {:.6e} Ha", elec.fermi);
    println!("fine_grid = {:?}", elec.fine_grid);
    println!(
        "eigen: {} k-points, wave block = {}",
        elec.eigen.len(),
        elec.wave.is_some()
    );
    for (i, k) in elec.eigen.iter().take(4).enumerate() {
        let kc = f64s(&k.kcoords);
        let (occ, eig) = &k.spins[0];
        let ev: Vec<f64> = f64s(eig);
        let ov: Vec<f64> = f64s(occ);
        println!(
            "  k{} = [{:.6} {:.6} {:.6}] eig[0]={:.6e} eig[last]={:.6e} occ[0]={:.3} occ[last]={:.3}",
            i, kc[0], kc[1], kc[2], ev[0], ev[ev.len() - 1], ov[0], ov[ov.len() - 1]
        );
    }

    println!("\n=== density audit ===");
    let ngx = elec.fine_grid[0].max(0) as usize;
    let ngy = elec.fine_grid[1].max(0) as usize;
    let ngz = elec.fine_grid[2].max(0) as usize;
    let nspins = elec.nspins.max(1) as usize;
    let mut total_charge = 0.0f64;
    let mut max_imag = 0.0f64;
    let mut values = 0usize;
    for rec in &elec.density_columns {
        if rec.len() < 8 {
            continue;
        }
        let _ix = i32::from_be_bytes(rec[0..4].try_into().unwrap());
        let _iy = i32::from_be_bytes(rec[4..8].try_into().unwrap());
        for s in 0..nspins {
            for k in 0..ngz {
                let off = 8 + (s * ngz + k) * 16;
                if off + 16 > rec.len() {
                    break;
                }
                let re = f64::from_be_bytes(rec[off..off + 8].try_into().unwrap());
                let im = f64::from_be_bytes(rec[off + 8..off + 16].try_into().unwrap());
                total_charge += re;
                max_imag = max_imag.max(im.abs());
                values += 1;
            }
        }
    }
    println!(
        "columns: {} (expected {}), values read: {}",
        elec.density_columns.len(),
        ngx * ngy,
        values
    );
    println!("sum(charge) = {total_charge:.6}  (expected electron count)");
    println!("max|imag(charge)| = {max_imag:.3e}");

    println!("\n=== optional sections ===");
    for s in &elec.optional {
        println!("  {:<24} values={}", s.header, s.values.len());
    }
}
