use crate::formatting::{BlockDisplay, FieldDisplay};

use super::{KpointListBlock, KpointMPGrid, KpointTask};

impl Default for KpointListBlock {
    fn default() -> Self {
        Self {
            task: KpointTask::SCF,
            kpoint_list: vec![[0_f64, 0_f64, 0_f64, 1_f64]],
        }
    }
}

impl BlockDisplay for KpointListBlock {
    fn block_tag(&self) -> String {
        match self.task {
            KpointTask::SCF => "KPOINT_LIST".to_string(),
            KpointTask::Spectral => "BS_KPOINT_LIST".to_string(),
            KpointTask::Phonon => "PHONON_KPOINT_LIST".to_string(),
        }
    }

    fn entries(&self) -> String {
        self.kpoint_list
            .iter()
            .map(|line| {
                let [x, y, z, weight] = line;
                format!("{:20.16}{:20.16}{:20.16}{:20.16}", x, y, z, weight)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl FieldDisplay for KpointMPGrid {
    fn field_tag(&self) -> String {
        match self.task {
            KpointTask::SCF => "KPOINT_MP_GRID".to_string(),
            KpointTask::Spectral => "BS_KPOINT_MP_GRID".to_string(),
            KpointTask::Phonon => "PHONON_KPOINT_MP_GRID".to_string(),
        }
    }

    fn value(&self) -> String {
        let [i, j, k] = self.grid;
        format!("{} {} {}", i, j, k)
    }
}
