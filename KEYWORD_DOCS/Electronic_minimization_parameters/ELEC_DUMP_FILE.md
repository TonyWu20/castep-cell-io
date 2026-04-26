# ELEC_DUMP_FILE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_elec_dump_file_castep.htm

**Group:** Electronic minimization parameters

---

# ELEC\_DUMP\_FILE (.param)

## Keyword type

String

## Description

This keyword determines the name of the file into which wavefunction and density data are written, periodically during electronic minimization.
This file can be used as a backup and is restored with the [ELEC\_RESTORE\_FILE](k_elec_restore_file_castep.htm) parameter.

If this parameter is set to NULL, no backup wavefunction or
density data will be written.

## Default

*seedname*.wvfn

## Example

```

ELEC_DUMP_FILE : test.wvfn
```

###### See Also:

[ELEC\_RESTORE\_FILE](k_elec_restore_file_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
