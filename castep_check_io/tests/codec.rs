//! Codec edge cases: error paths and synthetic record streams.

use castep_check_io::record::{
    char_str, parse_c128s, parse_f64s, parse_i32s, parse_logical4s, Complex, RecordReader,
    RecordWriter,
};
use castep_check_io::version::{check_file_version, gate_open, parse_version_record};
use castep_check_io::FileVersion;

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

#[test]
fn truncated_input_is_an_error() {
    let data = [1u8, 2, 3];
    let mut reader = RecordReader::new(data.as_slice());
    assert!(reader.read_record().is_err());
}

#[test]
fn marker_mismatch_is_an_error() {
    // header says 5, payload is 5, trailer says 3.
    let mut out = Vec::new();
    out.extend_from_slice(&5u32.to_be_bytes());
    out.extend_from_slice(&[1, 2, 3, 4, 5]);
    out.extend_from_slice(&3u32.to_be_bytes());
    let mut reader = RecordReader::new(out.as_slice());
    assert!(reader.read_record().is_err());
}

#[test]
fn read_fixed_rejects_wrong_size() {
    let mut out = Vec::new();
    {
        let mut w = RecordWriter::new(&mut out);
        w.write_record(&[1u8, 2, 3]).unwrap();
    }
    let mut reader = RecordReader::new(out.as_slice());
    assert!(reader.read_fixed(4).is_err());
}

#[test]
fn logical_parse_rejects_bad_values() {
    assert!(parse_logical4s(&[0u8, 0, 0, 0]).is_ok());
    assert!(parse_logical4s(&[0u8, 0, 0, 1]).is_ok());
    assert!(parse_logical4s(&[0u8, 0, 0, 2]).is_err());
    assert!(parse_logical4s(&[0u8, 0, 0]).is_err());
}

#[test]
fn payload_lengths_must_divide_evenly() {
    assert!(parse_i32s(&[0u8, 0, 0, 0, 1]).is_err());
    assert!(parse_f64s(&[0u8; 7]).is_err());
    let parsed = parse_c128s(&[0u8; 16]).unwrap();
    assert_eq!(parsed, vec![Complex::new(0.0, 0.0)]);
    assert!(parse_c128s(&[0u8; 15]).is_err());
}

#[test]
fn char_fields_trim_trailing_spaces() {
    let buf = b"CASTEP    ".to_vec();
    assert_eq!(char_str(&buf), "CASTEP");
    let mut out = Vec::new();
    let mut w = RecordWriter::new(&mut out);
    w.write_char_padded("AB", 4).unwrap();
    w.write_char_raw(b"XY").unwrap();
    w.write_i32s(&[7]).unwrap();
    w.write_f64s(&[1.5]).unwrap();
    w.write_c128s(&[Complex::new(1.0, -1.0)]).unwrap();
    w.write_logical4s(&[true, false]).unwrap();
    let bytes = w.into_inner();
    // Two char records, one i32, one f64, one c128, one logical record.
    let mut reader = RecordReader::new(bytes.as_slice());
    assert_eq!(char_str(&reader.read_record().unwrap()), "AB");
    assert_eq!(char_str(&reader.read_record().unwrap()), "XY");
    assert_eq!(parse_i32s(&reader.read_record().unwrap()).unwrap(), vec![7]);
    assert_eq!(parse_f64s(&reader.read_record().unwrap()).unwrap(), vec![1.5]);
    assert_eq!(
        parse_c128s(&reader.read_record().unwrap()).unwrap(),
        vec![Complex::new(1.0, -1.0)]
    );
    assert_eq!(
        parse_logical4s(&reader.read_record().unwrap()).unwrap(),
        vec![true, false]
    );
}

#[test]
fn version_checks_reject_out_of_range() {
    assert!(check_file_version(FileVersion::new(6, 11)).is_ok());
    assert!(check_file_version(FileVersion::new(2, 9)).is_err());
    assert!(check_file_version(FileVersion::new(9, 0)).is_err());
}

