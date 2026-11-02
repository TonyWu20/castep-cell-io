//! The shared NumPy `.npy` (v1.0) writer for real-space charge grids.
//!
//! Writes float64 data in C order. This is the single source of truth
//! for the file format in this workspace: `chemrust-hamiltonian`'s
//! `npy` module delegates here instead of duplicating the layout.
//!
//! Format notes:
//! - NPY v1.0 stores the header length as a 2-byte little-endian u16.
//!   v2.0 uses a 4-byte u32. Writing a 4-byte length with v1.0 version
//!   bytes makes NumPy misparse the header and reject the file.
//! - The 10-byte prefix plus the header block must be 64-byte aligned.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Writes `charge` (row-major, `shape` = (nx, ny, nz)) as a `.npy` file.
pub fn write_density_npy(
    path: &Path,
    shape: [usize; 3],
    charge: &[f64],
) -> std::io::Result<()> {
    if charge.len() != shape[0] * shape[1] * shape[2] {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "charge length does not match shape",
        ));
    }
    let shape_str = format!("({}, {}, {})", shape[0], shape[1], shape[2]);
    let dict = format!(
        "{{'descr': '<f8', 'fortran_order': False, 'shape': {shape_str}, }}"
    );

    // Magic(6) + version(2) + header_len(2) = 10-byte prefix. NumPy
    // requires (prefix + header block) to be 64-byte aligned.
    let with_newline = dict.len() + 1;
    // (54 + 64 - r) never underflows for r < 64; 54 - r alone does.
    let padding = (54usize + 64 - with_newline % 64) % 64;
    let mut block = dict;
    if padding > 0 {
        block.push_str(&" ".repeat(padding));
    }
    block.push('\n');
    let header_len = block.len() as u16;
    debug_assert_eq!((10 + header_len as usize) % 64, 0, "64-byte alignment");

    let mut data = Vec::with_capacity(10 + block.len() + charge.len() * 8);
    data.extend_from_slice(b"\x93NUMPY");
    data.push(1); // major version
    data.push(0); // minor version
    data.extend_from_slice(&header_len.to_le_bytes()); // u16 LE header length
    data.extend_from_slice(block.as_bytes());
    for &v in charge {
        data.extend_from_slice(&v.to_le_bytes());
    }

    let mut f = File::create(path)?;
    f.write_all(&data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_density_npy_layout() {
        let values = [1.0f64, 4.0, 3.0, 5.0];
        // A local file: TMPDIR is unset (or empty) in the Nix devShell,
        // which would make env::temp_dir() return an empty path.
        let path = std::path::Path::new("castep_check_io_npy_test.npy");
        write_density_npy(&path, [2, 1, 2], &values).unwrap();
        let b = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(&b[0..6], b"\x93NUMPY");
        assert_eq!(b[6], 1);
        assert_eq!(b[7], 0);
        let header_len = u16::from_le_bytes([b[8], b[9]]) as usize;
        let header = &b[10..10 + header_len];
        assert!(header.starts_with(b"{'descr': '<f8', 'fortran_order': False, 'shape': (2, 1, 2), }"));
        assert!(header.ends_with(b"\n"));
        assert_eq!((10 + header_len) % 64, 0, "64-byte alignment");
        let data = &b[10 + header_len..];
        assert_eq!(data.len(), 32);
        for (i, v) in values.iter().enumerate() {
            let got = f64::from_le_bytes(
                data[i * 8..i * 8 + 8].try_into().unwrap(),
            );
            assert_eq!(got, *v);
        }
    }

    #[test]
    fn write_density_npy_rejects_bad_length() {
        let path = std::path::Path::new("castep_check_io_npy_test2.npy");
        let e = write_density_npy(&path, [2, 1, 2], &[1.0]);
        assert!(e.is_err());
    }
}
