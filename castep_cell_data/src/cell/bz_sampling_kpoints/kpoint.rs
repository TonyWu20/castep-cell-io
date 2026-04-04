use castep_cell_io::{CellValue, ToCellValue, FromCellValue, CResult, query::value_as_f64};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A line of block `KpointsList`
/// The first three entries on a line are the fractional positions of the
/// k-point relative to the reciprocal space lattice vectors.
///
/// The final entry on a line is the weight of the k-point relative to the
/// others specified. The sum of the weights must be equal to 1.
pub struct Kpoint {
    pub coord: [f64; 3],
    pub weight: f64,
}

impl PartialOrd for Kpoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coord
            .iter()
            .zip(other.coord.iter())
            .find_map(|(a, b)| {
                let diff = a - b;
                if diff.abs() > 1e-6 {
                    a.partial_cmp(b)
                } else {
                    None
                }
            })
            .or(Some(std::cmp::Ordering::Equal))
    }
}

impl FromCellValue for Kpoint {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 4 => {
                let coord = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                let weight = value_as_f64(&arr[3])?;
                Ok(Kpoint { coord, weight })
            }
            _ => Err(castep_cell_io::Error::Message(
                "Kpoint must be an array of 4 floats".into(),
            )),
        }
    }
}

impl ToCellValue for Kpoint {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            self.coord
                .into_iter()
                .map(CellValue::Float)
                .chain([CellValue::Float(self.weight)])
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpoint_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        let kpt = Kpoint::from_cell_value(&val).unwrap();
        assert_eq!(kpt.coord, [0.5, 0.25, 0.0]);
        assert_eq!(kpt.weight, 1.0);
    }

    #[test]
    fn test_kpoint_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
        ]);
        assert!(Kpoint::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_kpoint_to_cell_value() {
        let kpt = Kpoint {
            coord: [0.5, 0.25, 0.0],
            weight: 1.0,
        };
        let val = kpt.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 4);
                assert_eq!(arr[3], CellValue::Float(1.0));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_kpoint_partial_ord() {
        let kpt1 = Kpoint {
            coord: [0.0, 0.0, 0.0],
            weight: 1.0,
        };
        let kpt2 = Kpoint {
            coord: [0.5, 0.0, 0.0],
            weight: 1.0,
        };
        assert!(kpt1 < kpt2);
    }
}
