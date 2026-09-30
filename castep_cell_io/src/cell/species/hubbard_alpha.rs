use castep_cell_fmt::{
    Cell, CellValue, ToCell, ToCellValue,
    parse::{FromBlock, FromCellValue},
    CResult, Error,
    query::{value_as_f64, value_as_string, value_as_u32},
};
use super::Species;
use super::hubbard_u::HubbardUUnit;

/// Represents an orbital type and its associated Hubbard alpha value.
///
/// Mirror of `OrbitalU`, kept as a distinct newtype so that alpha
/// (spin-dependent) values are not confused with U (on-site) values,
/// even though both are plain floats in the file.
#[derive(Debug, Clone, PartialEq)]
pub enum OrbitalAlpha {
    S(f64),
    P(f64),
    D(f64),
    F(f64),
}

impl OrbitalAlpha {
    /// Gets the orbital type as a character.
    pub fn orbital_char(&self) -> char {
        match self {
            OrbitalAlpha::S(_) => 's',
            OrbitalAlpha::P(_) => 'p',
            OrbitalAlpha::D(_) => 'd',
            OrbitalAlpha::F(_) => 'f',
        }
    }

    /// Gets the alpha value.
    pub fn alpha_value(&self) -> f64 {
        match self {
            OrbitalAlpha::S(v) => *v,
            OrbitalAlpha::P(v) => *v,
            OrbitalAlpha::D(v) => *v,
            OrbitalAlpha::F(v) => *v,
        }
    }

    /// Parses one orbital-spec token from a `HUBBARD_ALPHA` row.
    ///
    /// CASTEP 6.11 splits block lines on whitespace, `=`, `:` and TAB
    /// (`cell_read_line_real` in `Fundamental/cell.f90`), so an orbital
    /// character can appear as `d`, `d:` or `d:0.5` depending on how the
    /// line is written. When the value is joined into the same token
    /// (`d:0.5`), it is returned together with the letter.
    ///
    /// Returns `(letter, inline value)`, or `None` for tokens that are not
    /// orbital specs.
    fn parse_orbital_token(token: &str) -> Option<(char, Option<f64>)> {
        let t = token.to_ascii_lowercase();
        let bytes = t.as_bytes();
        if bytes.is_empty() {
            return None;
        }
        let letter = match bytes[0] {
            b's' => 's',
            b'p' => 'p',
            b'd' => 'd',
            b'f' => 'f',
            _ => return None,
        };
        let rest = t.get(1..)?;
        let rest = rest.strip_prefix(':').unwrap_or(rest);
        if rest.is_empty() {
            return Some((letter, None));
        }
        // The value may carry a leading space (e.g. `d: 0.5`); `f64` parsing
        // does not trim, so trim before parsing.
        Some((letter, Some(rest.trim().parse::<f64>().ok()?)))
    }

    fn new(letter: char, value: f64) -> Option<Self> {
        Some(match letter {
            's' => Self::S(value),
            'p' => Self::P(value),
            'd' => Self::D(value),
            'f' => Self::F(value),
            _ => return None,
        })
    }
}

impl ToCellValue for OrbitalAlpha {
    fn to_cell_value(&self) -> CellValue<'_> {
        // Format as "l: alpha" e.g., "d: 0.5". CASTEP accepts the value
        // separated from the letter by whitespace or a colon, so this
        // token re-parses as `d:` plus a value token.
        CellValue::String(format!(
            "{}: {}",
            self.orbital_char(),
            self.alpha_value()
        ))
    }
}

/// Represents the specification for Hubbard alpha values for a specific
/// atom/ion.
///
/// In CASTEP 6.11 the reader does `read(line, *) first_word, temp_int`
/// and aborts when the second token is not an integer
/// (`Fundamental/cell.f90`), so every row needs the ion number. This
/// differs from `HUBBARD_U`, where the ion number is optional and applies
/// to all ions when omitted.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct AtomHubbardAlpha {
    /// The species.
    pub species: Species,
    /// The 1-based ion number within the species.
    /// CASTEP 6.11 needs this on every row (unlike `HUBBARD_U`).
    pub ion_number: u32,
    /// The list of orbitals and their alpha values.
    /// Orbitals absent from the row keep the CASTEP default of 0.0.
    #[builder(default)]
    pub orbitals: Vec<OrbitalAlpha>,
}

