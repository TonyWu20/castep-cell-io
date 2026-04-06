use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::value_as_str};
use super::Species;

/// Represents a single parameter entry in SEDC_CUSTOM_PARAMS format.
/// Parameters are specified as `key:value` pairs (e.g., `C6:0.0`, `R0:1.6404`).
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SedcParameter {
    /// Parameter name (e.g., "C6", "R0", "alpha", "I", "Neff").
    pub key: String,
    /// Parameter value.
    pub value: f64,
}

impl SedcParameter {
    /// Parse a `key:value` string into a SedcParameter.
    fn from_str(s: &str) -> CResult<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::Message(
                format!("invalid SEDC parameter format: expected 'key:value', got '{s}'"),
            ));
        }
        let key = parts[0].trim().to_string();
        let value = parts[1]
            .trim()
            .parse::<f64>()
            .map_err(|_| Error::Message(format!("invalid float value: {}", parts[1])))?;
        Ok(SedcParameter { key, value })
    }
}

impl std::fmt::Display for SedcParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format with at least one decimal place to match CASTEP conventions
        if self.value.fract() == 0.0 && !self.value.is_infinite() && !self.value.is_nan() {
            write!(f, "{}:{:.1}", self.key, self.value)
        } else {
            write!(f, "{}:{}", self.key, self.value)
        }
    }
}

/// Represents a single entry within the SEDC_CUSTOM_PARAMS block.
/// Each entry specifies custom dispersion parameters for a species.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SedcCustomParamsEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The custom parameters for this species.
    #[builder(default)]
    pub params: Vec<SedcParameter>,
}

impl SedcCustomParamsEntry {
    /// Parse from a CellValue array row.
    /// Format: [species_str, param1_str, param2_str, ...]
    /// where each param_str is "key:value".
    fn from_array(arr: &[CellValue<'_>]) -> CResult<Self> {
        if arr.is_empty() {
            return Err(Error::Message(
                "SedcCustomParamsEntry requires at least a species".into(),
            ));
        }

        let species = Species::from_cell_value(&arr[0])?;
        let params = arr[1..]
            .iter()
            .map(|v| {
                let s = value_as_str(v)?;
                SedcParameter::from_str(s)
            })
            .collect::<CResult<Vec<_>>>()?;

        Ok(SedcCustomParamsEntry { species, params })
    }

    /// Convert to CellValue array.
    fn to_array(&self) -> CellValue<'_> {
        let mut arr = vec![self.species.to_cell_value()];
        arr.extend(
            self.params
                .iter()
                .map(|p| CellValue::String(p.to_string())),
        );
        CellValue::Array(arr)
    }
}

/// Represents the SEDC_CUSTOM_PARAMS block.
///
/// Defines customized parameters for semi-empirical dispersion corrections.
/// Format:
/// %BLOCK SEDC_CUSTOM_PARAMS
/// [UNITS energy_unit length_unit]
/// species1 param1:value1 param2:value2 ...
/// species2 param1:value1 param2:value2 ...
/// ...
/// %ENDBLOCK SEDC_CUSTOM_PARAMS
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SedcCustomParams {
    /// Optional units specification (e.g., "eV Angstrom").
    /// If None, defaults are used (eV and Å).
    pub units: Option<String>,
    /// The list of species and their custom parameters.
    #[builder(default)]
    pub entries: Vec<SedcCustomParamsEntry>,
}

impl FromBlock for SedcCustomParams {
    const BLOCK_NAME: &'static str = "SEDC_CUSTOM_PARAMS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                units: None,
                entries: Vec::new(),
            });
        }

        let (units, data_start) = if let CellValue::Array(arr) = &rows[0] {
            // Check if first row is a units line (single element or two elements that look like units)
            if arr.len() == 1 || (arr.len() == 2 && is_likely_units_row(arr)) {
                // Try to parse as units
                let units_str = arr
                    .iter()
                    .map(|v| value_as_str(v).map(|s| s.to_string()))
                    .collect::<CResult<Vec<_>>>()?
                    .join(" ");
                (Some(units_str), 1)
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

        let entries = rows[data_start..]
            .iter()
            .filter_map(|row| {
                if let CellValue::Array(arr) = row {
                    Some(SedcCustomParamsEntry::from_array(arr))
                } else {
                    None
                }
            })
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self { units, entries })
    }
}

