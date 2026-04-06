use crate::cell::species::{
    Species,
    hubbard_u::{AtomHubbardU, OrbitalU},
};
use castep_cell_fmt::{CellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::value_as_f64, query::value_as_u32};

impl FromBlock for AtomHubbardU {
    const BLOCK_NAME: &'static str = "ATOM_HUBBARD_U";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("AtomHubbardU row is empty".into()));
        }

        if let CellValue::Array(arr) = &rows[0] {
            if arr.is_empty() {
                return Err(Error::Message("AtomHubbardU array is empty".into()));
            }

            let species = Species::from_cell_value(&arr[0])?;
            let mut idx = 1;
            let mut ion_number = None;

            // Check if second element is a u32 (ion number)
            if idx < arr.len()
                && let Ok(ion_num) = value_as_u32(&arr[idx]) {
                    ion_number = Some(ion_num);
                    idx += 1;
                }

            // Parse remaining elements as orbital specifications
            let mut orbitals = Vec::new();
            while idx < arr.len() {
                if let CellValue::String(orb_str) = &arr[idx] {
                    if idx + 1 < arr.len() {
                        let val = value_as_f64(&arr[idx + 1])?;
                        if let Some(orbital) = OrbitalU::from_str_f64(orb_str, val) {
                            orbitals.push(orbital);
                        }
                        idx += 2;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            Ok(Self {
                species,
                ion_number,
                orbitals,
            })
        } else {
            Err(Error::Message("AtomHubbardU row must be an array".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_atom_hubbard_u_empty() {
        let result = AtomHubbardU::from_block_rows(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_atom_hubbard_u_species_only() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("Fe")]),
        ];
        let result = AtomHubbardU::from_block_rows(&rows).unwrap();
        assert!(result.species.as_symbol().is_some());
        assert!(result.ion_number.is_none());
        assert_eq!(result.orbitals.len(), 0);
    }

    #[test]
    fn test_atom_hubbard_u_with_ion_number() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::UInt(1),
            ]),
        ];
        let result = AtomHubbardU::from_block_rows(&rows).unwrap();
        assert_eq!(result.ion_number, Some(1));
    }

    #[test]
    fn test_block_name() {
        assert_eq!(AtomHubbardU::BLOCK_NAME, "ATOM_HUBBARD_U");
    }
}

