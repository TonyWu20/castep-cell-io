use crate::parser::span_into_str;
use castep_cell_serde::{Cell, CellValue, ToCell};
use castep_param_derive::{BuildFromPairs, KeywordDisplay, ParamEnumFromStr};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::Rule;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
    ParamEnumFromStr,
)]
#[keyword_display(field = "TASK")]
#[non_exhaustive]
#[pest_rule(rule=Rule::task, keyword="TASK")]
#[pest_ast(rule(Rule::task))]
#[serde(rename = "TASK")]
pub enum CastepTask {
    #[pest_ast(inner(
        rule(Rule::tasks),
        with(span_into_str),
        with(CastepTask::from_str),
        with(Option::unwrap)
    ))]
    BandStructure, // calculates band structure properties.
    GeometryOptimization, // searches for a minimum energy structure.
    #[default]
    SinglePoint, // performs a single-point energy calculation.
    //  TODO: Future
    MolecularDynamics,      // performs a molecular dynamics calculation.
    Optics,                 // calculates optical properties.
    Phonon, // performs a linear response calculation to determine phonon frequencies and eigenvectors.
    Efield, // performs an electric field linear response calculation to determine dielectric permittivity and polarizability.
    PhononEfield, // performs a linear response calculation to determine phonon frequencies and eigenvectors, and an electric field linear response calculation to determine dielectric permittivity and polarizability.
    TransitionStateSearch, // performs a transition-state search.
    MagRes,       // performs an NMR calculation.
    Elnes,        // performs core level spectroscopy calculation.
    ElectronicSpectroscopy, // performs electronic spectroscopy calculation.
    Autosolvation, // performs a free energy of solvation calculation.
}

impl CastepTask {
    pub fn keyword(&self) -> &str {
        "TASK"
    }
    pub fn value(&self) -> &str {
        match self {
            CastepTask::BandStructure => "BandStructure",
            CastepTask::GeometryOptimization => "GeometryOptimization",
            CastepTask::SinglePoint => "SinglePoint",
            CastepTask::MolecularDynamics => todo!(),
            CastepTask::Optics => todo!(),
            CastepTask::Phonon => todo!(),
            CastepTask::Efield => todo!(),
            CastepTask::PhononEfield => todo!(),
            CastepTask::TransitionStateSearch => todo!(),
            CastepTask::MagRes => todo!(),
            CastepTask::Elnes => todo!(),
            CastepTask::ElectronicSpectroscopy => todo!(),
            CastepTask::Autosolvation => todo!(),
        }
    }
}

impl ToCell for CastepTask {
    fn to_cell(&self) -> castep_cell_serde::Cell {
        let value = match self {
            CastepTask::BandStructure => "BandStructure",
            CastepTask::GeometryOptimization => "GeometryOptimization",
            CastepTask::SinglePoint => "SinglePoint",
            CastepTask::MolecularDynamics => todo!(),
            CastepTask::Optics => todo!(),
            CastepTask::Phonon => todo!(),
            CastepTask::Efield => todo!(),
            CastepTask::PhononEfield => todo!(),
            CastepTask::TransitionStateSearch => todo!(),
            CastepTask::MagRes => todo!(),
            CastepTask::Elnes => todo!(),
            CastepTask::ElectronicSpectroscopy => todo!(),
            CastepTask::Autosolvation => todo!(),
        };
        Cell::KeyValue("TASK", CellValue::Str(value))
    }
}

#[cfg(test)]
mod test {
    use castep_cell_serde::{from_str, to_string, ToCell};
    use from_pest::FromPest;
    use pest::Parser;
    use serde::{Deserialize, Serialize};

    use crate::{
        param::KeywordDisplay,
        parser::{ParamParser, Rule},
    };

    use super::CastepTask;

    #[test]
    fn castep_task() {
        let task = CastepTask::default();
        assert_eq!("TASK : SinglePoint", task.output());
        let bandstr = CastepTask::BandStructure;
        assert_eq!("TASK : BandStructure", bandstr.output());
        let geom_opt = CastepTask::GeometryOptimization;
        assert_eq!("TASK : GeometryOptimization", geom_opt.output());
        let magres = CastepTask::MagRes;
        let binding = magres.output();
        let mut parse = ParamParser::parse(Rule::task, &binding).unwrap();
        dbg!(&parse);
        // parse.into_iter().for_each(|pair| {
        //     let span = pair.as_span();
        //     let mut inner = pair.clone().into_inner();
        //     let inner = &mut inner;
        //     dbg!(inner.next());
        // });
        let parsed_task = CastepTask::from_pest(&mut parse).unwrap();
        #[derive(Deserialize, Serialize, Debug)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct Param {
            task: CastepTask,
        }
        let geom_opt = from_str::<CastepTask>("TASK : SinglePoint").unwrap();
        dbg!(&geom_opt);
        println!("{}", to_string(&geom_opt.to_cell()).unwrap());
    }
}
