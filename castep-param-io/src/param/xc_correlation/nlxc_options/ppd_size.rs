use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct PPDSizeX(u32);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct PPDSizeY(u32);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct PPDSizeZ(u32);

macro_rules! ppd_size_impl {
    ($($x:ident),*) =>  {
        $(
            impl From<u32> for $x {
                fn from(value: u32) -> Self {
                    Self(value)
                }
            }

            impl Display for $x {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        )*
    }
}
ppd_size_impl!(PPDSizeX, PPDSizeY, PPDSizeZ);
impl KeywordDisplay for PPDSizeX {
    fn field(&self) -> String {
        "NLXC_PPD_SIZE_X".to_string()
    }
}
impl KeywordDisplay for PPDSizeY {
    fn field(&self) -> String {
        "NLXC_PPD_SIZE_Y".to_string()
    }
}
impl KeywordDisplay for PPDSizeZ {
    fn field(&self) -> String {
        "NLXC_PPD_SIZE_Z".to_string()
    }
}
