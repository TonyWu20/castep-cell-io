This is the documentations for types in module `general`.

```
BACKUP_INTERVAL (.param)
Keyword type
Integer

Description
This keyword specifies the interval, in seconds, between updates of the backup restart files. This keyword is applicable for geometry optimization, molecular dynamics, phonon or transition state search runs. A value which is less than or equal to zero indicates that no updates will be performed.

This keyword cannot be used in conjunction with NUM_BACKUP_ITER.

Default
0

Example
BACKUP_INTERVAL : 3600
```

```
CALCULATE_STRESS (.param)
Keyword type
Logical

Description
This keyword controls whether or not a stress calculation will be performed.

If CALCULATE_STRESS is set to TRUE, the stress tensor will be calculated, no matter which TASK is selected. If it is set to FALSE, the stress tensor will only be calculated if it is required, for example during a cell geometry optimization with cell relaxation.

Default
FALSE

Example
CALCULATE_STRESS : TRUE
```

```
CALCULATE_DENSDIFF (.param)
Keyword type
Logical

Description
This keyword controls whether or not a calculation of the density difference with respect to the sum of atomic densities will be performed.

TRUE means that the density difference will be calculated and saved in a binary file seed.chdiff.

Default
FALSE

Example
CALCULATE_DENSDIFF : TRUE
```

```
CALCULATE_ELF (.param)
Keyword type
Logical

Description
This keyword controls whether or not a calculation of the electron localization function will be performed.

TRUE means that the ELF will be calculated and saved in a binary file seed.elf.

Default
FALSE

Example
CALCULATE_ELF : TRUE
```

```
CALCULATE_HIRSHFELD (.param)
Keyword type
Logical

Description
This keyword controls whether or not a calculation of Hirshfeld atomic charges will be performed.

TRUE means that the Hirshfeld charges will be calculated and reported in the output file seed.castep at the end of the calculation.

Default
FALSE

Example
CALCULATE_HIRSHFELD : TRUE
```

```
CHECKPOINT (.param)
Keyword type
String

Description
This keyword contains a string which specifies the name of file to which checkpoint (continuation) data are to be written.

Default
seedname.check

Example
CHECKPOINT : test.check
```

```
COMMENT (.param)
Keyword type
String

Description
This keyword can contain a comment string, used to label the output.

Default
By default this is blank.
```

```
CONTINUATION (.param)
Keyword type
String

Description
This keyword contains a string which specifies the model file used to continue the job.

DEFAULT (case-insensitive) means that the file seedname.check will be used. The same effect can be achieved by providing the CONTINUATION keyword on its own, with no value.

A run cannot be both a REUSE and a CONTINUATION. A CONTINUATION run restarts the current job from the end of the previous one. As a consequence, only a limited subset of the available parameters may be changed between CONTINUATION runs.

Default
NULL - corresponds to no continuation (a new run)

Example
CONTINUATION : DEFAULT
```

```
DATA_DISTRIBUTION (.param)
Keyword type
String

Description
This keyword determines the parallelization strategy used. Available options are:

Kpoint - only k-point parallelization will be used (best scalability)
Gvector - only g-vector parallelization will be used (worst scalability)
Mixed - a combination of k-point and g-vector parallelization will be used
Default - the optimal parallelization strategy for the architecture will be used
The Default setting is appropriate in most situations.

The Kpoint option is available only when the number of processors is a factor of the number of k-points.

The Gvector option is appropriate for calculations involving large systems where often only one k-point is used.

The Mixed option is available when the number of processors and the number of k-points have a common factor (for example 6 k-points and 4 processors).

The differences in the scaling properties of the different schemes dictate the number of processors that should be used. For example, a calculation that uses 16 k-points is likely to finish faster on 4 processors (using Kpoint distribution) than on 6 processors (using Mixed distribution) and most likely faster than on 9 processors (using Gvector distribution).

Default
Default

Example
DATA_DISTRIBUTION : Gvector
```
