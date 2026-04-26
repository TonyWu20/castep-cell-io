# NLXC_PAGE_EX_POT, PAGE_EX_POT

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_page_ex_pot_castep.htm

**Group:** Exchange-correlation parameters

---

# NLXC\_PAGE\_EX\_POT or PAGE\_EX\_POT (.param)

## Keyword type

Integer

## Description

This keyword controls how exchange potentials are paged to disk and can be used to save memory. The currently used version of the keyword
name is NLXC\_PAGE\_EX\_POT, while the obsolete version, PAGE\_EX\_POT, is supported for backward compatibility. Available options are:

* > 0 - All exchange potentials requiring more memory than this value in megabytes will be paged to disk.
* 0 - No paging will be performed.
* < 0 - All exchange potentials will be paged disk.

This keyword is only relevant when [XC\_FUNCTIONAL](k_xc_functional_castep.htm)
has a value of HF, sX, HF-LDA, or
sX-LDA.

## Default

0

## Example

```

NLXC_PAGE_EX_POT : 20
```

###### See Also:

[XC\_FUNCTIONAL](k_xc_functional_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
