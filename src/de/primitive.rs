use serde::{Deserializer, de::SeqAccess, forward_to_deserialize_any};

use crate::{CellValue, error::Error};

pub struct CellValueDeserializer<'a, 'de> {
    value: &'a CellValue<'de>,
}

impl<'a, 'de> CellValueDeserializer<'a, 'de> {
    pub fn new(value: &'a CellValue<'de>) -> Self {
        Self { value }
    }
}

struct ArrayIterSeqAccess<'a, 'de> {
    iter: std::slice::Iter<'a, CellValue<'de>>,
}

impl<'a, 'de> ArrayIterSeqAccess<'a, 'de> {
    fn new(values: &'a Vec<CellValue<'de>>) -> Self {
        Self {
            iter: values.iter(),
        }
    }
}

impl<'a, 'de> SeqAccess<'de> for ArrayIterSeqAccess<'a, 'de> {
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

struct ArraySeqAccess<'a, 'de> {
    inner: &'a mut CellValueDeserializer<'a, 'de>,
    curr_idx: usize,
}

impl<'a, 'de> ArraySeqAccess<'a, 'de> {
    fn new(inner: &'a mut CellValueDeserializer<'a, 'de>) -> Self {
        Self { inner, curr_idx: 0 }
    }
}

impl<'a, 'de> SeqAccess<'de> for ArraySeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.inner.value {
            CellValue::Array(cell_values) => match cell_values.get(self.curr_idx) {
                Some(item) => {
                    self.curr_idx += 1;
                    seed.deserialize(&mut CellValueDeserializer::new(item))
                        .map(Some)
                }
                None => Ok(None),
            },
            other => Err(Error::UnexpectedType(
                "array".to_string(),
                format!("{:?}", other),
            )),
        }
    }
}

impl<'a, 'de> Deserializer<'de> for &'a mut CellValueDeserializer<'a, 'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Null => visitor.visit_unit(),
            CellValue::Bool(b) => visitor.visit_bool(*b),
            CellValue::Str(s) => visitor.visit_borrowed_str(s),
            CellValue::UInt(u) => visitor.visit_u32(*u),
            CellValue::Int(i) => visitor.visit_i32(*i),
            CellValue::Float(f) => visitor.visit_f64(*f),
            CellValue::Array(cell_value) => visitor.visit_seq(ArrayIterSeqAccess::new(cell_value)),
        }
    }

    forward_to_deserialize_any! {bool f64 str string unit}

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Int(i) => visitor.visit_i8(*i as i8),
            other => Err(Error::UnexpectedType(
                "i8".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Int(i) => visitor.visit_i16(*i as i16),
            other => Err(Error::UnexpectedType(
                "i16".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Int(i) => visitor.visit_i32(*i),
            other => Err(Error::UnexpectedType(
                "i32".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("Only up to i32")
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::UInt(u) => visitor.visit_u8(*u as u8),
            other => Err(Error::UnexpectedType(
                "u8".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::UInt(u) => visitor.visit_u16(*u as u16),
            other => Err(Error::UnexpectedType(
                "u16".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::UInt(u) => visitor.visit_u32(*u),
            other => Err(Error::UnexpectedType(
                "u32".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("Only up to u32")
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Float(f) => visitor.visit_f32(*f as f32),
            other => Err(Error::UnexpectedType(
                "i8".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("char unimplemented!")
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("No bytes")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("No bytes")
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("name");
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Array(arr) => visitor.visit_seq(ArrayIterSeqAccess::new(arr)),
            other => Err(Error::UnexpectedType(
                "array".to_string(),
                format!("{:?}", other),
            )),
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
            other => Err(Error::UnexpectedType(
                "array for tuple".to_string(),
                format!("{:?}", other),
            )),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
