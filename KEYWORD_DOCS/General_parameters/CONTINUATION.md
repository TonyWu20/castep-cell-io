# CONTINUATION

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_continuation_castep.htm

**Group:** General parameters

---

# CONTINUATION (.param)

## Keyword type

String

## Description

This keyword contains a string which specifies the model file used to continue the job.

DEFAULT (case-insensitive) means that the file
*seedname*.check will be used. The same effect can be achieved by providing the CONTINUATION keyword on its
own, with no value.

A run cannot be both a [REUSE](k_reuse_castep.htm) and a CONTINUATION. A
CONTINUATION run restarts the current job from the end of the previous one. As a consequence, only a limited subset of the available parameters
may be changed between CONTINUATION runs.

## Default

NULL - corresponds to no continuation (a new run)

## Example

```

CONTINUATION : DEFAULT
```

###### See Also:

[REUSE](k_reuse_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)