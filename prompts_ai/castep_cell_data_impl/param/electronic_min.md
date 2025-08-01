This is module `electronic_minimisation`.

```
SPIN_FIX (.param)
Keyword type
Integer

Description
This keyword determines the number of electronic iterations for which the total spin is fixed. If SPIN_FIX is less than 0, the spin will be fixed for the duration of the calculation.

This keyword only effects the behavior of the electronic minimizer in the initial SCF calculation. There is a separate keyword, GEOM_SPIN_FIX, which should be used to fix the spin of the system during geometry optimization.

This parameter is only used if FIX_OCCUPANCY : FALSE. So, for insulators the spin is fixed regardless of the value of SPIN_FIX.

Default
10

Example
SPIN_FIX : 5
```

```
SMEARING_WIDTH (.param)
Keyword type
Real

Description
This keyword determines the width of the Fermi-surface smearing if the system is being treated as a metal.

This parameter is used only if FIX_OCCUPANCY : FALSE.

Default
0.2 eV

Example
SMEARING_WIDTH : 0.1 eV
```

```
SMEARING_SCHEME (.param)
Keyword type
String

Description
This keyword determines the Fermi-surface smearing scheme to be used if the system is being treated as a metal. Available options are:

Gaussian
GaussianSplines
FermiDirac
HermitePolynomials
ColdSmearing
This parameter is used only if FIX_OCCUPANCY : FALSE.

Default
Gaussian

Example
SMEARING_SCHEME : ColdSmearing
```

```
NUM_DUMP_CYCLES (.param)
Keyword type
Integer

Description
This keyword specifies the number of SCF cycles between updates to the wavefunction and density data file. If NUM_DUMP_CYCLES is less than or equal to 0, no data will be written to this file.

Default
5

Example
NUM_DUMP_CYCLES : 10
```

```
METALS_METHOD (.param)
Keyword type
String

Description
This keyword determines the electronic minimization method used in the self-consistent calculation. In spite of the term "METALS" in the name of the parameter its usage is the same for metals (FIX_OCCUPANCY : FALSE) and semiconductors (FIX_OCCUPANCY : TRUE). Available options are:

DM - system treated by density mixing (in this case density mixing parameters are printed in the .castep file).
EDFT - system treated by ensemble density functional method: CASTEP does a self-consistent all-bands wavefunction search, which for metals is followed by the self-consistent updating of occupancies.
NONE - currently not used: it is not supported for FIX_OCCUPANCY : FALSE and is the same as EDFT for FIX_OCCUPANCY : TRUE.
Default
EDFT

FIX_OCCUPANCY : FALSE, the default is DM

Example
METALS_METHOD : dm
```

```
MAX_SD_STEPS (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of steepest descent steps in an SCF cycle.

Default
The default depends on the value of ELECTRONIC_MINIMIZER:

SD then MAX_SD_STEPS : 10
CG then MAX_SD_STEPS : 1
RMM/DIIS then MAX_SD_STEPS : 1
If ELECTRONIC_MINIMIZER is not defined, the default is 1.

Example
MAX_SD_STEPS : 5
```

```
MAX_SCF_CYCLES (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of SCF cycles performed in an electronic minimization. The electronic minimization will end after this many cycles, regardless of whether the convergence criteria have been met.

Default
30

Example
MAX_SCF_CYCLES : 20
```

```
MAX_CG_STEPS (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of conjugate gradient steps in an SCF cycle.

Default
The default depends on the value of ELECTRONIC_MINIMIZER:

SD then MAX_CG_STEPS : 0
CG then MAX_CG_STEPS : 4
RMM/DIIS then MAX_CG_STEPS : 2
If ELECTRONIC_MINIMIZER is not defined, the default is 4.

Example
MAX_CG_STEPS : 5
```

```
FIX_OCCUPANCY (.param)
Keyword type
Logical

Description
This keyword specifies whether or not the occupancies of the bands should be fixed, that is, if the system should be treated as a zero temperature insulator or a metal.

Default
FALSE

Example
FIX_OCCUPANCY : TRUE
```

```
ELECTRONIC_MINIMIZER (.param)
Keyword type
String

Description
This keyword controls the method used to minimize electronic states. Available options are:

SD - minimizer takes up to 10 SD steps.
CG - minimizer takes one SD step followed by up to 4 CG steps.
The default values for the number of steps can be overwritten using the MAX_SD_STEPS and MAX_CG_STEPS keywords.

Default
CG

Example
ELECTRONIC_MINIMIZER : SD
```

```
ELEC_RESTORE_FILE (.param)
Keyword type
String

Description
This keyword specifies the name of the file from which wavefunction and density data should be restored when performing a CONTINUATION or a REUSE run.

NULL means that wavefunction and density data will not be restored.

The basis set and distribution for the new run must be the same as those from the run in which the data file was written.

Default
NULL

Example
ELEC_RESTORE_FILE : test.wvfn
```

