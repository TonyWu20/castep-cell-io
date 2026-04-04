# MAGRES_KPOINTS_LIST

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magres_kpoints_list_castep.htm

**Group:** Brillouin zone sampling k-points

---

# MAGRES\_KPOINTS\_LIST (.cell)

## Keyword type

Block

## Description

This data block contains a list of k-points at which the Brillouin zone will be sampled during an NMR calculation.

NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if
you have purchased this module.

The MAGRES\_KPOINTS\_LIST data block has the following format:

```

%BLOCK MAGRES_KPOINTS_LIST
	R1i     R1j     R1k     R1w
	R2i     R2j     R2k     R2w
	.
	.
	.
%ENDBLOCK MAGRES_KPOINTS_LIST
```

The first three entries on a line are the fractional positions of the k-point relative to the reciprocal space lattice vectors. The final
entry on a line is the weight of the k-point relative to the others specified. The sum of the weights must be equal to 1.

## Example

```

%BLOCK MAGRES_KPOINTS_LIST
    0.3333333333    0.3750000000    0.3333333333    0.2222222222
    0.3333333333    0.3750000000    0.0000000000    0.1111111111
    0.3333333333    0.1250000000    0.3333333333    0.2222222222
    0.3333333333    0.1250000000    0.0000000000    0.1111111111
    0.0000000000    0.3750000000    0.3333333333    0.1111111111
    0.0000000000    0.3750000000    0.0000000000    0.0555555556
    0.0000000000    0.1250000000    0.3333333333    0.1111111111
    0.0000000000    0.1250000000    0.0000000000    0.0555555556
%ENDBLOCK MAGRES_KPOINTS_LIST
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)