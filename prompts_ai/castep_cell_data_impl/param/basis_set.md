For module `basis_set`:

```
BASIS_DE_DLOGE (.param)
Keyword type
Real

Description
This keyword specifies the derivative of total energy with respect to the natural log of the basis cutoff energy. The value is used only if FINITE_BASIS_CORR : 1, which corresponds to manual finite basis set correction. If an automated calculation of this quantity has already been performed, then for the next run on the same system, it is possible to reuse the value using the BASIS_DE_DLOGE keyword. However, this type of correction has very limited applicability since both the system (that is, the atomic positions and cell parameters) and the calculation parameters have to be exactly the same as in the initial run. In general, the automatic correction mode (FINITE_BASIS_CORR : 2) should be used.

The value of the total energy derivative should always be negative, since the total energy decreases as the size of the basis increases.

CASTEP exits with an error if the value of BASIS_DE_DLOGE is not provided in the parameters file when FINITE_BASIS_CORR : 1.

Default
0.0

Example
BASIS_DE_DLOGE : -1.2345 eV
```

```
BASIS_PRECISION (.param)
Keyword type
String

Description
This keywords specifies the precision of the basis set by choosing the level of convergence of atomic energies with respect to the plane wave cutoff energy for the pseudopotentials used in the calculation. Available options are:

COARSE - convergence of about 1 eV/atom
MEDIUM - convergence of about 0.3 eV/atom
FINE - convergence of about 0.1 eV/atom
PRECISE - 1.2 × FINE cutoff energy
EXTREME - 1.6 × FINE cutoff energy, convergence of about 0.01 eV/atom
If the BASIS_PRECISION is defined, the CUT_OFF_ENERGY will be equal to the highest of the cutoff energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.

It is not possible to specify both the BASIS_PRECISION and the CUT_OFF_ENERGY in a single file.

Default
FINE

Example
BASIS_PRECISION : MEDIUM
```

```
GRID_SCALE (.param)
Keyword type
Real

Description
This keyword determines the size of the standard grid, relative to the diameter of the cutoff sphere.

Default
1.75

Example
GRID_SCALE : 2.0
```

```
FIXED_NPW (.param)
Keyword type
Logical

Description
This keyword determines whether a fixed number of plane waves (fixed size basis : TRUE) or a fixed plane wave cutoff energy (fixed quality basis : FALSE) will be used when doing a variable cell calculation.

This setting affects geometry optimization with variable cell parameters and molecular dynamics with NPT or NPH ensembles.

You should turn off finite basis set correction when using a fixed size basis (see FINITE_BASIS_CORR).

Default
FALSE

Example
FIXED_NPW : TRUE
```

```
FINITE_BASIS_SPACING (.param)
Keyword type
Real

Description
This keyword determines the spacing of cutoff energies used to estimate the BASIS_DE_DLOGE in automatic calculation of the finite basis set correction. Points are chosen such that the CUT_OFF_ENERGY corresponds to the highest energy in the set of FINITE_BASIS_NPOINTS cutoff energies.

This value is only used if FINITE_BASIS_CORR : 2.

Default
5.0 eV

Example
FINITE_BASIS_SPACING : 0.2 Ha
```

```
FINITE_BASIS_NPOINTS (.param)
Keyword type
Integer

Description
This keyword controls the number of points used to estimate the BASIS_DE_DLOGE in automatic calculation of the finite basis set correction. Points are chosen such that the CUT_OFF_ENERGY corresponds to the highest energy in the set of FINITE_BASIS_NPOINTS cutoff energies.

This value is only used if FINITE_BASIS_CORR : 2.

Default
3

Example
FINITE_BASIS_NPOINTS : 5
```

```
FINITE_BASIS_CORR (.param)
Keyword type
Integer

Description
This keyword determines whether or not to apply a finite basis set correction to energy and stress when cell parameters change. Available options are:

0 - no correction.
1 - manual correction using specified BASIS_DE_DLOGE.
2 - automatic correction using FINITE_BASIS_NPOINTS and FINITE_BASIS_SPACING.
If FINITE_BASIS_CORR : 1, a value for BASIS_DE_DLOGE must be supplied.

You should turn off finite basis set correction when using a fixed size basis (see FIXED_NPW).

Default
2

Example
FINITE_BASIS_CORR : 1
```

```
FINE_GRID_SCALE (.param)
Keyword type
Real

Description
This keyword determines the maximum size of the g-vectors included in the fine grid relative to the standard grid.

Default
1 - this results in the fine and standard grids being identical

Example
FINE_GRID_SCALE : 2.0
```

```
FINE_GMAX (.param)
Keyword type
Real

Description
This keyword determines the maximum size of the g-vectors included in the fine grid.

The fine grid is set up such that all g-vectors with |g| less than or equal to FINE_GMAX are included.

Default
-1 a0-1 - this results in the fine and standard grids being identical

Example
FINE_GMAX : 20 1/ang
There is a more convenient way of dealing with fine grid using dimensionless parameter FINE_GRID_SCALE which defines FINE_GMAX in terms of standard grid GMAX.
```

```
CUT_OFF_ENERGY (.param)
Keyword type
Real

Description
This keyword specifies the cutoff energy for the plane wave basis sets that will be used in the calculation.

If the BASIS_PRECISION is defined, the cutoff energy will be equal to the highest of the cutoff energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.

If neither the BASIS_PRECISION nor the CUT_OFF_ENERGY are defined, the default cutoff energy is that associated with the FINE level of accuracy, for the pseudopotentials in the calculation.

It is not possible to specify both the BASIS_PRECISION and the CUT_OFF_ENERGY in a single file.

Example
CUT_OFF_ENERGY = 125 eV
```