```
ELEC_ENERGY_TOL (.param)
Keyword type
Real

Description
This keyword controls the tolerance for accepting convergence of the total energy in an electronic minimization.

The difference between maximum and minimum energies over ELEC_CONVERGENCE_WIN iterations must be less than this value.

Default
1x10-5 eV per atom

Example
ELEC_ENERGY_TOL : 0.00007 eV
```

```
ELEC_EIGENVALUE_TOL (.param)
Keyword type
Real

Description
This keyword controls the tolerance for accepting convergence of a single eigenvalue during density mixing minimization.

The difference between maximum and minimum eigenvalues over ELEC_CONVERGENCE_WIN iterations must be less than this value.

Default
The default value is the lower of 1x10-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS, where NATOMS is the total number of atoms in the unit cell.

Example
ELEC_EIGENVALUE_TOL : 0.000007 eV
```

```
ELEC_DUMP_FILE (.param)
Keyword type
String

Description
This keyword determines the name of the file into which wavefunction and density data are written, periodically during electronic minimization. This file can be used as a backup and is restored with the ELEC_RESTORE_FILE parameter.

If this parameter is set to NULL, no backup wavefunction or density data will be written.

Default
seedname.wvfn

Example
ELEC_DUMP_FILE : test.wvfn
```

```
ELEC_CONVERGENCE_WIN (.param)
Keyword type
Integer

Description
This keyword determines the size of the convergence window during a electronic minimization run.

The total energy or eigenvalue must lie within ELEC_ENERGY_TOL or ELEC_EIGENVALUE_TOL respectively, for the last ELEC_CONVERGENCE_WIN iterations for the convergence criteria to be met.

The value of ELEC_CONVERGENCE_WIN must be greater than or equal to 2.

Default
3

Example
ELEC_CONVERGENCE_WIN : 4
```

```
EFERMI_TOL (.param)
Keyword type
Real

Description
This keyword controls the tolerance for accepting convergence of the Fermi-energy if the system is being treated as a metal.

This parameter is used only if FIX_OCCUPANCY : FALSE.

Default
0.1 × ELEC_EIGENVALUE_TOL

Example
EFERMI_TOL : 0.0000007 eV
```

---

Okay, following the established patterns from your `castep_cell_data` implementations and incorporating `clippy`'s suggestions and functional style, here are the individual modules for the new `electronic_minimisation` types. These assume the existence of `EnergyUnit` in your `units` module.

```rust
// File: electronic_minimisation/spin_fix.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the number of electronic iterations for which the total spin is fixed.
///
/// Keyword type: Integer
///
/// Default: 10
///
/// Example:
/// SPIN_FIX : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SPIN_FIX")]
pub struct SpinFix(pub i32);

impl Default for SpinFix {
    fn default() -> Self {
        Self(10) // Default is 10
    }
}

impl ToCell for SpinFix {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN_FIX", CellValue::Int(self.0))
    }
}

impl ToCellValue for SpinFix {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_spin_fix_serde() {
        let spin_fix_str = "SPIN_FIX : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSpinFix {
            spin_fix: SpinFix,
        }

        let cell_file_result: Result<CellFileWithSpinFix, _> = from_str(spin_fix_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.spin_fix.0, 5);

        let spin_fix_instance = SpinFix(-1);
        let serialized_result = to_string(&spin_fix_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SPIN_FIX (-1): {serialized_string}");
        assert!(serialized_string.contains("SPIN_FIX"));
        assert!(serialized_string.contains("-1"));

        assert_eq!(SpinFix::default(), SpinFix(10));
    }
}
```

```rust
// File: electronic_minimisation/smearing_width.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::energy_unit::EnergyUnit;

/// Determines the width of the Fermi-surface smearing.
///
/// Keyword type: Real
///
/// Default: 0.2 eV
///
/// Example:
/// SMEARING_WIDTH : 0.1 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "SMEARING_WIDTH")]
pub struct SmearingWidth {
    /// The smearing width value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: EnergyUnit,
}

// Intermediate representation for deserialization
#[derive(Debug, Deserialize)]
struct SmearingWidthRepr {
    value: f64,
    unit: EnergyUnit,
}

impl From<SmearingWidthRepr> for SmearingWidth {
    fn from(repr: SmearingWidthRepr) -> Self {
        Self {
            value: repr.value,
            unit: repr.unit,
        }
    }
}

impl ToCell for SmearingWidth {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "SMEARING_WIDTH",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for SmearingWidth {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_smearing_width_serde() {
        let smearing_width_str = "SMEARING_WIDTH : 0.1 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSmearingWidth {
            smearing_width: SmearingWidth,
        }

        let cell_file_result: Result<CellFileWithSmearingWidth, _> = from_str(smearing_width_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.smearing_width.value - 0.1).abs() < 1e-10);
        assert_eq!(cell_file.smearing_width.unit, EnergyUnit::ElectronVolt);

        let smearing_width_instance = SmearingWidth {
            value: 0.2,
            unit: EnergyUnit::Hartree,
        };
        let serialized_result = to_string(&smearing_width_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SMEARING_WIDTH (0.2 ha): {serialized_string}");
        assert!(serialized_string.contains("SMEARING_WIDTH"));
        assert!(serialized_string.contains("0.2"));
        assert!(serialized_string.contains("ha"));
    }
}
```

