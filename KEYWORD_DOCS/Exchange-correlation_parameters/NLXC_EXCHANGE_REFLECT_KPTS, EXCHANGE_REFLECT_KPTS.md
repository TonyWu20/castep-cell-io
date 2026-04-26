# NLXC_EXCHANGE_REFLECT_KPTS, EXCHANGE_REFLECT_KPTS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_exchange_reflect_kpts_castep.htm

**Group:** Exchange-correlation parameters

---

# NLXC\_EXCHANGE\_REFLECT\_KPTS or EXCHANGE\_REFLECT\_KPTS (.param)

## Keyword type

Logical

## Description

This keyword specifies that only half of the Brillouin zone will be used for summation during the exact exchange potential calculation. The
NLXC\_EXCHANGE\_REFLECT\_KPTS is the latest form of this keyword, while the obsolete version, EXCHANGE\_REFLECT\_KPTS, is supported for backward
compatibility.

This is possible only if time reversal symmetry is assumed (see
[NLXC\_IMPOSE\_TRS](k_impose_trs_castep.htm)).

## Default

TRUE

## Example

```

NLXC_EXCHANGE_REFLECT_KPTS : TRUE
```

###### See Also:

[NLXC\_IMPOSE\_TRS](k_impose_trs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
