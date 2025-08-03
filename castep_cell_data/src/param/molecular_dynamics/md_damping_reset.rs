use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the number of MD steps between recalculations of damping parameters.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// MD_DAMPING_RESET : 20
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_DAMPING_RESET")]
pub struct MdDampingReset(pub u32); // Using u32 as it's a count of steps

impl Default for MdDampingReset {
    fn default() -> Self {
        Self(30) // Default is 30
    }
}

impl ToCell for MdDampingReset {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_DAMPING_RESET", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdDampingReset {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_md_damping_reset_serde() {
        let md_damping_reset_str = "MD_DAMPING_RESET : 20";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdDampingReset {
            md_damping_reset: MdDampingReset,
        }

        let cell_file_result: Result<CellFileWithMdDampingReset, _> =
            from_str(md_damping_reset_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.md_damping_reset.0, 20);

        let md_damping_reset_instance = MdDampingReset(50);
        let serialized_result = to_string(&md_damping_reset_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_DAMPING_RESET (50): {serialized_string}");
        assert!(serialized_string.contains("MD_DAMPING_RESET"));
        assert!(serialized_string.contains("50"));

        assert_eq!(MdDampingReset::default(), MdDampingReset(30));
    }
}
