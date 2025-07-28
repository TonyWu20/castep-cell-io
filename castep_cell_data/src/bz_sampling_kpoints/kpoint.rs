use castep_cell_serde::{CellValue, ToCellValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
/// A line of block `KpointsList`
/// The first three entries on a line are the fractional positions of the
/// k-point relative to the reciprocal space lattice vectors.
///
/// The final entry on a line is the weight of the k-point relative to the
/// others specified. The sum of the weights must be equal to 1.
#[serde(from = "[f64;4]")]
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

impl From<[f64; 4]> for Kpoint {
    fn from(value: [f64; 4]) -> Self {
        Kpoint {
            coord: value[0..3].try_into().unwrap(),
            weight: value[3],
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
