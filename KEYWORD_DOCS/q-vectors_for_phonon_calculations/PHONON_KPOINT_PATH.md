# PHONON_KPOINT_PATH

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_kpoint_path_castep.htm

**Group:** q-vectors for phonon calculations

---

# PHONON\_KPOINT\_PATH (.cell)

## Keyword type

Block

## Description

This data block defines a path through reciprocal space, on which lie the k-points used for phonon calculations. It has the following format:

```

%BLOCK PHONON_KPOINT_PATH
	R1i     R1j     R1k 
	R<2i    R2j     R2k 
	.
	.
	.
%ENDBLOCK PHONON_KPOINT_PATH
```

The three numbers on each line are the fractional positions of the k-point relative to the reciprocal space lattice vectors. The k-points
define a continuous sequence of straight line segments. The path will be open unless the first and last point in the list are identical.

The maximum spacing of the points sampled along each line segment is defined by
[PHONON\_KPOINT\_PATH\_SPACING](k_phonon_kpoint_path_spacing_castep.htm).

## Example

```

%BLOCK PHONON_KPOINT_PATH
    0.3333333333    0.3750000000    0.3333333333
    0.3333333333    0.3750000000    0.0000000000
    0.3333333333    0.1250000000    0.3333333333
    0.3333333333    0.1250000000    0.0000000000
    0.0000000000    0.3750000000    0.3333333333
    0.0000000000    0.3750000000    0.0000000000
    0.0000000000    0.1250000000    0.3333333333
    0.0000000000    0.1250000000    0.0000000000
%ENDBLOCK PHONON_KPOINT_PATH
```

###### See Also:

[PHONON\_KPOINT\_PATH\_SPACING](k_phonon_kpoint_path_spacing_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
