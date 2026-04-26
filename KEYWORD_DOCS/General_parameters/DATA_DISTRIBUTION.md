# DATA_DISTRIBUTION

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_data_distribution_castep.htm

**Group:** General parameters

---

# DATA\_DISTRIBUTION (.param)

## Keyword type

String

## Description

This keyword determines the parallelization strategy used. Available options are:

* Kpoint - only k-point parallelization will be used (best scalability)
* Gvector - only g-vector parallelization will be used (worst scalability)
* Mixed - a combination of k-point and g-vector parallelization will be used
* Default - the optimal parallelization strategy for the architecture will be used

The Default setting is appropriate in most situations.

The Kpoint option is available only when the number of
processors is a factor of the number of k-points.

The Gvector option is appropriate for calculations involving large
systems where often only one k-point is used.

The Mixed option is available when the number of processors and the
number of k-points have a common factor (for example 6 k-points and 4 processors).

The differences in the scaling properties of the different schemes dictate the number of processors that should be used. For example, a
calculation that uses 16 k-points is likely to finish faster on 4 processors (using Kpoint distribution) than on 6
processors (using Mixed distribution) and most likely faster than on 9 processors (using Gvector
distribution).

## Default

Default

## Example

```

DATA_DISTRIBUTION : Gvector
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
