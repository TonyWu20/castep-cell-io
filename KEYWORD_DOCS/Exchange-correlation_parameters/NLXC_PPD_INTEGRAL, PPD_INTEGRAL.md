# NLXC_PPD_INTEGRAL, PPD_INTEGRAL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ppd_integral_castep.htm

**Group:** Exchange-correlation parameters

---

# NLXC\_PPD\_INTEGRAL or PPD\_INTEGRAL (.param)

## Keyword type

Logical

## Description

This keyword enables you to use parallelepiped integration to effectively smear out each k-point used in the SCF calculation onto a
parallelepiped whose dimensions are determined by the spacing of the Monkhorst-Pack grid. This technique is useful for band structure
calculations with exact or screened exchange functionals if the band structure path passes very close to one of the SCF k-points. The
NLXC\_PPD\_INTEGRAL is the latest form of this keyword, while the obsolete version, PPD\_INTEGRAL, is supported for backward compatibility.

If the band structure plot along high symmetry directions shows unexpected sharp dips, it
is possible that more accurate results will be obtained with the parallelepiped integration.

## Default

FALSE

## Example

```

NLXC_PPD_INTEGRAL : TRUE
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
