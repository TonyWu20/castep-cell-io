I implemented the top level file deserializer as:

```rust
use serde::{
    Deserializer,
    de::{MapAccess, SeqAccess, value::StrDeserializer},
    forward_to_deserialize_any,
};

use crate::{Cell, CellValue, error::Error};

use super::primitive::CellValueDeserializer;

pub struct CellFileDeserializer<'de> {
    entries: &'de [Cell<'de>],
}

impl<'de> CellFileDeserializer<'de> {
    pub fn new(entries: &'de [Cell<'de>]) -> Self {
        Self { entries }
    }
}

struct CellMapAccess<'a, 'de> {
    inner: &'a mut CellFileDeserializer<'de>,
    current_entry: usize,
}

impl<'a, 'de> CellMapAccess<'a, 'de> {
    fn new(inner: &'a mut CellFileDeserializer<'de>) -> Self {
        Self {
            inner,
            current_entry: 0,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for CellMapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.inner
            .entries
            .iter()
            .enumerate()
            .next()
            .map(|(idx, cell_item)| {
                seed.deserialize(StrDeserializer::new(cell_item.key()))
            })
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.inner.entries.ge {
            Some(entry) => match &self.inner.entries[entry] {
                Cell::KeyValue(_, cell_value) => {
                    seed.deserialize(&mut CellValueDeserializer::new(cell_value))
                }
                Cell::Block(_, cell_value) => {
                    seed.deserialize(&mut BlockDeserializer::new(cell_value))
                }
                Cell::Flag(_f) => seed
                    .deserialize(&mut CellValueDeserializer::new(&CellValue::Bool(true))),
            },
            None => Err(Error::Message(
                "entry not found, value deserialize failed".to_string(),
            )),
        }
    }
}

impl<'de> Deserializer<'de> for &mut CellFileDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(CellMapAccess::new(&mut *self))
    }

    /// For the top level `CellFile`
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64
        f32 f64 char str string bytes byte_buf unit unit_struct tuple tuple_struct enum identifier
        newtype_struct ignored_any option seq
    }
}
```

The following is the test:

```rust
#[derive(Debug, Deserialize, PartialEq)]
#[serde(from = "IonicPositionRepr")]
pub struct IonicPosition {
    element: String,
    coord: [f64; 3],
    spin: Option<f64>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum IonicPositionRepr<'a> {
    Essential(&'a str, f64, f64, f64),
    WithSpin(&'a str, f64, f64, f64, &'a str, f64),
}

impl<'a> From<IonicPositionRepr<'a>> for IonicPosition {
    fn from(value: IonicPositionRepr) -> Self {
        match value {
            IonicPositionRepr::Essential(elm, x, y, z) => Self {
                element: elm.to_string(),
                coord: [x, y, z],
                spin: None,
            },
            IonicPositionRepr::WithSpin(elm, x, y, z, _spin_str, spin) => Self {
                element: elm.to_string(),
                coord: [x, y, z],
                spin: Some(spin),
            },
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(from = "PositionsFracRepr")]
pub struct PositionsFrac {
    pub unit: Option<String>,
    pub positions: Vec<IonicPosition>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PositionsFracRepr<'a> {
    Essential(Vec<IonicPosition>),
    WithUnit(&'a str, Vec<IonicPosition>),
}

#[derive(Debug, Deserialize)]
struct CellFile {
    #[serde(rename(deserialize = "POSITIONS_FRAC"))]
    positions_frac: PositionsFrac,
}
#[test]
fn top_de() {
    let ion_line = CellValue::Array(vec![
        CellValue::Str("O"),
        CellValue::Float(0.5),
        CellValue::Float(0.5),
        CellValue::Float(0.5),
        CellValue::Str("SPIN="),
        CellValue::Float(1.0),
    ]);
    let position_frac_ir = Cell::Block("POSITIONS_FRAC", vec![ion_line]);
    let cell_file =
        CellFile::deserialize(&mut CellFileDeserializer::new(&[position_frac_ir]));
    dbg!(cell_file);
}
```

Test failed with:

```bash
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.15s
     Running unittests src/lib.rs (target/debug/deps/castep_cell_io-0265730ec12dc0a4)
[src/lib.rs:373:13] cell_file = Err(
    Message(
        "duplicate field `POSITIONS_FRAC`",
    ),
)
```

Why it will cause duplicate field?
