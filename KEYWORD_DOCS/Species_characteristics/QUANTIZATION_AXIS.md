# QUANTIZATION_AXIS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_quantization_axis_castep.htm

**Group:** Species characteristics

---

# QUANTIZATION\_AXIS (.cell)

## Keyword type

Real Vector

## Description

This keyword defines the quantization (magnetization) axis to be taken for LDA+U calculations.

The keyword has the following format:

```
QUANTIZATION_AXIS : fa fb fc
```

where fa fb fc are fractional coordinates for the chosen direction. If this data is missing CASTEP determines the best appropriate quantization
axis, trying to find the most symmetric axis in the crystal.

If an LDA+U calculation is not requested, the quantization axis is ignored.

## Default

The fractional coordinates equivalent to the c axis.

## Example

```
QUANTIZATION_AXIS : 1 1 -1
```

This input defines the quantization axis ( 1 1 -1 ) which is most appropriate for a fcc structure (along one of the fourth order axes).

###### See Also:

[HUBBARD\_U](k_hubbard_u_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)