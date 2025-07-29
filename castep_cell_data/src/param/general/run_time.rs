use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the maximum run time for the job, in seconds.
///
/// Keyword type: Integer
///
/// Default: 0 (no time limit)
///
/// Example:
/// RUN_TIME : 360
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename = "RUN_TIME")]
pub struct RunTime(pub i32); // i32 to allow <= 0 values

impl ToCell for RunTime {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("RUN_TIME", CellValue::Int(self.0))
    }
}

impl ToCellValue for RunTime {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_run_time_serde() {
        let run_time_str = "RUN_TIME : 360";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithRunTime {
            run_time: RunTime,
        }

        let cell_file_result: Result<CellFileWithRunTime, _> = from_str(run_time_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.run_time.0, 360);

        let run_time_instance = RunTime(7200);
        let serialized_result = to_string(&run_time_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized RUN_TIME (7200):\n{serialized_string}");
        assert!(serialized_string.contains("RUN_TIME"));
        assert!(serialized_string.contains("7200"));

        assert_eq!(RunTime::default(), RunTime(0));
    }
}
