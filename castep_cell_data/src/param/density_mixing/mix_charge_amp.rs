use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the mixing amplitude for the charge density in the density mixing procedure.
///
/// Keyword type: Real
///
/// Default: 0.8
///
/// Example:
/// MIX_CHARGE_AMP : 0.5
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_CHARGE_AMP")]
pub struct MixChargeAmp(pub f64);

impl Default for MixChargeAmp {
    fn default() -> Self {
        Self(0.8) // Default is 0.8
    }
}

impl ToCell for MixChargeAmp {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_CHARGE_AMP", CellValue::Float(self.0))
    }
}

impl ToCellValue for MixChargeAmp {
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
    fn test_mix_charge_amp_serde() {
        let mix_charge_amp_str = "MIX_CHARGE_AMP : 0.5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixChargeAmp {
            mix_charge_amp: MixChargeAmp,
        }

        let cell_file_result: Result<CellFileWithMixChargeAmp, _> = from_str(mix_charge_amp_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.mix_charge_amp.0 - 0.5).abs() < f64::EPSILON);

        let mix_charge_amp_instance = MixChargeAmp(0.6);
        let serialized_result = to_string(&mix_charge_amp_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MIX_CHARGE_AMP (0.6): {serialized_string}");
        assert!(serialized_string.contains("MIX_CHARGE_AMP"));
        assert!(serialized_string.contains("0.6"));

        assert_eq!(MixChargeAmp::default(), MixChargeAmp(0.8));
    }
}
