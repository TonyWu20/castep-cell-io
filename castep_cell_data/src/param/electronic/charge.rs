use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the total charge of the system.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// CHARGE : 3.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "CHARGE")]
pub struct Charge(pub f64);

impl Default for Charge {
    fn default() -> Self {
        Self(0.0) // Default is 0
    }
}

impl ToCell for Charge {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CHARGE", CellValue::Float(self.0))
    }
}

impl ToCellValue for Charge {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_charge_serde() {
        let charge_str = "CHARGE : 3.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithCharge {
            charge: Charge,
        }

        let cell_file_result: Result<CellFileWithCharge, _> = from_str(charge_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.charge.0 - 3.0).abs() < f64::EPSILON);

        let charge_instance = Charge(-1.5);
        let serialized_result = to_string(&charge_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized CHARGE (-1.5): {serialized_string}");
        assert!(serialized_string.contains("CHARGE"));
        assert!(serialized_string.contains("-1.5"));

        assert_eq!(Charge::default(), Charge(0.0));
    }
}
