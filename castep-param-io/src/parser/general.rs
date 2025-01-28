use crate::param::{BackUpSetting, CastepTask, Comment, ContinueReuse, General, OptStrategy};
use crate::parser::ConsumeKVPairs;

use super::{FromParamFile, KVPair, ParamFile};

#[allow(dead_code)]
impl FromParamFile for General {
    type Item = General;
    fn build_from_file(file: &ParamFile) -> Self {
        let parsed_pairs = file.pairs();
        General {
            stop: file.stop(),
            ..General::build_from_parsed(parsed_pairs)
        }
    }

    fn build_from_parsed(parsed_pairs: &[KVPair<'_>]) -> Self {
        General {
            task: CastepTask::find_from_pairs(parsed_pairs),
            backup: BackUpSetting::find_from_pairs(parsed_pairs),
            comment: Comment::find_from_pairs(parsed_pairs),
            continuation_reuse: ContinueReuse::find_from_pairs(parsed_pairs),
            opt_strategy: OptStrategy::find_from_pairs(parsed_pairs),
            ..General::default()
        }
    }
}
