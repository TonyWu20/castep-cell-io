use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::value_as_f64};
use super::XcFunctional;
use std::collections::HashMap;

/// Represents a single functional entry in the XC_DEFINITION block.
#[derive(Debug, Clone, PartialEq)]
pub struct XcFunctionalEntry {
    /// The exchange-correlation functional.
    pub functional: XcFunctional,
    /// The weight of this functional in the mixture.
    pub weight: f64,
}

impl FromCellValue for XcFunctionalEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(XcFunctionalEntry {
                    functional: XcFunctional::from_cell_value(&arr[0])?,
                    weight: value_as_f64(&arr[1])?,
                })
            }
            _ => Err(Error::Message(
                "XcFunctionalEntry must be an array of [functional, weight]".into(),
            )),
        }
    }
}

impl ToCellValue for XcFunctionalEntry {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            self.functional.to_cell_value(),
            CellValue::Float(self.weight),
        ])
    }
}

/// Represents the XC_DEFINITION block.
///
/// Allows construction of a bespoke exchange-correlation potential by mixing
/// contributions from various local and nonlocal functional forms.
///
/// Format:
/// %BLOCK XC_DEFINITION
/// <xc-functional> weight
/// [NLXC_SCREENING_LENGTH  length]
/// [NLXC_SCREENING_FUNCTION function]
/// [NLXC_PPD_INT  ON/OFF]
/// [NLXC_DIVERGENCE_CORR ON/OFF]
/// %ENDBLOCK XC_DEFINITION
#[derive(Debug, Clone, PartialEq)]
pub struct XcDefinition {
    /// List of functionals and their weights.
    pub functionals: Vec<XcFunctionalEntry>,
    /// Optional parameters for nonlocal exchange-correlation calculations.
    pub optional_params: HashMap<String, String>,
}

impl FromBlock for XcDefinition {
    const BLOCK_NAME: &'static str = "XC_DEFINITION";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("XC_DEFINITION block is empty".into()));
        }

        let mut functionals = Vec::new();
        let mut optional_params = HashMap::new();

        for row in rows {
            match row {
                CellValue::Array(arr) if arr.len() == 2 => {
                    // Try to parse as functional+weight pair first
                    if let Ok(entry) = XcFunctionalEntry::from_cell_value(row) {
                        functionals.push(entry);
                    } else {
                        // Parse as optional parameter (key-value pair)
                        let key_str = match &arr[0] {
                            CellValue::Str(s) => s.to_string(),
                            CellValue::String(s) => s.clone(),
                            _ => continue,
                        };
                        let val_str = match &arr[1] {
                            CellValue::Str(s) => s.to_string(),
                            CellValue::String(s) => s.clone(),
                            CellValue::Float(f) => f.to_string(),
                            CellValue::Bool(b) => if *b { "ON" } else { "OFF" }.to_string(),
                            _ => continue,
                        };
                        optional_params.insert(key_str.to_ascii_uppercase(), val_str);
                    }
                }
                _ => {
                    return Err(Error::Message(
                        "XC_DEFINITION rows must be arrays of length 2".into(),
                    ))
                }
            }
        }

        if functionals.is_empty() {
            return Err(Error::Message(
                "XC_DEFINITION must contain at least one functional".into(),
            ));
        }

        Ok(Self {
            functionals,
            optional_params,
        })
    }
}

impl ToCell for XcDefinition {
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();

        // Add functional entries
        for entry in &self.functionals {
            block_content.push(entry.to_cell_value());
        }

        // Add optional parameters
        for (key, val) in &self.optional_params {
            block_content.push(CellValue::Array(vec![
                CellValue::String(key.clone()),
                CellValue::String(val.clone()),
            ]));
        }

