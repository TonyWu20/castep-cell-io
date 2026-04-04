# SUPERCELL_KPOINT_LIST_CASTEP

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_supercell_kpoint_list_castep.htm

**Group:** q-vectors for phonon calculations

---

# SUPERCELL\_KPOINT\_LIST (.cell)

## Keyword type

Block

## Description

This data block contains a list of k-points at which the Brillouin zone will be sampled during a supercell FD phonon calculation, along with
the associated weights. It has the same format as [KPOINTS\_LIST](k_kpoints_list_castep.htm).

## Example

```

%BLOCK SUPERCELL_KPOINT_LIST
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
%ENDBLOCK SUPERCELL_KPOINT_LIST
```

###### See Also:

[KPOINTS\_LIST](k_kpoints_list_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)