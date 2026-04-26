# SPECTRAL_TASK

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_spectral_task.htm

**Group:** Electronic excitations parameters

---

# SPECTRAL\_TASK (.param)

## Keyword type

String

## Description

This keyword defines the type of electronic spectroscopy calculation to be performed. Available options are:

* BandStructure - calculates band structure.
* DOS - calculates density of states.
* Optics - performs a TD-DFT calculation of excitation energies and optical spectra.
* CoreLoss - performs core level spectroscopy calculation.
* All - performs all of the above calculations.

## Default

BandStructure

## Example

```

SPECTRAL_TASK : optics
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
