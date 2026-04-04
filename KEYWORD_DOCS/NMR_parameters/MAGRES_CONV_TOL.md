# MAGRES_CONV_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magres_conv_tol_castep.htm

**Group:** NMR parameters

---

# MAGRES\_CONV\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the first-order perturbed wavefunctions in the electronic minimizer during
an NMR calculation.

## Default

1 × 10-12 when [MAGRES\_METHOD](k_magres_method_castep.htm) :
crystal

1 × 10-9 when [MAGRES\_METHOD](k_magres_method_castep.htm) : molecular

## Example

```

MAGRES_CONV_TOL  = 0.00007 eV
```

To obtain reliable shielding constants, you must ensure a high convergence of the
first-order perturbed wavefunctions. However, there are rare cases where the convergence to within the default accuracy cannot be achieved with
a reasonable number of iterations. In such situations, a warning message will report the minimal residual (largest over the
k-points) achieved by the minimizer, for example:
  
  
`Calculation of first order wavefunctions not fully converged. Minimum residual = 0.285E-11 eV`  
  
In this case, the program does not proceed with the calculation of NMR parameters and the decision is left up to you. If the minimal
residual is still reasonably small, as in the example above, it makes sense to run the calculation to completion. To do that, you have to
change the MAGRES\_CONV\_TOL value by setting it in the `.param` file to a value larger than that in the warning message.
If MAGRES\_CONV\_TOL has to be increased by more than one order of magnitude, it is advisable to study the dependence of the NMR
parameters on the value of this computational parameter in order to confirm that the physically relevant results are stable to variations in
MAGRES\_CONV\_TOL.

###### See Also:

[MAGRES\_METHOD](k_magres_method_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)