use castep_cell_serde::{Cell, CellValue, Error, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the level of verbosity of the output.
///
/// Keyword type: Integer
///
/// Default: Iprint::Level1
///
/// Example:
/// IPRINT : 1
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "IPRINT")]
#[serde(try_from = "IprintRepr")]
pub enum Iprint {
    /// Minimal output (0)
    #[serde(rename = "0")]
    Level0,
    /// Standard output (1)
    #[serde(rename = "1")]
    #[default]
    Level1,
    /// Detailed output (2)
    #[serde(rename = "2")]
    Level2,
    /// Full debugging output (3)
    #[serde(rename = "3")]
    Level3,
}

#[derive(Debug, Deserialize)]
struct IprintRepr(u32);

impl TryFrom<IprintRepr> for Iprint {
    type Error = Error;

    fn try_from(value: IprintRepr) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::Level0),
            1 => Ok(Self::Level1),
            2 => Ok(Self::Level2),
            3 => Ok(Self::Level3),
            _ => Err(Error::Message(
                "value of `iprint` exceeds maximum of 3".to_string(),
            )),
        }
    }
}

impl ToCell for Iprint {
    fn to_cell(&self) -> Cell {
        let value = match self {
            Iprint::Level0 => 0,
            Iprint::Level1 => 1,
            Iprint::Level2 => 2,
            Iprint::Level3 => 3,
        };
        Cell::KeyValue("IPRINT", CellValue::Int(value))
    }
}

impl ToCellValue for Iprint {
    fn to_cell_value(&self) -> CellValue {
        let value = match self {
            Iprint::Level0 => 0,
            Iprint::Level1 => 1,
            Iprint::Level2 => 2,
            Iprint::Level3 => 3,
        };
        CellValue::Int(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_iprint_serde() {
        let test_cases = [
            ("IPRINT : 0", Iprint::Level0),
            ("IPRINT : 1", Iprint::Level1),
            ("IPRINT : 2", Iprint::Level2),
            ("IPRINT : 3", Iprint::Level3),
        ];

        for (input_str, expected_level) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithIprint {
                iprint: Iprint,
            }

            let cell_file_result: Result<CellFileWithIprint, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.iprint, expected_level,
                "Failed for input: {input_str}"
            );
        }

        let iprint_instance = Iprint::Level2;
        let serialized_result = to_string(&iprint_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized IPRINT (Level2):\n{serialized_string}");
        assert!(serialized_string.contains("IPRINT"));
        assert!(serialized_string.contains("2"));

        assert_eq!(Iprint::default(), Iprint::Level1);
    }
}
