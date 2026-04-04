use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromKeyValue, CResult, query::value_as_f64};

/// Defines the quantization (magnetization) axis for LDA+U calculations.
///
/// Keyword type: Real Vector
///
/// Default: The fractional coordinates equivalent to the c axis.
///
/// Example:
/// QUANTIZATION_AXIS : 1 1 -1
/// This input defines the quantization axis (1 1 -1) which is most appropriate
/// for a fcc structure (along one of the fourth order axes).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct QuantizationAxis {
    /// The fractional coordinates [fa, fb, fc] defining the quantization axis.
    pub direction: [f64; 3],
}

impl FromCellValue for QuantizationAxis {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                Ok(QuantizationAxis {
                    direction: [
                        value_as_f64(&arr[0])?,
                        value_as_f64(&arr[1])?,
                        value_as_f64(&arr[2])?,
                    ],
                })
            }
            _ => Err(castep_cell_io::Error::Message(
                "QuantizationAxis must be an array of 3 floats".into(),
            )),
        }
    }
}

impl FromKeyValue for QuantizationAxis {
    const KEY_NAME: &'static str = "QUANTIZATION_AXIS";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for QuantizationAxis {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "QUANTIZATION_AXIS",
            CellValue::Array(
                self.direction
                    .iter()
                    .map(|&val| CellValue::Float(val))
                    .collect(),
            ),
        )
    }
}

impl ToCellValue for QuantizationAxis {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            self.direction
                .iter()
                .map(|&val| CellValue::Float(val))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_quantization_axis_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(1.0),
            CellValue::Float(1.0),
            CellValue::Float(-1.0),
        ]);
        let result = QuantizationAxis::from_cell_value(&val).unwrap();
        assert_eq!(result.direction, [1.0, 1.0, -1.0]);
    }

    #[test]
    fn test_quantization_axis_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(1.0),
            CellValue::Float(1.0),
        ]);
        assert!(QuantizationAxis::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_quantization_axis_key_name() {
        assert_eq!(QuantizationAxis::KEY_NAME, "QUANTIZATION_AXIS");
    }

    #[test]
    fn test_quantization_axis_to_cell_value() {
        let qa = QuantizationAxis {
            direction: [1.0, 1.0, -1.0],
        };
        let cell_val = qa.to_cell_value();
        match cell_val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
            }
            _ => panic!("Expected array"),
        }
    }
}

