//! Byte-exact roundtrips on real CASTEP model files.
//!
//! The fixture paths resolve against the sibling
//! `chemrust-hamiltonian` checkout. A missing fixture skips the test
//! rather than failing it.

use castep_check_io::document::CheckDocument;
use castep_check_io::version::CheckKind;

fn fixture(name: &str) -> Option<std::path::PathBuf> {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("chemrust-hamiltonian/chemrust-hamiltonian-core/tests/fixtures")
        .join(name);
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

fn roundtrip_bytes(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let doc = CheckDocument::from_bytes(data).expect("parse");
    let is_check = doc.kind == CheckKind::Check;
    let check = if is_check {
        doc.to_check_bytes().expect("write check")
    } else {
        Vec::new()
    };
    let bin = if is_check {
        Vec::new()
    } else {
        doc.to_castep_bin_bytes().expect("write bin")
    };
    (check, bin)
}

fn assert_same(a: &[u8], b: &[u8]) {
    assert_eq!(a.len(), b.len(), "length mismatch");
    let diff = a
        .iter()
        .zip(b.iter())
        .position(|(x, y)| x != y)
        .unwrap_or(a.len());
    assert_eq!(diff, a.len(), "first differing byte offset {diff}");
}

#[test]
fn castep_bin_roundtrip_is_byte_exact() {
    let Some(path) = fixture("Cu111_CO.castep_bin") else {
        eprintln!("SKIP: Cu111_CO.castep_bin fixture not found");
        return;
    };
    let data = std::fs::read(&path).expect("read fixture");
    let (check, bin) = roundtrip_bytes(&data);
    assert!(check.is_empty(), "a .castep_bin file must not gain a wave block");
    assert_same(&bin, &data);
}

#[test]
fn check_roundtrip_is_byte_exact() {
    let Some(path) = fixture("Cu111_CO.check") else {
        eprintln!("SKIP: Cu111_CO.check fixture not found");
        return;
    };
    let data = std::fs::read(&path).expect("read fixture");
    let (check, bin) = roundtrip_bytes(&data);
    assert!(bin.is_empty(), "a .check file must not drop to .castep_bin bytes");
    assert_same(&check, &data);
}

#[test]
fn ferrous_oxide_roundtrip_is_byte_exact() {
    let Some(path) = fixture("Fe2O3.castep_bin") else {
        eprintln!("SKIP: Fe2O3.castep_bin fixture not found");
        return;
    };
    let data = std::fs::read(&path).expect("read fixture");
    let (check, bin) = roundtrip_bytes(&data);
    assert!(check.is_empty());
    assert_same(&bin, &data);
}

#[test]
fn spin_job_roundtrip_is_byte_exact() {
    let path = std::path::Path::new(
        "/export/public_castep_jobs/tony/Cu111_CO_Single_Point_0614_spin_cpu_serial/Cu111_CO.check",
    );
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("SKIP: {path:?} not readable");
            return;
        }
    };
    let (check, bin) = roundtrip_bytes(&data);
    assert!(bin.is_empty());
    assert_same(&check, &data);
}

#[test]
fn f7_check_roundtrip_is_byte_exact() {
    let Some(path) = fixture("Cu111_CO_F7.check") else {
        eprintln!("SKIP: Cu111_CO_F7.check fixture not found");
        return;
    };
    let data = std::fs::read(&path).expect("read fixture");
    let (check, bin) = roundtrip_bytes(&data);
    assert!(bin.is_empty());
    assert_same(&check, &data);
}
