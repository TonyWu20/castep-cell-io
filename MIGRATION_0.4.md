# Migration Guide: v0.3 → v0.4

## Breaking Changes

### ParamDocument Structure

The flat `ParamDocument` structure has been refactored into nested sub-structs for better organization and maintainability.

## Before (v0.3)

```rust
use castep_cell_io::{ParamDocument, param::general::Task};

let doc = ParamDocument::builder()
    .task(Task::SinglePoint)
    .charge(1.0)
    .cutoff_energy(500.0)
    .xc_functional(XcFunctional::Pbe)
    .build();

// Field access
let task = doc.task;
let charge = doc.charge;
let cutoff = doc.cutoff_energy;
```

## After (v0.4)

```rust
use castep_cell_io::{
    ParamDocument, 
    GeneralParams, ElectronicParams, BasisSetParams, ExchangeCorrelationParams,
    param::general::Task,
    param::electronic::Charge,
    param::basis_set::CutOffEnergy,
    param::exchange_correlation::XcFunctional,
    units::energy_units::EnergyUnit,
};

let doc = ParamDocument::builder()
    .general(GeneralParams::builder()
        .task(Task::SinglePoint)
        .build())
    .electronic(ElectronicParams::builder()
        .charge(Charge(1.0))
        .build())
    .basis_set(BasisSetParams::builder()
        .cutoff_energy(CutOffEnergy { value: 500.0, unit: Some(EnergyUnit::Ev) })
        .build())
    .exchange_correlation(ExchangeCorrelationParams::builder()
        .xc_functional(XcFunctional::Pbe)
        .build())
    .build();

// Field access - direct
let task = doc.general.task;
let charge = doc.electronic.charge;
let cutoff = doc.basis_set.cutoff_energy;

// Field access - convenience methods (for common fields)
let task = doc.task();
let xc = doc.xc_functional();
let cutoff = doc.cutoff_energy();
```

## Field Groupings

All fields are now organized into these 18 groups:

| Group | Module | Common Fields |
|-------|--------|---------------|
| **general** | `GeneralParams` | `task`, `comment`, `continuation`, `charge_unit`, `iprint`, `rand_seed`, `run_time`, `checkpoint`, `reuse` |
| **electronic** | `ElectronicParams` | `charge`, `spin`, `nbands`, `nextra_bands`, `perc_extra_bands`, `nup`, `ndown`, `nelectrons`, SEDC parameters |
| **basis_set** | `BasisSetParams` | `cutoff_energy`, `grid_scale`, `fine_grid_scale`, `fine_gmax`, `basis_precision`, `fixed_npw`, finite basis parameters |
| **exchange_correlation** | `ExchangeCorrelationParams` | `xc_functional`, `spin_polarized`, `xc_definition`, NLXC parameters |
| **electronic_minimisation** | `ElectronicMinimisationParams` | `max_scf_cycles`, `elec_energy_tol`, `elec_eigenvalue_tol`, `metals_method`, `smearing_width`, `electronic_minimizer` |
| **geometry_optimization** | `GeometryOptimizationParams` | `geom_method`, `geom_max_iter`, `geom_energy_tol`, `geom_force_tol`, `geom_stress_tol`, `geom_disp_tol` |
| **phonon** | `PhononParams` | `phonon_method`, `phonon_fine_method`, `calculate_born_charges`, `born_charge_sum_rule`, phonon convergence parameters |
| **band_structure** | `BandStructureParams` | `bs_nbands`, `bs_nextra_bands`, `bs_perc_extra_bands`, `bs_xc_functional`, `bs_max_iter` |
| **molecular_dynamics** | `MolecularDynamicsParams` | `md_ensemble`, `md_temperature`, `md_delta_t`, `md_num_iter`, `md_thermostat`, `md_barostat`, MD equilibration parameters |
| **electric_field** | `ElectricFieldParams` | `efield_max_cycles`, `efield_energy_tol`, `efield_convergence_win`, `efield_calculate_nonlinear` |
| **pseudopotential** | `PseudopotentialParams` | `pspot_nonlocal_type`, `pspot_beta_phi_type`, `relativistic_treatment` |
| **density_mixing** | `DensityMixingParams` | `mixing_scheme`, `mix_history_length`, `mix_charge_amp`, `mix_spin_amp`, `mix_cut_off_energy` |
| **population_analysis** | `PopulationAnalysisParams` | `popn_calculate`, `popn_write`, `popn_bond_cutoff`, `pdos_calculate_weights` |
| **optics** | `OpticsParams` | `optics_nbands`, `optics_nextra_bands`, `optics_perc_extra_bands`, `optics_xc_functional` |
| **nmr** | `NmrParams` | `magres_task`, `magres_method`, `magres_max_cg_steps`, `magres_conv_tol` |
| **solvation** | `SolvationParams` | `boundary_type`, `dielec_emb_bulk_permittivity`, `implicit_solvent_*` parameters |
| **electronic_excitations** | `ElectronicExcitationsParams` | `spectral_task`, `tddft_num_states`, `tddft_selected_state`, `tddft_position_method` |
| **transition_state** | `TransitionStateParams` | `tssearch_method`, `tssearch_max_path_points`, `tssearch_*_tol` parameters |

