These are submodules of module `density_mixing`

```
MIX_CHARGE_AMP (.param)
Keyword type
Real
Description
This keyword determines the mixing amplitude for the charge density in the density mixing procedure.
Default
0.8
Example MIX_CHARGE_AMP : 0.5
```

```
MIXING_SCHEME (.param)
Keyword type
String
Description
This keyword determines which mixing scheme will be used in the density mixing procedure. Available options are:
Kerker
Linear
Broyden
Pulay
Default
Broyden
Example MIXING_SCHEME : Pulay
```

```
MIX_SPIN_AMP (.param)
Keyword type
Real
Description
This keyword determines the mixing amplitude for the spin density in the density mixing procedure.
Default
2.0
Example MIX_SPIN_AMP : 1.754
```

```
MIX_METRIC_Q (.param)
Keyword type
Real
Description
This keyword determines the weighting factor for the densities used in the density mixing scheme.
CASTEP uses a weighting factor when evaluating scalar products of densities. The factor depends on the wave vector q, according to:
f(q) = (q 2  + q 1 2 )/q 2
where q 1  is the value of the MIX_METRIC_Q parameter.
CASTEP sets the value of q 1  automatically if MIX_METRIC_Q is not specified.
Default
-1  - CASTEP will automatically select the appropriate value
Example MIX_METRIC_Q : 20.0 1/ang
```

```
MIX_CHARGE_GMAX (.param)
Keyword type
Real
Description
This keyword determines the maximum g-vector at which the charge density is mixed in the density mixing procedure.
Default
1.5 Å -1
Example MIX_CHARGE_GMAX : 0.89 1/ang
```

```
MIX_HISTORY_LENGTH (.param)
Keyword type
Integer
Description
This keyword determines the maximum number of densities to store in the history used during the density mixing procedure.
Default
7
Example MIX_HISTORY_LENGTH : 5
```

```
MIX_SPIN_GMAX (.param)
Keyword type
Real
Description
This keyword determines the maximum g-vector at which the spin density is mixed in the density mixing procedure.
Default
1.5 Å -1
Example MIX_SPIN_GMAX : 0.89 1/ang
```

```
MIX_CUT_OFF_ENERGY (.param)
Keyword type
Real
Description
This keyword determines the cutoff energy for the densities used in the density mixing scheme.
The value specified determines the extent of the grid used for mixing old and new densities. Density components with wave vectors higher
than that specified are not mixed.
Default
The default value is the  CUT_OFF_ENERGY .
Example MIX_CUT_OFF_ENERGY : 250.0 eV
```
