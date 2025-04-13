use std::str::FromStr;

use castep_param_derive::KeywordDisplay;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{ConsumeKVPairs, ParamParser, Rule};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
)]
#[keyword_display(specified_fields = true)]
#[pest_ast(rule(Rule::backup_setting))]
pub enum BackUpSetting {
    // This keyword specifies the number of geometry optimization or molecular
    // dynamics iterations between updates of the backup restart files.
    #[keyword_display(field = "NUM_BACKUP_ITER")]
    NumBackupIter(
        #[pest_ast(inner(
            rule(Rule::num_backup_iter),
            with(span_into_num_backup_iter),
            with(Result::unwrap),
        ))]
        u64,
    ),
    /// This keyword specifies the interval, in seconds, between updates of the
    /// backup restart files. This keyword is applicable for geometry optimization,
    /// molecular dynamics, phonon or transition state search runs.
    /// A value which is less than or equal to zero indicates that no updates will be performed.
    #[keyword_display(field = "BACKUP_INTERVAL")]
    BackupInterval(
        #[pest_ast(inner(
            rule(Rule::backup_interval),
            with(span_into_backup_interval),
            with(Result::unwrap)
        ))]
        i64,
    ),
}

impl<'a> ConsumeKVPairs<'a> for BackUpSetting {
    type Item = BackUpSetting;

    fn find_from_pairs(pairs: &'a [crate::parser::ParamItems<'a>]) -> Option<Self::Item> {
        let num_backup_iter_id = pairs
            .iter()
            .position(|p| p.keyword().to_lowercase() == "num_backup_iter");
        let backup_interval_id = pairs
            .iter()
            .position(|p| p.keyword().to_lowercase() == "backup_interval");
        match (num_backup_iter_id, backup_interval_id) {
            (None, None) => None,
            (None, Some(n)) | (Some(n), None) => {
                let to_use = pairs[n];
                let kvpair = to_use.to_string();
                let mut parse = ParamParser::parse(Rule::backup_setting, &kvpair).unwrap();
                Some(BackUpSetting::from_pest(&mut parse).unwrap())
            }
            (Some(n), Some(b)) => {
                let to_use = if n > b { pairs[b] } else { pairs[n] };
                let kvpair = to_use.to_string();
                let mut parse = ParamParser::parse(Rule::backup_setting, &kvpair).unwrap();
                Some(BackUpSetting::from_pest(&mut parse).unwrap())
            }
        }
    }
}

fn span_into_num_backup_iter(span: Span) -> Result<u64, <u64 as FromStr>::Err> {
    let to_lower = span.as_str().to_lowercase();
    let binding = to_lower.replace("num_backup_iter", "").replace(":", "");
    let trim = binding.trim();
    trim.parse::<u64>()
}

fn span_into_backup_interval(span: Span) -> Result<i64, <i64 as FromStr>::Err> {
    let to_lower = span.as_str().to_lowercase();
    to_lower
        .replace("backup_interval", "")
        .replace(":", "")
        .trim()
        .parse::<i64>()
}

impl Default for BackUpSetting {
    fn default() -> Self {
        Self::NumBackupIter(5)
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{ParamParser, Rule};

    use super::BackUpSetting;

    #[test]
    fn backup_setting() {
        let input = "Backup_interval : 5";
        let mut parse = ParamParser::parse(Rule::backup_setting, input).unwrap();
        dbg!(&parse);
        let num_backup_iter = BackUpSetting::from_pest(&mut parse).unwrap();
        dbg!(num_backup_iter);
        let input = "Num_backup_Iter : 1";
        let mut parse = ParamParser::parse(Rule::backup_setting, input).unwrap();
        dbg!(&parse);
        let num_backup_iter = BackUpSetting::from_pest(&mut parse).unwrap();
        dbg!(num_backup_iter);
    }
}
