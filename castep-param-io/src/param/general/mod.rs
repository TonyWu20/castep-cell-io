use super::KeywordDisplay;
use castep_param_derive::{ParamDisplay, StructBuildFromPairs};
use serde::{Deserialize, Serialize};

mod backup_setting;
mod calculate_props;
mod castep_task;
mod charge_unit;
mod checkpoint;
mod comment;
mod continuation;
mod data_distribution;
mod iprint;
mod opt_strategy;
mod page_wvfns;
mod print_clock;
mod print_memory_usage;
mod rand_seed;
mod run_time;
mod stop;
mod write_checkpoint;
mod write_props;

pub use backup_setting::BackUpSetting;
pub use calculate_props::*;
pub use castep_task::CastepTask;
pub use charge_unit::ChargeUnit;
pub use checkpoint::Checkpoint;
pub use comment::Comment;
#[allow(unused_imports)]
pub use continuation::{Continuation, ContinueReuse, Reuse};
pub use data_distribution::DataDistribution;
pub use derive_builder::Builder;
pub use iprint::IPrint;
pub use opt_strategy::OptStrategy;
pub use page_wvfns::PageWvfns;
pub use print_clock::PrintClock;
pub use print_memory_usage::PrintMemoryUsage;
pub use rand_seed::RandSeed;
pub use run_time::RunTime;
pub use stop::Stop;
pub use write_checkpoint::*;
pub use write_props::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    Serialize,
    Deserialize,
    Builder,
    ParamDisplay,
    StructBuildFromPairs,
)]
#[builder(setter(into, strip_option), default)]
/// Keywords that belong to general category.
pub struct General {
    pub task: Option<CastepTask>, // Default: CastepTask::SinglePoint
    #[param_display(use_ref = true)]
    pub comment: Option<Comment>, // Blank
    #[param_display(use_ref = true)]
    pub continuation_reuse: Option<ContinueReuse>, // Null
    pub backup: Option<BackUpSetting>,
    pub charge_unit: Option<ChargeUnit>, // E
    #[param_display(use_ref = true)]
    pub checkpoint: Option<Checkpoint>, // seedname.check
    pub data_distribution: Option<DataDistribution>, // Default
    pub iprint: Option<IPrint>,          // 1
    pub opt_strategy: Option<OptStrategy>, // Default
    pub page_wvfns: Option<PageWvfns>,   // 0
    pub print_clock: Option<PrintClock>, // true
    pub print_memory_usage: Option<PrintMemoryUsage>, // true
    pub rand_seed: Option<RandSeed>,     // 0
    pub run_time: Option<RunTime>,       // 0
    pub stop: Option<Stop>,              // Example: STOP
    pub write_checkpoint: Option<WriteCheckpoint>,
    #[param_display(display=to_string())]
    pub calculate_props: Option<CalculateProperties>, // Default to all false
    #[param_display(display=to_string())]
    pub write_props: Option<WriteProperties>, // Default to all false
}

#[cfg(test)]
mod test {

    use crate::param::{
        general::{
            calculate_props::CalculatePropertiesBuilder, write_props::WritePropertiesBuilder,
            CastepTask, GeneralBuilder, OptStrategy,
        },
        Continuation,
    };

    use super::General;

    #[test]
    fn general_keywords() {
        let g = General::default();
        let gb = GeneralBuilder::default()
            .opt_strategy(OptStrategy::default())
            .task(CastepTask::GeometryOptimization)
            .continuation_reuse(Continuation::Default)
            .write_props(
                WritePropertiesBuilder::default()
                    .formatted_elf(true)
                    .formatted_density(true)
                    .build()
                    .unwrap(),
            )
            .calculate_props(
                CalculatePropertiesBuilder::default()
                    .stress(false)
                    .densdiff(false)
                    .elf(false)
                    .hirshfeld(false)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        assert!(g.to_string().is_empty());
        println!("{gb}");
    }
}
