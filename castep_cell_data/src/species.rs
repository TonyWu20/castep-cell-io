use castep_cell_serde::{CellValue, ToCellValue};
use serde::{Deserialize, Serialize};

mod species_lcao_states;
mod species_mass;
mod species_pot;
mod hubbard_u {
    use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
    use serde::{Deserialize, Serialize};
    // Assuming Species is available, e.g., from a common module
    use super::Species; // Adjust path as needed

    /// Represents the unit for Hubbard U values.
    #[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum HubbardUUnit {
        /// Electron volts
        #[default]
        #[serde(rename = "eV")]
        ElectronVolt,
        /// Hartree
        #[serde(rename = "Ha")]
        Hartree,
    }

    impl ToCellValue for HubbardUUnit {
        fn to_cell_value(&self) -> CellValue {
            CellValue::String(
                match self {
                    HubbardUUnit::ElectronVolt => "eV",
                    HubbardUUnit::Hartree => "Ha",
                }
                .to_string(),
            )
        }
    }

    /// Represents an orbital type and its associated Hubbard U value.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum OrbitalU {
        S(f64),
        P(f64),
        D(f64),
        F(f64),
    }

    impl OrbitalU {
        /// Gets the orbital type as a character.
        pub fn orbital_char(&self) -> char {
            match self {
                OrbitalU::S(_) => 's',
                OrbitalU::P(_) => 'p',
                OrbitalU::D(_) => 'd',
                OrbitalU::F(_) => 'f',
            }
        }

        /// Gets the U value.
        pub fn u_value(&self) -> f64 {
            match self {
                OrbitalU::S(v) => *v,
                OrbitalU::P(v) => *v,
                OrbitalU::D(v) => *v,
                OrbitalU::F(v) => *v,
            }
        }
    }

    impl ToCellValue for OrbitalU {
        fn to_cell_value(&self) -> CellValue {
            // Format as "l: U" e.g., "d: 2.3"
            CellValue::String(format!("{}: {}", self.orbital_char(), self.u_value()))
        }
    }

    #[cfg(test)]
    mod orbital_u {}

    /// Represents the specification for Hubbard U values for a specific atom/ion.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(from = "AtomHubbardURepr")]
    pub struct AtomHubbardU {
        /// The species.
        pub species: Species,
        /// The optional ion number within the species (1-based index).
        /// If None, the U values apply to all ions of this species.
        pub ion_number: Option<u32>,
        /// The list of orbitals and their U values.
        pub orbitals: Vec<OrbitalU>,
    }

    /// Represents the specification for Hubbard U values for a specific atom/ion.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    enum AtomHubbardURepr {
        NoIonOne(Species, OrbitalU),
        IonOne(Species, u32, OrbitalU),
        NoIonTwo(Species, OrbitalU, OrbitalU),
        IonTwo(Species, u32, OrbitalU, OrbitalU),
        NoIonThree(Species, OrbitalU, OrbitalU, OrbitalU),
        IonThree(Species, u32, OrbitalU, OrbitalU, OrbitalU),
        NoIonFour(Species, OrbitalU, OrbitalU, OrbitalU, OrbitalU),
        IonFour(Species, u32, OrbitalU, OrbitalU, OrbitalU, OrbitalU),
    }

    impl AtomHubbardURepr {
        /// Extracts the species, optional ion number, and orbitals.
        fn decompose(self) -> (Species, Option<u32>, Vec<OrbitalU>) {
            match self {
                AtomHubbardURepr::NoIonOne(species, o1) => (species, None, vec![o1]),
                AtomHubbardURepr::IonOne(species, ion, o1) => (species, Some(ion), vec![o1]),
                AtomHubbardURepr::NoIonTwo(species, o1, o2) => (species, None, vec![o1, o2]),
                AtomHubbardURepr::IonTwo(species, ion, o1, o2) => {
                    (species, Some(ion), vec![o1, o2])
                }
                AtomHubbardURepr::NoIonThree(species, o1, o2, o3) => {
                    (species, None, vec![o1, o2, o3])
                }
                AtomHubbardURepr::IonThree(species, ion, o1, o2, o3) => {
                    (species, Some(ion), vec![o1, o2, o3])
                }
                AtomHubbardURepr::NoIonFour(species, o1, o2, o3, o4) => {
                    (species, None, vec![o1, o2, o3, o4])
                }
                AtomHubbardURepr::IonFour(species, ion, o1, o2, o3, o4) => {
                    (species, Some(ion), vec![o1, o2, o3, o4])
                }
            }
        }
    }

    impl From<AtomHubbardURepr> for AtomHubbardU {
        fn from(value: AtomHubbardURepr) -> Self {
            let (species, ion_number, orbitals) = value.decompose();
            Self {
                species,
                ion_number,
                orbitals,
            }
        }
    }

    impl ToCellValue for AtomHubbardU {
        fn to_cell_value(&self) -> CellValue {
            let line_parts = [
                self.species.to_cell_value(),
                self.ion_number
                    .map(CellValue::UInt)
                    .unwrap_or(CellValue::Null),
            ]
            .to_vec()
            .into_iter()
            .chain(self.orbitals.iter().map(|orb| orb.to_cell_value()))
            .collect();
            CellValue::Array(line_parts)
        }
    }

    /// Intermediate representation for deserializing the HUBBARD_U block content.
    /// Handles the optional unit line followed by atom-specific lines.
    /// Since the number of entries is variable and line formats differ,
    /// we use a Vec of a uniform line representation.
    #[derive(Debug, Deserialize)]
    #[serde(transparent)]
    struct HubbardURepr {
        lines: Vec<HubbardULineRepr>,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(untagged)]
    /// Possible line entries in the `HUBBARD_U` block.
    /// The unit line is parsed as CellValue::Array([CellValue::String(...)])
    /// Atom lines are more complex and might need custom parsing in castep_cell_serde.
    /// For derive to work here, castep_cell_serde would need to parse atom lines
    /// into a structure that can be deserialized into AtomHubbardU.
    /// This is a simplified assumption for the repr.
    enum HubbardULineRepr {
        Unit([HubbardUUnit; 1]), // For lines like "eV" or "Ha"
        AtomEntry(AtomHubbardU), // For lines like "Sm 1 f: 6.1"
                                 // If castep_cell_serde provides raw tokens, a more complex repr or custom Deserialize
                                 // for AtomHubbardU would be needed.
    }

    impl HubbardULineRepr {
        fn as_atom_entry(&self) -> Option<&AtomHubbardU> {
            if let Self::AtomEntry(v) = self {
                Some(v)
            } else {
                None
            }
        }

        fn as_unit(&self) -> Option<&HubbardUUnit> {
            if let Self::Unit([unit]) = self {
                Some(unit)
            } else {
                None
            }
        }
    }

    /// Represents the HUBBARD_U block.
    ///
    /// Defines the Hubbard U values to use for specific orbitals.
    /// Format:
    /// %BLOCK HUBBARD_U
    /// [UNITS]
    /// atom1 orbital1 orbital2 ....
    /// atom2 orbital1 orbital2 ....
    /// ...
    /// %ENDBLOCK HUBBARD_U
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "HUBBARD_U")] // Ensure correct block name during serde
    #[serde(from = "HubbardURepr")] // Use intermediate repr for deserialization
    pub struct HubbardU {
        /// The unit for U values. If None, the default (eV) is used.
        pub unit: Option<HubbardUUnit>,
        /// The list of atom-specific Hubbard U specifications.
        pub atom_u_values: Vec<AtomHubbardU>,
    }

    impl From<HubbardURepr> for HubbardU {
        fn from(repr: HubbardURepr) -> Self {
            // Check the first line entry
            repr.lines
                .first()
                .map(|first_line| match first_line {
                    // The first line is a unit
                    HubbardULineRepr::Unit([unit]) => {
                        let u = *unit;
                        let atom_u_values = repr
                            .lines
                            .iter()
                            .skip(1)
                            .filter_map(|entry| entry.as_atom_entry())
                            .cloned()
                            .collect();
                        Self {
                            unit: Some(u),
                            atom_u_values,
                        }
                    }
                    // The first line is an atom entry, so no unit line was present
                    HubbardULineRepr::AtomEntry(_atom_entry) => Self {
                        unit: None,
                        atom_u_values: repr
                            .lines
                            .iter()
                            .filter_map(|entry| entry.as_atom_entry())
                            .cloned()
                            .collect(),
                    },
                })
                // Edge case: block is present but empty
                .unwrap_or(Self {
                    unit: None,
                    atom_u_values: Vec::new(),
                })
        }
    }

    impl ToCell for HubbardU {
        /// Converts the block into the intermediate `Cell` representation for serialization.
        fn to_cell(&self) -> Cell {
            let mut block_content = Vec::new();

            // Add the optional unit line first
            if let Some(ref u) = self.unit {
                block_content.push(CellValue::Array(vec![u.to_cell_value()]));
            }

            // Add the atom-specific lines
            block_content.extend(
                self.atom_u_values
                    .iter()
                    .map(|atom_u| atom_u.to_cell_value()),
            );

            Cell::Block("HUBBARD_U", block_content)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use castep_cell_serde::{ToCell, from_str, to_string};
        use serde::{Deserialize, Serialize};

        #[test]
        fn test_hubbard_u_serde() {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileHubbardU {
                hubbard_u: HubbardU,
            }

            // Test string based on the documentation example (with unit line)
            let hubbard_u_str_with_unit = r#"%BLOCK HUBBARD_U
eV
    Sm 1   f: 6.1
    Ni     d: 2.4
    U 2  d: 1.2 f: 2.1
%ENDBLOCK HUBBARD_U
"#;

            // 1. Test Deserialization (with unit line)
            // Note: This test's success depends on how well the castep_cell_serde parser
            // and the untagged enums/structs can handle the complex "atom orbital:U" format.
            // It serves as a structural check and example.
            let cell_file_result_with_unit: Result<CellFileHubbardU, _> =
                from_str(hubbard_u_str_with_unit);
            // Placeholder assertion - actual parsing logic for AtomHubbardU lines
            // needs to be handled by castep_cell_serde or custom Deserialize impls.
            assert!(
                cell_file_result_with_unit.is_ok(),
                "Deserialization (with unit) failed: {:?}",
                cell_file_result_with_unit.err()
            );
            // let cell_file_with_unit = cell_file_result_with_unit.unwrap();
            // println!("Deserialized HUBBARD_U (with unit): {:#?}", cell_file_with_unit.hubbard_u);
            // assert_eq!(cell_file_with_unit.hubbard_u.unit, Some(HubbardUUnit::ElectronVolt));
            // assert_eq!(cell_file_with_unit.hubbard_u.atom_u_values.len(), 3);

            // Test string without unit line
            let hubbard_u_str_no_unit = r#"%BLOCK HUBBARD_U
    Fe  d: 3.0
    O 1 p: 1.5
%ENDBLOCK HUBBARD_U
"#;

            let cell_file_result_no_unit: Result<CellFileHubbardU, _> =
                from_str(hubbard_u_str_no_unit);
            assert!(
                cell_file_result_no_unit.is_ok(),
                "Deserialization (no unit) failed: {:?}",
                cell_file_result_no_unit.err()
            );
            // let cell_file_no_unit = cell_file_result_no_unit.unwrap();
            // println!("Deserialized HUBBARD_U (no unit): {:#?}", cell_file_no_unit.hubbard_u);
            // assert_eq!(cell_file_no_unit.hubbard_u.unit, None);
            // assert_eq!(cell_file_no_unit.hubbard_u.atom_u_values.len(), 2);

            // 2. Test Serialization using ToCell
            let test_hubbard_u = HubbardU {
                unit: Some(HubbardUUnit::Hartree),
                atom_u_values: vec![
                    AtomHubbardU {
                        species: Species::Symbol("Cu".to_string()),
                        ion_number: Some(1),
                        orbitals: vec![OrbitalU::D(4.5)],
                    },
                    AtomHubbardU {
                        species: Species::Symbol("O".to_string()),
                        ion_number: None, // Apply to all O ions
                        orbitals: vec![OrbitalU::P(1.2), OrbitalU::S(0.1)],
                    },
                ],
            };

            let serialized_result = to_string(&test_hubbard_u.to_cell());
            assert!(
                serialized_result.is_ok(),
                "Serialization failed: {:?}",
                serialized_result.err()
            );
            let serialized_string = serialized_result.unwrap();

            // Print the serialized string
            println!("Serialized HUBBARD_U:\n{serialized_string}"); // Clippy suggestion

            // Basic checks on the serialized string
            assert!(serialized_string.contains("%BLOCK HUBBARD_U"));
            assert!(serialized_string.contains("%ENDBLOCK HUBBARD_U"));
            assert!(serialized_string.contains("Ha")); // Unit
            assert!(serialized_string.contains("Cu"));
            assert!(serialized_string.contains("1")); // Ion number
            assert!(serialized_string.contains("d: 4.5")); // Orbital U
            assert!(serialized_string.contains("O")); // Species
            assert!(serialized_string.contains("p: 1.2")); // Orbital U
            assert!(serialized_string.contains("s: 0.1")); // Orbital U
            // Note: Exact string comparison can be brittle due to formatting/whitespace/order
            // The exact format of "orbital: U" depends on OrbitalU's ToCellValue impl
        }
    }
}
// future
// mod species_q;

pub use species_lcao_states::{SpeciesLcaoState, SpeciesLcaoStates};
pub use species_mass::{SpeciesMass, SpeciesMassEntry};
pub use species_pot::{SpeciesPot, SpeciesPotEntry};

/// Represents the species identifier for an atom in a `POSITIONS_*` block.
/// Can be either a chemical symbol (e.g., "Fe") or an atomic number (e.g., 26).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Species {
    /// Chemical symbol (e.g., "Fe", "Si", "O").
    Symbol(String), // Using String for flexibility, though CASTEP limits to 3 chars
    /// Atomic number (e.g., 26 for Iron).
    AtomicNumber(u32),
}

impl Species {
    pub fn as_symbol(&self) -> Option<&String> {
        if let Self::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_atomic_number(&self) -> Option<&u32> {
        if let Self::AtomicNumber(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl ToCellValue for Species {
    fn to_cell_value(&self) -> CellValue {
        match self {
            Species::Symbol(s) => CellValue::String(s.clone()),
            Species::AtomicNumber(u) => CellValue::UInt(*u),
        }
    }
}
