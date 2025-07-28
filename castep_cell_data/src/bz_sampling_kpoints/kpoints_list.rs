use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

use super::Kpoint;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
/// Represents the KPOINTS_LIST block.
///
/// Contains a list of k-points and their weights for Brillouin zone sampling.
/// # Format:
/// %BLOCK KPOINTS_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK KPOINTS_LIST
pub struct KpointsList {
    pub kpts: Vec<Kpoint>,
}

impl ToCell for KpointsList {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "KPOINTS_LIST",
            self.kpts
                .iter()
                .map(|kpt| kpt.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod test_kpoints_list {
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    use crate::bz_sampling_kpoints::kpoints_list::KpointsList;

    #[test]
    fn kpoints_list_serde() {
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFile {
            kpoints_list: KpointsList,
        }
        let block_kpt_str = r#"%BLOCK KPOINTS_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK KPOINTS_LIST
"#;
        let cell_file = dbg!(from_str::<CellFile>(block_kpt_str).unwrap());
        println!("{}", to_string(&cell_file.kpoints_list.to_cell()).unwrap());
    }
}