```rust
// File: electronic_minimisation/smearing_scheme.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the Fermi-surface smearing scheme.
///
/// Keyword type: String
///
/// Default: SmearingScheme::Gaussian
///
/// Example:
/// SMEARING_SCHEME : ColdSmearing
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SMEARING_SCHEME")]
pub enum SmearingScheme {
    /// Gaussian smearing
    #[serde(rename = "Gaussian")]
    Gaussian,
    /// Gaussian splines smearing
    #[serde(rename = "GaussianSplines")]
    GaussianSplines,
    /// Fermi-Dirac smearing
    #[serde(rename = "FermiDirac")]
    FermiDirac,
    /// Hermite polynomials smearing
    #[serde(rename = "HermitePolynomials")]
    HermitePolynomials,
    /// Cold smearing
    #[serde(rename = "ColdSmearing")]
    ColdSmearing,
}

impl Default for SmearingScheme {
    fn default() -> Self {
        Self::Gaussian // Default is Gaussian
    }
}

impl ToCell for SmearingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SMEARING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for SmearingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SmearingScheme::Gaussian => "Gaussian",
                SmearingScheme::GaussianSplines => "GaussianSplines",
                SmearingScheme::FermiDirac => "FermiDirac",
                SmearingScheme::HermitePolynomials => "HermitePolynomials",
                SmearingScheme::ColdSmearing => "ColdSmearing",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_smearing_scheme_serde() {
        let test_cases = [
            ("SMEARING_SCHEME : Gaussian", SmearingScheme::Gaussian),
            ("SMEARING_SCHEME : FermiDirac", SmearingScheme::FermiDirac),
            ("SMEARING_SCHEME : ColdSmearing", SmearingScheme::ColdSmearing),
        ];

        for (input_str, expected_scheme) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithSmearingScheme {
                smearing_scheme: SmearingScheme,
            }

            let cell_file_result: Result<CellFileWithSmearingScheme, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(cell_file.smearing_scheme, expected_scheme, "Failed for input: {}", input_str);
        }

        let smearing_scheme_instance = SmearingScheme::ColdSmearing;
        let serialized_result = to_string(&smearing_scheme_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SMEARING_SCHEME (ColdSmearing): {serialized_string}");
        assert!(serialized_string.contains("SMEARING_SCHEME"));
        assert!(serialized_string.contains("ColdSmearing"));

        assert_eq!(SmearingScheme::default(), SmearingScheme::Gaussian);
    }
}
```

```rust
// File: electronic_minimisation/num_dump_cycles.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the number of SCF cycles between updates to the wavefunction and density data file.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// NUM_DUMP_CYCLES : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "NUM_DUMP_CYCLES")]
pub struct NumDumpCycles(pub i32); // Using i32 to allow for <= 0 values

impl Default for NumDumpCycles {
    fn default() -> Self {
        Self(5) // Default is 5
    }
}

impl ToCell for NumDumpCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NUM_DUMP_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for NumDumpCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_num_dump_cycles_serde() {
        let num_dump_cycles_str = "NUM_DUMP_CYCLES : 10";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNumDumpCycles {
            num_dump_cycles: NumDumpCycles,
        }

        let cell_file_result: Result<CellFileWithNumDumpCycles, _> = from_str(num_dump_cycles_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.num_dump_cycles.0, 10);

        let num_dump_cycles_instance = NumDumpCycles(0);
        let serialized_result = to_string(&num_dump_cycles_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NUM_DUMP_CYCLES (0): {serialized_string}");
        assert!(serialized_string.contains("NUM_DUMP_CYCLES"));
        assert!(serialized_string.contains("0"));

        assert_eq!(NumDumpCycles::default(), NumDumpCycles(5));
    }
}
```

