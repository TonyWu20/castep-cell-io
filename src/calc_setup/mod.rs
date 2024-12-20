use std::path::Path;

use castep_periodic_table::data::ELEMENT_TABLE;
use castep_periodic_table::element::LookupElement;

use crate::{
    cell_document::params::{CastepParams, CastepTask},
    CellDocument, EnergyCutoff, EnergyCutoffError,
};

pub trait CellBuilding {
    fn template_cell(&self) -> &CellDocument;
    fn get_total_spin(&self) -> u32 {
        self.template_cell()
            .get_elements()
            .iter()
            .map(|&elm| ELEMENT_TABLE.get_by_symbol(elm).spin() as u32)
            .sum::<u32>()
    }
    fn build_cell_for_task(&self, castep_task: CastepTask);
}
pub trait ParamBuilding {
    fn template_cell(&self) -> &CellDocument;
    fn cutoff_energy<P: AsRef<Path>>(
        &self,
        energy_cutoff: EnergyCutoff,
        potentials_loc: P,
    ) -> Result<f64, EnergyCutoffError> {
        let cutoff_energies = self
            .template_cell()
            .get_elements()
            .iter()
            .map(|&elm| {
                let potential_name = ELEMENT_TABLE.get_by_symbol(elm).potential();
                let potential_file = Path::new(potentials_loc.as_ref()).join(potential_name);
                energy_cutoff.get_cutoff_energy_from_pot(potential_file)
            })
            .collect::<Result<Vec<f64>, EnergyCutoffError>>()?;
        Ok(cutoff_energies
            .into_iter()
            .reduce(|prev, next| if next > prev { next } else { prev })
            .expect("Error in comparing the largest cutoff energy"))
    }
    fn build_param_for_task(&self, castep_task: CastepTask) -> Result<CastepParams, EnergyCutoff>;
}

// pub struct SeedfileGenerator {
//     task: CastepTask,
//     cell_doc: CellDocument,
//     use_edft: Option<bool>,
//     kpoint_quality: Option<KpointQuality>,
// }

// impl SeedfileGenerator {
//     pub fn use_edft(&mut self, use_edft: bool) {
//         self.use_edft = Some(use_edft);
//     }

//     pub fn set_kpoint_quality(&mut self, kpoint_quality: KpointQuality) {
//         self.kpoint_quality = Some(kpoint_quality);
//     }

//     pub fn new(task: CastepTask, cell_doc: CellDocument) -> Self {
//         let use_edft = cell_doc.get_elements().iter().any(|elm| {
//             matches!(elm.family(), ElementFamily::RareEarthLa)
//                 || matches!(elm.family(), ElementFamily::RareEarthAc)
//         });
//         Self {
//             task,
//             cell_doc,
//             use_edft: Some(use_edft),
//             kpoint_quality: None,
//         }
//     }

//     fn geom_opt_cell(&self) -> CellDocument {
//         let elements = self.cell_doc.get_elements();
//         let entries = vec![
//             CellEntries::KpointSettings(KpointSettings::MPSpacing(KpointMPSpacing::default())),
//             CellEntries::FixAllCell(FixAllCell::new(true)),
//             CellEntries::FixCom(FixCom::new(false)),
//             CellEntries::IonicConstraints(IonicConstraintsBlock::default()),
//             CellEntries::ExtEfield(ExtEFieldBlock::default()),
//             CellEntries::ExtPressure(ExtPressureBlock::default()),
//             CellEntries::SpeciesMass(SpeciesMassBlock::from_elements(&elements)),
//             CellEntries::SpeciesPot(SpeciesPotBlock::from_elements(&elements)),
//             CellEntries::SpeciesLCAOStates(SpeciesLCAOStatesBlock::from_elememts(&elements)),
//         ];
//         let mut geom_cell = self.cell_doc.clone();
//         geom_cell.set_entries(Some(entries));
//         geom_cell
//     }

//     fn bs_cell(&self) -> CellDocument {
//         let mut bs_cell = self.cell_doc.clone();
//         let elements = self.cell_doc.get_elements();
//         let entries = vec![
//             CellEntries::KpointSettings(KpointSettings::MPSpacing(KpointMPSpacing::default())),
//             CellEntries::NCKpointSettings(NCKpointSettings::PathSpacing(BSKpointPathSpacing::new(
//                 crate::InvLengthUnit::Ang,
//                 0.07,
//             ))),
//             CellEntries::FixAllCell(FixAllCell::new(true)),
//             CellEntries::FixCom(FixCom::new(false)),
//             CellEntries::IonicConstraints(IonicConstraintsBlock::default()),
//             CellEntries::ExtEfield(ExtEFieldBlock::default()),
//             CellEntries::ExtPressure(ExtPressureBlock::default()),
//             CellEntries::SpeciesMass(SpeciesMassBlock::from_elements(&elements)),
//             CellEntries::SpeciesPot(SpeciesPotBlock::from_elements(&elements)),
//             CellEntries::SpeciesLCAOStates(SpeciesLCAOStatesBlock::from_elememts(&elements)),
//         ];
//         bs_cell.set_entries(Some(entries));
//         bs_cell
//     }

//     pub fn generate_cell_file(&self) -> CellDocument {
//         match self.task {
//             CastepTask::BandStructure => self.bs_cell(),
//             CastepTask::GeometryOptimization => self.geom_opt_cell(),
//         }
//     }

//     // fn geom_opt_param<P: AsRef<Path>>(&self, potentials_loc: P) -> CastepParams {
//     //     CastepParams::geom_opt(
//     //         self.get_cutoff_energy(potentials_loc)
//     //             .expect("Failed to get cutoff energy"),
//     //         self.get_total_spin(),
//     //         self.use_edft.unwrap_or(false),
//     //     )
//     // }

//     // fn bs_param<P: AsRef<Path>>(&self, potentials_loc: P) -> CastepParams {
//     //     CastepParams::band_structure(
//     //         self.get_cutoff_energy(potentials_loc)
//     //             .expect("Failed to get cutoff energy"),
//     //         self.get_total_spin(),
//     //         self.use_edft.unwrap_or(false),
//     //     )
//     // }

//     // pub fn generate_castep_param<P: AsRef<Path>>(&self, potentials_loc: P) -> CastepParams {
//     //     match self.task {
//     //         CastepTask::BandStructure => self.bs_param(potentials_loc),
//     //         CastepTask::GeometryOptimization => self.geom_opt_param(potentials_loc),
//     //     }
//     // }
//     // TODO: kptaux
// }
