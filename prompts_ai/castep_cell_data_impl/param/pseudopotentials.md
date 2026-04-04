```
PSPOT_BETA_PHI_TYPE (.param)
Keyword type
String (expert)

Description
This keyword controls the representation of the nonlocal part of the pseudopotential used for calculation of the <β|ϕ> overlaps. Available options are:

RECIPROCAL - reciprocal space nonlocal pseudopotentials
REAL - real space nonlocal pseudopotentials
This parameter can only take the value REAL if PSPOT_NONLOCAL_TYPE is also REAL.

Default
The default is the value of PSPOT_NONLOCAL_TYPE.

Example
PSPOT_BETA_PHI_TYPE : REAL
```

```
PSPOT_NONLOCAL_TYPE (.param)
Keyword type
String

Description
This keyword controls the representation of the nonlocal part of the pseudopotential. Available options are:

RECIPROCAL - reciprocal space nonlocal pseudopotentials.
REAL - real space nonlocal pseudopotentials.
Default
The default is the value of RECIPROCAL.

Example
PSPOT_NONLOCAL_TYPE : REAL
```

```
RELATIVISTIC_TREATMENT (.param)
Keyword type
String

Description
Selects the method used to treat relativistic effects. This functionality is implemented for on-the-fly generated pseudopotentials, so this keyword has no effect when pseudopotentials are read from a file. Available options are:

SCHROEDINGER - this option produces completely non-relativistic pseudopotentials.
ZORA - scalar relativistic treatment, which is equivalent to the zeroth-order expansion of the Koelling-Harmon equation.
KOELLING-HARMON - scalar relativistic treatment.
DIRAC - fully relativistic treatment.
Default
KOELLING-HARMON

Example
RELATIVISTIC_TREATMENT : ZORA
```