```rust
// File: electronic_minimisation/metals_method.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the electronic minimization method used in the self-consistent calculation.
///
/// Keyword type: String
///
/// Default: MetalsMethod::Edft (or MetalsMethod::Dm if FIX_OCCUPANCY is FALSE)
///
/// Example:
/// METALS_METHOD : dm
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "METALS_METHOD")]
pub enum MetalsMethod {
    /// System treated by density mixing
    #[serde(rename = "DM")]
    Dm,
    /// System treated by ensemble density functional method
    #[serde(rename = "EDFT")]
    Edft,
    /// Currently not used
    #[serde(rename = "NONE")]
    None,
}

// Note: Default logic is context-dependent (depends on FIX_OCCUPANCY).
// The `Default` implementation here provides a base default.
impl Default for MetalsMethod {
    fn default() -> Self {
        Self::Edft // Base default is EDFT
    }
}

impl ToCell for MetalsMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("METALS_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MetalsMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MetalsMethod::Dm => "DM",
                MetalsMethod::Edft => "EDFT",
                MetalsMethod::None => "NONE",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_metals_method_serde() {
        let test_cases = [
            ("METALS_METHOD : DM", MetalsMethod::Dm),
            ("METALS_METHOD : EDFT", MetalsMethod::Edft),
            ("METALS_METHOD : NONE", MetalsMethod::None),
        ];

        for (input_str, expected_method) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMetalsMethod {
                metals_method: MetalsMethod,
            }

            let cell_file_result: Result<CellFileWithMetalsMethod, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(cell_file.metals_method, expected_method, "Failed for input: {}", input_str);
        }

        let metals_method_instance = MetalsMethod::Dm;
        let serialized_result = to_string(&metals_method_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized METALS_METHOD (DM): {serialized_string}");
        assert!(serialized_string.contains("METALS_METHOD"));
        assert!(serialized_string.contains("DM"));

        assert_eq!(MetalsMethod::default(), MetalsMethod::Edft);
    }
}
```

```rust
// File: electronic_minimisation/max_sd_steps.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of steepest descent steps in an SCF cycle.
///
/// Keyword type: Integer
///
/// Default:
/// SD then MAX_SD_STEPS : 10
/// CG then MAX_SD_STEPS : 1
/// RMM/DIIS then MAX_SD_STEPS : 1
/// If ELECTRONIC_MINIMIZER is not defined, the default is 1.
///
/// Example:
/// MAX_SD_STEPS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAX_SD_STEPS")]
pub struct MaxSdSteps(pub i32);

// Note: Default logic is context-dependent (depends on ELECTRONIC_MINIMIZER).
// The `Default` implementation here provides a base default.
impl Default for MaxSdSteps {
    fn default() -> Self {
        Self(1) // Base default is 1 (if ELECTRONIC_MINIMIZER is not defined)
    }
}

impl ToCell for MaxSdSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_SD_STEPS", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxSdSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_max_sd_steps_serde() {
        let max_sd_steps_str = "MAX_SD_STEPS : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMaxSdSteps {
            max_sd_steps: MaxSdSteps,
        }

        let cell_file_result: Result<CellFileWithMaxSdSteps, _> = from_str(max_sd_steps_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.max_sd_steps.0, 5);

        let max_sd_steps_instance = MaxSdSteps(10);
        let serialized_result = to_string(&max_sd_steps_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAX_SD_STEPS (10): {serialized_string}");
        assert!(serialized_string.contains("MAX_SD_STEPS"));
        assert!(serialized_string.contains("10"));

        assert_eq!(MaxSdSteps::default(), MaxSdSteps(1));
    }
}
```

```rust
// File: electronic_minimisation/max_scf_cycles.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of SCF cycles performed in an electronic minimization.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// MAX_SCF_CYCLES : 20
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAX_SCF_CYCLES")]
pub struct MaxScfCycles(pub i32);

impl Default for MaxScfCycles {
    fn default() -> Self {
        Self(30) // Default is 30
    }
}

impl ToCell for MaxScfCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_SCF_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxScfCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_max_scf_cycles_serde() {
        let max_scf_cycles_str = "MAX_SCF_CYCLES : 20";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMaxScfCycles {
            max_scf_cycles: MaxScfCycles,
        }

        let cell_file_result: Result<CellFileWithMaxScfCycles, _> = from_str(max_scf_cycles_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.max_scf_cycles.0, 20);

        let max_scf_cycles_instance = MaxScfCycles(50);
        let serialized_result = to_string(&max_scf_cycles_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAX_SCF_CYCLES (50): {serialized_string}");
        assert!(serialized_string.contains("MAX_SCF_CYCLES"));
        assert!(serialized_string.contains("50"));

        assert_eq!(MaxScfCycles::default(), MaxScfCycles(30));
    }
}
```

```rust
// File: electronic_minimisation/max_cg_steps.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of conjugate gradient steps in an SCF cycle.
///
/// Keyword type: Integer
///
/// Default:
/// SD then MAX_CG_STEPS : 0
/// CG then MAX_CG_STEPS : 4
/// RMM/DIIS then MAX_CG_STEPS : 2
/// If ELECTRONIC_MINIMIZER is not defined, the default is 4.
///
/// Example:
/// MAX_CG_STEPS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAX_CG_STEPS")]
pub struct MaxCgSteps(pub i32);

// Note: Default logic is context-dependent (depends on ELECTRONIC_MINIMIZER).
// The `Default` implementation here provides a base default.
impl Default for MaxCgSteps {
    fn default() -> Self {
        Self(4) // Base default is 4 (if ELECTRONIC_MINIMIZER is not defined)
    }
}

impl ToCell for MaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_CG_STEPS", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxCgSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_max_cg_steps_serde() {
        let max_cg_steps_str = "MAX_CG_STEPS : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMaxCgSteps {
            max_cg_steps: MaxCgSteps,
        }

        let cell_file_result: Result<CellFileWithMaxCgSteps, _> = from_str(max_cg_steps_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.max_cg_steps.0, 5);

        let max_cg_steps_instance = MaxCgSteps(2);
        let serialized_result = to_string(&max_cg_steps_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAX_CG_STEPS (2): {serialized_string}");
        assert!(serialized_string.contains("MAX_CG_STEPS"));
        assert!(serialized_string.contains("2"));

        assert_eq!(MaxCgSteps::default(), MaxCgSteps(4));
    }
}
```

