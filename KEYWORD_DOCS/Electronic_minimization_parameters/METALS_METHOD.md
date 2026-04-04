# METALS_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_metals_method_castep.htm

**Group:** Electronic minimization parameters

---

# METALS\_METHOD (.param)

## Keyword type

String

## Description

This keyword determines the  [electronic minimization](../thcastepselfelec.htm) method used in the self-consistent calculation. In spite
of the term "METALS" in the name of the parameter its usage is the same for metals ([FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) :
FALSE) and semiconductors ([FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) : TRUE). Available options are:

* DM - system treated by [density mixing](../dlgcastepdensmix.htm)  (in this case density mixing
  parameters are printed in the `.castep` file).
* EDFT - system treated by ensemble density functional method: CASTEP does a self-consistent all-bands
  wavefunction search, which for metals is followed by the self-consistent updating of occupancies.
* NONE - currently not used: it is not supported for FIX\_OCCUPANCY : FALSE and is the same as
  EDFT for FIX\_OCCUPANCY : TRUE.

## Default

EDFT

[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) : FALSE, the default is DM

## Example

```

METALS_METHOD : dm
```

###### See Also:

[SCF - Electronic Options](../dlgcastepelecoptscf.htm)
  
[Setting up SCF parameters](../tskcastepsetelecscf.htm)
  
[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)