use crate::cell::species::Species;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Represents a specific atom site, including its species, index within that species,
/// and the periodic image (Miller indices) it occupies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtomSite {
    /// The species of the atom (e.g., "H", "O", "C").
    pub species: Species, // Reusing the shared Species enum
    /// The ion number within the species (1-based index).
    pub ion_number: u32,
    /// The periodic image indices [N1, N2, N3] (Miller indices).
    pub image_indices: [i32; 3],
}

impl AtomSite {
    pub fn new(species: Species, ion_number: u32, image_indices: [i32; 3]) -> Self {
        Self {
            species,
            ion_number,
            image_indices,
        }
    }
}

/// Intermediate representation for deserializing a single `AtomSite`.
/// Handles the flat format: Species IonNum N1 N2 N3
#[derive(Debug, Deserialize)]
struct AtomSiteRepr {
    species: Species,
    ion_number: u32,
    n1: i32,
    n2: i32,
    n3: i32,
}

impl From<AtomSiteRepr> for AtomSite {
    fn from(repr: AtomSiteRepr) -> Self {
        let AtomSiteRepr {
            species,
            ion_number,
            n1,
            n2,
            n3,
        } = repr;
        Self {
            species,
            ion_number,
            image_indices: [n1, n2, n3],
        }
    }
}

impl ToCellValue for AtomSite {
    /// Converts the atom site into a `CellValue::Array` representing part of a constraint line.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            vec![
                self.species.to_cell_value(), // Converts Species to CellValue::String or CellValue::UInt
                CellValue::UInt(self.ion_number),
            ]
            .into_iter()
            .chain(self.image_indices.into_iter().map(CellValue::Int))
            .collect(),
        )
    }
}

/// Represents the type of internal coordinate constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // Matches "distance", "bend", "torsion" in the example
pub enum ConstraintType {
    /// Constrains the distance between two atoms.
    Distance,
    /// Constrains the bend (angle) formed by three atoms.
    Bend,
    /// Constrains the torsion (dihedral) angle formed by four atoms.
    Torsion,
    // Add other constraint types if they exist in CASTEP
}

impl ToCellValue for ConstraintType {
    fn to_cell_value(&self) -> CellValue {
        // Serialize the enum variant name as a string, matching the lowercase format
        CellValue::String(
            match self {
                ConstraintType::Distance => "distance",
                ConstraintType::Bend => "    bend",
                ConstraintType::Torsion => " torsion",
                // Add arms for other variants
            }
            .to_string(),
        )
    }
}

/// Represents a single nonlinear constraint entry within the NONLINEAR_CONSTRAINTS block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "NonlinearConstraintRepr")] // Use intermediate repr for deserialization
pub struct NonlinearConstraint {
    /// The type of the constraint (distance, bend, torsion).
    pub constraint_type: ConstraintType,
    /// The list of atom sites involved in the constraint.
    /// - 2 atoms for Distance
    /// - 3 atoms for Bend
    /// - 4 atoms for Torsion
    pub atom_sites: Vec<AtomSite>,
}

/// Intermediate representation for deserializing a `NonlinearConstraint`.
/// Handles the variable number of `AtomSite`s based on constraint type.
/// Note: This assumes the deserializer can parse the variable parts correctly.
/// The actual parsing might be complex depending on `castep_cell_serde`'s tokenization.
/// This is a simplified representation assuming correct parsing into nested structures.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(from = "Lines")]
enum NonlinearConstraintRepr {
    Distance(ConstraintType, AtomSite, AtomSite),
    Bend(ConstraintType, AtomSite, AtomSite, AtomSite),
    Torsion(ConstraintType, AtomSite, AtomSite, AtomSite, AtomSite),
}