```rust
// File: electronic_minimisation/fix_occupancy.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not the occupancies of the bands should be fixed.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// FIX_OCCUPANCY : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIX_OCCUPANCY")]
pub struct FixOccupancy(pub bool);

impl Default for FixOccupancy {
    fn default() -> Self {
        Self(false) // Default is FALSE
    }
}

impl ToCell for FixOccupancy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_OCCUPANCY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixOccupancy {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_fix_occupancy_serde() {
        let fix_occupancy_true_str = "FIX_OCCUPANCY : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixOccupancyTrue {
            fix_occupancy: FixOccupancy,
        }

        let cell_file_true_result: Result<CellFileWithFixOccupancyTrue, _> = from_str(fix_occupancy_true_str);
        assert!(cell_file_true_result.is_ok(), "Deserialization (TRUE) failed: {:?}", cell_file_true_result.err());
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.fix_occupancy.0);

        let fix_occupancy_false_str = "FIX_OCCUPANCY : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixOccupancyFalse {
            fix_occupancy: FixOccupancy,
        }

        let cell_file_false_result: Result<CellFileWithFixOccupancyFalse, _> = from_str(fix_occupancy_false_str);
        assert!(cell_file_false_result.is_ok(), "Deserialization (FALSE) failed: {:?}", cell_file_false_result.err());
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.fix_occupancy.0);

        let fix_occupancy_instance = FixOccupancy(true);
        let serialized_result = to_string(&fix_occupancy_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FIX_OCCUPANCY (TRUE): {serialized_string}");
        assert!(serialized_string.contains("FIX_OCCUPANCY"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        assert_eq!(FixOccupancy::default(), FixOccupancy(false));
    }
}
```

```rust
// File: electronic_minimisation/electronic_minimizer.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the method used to minimize electronic states.
///
/// Keyword type: String
///
/// Default: ElectronicMinimizer::Cg
///
/// Example:
/// ELECTRONIC_MINIMIZER : SD
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELECTRONIC_MINIMIZER")]
pub enum ElectronicMinimizer {
    /// Minimizer takes up to 10 SD steps
    #[serde(rename = "SD", alias="sd")]
    Sd,
    /// Minimizer takes one SD step followed by up to 4 CG steps
    #[serde(rename = "CG", alias="cg")]
    Cg,
    #[serde(rename = "RMM/DIIS",alias="rmm/diis")]
	RmmDiis,
    // Note: RMM/DIIS is mentioned in defaults for MAX_SD_STEPS/MAX_CG_STEPS
    // but not listed as an option in the main description. If it's a valid option,
    // it should be added here.
}

impl Default for ElectronicMinimizer {
    fn default() -> Self {
        Self::Cg // Default is CG
    }
}

impl ToCell for ElectronicMinimizer {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELECTRONIC_MINIMIZER", self.to_cell_value())
    }
}

impl ToCellValue for ElectronicMinimizer {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ElectronicMinimizer::Sd => "SD",
                ElectronicMinimizer::Cg => "CG",
                // ElectronicMinimizer::RmmDiis => "RMM/DIIS", // If added
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_electronic_minimizer_serde() {
        let test_cases = [
            ("ELECTRONIC_MINIMIZER : SD", ElectronicMinimizer::Sd),
            ("ELECTRONIC_MINIMIZER : CG", ElectronicMinimizer::Cg),
            // ("ELECTRONIC_MINIMIZER : RMM/DIIS", ElectronicMinimizer::RmmDiis), // If added
        ];

        for (input_str, expected_minimizer) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithElectronicMinimizer {
                electronic_minimizer: ElectronicMinimizer,
            }

            let cell_file_result: Result<CellFileWithElectronicMinimizer, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(cell_file.electronic_minimizer, expected_minimizer, "Failed for input: {}", input_str);
        }

        let electronic_minimizer_instance = ElectronicMinimizer::Sd;
        let serialized_result = to_string(&electronic_minimizer_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELECTRONIC_MINIMIZER (SD): {serialized_string}");
        assert!(serialized_string.contains("ELECTRONIC_MINIMIZER"));
        assert!(serialized_string.contains("SD"));

        assert_eq!(ElectronicMinimizer::default(), ElectronicMinimizer::Cg);
    }
}
```

