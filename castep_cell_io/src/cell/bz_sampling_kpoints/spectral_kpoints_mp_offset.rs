use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::query::value_as_f64;
use castep_cell_fmt::{CResult, Cell, CellValue, Error, ToCell, ToCellValue};

/// Specifies the offset of the Monkhorst-Pack grid for spectral k-point sampling.
///
/// Keyword type: Key-value with 3 floats
///
/// Format: `SPECTRAL_KPOINTS_MP_OFFSET : x y z`
///
/// Example:
/// SPECTRAL_KPOINTS_MP_OFFSET : 0.0 0.5 0.5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralKpointsMpOffset(pub [f64; 3]);

impl FromCellValue for SpectralKpointsMpOffset {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let offset = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                Ok(SpectralKpointsMpOffset(offset))
            }
            _ => Err(Error::Message(
                "SpectralKpointsMpOffset must be an array of 3 floats".into(),
            )),
        }
    }
}

impl FromKeyValue for SpectralKpointsMpOffset {
    const KEY_NAME: &'static str = "SPECTRAL_KPOINT_MP_OFFSET";
    const KEY_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_MP_OFFSET"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SpectralKpointsMpOffset {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "SPECTRAL_KPOINT_MP_OFFSET",
            CellValue::Array(self.0.iter().map(|&v| CellValue::Float(v)).collect()),
        )
    }
}

impl ToCellValue for SpectralKpointsMpOffset {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(self.0.iter().map(|&v| CellValue::Float(v)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_kpoints_mp_offset_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.0),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
        ]);
        let offset = SpectralKpointsMpOffset::from_cell_value(&val).unwrap();
        assert_eq!(offset.0, [0.0, 0.5, 0.5]);
    }

    #[test]
    fn test_spectral_kpoints_mp_offset_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.5)]);
        assert!(SpectralKpointsMpOffset::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_spectral_kpoints_mp_offset_too_many_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.0),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(0.0),
        ]);
        assert!(SpectralKpointsMpOffset::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_spectral_kpoints_mp_offset_non_array() {
        let val = CellValue::Float(0.25);
        assert!(SpectralKpointsMpOffset::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_spectral_kpoints_mp_offset_to_cell_value() {
        let offset = SpectralKpointsMpOffset([0.0, 0.5, 0.5]);
        let val = offset.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::Float(0.0));
                assert_eq!(arr[1], CellValue::Float(0.5));
                assert_eq!(arr[2], CellValue::Float(0.5));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_spectral_kpoints_mp_offset_to_cell() {
        let offset = SpectralKpointsMpOffset([0.0, 0.5, 0.5]);
        let cell = offset.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "SPECTRAL_KPOINT_MP_OFFSET");
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::Float(0.0));
            }
            _ => panic!("Expected Cell::KeyValue with Array"),
        }
    }

    #[test]
    fn test_spectral_kpoints_mp_offset_round_trip() {
        let original = SpectralKpointsMpOffset([0.125, 0.25, 0.375]);
        let cell_value = CellValue::Array(vec![
            CellValue::Float(0.125),
            CellValue::Float(0.25),
            CellValue::Float(0.375),
        ]);
        let parsed = SpectralKpointsMpOffset::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed, original);
    }
}
