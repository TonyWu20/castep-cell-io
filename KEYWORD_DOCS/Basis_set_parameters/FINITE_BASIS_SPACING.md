# FINITE_BASIS_SPACING

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_finite_basis_spacing_castep.htm

**Group:** Basis set parameters

---

# FINITE\_BASIS\_SPACING (.param)

## Keyword type

Real

## Description

This keyword determines the spacing of cutoff energies used to estimate the [BASIS\_DE\_DLOGE](k_basis_de_dloge_castep.htm) in
automatic calculation of the finite basis set correction. Points are chosen such that the [CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm)
corresponds to the highest energy in the set of [FINITE\_BASIS\_NPOINTS](k_finite_basis_npoints_castep.htm) cutoff energies.

This value is only used if [FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm) :
2.

## Default

5.0 eV

## Example

```

FINITE_BASIS_SPACING : 0.2 Ha
```

###### See Also:

[BASIS\_DE\_DLOGE](k_basis_de_dloge_castep.htm)
  
[CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm)
  
[FINITE\_BASIS\_NPOINTS](k_finite_basis_npoints_castep.htm)
  
[FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