/// Check if a row looks like a units specification.
/// Units rows typically have 1-2 string elements that are known unit keywords.
fn is_likely_units_row(arr: &[CellValue<'_>]) -> bool {
    if arr.is_empty() || arr.len() > 2 {
        return false;
    }
    // Try to extract strings; if any element is not a string, it's not a units row
    arr.iter().all(|v| matches!(v, CellValue::Str(_) | CellValue::String(_)))
}

impl ToCell for SedcCustomParams {
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();

        if let Some(ref u) = self.units {
            // Split units string and create array
            let unit_parts: Vec<CellValue> = u
                .split_whitespace()
                .map(|s| CellValue::String(s.to_string()))
                .collect();
            block_content.push(CellValue::Array(unit_parts));
        }

        block_content.extend(self.entries.iter().map(|entry| entry.to_array()));

        Cell::Block("SEDC_CUSTOM_PARAMS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_sedc_parameter_from_str() {
        let param = SedcParameter::from_str("C6:0.0").unwrap();
        assert_eq!(param.key, "C6");
        assert_eq!(param.value, 0.0);
    }

    #[test]
    fn test_sedc_parameter_from_str_with_spaces() {
        let param = SedcParameter::from_str(" R0 : 1.6404 ").unwrap();
        assert_eq!(param.key, "R0");
        assert_eq!(param.value, 1.6404);
    }

    #[test]
    fn test_sedc_parameter_from_str_invalid() {
        assert!(SedcParameter::from_str("C6").is_err());
        assert!(SedcParameter::from_str("C6:invalid").is_err());
    }

    #[test]
    fn test_sedc_parameter_to_string() {
        let param = SedcParameter {
            key: "C6".to_string(),
            value: 0.0,
        };
        assert_eq!(param.to_string(), "C6:0.0");
    }

    #[test]
    fn test_sedc_custom_params_entry_from_array() {
        let arr = vec![
            CellValue::Str("H"),
            CellValue::Str("C6:0.0"),
            CellValue::Str("R0:1.6404"),
        ];
        let entry = SedcCustomParamsEntry::from_array(&arr).unwrap();
        assert_eq!(entry.species, Species::Symbol("H".to_string()));
        assert_eq!(entry.params.len(), 2);
        assert_eq!(entry.params[0].key, "C6");
        assert_eq!(entry.params[0].value, 0.0);
        assert_eq!(entry.params[1].key, "R0");
        assert_eq!(entry.params[1].value, 1.6404);
    }

    #[test]
    fn test_sedc_custom_params_entry_empty() {
        let arr = vec![];
        assert!(SedcCustomParamsEntry::from_array(&arr).is_err());
    }

    #[test]
    fn test_sedc_custom_params_entry_to_array() {
        let entry = SedcCustomParamsEntry {
            species: Species::Symbol("H".to_string()),
            params: vec![
                SedcParameter {
                    key: "C6".to_string(),
                    value: 0.0,
                },
                SedcParameter {
                    key: "R0".to_string(),
                    value: 1.6404,
                },
            ],
        };
        let arr = entry.to_array();
        if let CellValue::Array(a) = arr {
            assert_eq!(a.len(), 3);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_sedc_custom_params_empty() {
        let result = SedcCustomParams::from_block_rows(&[]).unwrap();
        assert!(result.units.is_none());
        assert_eq!(result.entries.len(), 0);
    }

    #[test]
    fn test_sedc_custom_params_without_units() {
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("H"),
            CellValue::Str("C6:0.0"),
            CellValue::Str("R0:1.6404"),
        ])];
        let result = SedcCustomParams::from_block_rows(&rows).unwrap();
        assert!(result.units.is_none());
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].species, Species::Symbol("H".to_string()));
    }

    #[test]
    fn test_sedc_custom_params_with_units() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV"), CellValue::Str("Angstrom")]),
            CellValue::Array(vec![
                CellValue::Str("H"),
                CellValue::Str("C6:0.0"),
                CellValue::Str("R0:1.6404"),
            ]),
        ];
        let result = SedcCustomParams::from_block_rows(&rows).unwrap();
        assert!(result.units.is_some());
        assert_eq!(result.units.as_ref().unwrap(), "eV Angstrom");
        assert_eq!(result.entries.len(), 1);
    }

    #[test]
    fn test_sedc_custom_params_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV")]),
            CellValue::Array(vec![
                CellValue::Str("H"),
                CellValue::Str("C6:0.0"),
                CellValue::Str("R0:1.6404"),
            ]),
            CellValue::Array(vec![
                CellValue::Str("C"),
                CellValue::Str("C6:1.5"),
                CellValue::Str("R0:2.0"),
                CellValue::Str("alpha:0.5"),
            ]),
        ];
        let result = SedcCustomParams::from_block_rows(&rows).unwrap();
        assert_eq!(result.entries.len(), 2);
        assert_eq!(result.entries[0].params.len(), 2);
        assert_eq!(result.entries[1].params.len(), 3);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SedcCustomParams::BLOCK_NAME, "SEDC_CUSTOM_PARAMS");
    }

    #[test]
    fn test_sedc_custom_params_to_cell() {
        let params = SedcCustomParams {
            units: Some("eV Angstrom".to_string()),
            entries: vec![SedcCustomParamsEntry {
                species: Species::Symbol("H".to_string()),
                params: vec![
                    SedcParameter {
                        key: "C6".to_string(),
                        value: 0.0,
                    },
                    SedcParameter {
                        key: "R0".to_string(),
                        value: 1.6404,
                    },
                ],
            }],
        };
        let cell = params.to_cell();
        if let Cell::Block(name, content) = cell {
            assert_eq!(name, "SEDC_CUSTOM_PARAMS");
            assert_eq!(content.len(), 2); // units + 1 entry
        } else {
            panic!("Expected block");
        }
    }
}
