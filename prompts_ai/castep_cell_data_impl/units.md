This is the documentation of `MASS_UNIT`

```markdown
# MASS_UNIT (.param)

## Keyword type

String

## Description

This keyword specifies the units in which masses will be reported.

The available units and their associated identifiers are:

| Unit             | Identifier |
| ---------------- | ---------- |
| Electron mass    | me         |
| Atomic mass unit | amu        |
| Kilogram         | kg         |
| Gram             | g          |

## Default

amu

## Example

MASS_UNIT : kg
```

This is the documentation of `FORCE_UNIT`:

```markdown
# FORCE_UNIT (.param)

## Keyword type

String

## Description

This keyword specifies the units in which force will be reported.

The available units and their associated identifiers are:

| Unit                 | Identifier   |
| -------------------- | ------------ |
| Hartree per Bohr     | hartree/bohr |
| Electron volts per Å | ev/ang       |
| Newton               | n            |

## Default

ev/ang

## Example

FORCE_UNIT : n
```

This is the documentation of `INV_LENGTH_UNIT`

```markdown
# INV_LENGTH_UNIT (.param)

## Keyword type

String

## Description

This keyword specifies the units in which inverse length will be reported.

The available units and their associated identifiers are:

| Unit        | Identifier |
| ----------- | ---------- |
| Bohr-1      | 1/         |
| Meter-1     | 1/m        |
| Nanometer-1 | 1/nm       |
| Å-1         | 1/ang      |

Default
1/ang

Example
INV_LENGTH_UNIT : 1/nm
```

This is the documentation of `PRESSURE_UNIT`

```markdown
# PRESSURE_UNIT (.param)

## Keyword type

String

## Description

This keyword specifies the units in which pressure will be reported.

The available units and their associated identifiers are:

| Unit                  | Identifier        |
| --------------------- | ----------------- |
| Hartree per Bohr3     | hartree/bohr\*\*3 |
| Electron Volts per Å3 | ev/ang\*\*3       |
| Pascal                | pa                |
| Megapascal            | mpa               |
| Gigapascal            | gpa               |
| Atmosphere            | atm               |
| Bar                   | bar               |
| Megabar               | mbar              |

## Default

gpa

## Example

PRESSURE_UNIT : atm
```

```
ENERGY_UNIT (.param)
Keyword type
String

Description
This keyword specifies the units in which energies will be reported.

The available units and their associated identifiers are:

Unit	Identifier
Hartree	ha
Millihartree	mha
Electron Volt	ev
Milli-electron Volt	mev
Rydberg	ry
Millirydberg	mry
Kilojoules per mole	kj/mol
Kilocalories per mole	kcal/mol
Joules	j
Erg	erg
Hertz	hz
Megahertz	mhz
Gigahertz	ghz
Terahertz	thz
Wavenumber	cm-1
Kelvin	k

Default
ev

Example
ENERGY_UNIT : kcal/mol
```

```
FORCE_CONSTANT_UNIT (.param)
Keyword type
String

Description
This keyword specifies the units in which force constants will be reported.

The available units and their associated identifiers are:

Unit	Identifier
Hartree per Bohr2	hartree/bohr**2
Electron Volts per Å2	ev/ang**2
Newton per meter	n/m
Dynes per centimeter	dyne/cm
Default
ev/ang**2

Example
FORCE_CONSTANT_UNIT : n/m
```

```
FREQUENCY_UNIT (.param)
Keyword type
String

Description
This keyword specifies the units in which frequency will be reported.

The available units and their associated identifiers are:

Unit	Identifier
Hartree	ha
Millihartree	mha
Electron Volt	ev
Milli-electron Volt	mev
Rydberg	ry
Millirydberg	mry
Kilojoules per mole	kj/mol
Kilocalories per mole	kcal/mol
Joules	j
Erg	erg
Hertz	hz
Megahertz	mhz
Gigahertz	ghz
Terahertz	thz
Wavenumber	cm-1
Kelvin	k
Default
cm-1

Example
FREQUENCY_UNIT : hz
```

```
TIME_UNIT (.param)
Keyword type
String

Description
This keyword specifies the units in which time will be reported.

The available units and their associated identifiers are:

Unit	Identifier
Atomic unit of time	aut
Second	s
Millisecond	ms
Microsecond	mus
Nanosecond	ns
Picosecond	ps
Femtosecond	fs
Default
ps

Example
TIME_UNIT : aut
```

```
VELOCITY_UNIT (.param)
Keyword type
String

Description
This keyword specifies the units in which velocity will be reported.

The available units and their associated identifiers are:

Unit	Identifier
Atomic unit of velocity	auv
Å/ps	ang/ps
Å/fs	ang/fs
Bohr per picosecond	bohr/ps
Bohr per femtosecond	bohr/fs
Meters per second	m/s
Default
ang/ps

Example
VELOCITY_UNIT : bohr/fs
```

```
VOLUME_UNIT (.param)
Keyword type
String

Description
This keyword specifies the units in which volume will be reported.

The available units and their associated identifiers are:

Unit	Identifier
Bohr3	bohr**3
Meter3	m**3
Centimeter3	cm**3
Nanometer3	nm**3
Ångstrom3	ang**3
Default
ang**3

Example
VOLUME_UNIT : nm**3
```
