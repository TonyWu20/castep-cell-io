The documentatio of `KPOINTS_LIST` block is:

> KPOINTS_LIST (.cell)
> Keyword type
> Block
>
> Description
> This data block contains a list of k-points at which the Brillouin zone will be sampled during a self consistent calculation to find the electronic ground state, along with the associated weights. It has the following format:
>
> %BLOCK KPOINTS_LIST
> R1i R1j R1k R1w
> R2i R2j R2k R2w
> .
> .
> .
> %ENDBLOCK KPOINTS_LIST
> The first three entries on a line are the fractional positions of the k-point relative to the reciprocal space lattice vectors.
>
> The final entry on a line is the weight of the k-point relative to the others specified. The sum of the weights must be equal to 1.

My orders:

1. Define a struct representing the `KPOINTS_LIST` block.
2. Define a struct to represent the entries in each line of the block. For example:
   ```rust
   struct Kpoint {
       coord: [f64;3],
       weight: [f64]
   }
   ```
3. Add `#derive` for the block struct and line struct and implement the traits as demonstrated in the first uploaded example.
