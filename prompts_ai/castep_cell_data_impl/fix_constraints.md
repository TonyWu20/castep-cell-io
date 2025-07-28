This is the documentation of a key-value pair `FIX_ALL_CELL`:

```
FIX_ALL_CELL (.cell)
Keyword type
Logical

Description
This keyword controls whether or not all of the lattice parameters remain fixed during relaxation or molecular dynamics.

Default
FALSE

Example
FIX_ALL_CELL : TRUE
```

This is the documentation of a key-value pair `FIX_ALL_IONS`:

```
FIX_ALL_IONS (.cell)
Keyword type
Logical

Description
This keyword controls whether or not all of the ionic positions remain fixed during relaxation or molecular dynamics.

Default
FALSE

Example
FIX_ALL_IONS : TRUE
```

Feedback:
Here are the `clippy`'s suggestions:

1. Instead of
   ```rust
   println!( "Serialized FIX_ALL_IONS (FALSE): {}", serialized_string_false);
   ```
   Do
   ```rust
    println!( "Serialized FIX_ALL_IONS (FALSE): {serialized_string_false}");
   ```
2. Instead of
   ```rust
   assert_eq!(cell_file_true.fix_all_ions.0, true);
   assert_eq!(cell_file_false.fix_all_ions.0, false);
   ```
   Do
   ```rust
   assert!(cell_file_true.fix_all_ions.0);
   assert!(!cell_file_false.fix_all_ions.0);
   ```

This is the documentation of a key-value pair `FIX_COM`:

```
FIX_COM (.cell)
Keyword type
Logical

Description
This keyword controls whether or not the center of mass of the ions remains fixed during relaxation or molecular dynamics.

Default
If FIX_ALL_IONS : FALSE then the default value is TRUE.

Example
FIX_COM : TRUE
```

This is the documentation of a key-value pair `FIX_VOL`:

```
FIX_VOL (.cell)
Keyword type
Logical

Description
This keyword controls whether or not the volume of the cell remains fixed during relaxation or molecular dynamics. Cell angles and cell lengths may still be varied.

Default
FALSE

Example
FIX_VOL : TRUE
```

This is the documentation of a block `NONLINEAR_CONSTRAINTS`:

```
NONLINEAR_CONSTRAINTS (.cell)
Keyword type
Block

Description
This data block defines constrained internal coordinates (bonds, angles, torsions). These internals are held fixed at the values of the initial structure.

The data block has the following format:

%BLOCK NONLINEAR_CONSTRAINTS
	CONSTRAIN_TYPE atom1 atom2 (atom3 atom4)
	CONSTRAIN_TYPE atom1 atom2 (atom3 atom4)
	.
	.
	.
%ENDBLOCK NONLINEAR_CONSTRAINTS
The first element on each line is a constraint type (distance, bend or torsion). Then depending on the type of the constraint two, three or for fields describing atoms involved in the constraint appears. Each field has the following format:

CCC  I N1 N2 N3
The first entry is the symbol species of the ion to which this constraint applies. The second element is the number of the ion within the species. The final three numbers are integer numbers specifying the periodic image where the atom is located.

The ordering of the ions in a species is the order in which they appear in the POSITIONS_FRAC or POSITIONS_ABS block in the cell definition file.

Example
%BLOCK NONLINEAR_CONSTRAINTS
distance       H  4  0  0  0       O  2  0  1  0
    bend       H  5  0  0  0       C  1  1  0  1       H  2  0  0  0
 torsion       H  6  0  0  0       H  3  1  0  0       H  1  0  0  1       H  9  1  1  0
%ENDBLOCK NONLINEAR_CONSTRAINTS
This input fixes:

the distance between the fourth hydrogen atom and second oxygen atom in the adjacent ( 0 1 0) unit cell
the bend defined by the fifth hydrogen atom, the first carbon atom in the (1 0 1) cell, and the second hydrogen atom
the torsion defined by hydrogen atoms: the sixth in the original cell, the third in the cell (1 0 0), the first in the cell (0 0 1), and the ninth in the (1 1 0)
```
