These are submodules of module `molecular_dynamics`
```
MD_BAROSTAT (.param)
Keyword type
String
Description
This keyword determines the barostat method used for molecular dynamics with variable cell volume. Available options are:
Andersen-Hoover
Parrinello-Rahman
Default
Andersen-Hoover .
Example MD_BAROSTAT : Andersen-Hoover
```
```
MD_CELL_T (.param)
Keyword type
Real
Description
This keyword is used to set the relevant MD barostat parameters, for example Nosé-Hoover barostat mass or Langevin damping time.
This keyword is used only if  MD_ENSEMBLE
:   NPT  or  NPH .
Default
10 ×   MD_ION_T
Example MD_CELL_T : 2 ps
```
```
MD_DAMPING_RESET (.param)
Keyword type
Integer
Description
This keyword specifies the number of molecular dynamics steps between recalculations of the damping parameters. If this parameter is set to
0 , the damping parameters remain fixed.
Default
30
Example MD_DAMPING_RESET : 20
```
```
MD_DAMPING_SCHEME (.param)
Keyword type
String
Description
This keyword controls the damping scheme used during damped molecular dynamics for geometry optimization. Available options are:
Independent  - calculates optimal damping parameters according to the independent modes algorithm.
Coupled  - calculates optimal damping parameters according to the coupled modes algorithm.
SteepestDescents  - performs steepest descent dynamics.
Default
Independent
Example MD_DAMPING_SCHEME : Coupled
```
```
MD_DELTA_T (.param)
Keyword type
Real
Description
This keyword determines the time step that will be used for a molecular dynamics calculation.
Default
1.0 fs
Example MD_DELTA_T : 1.54 fs
```
```
MD_ELEC_CONVERGENCE_WIN (.param)
Keyword type
Integer
Description
This keyword determines the size of the convergence window for electronic minimization during a molecular dynamics run.
The total energy or eigenvalue must lie within  MD_ELEC_ENERGY_TOL  or
MD_ELEC_EIGENVALUE_TOL  respectively, for the last MD_ELEC_CONVERGENCE_WIN iterations for the
convergence criteria to be met.
The value of MD_ELEC_CONVERGENCE_WIN must be greater than or equal to 2.
Default
By default this has the same value as  ELEC_CONVERGENCE_WIN .
Example MD_ELEC_CONVERGENCE_WIN : 4
```
```
MD_ELEC_EIGENVALUE_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of a single eigenvalue in a DM/DIIS minimization during a molecular dynamics
run.
The difference between maximum and minimum eigenvalues over
MD_ELEC_CONVERGENCE_WIN  iterations must be less than this value.
Default
By default this has the same value as  ELEC_EIGENVALUE_TOL .
Example MD_ELEC_EIGENVALUE_TOL : 0.000007 eV
```
```
MD_ELEC_ENERGY_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of the total energy in an electronic minimization during a molecular dynamics
run.
The difference between maximum and minimum energies over
MD_ELEC_CONVERGENCE_WIN  iterations must be less than this value.
Default
By default this has the same value as  ELEC_ENERGY_TOL .
Example MD_ELEC_ENERGY_TOL : 0.00007 eV
```
```
MD_ENSEMBLE (.param)
Keyword type
String
Description
This keyword determines the ensemble used for a molecular dynamics calculation. Available options are:
NVT
NVE
NPT
NPH
Default
NVE
Example MD_ENSEMBLE : NVT
```
```
MD_EQM_CELL_T (.param)
Keyword type
Real
Description
This keyword is used to set the MD barostat parameter for use in enhanced MD equilibration method (when
MD_EQM_METHOD  is not set to  NONE ).
Default
By default this has the same value as  MD_CELL_T .
Example MD_EQM_CELL_T : 2 ps
```
```
MD_EQM_ION_T (.param)
Keyword type
Real
Description
This keyword is used to set the MD barostat parameter for use in enhanced MD equilibration method (when
MD_EQM_METHOD  is not set to  NONE ).
Default
By default this has the same value as  MD_ION_T .
Example MD_EQM_ION_T : 0.5 ps
```
```
MD_EQM_METHOD (.param)
Keyword type
String
Description
This advanced keyword determines the scheme to be used for enhanced MD equilibration. Equilibration is achieved by turning on a weakly
coupled thermostat and/or barostat (depending on the setting for   MD_ENSEMBLE ). The use of this option
allows you to speed up equilibration in the way that does not necessarily conform to any ensemble. Available options are:
NONE
BERENDSEN
The time scale of the ionic thermostat is controlled by  MD_EQM_ION_T , the cell barostat is controlled
by  MD_EQM_CELL_T . The weak coupling will be turned off after
MD_EQM_T  seconds.
Default
NONE
Example MD_EQM_METHOD : BERENDSEN
```
```
MD_EQM_T (.param)
Keyword type
Real
Description
This keyword is used to set the length of the enhanced MD equilibration step (when  MD_EQM_METHOD
is not set to  NONE ).
Default 1 ps
Example MD_EQM_T : 0.5 ps
```
```
MD_EXTRAP (.param)
Keyword type
String
Description
This keyword determines the wavefunction extrapolation scheme used for a molecular dynamics calculation. The same scheme is also used for
charge density extrapolation in density mixing calculations. Available options are:
None  - No extrapolation used.
First  - First order extrapolation.
Second  - Second order extrapolation.
Mixed  - Alternating first and second order extrapolation.
Default
First
Example MD_EXTRAP : Mixed
```
```
MD_EXTRAP_FIT (.param)
Keyword type
Logical
Description
This keyword controls whether or not best fit extrapolation parameters will be calculated after each molecular dynamics step.
If MD_EXTRAP_FIT =  TRUE , best fit extrapolation parameters will be calculated after each molecular dynamics step.
Otherwise, fixed extrapolation parameters will be used.
Default
TRUE
Example MD_EXTRAP_FIT = FALSE
```
```
MD_ION_T (.param)
Keyword type
Real
Description
This keyword is used to set the relevant MD thermostat parameters, for example Nosé-Hoover thermostat mass or Langevin damping time.
This keyword is used only if  MD_ENSEMBLE  =
NVT  or  NPT .
Default
10 ×   MD_DELTA_T  if
MD_THERMOSTAT  :  Nosé-Hoover
100 ×   MD_DELTA_T  if  MD_THERMOSTAT  :
Langevin
Example MD_ION_T : 0.5 ps
```
```
MD_NHC_LENGTH (.param)
Keyword type
Integer
Description
This keyword is used to control the length of Nosé-Hoover thermostat chains. A chain of Nosé-Hoover thermostats of a given
length may be used to thermostat an NVT run or two separate chains (ions and cell) may be used to thermostat an NPT run.
This keyword is used only if  MD_ENSEMBLE  :  NVT  or  NPT  and  MD_THERMOSTAT  :
Nosé-Hoover .
Default
5
Example MD_NHC_LENGTH : 3
```
```
MD_NUM_ITER (.param)
Keyword type
Integer
Description
This keyword determines the total number of steps that will be performed in a molecular dynamics calculation.
The default value for this parameter is  100 .
Example MD_NUM_ITER : 125
```
```
MD_OPT_DAMPED_DELTA_T (.param)
Keyword type
Logical
Description
This keyword controls whether or not the optimal time step will be calculated after each damped molecular dynamics step.
If MD_OPT_DAMPED_DELTA_T :  TRUE , the optimal time step will be calculated after each damped molecular dynamics
step. Otherwise, a fixed time step will be used.
Default
FALSE
Example MD_OPT_DAMPED_DELTA_T : TRUE
```
```
MD_TEMPERATURE (.param)
Keyword type
Real
Description
This keyword determines the temperature for a molecular dynamics calculation.
This parameter is used only if  MD_ENSEMBLE  : NVT.
Default
The default value for this parameter is  300 K .
Example MD_ENSEMBLE : 275.4 K
```
```
MD_THERMOSTAT (.param)
Keyword type
String
Description
This keyword determines the thermostat used for a molecular dynamics calculation. Available options are:
Nose-Hoover
Langevin
This parameter is used only if  MD_ENSEMBLE  : NVT.
Default
The default value for this parameter is  Nose-Hoover .
Example MD_THERMOSTAT : Langevin
```
