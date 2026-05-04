use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::query::value_as_f64;
use castep_cell_fmt::{CResult, Cell, CellValue, Error, ToCell, ToCellValue};

/// Specifies the offset of the fine Monkhorst-Pack grid for phonon calculations.
///
/// Keyword type: Key-value with 3 floats
///
/// Format: `phonon_fine_kpoint_mp_offset 0.25 0.25 0.25`
///
/// Example:
/// PHONON_FINE_KPOINT_MP_OFFSET : 0.25 0.25 0.25
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhononFineKpointsMpOffset(pub [f64; 3]);

impl FromCellValue for PhononFineKpointsMpOffset {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let offset = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                Ok(PhononFineKpointsMpOffset(offset))
            }
            _ => Err(Error::Message(
                "PhononFineKpointsMpOffset must be an array of 3 floats".into(),
            )),
        }
    }
}

impl FromKeyValue for PhononFineKpointsMpOffset {
    const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_MP_OFFSET";
    const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_MP_OFFSET"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononFineKpointsMpOffset {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "PHONON_FINE_KPOINT_MP_OFFSET",
            CellValue::Array(self.0.iter().map(|&v| CellValue::Float(v)).collect()),
        )
    }
}

impl ToCellValue for PhononFineKpointsMpOffset {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(self.0.iter().map(|&v| CellValue::Float(v)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_fine_kpoints_mp_offset_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.25),
            CellValue::Float(0.25),
            CellValue::Float(0.25),
        ]);
        let offset = PhononFineKpointsMpOffset::from_cell_value(&val).unwrap();
        assert_eq!(offset.0, [0.25, 0.25, 0.25]);
    }
}
