# PHONON_SUPERCELL_MATRIX

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_supercell_matrix_castep.htm

**Group:** q-vectors for phonon calculations

---

# PHONON\_SUPERCELL\_MATRIX (.cell)

## Keyword type

Block

## Description

The supercelling matrix for force constant matrix calculations. The supercell matrix is specified by a 3 × 3 integer matrix which gives the
supercell used in finite-difference phonon calculations.

## Example

```

%BLOCK PHONON_SUPERCELL_MATRIX
 2 2 0
 0 2 2
 2 0 2
%ENDBLOCK PHONON_SUPERCELL_MATRIX
```

###### See Also:

[KPOINTS\_LIST](k_kpoints_list_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)