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
| `main`        | The `castep-check-io` command: inspect or rewrite a model file. |

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

## Local build layouts

The standard CASTEP 6.11 layout is the default. A local build can change
record widths inside one release, for example the `devel_code` record of
the DEVEL block. The width axis is `ParamsLayout`, separate from the
version axis. The reader and the writer each take a layout. A document
stores the layout it was read with, and the writer emits that layout
unless you call `with_layout`. The writer space-pads `devel_code` to the
target width. It refuses to truncate a stored value that is wider, with
`CheckError::LayoutOverflow`.

```rust
use castep_check_io::{CheckDocument, ParamsLayout};

let wide = ParamsLayout { devel_code_len: 256 };
let doc = CheckDocument::from_check_file_with("run_wide.check", &wide)?;
let standard = doc.with_layout(ParamsLayout::default());
```

### Layout config file

The crate reads a TOML layout config. The `params` side is the write
width. The `read` side is the input width. Missing sides take the
standard values.

```toml
[params]
devel_code_len = 256

[read.params]
devel_code_len = 80
```

```rust
use castep_check_io::LayoutConfig;

let config = LayoutConfig::from_file("layout.toml")?;
let write_layout = config.write_layout();
let read_layout = config.read_layout();
```

## Command line

The crate builds a `castep-check-io` command. It inspects a model file
or rewrites it under configured widths, and verifies the write by
re-reading it.

```
castep-check-io [--devel-code-len N] [--read-devel-code-len N]
    [--layout-config PATH] INPUT [OUTPUT]
```

- No `OUTPUT`: print one summary line: the kind, the file version, the
  total energy, the ground-state flags, the optional section headers,
  and the active layout width.
- With `OUTPUT`: read `INPUT` under the read width, write `OUTPUT`
  under the write width, verify the result, then print the summary.
- `--devel-code-len` sets the write width. `--read-devel-code-len` sets
  the input width. A flag wins over the config file value on its side.

## Roundtrip guarantee

A `from_bytes` to `to_bytes` roundtrip is byte-exact. The codec stores raw
record payloads and replays them. The guarantee holds on the fixture files
covered by `tests/roundtrip.rs`, including a wavefunction file over 160 MB.

See `docs/adr/0005-check-io-crate.md` for the design decisions and the
fixed layout facts.
