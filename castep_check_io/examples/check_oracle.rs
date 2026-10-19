//! Oracle harness for the CASTEP write test.
//!
//! Loads a seed `.check` file with `CheckDocument`, writes it into the
//! oracle job directory, and proves the bytes are intact. CASTEP then
//! reads the file back. A matching stored model makes the ground-state
//! reuse flag true and skips the SCF.
//!
//! Usage: `cargo run -p castep-check-io --example check_oracle -- <src.check> <dst_dir> <seedname>`

use castep_check_io::CheckDocument;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: check_oracle <src.check> <dst_dir> <seedname>");
        std::process::exit(2);
    }
    let src = std::path::Path::new(&args[1]);
    let dst_dir = std::path::Path::new(&args[2]);
    let seedname = args[3].as_str();

    let data = std::fs::read(src).expect("read the seed .check file");
    let doc = CheckDocument::from_bytes(&data).expect("parse the seed .check file");
    assert_eq!(
        doc.kind,
        castep_check_io::CheckKind::Check,
        "the oracle seed must be a .check file"
    );

    std::fs::create_dir_all(dst_dir).expect("create the job directory");
    let dst = dst_dir.join(format!("{seedname}.check"));
    doc.to_check_file(&dst).expect("write the oracle .check file");

    let written = std::fs::read(&dst).expect("re-read the written file");
    assert_eq!(
        written.len(),
        data.len(),
        "byte count changed: {} vs {}",
        written.len(),
        data.len()
    );
    let diff = written
        .iter()
        .zip(data.iter())
        .position(|(a, b)| a != b)
        .unwrap_or(data.len());
    assert_eq!(diff, data.len(), "first differing byte at offset {diff}");

    // The ground-state flags must be set for the reuse oracle to work.
    assert!(
        doc.electronic.found_gs_wvfn && doc.electronic.found_gs_den,
        "the seed model did not find a ground state; reuse will not trigger"
    );
    println!(
        "oracle .check written: {} bytes, energy {}, optional sections {:?}",
        written.len(),
        doc.electronic.total_energy,
        doc.electronic
            .optional
            .iter()
            .map(|s| s.header.clone())
            .collect::<Vec<_>>()
    );
}
