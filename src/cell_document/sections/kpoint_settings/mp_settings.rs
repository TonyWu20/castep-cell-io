use std::fmt::Display;

use crate::{
    formatting::{BlockDisplay, FieldDisplay},
    InvLengthUnit,
};

use super::{
    KpointListBlock, KpointMPGrid, KpointMPOffset, KpointMPSpacing, KpointQuality, KpointTask,
};

impl Default for KpointListBlock {
    fn default() -> Self {
        Self {
            task: KpointTask::default(),
            kpoint_list: vec![[0_f64, 0_f64, 0_f64, 1_f64]],
        }
    }
}

impl Default for KpointMPSpacing {
    fn default() -> Self {
        let spacing = KpointQuality::Coarse.spacing_value(true);
        Self {
            task: KpointTask::default(),
            spacing,
            unit: InvLengthUnit::default(),
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

impl Display for KpointListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
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

impl Display for KpointMPGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl FieldDisplay for KpointMPSpacing {
    fn field_tag(&self) -> String {
        match self.task {
            KpointTask::SCF => "KPOINT_MP_SPACING".to_string(),
            KpointTask::Spectral => "BS_KPOINT_MP_SPACING".to_string(),
            KpointTask::Phonon => "PHONON_KPOINT_MP_SPACING".to_string(),
        }
    }

    fn value(&self) -> String {
        let unit = match self.unit {
            crate::InvLengthUnit::Ang => String::new(),
            _ => format!("{}", self.unit),
        };
        format!("{} {}", self.spacing, unit)
    }
}

impl Display for KpointMPSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl FieldDisplay for KpointMPOffset {
    fn field_tag(&self) -> String {
        "KPOINT_MP_OFFSET".to_string()
    }

    fn value(&self) -> String {
        let [i, j, k] = self.offset;
        format!("{i:20.16}{j:20.16}{k:20.16}")
    }
}

impl Display for KpointMPOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}
