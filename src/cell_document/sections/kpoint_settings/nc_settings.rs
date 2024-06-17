use std::fmt::Display;

use crate::{
    formatting::{BlockDisplay, FieldDisplay},
    InvLengthUnit,
};

use super::{BSKpointPath, BSKpointPathSpacing};

impl Default for BSKpointPathSpacing {
    fn default() -> Self {
        Self {
            unit: InvLengthUnit::default(),
            spacing: 0.1,
        }
    }
}

impl BlockDisplay for BSKpointPath {
    fn block_tag(&self) -> String {
        "BS_KPOINT_PATH".to_string()
    }

    fn entries(&self) -> String {
        self.paths
            .iter()
            .map(|p| {
                let [i, j, k] = p;
                format!("{i:20.16}{j:20.16}{k:20.16}")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for BSKpointPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl FieldDisplay for BSKpointPathSpacing {
    fn field_tag(&self) -> String {
        "BS_KPOINT_PATH_SPACING".to_string()
    }

    fn value(&self) -> String {
        let unit = match self.unit {
            InvLengthUnit::Ang => String::new(),
            _ => format!(" {}", self.unit),
        };
        format!("{}{}", self.spacing, unit)
    }
}

impl Display for BSKpointPathSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}
