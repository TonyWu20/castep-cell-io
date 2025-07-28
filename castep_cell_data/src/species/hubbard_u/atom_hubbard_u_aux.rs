use crate::species::hubbard_u::AtomHubbardU;
use crate::species::hubbard_u::OrbitalU;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

use crate::species::Species;
/// Represents the specification for Hubbard U values for a specific atom/ion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum AtomHubbardURepr<'a> {
    NoIonOne(Species, &'a str, f64),
    IonOne(Species, u32, &'a str, f64),
    NoIonTwo(Species, &'a str, f64, &'a str, f64),
    IonTwo(Species, u32, &'a str, f64, &'a str, f64),
    NoIonThree(Species, &'a str, f64, &'a str, f64, &'a str, f64),
    IonThree(Species, u32, &'a str, f64, &'a str, f64, &'a str, f64),
    NoIonFour(
        Species,
        &'a str,
        f64,
        &'a str,
        f64,
        &'a str,
        f64,
        &'a str,
        f64,
    ),
    IonFour(
        Species,
        u32,
        &'a str,
        f64,
        &'a str,
        f64,
        &'a str,
        f64,
        &'a str,
        f64,
    ),
}

impl AtomHubbardURepr<'_> {
    /// Extracts the species, optional ion number, and orbitals.
    fn decompose(self) -> (Species, Option<u32>, Vec<OrbitalU>) {
        match self {
            AtomHubbardURepr::NoIonOne(species, orb, val) => (
                species,
                None,
                vec![OrbitalU::from_str_f64(orb, val).unwrap()],
            ),
            AtomHubbardURepr::IonOne(species, ion_number, orb, val) => (
                species,
                Some(ion_number),
                vec![OrbitalU::from_str_f64(orb, val).unwrap()],
            ),
            AtomHubbardURepr::NoIonTwo(species, orb, val, orb1, val1) => (
                species,
                None,
                [
                    OrbitalU::from_str_f64(orb, val),
                    OrbitalU::from_str_f64(orb1, val1),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
            AtomHubbardURepr::IonTwo(species, ion_number, orb, val, orb1, val1) => (
                species,
                Some(ion_number),
                [
                    OrbitalU::from_str_f64(orb, val),
                    OrbitalU::from_str_f64(orb1, val1),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
            AtomHubbardURepr::NoIonThree(species, orb, val, orb1, val1, orb2, val2) => (
                species,
                None,
                [
                    OrbitalU::from_str_f64(orb, val),
                    OrbitalU::from_str_f64(orb1, val1),
                    OrbitalU::from_str_f64(orb2, val2),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
            AtomHubbardURepr::IonThree(species, ion_number, orb, val, orb1, val1, orb2, val2) => (
                species,
                Some(ion_number),
                [
                    OrbitalU::from_str_f64(orb, val),
                    OrbitalU::from_str_f64(orb1, val1),
                    OrbitalU::from_str_f64(orb2, val2),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
            AtomHubbardURepr::NoIonFour(species, orb, val, orb1, val1, orb2, val2, orb3, val3) => (
                species,
                None,
                [
                    OrbitalU::from_str_f64(orb, val),
                    OrbitalU::from_str_f64(orb1, val1),
                    OrbitalU::from_str_f64(orb2, val2),
                    OrbitalU::from_str_f64(orb3, val3),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
            AtomHubbardURepr::IonFour(
                species,
                ion_number,
                orb,
                val,
                orb1,
                val1,
                orb2,
                val2,
                orb3,
                val3,
            ) => (
                species,
                Some(ion_number),
                [
                    OrbitalU::from_str_f64(orb, val),
                    OrbitalU::from_str_f64(orb1, val1),
                    OrbitalU::from_str_f64(orb2, val2),
                    OrbitalU::from_str_f64(orb3, val3),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
        }
    }
}

impl<'a> From<AtomHubbardURepr<'a>> for AtomHubbardU {
    fn from(value: AtomHubbardURepr) -> Self {
        let (species, ion_number, orbitals) = value.decompose();
        Self {
            species,
            ion_number,
            orbitals,
        }
    }
}
#[cfg(test)]
mod atom_hubbard_u {
    use castep_cell_serde::{CellValue, CellValueDeserializer};
    use serde::Deserialize;

    use crate::species::hubbard_u::AtomHubbardU;

    #[test]
    fn test_atom_hubbard_u() {
        let atom_hubbard_u =
            AtomHubbardU::deserialize(&mut CellValueDeserializer::new(&CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Str("d:"),
                CellValue::Float(3.0),
            ])))
            .unwrap();
        dbg!(atom_hubbard_u);
    }
}
