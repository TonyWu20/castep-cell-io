# PHONON_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_method_castep.htm

**Group:** Phonon parameters

---

# SECONDD\_METHOD, PHONON\_METHOD (.param)

## Keyword type

String

## Description

This keyword specifies which calculation method will be used for phonons. SECONDD\_METHOD is the latest form of this keyword, while the
obsolete version, PHONON\_METHOD, is supported for backward compatibility. Available options are:

* LINEARRESPONSE (or DFPT) - Linear response method (or density functional perturbation theory)
* FINITEDISPLACEMENT - Finite displacement method

The finite displacement method produces phonons at the Γ point only.

Using the linear response method imposes restrictions on the calculation settings.
Specifically, norm-conserving pseudopotentials must be used, with fixed occupation numbers and without spin polarization.

## Default

LINEARRESPONSE

## Example

```

SECONDD_METHOD : FINITEDISPLACEMENT
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)