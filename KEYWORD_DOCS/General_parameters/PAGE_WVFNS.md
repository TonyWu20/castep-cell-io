# PAGE_WVFNS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_page_wvfns_castep.htm

**Group:** General parameters

---

# PAGE\_WVFNS (.param)

## Keyword type

Integer

## Description

This keyword controls the paging of wavefunctions to disk in order to save memory. Available options are:

* > 0 - all wavefunctions requiring more memory than this value in megabytes will be paged to disk.
* 0 - no paging will be performed.
* < 0 - all wavefunctions will be paged to disk.

## Default

0

## Example

```

PAGE_WVFNS : 2
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
