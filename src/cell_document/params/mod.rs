mod options;

use std::fmt::Display;

use options::{
    BandStructureParamSection, BasisSetParamSection, ElectroMinParamSection,
    ElectronicParamSection, GeneralParamSection, GeomOptParamSection, PopulationAnalysisSection,
    XcParamSection,
};
pub use options::{CastepTask, EnergyCutoff, EnergyCutoffError};

#[derive(Debug, Clone)]
pub enum ParamSections {
    General(GeneralParamSection),
    BandStructure(BandStructureParamSection),
    BasisSet(BasisSetParamSection),
    ElectroMinimization(ElectroMinParamSection),
    Electronic(ElectronicParamSection),
    GeomOpt(GeomOptParamSection),
    ExchangeCorrelation(XcParamSection),
    Population(PopulationAnalysisSection),
}

impl Display for ParamSections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamSections::General(section) => write!(f, "{section}"),
            ParamSections::BandStructure(section) => write!(f, "{section}"),
            ParamSections::BasisSet(section) => write!(f, "{section}"),
            ParamSections::ElectroMinimization(section) => write!(f, "{section}"),
            ParamSections::Electronic(section) => write!(f, "{section}"),
            ParamSections::GeomOpt(section) => write!(f, "{section}"),
            ParamSections::ExchangeCorrelation(section) => write!(f, "{section}"),
            ParamSections::Population(section) => write!(f, "{section}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CastepParams {
    sections: Vec<ParamSections>,
}

impl CastepParams {
    fn new(sections: Vec<ParamSections>) -> Self {
        Self { sections }
    }
    pub fn builder() -> CastepParamsBuilder {
        CastepParamsBuilder::default()
    }
}

#[cfg(feature = "template")]
pub mod template {
    use crate::CastepParams;

    use super::options::{
        BandStructureParamSection, BasisSetParamSection, ElectroMinParamSection,
        ElectronicParamSection, GeneralParamSection, GeomOptParamSection,
        PopulationAnalysisSection, PopulationParam, XcParamSection,
    };

    impl CastepParams {
        pub fn geom_opt(cutoff_energy: f64, spin_total: u32, use_edft: bool) -> Self {
            Self::builder()
                .with_general(GeneralParamSection::default())
                .with_basis_set(BasisSetParamSection::with_cutoff_energy(cutoff_energy))
                .with_electro_min(if use_edft {
                    ElectroMinParamSection::with_edft()
                } else {
                    ElectroMinParamSection::default()
                })
                .with_electronic(ElectronicParamSection::with_spin_and_perc_extra_bands(
                    spin_total, 72_f64,
                ))
                .with_geom_opt(GeomOptParamSection::default())
                .with_xc(XcParamSection::default())
                .with_population(PopulationAnalysisSection::default())
                .build()
        }
        pub fn band_structure(cutoff_energy: f64, spin_total: u32, use_edft: bool) -> Self {
            Self::builder()
                .with_general(GeneralParamSection::band_structure())
                .with_basis_set(BasisSetParamSection::with_cutoff_energy(cutoff_energy))
                .with_electro_min(if use_edft {
                    ElectroMinParamSection::with_edft()
                } else {
                    ElectroMinParamSection::default()
                })
                .with_electronic(ElectronicParamSection::with_spin_and_perc_extra_bands(
                    spin_total, 72_f64,
                ))
                .with_band_structure(BandStructureParamSection::default())
                .with_xc(XcParamSection::default())
                .with_population(PopulationAnalysisSection::new(vec![
                    PopulationParam::PDOS_CALCULATE_WEIGHTS(true),
                    PopulationParam::POPN_CALCULATE(false),
                ]))
                .build()
        }
    }
}

impl Display for CastepParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self
            .sections
            .iter()
            .map(|s| format!("{s}"))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{content}")
    }
}

#[derive(Debug, Clone, Default)]
pub struct CastepParamsBuilder {
    sections: Vec<ParamSections>,
}

impl CastepParamsBuilder {
    pub fn with_general(mut self, param: GeneralParamSection) -> Self {
        self.sections.push(ParamSections::General(param));
        self
    }
    pub fn with_band_structure(mut self, param: BandStructureParamSection) -> Self {
        self.sections.push(ParamSections::BandStructure(param));
        self
    }
    pub fn with_basis_set(mut self, param: BasisSetParamSection) -> Self {
        self.sections.push(ParamSections::BasisSet(param));
        self
    }
    pub fn with_electro_min(mut self, param: ElectroMinParamSection) -> Self {
        self.sections
            .push(ParamSections::ElectroMinimization(param));
        self
    }
    pub fn with_electronic(mut self, param: ElectronicParamSection) -> Self {
        self.sections.push(ParamSections::Electronic(param));
        self
    }
    pub fn with_geom_opt(mut self, param: GeomOptParamSection) -> Self {
        self.sections.push(ParamSections::GeomOpt(param));
        self
    }
    pub fn with_xc(mut self, param: XcParamSection) -> Self {
        self.sections
            .push(ParamSections::ExchangeCorrelation(param));
        self
    }
    pub fn with_population(mut self, param: PopulationAnalysisSection) -> Self {
        self.sections.push(ParamSections::Population(param));
        self
    }
    pub fn build(self) -> CastepParams {
        let Self { sections } = self;
        CastepParams::new(sections)
    }
}

#[cfg(test)]
mod test {
    use crate::cell_document::params::CastepParams;

    #[test]
    fn geom_opt_param() {
        println!("{}", CastepParams::geom_opt(380_f64, 2, false));
    }
    #[test]
    fn bs_param() {
        println!("{}", CastepParams::band_structure(380_f64, 2, false));
    }
}
