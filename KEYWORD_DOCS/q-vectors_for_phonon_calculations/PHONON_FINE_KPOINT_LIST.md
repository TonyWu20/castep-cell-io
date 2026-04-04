# PHONON_FINE_KPOINT_LIST

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_fine_kpoint_list_castep.htm

**Group:** q-vectors for phonon calculations

---

# PHONON\_FINE\_KPOINT\_LIST (.cell)

## Keyword type

Block

## Description

Phonon frequencies are calculated on a coarse set of wavevectors using DFPT and interpolated onto a list of k-points defined by this data
block, it has the same format as [KPOINTS\_LIST](k_kpoints_list_castep.htm).

## Example

```

%BLOCK PHONON_FINE_KPOINT_LIST
    0.0000000000    0.0000000000    0.0000000000    0.0800000000
    0.0000000000    0.0833333333    0.0000000000    0.0400000000
    0.0000000000    0.1666666667    0.0000000000    0.0400000000
    0.0000000000    0.2500000000    0.0000000000    0.0400000000
    0.0000000000    0.3333333333    0.0000000000    0.0400000000
    0.0000000000    0.4166666667    0.0000000000    0.0400000000
    0.0000000000    0.5000000000    0.0000000000    0.0400000000
    0.0000000000    0.5000000000    0.0833333333    0.0400000000
    0.0000000000    0.5000000000    0.1666666667    0.0400000000
    0.0000000000    0.5000000000    0.2500000000    0.0400000000
    0.0000000000    0.5000000000    0.3333333333    0.0400000000
    0.0000000000    0.5000000000    0.4166666667    0.0400000000
    0.0000000000    0.5000000000    0.5000000000    0.0400000000
    0.0000000000    0.4166666667    0.5000000000    0.0400000000
    0.0000000000    0.3333333333    0.5000000000    0.0400000000
    0.0000000000    0.2500000000    0.5000000000    0.0400000000
    0.0000000000    0.1666666667    0.5000000000    0.0400000000
    0.0000000000    0.0833333333    0.5000000000    0.0400000000
    0.0000000000    0.0000000000    0.5000000000    0.0400000000
    0.0000000000    0.0000000000    0.4166666667    0.0400000000
    0.0000000000    0.0000000000    0.3333333333    0.0400000000
    0.0000000000    0.0000000000    0.2500000000    0.0400000000
    0.0000000000    0.0000000000    0.1666666667    0.0400000000
    0.0000000000    0.0000000000    0.0833333333    0.0400000000
%ENDBLOCK PHONON_FINE_KPOINT_LIST
```

###### See Also:

[KPOINTS\_LIST](k_kpoints_list_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)