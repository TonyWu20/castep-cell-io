This is the documentation of `SYMMETRY_OPS`.
Since a symmetry operation is bound to take 4 lines, I think we can deserialize into a `struct SymmetryOpsRepr {matrix_lines: Vec<[f64;3]>}`first,
and then implement a conversion to `struct SymmetryOps {ops: Vec<SymmetryOpsEntry>}`,
by chunking the `Vec<[f64;3]>` by 4.

```
SYMMETRY_OPS (.cell)
Keyword type
Block

Description
This data block contains the symmetry operations under which the unit cell is invariant. Each operation is represented by a 3 × 3 array. The data block has the following format:

%BLOCK SYMMETRY_OPS
	R11        R21        R31
	R12        R22        R32
	R13        R23        R33
	T1         T2         T3
	R11        R21        R31
	R12        R22        R32
	R13        R23        R33
	T1         T2         T3
	.
	.
	.
%ENDBLOCK SYMMETRY_OPS
Each of the first three lines contains 3 entries representing a row of a 3 × 3 array. These represent one symmetry rotation. The three entries on the following line contain the translation associated with this rotation.

If no symmetry is specified in the cell definition file, the default is for no symmetry to be applied.

Example
%BLOCK SYMMETRY_OPS
	.
	.
	.
   -1.0000000000    0.0000000000    0.0000000000
    0.0000000000   -1.0000000000    0.0000000000
    0.0000000000    0.0000000000    1.0000000000
    0.5000000000    0.0000000000    0.5000000000
	.
	.
	.
%ENDBLOCK SYMMETRY_OPS
```

This is the documentation of `SYMMETRY_TOL`.
Note the length unit is required. The `LengthUnit` we previously defined is in the path `crate::units::LengthUnit`.

```
SYMMETRY_TOL (.cell)
Keyword type
Real

Description
This keyword controls the tolerance within which symmetry will be considered to be satisfied. If an ion is found within this distance of its symmetric position, the symmetry will be considered to be satisfied. Unit of length must be specified.

Default
0.01 ang

Example
SYMMETRY_TOL : 0.25 ang
```
