# castep-cell-parser

Parser for `.cell` of `Castep`. Strive to provide `Serialize` and `Deserialize` support to use with `Serde`.

## Mechanism of the lib

There are the underlying hierarchies for the data and format:

1. Top: The `.cell` text file, which has its format defined by `castep`.
2. Middle: a format that is serialized/deserialized by `serde`.
3. Bottom: `struct`s that correspond to the data types in `.cell`.

- A parser is needed for the conversion of **1-2**;
- An implementation of `std::fmt::Display` is needed for the conversion of **3-1**;
- Implementation of `Serializer` and `Deserializer` to convert between **2** and **3**;
