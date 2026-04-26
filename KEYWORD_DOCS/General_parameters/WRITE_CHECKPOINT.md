# WRITE_CHECKPOINT

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_write_checkpoint_castep.htm

**Group:** General parameters

---

# WRITE\_CHECKPOINT(.param)

## Keyword type

String

## Description

Specifies whether or not to write both `seedname.check`and `seedname.castep_bin` files, and (optionally) fine control on what to write for periodic backup, failure or success.

Allowed values:

* NONE
* MINIMAL or BOTH
* ALL or FULL

Options:

* SUCCESS=
* FAILURE=
* BACKUP=

Only one of these options can be specified (that is, it is not possible to request different behavior, for example for successful termination and for the failed calculations.

If, for example, an option is requested for a failed calculation, the default behavior will be used in backups and in the case of failure; that is, both `seedname.check`and `seedname.castep_bin` files will be produced at intermediate backups and in the case of a job completed with a failure termination.

## Default

ALL

## Example

```

WRITE_CHECKPOINT : SUCCESS=BOTH
```

```

WRITE_CHECKPOINT : MINIMAL
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