        Cell::Block("XC_DEFINITION", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_xc_functional_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Str("PBE"),
            CellValue::Float(1.0),
        ]);
        let entry = XcFunctionalEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.functional, XcFunctional::Pbe);
        assert_eq!(entry.weight, 1.0);
    }

    #[test]
    fn test_xc_functional_entry_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Str("PBE")]);
        assert!(XcFunctionalEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_xc_definition_empty() {
        let result = XcDefinition::from_block_rows(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_xc_definition_single_functional() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("PBE"),
                CellValue::Float(1.0),
            ]),
        ];
        let result = XcDefinition::from_block_rows(&rows).unwrap();
        assert_eq!(result.functionals.len(), 1);
        assert_eq!(result.functionals[0].functional, XcFunctional::Pbe);
        assert_eq!(result.functionals[0].weight, 1.0);
        assert!(result.optional_params.is_empty());
    }

    #[test]
    fn test_xc_definition_b3lyp_example() {
        // Example from documentation
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("LDA-X"),
                CellValue::Float(0.08),
            ]),
            CellValue::Array(vec![
                CellValue::Str("B88_X"),
                CellValue::Float(0.72),
            ]),
            CellValue::Array(vec![
                CellValue::Str("LYP_C"),
                CellValue::Float(0.81),
            ]),
            CellValue::Array(vec![
                CellValue::Str("LDA-C"),
                CellValue::Float(0.19),
            ]),
            CellValue::Array(vec![
                CellValue::Str("HF"),
                CellValue::Float(0.2),
            ]),
        ];
        let result = XcDefinition::from_block_rows(&rows).unwrap();
        assert_eq!(result.functionals.len(), 5);
        assert_eq!(result.functionals[0].weight, 0.08);
        assert_eq!(result.functionals[1].weight, 0.72);
        assert_eq!(result.functionals[2].weight, 0.81);
        assert_eq!(result.functionals[3].weight, 0.19);
        assert_eq!(result.functionals[4].weight, 0.2);
    }

    #[test]
    fn test_xc_definition_with_optional_params() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("PBE"),
                CellValue::Float(1.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("NLXC_SCREENING_LENGTH"),
                CellValue::Float(0.76),
            ]),
            CellValue::Array(vec![
                CellValue::Str("NLXC_SCREENING_FUNCTION"),
                CellValue::Str("THOMAS-FERMI"),
            ]),
        ];
        let result = XcDefinition::from_block_rows(&rows).unwrap();
        assert_eq!(result.functionals.len(), 1);
        assert_eq!(result.optional_params.len(), 2);
        assert!(result.optional_params.contains_key("NLXC_SCREENING_LENGTH"));
        assert!(result.optional_params.contains_key("NLXC_SCREENING_FUNCTION"));
    }

    #[test]
    fn test_xc_definition_with_bool_params() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("PBE"),
                CellValue::Float(1.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("NLXC_PPD_INT"),
                CellValue::Bool(true),
            ]),
            CellValue::Array(vec![
                CellValue::Str("NLXC_DIVERGENCE_CORR"),
                CellValue::Bool(false),
            ]),
        ];
        let result = XcDefinition::from_block_rows(&rows).unwrap();
        assert_eq!(result.functionals.len(), 1);
        assert_eq!(result.optional_params.len(), 2);
        assert_eq!(result.optional_params.get("NLXC_PPD_INT"), Some(&"ON".to_string()));
        assert_eq!(result.optional_params.get("NLXC_DIVERGENCE_CORR"), Some(&"OFF".to_string()));
    }

    #[test]
    fn test_xc_definition_to_cell() {
        let xc_def = XcDefinition {
            functionals: vec![
                XcFunctionalEntry {
                    functional: XcFunctional::Pbe,
                    weight: 1.0,
                },
            ],
            optional_params: HashMap::new(),
        };
        match xc_def.to_cell() {
            Cell::Block(name, content) => {
                assert_eq!(name, "XC_DEFINITION");
                assert_eq!(content.len(), 1);
            }
            _ => panic!("Expected Block cell"),
        }
    }

    #[test]
    fn test_xc_definition_to_cell_with_params() {
        let mut params = HashMap::new();
        params.insert("NLXC_SCREENING_LENGTH".to_string(), "0.76".to_string());

        let xc_def = XcDefinition {
            functionals: vec![
                XcFunctionalEntry {
                    functional: XcFunctional::Pbe,
                    weight: 1.0,
                },
            ],
            optional_params: params,
        };
        match xc_def.to_cell() {
            Cell::Block(name, content) => {
                assert_eq!(name, "XC_DEFINITION");
                assert_eq!(content.len(), 2);
            }
            _ => panic!("Expected Block cell"),
        }
    }

    #[test]
    fn test_block_name() {
        assert_eq!(XcDefinition::BLOCK_NAME, "XC_DEFINITION");
    }

    #[test]
    fn test_xc_definition_case_insensitive_functional() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("pbe"),
                CellValue::Float(1.0),
            ]),
        ];
        let result = XcDefinition::from_block_rows(&rows).unwrap();
        assert_eq!(result.functionals[0].functional, XcFunctional::Pbe);
    }

    #[test]
    fn test_xc_definition_multiple_functionals() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("PBE"),
                CellValue::Float(0.5),
            ]),
            CellValue::Array(vec![
                CellValue::Str("HF"),
                CellValue::Float(0.5),
            ]),
        ];
        let result = XcDefinition::from_block_rows(&rows).unwrap();
        assert_eq!(result.functionals.len(), 2);
        assert_eq!(result.functionals[0].weight, 0.5);
        assert_eq!(result.functionals[1].weight, 0.5);
    }
}
