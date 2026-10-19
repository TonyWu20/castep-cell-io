//! The `castep-check-io` command.
//!
//! Inspects a CASTEP model file or rewrites it under a configured
//! layout. Layout widths follow the target local build, for example a
//! widened `devel_code` record.

use std::path::PathBuf;

use castep_check_io::{CheckDocument, CheckError, CResult, LayoutConfig, ParamsLayout};

/// Parsed command line.
#[derive(Debug)]
struct Cli {
    /// The input model file.
    input: PathBuf,
    /// An optional output path. Absent: inspect only.
    output: Option<PathBuf>,
    /// The layout for the read and the write.
    layout: ParamsLayout,
}

/// Parses the command line.
///
/// Usage: `castep-check-io [--devel-code-len N | --layout-config PATH]
/// INPUT [OUTPUT]`
///
/// The width names the build in use. It applies to both the read and the
/// write. A flag wins over a config file value. Cross-width conversion is
/// a library flow: read with one layout, `with_layout`, write.
fn parse_args(args: &[String]) -> CResult<Cli> {
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut layout = ParamsLayout::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--devel-code-len" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| CheckError::Section("--devel-code-len needs a value".into()))?;
                let n = value
                    .parse::<u32>()
                    .map_err(|_| CheckError::Section(format!("bad width {value}")))?;
                layout.devel_code_len = n;
            }
            "--layout-config" => {
                i += 1;
                let path = args
                    .get(i)
                    .ok_or_else(|| CheckError::Section("--layout-config needs a path".into()))?;
                let config = LayoutConfig::from_file(path)?;
                layout = config.layout();
            }
            other => {
                if other.starts_with('-') {
                    return Err(CheckError::Section(format!(
                        "unknown flag {other}"
                    )));
                }
                if input.is_none() {
                    input = Some(PathBuf::from(other));
                } else if output.is_none() {
                    output = Some(PathBuf::from(other));
                } else {
                    return Err(CheckError::Section(format!(
                        "too many positional arguments: {other}"
                    )));
                }
            }
        }
        i += 1;
    }
    let input = input
        .ok_or_else(|| CheckError::Section("missing INPUT argument".into()))?;
    Ok(Cli { input, output, layout })
}

/// One summary line for an inspected document.
fn summary(doc: &CheckDocument) -> String {
    let headers: Vec<&str> = doc.electronic.optional.iter().map(|s| s.header.as_str()).collect();
    format!(
        "{} v{} energy={:.4} found_gs_wvfn={} found_gs_den={} optional={:?} layout.devel_code_len={}",
        doc.kind.extension(),
        doc.params.version,
        doc.electronic.total_energy,
        doc.electronic.found_gs_wvfn,
        doc.electronic.found_gs_den,
        headers,
        doc.layout.devel_code_len
    )
}

