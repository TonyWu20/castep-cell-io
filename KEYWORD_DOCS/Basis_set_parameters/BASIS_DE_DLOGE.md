# BASIS_DE_DLOGE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_basis_de_dloge_castep.htm

**Group:** Basis set parameters

---

# BASIS\_DE\_DLOGE (.param)

## Keyword type

Real

## Description

This keyword specifies the derivative of total energy with respect to the natural log of the basis cutoff energy. The value is used only if
[FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm) : 1, which corresponds to manual finite basis set correction. If an automated
calculation of this quantity has already been performed, then for the next run on the same system, it is possible to reuse the value using the
BASIS\_DE\_DLOGE keyword. However, this type of correction has very limited applicability since both the system (that is, the atomic positions and
cell parameters) and the calculation parameters have to be exactly the same as in the initial run. In general, the automatic correction mode
([FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm) : 2) should be used.

The value of the total energy derivative should always be negative, since the total energy
decreases as the size of the basis increases.

CASTEP exits with an error if the value of BASIS\_DE\_DLOGE is not provided in the parameters
file when [FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm) : 1.

## Default

0.0

## Example

```

BASIS_DE_DLOGE : -1.2345 eV
```

###### See Also:

[FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)