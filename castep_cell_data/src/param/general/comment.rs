use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Can contain a comment string, used to label the output.
///
/// Keyword type: String
///
/// Default: (blank)
///
/// Example:
/// COMMENT : This is a test run
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "COMMENT")]
#[serde(from = "CommentRepr")]
pub struct Comment(pub String);

impl ToCell for Comment {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("COMMENT", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Comment {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}

#[derive(Debug, Deserialize)]
struct CommentRepr(Vec<String>);

impl From<CommentRepr> for Comment {
    fn from(value: CommentRepr) -> Self {
        Self(value.0.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_comment_serde() {
        // 1. Test Deserialization
        let comment_str = "COMMENT : This is a test run with spaces";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithComment {
            comment: Comment,
        }

        let cell_file_result: Result<CellFileWithComment, _> = from_str(comment_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.comment.0, "This is a test run with spaces");

        // 2. Test Serialization using ToCell
        let comment_instance = Comment("Another test comment".to_string());
        let serialized_result = to_string(&comment_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized COMMENT (Another test comment):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("COMMENT"));
        assert!(serialized_string.contains("Another test comment"));
    }
}
