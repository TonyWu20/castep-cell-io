use std::path::Path;

use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{ElementFamily, LookupElement},
};

use crate::{
    cell_document::{
        params::{CastepParams, CastepTask},
        BSKpointPathSpacing, CellEntries, ExtEFieldBlock, ExtPressureBlock, FixAllCell, FixCom,
        IonicConstraintsBlock, KpointMPSpacing, KpointSettings, NCKpointSettings,
        SpeciesLCAOStatesBlock, SpeciesMassBlock, SpeciesPotBlock,
    },
    CellDocument, EnergyCutoff, EnergyCutoffError,
};

pub trait CellBuilding {
    #[cfg(feature = "template")]
    fn geom_opt_cell_template(template_cell: &CellDocument) -> CellDocument {
        let elements = template_cell.get_elements();
        let entries = vec![
            CellEntries::KpointSettings(KpointSettings::MPSpacing(KpointMPSpacing::default())),
            CellEntries::FixAllCell(FixAllCell::new(true)),
            CellEntries::FixCom(FixCom::new(false)),
            CellEntries::IonicConstraints(IonicConstraintsBlock::default()),
            CellEntries::ExtEfield(ExtEFieldBlock::default()),
            CellEntries::ExtPressure(ExtPressureBlock::default()),
            CellEntries::SpeciesMass(SpeciesMassBlock::from_elements(&elements)),
            CellEntries::SpeciesPot(SpeciesPotBlock::from_elements(&elements)),
            CellEntries::SpeciesLCAOStates(SpeciesLCAOStatesBlock::from_elememts(&elements)),
        ];
        let mut geom_cell = template_cell.clone();
        geom_cell.set_entries(Some(entries));
        geom_cell
    }

    #[cfg(feature = "template")]
    fn bs_cell_template(template_cell: &CellDocument) -> CellDocument {
        let mut bs_cell = template_cell.clone();
        let elements = template_cell.get_elements();
        let entries = vec![
            CellEntries::KpointSettings(KpointSettings::MPSpacing(KpointMPSpacing::default())),
            CellEntries::NCKpointSettings(NCKpointSettings::PathSpacing(BSKpointPathSpacing::new(
                crate::InvLengthUnit::Ang,
                0.07,
            ))),
            CellEntries::FixAllCell(FixAllCell::new(true)),
            CellEntries::FixCom(FixCom::new(false)),
            CellEntries::IonicConstraints(IonicConstraintsBlock::default()),
            CellEntries::ExtEfield(ExtEFieldBlock::default()),
            CellEntries::ExtPressure(ExtPressureBlock::default()),
            CellEntries::SpeciesMass(SpeciesMassBlock::from_elements(&elements)),
            CellEntries::SpeciesPot(SpeciesPotBlock::from_elements(&elements)),
            CellEntries::SpeciesLCAOStates(SpeciesLCAOStatesBlock::from_elememts(&elements)),
        ];
        bs_cell.set_entries(Some(entries));
        bs_cell
    }

    fn build_cell_for_task(
        &self,
        template_cell: &CellDocument,
        castep_task: CastepTask,
    ) -> CellDocument {
        #[cfg(feature = "template")]
        match castep_task {
            CastepTask::BandStructure => Self::bs_cell_template(template_cell),
            CastepTask::GeometryOptimization => Self::geom_opt_cell_template(template_cell),
        }
    }
}

pub trait ParamBuilding {
    fn cutoff_energy<P: AsRef<Path>>(
        &self,
        template_cell: &CellDocument,
        energy_cutoff: EnergyCutoff,
        potentials_loc: P,
    ) -> Result<f64, EnergyCutoffError> {
        let cutoff_energies = template_cell
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
    fn build_param_for_task(
        &self,
        template_cell: &CellDocument,
        castep_task: CastepTask,
    ) -> Result<CastepParams, EnergyCutoffError>;

    #[cfg(feature = "template")]
    fn geom_opt_param_template<P: AsRef<Path>>(
        &self,
        template_cell: &CellDocument,
        energy_cutoff: EnergyCutoff,
        use_edft: bool,
        potentials_loc: P,
    ) -> Result<CastepParams, EnergyCutoffError> {
        {
            let use_edft = if use_edft {
                template_cell.get_elements().iter().any(|elm| {
                    matches!(
                        elm.family(),
                        ElementFamily::RareEarthLa | ElementFamily::RareEarthAc
                    )
                })
            } else {
                false
            };
            Ok(CastepParams::geom_opt(
                self.cutoff_energy(template_cell, energy_cutoff, potentials_loc)?,
                template_cell.total_spin(),
                use_edft,
            ))
        }
    }

    #[cfg(feature = "template")]
    fn bs_param_template<P: AsRef<Path>>(
        &self,
        template_cell: &CellDocument,
        energy_cutoff: EnergyCutoff,
        use_edft: bool,
        potentials_loc: P,
    ) -> Result<CastepParams, EnergyCutoffError> {
        let use_edft = if use_edft {
            template_cell.get_elements().iter().any(|elm| {
                matches!(
                    elm.family(),
                    ElementFamily::RareEarthLa | ElementFamily::RareEarthAc
                )
            })
        } else {
            false
        };

        Ok(CastepParams::band_structure(
            self.cutoff_energy(template_cell, energy_cutoff, potentials_loc)?,
            template_cell.total_spin(),
            use_edft,
        ))
    }
}
