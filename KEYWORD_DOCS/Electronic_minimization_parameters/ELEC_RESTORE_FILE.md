# ELEC_RESTORE_FILE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_elec_restore_file_castep.htm

**Group:** Electronic minimization parameters

---

# ELEC\_RESTORE\_FILE (.param)

## Keyword type

String

## Description

This keyword specifies the name of the file from which wavefunction and density data should be restored when performing a
[CONTINUATION](k_continuation_castep.htm) or a [REUSE](k_reuse_castep.htm) run.

NULL means that wavefunction and density data will not be restored.

The basis set and distribution for the new run must be the same as those from the run in
which the data file was written.

## Default

NULL

## Example

```

ELEC_RESTORE_FILE : test.wvfn
```

###### See Also:

[ELEC\_DUMP\_FILE](k_elec_dump_file_castep.htm)
  
[CONTINUATION](k_continuation_castep.htm)
  
[REUSE](k_reuse_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)