impl ToCellValue for AtomHubbardAlpha {
    fn to_cell_value(&self) -> CellValue<'_> {
        let line_parts = [
            self.species.to_cell_value(),
            CellValue::UInt(self.ion_number),
        ]
        .to_vec()
        .into_iter()
        .chain(self.orbitals.iter().map(|orb| orb.to_cell_value()))
        .collect();
        CellValue::Array(line_parts)
    }
}

/// Represents the HUBBARD_ALPHA block.
///
/// Defines the spin-dependent (alpha) Hubbard correction values to use for
/// specific orbitals. Introduced in CASTEP 6.11.
/// Format:
/// %BLOCK HUBBARD_ALPHA
/// [UNITS]
/// species ion [s: val] [p: val] [d: val] [f: val]
/// ...
/// %ENDBLOCK HUBBARD_ALPHA
///
/// The optional units line is an energy unit; when omitted, the CASTEP
/// default is eV (`Fundamental/cell.f90`, `units_alpha='ev'`).
#[derive(Debug, Clone, PartialEq, Default, bon::Builder)]
pub struct HubbardAlpha {
    /// The unit for alpha values. If `None`, the default (eV) is used.
    pub unit: Option<HubbardUUnit>,
    /// The list of atom-specific Hubbard alpha specifications.
    #[builder(default)]
    pub atom_alpha_values: Vec<AtomHubbardAlpha>,
}

impl FromBlock for AtomHubbardAlpha {
    const BLOCK_NAME: &'static str = "ATOM_HUBBARD_ALPHA";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("AtomHubbardAlpha row is empty".into()));
        }

        let arr = match &rows[0] {
            CellValue::Array(arr) => arr,
            _ => {
                return Err(Error::Message(
                    "AtomHubbardAlpha row must be an array".into(),
                ))
            }
        };

        if arr.len() < 2 {
            return Err(Error::Message(
                "HUBBARD_ALPHA row must be 'species ion' followed by orbital specs"
                    .to_string(),
            ));
        }

        let species = Species::from_cell_value(&arr[0])?;
        let ion_number = value_as_u32(&arr[1]).map_err(|_| {
            Error::Message(
                "HUBBARD_ALPHA needs the ion number as the second token".to_string(),
            )
        })?;

        // Remaining tokens are orbital specs. CASTEP 6.11 scans the row
        // for each of s/p/d/f and takes the following token as the value
        // (`cell_read_line_real`); a later duplicate overwrites an
        // earlier one, and plain numbers are ignored.
        let mut orbitals: Vec<OrbitalAlpha> = Vec::new();
        let mut idx = 2;
        while idx < arr.len() {
            let text = value_as_string(&arr[idx]).ok();
            if let Some(t) = text {
                if let Some((letter, value)) = OrbitalAlpha::parse_orbital_token(t.as_str()) {
                    let val = match value {
                        Some(v) => v,
                        None => {
                            idx += 1;
                            if idx >= arr.len() {
                                return Err(Error::Message(
                                    "HUBBARD_ALPHA orbital spec has no value".to_string(),
                                ));
                            }
                            value_as_f64(&arr[idx])?
                        }
                    };
                    let orbital = OrbitalAlpha::new(letter, val).ok_or_else(|| {
                        Error::Message("HUBBARD_ALPHA orbital spec is malformed".into())
                    })?;
                    // Last occurrence wins, matching the Fortran scan.
                    if let Some(slot) = orbitals
                        .iter_mut()
                        .find(|o| o.orbital_char() == orbital.orbital_char())
                    {
                        *slot = orbital;
                    } else {
                        orbitals.push(orbital);
                    }
                } else if t.parse::<f64>().is_ok() {
                    // CASTEP ignores bare numbers in the row.
                } else {
                    return Err(Error::Message(format!(
                        "HUBBARD_ALPHA row has an unrecognized token: {t}"
                    )));
                }
            }
            idx += 1;
        }

        Ok(Self {
            species,
            ion_number,
            orbitals,
        })
    }
}

impl FromBlock for HubbardAlpha {
    const BLOCK_NAME: &'static str = "HUBBARD_ALPHA";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                atom_alpha_values: Vec::new(),
            });
        }

        // An optional single-token units line precedes the data rows
        // (`units_alpha='ev'` is the default when absent).
        let (unit, data_start) = match &rows[0] {
            CellValue::Array(arr) if arr.len() == 1 => match HubbardUUnit::from_cell_value(&arr[0]) {
                Ok(u) => (Some(u), 1),
                Err(_) => (None, 0),
            },
            _ => (None, 0),
        };

        let atom_alpha_values = rows[data_start..]
            .iter()
            .map(|row| AtomHubbardAlpha::from_block_rows(std::slice::from_ref(row)))
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self {
            unit,
            atom_alpha_values,
        })
    }
}