#[test]
fn version_record_roundtrips() {
    let rec = castep_check_io::version::target_version_record();
    assert_eq!(rec.len(), 10);
    let parsed = parse_version_record(&rec).unwrap();
    assert_eq!(parsed, castep_check_io::version::target_file_version());
    assert!(parse_version_record(b"xxxx").is_err());
}

#[test]
fn gate_table_orders_versions() {
    use castep_check_io::version::gate;
    let v = FileVersion::new(6, 11);
    assert!(gate_open(v, gate::V6_1));
    assert!(gate_open(v, gate::V3_0));
    let old = FileVersion::new(3, 0);
    assert!(gate_open(old, gate::V3_0));
    assert!(!gate_open(old, gate::V3_1));
}

#[test]
fn params_read_real_fixture() {
    let Some(path) = fixture("Cu111_CO.castep_bin") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let data = std::fs::read(&path).unwrap();
    let mut reader = RecordReader::new(data.as_slice());
    let params = castep_check_io::params::read_params(&mut reader, true, &castep_check_io::ParamsLayout::default()).unwrap();
    assert_eq!(params.version, FileVersion::new(6, 11));
    assert_eq!(params.text("xc_functional").unwrap(), "PBE");
    let seed = params.text("seedname").unwrap();
    assert_eq!(seed, "Cu111_CO");
}

#[test]
fn params_write_fills_absent_records() {
    use castep_check_io::params::{self, RawParams};
    use castep_check_io::version::target_file_version;
    let mut p = RawParams {
        version: target_file_version(),
        payloads: vec![],
    };
    p.set_i32("iprint", 1).unwrap();
    p.set_text("seedname", "X").unwrap();
    let mut out = Vec::new();
    let mut w = RecordWriter::new(&mut out);
    params::write_params(&mut w, &p, false, &castep_check_io::ParamsLayout::default()).unwrap();
    // The full 340-record stream must re-parse.
    let mut reader = RecordReader::new(out.as_slice());
    let reparsed = params::read_params(&mut reader, false, &castep_check_io::ParamsLayout::default()).unwrap();
    assert_eq!(reparsed.i32("iprint").unwrap(), Some(1));
    assert_eq!(reparsed.text("seedname"), Some("X"));
}

#[test]
fn cell_read_real_fixture() {
    let Some(path) = fixture("Cu111_CO.castep_bin") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let data = std::fs::read(&path).unwrap();
    let mut reader = RecordReader::new(data.as_slice());
    let _ = castep_check_io::params::read_params(&mut reader, true, &castep_check_io::ParamsLayout::default()).unwrap();
    use castep_check_io::cell::{read_cell_section, Counts, UNIT_CELL};
    let cell = read_cell_section(&mut reader, UNIT_CELL, &Counts::default()).unwrap();
    assert_eq!(cell.i32s("CELL%NUM_SPECIES").unwrap(), vec![3]);
    assert_eq!(cell.f64s("CELL%VOLUME").unwrap().len(), 1);
    assert_eq!(cell.text("CELL%SPECIES_SYMBOL"), Some("C       O       Cu"));
}

#[test]
fn cell_write_missing_value_is_an_error() {
    use castep_check_io::cell::{write_cell_section, Counts, RawCell, UNIT_CELL};
    let raw = RawCell::new();
    let mut out = Vec::new();
    let mut w = RecordWriter::new(&mut out);
    let err = write_cell_section(&mut w, &raw, UNIT_CELL, &Counts::default()).unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::Section(_)));
}

