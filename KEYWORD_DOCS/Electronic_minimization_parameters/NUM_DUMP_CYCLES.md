# NUM_DUMP_CYCLES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_num_dump_cycles_castep.htm

**Group:** Electronic minimization parameters

---

# NUM\_DUMP\_CYCLES (.param)

## Keyword type

Integer

## Description

This keyword specifies the number of SCF cycles between updates to the wavefunction and density data file. If NUM\_DUMP\_CYCLES is less than or
equal to 0, no data will be written to this file.

## Default

5

## Example

```

NUM_DUMP_CYCLES : 10
```

###### See Also:

[ELEC\_DUMP\_FILE](k_elec_dump_file_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
