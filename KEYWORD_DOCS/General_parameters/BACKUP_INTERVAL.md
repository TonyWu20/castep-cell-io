# BACKUP_INTERVAL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_backup_iter_castep.htm

**Group:** General parameters

---

# BACKUP\_INTERVAL (.param)

## Keyword type

Integer

## Description

This keyword specifies the interval, in seconds, between updates of the backup restart files. This keyword is applicable for geometry
optimization, molecular dynamics, phonon or transition state search runs. A value which is less than or equal to zero indicates that no updates
will be performed.

This keyword cannot be used in conjunction with
[NUM\_BACKUP\_ITER](k_num_backup_iter_castep.htm).

## Default

0

## Example

```

BACKUP_INTERVAL : 3600
```

###### See Also:

[NUM\_BACKUP\_ITER](k_num_backup_iter_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)