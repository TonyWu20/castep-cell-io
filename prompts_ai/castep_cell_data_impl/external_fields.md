This is the documentation of `EXTERNAL_EFIELD`

```
EXTERNAL_EFIELD (.cell)
Keyword type
Block

Description
This data block contains electric field vector in Cartesian coordinates.

The data block has the following format:

%BLOCK EXTERNAL_EFIELD
[units]
Ex Ey Ez
%ENDBLOCK EXTERNAL_EFIELD
The first line is optional and contains electric field units. The default unit is eV/Å/electron (eV/Ang/e in CASTEP notation). The next line contains three real numbers representing electric field vector components in Cartesian coordinates.

The external electric field is applied in a sawtooth formalism. Electric field discontinuity is on the cell boundary, so the only sensible systems to study using this approach are molecules or slabs (the field must be along the surface normal in the latter case). In both cases atoms should be positioned in the middle of the cell.

Example
%BLOCK EXTERNAL_EFIELD
HARTREE/BOHR/E
0.0 0.0 0.1
%ENDBLOCK EXTERNAL_EFIELD
```

This is the documentation of `EXTERNAL_PRESSURE`:

```
EXTERNAL_PRESSURE (.cell)
Keyword type
Block

Description
This data block contains the external pressure tensor. Positive external pressure corresponds to compression and a negative pressure to tension. The data block has the following format:

%BLOCK EXTERNAL_PRESSURE
[units]
		Rxx	        Rxy	        Rxz
			        Ryy	        Ryz
				                Rzz
%ENDBLOCK EXTERNAL_PRESSURE
Where, Rxx is the xx-component of the pressure, Rxy is the xy-component, and so on.

[units] specifies the units in which the pressure is defined. If no units are given, the default of Gigapascals is used.

By default, no external pressure is applied.

Example
%BLOCK EXTERNAL_PRESSURE
GPa
    5.0000000000    0.0000000000    0.0000000000
                    5.0000000000    0.0000000000
                                    5.0000000000
%ENDBLOCK EXTERNAL_PRESSURE
```
