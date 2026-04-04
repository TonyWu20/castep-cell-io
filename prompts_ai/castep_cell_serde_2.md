This is my current implementations:

```rust
use serde::{Deserializer, de::SeqAccess, forward_to_deserialize_any};

use crate::{CellValue, error::Error};
pub struct CellValueDeserializer<'de> {
    value: &'de CellValue<'de>,
}

impl<'de> CellValueDeserializer<'de> {
    pub fn new(value: &'de CellValue<'de>) -> Self {
        Self { value }
    }
}


struct ArrayIterSeqAccess<'de> {
    iter: std::slice::Iter<'de, CellValue<'de>>,
}

impl<'de> ArrayIterSeqAccess<'de> {
    fn new(values: &'de Vec<CellValue<'de>>) -> Self {
        Self {
            iter: values.iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for ArrayIterSeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.iter
            .next()
            .map(|value| seed.deserialize(&mut CellValueDeserializer::new(value)))
            .transpose()
    }
}

impl<'a, 'de> Deserializer<'de> for &'a mut CellValueDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Null => visitor.visit_unit(),
            CellValue::Bool(b) => visitor.visit_bool(*b),
            CellValue::Str(s) => visitor.visit_borrowed_str(s),
            CellValue::Int(i) => visitor.visit_i128(*i as i128),
            CellValue::Float(f) => visitor.visit_f64(*f),
            CellValue::Array(cell_values) => {
                visitor.visit_seq(ArrayIterSeqAccess::new(cell_values))
            }
        }
    }

    forward_to_deserialize_any! {bool i8 i16 i32 f32 f64 str string unit}
    // ... leaving others unimplemented
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Array(arr) => visitor.visit_seq(ArrayIterSeqAccess::new(arr)),
            _ => Err(Error::UnexpectedType("array".to_string())),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Array(arr) => {
                if len == arr.len() {
                    visitor.visit_seq(ArrayIterSeqAccess::new(arr))
                } else {
                    Err(Error::Message(format!(
                        "len {} unmatched: got {}",
                        len,
                        arr.len()
                    )))
                }
            }
            _ => Err(Error::UnexpectedType("array".to_string())),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("struct");
        self.deserialize_seq(visitor)
        // match self.value {
        //     CellValue::Array(arr) => {
        //         if fields.len() == arr.len() {
        //             visitor.visit_seq(ArrayIterSeqAccess::new(arr))
        //         } else {
        //             Err(Error::Message(format!(
        //                 "len {} unmatched: got {}",
        //                 fields.len(),
        //                 arr.len()
        //             )))
        //         }
        //     }
        //     _ => Err(Error::UnexpectedType("array".to_string())),
        // }
    }
}
```

It currently pass these test:

```rust
#[derive(Debug, Deserialize, PartialEq)]
pub struct Kpoint {
    kx: f64,
    ky: f64,
    kz: f64,
    weight: f64,
}

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

#[test]
fn line_de() {
    let kpoint_line = CellValue::Array(vec![
        CellValue::Float(0.0),
        CellValue::Float(0.0),
        CellValue::Float(0.0),
        CellValue::Float(1.0),
    ]);
    let kpoint =
        Kpoint::deserialize(&mut CellValueDeserializer::new(&kpoint_line)).unwrap();
    dbg!(&kpoint);
    debug_assert_eq!(
        kpoint,
        Kpoint {
            kx: 0.0,
            ky: 0.0,
            kz: 0.0,
            weight: 1.0
        }
    );
    let ion_line = CellValue::Array(vec![
        CellValue::Str("O"),
        CellValue::Float(0.5),
        CellValue::Float(0.5),
        CellValue::Float(0.5),
        CellValue::Str("SPIN="),
        CellValue::Float(1.0),
    ]);
    let ion =
        IonicPosition::deserialize(&mut CellValueDeserializer::new(&ion_line)).unwrap();
}
```

## But I am not sure if my implementations are reasonable.

Now I test with:

```rust
#[derive(Debug, Deserialize, PartialEq)]
#[serde(from = "IonicPositionRepr")]
pub struct IonicPosition {
    element: String,
    coord: [f64; 3],
    spin: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct BlockLines<T>(pub Vec<T>);

#[derive(Debug, Deserialize)]
pub struct PositionsFrac {
    #[serde(rename(deserialize = "POSITIONS_FRAC"))]
    ionic_positions: BlockLines<IonicPosition>,
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
#[test]
fn test() {
    let ion_line = CellValue::Array(vec![
        CellValue::Str("O"),
        CellValue::Float(0.5),
        CellValue::Float(0.5),
        CellValue::Float(0.5),
        CellValue::Str("SPIN="),
        CellValue::Float(1.0),
    ]);
    let positions = vec![
        ion_line.clone(),
        CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.25),
            CellValue::Str("SPIN="),
            CellValue::Float(1.0),
        ]),
    ];
    let ion =
        IonicPosition::deserialize(&mut CellValueDeserializer::new(&ion_line)).unwrap();
    dbg!(ion);
    let positions = PositionsFrac::deserialize(
        &mut BlockDeserializer::<PositionsFrac>::new(&positions),
    )
    .unwrap();
}
```

A single `IonicPosition` can be successfully deserialized, but the `PositionsFrac` failed
with

```
called `Result::unwrap()` on an `Err` value: Message("data did not match any variant of untagged enum IonicPo
sitionRepr")
```
