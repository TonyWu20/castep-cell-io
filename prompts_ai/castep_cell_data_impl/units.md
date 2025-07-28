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
