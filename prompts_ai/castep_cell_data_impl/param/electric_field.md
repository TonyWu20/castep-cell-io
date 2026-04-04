These are submodules of module `electric_field`
```
EFIELD_CALC_ION_PERMITTIVITY (.param)
Keyword type
Logical
Description
This keyword controls whether to compute the zero-frequency dielectric permittivity based on the ionic response to electric fields.
This requires a gamma point phonon calculation in addition to the linear response electric
field calculation.
Default
TRUE
Example EFIELD_CALC_ION_PERMITTIVITY : FALSE
```
```
EFIELD_CALCULATE_NONLINEAR (.param)
Keyword type
String
Description
Specifies which non-linear optical property to calculate during a TASK=EFIELD calculation. Available options are:
CHI2  - produces second harmonic generation coefficients.
NONE  - non-linear optical properties are not calculated.
Default
NONE
Example EFIELD_CALCULATE_NONLINEAR : CHI2
```
```
EFIELD_CONVERGENCE_WIN (.param)
Keyword type
Integer
Description
This keyword determines the size of the convergence window during a linear response to electric field calculation.
The linear response convergence criteria must be met for EFIELD_CONVERGENCE_WIN iterations before acceptance.
Default
2
Example EFIELD_CONVERGENCE_WIN : 30
```
```
EFIELD_ENERGY_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the field constants during a linear response to electric field calculation.
This parameter has the units of volume.
The difference between maximum and minimum second order energies over
EFIELD_CONVERGENCE_WIN  iterations must be less than this value.
Default
1×10 -5  Å
Example EFIELD_ENERGY_TOL : 0.000002 ANG**3
```
```
EFIELD_IGNORE_MOL_MODES (.param)
Keyword type
String
Description
This keyword specifies how many of the lowest lying modes to ignore when computing the ionic contribution to the permittivity and
polarizability. Available options are:
Crystal  - Ignore the three lowest lying modes
Molecule  - Ignore the six lowest lying modes
Linear_molecule  - Ignore the five lowest lying modes
Default
Crystal
Example EFIELD_IGNORE_MOL_MODES : Molecule
```
```
EFIELD_MAX_CG_STEPS (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of conjugate gradient steps taken for each electronic band in the electronic minimizer before
resetting to the steepest descent direction, during a linear response to electric field calculation.
Default
20
Example EFIELD_MAX_CG_STEPS : 30
```
```
EFIELD_MAX_CYCLES (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of iterations in a linear response to electric field calculation.
It may be necessary to increase this value if a low value of
EFIELD_MAX_CG_STEPS  is used.
Default
50
Example EFIELD_MAX_CYCLES : 100
```