#[test]
fn electronic_unknown_header_consumes_raw_values() {
    // A synthetic stream: END after one unknown section with two values.
    use castep_check_io::record::RecordWriter;
    let mut out = Vec::new();
    {
        let mut w = RecordWriter::new(&mut out);
        w.write_i32s(&[0]).unwrap();
        w.write_i32s(&[0]).unwrap();
        w.write_f64s(&[0.0]).unwrap();
        w.write_f64s(&[0.0]).unwrap();
        w.write_i32s(&[0, 0]).unwrap();
        w.write_i32s(&[0]).unwrap();
        w.write_i32s(&[0, 0, 0]).unwrap();
        w.write_char_padded("ZZTEST", 10).unwrap();
        w.write_record(&[9u8; 40]).unwrap();
        w.write_record(&[9u8; 8]).unwrap();
        w.write_char_padded("END", 10).unwrap();
    }
    let mut reader = RecordReader::new(out.as_slice());
    let elec = castep_check_io::electronic::read_electronic(
        &mut reader,
        false,
        0,
        &castep_check_io::cell::Counts::default(),
    )
    .unwrap();
    assert_eq!(elec.optional.len(), 1);
    assert_eq!(elec.optional[0].header, "ZZTEST");
    assert_eq!(elec.optional[0].values.len(), 2);
    assert_eq!(elec.optional[0].values[0].len(), 40);
}

#[test]
fn value_before_header_is_an_error() {
    let mut out = Vec::new();
    {
        let mut w = RecordWriter::new(&mut out);
        w.write_i32s(&[0]).unwrap();
        w.write_i32s(&[0]).unwrap();
        w.write_f64s(&[0.0]).unwrap();
        w.write_f64s(&[0.0]).unwrap();
        w.write_i32s(&[0, 0]).unwrap();
        w.write_i32s(&[0]).unwrap();
        w.write_i32s(&[0, 0, 0]).unwrap();
        w.write_record(&[9u8; 40]).unwrap();
        w.write_char_padded("END", 10).unwrap();
    }
    let mut reader = RecordReader::new(out.as_slice());
    let err = castep_check_io::electronic::read_electronic(
        &mut reader,
        false,
        0,
        &castep_check_io::cell::Counts::default(),
    )
    .unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::Section(_)));
}

#[test]
fn check_write_requires_a_wave_block() {
    let doc = castep_check_io::document::CheckDocument::default();
    let mut out = Vec::new();
    let mut w = RecordWriter::new(&mut out);
    
    let err = castep_check_io::electronic::write_electronic(
        &mut w,
        &doc.electronic,
        true,
    )
    .unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::Section(_)));
}

#[test]
fn document_file_apis_read_fixtures() {
    let Some(path) = fixture("Cu111_CO.castep_bin") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let doc = castep_check_io::document::CheckDocument::from_castep_bin_file(&path).unwrap();
    assert_eq!(doc.kind, castep_check_io::CheckKind::CastepBin);
    assert!(castep_check_io::document::CheckDocument::from_check_file(&path).is_err());
}

#[test]
fn document_file_apis_mismatch_kind() {
    let Some(path) = fixture("Cu111_CO.check") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let err = castep_check_io::document::CheckDocument::from_castep_bin_file(&path).unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::Section(_)));
}

#[test]
fn document_check_file_success_path() {
    let Some(path) = fixture("Cu111_CO.check") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let doc = castep_check_io::document::CheckDocument::from_check_file(&path).unwrap();
    assert_eq!(doc.kind, castep_check_io::CheckKind::Check);
}