## Migration Steps

### Step 1: Update Imports

Add imports for the parameter group types you need:

```rust
use castep_cell_io::{
    ParamDocument,
    GeneralParams,
    ElectronicParams,
    BasisSetParams,
    // ... other groups as needed
};
```

### Step 2: Update Builder Calls

Replace flat builder calls with nested builders:

```rust
// Old
let doc = ParamDocument::builder()
    .task(Task::SinglePoint)
    .cutoff_energy(500.0)
    .build();

// New
let doc = ParamDocument::builder()
    .general(GeneralParams::builder()
        .task(Task::SinglePoint)
        .build())
    .basis_set(BasisSetParams::builder()
        .cutoff_energy(CutOffEnergy { value: 500.0, unit: Some(EnergyUnit::Ev) })
        .build())
    .build();
```

### Step 3: Update Field Access

Change field access to use the group prefix:

```rust
// Old
let task = doc.task;
let energy = doc.cutoff_energy;

// New - direct access
let task = doc.general.task;
let energy = doc.basis_set.cutoff_energy;

// New - convenience methods (for common fields)
let task = doc.task();
let energy = doc.cutoff_energy();
```

### Step 4: Update Field Mutations

```rust
// Old
doc.task = Some(Task::GeometryOptimization);

// New
doc.general.task = Some(Task::GeometryOptimization);

// Or use convenience setter
doc.set_task(Task::GeometryOptimization);
```

## Convenience Methods

For frequently accessed fields, convenience methods are available:

```rust
// Getters
doc.task()                  // → doc.general.task
doc.xc_functional()         // → doc.exchange_correlation.xc_functional
doc.cutoff_energy()         // → doc.basis_set.cutoff_energy
doc.spin_polarized()        // → doc.exchange_correlation.spin_polarized
doc.max_scf_cycles()        // → doc.electronic_minimisation.max_scf_cycles
doc.geom_method()           // → doc.geometry_optimization.geom_method

// Setters
doc.set_task(task)
doc.set_xc_functional(xc)
doc.set_cutoff_energy(energy)
```

## Common Patterns

### Creating a Simple SCF Calculation

```rust
let doc = ParamDocument::builder()
    .general(GeneralParams::builder()
        .task(Task::SinglePoint)
        .build())
    .exchange_correlation(ExchangeCorrelationParams::builder()
        .xc_functional(XcFunctional::Pbe)
        .build())
    .basis_set(BasisSetParams::builder()
        .cutoff_energy(CutOffEnergy { value: 500.0, unit: Some(EnergyUnit::Ev) })
        .build())
    .build();
```

### Creating a Geometry Optimization

```rust
let doc = ParamDocument::builder()
    .general(GeneralParams::builder()
        .task(Task::GeometryOptimization)
        .build())
    .geometry_optimization(GeometryOptimizationParams::builder()
        .geom_method(GeomMethod::Bfgs)
        .geom_max_iter(GeomMaxIter(100))
        .build())
    .build();
```

### Using Default Groups

All parameter groups have `Default` implementations, so you can use defaults for groups you don't need to configure:

```rust
let doc = ParamDocument::builder()
    .general(GeneralParams::builder()
        .task(Task::SinglePoint)
        .build())
    // All other groups will use Default::default()
    .build();
```

## Benefits of the New Structure

1. **Better organization**: Related parameters are grouped together
2. **Improved type safety**: Smaller builders with proper type-state checking
3. **Easier navigation**: Find parameters by their logical category
4. **Better documentation**: Each group is self-documenting
5. **Faster compilation**: Smaller builders compile faster
6. **No experimental features**: Removed dependency on unstable bon features

## Need Help?

If you encounter issues during migration, please file an issue at:
https://github.com/TonyWu20/castep-cell-io/issues
