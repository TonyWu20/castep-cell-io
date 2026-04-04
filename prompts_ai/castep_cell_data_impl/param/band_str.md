These are submodules of module `band_structure`
```
BS_EIGENVALUE_TOL (.param)
Keyword type
Real
Description
This keyword controls the tolerance for accepting convergence of a single eigenvalue or band during a band structure calculation.
The difference between maximum and minimum eigenvalue for every band over
ELEC_CONVERGENCE_WIN  iterations must be less than this value.
Default
1×10 -6  eV .
Example BS_EIGENVALUE_TOL = 1.0e-5 Ha
```
```
BS_MAX_CG_STEPS (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of conjugate gradient steps taken for each electronic band in the electronic minimizer before
resetting to the steepest descent direction, during a band structure calculation.
Default
4
Example BS_MAX_CG_STEPS : 10
```
```
BS_MAX_ITER (.param)
Keyword type
Integer
Description
This keyword controls the maximum number of iterations to perform when calculating band structure.
It might be necessary to increase this value if a low
BS_MAX_CG_STEPS  is used.
Default
60
Example BS_MAX_ITER : 50
```
```
BS_NBANDS (.param)
Keyword type
Integer
Description
This keyword determines the number of bands at each k-point when performing a band structure calculation.
There are three ways in which you can specify the number of bands at each k-point:
Directly, using BS_NBANDS.
Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using
BS_NEXTRA_BANDS .
Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value using
BS_PERC_EXTRA_BANDS .
It is not possible to have both the BS_NBANDS keyword and either the
BS_NEXTRA_BANDS  or  BS_PERC_EXTRA_BANDS  keywords present
in the same input file.
Default
NBANDS + 5√NBANDS
Example BS_NBANDS : 64
```
```
BS_NEXTRA_BANDS (.param)
Keyword type
Integer
Description
This keyword controls the number of extra bands at each k-point in addition to the number of occupied bands, when performing a band structure
calculation.
It is not possible to have both the  BS_NBANDS  keyword
and either the BS_NEXTRA_BANDS or  BS_PERC_EXTRA_BANDS  keywords present in the same input file.
Default
0
The default value corresponds to a calculation of the band structure for the valence band only.
To calculate band structure for the conduction band use positive values of BS_NEXTRA_BANDS.
Example BS_NEXTRA_BANDS : 12
```
```
BS_PERC_EXTRA_BANDS (.param)
Keyword type
Real
Description
This keyword controls the percentage of extra bands at each k-point in addition to the number of occupied bands, when performing a band
structure calculation.
It is not possible to have both the  BS_NBANDS  keyword
and either the  BS_NEXTRA_BANDS  or BS_PERC_EXTRA_BANDS keywords present in the same input file.
Default
0
The default value corresponds to a calculation of the band structure for the valence band only.
To calculate band structure for the conduction band use positive values of BS_PERC_EXTRA_BANDS.
Example BS_PERC_EXTRA_BANDS : 60.0
```
```
BS_RE_EST_K_SCRN (.param)
Keyword type
Logical
Description
This keyword determines whether or not to update the estimate of the Thomas-Fermi screening length in the screened exchange formalism before
the start of a band structure calculation.
This keyword is not relevant if  RE_EST_K_SCRN  :  TRUE , since the reevaluation will happen automatically in this case.
Default
FALSE
Example BS_RE_EST_K_SCRN : TRUE
```
```
BS_XC_FUNCTIONAL (.param)
Keyword type
String
Description
This keyword controls which functional is used to determine the exchange-correlation potential during a band structure calculation. This
option allows you to apply screened and exact exchange functionals non self-consistently to obtain more accurate band gaps than with LDA or GGA
functionals. Available options are:
LDA  - Local Density Approximation
PW91  - Perdew Wang '91 GGA
PBE  - Perdew Burke Ernzerhof
RPBE  - Revised Perdew Burke Ernzerhof
WC  - Wu-Cohen
PBESOL  - PBEsol, PBE functional for solids
HF  - exact exchange, no correlation
HF-LDA  - exact exchange, LDA correlation
SHF  - screened exchange, no correlation
SHF-LDA  - screened exchange, LDA correlation
PBE0  - PBE0 hybrid functional
B3LYP  - B3LYP hybrid functional
HSE03  - HSE03 hybrid functional
HSE06  - HSE06 hybrid functional
RSCAN  - regularized SCAN meta-GGA functional
Default
The default value is derived from the value for  XC_FUNCTIONAL .
Example BS_XC_FUNCTIONAL : PW91
```
