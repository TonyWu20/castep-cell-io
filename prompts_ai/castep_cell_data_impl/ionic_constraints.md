This is the documentation of the block `IONIC_CONSTRAINTS`:

```
IONIC_CONSTRAINTS (.cell)
Keyword type
Block

Description
This data block defines constraints on changes to the Cartesian ionic positions during relaxation or molecular dynamics. These are specified as a set of linear constraints. Each constraint is made up of a series of coefficients $a_{ijk} such that:

$$
\sum_{k}^{x}\sum_{j}^{y}\sum_{i}^{3}a_{ijk}^{ionic positions(i,j,k)=constant}
$$

Where:

x is the number of species in the system
y is the number of ions in species k

The data block has the following format:

%BLOCK IONIC_CONSTRAINTS
	I1 CCC1s/I1s In1 R1i R1j R1k
	I2 CCC2s/I2s In2 R2i R2j R2k
	.
	.
	.
%ENDBLOCK IONIC_CONSTRAINTS
The first element on each line is an integer specifying the number of the constraint being specified. The second entry is either the symbol or atomic number of the species of the ion to which this constraint applies. The third element is the number of the ion within the species.

The ordering of the ions in a species is the order in which they appear in the POSITIONS_FRAC or POSITIONS_ABS block in the cell definition file.

The final three numbers are real numbers representing the coefficients of the Cartesian coordinates of the ionic position in the constraint sum. All coefficients in the sum that are not explicitly specified will be set to zero.

On reading this data, the matrix of ionic constraints will be orthogonalized.

Fixing Cartesian coordinates together with cell optimization is not well defined procedure, as it is not clear what exactly should be kept (each atom has an infinite set of periodically repeated images and if translation vector is changed under optimization step Cartesian coordinates of all images cannot be kept the same). At the same time fixing all fractional coordinates of an atom is still perfectly valid operation under this circumstances, thus if all the Cartesian coordinate of an atom are asked to be fixed CASTEP will fix fractional coordinates of that atom instead.

Example
To fix the Cartesian coordinates of the first four tungsten atoms in a cell:

%BLOCK IONIC_CONSTRAINTS
       1       W       1    1.0000000000    0.0000000000    0.0000000000
       2       W       1    0.0000000000    1.0000000000    0.0000000000
       3       W       1    0.0000000000    0.0000000000    1.0000000000
       4       W       2    1.0000000000    0.0000000000    0.0000000000
       5       W       2    0.0000000000    1.0000000000    0.0000000000
       6       W       2    0.0000000000    0.0000000000    1.0000000000
       7       W       3    1.0000000000    0.0000000000    0.0000000000
       8       W       3    0.0000000000    1.0000000000    0.0000000000
       9       W       3    0.0000000000    0.0000000000    1.0000000000
      10       W       4    1.0000000000    0.0000000000    0.0000000000
      11       W       4    0.0000000000    1.0000000000    0.0000000000
      12       W       4    0.0000000000    0.0000000000    1.0000000000
%ENDBLOCK IONIC_CONSTRAINTS
```

Feedback:
During the deserialization, we have to match the entries of the line one by one.
If we want to group the final three entries into `[f64;3]` we have to use an IR
to convert, like:

```rust
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(from = "ICRepr")]
    pub struct IonicConstraintEntry {
        /// The constraint number.
        pub constraint_number: u32,
        /// The species of the ion.
        pub species: Species,
        /// The ion number within the species (1-based index).
        pub ion_number: u32,
        /// Coefficients for the Cartesian coordinates [x, y, z].
        pub coefficients: [f64; 3],
    }

    #[derive(Debug, Deserialize)]
    struct ICRepr {
        /// The constraint number.
        pub constraint_number: u32,
        /// The species of the ion.
        pub species: Species,
        /// The ion number within the species (1-based index).
        pub ion_number: u32,
        /// Coefficients for the Cartesian coordinates [x, y, z].
        pub ri_x: f64,
        /// Coefficients for the Cartesian coordinates [x, y, z].
        pub ri_y: f64,
        /// Coefficients for the Cartesian coordinates [x, y, z].
        pub ri_z: f64,
    }

    impl From<ICRepr> for IonicConstraintEntry {
        fn from(value: ICRepr) -> Self {
            let ICRepr {
                constraint_number,
                species,
                ion_number,
                ri_x,
                ri_y,
                ri_z,
            } = value;
            Self {
                constraint_number,
                species,
                ion_number,
                coefficients: [ri_x, ri_y, ri_z],
            }
        }
    }
```

Please remember this.
Moreover, I prefer to implement `ToCellValue` like this, instead of pushing entries to a `mut Vec<CellValue>`:

```rust
    impl ToCellValue for IonicConstraintEntry {
        /// Converts the entry into a `CellValue::Array` representing one line of the block.
        fn to_cell_value(&self) -> CellValue {
            CellValue::Array(
                vec![
                    CellValue::UInt(self.constraint_number),
                    self.species.to_cell_value(),
                    CellValue::UInt(self.ion_number),
                ]
                .into_iter()
                .chain(self.coefficients.into_iter().map(CellValue::Float))
                .collect(),
            )
        }
    }
```
