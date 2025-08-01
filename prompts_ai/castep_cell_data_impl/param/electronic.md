```
CHARGE (.param)
Keyword type
Real

Description
This keyword specifies the total charge of the system. It can be used instead of NELECTRONS.

It is not possible to specify the NELECTRONS, NUP, or NDOWN keywords when the CHARGE keyword is specified.

Default
0

Example
CHARGE : 3
```

```
NBANDS (.param)
Keyword type
Integer

Description
This keyword determines the maximum number of bands at any k-point and spin.

There are three ways in which you can specify the maximum number of bands at any k-point and spin:

Directly, using NBANDS.
Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using NEXTRA_BANDS.
This is the method used in the CASTEP interface.

Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value using PERC_EXTRA_BANDS.
It is not possible to have both the NBANDS keyword and either the NEXTRA_BANDS or PERC_EXTRA_BANDS keywords present in the same input file.

Example
NBANDS : 64
```

```
NDOWN (.param)
Keyword type
Real

Description
This keyword determines the total number of down-spin electrons.

This parameter is used only if SPIN_POLARIZED = TRUE.

Default
If the SPIN has been specified then:

	NDOWN : (NELECTRONS - SPIN)/2
If neither NDOWN nor SPIN have been specified:

	NDOWN : NELECTRONS/2
Example
NDOWN : 12
```

```
NELECTRONS (.param)
Keyword type
Real

Description
This keyword specifies the total number of electrons in the system.

Default
If the CHARGE keyword is specified, NELECTRONS will be chosen such that the total system charge is equal to the argument of CHARGE.

Alternatively, if NUP and NDOWN are specified, NELECTRONS will be the sum of the arguments of NUP and NDOWN.

If the number of electrons is not specified, a default value will be chosen such that the system is charge neutral.

Example
NELECTRONS : 3
```

```
NEXTRA_BANDS (.param)
Keyword type
Integer

Description
This keywords controls the number of extra bands in addition to the number of occupied bands. These extra bands are necessary for metals or finite temperature insulators.

It is not possible to have both the NBANDS keyword and either the NEXTRA_BANDS or PERC_EXTRA_BANDS keywords present in the same input file.

Default
0

Example
NEXTRA_BANDS : 12
```

```
NUP (.param)
Keyword type
Real

Description
This keyword determines the total number of up-spin electrons.

This parameter is used only if SPIN_POLARIZED = TRUE.

Default
If the SPIN has been specified then:

	NUP : (NELECTRONS + SPIN)/2
If neither NUP nor SPIN have been specified:

	NUP : NELECTRONS/2
Example
NUP : 12
```

```
SPIN (.param)
Keyword type
Real

Description
This keyword determines the initial value for the number of unpaired electrons in a spin-polarized calculation. This value may be optimized during the CASTEP calculation depending on the values of SPIN_FIX and FIX_OCCUPANCY.

The SPIN keyword cannot be used in conjunction with either of NUP or NDOWN keywords.

Default
0 when the total number of electrons in the system is even.

1 when the total number of electrons in the system is odd.

Example
SPIN : 3
```
