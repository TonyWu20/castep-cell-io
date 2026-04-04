# FINITE_BASIS_CORR

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_finite_basis_corr_castep.htm

**Group:** Basis set parameters

---

# FINITE\_BASIS\_CORR (.param)

## Keyword type

Integer

## Description

This keyword determines whether or not to apply a finite basis set correction to energy and stress when cell parameters change. Available
options are:

* 0 - no correction.
* 1 - manual correction using specified [BASIS\_DE\_DLOGE](k_basis_de_dloge_castep.htm).
* 2 - automatic correction using [FINITE\_BASIS\_NPOINTS](k_finite_basis_npoints_castep.htm) and [FINITE\_BASIS\_SPACING](k_finite_basis_spacing_castep.htm).

If FINITE\_BASIS\_CORR : 1, a value for [BASIS\_DE\_DLOGE](k_basis_de_dloge_castep.htm)
must be supplied.

You should turn off finite basis set correction when using a fixed size basis (see
[FIXED\_NPW](k_fixed_npw_castep.htm)).

## Default

2

## Example

```

FINITE_BASIS_CORR : 1
```

###### See Also:

[BASIS\_DE\_DLOGE](k_basis_de_dloge_castep.htm)
  
[FINITE\_BASIS\_NPOINTS](k_finite_basis_npoints_castep.htm)
  
[FINITE\_BASIS\_SPACING](k_finite_basis_spacing_castep.htm)
  
[FIXED\_NPW](k_fixed_npw_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)