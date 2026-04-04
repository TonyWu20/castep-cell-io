use crate::{
    Cell, CellValue,
    error::{CResult, Error},
};

/// Find a block by name (case-insensitive). Returns its rows slice on success.
pub fn find_block<'a>(tokens: &'a [Cell<'a>], name: &str) -> CResult<&'a [CellValue<'a>]> {
    for token in tokens {
        if let Cell::Block(blk_name, rows) = token {
            if blk_name.eq_ignore_ascii_case(name) {
                return Ok(rows.as_slice());
            }
        }
    }
    Err(Error::KeyNotFound(name.to_string()))
}

/// Find a key-value entry by key (case-insensitive). Returns reference to its CellValue.
pub fn find_keyvalue<'a>(tokens: &'a [Cell<'a>], key: &str) -> CResult<&'a CellValue<'a>> {
    for token in tokens {
        if let Cell::KeyValue(k, v) = token {
            if k.eq_ignore_ascii_case(key) {
                return Ok(v);
            }
        }
    }
    Err(Error::KeyNotFound(key.to_string()))
}

/// Check if a Flag entry with the given name exists (case-insensitive).
pub fn has_flag(tokens: &[Cell<'_>], name: &str) -> bool {
    tokens.iter().any(|token| {
        if let Cell::Flag(f) = token {
            f.eq_ignore_ascii_case(name)
        } else {
            false
        }
    })
}

/// Extract an f64 from a scalar CellValue.
pub fn value_as_f64(v: &CellValue<'_>) -> CResult<f64> {
    match v {
        CellValue::Float(f) => Ok(*f),
        CellValue::UInt(u) => Ok(*u as f64),
        CellValue::Int(i) => Ok(*i as f64),
        other => Err(Error::UnexpectedType("f64".into(), format!("{other:?}"))),
    }
}

/// Extract a u32 from a scalar CellValue.
pub fn value_as_u32(v: &CellValue<'_>) -> CResult<u32> {
    match v {
        CellValue::UInt(u) => Ok(*u),
        other => Err(Error::UnexpectedType("u32".into(), format!("{other:?}"))),
    }
}

/// Extract an i32 from a scalar CellValue.
pub fn value_as_i32(v: &CellValue<'_>) -> CResult<i32> {
    match v {
        CellValue::Int(i) => Ok(*i),
        CellValue::UInt(u) => i32::try_from(*u).map_err(Error::TryFromInt),
        other => Err(Error::UnexpectedType("i32".into(), format!("{other:?}"))),
    }
}

/// Extract a bool from a scalar CellValue.
pub fn value_as_bool(v: &CellValue<'_>) -> CResult<bool> {
    match v {
        CellValue::Bool(b) => Ok(*b),
        other => Err(Error::UnexpectedType("bool".into(), format!("{other:?}"))),
    }
}

/// Extract a &str from a CellValue::Str.
pub fn value_as_str<'a>(v: &'a CellValue<'a>) -> CResult<&'a str> {
    match v {
        CellValue::Str(s) => Ok(s),
        other => Err(Error::UnexpectedType("&str".into(), format!("{other:?}"))),
    }
}

/// Extract a String from a CellValue::Str or CellValue::String.
pub fn value_as_string(v: &CellValue<'_>) -> CResult<String> {
    match v {
        CellValue::Str(s) => Ok(s.to_string()),
        CellValue::String(s) => Ok(s.clone()),
        other => Err(Error::UnexpectedType("String".into(), format!("{other:?}"))),
    }
}

/// Extract exactly N f64 values from a CellValue::Array row.
pub fn row_as_f64_n<const N: usize>(v: &CellValue<'_>) -> CResult<[f64; N]> {
    match v {
        CellValue::Array(arr) => {
            if arr.len() < N {
                return Err(Error::Message(format!(
                    "expected array of length {N}, got {}",
                    arr.len()
                )));
            }
            let mut out = [0.0f64; N];
            for (i, val) in arr.iter().take(N).enumerate() {
                out[i] = value_as_f64(val)?;
            }
            Ok(out)
        }
        other => Err(Error::UnexpectedType(
            "Array".into(),
            format!("{other:?}"),
        )),
    }
}

#[cfg(test)]
mod query_test {
    use crate::{CellValue, parse_cell_file};
    use super::{find_block, find_keyvalue, has_flag, row_as_f64_n, value_as_bool, value_as_f64};

    const MIXED_CASE: &str = r#"
LATTICE_CART : dummy
FIX_COM : true
SYMMETRY_GENERATE

%BLOCK Positions_Frac
   O  0.1  0.2  0.3
%ENDBLOCK Positions_Frac
"#;

    #[test]
    fn case_insensitive_find_keyvalue() {
        let tokens = parse_cell_file(MIXED_CASE).unwrap();
        // Lookup with uppercase, lowercase, mixed — all must succeed
        assert!(find_keyvalue(&tokens, "lattice_cart").is_ok());
        assert!(find_keyvalue(&tokens, "LATTICE_CART").is_ok());
        assert!(find_keyvalue(&tokens, "Lattice_Cart").is_ok());
        // Bool value extraction
        let fix_com = find_keyvalue(&tokens, "fix_com").unwrap();
        assert_eq!(value_as_bool(fix_com).unwrap(), true);
    }

    #[test]
    fn case_insensitive_find_block() {
        let tokens = parse_cell_file(MIXED_CASE).unwrap();
        // Block lookup with various cases must succeed
        assert!(find_block(&tokens, "positions_frac").is_ok());
        assert!(find_block(&tokens, "POSITIONS_FRAC").is_ok());
        assert!(find_block(&tokens, "Positions_Frac").is_ok());
        // Non-existent block returns Err
        assert!(find_block(&tokens, "no_such_block").is_err());
    }

    #[test]
    fn case_insensitive_has_flag() {
        let tokens = parse_cell_file(MIXED_CASE).unwrap();
        assert!(has_flag(&tokens, "symmetry_generate"));
        assert!(has_flag(&tokens, "SYMMETRY_GENERATE"));
        assert!(!has_flag(&tokens, "no_such_flag"));
    }

    #[test]
    fn row_as_f64_n_correct() {
        let row = CellValue::Array(vec![
            CellValue::Float(1.0),
            CellValue::Float(2.0),
            CellValue::Float(3.0),
        ]);
        let arr: [f64; 3] = row_as_f64_n(&row).unwrap();
        assert_eq!(arr, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn row_as_f64_n_too_short() {
        let row = CellValue::Array(vec![CellValue::Float(1.0)]);
        let result = row_as_f64_n::<3>(&row);
        assert!(result.is_err());
    }

    #[test]
    fn value_as_f64_conversions() {
        assert_eq!(value_as_f64(&CellValue::Float(3.14)).unwrap(), 3.14);
        assert_eq!(value_as_f64(&CellValue::UInt(42)).unwrap(), 42.0);
        assert_eq!(value_as_f64(&CellValue::Int(-5)).unwrap(), -5.0);
        assert!(value_as_f64(&CellValue::Bool(true)).is_err());
    }
}
