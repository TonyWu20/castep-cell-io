This is the documentations for types in module `general`.

`IPRINT`: I'd like to make it into an enum, to prevent out of range case.

```
IPRINT (.param)
Keyword type
Integer

Description
This keyword controls the level of verbosity of the output. Possible values range from 0, which produces minimal output, to 3, which corresponds to full debugging output.

Default
1

Example
IPRINT : 1
```

```
NUM_BACKUP_ITER (.param)
Keyword type
Integer

Description
This keyword specifies the number of geometry optimization or molecular dynamics iterations between updates of the backup restart files.

This keyword cannot be used in conjunction with BACKUP_INTERVAL.

Default
5

Example
NUM_BACKUP_ITER : 2
```

```
OPT_STRATEGY (.param)
Keyword type
String

Description
This parameter determines the optimization strategy used when there are multiple strategies available for the selected algorithm and they have differing costs in terms of memory usage and performance. Available options are:

Speed - uses the optimization strategy which maximizes performance at the cost of additional memory usage.
Default - uses the optimization strategy that best balances performance and memory usage.
Memory - uses the optimization strategy which minimizes memory usage at a cost of decreased performance.
Default
Default

Example
OPT_STRATEGY : Memory
```

```
PAGE_WVFNS (.param)
Keyword type
Integer

Description
This keyword controls the paging of wavefunctions to disk in order to save memory. Available options are:

> 0 - all wavefunctions requiring more memory than this value in megabytes will be paged to disk.
0 - no paging will be performed.
< 0 - all wavefunctions will be paged to disk.
Default
0

Example
PAGE_WVFNS : 2
```

```
PRINT_CLOCK (.param)
Keyword type
Logical

Description
This keyword specifies whether or not timing information will be printed, as the calculation progresses.

Default
TRUE

Example
PRINT_CLOCK : TRUE
```

```
PRINT_MEMORY_USAGE (.param)
Keyword type
Logical

Description
This keyword specifies whether CASTEP should print memory estimates during the run. These estimates may change during the execution, since for example the size of the basis set can change as a result of a change in cell parameters. An updated estimates would be printed when this is the case.

FALSE turns off the estimates.

Default
TRUE.

Example
PRINT_MEMORY_USAGE : FALSE
```

```
RAND_SEED (.param)
Keyword type
Integer (expert)

Description
This keyword controls the initialization of random number seeds. Available options are:

0 - the seed for the random number generation is selected pseudorandomly.
> or < 0 - this value is used as a seed for the random number generator.
Default
0

Example
RAND_SEED : -25
```

```
REUSE (.param)
Keyword type
String

Description
This keyword contains a string which specifies the model file from which data will be read to initialize a new calculation. If no file is to be used, this string should be NULL, which means no reuse (a new run).

DEFAULT (case-insensitive) will use the data from the seedname.check file. The same effect can be achieved by providing the REUSE keyword on its own, with no value.

A run cannot be both a REUSE and a CONTINUATION. A REUSE run is a new calculation that uses as much as data as possible from a previous calculation.

Default
NULL

Example
REUSE : DEFAULT
```

```
RUN_TIME (.param)
Keyword type
Integer

Description
This keyword specifies the maximum run time for the job, in seconds. If the RUN_TIME is greater than zero, the job will exit cleanly before the specified time has elapsed, leaving as little unused time as possible.

Clean termination before RUN_TIME cannot be guaranteed. The shortest operation which can be timed is an electronic minimization or a single molecular dynamics or geometry optimization step.

If RUN_TIME is less than or equal to zero, no time limit will be imposed on the run.

Default
0

Example
RUN_TIME : 360
```

`STOP` would be `Cell::Flag`, we can define it as a newtype wrapper of `bool`.

```
STOP (.param)
Keyword type
Defined

Description
This keyword, if present, will cause the current run to be aborted as if RUN_TIME had been exceeded.

CASTEP checks the contents of the input file periodically during a run. This allows you to modify certain parameters and also to terminate the run early.

This keyword is valid only when the input file is reread. It is ignored if it is present at the start of a run.

Example
STOP
```

```
TASK (.param)
Keyword type
String

Description
This keyword defines the type of calculation to perform. Available options are:

SinglePoint - performs a single-point energy calculation.
BandStructure - calculates band structure properties.
GeometryOptimization - searches for a minimum energy structure.
MolecularDynamics - performs a molecular dynamics calculation.
Optics - calculates optical properties.
Phonon - performs a linear response calculation to determine phonon frequencies and eigenvectors.
Efield - performs an electric field linear response calculation to determine dielectric permittivity and polarizability.
Phonon+Efield - performs a linear response calculation to determine phonon frequencies and eigenvectors, and an electric field linear response calculation to determine dielectric permittivity and polarizability.
TransitionStateSearch - performs a transition-state search.
MagRes - performs an NMR calculation.
Elnes - performs core level spectroscopy calculation.
ElectronicSpectroscopy - performs electronic spectroscopy calculation.
Autosolvation - performs a free energy of solvation calculation.
NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if you have purchased this module.

Default
SinglePoint

Example
TASK : optics
```
