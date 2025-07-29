use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Import the new dedicated unit enum
use crate::units::EFieldUnit;

/// Represents the electric field vector in Cartesian coordinates.
///
/// Keyword type: Block
///
/// Default unit for the field vector: eV/Ang/e (if units are not specified).
///
/// Example:
/// %BLOCK EXTERNAL_EFIELD
/// HARTREE/BOHR/E
/// 0.0 0.0 0.1
/// %ENDBLOCK EXTERNAL_EFIELD
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "ExternalEfieldRepr")] // Use intermediate repr for deserialization
#[serde(rename = "EXTERNAL_EFIELD")] // Ensure correct block name during serde
pub struct ExternalEfield {
    /// Optional unit specification for the electric field.
    /// If None, the default unit (eV/Ang/e) is implied.
    pub unit: Option<EFieldUnit>,
    /// The electric field vector components [Ex, Ey, Ez] in Cartesian coordinates.
    pub field_vector: [f64; 3],
}

/// Intermediate representation for deserializing the `EXTERNAL_EFIELD` block.
/// Handles the optional unit line.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExternalEfieldRepr {
    /// Format: [units] \n Ex Ey Ez
    WithUnit([EFieldUnit; 1], [f64; 3]), // Parser provides unit then vector
    /// Format: Ex Ey Ez (default unit implied)
    Essential([[f64; 3]; 1]),
    // Note: If the parser provides content differently (e.g., Vec<CellValue>),
    // this repr might need adjustment. The goal is to match the parser's output structure.
}

impl From<ExternalEfieldRepr> for ExternalEfield {
    /// Converts the intermediate representation into the final struct.
    fn from(repr: ExternalEfieldRepr) -> Self {
        match repr {
            ExternalEfieldRepr::WithUnit(unit, field_vector) => Self {
                unit: Some(unit[0]),
                field_vector,
            },
            ExternalEfieldRepr::Essential(field_vector) => Self {
                unit: None, // Default unit implied
                field_vector: field_vector[0],
            },
        }
    }
}

impl ToCell for ExternalEfield {
    /// Converts the struct into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        let block_content = [
            match &self.unit {
                Some(u) => CellValue::Array(vec![u.to_cell_value()]),
                None => CellValue::Null,
            },
            CellValue::Array(
                self.field_vector
                    .into_iter()
                    .map(CellValue::Float)
                    .collect(),
            ),
        ]
        .to_vec();

        Cell::Block("EXTERNAL_EFIELD", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_external_efield_serde() {
        // 1. Test Deserialization with unit
        let external_efield_with_unit_str = r#"%BLOCK EXTERNAL_EFIELD
hartree/bohr/e
0.0 0.0 0.1
%ENDBLOCK EXTERNAL_EFIELD
"#;
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldUnit {
            external_efield: ExternalEfield,
        }

        let cell_file_result: Result<CellFileWithEfieldUnit, _> =
            from_str(external_efield_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        assert_eq!(
            cell_file.external_efield.unit,
            Some(EFieldUnit::HartreePerBohrPerE)
        );
        assert_eq!(cell_file.external_efield.field_vector, [0.0, 0.0, 0.1]);

        // 2. Test Deserialization without unit (default unit implied)
        let external_efield_default_str = r#"%BLOCK EXTERNAL_EFIELD
0.0 0.0 0.1
%ENDBLOCK EXTERNAL_EFIELD
"#;
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldDefault {
            external_efield: ExternalEfield,
        }

        let cell_file_default_result: Result<CellFileWithEfieldDefault, _> =
            from_str(external_efield_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert_eq!(cell_file_default.external_efield.unit, None);
        assert_eq!(
            cell_file_default.external_efield.field_vector,
            [0.0, 0.0, 0.1]
        );

        // 3. Test Serialization using ToCell (with unit)
        let external_efield_instance = ExternalEfield {
            unit: Some(EFieldUnit::EvPerAngPerE), // Using the new dedicated enum
            field_vector: [0.5, -0.2, 0.1],
        };
        let serialized_result = to_string(&external_efield_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized EXTERNAL_EFIELD (with unit):\n{serialized_string}"); // Clippy suggestion

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK EXTERNAL_EFIELD"));
        assert!(serialized_string.contains("%ENDBLOCK EXTERNAL_EFIELD"));
        assert!(serialized_string.contains("ev/ang/e")); // Check unit serialization
        assert!(serialized_string.contains("0.5"));
        assert!(serialized_string.contains("-0.2"));
        assert!(serialized_string.contains("0.1"));

        let external_efield_instance = ExternalEfield {
            unit: None,
            field_vector: [0.5, -0.2, 0.1],
        };
        let serialized_result = to_string(&external_efield_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized EXTERNAL_EFIELD (without unit):\n{serialized_string}"); // Clippy suggestion
    }
}
