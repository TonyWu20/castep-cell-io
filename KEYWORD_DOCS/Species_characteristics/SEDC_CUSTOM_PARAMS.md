# SEDC_CUSTOM_PARAMS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_custom_params_castep.htm

**Group:** Species characteristics

---

# SEDC\_CUSTOM\_PARAMS (.cell)

## Keyword type

Block

## Description

This data block defines customized parameters for semi-empirical dispersion corrections.

The data block has the following format:

```
%BLOCK SEDC_CUSTOM_PARAMS
 UNITS
	atom1  parameter1 parameter2 ....
	atom2  parameter1 parameter2 ....
	.
	.
	.
%ENDBLOCK SEDC_CUSTOM_PARAMS
```

The first (optional) line declares the energy and length units used for the values. If no units are given then eV and Å are the defaults. Some of the derived units in the block will be based on the energy and length units specified.

Each following line describes a full set of parameters for an atom type. The first item on each obligatory line (atom) describes the ion and contains the species' ion symbol. The same custom parameter values will be applied to all ions with this name.

After an ion is specified, all atomic parameters for each orbital of the ion must be defined, as required by the value of [SEDC\_SCHEME](k_sedc_scheme_castep.htm) . The format is:

```
l:V
```

where l is C6, R0, alpha, I, or Neff (depending on the value of [SEDC\_SCHEME](k_sedc_scheme_castep.htm)) and V is a real number, for example:

```
C6:0.0
```

All values which are not defined are taken from the scheme defaults.

## Example

```
%BLOCK SEDC_CUSTOM_PARAMS
H C6:0.0 R0:1.6404 alpha:0.6668
%ENDBLOCK SEDC_CUSTOM_PARAMS
```

This input turns off the dispersion interaction for all hydrogen atoms.

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)