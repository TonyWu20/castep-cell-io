use crate::param::{BackUpSetting, CastepTask, General, GeneralBuilder};
use crate::parser::ConsumeKVPairs;

use super::KVPair;

#[allow(dead_code)]
impl General {
    pub fn build_from_parsed<'a>(parsed_pairs: &'a [KVPair<'a>]) -> Self {
        GeneralBuilder::default()
            .task(CastepTask::find_from_pairs(parsed_pairs).unwrap_or_default())
            .backup(BackUpSetting::find_from_pairs(parsed_pairs).unwrap_or_default())
            .build()
            .expect("Error in given fields")
    }
}
