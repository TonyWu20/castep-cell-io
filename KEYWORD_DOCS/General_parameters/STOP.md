# STOP

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_stop_castep.htm

**Group:** General parameters

---

# STOP (.param)

## Keyword type

Defined

## Description

This keyword, if present, will cause the current run to be aborted as if [RUN\_TIME](k_run_time_castep.htm) had been exceeded.

CASTEP checks the contents of the input file periodically during a run. This allows you to modify certain parameters and also to terminate
the run early.

This keyword is valid only when the input file is reread. It is ignored if it is present at
the start of a run.

## Example

```

STOP
```

###### See Also:

[RUN\_TIME](k_run_time_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
