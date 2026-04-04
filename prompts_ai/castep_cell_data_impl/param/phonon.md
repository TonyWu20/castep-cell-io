These are submodules of module `phonon`
```
BORN_CHARGE_SUM_RULE (.param)
Keyword type
Logical
Description
This keyword specifies whether to correct the Born effective charge tensor explicitly to enforce the sum rule that effective charges sum to
zero.
Default
FALSE
Example BORN_CHARGE_SUM_RULE : TRUE
```
```
CALCULATE_BORN_CHARGES (.param)
Keyword type
Logical
Description
This keyword specifies whether to compute Born effective charge tensors as part of a phonon or electric field linear-response calculation.
TRUE  means that the calculation of the dielectric permittivity is carried out using a linear response
scheme for an applied electric field. The Born effective charges are also evaluated.
This keyword should be set to  TRUE  if
PHONON_CALC_LO_TO_SPLITTING  :  TRUE  or
EFIELD_CALC_ION_PERMITTIVITY  :   TRUE .
Default
TRUE
Example CALCULATE_BORN_CHARGES : FALSE
```
```
PHONON_CALC_LO_TO_SPLITTING (.param)
Keyword type
Logical
Description
This keyword specifies whether to calculate the non-analytical contribution to the dynamical matrix from long-range electric field effects,
which are responsible for the LO/TO splitting of phonon frequencies at the gamma point.
When this keyword is  TRUE , the calculation of the dielectric permittivity is carried out using a linear response
scheme for an applied electric field. The Born effective charges are also evaluated.
You should set this keyword to  FALSE  for molecule in a box systems,
where there is no physical LO-TO splitting of phonon frequencies.
The value of the LO-TO splitting depends on the approach direction to gamma. It is possible
to specify multiple directions using the  PHONON_GAMMA_DIRECTIONS  block.
Default
TRUE
Example PHONON_CALC_LO_TO_SPLITTING : FALSE
```
```
PHONON_CONVERGENCE_WIN (.param)
Keyword type
Integer
Description
This keyword determines the size of the convergence window during a phonon calculation.
The linear response convergence criteria must be met for PHONON_CONVERGENCE_WIN iterations before acceptance.
Default
2
Example PHONON_CONVERGENCE_WIN : 4
```
```
PHONON_ENERGY_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the force constants during a phonon calculation.
The difference between maximum and minimum second order energies over
PHONON_CONVERGENCE_WIN  iterations must be less than this value.
Default
The default value is the same as  ELEC_ENERGY_TOL .
Example PHONON_ENERGY_TOL : 0.00007 eV
```
```
PHONON_FINITE_DISP (.param)
Keyword type
Real
Description
This keyword specifies the amplitude of the ionic perturbation that will be used in finite displacement phonon calculations.
Default
0.01 Bohr
Example PHONON_FINITE_DISP : 0.01 ANG
```
```
PHONON_FINE_METHOD (.param)
Keyword type
String
Description
This keyword selects which calculation method to use for phonon calculation on a fine grid:
NONE  - No interpolation, direct calculations
INTERPOLATE  - Use Linear response (density functional perturbation theory) method
SUPERCELL  - Use Finite displacement method
Default
NONE
Example PHONON_FINE_METHOD : SUPERCELL
```
```
PHONON_FORCE_CONSTANT_CUTOFF (.param)
Keyword type
Real
Description
This keyword specifies the cutoff for the force constant matrix in a phonon calculation on a fine grid with supercell method.
Default
0.0
Example PHONON_FORCE_CONSTANT_CUTOFF : 6.34 ang
```
```
PHONON_MAX_CYCLES (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of iterations in a phonon calculation.
It might be necessary to increase this value if a low
PHONON_MAX_CG_STEPS  is used.
Default
50
Example PHONON_MAX_CYCLES : 30
```
```
PHONON_MAX_CG_STEPS (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of conjugate gradient steps taken for each electronic band in the electronic minimizer before
resetting to the steepest descent direction, during a phonon calculation.
Default
20
Example PHONON_MAX_CG_STEPS : 10
```
```
SECONDD_METHOD, PHONON_METHOD (.param)
Keyword type
String
Description
This keyword specifies which calculation method will be used for phonons. SECONDD_METHOD is the latest form of this keyword, while the
obsolete version, PHONON_METHOD, is supported for backward compatibility. Available options are:
LINEARRESPONSE  (or DFPT) - Linear response method (or density functional perturbation theory)
FINITEDISPLACEMENT  - Finite displacement method
The finite displacement method produces phonons at the Γ point only.
Using the linear response method imposes restrictions on the calculation settings.
Specifically, norm-conserving pseudopotentials must be used, with fixed occupation numbers and without spin polarization.
Default
LINEARRESPONSE
Example SECONDD_METHOD : FINITEDISPLACEMENT
```
```
PHONON_SUM_RULE (.param)
Keyword type
Logical
Description
This keyword specifies whether to correct the dynamical matrix explicitly to enforce the acoustic q=0 phonon sum rule, that is, that 3 modes
have zero frequency at q=0.
Default
FALSE
Example PHONON_SUM_RULE : TRUE
```
