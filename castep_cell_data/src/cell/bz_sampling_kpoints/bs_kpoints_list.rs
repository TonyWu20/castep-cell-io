use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

use super::Kpoint;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
/// Represents the KPOINTS_LIST block.
/// This data block contains a list of k-points at which the Brillouin zone
/// will be sampled during non-self consistent band structure calculations,
/// along with the associated weights. It has the same format as `KPOINTS_LIST`.
/// # Format:
/// %BLOCK BS_KPOINTS_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK BS_KPOINTS_LIST
pub struct BSKpointList {
    pub kpts: Vec<Kpoint>,
}

impl ToCell for BSKpointList {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "BS_KPOINT_LIST",
            self.kpts
                .iter()
                .map(|kpt| kpt.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod test_bs_kpoint_list {
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    use crate::cell::bz_sampling_kpoints::bs_kpoints_list::BSKpointList;

    #[test]
    fn bs_kpoint_list_serde() {
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFile {
            #[serde(alias = "SPECTRAL_KPOINT_LIST")]
            bs_kpoint_list: BSKpointList,
        }
        let block_kpt_str = r#"%BLOCK BS_KPOINT_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK BS_KPOINT_LIST
"#;
        let cell_file = dbg!(from_str::<CellFile>(block_kpt_str).unwrap());
        println!(
            "{}",
            to_string(&cell_file.bs_kpoint_list.to_cell()).unwrap()
        );
        let block_kpt_str = r#"%BLOCK SPECTRAL_KPOINT_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK SPECTRAL_KPOINT_LIST
"#;
        let cell_file = dbg!(from_str::<CellFile>(block_kpt_str).unwrap());
        println!(
            "{}",
            to_string(&cell_file.bs_kpoint_list.to_cell()).unwrap()
        );
    }
}
