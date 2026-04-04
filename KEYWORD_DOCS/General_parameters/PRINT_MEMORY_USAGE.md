# PRINT_MEMORY_USAGE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_print_memory_castep.htm

**Group:** General parameters

---

# PRINT\_MEMORY\_USAGE (.param)

## Keyword type

Logical

## Description

This keyword specifies whether CASTEP should print memory estimates during the run. These estimates may change during the execution, since
for example the size of the basis set can change as a result of a change in cell parameters. An updated estimates would be printed when this is
the case.

FALSE turns off the estimates.

## Default

TRUE.

## Example

```

PRINT_MEMORY_USAGE : FALSE
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)