```rust
// File: electronic_minimisation/elec_restore_file.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the name of the file from which wavefunction and density data should be restored.
///
/// Keyword type: String
///
/// Default: NULL (no restore)
///
/// Example:
/// ELEC_RESTORE_FILE : test.wvfn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELEC_RESTORE_FILE")]
pub struct ElecRestoreFile(pub String); // Could be an enum Option<String> or similar for NULL handling

impl ToCell for ElecRestoreFile {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_RESTORE_FILE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for ElecRestoreFile {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_restore_file_serde() {
        let elec_restore_file_str = "ELEC_RESTORE_FILE : test.wvfn";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecRestoreFile {
            elec_restore_file: ElecRestoreFile,
        }

        let cell_file_result: Result<CellFileWithElecRestoreFile, _> = from_str(elec_restore_file_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.elec_restore_file.0, "test.wvfn");

        let elec_restore_file_instance = ElecRestoreFile("backup.wvfn".to_string());
        let serialized_result = to_string(&elec_restore_file_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELEC_RESTORE_FILE (backup.wvfn): {serialized_string}");
        assert!(serialized_string.contains("ELEC_RESTORE_FILE"));
        assert!(serialized_string.contains("backup.wvfn"));
    }
}
```

```rust
// File: electronic_minimisation/elec_energy_tol.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::energy_unit::EnergyUnit;

/// Controls the tolerance for accepting convergence of the total energy in an electronic minimization.
///
/// Keyword type: Real
///
/// Default: 1e-5 eV per atom
///
/// Example:
/// ELEC_ENERGY_TOL : 0.00007 eV
/// ELEC_ENERGY_TOL : 0.00007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "ELEC_ENERGY_TOL")]
#[serde(from = "ElecEnergyTolRepr")] // Use intermediate repr for deserialization
pub struct ElecEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `ElecEnergyTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ElecEnergyTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<ElecEnergyTolRepr> for ElecEnergyTol {
    fn from(repr: ElecEnergyTolRepr) -> Self {
        match repr {
            ElecEnergyTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            ElecEnergyTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for ElecEnergyTol {
    fn to_cell(&self) -> Cell {
        // Create a CellValue::Array containing the value and optionally the unit
        let mut array_contents = vec![CellValue::Float(self.value)];
        if let Some(unit) = &self.unit {
            array_contents.push(unit.to_cell_value());
        }
        Cell::KeyValue("ELEC_ENERGY_TOL", CellValue::Array(array_contents))
    }
}

impl ToCellValue for ElecEnergyTol {
    fn to_cell_value(&self) -> CellValue {
        let mut array_contents = vec![CellValue::Float(self.value)];
        if let Some(unit) = &self.unit {
            array_contents.push(unit.to_cell_value());
        }
        CellValue::Array(array_contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_energy_tol_serde() {
        // 1. Test Deserialization with unit
        let elec_energy_tol_with_unit_str = "ELEC_ENERGY_TOL : 0.00007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEnergyTolUnit {
            elec_energy_tol: ElecEnergyTol,
        }

        let cell_file_result: Result<CellFileWithElecEnergyTolUnit, _> = from_str(elec_energy_tol_with_unit_str);
        assert!(cell_file_result.is_ok(), "Deserialization (with unit) failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.elec_energy_tol.value - 0.00007).abs() < 1e-10);
        assert_eq!(cell_file.elec_energy_tol.unit, Some(EnergyUnit::ElectronVolt));


        // 2. Test Deserialization without unit (default unit implied)
        let elec_energy_tol_default_str = "ELEC_ENERGY_TOL : 0.00001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEnergyTolDefault {
            elec_energy_tol: ElecEnergyTol,
        }

        let cell_file_default_result: Result<CellFileWithElecEnergyTolDefault, _> = from_str(elec_energy_tol_default_str);
        assert!(cell_file_default_result.is_ok(), "Deserialization (default unit) failed: {:?}", cell_file_default_result.err());
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.elec_energy_tol.value - 0.00001).abs() < 1e-10);
        assert_eq!(cell_file_default.elec_energy_tol.unit, None);


        // 3. Test Serialization using ToCell (with unit)
        let elec_energy_tol_instance_with_unit = ElecEnergyTol {
            value: 1e-5,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&elec_energy_tol_instance_with_unit.to_cell());
        assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized ELEC_ENERGY_TOL (1e-5 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("ELEC_ENERGY_TOL"));
        assert!(serialized_string_with_unit.contains("1e-5") || serialized_string_with_unit.contains("0.00001"));
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let elec_energy_tol_instance_no_unit = ElecEnergyTol {
            value: 2e-5,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&elec_energy_tol_instance_no_unit.to_cell());
        assert!(serialized_result_no_unit.is_ok(), "Serialization (no unit) failed: {:?}", serialized_result_no_unit.err());
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized ELEC_ENERGY_TOL (2e-5, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("ELEC_ENERGY_TOL"));
        assert!(serialized_string_no_unit.contains("2e-5") || serialized_string_no_unit.contains("0.00002"));
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
```

```rust
// File: electronic_minimisation/elec_eigenvalue_tol.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::energy_unit::EnergyUnit;

/// Controls the tolerance for accepting convergence of a single eigenvalue during density mixing minimization.
///
/// Keyword type: Real
///
/// Default: The lower of 1e-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS
///
/// Example:
/// ELEC_EIGENVALUE_TOL : 0.000007 eV
/// ELEC_EIGENVALUE_TOL : 0.000007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "ELEC_EIGENVALUE_TOL")]
#[serde(from = "ElecEigenvalueTolRepr")] // Use intermediate repr for deserialization
pub struct ElecEigenvalueTol {
    /// The eigenvalue tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `ElecEigenvalueTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ElecEigenvalueTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<ElecEigenvalueTolRepr> for ElecEigenvalueTol {
    fn from(repr: ElecEigenvalueTolRepr) -> Self {
        match repr {
            ElecEigenvalueTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            ElecEigenvalueTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for ElecEigenvalueTol {
    fn to_cell(&self) -> Cell {
        // Create a CellValue::Array containing the value and optionally the unit
        let mut array_contents = vec![CellValue::Float(self.value)];
        if let Some(unit) = &self.unit {
            array_contents.push(unit.to_cell_value());
        }
        Cell::KeyValue("ELEC_EIGENVALUE_TOL", CellValue::Array(array_contents))
    }
}

impl ToCellValue for ElecEigenvalueTol {
    fn to_cell_value(&self) -> CellValue {
        let mut array_contents = vec![CellValue::Float(self.value)];
        if let Some(unit) = &self.unit {
            array_contents.push(unit.to_cell_value());
        }
        CellValue::Array(array_contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_eigenvalue_tol_serde() {
        // 1. Test Deserialization with unit
        let elec_eigenvalue_tol_with_unit_str = "ELEC_EIGENVALUE_TOL : 0.000007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEigenvalueTolUnit {
            elec_eigenvalue_tol: ElecEigenvalueTol,
        }

        let cell_file_result: Result<CellFileWithElecEigenvalueTolUnit, _> = from_str(elec_eigenvalue_tol_with_unit_str);
        assert!(cell_file_result.is_ok(), "Deserialization (with unit) failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.elec_eigenvalue_tol.value - 0.000007).abs() < 1e-10);
        assert_eq!(cell_file.elec_eigenvalue_tol.unit, Some(EnergyUnit::ElectronVolt));


        // 2. Test Deserialization without unit (default unit implied)
        let elec_eigenvalue_tol_default_str = "ELEC_EIGENVALUE_TOL : 0.000001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEigenvalueTolDefault {
            elec_eigenvalue_tol: ElecEigenvalueTol,
        }

        let cell_file_default_result: Result<CellFileWithElecEigenvalueTolDefault, _> = from_str(elec_eigenvalue_tol_default_str);
        assert!(cell_file_default_result.is_ok(), "Deserialization (default unit) failed: {:?}", cell_file_default_result.err());
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.elec_eigenvalue_tol.value - 0.000001).abs() < 1e-10);
        assert_eq!(cell_file_default.elec_eigenvalue_tol.unit, None);


        // 3. Test Serialization using ToCell (with unit)
        let elec_eigenvalue_tol_instance_with_unit = ElecEigenvalueTol {
            value: 1e-6,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&elec_eigenvalue_tol_instance_with_unit.to_cell());
        assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized ELEC_EIGENVALUE_TOL (1e-6 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("ELEC_EIGENVALUE_TOL"));
        assert!(serialized_string_with_unit.contains("1e-6") || serialized_string_with_unit.contains("0.000001"));
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let elec_eigenvalue_tol_instance_no_unit = ElecEigenvalueTol {
            value: 2e-6,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&elec_eigenvalue_tol_instance_no_unit.to_cell());
        assert!(serialized_result_no_unit.is_ok(), "Serialization (no unit) failed: {:?}", serialized_result_no_unit.err());
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized ELEC_EIGENVALUE_TOL (2e-6, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("ELEC_EIGENVALUE_TOL"));
        assert!(serialized_string_no_unit.contains("2e-6") || serialized_string_no_unit.contains("0.000002"));
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
```

```rust
// File: electronic_minimisation/elec_dump_file.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the name of the file into which wavefunction and density data are written.
///
/// Keyword type: String
///
/// Default: seedname.wvfn
///
/// Example:
/// ELEC_DUMP_FILE : test.wvfn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELEC_DUMP_FILE")]
pub struct ElecDumpFile(pub String); // Could be an enum Option<String> or similar for NULL handling

impl ToCell for ElecDumpFile {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_DUMP_FILE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for ElecDumpFile {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_dump_file_serde() {
        let elec_dump_file_str = "ELEC_DUMP_FILE : test.wvfn";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecDumpFile {
            elec_dump_file: ElecDumpFile,
        }

        let cell_file_result: Result<CellFileWithElecDumpFile, _> = from_str(elec_dump_file_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.elec_dump_file.0, "test.wvfn");

        let elec_dump_file_instance = ElecDumpFile("backup.wvfn".to_string());
        let serialized_result = to_string(&elec_dump_file_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELEC_DUMP_FILE (backup.wvfn): {serialized_string}");
        assert!(serialized_string.contains("ELEC_DUMP_FILE"));
        assert!(serialized_string.contains("backup.wvfn"));
    }
}
```

```rust
// File: electronic_minimisation/elec_convergence_win.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window during a electronic minimization run.
///
/// Keyword type: Integer
///
/// Default: 3
///
/// Example:
/// ELEC_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELEC_CONVERGENCE_WIN")]
pub struct ElecConvergenceWin(pub i32); // Should be >= 2

impl Default for ElecConvergenceWin {
    fn default() -> Self {
        Self(3) // Default is 3
    }
}

impl ToCell for ElecConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_CONVERGENCE_WIN", CellValue::Int(self.0))
    }
}

impl ToCellValue for ElecConvergenceWin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_convergence_win_serde() {
        let elec_convergence_win_str = "ELEC_CONVERGENCE_WIN : 4";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecConvergenceWin {
            elec_convergence_win: ElecConvergenceWin,
        }

        let cell_file_result: Result<CellFileWithElecConvergenceWin, _> = from_str(elec_convergence_win_str);
        assert!(cell_file_result.is_ok(), "Deserialization failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.elec_convergence_win.0, 4);

        let elec_convergence_win_instance = ElecConvergenceWin(5);
        let serialized_result = to_string(&elec_convergence_win_instance.to_cell());
        assert!(serialized_result.is_ok(), "Serialization failed: {:?}", serialized_result.err());
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELEC_CONVERGENCE_WIN (5): {serialized_string}");
        assert!(serialized_string.contains("ELEC_CONVERGENCE_WIN"));
        assert!(serialized_string.contains("5"));

        assert_eq!(ElecConvergenceWin::default(), ElecConvergenceWin(3));
    }
}
```

```rust
// File: electronic_minimisation/efemi_tol.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::energy_unit::EnergyUnit;

/// Controls the tolerance for accepting convergence of the Fermi-energy.
///
/// Keyword type: Real
///
/// Default: 0.1 × ELEC_EIGENVALUE_TOL
///
/// Example:
/// EFERMI_TOL : 0.0000007 eV
/// EFERMI_TOL : 0.0000007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "EFERMI_TOL")]
#[serde(from = "EFermiTolRepr")] // Use intermediate repr for deserialization
pub struct EFermiTol {
    /// The Fermi energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `EFermiTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EFermiTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<EFermiTolRepr> for EFermiTol {
    fn from(repr: EFermiTolRepr) -> Self {
        match repr {
            EFermiTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            EFermiTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for EFermiTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFERMI_TOL", self.to_cell_value())
    }
}

impl ToCellValue for EFermiTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]
            .to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{from_str, to_string, ToCell};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_efermi_tol_serde() {
        // 1. Test Deserialization with unit
        let efermi_tol_with_unit_str = "EFERMI_TOL : 0.0000007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEFermiTolUnit {
            efermi_tol: EFermiTol,
        }

        let cell_file_result: Result<CellFileWithEFermiTolUnit, _> = from_str(efermi_tol_with_unit_str);
        assert!(cell_file_result.is_ok(), "Deserialization (with unit) failed: {:?}", cell_file_result.err());
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.efermi_tol.value - 0.0000007).abs() < 1e-10);
        assert_eq!(cell_file.efermi_tol.unit, Some(EnergyUnit::ElectronVolt));


        // 2. Test Deserialization without unit (default unit implied)
        let efermi_tol_default_str = "EFERMI_TOL : 0.0000001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEFermiTolDefault {
            efermi_tol: EFermiTol,
        }

        let cell_file_default_result: Result<CellFileWithEFermiTolDefault, _> = from_str(efermi_tol_default_str);
        assert!(cell_file_default_result.is_ok(), "Deserialization (default unit) failed: {:?}", cell_file_default_result.err());
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.efermi_tol.value - 0.0000001).abs() < 1e-10);
        assert_eq!(cell_file_default.efermi_tol.unit, None);


        // 3. Test Serialization using ToCell (with unit)
        let efermi_tol_instance_with_unit = EFermiTol {
            value: 1e-7,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&efermi_tol_instance_with_unit.to_cell());
        assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized EFERMI_TOL (1e-7 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("EFERMI_TOL"));
        assert!(serialized_string_with_unit.contains("1e-7") || serialized_string_with_unit.contains("0.0000001"));
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let efermi_tol_instance_no_unit = EFermiTol {
            value: 2e-7,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&efermi_tol_instance_no_unit.to_cell());
        assert!(serialized_result_no_unit.is_ok(), "Serialization (no unit) failed: {:?}", serialized_result_no_unit.err());
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized EFERMI_TOL (2e-7, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("EFERMI_TOL"));
        assert!(serialized_string_no_unit.contains("2e-7") || serialized_string_no_unit.contains("0.0000002"));
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
```
