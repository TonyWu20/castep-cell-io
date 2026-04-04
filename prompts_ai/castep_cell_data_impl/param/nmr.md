These are submodules of module `nmr`
```
MAGRES_CONV_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the first-order perturbed wavefunctions in the electronic minimizer during
an NMR calculation.
Default
1 × 10 -12  when  MAGRES_METHOD  :
crystal
1 × 10 -9  when  MAGRES_METHOD  :  molecular
Example MAGRES_CONV_TOL  = 0.00007 eV
To obtain reliable shielding constants, you must ensure a high convergence of the
first-order perturbed wavefunctions. However, there are rare cases where the convergence to within the default accuracy cannot be achieved with
a reasonable number of iterations. In such situations, a warning message will report the minimal residual (largest over the
k-points) achieved by the minimizer, for example:
Calculation of first order wavefunctions not fully converged. Minimum residual = 0.285E-11 eV In this case, the program does not proceed with the calculation of NMR parameters and the decision is left up to you. If the minimal
residual is still reasonably small, as in the example above, it makes sense to run the calculation to completion. To do that, you have to
change the MAGRES_CONV_TOL value by setting it in the  .param  file to a value larger than that in the warning message.
If MAGRES_CONV_TOL has to be increased by more than one order of magnitude, it is advisable to study the dependence of the NMR
parameters on the value of this computational parameter in order to confirm that the physically relevant results are stable to variations in
MAGRES_CONV_TOL.
```
```
MAGRES_MAX_CG_STEPS (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of conjugate gradient steps taken for each electronic band in the electronic minimizer during an
NMR calculation of first-order perturbed wavefunctions.
NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if
you have purchased this module.
Apart from the initial steepest descent step, no further steepest descent steps are performed
during a calculation of first-order perturbed wavefunctions.
Default
250
Example MAGRES_MAX_CG_STEPS : 300
```
```
MAGRES_METHOD (.param)
Keyword type
String
Description
This keyword selects the method used by CASTEP for the evaluation of the quantum-mechanical position operator. Available options are:
Crystal  - uses the reciprocal space representation; applicable for both truly periodic crystals and a
"molecule in a box" supercell representation of a molecular system.
Molecular  - applicable only for a "molecule in a box" supercell representation of a molecular system. For
such a system, the  Molecular  option produces a noticeably faster calculation than the
Crystal  option.
Default
Crystal
Example MAGRES_METHOD : molecular
```
```
MAGRES_TASK (.param)
Keyword type
String
Description
This keyword defines the type of NMR calculation to be performed.
NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if
you have purchased this module.
Available options for the  MAGRES_TASK  keyword are:
Shielding  - performs a calculation of the NMR shielding tensor for all atoms in the model.
EFG  - performs a calculation of the electric field gradient tensor for all atoms in the model.
NMR  - performs a calculation of both the NMR shielding tensor and the electric field gradient tensor for all
atoms in the model.
Default
Shielding
Example MAGRES_TASK : NMR
```
