use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use castep_periodic_table::data::ELEMENT_TABLE;
use castep_periodic_table::element::LookupElement;

use crate::{
    cell_document::{
        params::{CastepParams, CastepTask},
        CellEntries, ExtEFieldBlock, ExtPressureBlock, FixAllCell, FixCom, IonicConstraintsBlock,
        KpointListBlock, KpointQuality, KpointSettings, NCKpointSettings, SpeciesLCAOStatesBlock,
        SpeciesMassBlock, SpeciesPotBlock,
    },
    CellDocument,
};

pub struct SeedfileGenerator {
    task: CastepTask,
    cell_doc: CellDocument,
    use_edft: Option<bool>,
    kpoint_quality: Option<KpointQuality>,
}

impl SeedfileGenerator {
    pub fn new(task: CastepTask, cell_doc: CellDocument) -> Self {
        Self {
            task,
            cell_doc,
            use_edft: None,
            kpoint_quality: None,
        }
    }
    fn get_total_spin(&self) -> u32 {
        self.cell_doc
            .get_elements()
            .iter()
            .map(|&elm| ELEMENT_TABLE.get_by_symbol(elm).spin() as u32)
            .sum::<u32>()
    }
    fn get_cutoff_energy<P: AsRef<Path>>(&self, potentials_loc: P) -> Result<f64, io::Error> {
        Ok(self
            .cell_doc
            .get_elements()
            .iter()
            .map(|&elm| -> Result<f64, io::Error> {
                let potential_file = ELEMENT_TABLE.get_by_symbol(elm).potential();
                let potential_path = Path::new(potentials_loc.as_ref()).join(potential_file);
                let file = File::open(potential_path)?;
                let reader = BufReader::new(file);
                let fine_energy = reader
                    .lines()
                    .find(|line| line.as_ref().unwrap().contains("FINE"))
                    .map(|line| {
                        let num_str = line.as_ref().unwrap().split_whitespace().next().unwrap();
                        num_str.parse::<u32>().expect("Can't parse into `u32`")
                    })
                    .expect("Failed to parse fine energy from pseudopotential file.");
                let round_bigger_tenth = |num: u32| -> f64 {
                    match num % 10 {
                        0 => num as f64,
                        _ => ((num / 10 + 1) * 10) as f64,
                    }
                };
                let ultra_fine_energy = round_bigger_tenth((fine_energy as f64 * 1.1) as u32);
                Ok(ultra_fine_energy)
            })
            .filter_map(|res| res.ok())
            .reduce(|prev, next| if next > prev { next } else { prev })
            .expect("Error in comparing the largest cutoff energy"))
    }

    fn geom_opt_cell(&self) -> CellDocument {
        let elements = self.cell_doc.get_elements();
        let entries = vec![
            CellEntries::KpointSettings(KpointSettings::List(KpointListBlock::default())),
            CellEntries::FixAllCell(FixAllCell::new(true)),
            CellEntries::FixCom(FixCom::new(false)),
            CellEntries::IonicConstraints(IonicConstraintsBlock::default()),
            CellEntries::ExtEfield(ExtEFieldBlock::default()),
            CellEntries::ExtPressure(ExtPressureBlock::default()),
            CellEntries::SpeciesMass(SpeciesMassBlock::from_elements(&elements)),
            CellEntries::SpeciesPot(SpeciesPotBlock::from_elements(&elements)),
            CellEntries::SpeciesLCAOStates(SpeciesLCAOStatesBlock::from_elememts(&elements)),
        ];
        let mut geom_cell = self.cell_doc.clone();
        geom_cell.set_entries(Some(entries));
        geom_cell
    }
    fn bs_cell(&self) -> CellDocument {
        let mut bs_cell = self.cell_doc.clone();
        let elements = self.cell_doc.get_elements();
        let entries = vec![
            CellEntries::KpointSettings(KpointSettings::List(KpointListBlock::default())),
            CellEntries::NCKpointSettings(NCKpointSettings::List(KpointListBlock::default())),
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
    pub fn generate_cell_file(&self) -> CellDocument {
        match self.task {
            CastepTask::BandStructure => self.bs_cell(),
            CastepTask::GeometryOptimization => self.geom_opt_cell(),
        }
    }
    fn geom_opt_param<P: AsRef<Path>>(&self, potentials_loc: P) -> CastepParams {
        CastepParams::geom_opt(
            self.get_cutoff_energy(potentials_loc)
                .expect("Failed to get cutoff energy"),
            self.get_total_spin(),
            self.use_edft.unwrap_or(false),
        )
    }
    fn bs_param<P: AsRef<Path>>(&self, potentials_loc: P) -> CastepParams {
        CastepParams::band_structure(
            self.get_cutoff_energy(potentials_loc)
                .expect("Failed to get cutoff energy"),
            self.get_total_spin(),
            self.use_edft.unwrap_or(false),
        )
    }
    pub fn generate_castep_param<P: AsRef<Path>>(&self, potentials_loc: P) -> CastepParams {
        match self.task {
            CastepTask::BandStructure => self.bs_param(potentials_loc),
            CastepTask::GeometryOptimization => self.geom_opt_param(potentials_loc),
        }
    }
    // TODO: kptaux
}