/// Runs one command.
fn run(cli: &Cli) -> CResult<()> {
    let doc = if cli.input.extension().is_some_and(|e| e == "castep_bin") {
        CheckDocument::from_castep_bin_file_with(&cli.input, &cli.layout)?
    } else {
        CheckDocument::from_check_file_with(&cli.input, &cli.layout)?
    };
    let doc = if let Some(out) = &cli.output {
        let doc = doc.with_layout(cli.layout);
        match doc.kind {
            castep_check_io::CheckKind::Check => doc.to_check_file(out)?,
            castep_check_io::CheckKind::CastepBin => doc.to_castep_bin_file(out)?,
        }
        // Verify: re-read the written file under the same layout.
        let check = if out.extension().is_some_and(|e| e == "castep_bin") {
            CheckDocument::from_castep_bin_file_with(out, &cli.layout)?
        } else {
            CheckDocument::from_check_file_with(out, &cli.layout)?
        };
        let a = match doc.kind {
            castep_check_io::CheckKind::Check => doc.to_check_bytes()?,
            castep_check_io::CheckKind::CastepBin => doc.to_castep_bin_bytes()?,
        };
        let b = match check.kind {
            castep_check_io::CheckKind::Check => check.to_check_bytes()?,
            castep_check_io::CheckKind::CastepBin => check.to_castep_bin_bytes()?,
        };
        if a != b {
            return Err(CheckError::Section(format!(
                "write verify failed: {out:?} is not stable under the {} layout",
                cli.layout.devel_code_len
            )));
        }
        doc
    } else {
        doc
    };
    println!("{}", summary(&doc));
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = parse_args(&args).and_then(|cli| run(&cli));
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_requires_input() {
        let err = parse_args(&[]).unwrap_err();
        assert!(matches!(err, CheckError::Section(_)));
    }

    #[test]
    fn parse_reads_input_and_output() {
        let args = vec!["a.check".into(), "b.check".into()];
        let cli = parse_args(&args).unwrap();
        assert_eq!(cli.input.to_str().unwrap(), "a.check");
        assert_eq!(cli.output.as_ref().unwrap().to_str().unwrap(), "b.check");
        assert_eq!(cli.layout.devel_code_len, 80);
    }

    #[test]
    fn flag_sets_the_width() {
        let args = vec!["a.check".into(), "--devel-code-len".into(), "256".into()];
        let cli = parse_args(&args).unwrap();
        assert_eq!(cli.layout.devel_code_len, 256);
    }

    #[test]
    fn flag_beats_config_file_value() {
        let dir = std::env::temp_dir().join("castep_check_io_cli_test.toml");
        std::fs::write(&dir, "[params]\ndevel_code_len = 256\n").unwrap();
        let args = vec![
            "a.check".into(),
            "--layout-config".into(),
            dir.to_str().unwrap().to_string(),
            "--devel-code-len".into(),
            "128".into(),
        ];
        let cli = parse_args(&args).unwrap();
        assert_eq!(cli.layout.devel_code_len, 128);
        let _ = std::fs::remove_file(&dir);
    }

    #[test]
    fn config_file_sets_the_width() {
        let dir = std::env::temp_dir().join("castep_check_io_cli_test2.toml");
        std::fs::write(&dir, "[params]\ndevel_code_len = 256\n").unwrap();
        let args = vec!["a.check".into(), "--layout-config".into(), dir.to_str().unwrap().to_string()];
        let cli = parse_args(&args).unwrap();
        assert_eq!(cli.layout.devel_code_len, 256);
        let _ = std::fs::remove_file(&dir);
    }

    #[test]
    fn bad_width_is_an_error() {
        let args = vec!["a.check".into(), "--devel-code-len".into(), "abc".into()];
        assert!(parse_args(&args).is_err());
    }

    #[test]
    fn unknown_flag_is_an_error() {
        let args = vec!["a.check".into(), "--nope".into()];
        assert!(parse_args(&args).is_err());
    }
    fn fixture(name: &str) -> Option<std::path::PathBuf> {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("chemrust-hamiltonian/chemrust-hamiltonian-core/tests/fixtures")
            .join(name);
        p.exists().then_some(p)
    }

    fn cli(input: PathBuf, output: Option<PathBuf>, layout: castep_check_io::ParamsLayout) -> Cli {
        Cli { input, output, layout }
    }

    #[test]
    fn run_inspects_a_castep_bin_fixture() {
        let Some(path) = fixture("Cu111_CO.castep_bin") else {
            eprintln!("SKIP: fixture missing");
            return;
        };
        let c = cli(path, None, castep_check_io::ParamsLayout::default());
        assert!(run(&c).is_ok());
    }

    fn wide_input_file(suffix: &str) -> Option<std::path::PathBuf> {
        let Some(path) = fixture("Cu111_CO.castep_bin") else {
            return None;
        };
        let doc = castep_check_io::CheckDocument::from_castep_bin_file(&path).ok()?;
        let wide_in = std::env::temp_dir()
            .join(format!("castep_check_io_main_widein_{suffix}.castep_bin"));
        doc.with_layout(castep_check_io::ParamsLayout { devel_code_len: 256 })
            .to_castep_bin_file(&wide_in)
            .ok()?;
        Some(wide_in)
    }

    #[test]
    fn run_rewrites_a_wide_castep_bin() {
        let Some(wide_in) = wide_input_file("b") else {
            eprintln!("SKIP: fixture missing");
            return;
        };
        let out = std::env::temp_dir().join("castep_check_io_main_wide.castep_bin");
        let wide = castep_check_io::ParamsLayout { devel_code_len: 256 };
        let c = cli(wide_in, Some(out.clone()), wide);
        assert!(run(&c).is_ok());
        assert!(out.exists());
        let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn run_reads_a_wide_file_wide() {
        let Some(wide_in) = wide_input_file("a") else {
            eprintln!("SKIP: fixture missing");
            return;
        };
        let wide = castep_check_io::ParamsLayout { devel_code_len: 256 };
        let c = cli(wide_in.clone(), None, wide);
        assert!(run(&c).is_ok());
    }

    #[test]
    fn run_refuses_to_read_a_standard_file_wide() {
        let Some(path) = fixture("Cu111_CO.castep_bin") else {
            eprintln!("SKIP: fixture missing");
            return;
        };
        let wide = castep_check_io::ParamsLayout { devel_code_len: 256 };
        let c = cli(path, None, wide);
        assert!(run(&c).is_err());
    }

    #[test]
    fn run_missing_input_is_an_io_error() {
        let c = cli(PathBuf::from("/no/such/castep_bin"), None, castep_check_io::ParamsLayout::default());
        let err = run(&c).unwrap_err();
        assert!(matches!(err, CheckError::Io { .. }));
    }

    #[test]
    fn run_missing_check_input_is_an_io_error() {
        let c = cli(PathBuf::from("/no/such/Cu111_CO.check"), None, castep_check_io::ParamsLayout::default());
        let err = run(&c).unwrap_err();
        assert!(matches!(err, CheckError::Io { .. }));
    
    }
}