# REUSE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_reuse_castep.htm

**Group:** General parameters

---

# REUSE (.param)

## Keyword type

String

## Description

This keyword contains a string which specifies the model file from which data will be read to initialize a new calculation. If no file is to
be used, this string should be NULL, which means no reuse (a new run).

DEFAULT (case-insensitive) will use the data from the
*seedname*.check file. The same effect can be achieved by providing the REUSE keyword on its own, with
no value.

A run cannot be both a REUSE and a [CONTINUATION](k_continuation_castep.htm). A
REUSE run is a new calculation that uses as much as data as possible from a previous calculation.

## Default

NULL

## Example

```

REUSE : DEFAULT
```

###### See Also:

[CONTINUATION](k_continuation_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)