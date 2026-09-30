use castep_cell_fmt::{
    Cell, CellValue, ToCell, ToCellValue,
    parse::{FromCellValue, FromKeyValue},
    CResult, Error, query::value_as_u32,
};
use crate::cell::species::Species;

/// The perturbing site for a MAGRES J-coupling calculation.
///
/// 6.11 keyword type: string (`S:B`), read with `io_freeform_string`
/// (`Fundamental/cell.f90`). It is written inline, not as a block:
/// `JCOUPLING_SITE : species ion_number`, where `species` is a chemical
/// symbol or an atomic number and `ion_number` is the 1-based index of the
/// ion of that species. CASTEP rejects `ion_number < 1` and one larger
/// than the ion count of that species.
///
/// Example:
/// JCOUPLING_SITE : Fe 2
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct JcouplingSite {
    /// The species of the perturbing site.
    pub species: Species,
    /// The 1-based ion number of the perturbing site within its species.
    /// Must be >= 1; CASTEP also rejects a number larger than the number
    /// of ions of that species in the cell.
    pub ion_number: u32,
}

impl FromKeyValue for JcouplingSite {
    const KEY_NAME: &'static str = "JCOUPLING_SITE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        // `JCOUPLING_SITE : species ion_number` -> [species, ion_number].
        // CASTEP reads the first two tokens and ignores any extras.
        let arr = match value {
            CellValue::Array(arr) if arr.len() >= 2 => arr,
            _ => {
                return Err(Error::Message(
                    "JCOUPLING_SITE must be 'species ion_number'".to_string(),
                ))
            }
        };
        Ok(Self {
            species: Species::from_cell_value(&arr[0])?,
            ion_number: value_as_u32(&arr[1])?,
        })
    }
}

impl ToCell for JcouplingSite {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "JCOUPLING_SITE",
            CellValue::Array(vec![
                self.species.to_cell_value(),
                CellValue::UInt(self.ion_number),
            ]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_from_cell_value_kv() {
        let val = CellValue::Array(vec![CellValue::Str("Fe"), CellValue::UInt(2)]);
        let site = JcouplingSite::from_cell_value_kv(&val).unwrap();
        assert_eq!(site.species, Species::Symbol("Fe".to_string()));
        assert_eq!(site.ion_number, 2);
    }

    #[test]
    fn test_from_cell_value_kv_atomic_number() {
        let val = CellValue::Array(vec![CellValue::UInt(26), CellValue::UInt(1)]);
        let site = JcouplingSite::from_cell_value_kv(&val).unwrap();
        assert_eq!(site.species, Species::AtomicNumber(26));
        assert_eq!(site.ion_number, 1);
    }

    #[test]
    fn test_from_cell_value_kv_malformed() {
        // A single token is not enough (CASTEP reads two).
        let val = CellValue::Array(vec![CellValue::Str("Fe")]);
        assert!(JcouplingSite::from_cell_value_kv(&val).is_err());
        // A scalar (non-array) value is not a valid site.
        let val = CellValue::Str("Fe");
        assert!(JcouplingSite::from_cell_value_kv(&val).is_err());
    }

    #[test]
    fn test_to_cell() {
        let site = JcouplingSite::builder()
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(3)
            .build();
        match site.to_cell() {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "JCOUPLING_SITE");
                assert_eq!(arr.len(), 2);
                assert!(matches!(arr[1], CellValue::UInt(3)));
            }
            other => panic!("Expected KeyValue, got {:?}", other),
        }
    }

    #[test]
    fn test_builder() {
        let site = JcouplingSite::builder()
            .species(Species::AtomicNumber(8))
            .ion_number(1)
            .build();
        assert_eq!(site.species, Species::AtomicNumber(8));
        assert_eq!(site.ion_number, 1);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(JcouplingSite::KEY_NAME, "JCOUPLING_SITE");
    }
}