impl ToCell for HubbardAlpha {
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();

        // Add the optional unit line first
        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        // Add the atom-specific lines
        block_content.extend(
            self.atom_alpha_values
                .iter()
                .map(|atom_a| atom_a.to_cell_value()),
        );

        Cell::Block("HUBBARD_ALPHA", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_atom_hubbard_alpha_builder_basic() {
        let species = Species::Symbol("Fe".to_string());
        let orbital = OrbitalAlpha::D(0.5);

        let atom_a = AtomHubbardAlpha::builder()
            .species(species.clone())
            .ion_number(1)
            .orbitals(vec![orbital.clone()])
            .build();

        assert_eq!(atom_a.species, species);
        assert_eq!(atom_a.ion_number, 1);
        assert_eq!(atom_a.orbitals.len(), 1);
        assert_eq!(atom_a.orbitals[0], orbital);
    }

    #[test]
    fn test_atom_hubbard_alpha_orbital_chars() {
        assert_eq!(OrbitalAlpha::S(1.0).orbital_char(), 's');
        assert_eq!(OrbitalAlpha::P(1.0).orbital_char(), 'p');
        assert_eq!(OrbitalAlpha::D(1.0).orbital_char(), 'd');
        assert_eq!(OrbitalAlpha::F(1.0).orbital_char(), 'f');
        assert_eq!(OrbitalAlpha::D(2.5).alpha_value(), 2.5);
    }

    #[test]
    fn test_atom_hubbard_alpha_to_cell_value() {
        let atom_a = AtomHubbardAlpha::builder()
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(2)
            .orbitals(vec![OrbitalAlpha::D(0.5)])
            .build();

        let val = atom_a.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                // [species, ion, "d: 0.5"]; a Symbol species serializes to
                // an owned String token.
                assert_eq!(arr.len(), 3);
                assert!(matches!(arr[0], CellValue::String(_)));
                assert!(matches!(arr[1], CellValue::UInt(2)));
                assert!(matches!(arr[2], CellValue::String(_)));
            }
            _ => panic!("Expected Array, got {:?}", val),
        }
    }

    #[test]
    fn test_atom_hubbard_alpha_round_trip() {
        // Serialized form: `Fe   2 d: 0.5` -> [Str("Fe"), UInt(2),
        // Str("d:"), Float(0.5)] when re-parsed.
        let atom_a = AtomHubbardAlpha::builder()
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(2)
            .orbitals(vec![OrbitalAlpha::D(0.5)])
            .build();

        let row = atom_a.to_cell_value();
        let arr = match &row {
            CellValue::Array(arr) => arr,
            other => panic!("Expected Array, got {:?}", other),
        };
        let re_parsed =
            AtomHubbardAlpha::from_block_rows(&[CellValue::Array(arr.clone())]).unwrap();
        assert_eq!(re_parsed.species, atom_a.species);
        assert_eq!(re_parsed.ion_number, 2);
        assert_eq!(re_parsed.orbitals, atom_a.orbitals);
    }

    #[test]
    fn test_atom_alpha_castep_style_row() {
        // `Fe 1 s 0.0 p 0.0 d 0.5 f 0.0` as parsed tokens.
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Str("s"),
            CellValue::Float(0.0),
            CellValue::Str("p"),
            CellValue::Float(0.0),
            CellValue::Str("d"),
            CellValue::Float(0.5),
            CellValue::Str("f"),
            CellValue::Float(0.0),
        ])];
        let result = AtomHubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.ion_number, 1);
        assert_eq!(
            result.orbitals,
            vec![
                OrbitalAlpha::S(0.0),
                OrbitalAlpha::P(0.0),
                OrbitalAlpha::D(0.5),
                OrbitalAlpha::F(0.0),
            ]
        );
    }

    #[test]
    fn test_atom_alpha_colon_joined_spec() {
        // `Fe 1 d:0.5` — the value joined to the letter in one token.
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Str("d:0.5"),
        ])];
        let result = AtomHubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.orbitals, vec![OrbitalAlpha::D(0.5)]);
    }

    #[test]
    fn test_atom_alpha_colon_separated_spec() {
        // `Fe 1 d: 0.5` — letter with trailing colon, value next token.
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Str("d:"),
            CellValue::Float(0.5),
        ])];
        let result = AtomHubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.orbitals, vec![OrbitalAlpha::D(0.5)]);
    }

    #[test]
    fn test_atom_alpha_atomic_number_species() {
        let rows = vec![CellValue::Array(vec![
            CellValue::UInt(26),
            CellValue::UInt(1),
            CellValue::Str("d:"),
            CellValue::Float(0.5),
        ])];
        let result = AtomHubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.species, Species::AtomicNumber(26));
    }

    #[test]
    fn test_atom_alpha_missing_ion_number_errors() {
        // 6.11 needs the ion number as the second token.
        let rows = vec![CellValue::Array(vec![CellValue::Str("Fe")])];
        assert!(AtomHubbardAlpha::from_block_rows(&rows).is_err());

        let rows = vec![CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Str("d:"),
            CellValue::Float(0.5),
        ])];
        assert!(AtomHubbardAlpha::from_block_rows(&rows).is_err());
    }

    #[test]
    fn test_atom_alpha_unrecognized_token_errors() {
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Str("x:0.5"),
        ])];
        assert!(AtomHubbardAlpha::from_block_rows(&rows).is_err());
    }

    #[test]
    fn test_atom_alpha_duplicate_last_wins() {
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Str("d:"),
            CellValue::Float(1.0),
            CellValue::Str("d:"),
            CellValue::Float(2.0),
        ])];
        let result = AtomHubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.orbitals, vec![OrbitalAlpha::D(2.0)]);
    }

    #[test]
    fn test_hubbard_alpha_builder_basic() {
        let atom_a = AtomHubbardAlpha::builder()
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(1)
            .orbitals(vec![OrbitalAlpha::D(0.5)])
            .build();

        let ha = HubbardAlpha::builder()
            .unit(HubbardUUnit::ElectronVolt)
            .atom_alpha_values(vec![atom_a.clone()])
            .build();

        assert_eq!(ha.unit, Some(HubbardUUnit::ElectronVolt));
        assert_eq!(ha.atom_alpha_values.len(), 1);
        assert_eq!(ha.atom_alpha_values[0], atom_a);
    }

    #[test]
    fn test_hubbard_alpha_default() {
        let ha = HubbardAlpha::default();
        assert!(ha.unit.is_none());
        assert_eq!(ha.atom_alpha_values.len(), 0);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(HubbardAlpha::BLOCK_NAME, "HUBBARD_ALPHA");
    }

    #[test]
    fn test_atom_alpha_empty_rows_error() {
        assert!(AtomHubbardAlpha::from_block_rows(&[]).is_err());
    }

    #[test]
    fn test_hubbard_alpha_empty() {
        let result = HubbardAlpha::from_block_rows(&[]).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.atom_alpha_values.len(), 0);
    }

    #[test]
    fn test_hubbard_alpha_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::UInt(1),
                CellValue::Str("d:"),
                CellValue::Float(0.5),
            ]),
        ];
        let result = HubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(HubbardUUnit::ElectronVolt));
        assert_eq!(result.atom_alpha_values.len(), 1);
        assert_eq!(result.atom_alpha_values[0].ion_number, 1);
    }

    #[test]
    fn test_hubbard_alpha_without_unit() {
        let rows = vec![CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::UInt(1),
            CellValue::Str("p:"),
            CellValue::Float(0.3),
        ])];
        let result = HubbardAlpha::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.atom_alpha_values.len(), 1);
    }

    #[test]
    fn test_hubbard_alpha_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::UInt(1),
                CellValue::Str("d:"),
                CellValue::Float(0.5),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::UInt(2),
                CellValue::Str("p:"),
                CellValue::Float(0.3),
            ]),
        ];
        let result = HubbardAlpha::from_block_rows(&rows).unwrap();
        assert_eq!(result.atom_alpha_values.len(), 2);
    }

    #[test]
    fn test_hubbard_alpha_to_cell() {
        let atom_a = AtomHubbardAlpha::builder()
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(1)
            .orbitals(vec![OrbitalAlpha::D(0.5)])
            .build();
        let ha = HubbardAlpha::builder()
            .unit(HubbardUUnit::ElectronVolt)
            .atom_alpha_values(vec![atom_a])
            .build();

        let cell = ha.to_cell();
        match cell {
            Cell::Block(name, content) => {
                assert_eq!(name, "HUBBARD_ALPHA");
                // 1 unit line + 1 atom line
                assert_eq!(content.len(), 2);
            }
            _ => panic!("Expected Block, got {:?}", cell),
        }
    }
}
