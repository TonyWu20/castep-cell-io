use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
// Assuming Species and MassUnit are defined in common modules
use super::Species;
use crate::units::MassUnit; // Assuming MassUnit is defined as previously discussed
use serde::{Deserialize, Serialize};

/// Represents a single entry within the SPECIES_MASS block,
/// linking a species to its mass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeciesMassEntry {
    /// The species (symbol or atomic number).
    pub species: Species, // Reusing the shared Species enum
    /// The mass of the species.
    pub mass: f64,
}

// Implement ToCellValue for SpeciesMassEntry to enable serialization of individual lines.
impl ToCellValue for SpeciesMassEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            self.species.to_cell_value(), // Converts Species to CellValue::String or CellValue::UInt
            CellValue::Float(self.mass),
        ])
    }
}

/// Intermediate representation for deserializing the SPECIES_MASS block content.
/// handles optional unit followed by entries.
/// Since the number of entries is not determined,
/// we have to trick `serde` with an enum `SpeciesMassLineRepr` to hold the lines
/// as a uniform type.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct SpeciesMassRepr {
    lines: Vec<SpeciesMassLineRepr>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
/// Possible line entries in the `SPECIES_MASS` block,
/// but we only care if the first line is the `unit`
enum SpeciesMassLineRepr {
    Unit([MassUnit; 1]),
    MassEntry(SpeciesMassEntry),
}

impl SpeciesMassLineRepr {
    fn as_mass_entry(&self) -> Option<&SpeciesMassEntry> {
        if let Self::MassEntry(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_unit(&self) -> Option<&MassUnit> {
        if let Self::Unit(v) = self {
            Some(&v[0])
        } else {
            None
        }
    }

    fn try_into_mass_entry(self) -> Result<SpeciesMassEntry, Self> {
        if let Self::MassEntry(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

/// Represents the SPECIES_MASS block.
///
/// Defines the mass of each atomic species.
/// Format:
/// %BLOCK SPECIES_MASS
/// [units]
/// CCC1/I1 R1
/// CCC2/I2 R2
/// ...
/// %ENDBLOCK SPECIES_MASS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "SPECIES_MASS")] // Ensure correct block name during serde
#[serde(from = "SpeciesMassRepr")] // Use intermediate repr for deserialization
pub struct SpeciesMass {
    /// The unit of mass. If None, the default (amu) is used.
    pub unit: Option<MassUnit>,
    /// The list of species and their corresponding masses.
    pub masses: Vec<SpeciesMassEntry>,
}

impl From<SpeciesMassRepr> for SpeciesMass {
    fn from(repr: SpeciesMassRepr) -> Self {
        // Check the first line entry
        repr.lines
            .first()
            // If it is `Some`
            .map(|first_line| match first_line {
                // The first line is unit
                SpeciesMassLineRepr::Unit(mass_unit) => {
                    let unit = mass_unit[0];
                    let masses = repr
                        .lines
                        .iter()
                        .skip(1) // skip the unit line
                        .map(|entry| entry.as_mass_entry().cloned().unwrap())
                        .collect::<Vec<SpeciesMassEntry>>();
                    Self {
                        unit: Some(unit),
                        masses,
                    }
                }
                // The first line is already species_mass,
                SpeciesMassLineRepr::MassEntry(_species_mass_entry) => Self {
                    unit: None,
                    masses: repr
                        .lines
                        .iter()
                        .map(|entry| entry.as_mass_entry().cloned().unwrap())
                        .collect(),
                },
            })
            // Edge case: block wrappers are present, but no contents
            .unwrap_or(Self {
                unit: None,
                masses: Vec::new(),
            })
    }
}

impl ToCell for SpeciesMass {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();

        // Add the optional unit line first, using the same pattern as LATTICE_CART
        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        // Add the species/mass entries
        block_content.extend(
            self.masses.iter().map(|entry| entry.to_cell_value()), // Convert each entry to CellValue::Array
        );

        Cell::Block("SPECIES_MASS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_species_mass_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileSpeciesMass {
            species_mass: SpeciesMass,
        }

        // Test string based on the documentation example (without units line)
        let species_mass_str_no_unit = r#"%BLOCK SPECIES_MASS
       O     15.9989995956
      Al     26.9820003510
      Ti     47.9000015259
      Cs    132.9049987793
%ENDBLOCK SPECIES_MASS
"#;

        // 1. Test Deserialization (without unit line)
        let cell_file_result_no_unit: Result<CellFileSpeciesMass, _> =
            from_str(species_mass_str_no_unit);
        assert!(
            cell_file_result_no_unit.is_ok(),
            "Deserialization (no unit) failed: {:?}",
            cell_file_result_no_unit.err()
        );
        let cell_file_no_unit = cell_file_result_no_unit.unwrap();
        // Debug print the deserialized struct
        println!(
            "Deserialized SPECIES_MASS (no unit): {:#?}",
            cell_file_no_unit.species_mass
        );
        assert_eq!(cell_file_no_unit.species_mass.unit, None);
        assert_eq!(cell_file_no_unit.species_mass.masses.len(), 4);
        assert_eq!(
            cell_file_no_unit.species_mass.masses[0].species,
            Species::Symbol("O".to_string())
        );
        assert!((cell_file_no_unit.species_mass.masses[0].mass - 15.9989995956).abs() < 1e-10);

        // Test string with unit line
        let species_mass_str_with_unit = r#"%BLOCK SPECIES_MASS
kg
       O     15.9989995956
      Al     26.9820003510
%ENDBLOCK SPECIES_MASS
"#;

        // 2. Test Deserialization (with unit line)
        let cell_file_result_with_unit: Result<CellFileSpeciesMass, _> =
            from_str(species_mass_str_with_unit);
        assert!(
            cell_file_result_with_unit.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result_with_unit.err()
        );
        let cell_file_with_unit = cell_file_result_with_unit.unwrap();
        // Debug print the deserialized struct
        println!(
            "Deserialized SPECIES_MASS (with unit): {:#?}",
            cell_file_with_unit.species_mass
        );
        assert_eq!(
            cell_file_with_unit.species_mass.unit,
            Some(MassUnit::Kilogram)
        );
        assert_eq!(cell_file_with_unit.species_mass.masses.len(), 2);
        assert_eq!(
            cell_file_with_unit.species_mass.masses[1].species,
            Species::Symbol("Al".to_string())
        );
        assert!((cell_file_with_unit.species_mass.masses[1].mass - 26.9820003510).abs() < 1e-10);

        // 3. Test Serialization using ToCell
        let test_masses = SpeciesMass {
            unit: Some(MassUnit::Gram), // Example with unit
            masses: vec![
                SpeciesMassEntry {
                    species: Species::Symbol("Fe".to_string()),
                    mass: 55.845,
                },
                SpeciesMassEntry {
                    species: Species::AtomicNumber(8), // Oxygen
                    mass: 15.999,
                },
            ],
        };

        let serialized_result = to_string(&test_masses.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized SPECIES_MASS:\n{serialized_string}"); // Clippy suggestion

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK SPECIES_MASS"));
        assert!(serialized_string.contains("%ENDBLOCK SPECIES_MASS"));
        assert!(serialized_string.contains("Fe"));
        assert!(serialized_string.contains("8")); // Atomic number for Oxygen
        assert!(serialized_string.contains("55.84499999"));
        assert!(serialized_string.contains("15.999"));
        assert!(serialized_string.contains("g")); // Unit identifier
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
    }
}
