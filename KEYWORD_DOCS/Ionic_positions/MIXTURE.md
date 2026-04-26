# MIXTURE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_mixture_castep.htm

**Group:** Ionic positions

---

# MIXTURE (.cell)

## Keyword type

Qualifier

## Description

This keyword can be added to any line containing atomic coordinates in either [POSITIONS\_ABS](k_positions_abs_castep.htm) or
[POSITIONS\_FRAC](k_positions_frac_castep.htm) data blocks to specify the characteristics of the component atoms in a disordered
system. The format of the MIXTURE entry contains the index of the mixture atom and the weight associated with the given component. The example
below contains three Al/Si mixture atoms for the 2:1 Al:Si ratio.

The following separators are allowed: SPACE, TAB, =,
:, (, ).

## Example

```

%BLOCK POSITIONS_FRAC
    Al    0.2500000000    0.5000000000    0.0000000000 MIXTURE:(   1  0.666667)
    Al   -0.2500000000   -0.5000000000    0.0000000000 MIXTURE:(   2  0.666667)
    Al    0.2500000000    0.0000000000    0.5000000000 MIXTURE:(   3  0.666667)
    Si    0.2500000000    0.5000000000    0.0000000000 MIXTURE:(   1  0.333333)
    Si   -0.2500000000   -0.5000000000    0.0000000000 MIXTURE:(   2  0.333333)
    Si    0.2500000000    0.0000000000    0.5000000000 MIXTURE:(   3  0.333333)
%ENDBLOCK POSITIONS_FRAC
```

###### See Also:

[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
