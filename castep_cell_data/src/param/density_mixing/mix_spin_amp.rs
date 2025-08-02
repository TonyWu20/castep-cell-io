use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the mixing amplitude for the spin density in the density mixing procedure.
///
/// Keyword type: Real
///
/// Default: 2.0
///
/// Example:
/// MIX_SPIN_AMP : 1.754
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_SPIN_AMP")]
pub struct MixSpinAmp(pub f64);

impl Default for MixSpinAmp {
    fn default() -> Self {
        Self(2.0) // Default is 2.0
    }
}

impl ToCell for MixSpinAmp {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_SPIN_AMP", CellValue::Float(self.0))
    }
}

impl ToCellValue for MixSpinAmp {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_mix_spin_amp_serde() {
        let mix_spin_amp_str = "MIX_SPIN_AMP : 1.754";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixSpinAmp {
            mix_spin_amp: MixSpinAmp,
        }

        let cell_file_result: Result<CellFileWithMixSpinAmp, _> = from_str(mix_spin_amp_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.mix_spin_amp.0 - 1.754).abs() < 1e-10);

        let mix_spin_amp_instance = MixSpinAmp(1.5);
        let serialized_result = to_string(&mix_spin_amp_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MIX_SPIN_AMP (1.5): {serialized_string}");
        assert!(serialized_string.contains("MIX_SPIN_AMP"));
        assert!(serialized_string.contains("1.5"));

        assert_eq!(MixSpinAmp::default(), MixSpinAmp(2.0));
    }
}
