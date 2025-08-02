These are submodules of module `geometry_optimization`
```
GEOM_CONVERGENCE_WIN (.param)
Keyword type
Integer
Description
This keyword determines the size of the convergence window for a geometry optimization. It defines the number of geometry optimization steps
over which the energy convergence criteria must be met for convergence to be accepted.
Default
2
Example GEOM_CONVERGENCE_WIN : 4
```
```
GEOM_DISP_TOL (.param)
Keyword type
Real
Description
This keyword determines the tolerance for accepting convergence of the ionic displacement during a geometry optimization.
Default
0.001 Å
Example GEOM_DISP_TOL : 0.002 ang
```
```
GEOM_ENERGY_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the free energy per atom during a geometry optimization.
The difference between maximum and minimum values of the free energy over
GEOM_CONVERGENCE_WIN  iterations must be less than this value.
Default
2×10 -5  eV per atom
Example GEOM_ENERGY_TOL : 0.00005 eV
```
```
GEOM_FORCE_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the ionic force during a geometry optimization.
Default
0.05 eV Å -1
Example GEOM_FORCE_TOL : 0.07 ev/ang
```
```
GEOM_FREQUENCY_EST (.param)
Keyword type
Real
Description
This keyword provides an estimate of the average phonon frequency at the gamma point. It is used to initialize the Hessian for BFGS geometry
optimization with ionic relaxation. One expects to achieve faster convergence of the geometry optimization if the value is realistic. The idea
is that this single number represents the whole of the Hessian in a compact form.
The value of the estimated frequency is provided in the CASTEP output after the geometry optimization run, and this value can be used as an
input parameter for a new calculation on a similar system. An example would be a study of different configurations of a self-interstitial defect
in silicon.
The value of the estimated frequency as returned by CASTEP should not be interpreted as an
actual frequency of any real vibrational mode.
Default
50 THz
Example GEOM_FREQUENCY_EST : 17.54 THz
```
```
GEOM_MAX_ITER (.param)
Keyword type
Integer
Description
This keyword determines the maximum number of steps in a geometry optimization.
Default
30
Example GEOM_MAX_ITER : 25
```
```
GEOM_METHOD (.param)
Keyword type
String
Description
This keyword determines the method used for geometry optimization. Available options are:
BFGS  - BFGS minimization.
LBFGS  - low-memory BFGS minimization.
Delocalized  (or  Delocalised ) - BFGS minimization using delocalized internal
coordinates instead of Cartesian coordinates.
DampedMD  - Damped molecular dynamics.
TPSD  - Two-point steepest descent.
The  Delocalized  and  DampedMD  options
are available only for geometry optimization with a fixed cell.
Default
BFGS
Example GEOM_METHOD : DampedMD
```
```
GEOM_MODULUS_EST (.param)
Keyword type
Real
Description
This keyword provides an estimate of the bulk modulus of the system. It is used to initialize the Hessian for BFGS geometry optimization
with cell relaxation.
Default
500.0 GPa
Example GEOM_MODULUS_EST : 125.4 GPa
```
```
GEOM_PRECONDITIONER (.param)
Keyword type
String
Description
This keyword selects the preconditioner used for LBFGS geometry optimization. Available options are:
ID  - identity; LBFGS is used without a preconditioner.
EXP  - exponential preconditioner.
FF - forcefield based preconditioner using the scheme of  Lindh  et al.  (1995) .
The  ID  option's only advantage over the  BFGS  minimizer is lower memory requirements. The  EXP  option is generally the best in terms of performance gains. The forcefield based preconditioner  FF  can sometimes provide further gains, although it is less stable and might require more steps to converge.
Default
ID
Example GEOM_PRECONDITIONER : EXP
```
```
GEOM_SPIN_FIX (.param)
Keyword type
Integer
Description
This keyword determines the number of geometry optimization steps for which the total spin is fixed. If GEOM_SPIN_FIX is less than
0 , the spin will be fixed to the value found at the end of the SCF calculation for the initial structure
for the duration of the calculation.
This parameter is only used if  FIX_OCCUPANCY  =
FALSE. So for insulators the spin is fixed regardless of the value of GEOM_SPIN_FIX.
The default value for this parameter is  0 , so spin is allowed to vary.
Example GEOM_SPIN_FIX : 5
```
```
GEOM_STRESS_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the maximum stress component during unit cell optimization.
Default
0.1 GPa
Example GEOM_STRESS_TOL : 0.2 GPa
```
