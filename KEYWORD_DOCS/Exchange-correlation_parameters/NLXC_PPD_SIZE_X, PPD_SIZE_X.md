# NLXC_PPD_SIZE_X, PPD_SIZE_X

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ppd_size_castep.htm

**Group:** Exchange-correlation parameters

---

# NLXC\_PPD\_SIZE\_X, NLXC\_PPD\_SIZE\_Y, NLXC\_PPD\_SIZE\_Z or PPD\_SIZE\_X, PPD\_SIZE\_Y, PPD\_SIZE\_Z (.param)

## Keyword type

Integer

## Description

These keywords determine the number of points in the x, y, and z directions at which to sample the parallelepiped when performing
parallelepiped integration. The NLXC\_ is the latest form of these keywords, while the obsolete versions (PPD\_SIZE\_X, PPD\_SIZE\_Y, PPD\_SIZE\_Z)
are supported for backward compatibility.

These keywords must be used in conjunction with
[NLXC\_PPD\_INTEGRAL](k_ppd_integral_castep.htm).

Defaults

1

## Example

```

NLXC_PPD_SIZE_X : 4
NLXC_PPD_SIZE_Y : 2
NLXC_PPD_SIZE_Z : 2
```

###### See Also:

[NLXC\_PPD\_INTEGRAL](k_ppd_integral_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
