# RUN_TIME

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_run_time_castep.htm

**Group:** General parameters

---

# RUN\_TIME (.param)

## Keyword type

Integer

## Description

This keyword specifies the maximum run time for the job, in seconds. If the RUN\_TIME is greater than zero, the job will exit cleanly before
the specified time has elapsed, leaving as little unused time as possible.

Clean termination before RUN\_TIME cannot be guaranteed. The shortest operation which can be
timed is an electronic minimization or a single molecular dynamics or geometry optimization step.

If RUN\_TIME is less than or equal to zero, no time limit will be imposed on the run.

## Default

0

## Example

```

RUN_TIME : 360
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)