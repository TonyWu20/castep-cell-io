# EXTERNAL_PRESSURE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_external_pressure_castep.htm

**Group:** External fields

---

# EXTERNAL\_PRESSURE (.cell)

## Keyword type

Block

## Description

This data block contains the external pressure tensor. Positive external pressure corresponds to compression and a negative pressure to
tension. The data block has the following format:

```

%BLOCK EXTERNAL_PRESSURE
[units]
		Rxx	        Rxy	        Rxz
			        Ryy	        Ryz
				                Rzz
%ENDBLOCK EXTERNAL_PRESSURE
```

Where, Rxx is the xx-component of the pressure, Rxy is the xy-component, and so on.

[units] specifies the units in which the pressure is defined. If no units are given, the default of Gigapascals is used.

By default, no external pressure is applied.

## Example

```

%BLOCK EXTERNAL_PRESSURE
GPa
    5.0000000000    0.0000000000    0.0000000000
                    5.0000000000    0.0000000000
                                    5.0000000000
%ENDBLOCK EXTERNAL_PRESSURE
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
