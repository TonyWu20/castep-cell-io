This is the documentation of `SPECIES_LCAO_STATES`:

```
SPECIES_LCAO_STATES (.cell)
Keyword type
Block

Description
This data block defines the size of the LCAO basis set used for population analysis. It has the following format:

%BLOCK SPECIES_LCAO_STATES
	CCC1/I1 	IB1
	CCC2/I2 	IB2
	.
	.
	.
%ENDBLOCK SPECIES_LCAO_STATES
The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of the species in the POSITIONS_FRAC or POSITIONS_ABS block. The second entry is the number of angular momentum channels to use in the LCAO basis set for the species when performing population analysis.

For example, to use the 2s and 2p states for C (the 1s state is a core state) this value should be 2. By default, the number of states will be the appropriate number to complete the valence shell to the next noble gas. If shallow core states are excluded from a pseudopotential, the value of SPECIES_LCAO_STATES for that species should be included in the cell file to ensure a meaningful basis set is used.

Example
%BLOCK SPECIES_LCAO_STATES
       O         2
      Al         2
      Ti         3
      Cs         4
%ENDBLOCK SPECIES_LCAO_STATES
```

This is the documentation of `SPECIES_MASS`

```
SPECIES_MASS (.cell)
Keyword type
Block

Description
This data block contains mass of each atomic species. It has the following format:

%BLOCK SPECIES_MASS
[units]
	CCC1/I1 	R1
	CCC2/I2 	R2
	.
	.
	.
%ENDBLOCK SPECIES_MASS
The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of the species in the POSITIONS_FRAC or POSITIONS_ABS block. The second entry on each line is the mass of that species.

Not all of the species present in the system have to appear in the SPECIES_MASS block. Any that are not present will be assigned the default mass for that species. However, if the symbol specified for a species is not a standard symbol in the periodic table, the mass of the species must be specified.

[units] specifies the units in which the masses are defined. If no units are given, the default of atomic mass units is used.

Example
%BLOCK SPECIES_MASS
       O     15.9989995956
      Al     26.9820003510
      Ti     47.9000015259
      Cs    132.9049987793
%ENDBLOCK SPECIES_MASS
```

This is the documentation of `SPECIES_POT`. Our `Species` path is `super::Species`.

```
SPECIES_POT (.cell)
Keyword type
Block

Description
This data block contains the names of the pseudopotential files associated with each species. It has the following format:

%BLOCK SPECIES_POT
	CCC1/I1 	[filename]
	CCC2/I2 	[filename]
	.
	.
	.
%ENDBLOCK SPECIES_POT
The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of the species in the POSITIONS_FRAC or POSITIONS_ABS block. The second entry on each line is the name of the file containing the definition of the pseudopotential representing that ionic species.

The filename qualifier in the SPECIES_POT block may be either the name of the file that contains the tabulated pseudopotential on a grid (can be either ultrasoft or norm-conserving) or a text string containing the definition of the pseudopotential which is to be generated at runtime (on-the-fly generation).

Not all of the species present in the system have to appear in the SPECIES_POT block. Any that are not present will be assigned the default pseudopotential for that species, which will be generated on the fly. However, if the symbol specified for a species is not a standard symbol in the periodic table, the potential for the species must be specified.

The charge on the ion for each species will be derived from the pseudopotential corresponding to that ion.

Example
%BLOCK SPECIES_POT
       O  O_00.usp
      Al  Al_00.usp
      Ti  Ti_00.uspcc
      Cs  Cs_00.usp
%ENDBLOCK SPECIES_POT
```

This is the documentation of `HUBBARD_U`

```
HUBBARD_U (.cell)
Keyword type
Block

Description
This data block defines the Hubbard U values to use for specific orbitals, with the following format:

%BLOCK HUBBARD_U
 UNITS
	atom1  orbital1 orbital2 ....
	atom2  orbital1 orbital2 ....
	.
	.
	.
%ENDBLOCK HUBBARD_U
The first (optional) line declares the units to use for Hubbard U values, eV (default) or Ha. Each following line describes the orbitals affected by the Hubbard term.

The first element on each obligatory line (atom) describes the ion and can contain either only the species symbol for the ion (in which case the same Hubbard U values will be applied to all ions with this name) or both the species symbol for the ion and its number within the species.

The order of the ions for a species is the order in which they appear in the POSITIONS_FRAC or POSITIONS_ABS block in the cell definition file.

After an ion is defined, the Hubbard U value for each orbital of the ion can be defined, with the format:

l: U
where l represents the s, p, d, or f orbital and U is a real number specifying the value of U, for example:

d: 2.3
All the values which are not defined are taken to be zero. If all the values for all orbitals are zero, LDA+U is turned off.

Example
%BLOCK HUBBARD_U
  eV
    Sm 1   f: 6.1
    Ni     d: 2.4
    U 2  d: 1.2 f: 2.1
%ENDBLOCK HUBBARD_U
This input defines the following Hubbard U values:

f orbitals of the first Sm ion are 6.1 eV
d orbitals of all Ni ions are 2.4 eV
d orbitals of second U ion are 1.2 eV and its f orbitals are 2.1 eV
```

Use this enum to cover all the cases for a line:

```rust
    /// Represents the specification for Hubbard U values for a specific atom/ion.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    enum AtomHubbardURepr {
        NoIonOne(Species, OrbitalU),
        IonOne(Species, u32, OrbitalU),
        NoIonTwo(Species, OrbitalU, OrbitalU),
        IonTwo(Species, u32, OrbitalU, OrbitalU),
        NoIonThree(Species, OrbitalU, OrbitalU, OrbitalU),
        IonThree(Species, u32, OrbitalU, OrbitalU, OrbitalU),
        NoIonFour(Species, OrbitalU, OrbitalU, OrbitalU, OrbitalU),
        IonFour(Species, u32, OrbitalU, OrbitalU, OrbitalU, OrbitalU),
    }
```