impl From<NonlinearConstraintRepr> for NonlinearConstraint {
    fn from(repr: NonlinearConstraintRepr) -> Self {
        match repr {
            NonlinearConstraintRepr::Distance(constraint_type, atom_site, atom_site1) => Self {
                constraint_type,
                atom_sites: vec![atom_site, atom_site1],
            },
            NonlinearConstraintRepr::Bend(constraint_type, atom_site, atom_site1, atom_site2) => {
                Self {
                    constraint_type,
                    atom_sites: [atom_site, atom_site1, atom_site2].to_vec(),
                }
            }
            NonlinearConstraintRepr::Torsion(
                constraint_type,
                atom_site,
                atom_site1,
                atom_site2,
                atom_site3,
            ) => Self {
                constraint_type,
                atom_sites: [atom_site, atom_site1, atom_site2, atom_site3].to_vec(),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Lines {
    Distance(
        ConstraintType,
        Species,
        u32,
        i32,
        i32,
        i32,
        Species,
        u32,
        i32,
        i32,
        i32,
    ),
    Bend(
        ConstraintType,
        Species,
        u32,
        i32,
        i32,
        i32,
        Species,
        u32,
        i32,
        i32,
        i32,
        Species,
        u32,
        i32,
        i32,
        i32,
    ),
    Torsion(
        ConstraintType,
        Species,
        u32,
        i32,
        i32,
        i32,
        Species,
        u32,
        i32,
        i32,
        i32,
        Species,
        u32,
        i32,
        i32,
        i32,
        Species,
        u32,
        i32,
        i32,
        i32,
    ),
}

impl From<Lines> for NonlinearConstraintRepr {
    fn from(value: Lines) -> Self {
        match value {
            Lines::Distance(
                constraint_type,
                species,
                u,
                ix,
                iy,
                iz,
                species1,
                u1,
                ix1,
                iy1,
                iz1,
            ) => NonlinearConstraintRepr::Distance(
                constraint_type,
                AtomSite::new(species, u, [ix, iy, iz]),
                AtomSite::new(species1, u1, [ix1, iy1, iz1]),
            ),
            Lines::Bend(
                constraint_type,
                species,
                u,
                ix,
                iy,
                iz,
                species1,
                u1,
                ix1,
                iy1,
                iz1,
                species2,
                u2,
                ix2,
                iy2,
                iz2,
            ) => NonlinearConstraintRepr::Bend(
                constraint_type,
                AtomSite::new(species, u, [ix, iy, iz]),
                AtomSite::new(species1, u1, [ix1, iy1, iz1]),
                AtomSite::new(species2, u2, [ix2, iy2, iz2]),
            ),
            Lines::Torsion(
                constraint_type,
                species,
                u,
                ix,
                iy,
                iz,
                species1,
                u1,
                ix1,
                iy1,
                iz1,
                species2,
                u2,
                ix2,
                iy2,
                iz2,
                species3,
                u3,
                ix3,
                iy3,
                iz3,
            ) => NonlinearConstraintRepr::Torsion(
                constraint_type,
                AtomSite::new(species, u, [ix, iy, iz]),
                AtomSite::new(species1, u1, [ix1, iy1, iz1]),
                AtomSite::new(species2, u2, [ix2, iy2, iz2]),
                AtomSite::new(species3, u3, [ix3, iy3, iz3]),
            ),
        }
    }
}

impl ToCellValue for NonlinearConstraint {
    /// Converts the constraint into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [self.constraint_type.to_cell_value()]
                .to_vec()
                .into_iter()
                .chain(
                    self.atom_sites
                        .iter()
                        .flat_map(|atom_site| atom_site.to_cell_value().as_array().unwrap().clone())
                        .collect::<Vec<CellValue>>(),
                )
                .collect(),
        )
    }
}

/// Represents the NONLINEAR_CONSTRAINTS block.
///
/// Defines constrained internal coordinates (bonds, angles, torsions) that are
/// held fixed at the values of the initial structure.
/// Format:
/// %BLOCK NONLINEAR_CONSTRAINTS
/// CONSTRAIN_TYPE atom1 atom2 (atom3 atom4)
/// ...
/// %ENDBLOCK NONLINEAR_CONSTRAINTS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "NONLINEAR_CONSTRAINTS")] // Ensure correct block name during serde
#[serde(transparent)] // Serialize/Deserialize as if it's directly the Vec
pub struct NonlinearConstraints {
    /// The list of nonlinear constraint entries.
    pub constraints: Vec<NonlinearConstraint>,
}

impl ToCell for NonlinearConstraints {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "NONLINEAR_CONSTRAINTS", // Block name
            self.constraints
                .iter()
                .map(|constraint| constraint.to_cell_value()) // Convert each entry to CellValue::Array
                .collect::<Vec<CellValue>>(), // Collect into Vec for the block content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};
    // Ensure the test can access the shared Species if needed,
    // or mock/duplicate it if it's in a separate crate/module.

    #[test]
    fn test_nonlinear_constraints_serde_structure() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileNonlinearConstraints {
            nonlinear_constraints: NonlinearConstraints,
        }

        // Test string based on the documentation example
        let nonlinear_constraints_str = r#"%BLOCK NONLINEAR_CONSTRAINTS
distance       H  4  0  0  0       O  2  0  1  0
    bend       H  5  0  0  0       C  1  1  0  1       H  2  0  0  0
 torsion       H  6  0  0  0       H  3  1  0  0       H  1  0  0  1       H  9  1  1  0
%ENDBLOCK NONLINEAR_CONSTRAINTS
"#;

        // --- Deserialization Test ---
        // Note: The deserialization of `NonlinearConstraint` is complex due to the
        // variable number of atom sites per line. The provided `NonlinearConstraintRepr`
        // and `From` implementation are simplified and might not work directly
        // with `castep_cell_serde`'s default parsing, which typically provides
        // a flat `Vec<CellValue>` for each line.
        //
        // A full implementation would likely require:
        // 1. A custom `Deserialize` implementation for `NonlinearConstraint`
        //    that consumes a `Vec<CellValue>` representing the tokens of one line.
        // 2. Or, deserializing the block content into a `Vec<Vec<CellValue>>` first,
        //    and then manually parsing each inner `Vec<CellValue>` into a `NonlinearConstraint`.
        //
        // For this example, we will focus on testing the *structure* and serialization
        // assuming deserialization works correctly (e.g., by mocking the deserialized data).
        //
        // Uncommenting the lines below will likely fail due to the complexity mentioned.
        //
        let cell_file_result: Result<CellFileNonlinearConstraints, _> =
            from_str(nonlinear_constraints_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        println!(
            "Deserialized NONLINEAR_CONSTRAINTS: {:#?}",
            cell_file.nonlinear_constraints
        );
        assert_eq!(cell_file.nonlinear_constraints.constraints.len(), 3); // Example check

        // --- Serialization Test (using manually created data) ---
        // Create test data manually to verify the ToCell/ToCellValue implementations
        let test_constraints = NonlinearConstraints {
            constraints: vec![
                NonlinearConstraint {
                    constraint_type: ConstraintType::Distance,
                    atom_sites: vec![
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 4,
                            image_indices: [0, 0, 0],
                        },
                        AtomSite {
                            species: Species::Symbol("O".to_string()),
                            ion_number: 2,
                            image_indices: [0, 1, 0],
                        },
                    ],
                },
                NonlinearConstraint {
                    constraint_type: ConstraintType::Bend,
                    atom_sites: vec![
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 5,
                            image_indices: [0, 0, 0],
                        },
                        AtomSite {
                            species: Species::Symbol("C".to_string()),
                            ion_number: 1,
                            image_indices: [1, 0, 1],
                        },
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 2,
                            image_indices: [0, 0, 0],
                        },
                    ],
                },
                NonlinearConstraint {
                    constraint_type: ConstraintType::Torsion,
                    atom_sites: vec![
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 6,
                            image_indices: [0, 0, 0],
                        },
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 3,
                            image_indices: [1, 0, 0],
                        },
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 1,
                            image_indices: [0, 0, 1],
                        },
                        AtomSite {
                            species: Species::Symbol("H".to_string()),
                            ion_number: 9,
                            image_indices: [1, 1, 0],
                        },
                    ],
                },
            ],
        };

        let serialized_result = to_string(&test_constraints.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized NONLINEAR_CONSTRAINTS:\n{serialized_string}");

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK NONLINEAR_CONSTRAINTS"));
        assert!(serialized_string.contains("%ENDBLOCK NONLINEAR_CONSTRAINTS"));
        assert!(serialized_string.contains("distance"));
        assert!(serialized_string.contains("bend"));
        assert!(serialized_string.contains("torsion"));
        assert!(serialized_string.contains("H"));
        assert!(serialized_string.contains("O"));
        assert!(serialized_string.contains("C"));
        // Add more specific checks if needed, though exact formatting might vary
    }
}
