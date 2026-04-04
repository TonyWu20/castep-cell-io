use crate::{
    Cell, CellValue,
    error::{CResult, Error},
    parser::parse_cell_file,
    query::{find_block, find_keyvalue, value_as_bool, value_as_f64, value_as_i32,
            value_as_string, value_as_u32},
};

// ── Core traits ──────────────────────────────────────────────────────────────

/// Leaf-level: parse a single `CellValue` into a Rust type.
///
/// Implemented by scalars, unit enums, keyword enums, and row structs.
pub trait FromCellValue: Sized {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self>;
}

/// Block-level: parse a block's rows into a struct.
///
/// `BLOCK_NAME` is the case-insensitive block name used for lookup.
pub trait FromBlock: Sized {
    const BLOCK_NAME: &'static str;

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self>;

    /// Provided: find the block in the token slice and parse it.
    fn from_cells(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::from_block_rows(find_block(tokens, Self::BLOCK_NAME)?)
    }
}

/// KeyValue-level: parse the value at a known key into a type.
///
/// `KEY_NAME` is the case-insensitive key used for lookup.
pub trait FromKeyValue: Sized {
    const KEY_NAME: &'static str;

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self>;

    /// Provided: returns `None` if the key is absent (optional fields).
    fn from_cells(tokens: &[Cell<'_>]) -> CResult<Option<Self>> {
        match find_keyvalue(tokens, Self::KEY_NAME) {
            Ok(v) => Self::from_cell_value_kv(v).map(Some),
            Err(Error::KeyNotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// File-level: assemble a top-level struct from the full token slice.
pub trait FromCellFile: Sized {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self>;
}

// ── Entry point ──────────────────────────────────────────────────────────────

/// Parse a `.cell` / `.param` file string and deserialize into `T`.
pub fn parse<T: FromCellFile>(input: &str) -> CResult<T> {
    let tokens =
        parse_cell_file(input).map_err(|errors| Error::Message(format!("{errors:?}")))?;
    T::from_cell_file(&tokens)
}

// ── Primitive FromCellValue impls ─────────────────────────────────────────────

impl FromCellValue for f64 {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        value_as_f64(value)
    }
}

impl FromCellValue for u32 {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        value_as_u32(value)
    }
}

impl FromCellValue for i32 {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        value_as_i32(value)
    }
}

impl FromCellValue for bool {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        value_as_bool(value)
    }
}

impl FromCellValue for String {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        value_as_string(value)
    }
}

/// `FromCellValue` for borrowed `&str` is intentionally not provided here
/// because the lifetime dependency makes it impractical as a blanket impl.
/// Use `value_as_str` directly when a `&str` is needed.
impl<T: FromCellValue, const N: usize> FromCellValue for [T; N] {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                if arr.len() < N {
                    return Err(Error::Message(format!(
                        "expected array of length {N}, got {}",
                        arr.len()
                    )));
                }
                // Collect into a Vec then convert to array
                let items: Vec<T> = arr
                    .iter()
                    .take(N)
                    .map(T::from_cell_value)
                    .collect::<CResult<Vec<T>>>()?;
                items.try_into().map_err(|_| {
                    Error::Message(format!("failed to convert Vec to [{N}] array"))
                })
            }
            other => Err(Error::UnexpectedType(
                format!("Array[{N}]"),
                format!("{other:?}"),
            )),
        }
    }
}
