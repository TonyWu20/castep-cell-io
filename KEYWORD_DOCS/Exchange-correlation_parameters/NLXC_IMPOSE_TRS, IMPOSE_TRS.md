# NLXC_IMPOSE_TRS, IMPOSE_TRS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_impose_trs_castep.htm

**Group:** Exchange-correlation parameters

---

# NLXC\_IMPOSE\_TRS or IMPOSE\_TRS (.param)

## Keyword type

Logical

## Description

This keyword imposes time reversal symmetry on the gamma point wavefunctions. Time reversal symmetry is a necessary property of any
Kohn-Sham orbital. NLXC\_EXCHANGE\_REFLECT\_KPTS is the latest form of this keyword, while the obsolete version, EXCHANGE\_REFLECT\_KPTS, is
supported for backward compatibility.

In the reciprocal space representation, time reversal symmetry means that the coefficient of plane wave |k + g> must be the conjugate of
the coefficient of plane wave |-k - g>. With the standard LDA and GGA exchange-correlation functionals, the absence of such symmetry does
not affect the total energies or densities and can therefore be ignored. However, when using exact or screened exchange, the total energy may
be incorrect if time reversal symmetry is not present. Also, the efficiency of calculation of the exchange operator is significantly improved
by assuming time reversal symmetry (see also [NLXC\_EXCHANGE\_REFLECT\_KPTS](k_exchange_reflect_kpts_castep.htm).

## Default

FALSE

## Example

```

NLXC_IMPOSE_TRS : TRUE
```

###### See Also:

[NLXC\_EXCHANGE\_REFLECT\_KPTS](k_exchange_reflect_kpts_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
