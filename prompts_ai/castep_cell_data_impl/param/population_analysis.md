These are submodules of module `population_analysis`
```
PDOS_CALCULATE_WEIGHTS (.param)
Keyword type
Logical
Description
This keyword specifies whether or not the weight of the bands in each localized orbital will be calculated for the final ground state of the
calculation, in order to allow a partial density of states analysis to be performed.
Default
FALSE
Example PDOS_CALCULATE_WEIGHTS : TRUE
```
```
POPN_BOND_CUTOFF (.param)
Keyword type
Real
Description
This keyword controls the maximum distance between two atoms, for which a bond population will be generated, when performing a population
analysis.
Default
3.0 Å
Example POPN_BOND_CUTOFF : 2.54 ang
```
```
POPN_WRITE (.param)
Keyword type
String
Description
This keyword specifies the verbosity of reporting of population analysis results.
The available values are:
Value
Meaning
NONE
No output
MINIMAL
Summary only
SUMMARY
Same as  MINIMAL
ENHANCED
Summary and orbital-resolved PDOS components
VERBOSE
As  ENHANCED  and S and T matrices
Default
ENHANCED
Example POPN_WRITE : SUMMARY
```
```
POPN_CALCULATE (.param)
Keyword type
Logical
Description
This keyword specifies whether or not to perform a population analysis on the final ground state of the calculation.
Default
TRUE
Example POPN_CALCULATE : FALSE
```
