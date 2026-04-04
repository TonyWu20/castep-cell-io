# RAND_SEED

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_rand_seed_castep.htm

**Group:** General parameters

---

# RAND\_SEED (.param)

## Keyword type

Integer (expert)

## Description

This keyword controls the initialization of random number seeds. Available options are:

* 0 - the seed for the random number generation is selected pseudorandomly.
* > or < 0 - this value is used as a seed for the random number generator.

## Default

0

## Example

```

RAND_SEED : -25
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)