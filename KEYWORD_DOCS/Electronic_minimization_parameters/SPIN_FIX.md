# SPIN_FIX

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_spin_fix_castep.htm

**Group:** Electronic minimization parameters

---

# SPIN\_FIX (.param)

## Keyword type

Integer

## Description

This keyword determines the number of electronic iterations for which the total spin is fixed. If SPIN\_FIX is less than
0, the spin will be fixed for the duration of the calculation.

This keyword only effects the behavior of the electronic minimizer in the initial SCF calculation.
There is a separate keyword, [GEOM\_SPIN\_FIX](k_geom_spin_fix_castep.htm), which
should be used to fix the spin of the system during geometry optimization.

This parameter is only used if [FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) :
FALSE. So, for insulators the spin is fixed regardless of the value of SPIN\_FIX.

## Default

10

## Example

```

SPIN_FIX : 5
```

###### See Also:

[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[GEOM\_SPIN\_FIX](k_geom_spin_fix_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)