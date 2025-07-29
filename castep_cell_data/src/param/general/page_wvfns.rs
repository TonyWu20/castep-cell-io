use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the paging of wavefunctions to disk in order to save memory.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// PAGE_WVFNS : 2
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PAGE_WVFNS")]
pub struct PageWvfns(pub i32);

impl ToCell for PageWvfns {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PAGE_WVFNS", CellValue::Int(self.0))
    }
}

impl ToCellValue for PageWvfns {
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
    fn test_page_wvfns_serde() {
        let page_wvfns_str = "PAGE_WVFNS : 2";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPageWvfns {
            page_wvfns: PageWvfns,
        }

        let cell_file_result: Result<CellFileWithPageWvfns, _> = from_str(page_wvfns_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.page_wvfns.0, 2);

        let page_wvfns_instance = PageWvfns(-1);
        let serialized_result = to_string(&page_wvfns_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PAGE_WVFNS (-1):\n{serialized_string}");
        assert!(serialized_string.contains("PAGE_WVFNS"));
        assert!(serialized_string.contains("-1"));

        assert_eq!(PageWvfns::default(), PageWvfns(0));
    }
}
