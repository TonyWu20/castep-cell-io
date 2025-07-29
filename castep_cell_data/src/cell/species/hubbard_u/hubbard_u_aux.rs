use serde::Deserialize;

use super::{AtomHubbardU, HubbardU, HubbardUUnit};

/// Intermediate representation for deserializing the HUBBARD_U block content.
/// Handles the optional unit line followed by atom-specific lines.
/// Since the number of entries is variable and line formats differ,
/// we use a Vec of a uniform line representation.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(super) struct HubbardURepr {
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