#[test]
fn document_to_check_file_writes_a_readable_file() {
    let Some(path) = fixture("Cu111_CO_F7.check") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let doc = castep_check_io::document::CheckDocument::from_check_file(&path).unwrap();
    let tmp = std::env::temp_dir().join("castep_check_io_tof.check");
    doc.to_check_file(&tmp).unwrap();
    let back =
        castep_check_io::document::CheckDocument::from_check_file(&tmp).unwrap();
    assert_eq!(doc.to_check_bytes().unwrap(), back.to_check_bytes().unwrap());
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn file_version_displays_major_minor() {
    assert_eq!(FileVersion::new(6, 11).to_string(), "6.11");
    assert_eq!(FileVersion::new(3, 0).to_string(), "3.0");
}

#[test]
fn layout_writes_and_rereads_the_devel_code_width() {
    let Some(path) = fixture("Cu111_CO.castep_bin") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let wide_layout = castep_check_io::ParamsLayout { devel_code_len: 256 };
    let doc = castep_check_io::document::CheckDocument::from_castep_bin_file(&path).unwrap();
    assert_eq!(doc.params.raw("devel_code").unwrap().len(), 80);
    let wide = doc.with_layout(wide_layout);
    assert_eq!(wide.params.raw("devel_code").unwrap().len(), 80);
    let wide_bytes = wide.to_castep_bin_bytes().unwrap();
    let back = castep_check_io::document::CheckDocument::from_bytes_with(
        &wide_bytes,
        &wide_layout,
    )
    .unwrap();
    assert_eq!(back.params.raw("devel_code").unwrap().len(), 256);
    assert_eq!(back.to_castep_bin_bytes().unwrap(), wide_bytes);
}

#[test]
fn narrow_write_refuses_to_truncate_devel_code() {
    let Some(path) = fixture("Cu111_CO.castep_bin") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let wide_layout = castep_check_io::ParamsLayout { devel_code_len: 256 };
    let doc = castep_check_io::document::CheckDocument::from_castep_bin_file(&path).unwrap();
    let wide_bytes = doc
        .with_layout(wide_layout)
        .to_castep_bin_bytes()
        .unwrap();
    let back = castep_check_io::document::CheckDocument::from_bytes_with(
        &wide_bytes,
        &wide_layout,
    )
    .unwrap();
    let mut doc = back.with_layout(castep_check_io::ParamsLayout::default());
    let i = castep_check_io::params::RawParams::index_of("devel_code").unwrap();
    let long = format!("PROF:{}", "X".repeat(200));
    let mut payload = long.into_bytes();
    payload.resize(256, b' ');
    doc.params.payloads[i] = Some(payload);
    let err = doc.to_castep_bin_bytes().unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::LayoutOverflow { .. }));
}

#[test]
fn layout_config_parses_toml() {
    let cfg = castep_check_io::LayoutConfig::parse_toml("[params]\ndevel_code_len = 256\n").unwrap();
    assert_eq!(cfg.params.devel_code_len, 256);
    let empty = castep_check_io::LayoutConfig::parse_toml("").unwrap();
    assert_eq!(empty.params.devel_code_len, 80);
    let bad = castep_check_io::LayoutConfig::parse_toml(
        "[params]\ndevel_code_len = \"wide\"\n",
    );
    assert!(matches!(bad, Err(castep_check_io::CheckError::Config(_))));
}

#[test]
fn layout_config_file_errors_name_the_path() {
    let err = castep_check_io::LayoutConfig::from_file("/no/such/layout.toml").unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::ConfigFile { .. }));
}

#[test]
fn document_write_file_apis_work() {
    let Some(path) = fixture("Fe2O3.castep_bin") else {
        eprintln!("SKIP: fixture missing");
        return;
    };
    let doc = castep_check_io::document::CheckDocument::from_castep_bin_file(&path).unwrap();
    let tmp = std::env::temp_dir().join("castep_check_io_roundcast.castep_bin");
    doc.to_castep_bin_file(&tmp).unwrap();
    let back =
        castep_check_io::document::CheckDocument::from_castep_bin_file(&tmp).unwrap();
    let a = doc.to_castep_bin_bytes().unwrap();
    let b = back.to_castep_bin_bytes().unwrap();
    assert_eq!(a, b);
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn check_kind_extension() {
    use castep_check_io::CheckKind;
    assert_eq!(CheckKind::Check.extension(), ".check");
    assert_eq!(CheckKind::CastepBin.extension(), ".castep_bin");
}

#[test]
fn io_error_names_the_path() {
    let err =
        castep_check_io::document::CheckDocument::from_check_file("/no/such/file.check").unwrap_err();
    assert!(matches!(err, castep_check_io::CheckError::Io { .. }));
}
