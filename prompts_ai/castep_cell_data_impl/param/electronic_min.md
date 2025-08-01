This is module `electronic_minimisation`.

```
SPIN_FIX (.param)
Keyword type
Integer

Description
This keyword determines the number of electronic iterations for which the total spin is fixed. If SPIN_FIX is less than 0, the spin will be fixed for the duration of the calculation.

This keyword only effects the behavior of the electronic minimizer in the initial SCF calculation. There is a separate keyword, GEOM_SPIN_FIX, which should be used to fix the spin of the system during geometry optimization.

This parameter is only used if FIX_OCCUPANCY : FALSE. So, for insulators the spin is fixed regardless of the value of SPIN_FIX.

Default
10

Example
SPIN_FIX : 5
```

```
SMEARING_WIDTH (.param)
Keyword type
Real

Description
This keyword determines the width of the Fermi-surface smearing if the system is being treated as a metal.

This parameter is used only if FIX_OCCUPANCY : FALSE.

Default
0.2 eV

Example
SMEARING_WIDTH : 0.1 eV
```

```
SMEARING_SCHEME (.param)
Keyword type
String

Description
This keyword determines the Fermi-surface smearing scheme to be used if the system is being treated as a metal. Available options are:

Gaussian
GaussianSplines
FermiDirac
HermitePolynomials
ColdSmearing
This parameter is used only if FIX_OCCUPANCY : FALSE.

Default
Gaussian

Example
SMEARING_SCHEME : ColdSmearing
```

```
NUM_DUMP_CYCLES (.param)
Keyword type
Integer

Description
This keyword specifies the number of SCF cycles between updates to the wavefunction and density data file. If NUM_DUMP_CYCLES is less than or equal to 0, no data will be written to this file.

Default
5

Example
NUM_DUMP_CYCLES : 10
```

```
METALS_METHOD (.param)
Keyword type
String

Description
This keyword determines the electronic minimization method used in the self-consistent calculation. In spite of the term "METALS" in the name of the parameter its usage is the same for metals (FIX_OCCUPANCY : FALSE) and semiconductors (FIX_OCCUPANCY : TRUE). Available options are:

DM - system treated by density mixing (in this case density mixing parameters are printed in the .castep file).
EDFT - system treated by ensemble density functional method: CASTEP does a self-consistent all-bands wavefunction search, which for metals is followed by the self-consistent updating of occupancies.
NONE - currently not used: it is not supported for FIX_OCCUPANCY : FALSE and is the same as EDFT for FIX_OCCUPANCY : TRUE.
Default
EDFT

FIX_OCCUPANCY : FALSE, the default is DM

Example
METALS_METHOD : dm
```

```
MAX_SD_STEPS (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of steepest descent steps in an SCF cycle.

Default
The default depends on the value of ELECTRONIC_MINIMIZER:

SD then MAX_SD_STEPS : 10
CG then MAX_SD_STEPS : 1
RMM/DIIS then MAX_SD_STEPS : 1
If ELECTRONIC_MINIMIZER is not defined, the default is 1.

Example
MAX_SD_STEPS : 5
```

```
MAX_SCF_CYCLES (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of SCF cycles performed in an electronic minimization. The electronic minimization will end after this many cycles, regardless of whether the convergence criteria have been met.

Default
30

Example
MAX_SCF_CYCLES : 20
```

```
MAX_CG_STEPS (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of conjugate gradient steps in an SCF cycle.

Default
The default depends on the value of ELECTRONIC_MINIMIZER:

SD then MAX_CG_STEPS : 0
CG then MAX_CG_STEPS : 4
RMM/DIIS then MAX_CG_STEPS : 2
If ELECTRONIC_MINIMIZER is not defined, the default is 4.

Example
MAX_CG_STEPS : 5
```

```
FIX_OCCUPANCY (.param)
Keyword type
Logical

Description
This keyword specifies whether or not the occupancies of the bands should be fixed, that is, if the system should be treated as a zero temperature insulator or a metal.

Default
FALSE

Example
FIX_OCCUPANCY : TRUE
```

```
ELECTRONIC_MINIMIZER (.param)
Keyword type
String

Description
This keyword controls the method used to minimize electronic states. Available options are:

SD - minimizer takes up to 10 SD steps.
CG - minimizer takes one SD step followed by up to 4 CG steps.
The default values for the number of steps can be overwritten using the MAX_SD_STEPS and MAX_CG_STEPS keywords.

Default
CG

Example
ELECTRONIC_MINIMIZER : SD
```

```
ELEC_RESTORE_FILE (.param)
Keyword type
String

Description
This keyword specifies the name of the file from which wavefunction and density data should be restored when performing a CONTINUATION or a REUSE run.

NULL means that wavefunction and density data will not be restored.

The basis set and distribution for the new run must be the same as those from the run in which the data file was written.

Default
NULL

Example
ELEC_RESTORE_FILE : test.wvfn
```

```
ELEC_ENERGY_TOL (.param)
Keyword type
Real

Description
This keyword controls the tolerance for accepting convergence of the total energy in an electronic minimization.

The difference between maximum and minimum energies over ELEC_CONVERGENCE_WIN iterations must be less than this value.

Default
1x10-5 eV per atom

Example
ELEC_ENERGY_TOL : 0.00007 eV
```

```
ELEC_EIGENVALUE_TOL (.param)
Keyword type
Real

Description
This keyword controls the tolerance for accepting convergence of a single eigenvalue during density mixing minimization.

The difference between maximum and minimum eigenvalues over ELEC_CONVERGENCE_WIN iterations must be less than this value.

Default
The default value is the lower of 1x10-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS, where NATOMS is the total number of atoms in the unit cell.

Example
ELEC_EIGENVALUE_TOL : 0.000007 eV
```

```
ELEC_DUMP_FILE (.param)
Keyword type
String

Description
This keyword determines the name of the file into which wavefunction and density data are written, periodically during electronic minimization. This file can be used as a backup and is restored with the ELEC_RESTORE_FILE parameter.

If this parameter is set to NULL, no backup wavefunction or density data will be written.

Default
seedname.wvfn

Example
ELEC_DUMP_FILE : test.wvfn
```

```
ELEC_CONVERGENCE_WIN (.param)
Keyword type
Integer

Description
This keyword determines the size of the convergence window during a electronic minimization run.

The total energy or eigenvalue must lie within ELEC_ENERGY_TOL or ELEC_EIGENVALUE_TOL respectively, for the last ELEC_CONVERGENCE_WIN iterations for the convergence criteria to be met.

The value of ELEC_CONVERGENCE_WIN must be greater than or equal to 2.

Default
3

Example
ELEC_CONVERGENCE_WIN : 4
```

```
EFERMI_TOL (.param)
Keyword type
Real

Description
This keyword controls the tolerance for accepting convergence of the Fermi-energy if the system is being treated as a metal.

This parameter is used only if FIX_OCCUPANCY : FALSE.

Default
0.1 × ELEC_EIGENVALUE_TOL

Example
EFERMI_TOL : 0.0000007 eV
```
