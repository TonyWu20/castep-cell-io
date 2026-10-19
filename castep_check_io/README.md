# castep-check-io

Read, edit, and write CASTEP binary model files in Rust.

A model file is the full resolved state of a CASTEP run: the parameters
section, the cell and original cell sections, the wavefunction and
eigenvalue blocks, the charge density grid, and the optional analysis
sections. Two kinds exist:

- `.check`: the full model, with the wavefunction block.
- `.castep_bin`: the analysis model, without the wavefunction block. It
  starts with a 10-byte `CASTEP_BIN` magic record.

## Quick start

```toml
[dependencies]
castep-check-io = "0.1"
```

```rust
use castep_check_io::CheckDocument;

let doc = CheckDocument::from_castep_bin_file("run.castep_bin")?;
println!("total energy: {}", doc.electronic.total_energy);

// Edit one parameter, keep every other record byte-exact.
doc.params.set_text("xc_functional", "BLYP")?;
doc.to_castep_bin_file("run_edited.castep_bin")?;
```

## Crate layout

| Module        | Contents                                                        |
|---------------|-----------------------------------------------------------------|
| `record`      | The 4-byte-marker record framing and big-endian primitives.    |
| `version`     | The file version record, the gate table, the `CheckKind` tag.  |
| `params`      | The 340-field parameters table and the gate-aware codec.        |
| `cell`        | The unit cell and global cell tables with derived sizes.        |
| `electronic`  | Flags, energies, wavefunction, eigenvalues, density, options.   |
| `document`    | The `CheckDocument` type and the file and byte entry points.    |

## Version handling

The file version lives in the second record of the parameters section.
The reader accepts files at or below the build target, with a 3.0 floor.
It rejects newer files with `VersionTooNew` and older files with
`VersionTooOld`. The writer always emits the target release layout and the
target version string, so a roundtrip is byte-exact when the file version
equals the target version.

The release target is a cargo feature, mirroring `castep-cell-io`:

```toml
[dependencies]
castep-check-io = { version = "0.1", features = ["castep-6-11"] }
```

`castep-6-11` is the default. `castep-23` is a placeholder that enables no
binary layout yet.

## Roundtrip guarantee

A `from_bytes` to `to_bytes` roundtrip is byte-exact. The codec stores raw
record payloads and replays them. The guarantee holds on the fixture files
covered by `tests/roundtrip.rs`, including a wavefunction file over 160 MB.

See `docs/adr/0005-check-io-crate.md` for the design decisions and the
fixed layout facts.
