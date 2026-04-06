use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64};

/// Represents a single symmetry operation, consisting of a 3x3 rotation matrix and a 3-element translation vector.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SymmetryOp {
    /// The 3x3 rotation matrix (stored as 3 rows of 3 elements each).
    pub rotation: [[f64; 3]; 3],
    /// The 3-element translation vector.
    pub translation: [f64; 3],
}

/// Represents the SYMMETRY_OPS block.
///
/// Contains a list of symmetry operations under which the unit cell is invariant.
/// Each operation consists of a 3x3 rotation matrix and a 3-element translation vector.
/// Format:
/// %BLOCK SYMMETRY_OPS
/// R11 R21 R31  <- Row 1 of rotation matrix for op 1
/// R12 R22 R32  <- Row 2 of rotation matrix for op 1
/// R13 R23 R33  <- Row 3 of rotation matrix for op 1
/// T1  T2  T3   <- Translation vector for op 1
/// R11 R21 R31  <- Row 1 of rotation matrix for op 2
/// ...          <- And so on
/// %ENDBLOCK SYMMETRY_OPS
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SymmetryOps {
    /// The list of symmetry operations.
    #[builder(default)]
    pub ops: Vec<SymmetryOp>,
}

impl FromBlock for SymmetryOps {
    const BLOCK_NAME: &'static str = "SYMMETRY_OPS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let ops = rows
            .chunks_exact(4)
            .map(|chunk| {
                let rotation = [
                    parse_row(&chunk[0])?,
                    parse_row(&chunk[1])?,
                    parse_row(&chunk[2])?,
                ];
                let translation = parse_row(&chunk[3])?;
                Ok(SymmetryOp { rotation, translation })
            })
            .collect::<CResult<Vec<_>>>()?;
        Ok(SymmetryOps { ops })
    }
}

fn parse_row(value: &CellValue<'_>) -> CResult<[f64; 3]> {
    match value {
        CellValue::Array(arr) if arr.len() == 3 => {
            Ok([
                value_as_f64(&arr[0])?,
                value_as_f64(&arr[1])?,
                value_as_f64(&arr[2])?,
            ])
        }
        _ => Err(castep_cell_fmt::Error::Message(
            "Row must be an array of 3 floats".into(),
        )),
    }
}

impl ToCell for SymmetryOps {
    fn to_cell(&self) -> Cell<'_> {
        let block_content = self
            .ops
            .iter()
            .flat_map(|op| {
                op.rotation
                    .iter()
                    .map(|row| {
                        CellValue::Array(row.iter().map(|&val| CellValue::Float(val)).collect())
                    })
                    .chain(std::iter::once(CellValue::Array(
                        op.translation
                            .iter()
                            .map(|&val| CellValue::Float(val))
                            .collect(),
                    )))
            })
            .collect::<Vec<CellValue>>();

        Cell::Block("SYMMETRY_OPS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_symmetry_ops_empty() {
        let result = SymmetryOps::from_block_rows(&[]).unwrap();
        assert_eq!(result.ops.len(), 0);
    }

    #[test]
    fn test_symmetry_ops_single_operation() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(1.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(1.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = SymmetryOps::from_block_rows(&rows).unwrap();
        assert_eq!(result.ops.len(), 1);
        assert_eq!(result.ops[0].rotation[0], [1.0, 0.0, 0.0]);
        assert_eq!(result.ops[0].translation, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_symmetry_ops_multiple_operations() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(1.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(1.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(-1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(-1.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(1.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = SymmetryOps::from_block_rows(&rows).unwrap();
        assert_eq!(result.ops.len(), 2);
    }

    #[test]
    fn test_symmetry_ops_incomplete_operation() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(1.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = SymmetryOps::from_block_rows(&rows).unwrap();
        assert_eq!(result.ops.len(), 0);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SymmetryOps::BLOCK_NAME, "SYMMETRY_OPS");
    }

    // Builder tests
    #[test]
    fn test_symmetry_op_builder_basic() {
        let op = SymmetryOp::builder()
            .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.0, 0.0, 0.0])
            .build();

        assert_eq!(op.rotation[0], [1.0, 0.0, 0.0]);
        assert_eq!(op.rotation[1], [0.0, 1.0, 0.0]);
        assert_eq!(op.rotation[2], [0.0, 0.0, 1.0]);
        assert_eq!(op.translation, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_symmetry_ops_builder_empty() {
        let ops = SymmetryOps::builder().build();
        assert_eq!(ops.ops.len(), 0);
    }

    #[test]
    fn test_symmetry_ops_builder_single_push() {
        let op = SymmetryOp::builder()
            .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.0, 0.0, 0.0])
            .build();

        let mut ops = SymmetryOps::builder().build();
        ops.ops.push(op.clone());

        assert_eq!(ops.ops.len(), 1);
        assert_eq!(ops.ops[0].rotation[0], [1.0, 0.0, 0.0]);
        assert_eq!(ops.ops[0], op);
    }

    #[test]
    fn test_symmetry_ops_builder_multiple_push() {
        let op1 = SymmetryOp::builder()
            .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.0, 0.0, 0.0])
            .build();

        let op2 = SymmetryOp::builder()
            .rotation([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.5, 0.5, 0.0])
            .build();

        let mut ops = SymmetryOps::builder().build();
        ops.ops.push(op1.clone());
        ops.ops.push(op2.clone());

        assert_eq!(ops.ops.len(), 2);
        assert_eq!(ops.ops[0].rotation[0], [1.0, 0.0, 0.0]);
        assert_eq!(ops.ops[1].rotation[0], [-1.0, 0.0, 0.0]);
        assert_eq!(ops.ops[1].translation, [0.5, 0.5, 0.0]);
    }

    #[test]
    fn test_symmetry_ops_builder_set_vec() {
        let op1 = SymmetryOp::builder()
            .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.0, 0.0, 0.0])
            .build();

        let op2 = SymmetryOp::builder()
            .rotation([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.5, 0.5, 0.0])
            .build();

        let ops = SymmetryOps::builder()
            .ops(vec![op1.clone(), op2.clone()])
            .build();

        assert_eq!(ops.ops.len(), 2);
        assert_eq!(ops.ops[0].rotation[0], [1.0, 0.0, 0.0]);
        assert_eq!(ops.ops[1].rotation[0], [-1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_symmetry_ops_builder_method_chaining() {
        let op1 = SymmetryOp::builder()
            .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.0, 0.0, 0.0])
            .build();

        let op2 = SymmetryOp::builder()
            .rotation([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.5, 0.5, 0.0])
            .build();

        let op3 = SymmetryOp::builder()
            .rotation([[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]])
            .translation([0.25, 0.25, 0.0])
            .build();

        let ops = SymmetryOps::builder()
            .ops(vec![op1.clone(), op2.clone(), op3.clone()])
            .build();

        assert_eq!(ops.ops.len(), 3);
        assert_eq!(ops.ops[0].rotation[0], [1.0, 0.0, 0.0]);
        assert_eq!(ops.ops[1].rotation[0], [-1.0, 0.0, 0.0]);
        assert_eq!(ops.ops[2].rotation[0], [0.0, 1.0, 0.0]);
        assert_eq!(ops.ops[2].translation, [0.25, 0.25, 0.0]);
    }
}

