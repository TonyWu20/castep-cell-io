# OPT_STRATEGY

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_opt_strategy_castep.htm

**Group:** General parameters

---

# OPT\_STRATEGY (.param)

## Keyword type

String

## Description

This parameter determines the optimization strategy used when there are multiple strategies available for the selected algorithm and they
have differing costs in terms of memory usage and performance. Available options are:

* Speed - uses the optimization strategy which maximizes performance at the cost of additional memory usage.
* Default - uses the optimization strategy that best balances performance and memory usage.
* Memory - uses the optimization strategy which minimizes memory usage at a cost of decreased performance.

## Default

Default

## Example

```

OPT_STRATEGY : Memory
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)