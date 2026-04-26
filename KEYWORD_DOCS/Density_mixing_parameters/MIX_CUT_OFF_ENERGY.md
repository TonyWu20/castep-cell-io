# MIX_CUT_OFF_ENERGY

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_mix_cut_off_energy_castep.htm

**Group:** Density mixing parameters

---

# MIX\_CUT\_OFF\_ENERGY (.param)

## Keyword type

Real

## Description

This keyword determines the cutoff energy for the densities used in the density mixing scheme.

The value specified determines the extent of the grid used for mixing old and new densities. Density components with wave vectors higher
than that specified are not mixed.

## Default

The default value is the [CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm).

## Example

```

MIX_CUT_OFF_ENERGY : 250.0 eV
```

###### See Also:

[CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
