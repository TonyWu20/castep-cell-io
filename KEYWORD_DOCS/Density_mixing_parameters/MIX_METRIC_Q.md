# MIX_METRIC_Q

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_mix_metric_q_castep.htm

**Group:** Density mixing parameters

---

# MIX\_METRIC\_Q (.param)

## Keyword type

Real

## Description

This keyword determines the weighting factor for the densities used in the density mixing scheme.

CASTEP uses a weighting factor when evaluating scalar products of densities. The factor depends on the wave vector q, according to:

f(q) = (q2 + q12)/q2

where q1 is the value of the MIX\_METRIC\_Q parameter.

CASTEP sets the value of q1 automatically if MIX\_METRIC\_Q is not specified.

## Default

-1 - CASTEP will automatically select the appropriate value

## Example

```

MIX_METRIC_Q : 20.0 1/ang
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)