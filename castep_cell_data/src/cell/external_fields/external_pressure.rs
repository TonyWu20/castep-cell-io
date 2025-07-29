use crate::units::PressureUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Represents the external pressure tensor applied to the system.
///
/// Keyword type: Block
///
/// Default unit for the tensor components: GPa (if units are not specified).
/// Default value: No external pressure (zero tensor).
///
/// Example:
/// %BLOCK EXTERNAL_PRESSURE
/// GPa
///     5.0000000000    0.0000000000    0.0000000000
///                     5.0000000000    0.0000000000
///                                     5.0000000000
/// %ENDBLOCK EXTERNAL_PRESSURE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "ExternalPressureRepr")] // Use intermediate repr for deserialization
#[serde(rename = "EXTERNAL_PRESSURE")] // Ensure correct block name during serde
pub struct ExternalPressure {
    /// Optional unit specification for the pressure tensor.
    /// If None, the default unit (GPa) is implied.
    pub unit: Option<PressureUnit>,
    /// The upper triangular part of the pressure tensor [Rxx, Rxy, Rxz, Ryy, Ryz, Rzz].
    /// Stored in row-major order for the upper triangle.
    pub tensor: [f64; 6],
}

/// Intermediate representation for deserializing the `EXTERNAL_PRESSURE` block.
/// Handles the optional unit line and the structured tensor data.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExternalPressureRepr {
    /// Format: [units] \n [Rxx, Rxy, Rxz] \n [Ryy, Ryz] \n [Rzz]
    /// Assuming the parser provides the unit and then the 3 rows as arrays.
    WithUnit([PressureUnit; 1], [f64; 3], [f64; 2], [f64; 1]),
    /// Format: [Rxx, Rxy, Rxz] \n [Ryy, Ryz] \n [Rzz] (default unit implied)
    /// Assuming the parser provides the 3 rows as arrays directly.
    Essential([f64; 3], [f64; 2], [f64; 1]),
    // Note: The actual structure provided by castep_cell_serde might differ,
    // e.g., it could be Vec<Vec<f64>> or Vec<CellValue::Float>.
    // The key is handling the optional first element (unit) and the triangular structure.
}

impl From<ExternalPressureRepr> for ExternalPressure {
    /// Converts the intermediate representation into the final struct.
    fn from(repr: ExternalPressureRepr) -> Self {
        match repr {
            ExternalPressureRepr::WithUnit(unit, row1, row2, row3) => {
                // Convert the triangular structure [Rxx,Rxy,Rxz], [Ryy,Ryz], [Rzz]
                // into a flat array [Rxx, Rxy, Rxz, Ryy, Ryz, Rzz]
                let tensor = [row1[0], row1[1], row1[2], row2[0], row2[1], row3[0]];
                Self {
                    unit: Some(unit[0]),
                    tensor,
                }
            }
            ExternalPressureRepr::Essential(row1, row2, row3) => {
                // Convert the triangular structure with default unit
                let tensor = [row1[0], row1[1], row1[2], row2[0], row2[1], row3[0]];
                Self {
                    unit: None, // Default unit (GPa) implied
                    tensor,
                }
            }
        }
    }
}

impl ToCell for ExternalPressure {
    /// Converts the struct into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::with_capacity(4); // Max 4 lines (1 unit + 3 tensor rows)

        // 1. Add the optional unit line if present
        if let Some(unit) = &self.unit {
            block_content.push(CellValue::Array(vec![unit.to_cell_value()]));
        }

        // 2. Add the tensor rows in upper triangular format
        // Row 1: Rxx, Rxy, Rxz
        block_content.push(CellValue::Array(
            self.tensor[0..3] // Rxx, Rxy, Rxz
                .iter()
                .map(|&val| CellValue::Float(val))
                .collect(),
        ));
        // Row 2: (implicit 0.0), Ryy, Ryz
        block_content.push(CellValue::Array(
            // The implicit 0.0 for Rxy is typically not stored, but the output format shows spacing.
            // We only serialize the actual values: Ryy, Ryz
            [CellValue::String(
                (0..18).map(|_| " ").collect::<Vec<&str>>().concat(),
            )]
            .to_vec()
            .into_iter()
            .chain(
                self.tensor[3..5] // Ryy, Ryz
                    .iter()
                    .map(|&val| CellValue::Float(val)),
            )
            .collect(),
        ));
        // Row 3: (implicit 0.0), (implicit 0.0), Rzz
        block_content.push(CellValue::Array(
            // Create two empty &str to push it to upper triangle form
            (0..2)
                .map(|_| CellValue::String((0..18).map(|_| " ").collect::<Vec<&str>>().concat()))
                .chain(
                    // We only serialize the actual value: Rzz
                    vec![CellValue::Float(self.tensor[5])], // Rzz
                )
                .collect(),
        ));

        Cell::Block("EXTERNAL_PRESSURE", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_external_pressure_serde() {
        // 1. Test Deserialization with unit
        // Note: The exact whitespace/formatting in the example might affect parsing.
        // This test assumes the parser can handle the triangular format.
        let external_pressure_with_unit_str = r#"%BLOCK EXTERNAL_PRESSURE
gpa
    5.0000000000    0.0000000000    0.0000000000
                    5.0000000000    0.0000000000
                                    5.0000000000
%ENDBLOCK EXTERNAL_PRESSURE
"#;
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPressureUnit {
            external_pressure: ExternalPressure,
        }

        let cell_file_result: Result<CellFileWithPressureUnit, _> =
            from_str(external_pressure_with_unit_str);
        // Assert successful deserialization (assuming the parser handles the triangular format)
        // If this fails, the ExternalPressureRepr or the parser's tokenization needs adjustment.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(
            cell_file.external_pressure.unit,
            Some(PressureUnit::GigaPascal)
        );
        assert_eq!(
            cell_file.external_pressure.tensor,
            [5.0, 0.0, 0.0, 5.0, 0.0, 5.0]
        );

        // 2. Test Deserialization without unit (default unit implied)
        let external_pressure_default_str = r#"%BLOCK EXTERNAL_PRESSURE
    1.0 0.1 0.2
        2.0 0.3
            3.0
%ENDBLOCK EXTERNAL_PRESSURE
"#; // Using a simpler format for testing
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPressureDefault {
            external_pressure: ExternalPressure,
        }

        let cell_file_default_result: Result<CellFileWithPressureDefault, _> =
            from_str(external_pressure_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert_eq!(cell_file_default.external_pressure.unit, None);
        assert_eq!(
            cell_file_default.external_pressure.tensor,
            [1.0, 0.1, 0.2, 2.0, 0.3, 3.0]
        );

        // 3. Test Serialization using ToCell
        let external_pressure_instance = ExternalPressure {
            unit: Some(PressureUnit::Atmosphere), // Using the PressureUnit enum
            tensor: [1.0, 0.1, 0.2, 2.0, 0.3, 3.0],
        };
        let serialized_result = to_string(&external_pressure_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized EXTERNAL_PRESSURE (with unit):\n{serialized_string}"); // Clippy suggestion

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK EXTERNAL_PRESSURE"));
        assert!(serialized_string.contains("%ENDBLOCK EXTERNAL_PRESSURE"));
        assert!(serialized_string.contains("atm")); // Check unit serialization
        assert!(serialized_string.contains("1.0"));
        assert!(serialized_string.contains("0.1"));
        assert!(serialized_string.contains("3.0"));
        // Note: Exact formatting (whitespace, alignment) might differ from the example.
    }
}
