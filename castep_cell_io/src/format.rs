use crate::{Cell, CellValue};

/// Format a single `CellValue` into its text representation.
///
/// Floats use `{v:20.16}` fixed-width format. Strings are right-aligned
/// to a width bracket. Arrays are space-joined with a trailing newline.
fn fmt_cell_value(v: &CellValue<'_>, buf: &mut String) {
    match v {
        CellValue::Null => {}
        CellValue::Bool(b) => buf.push_str(&format!("{b:6?}")),
        CellValue::Str(s) => {
            let width = padded_width(s.len());
            buf.push_str(&format!("{s:>width$}"));
        }
        CellValue::String(s) => {
            let width = padded_width(s.len());
            buf.push_str(&format!("{s:>width$}"));
        }
        CellValue::UInt(u) => buf.push_str(&format!("{u:4}")),
        CellValue::Int(i) => buf.push_str(&format!("{i:4}")),
        CellValue::Float(f) => buf.push_str(&format!("{f:20.16}")),
        CellValue::Array(items) => {
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    buf.push(' ');
                }
                fmt_cell_value(item, buf);
            }
        }
    }
}

/// Round a string length up to the next bracket for right-alignment.
fn padded_width(len: usize) -> usize {
    match len {
        x if x < 4 => 4,
        x if (4..6).contains(&x) => 6,
        x if (6..8).contains(&x) => 8,
        x if (8..10).contains(&x) => 10,
        x if (10..16).contains(&x) => 16,
        x if (16..20).contains(&x) => 20,
        x => x + 2,
    }
}

/// Format a single `Cell` IR entry into its `.cell` text representation.
///
/// Fixes the double-`\n` bug: a `%ENDBLOCK` line is emitted with exactly one
/// trailing newline; no extra newline is added by a seq-end callback.
pub fn to_string(cell: &Cell<'_>) -> String {
    let mut buf = String::new();
    fmt_cell(cell, &mut buf);
    buf
}

/// Format a slice of `Cell` IR entries into a `.cell` text representation.
pub fn to_string_many(cells: &[Cell<'_>]) -> String {
    let mut buf = String::new();
    for cell in cells {
        fmt_cell(cell, &mut buf);
    }
    buf
}

/// Format a slice of `Cell` IR entries with blank lines between each entry.
pub fn to_string_many_spaced(cells: &[Cell<'_>]) -> String {
    let mut buf = String::new();
    for cell in cells {
        buf.push_str(&to_string(cell));
        buf.push('\n');
    }
    buf
}

fn fmt_cell(cell: &Cell<'_>, buf: &mut String) {
    match cell {
        Cell::KeyValue(key, value) => {
            buf.push_str(key);
            buf.push_str(" : ");
            fmt_cell_value(value, buf);
            buf.push('\n');
        }
        Cell::Block(name, rows) => {
            buf.push_str("%BLOCK ");
            buf.push_str(&name.to_uppercase());
            buf.push('\n');
            for row in rows {
                fmt_cell_value(row, buf);
                buf.push('\n');
            }
            buf.push_str("%ENDBLOCK ");
            buf.push_str(&name.to_uppercase());
            buf.push('\n');
        }
        Cell::Flag(key) => {
            buf.push_str(key);
            buf.push('\n');
        }
    }
}

#[cfg(test)]
mod format_test {
    use super::{to_string, to_string_many};
    use crate::{Cell, CellValue};

    #[test]
    fn no_double_newline_in_block() {
        let block = Cell::Block(
            "POSITIONS_FRAC",
            vec![CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ])],
        );
        let output = to_string(&block);
        // Must not contain \n\n
        assert!(
            !output.contains("\n\n"),
            "Block output must not contain double newline:\n{output:?}"
        );
        // Must end with exactly one newline
        assert!(output.ends_with('\n'));
    }

    #[test]
    fn keyvalue_format() {
        let kv = Cell::KeyValue("FIX_COM", CellValue::Bool(true));
        let output = to_string(&kv);
        // Bool is formatted with {b:6?} -> "true  " (debug format, width 6)
        assert_eq!(output, "FIX_COM : true  \n");
    }

    #[test]
    fn flag_format() {
        let flag = Cell::Flag("SYMMETRY_GENERATE");
        let output = to_string(&flag);
        assert_eq!(output, "SYMMETRY_GENERATE\n");
    }

    #[test]
    fn block_structure() {
        let block = Cell::Block(
            "LATTICE_CART",
            vec![
                CellValue::Array(vec![
                    CellValue::Float(1.0),
                    CellValue::Float(0.0),
                    CellValue::Float(0.0),
                ]),
                CellValue::Array(vec![
                    CellValue::Float(0.0),
                    CellValue::Float(1.0),
                    CellValue::Float(0.0),
                ]),
                CellValue::Array(vec![
                    CellValue::Float(0.0),
                    CellValue::Float(0.0),
                    CellValue::Float(1.0),
                ]),
            ],
        );
        let output = to_string(&block);
        assert!(output.starts_with("%BLOCK LATTICE_CART\n"));
        assert!(output.ends_with("%ENDBLOCK LATTICE_CART\n"));
        // Exactly one newline at the end (no double-newline)
        assert!(!output.contains("\n\n"));
    }

    #[test]
    fn many_cells() {
        let cells = vec![
            Cell::KeyValue("FIX_COM", CellValue::Bool(false)),
            Cell::Flag("SYMMETRY_GENERATE"),
        ];
        let output = to_string_many(&cells);
        // Bool is formatted with {b:6?} -> "false " (debug format, width 6)
        assert_eq!(output, "FIX_COM : false \nSYMMETRY_GENERATE\n");
    }

    #[test]
    fn round_trip_no_double_newline() {
        use std::fs::read_to_string;
        use crate::parse_cell_file;

        let example = read_to_string("Mg2SiO4_Cr_1.cell").unwrap();
        let tokens = parse_cell_file(&example).unwrap();
        let output = to_string_many(&tokens);
        // The formatted output must not contain any double-newline sequences
        assert!(
            !output.contains("\n\n"),
            "Round-trip output contains double newline"
        );
        // Verify key block headers are present in output
        assert!(output.contains("%BLOCK LATTICE_CART\n"));
        assert!(output.contains("%ENDBLOCK LATTICE_CART\n"));
        assert!(output.contains("%BLOCK POSITIONS_FRAC\n"));
        assert!(output.contains("%ENDBLOCK POSITIONS_FRAC\n"));
    }
}
