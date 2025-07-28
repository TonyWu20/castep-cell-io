use serde::{
    Deserializer,
    de::{EnumAccess, SeqAccess, VariantAccess, value::StrDeserializer},
    forward_to_deserialize_any,
};

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

struct EnumAcc<'a, 'de: 'a> {
    de: &'a mut CellValueDeserializer<'a, 'de>,
}

impl<'a, 'de: 'a> EnumAcc<'a, 'de> {
    fn new(de: &'a mut CellValueDeserializer<'a, 'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> EnumAccess<'de> for EnumAcc<'a, 'de> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.de.value {
            CellValue::Str(s) => {
                let val = seed.deserialize(StrDeserializer::new(s))?;
                Ok((val, self))
            }
            CellValue::Array(array) => {
                let variant = array
                    .iter()
                    .next()
                    .map(|item| match item {
                        CellValue::Str(s) => {
                            let trimmed_key = s.trim_end_matches([':', ' ', '=']);
                            let variant = seed.deserialize(StrDeserializer::new(trimmed_key))?;
                            Ok(variant)
                        }
                        other => Err(Error::UnexpectedType(
                            "CellValue::Str".to_string(),
                            format!("{other:?}"),
                        )),
                    })
                    .ok_or(Error::Empty)??;
                // This is a newtype variant
                if array.len() == 2 {
                    self.de.value = &array[1];
                    Ok((variant, self))
                } else {
                    // Tuple variant
                    Ok((variant, self))
                }
            }
            other => Err(Error::UnexpectedType(
                "CellValue::Str".to_string(),
                format!("{other:?}"),
            )),
        }
    }
}

impl<'a, 'de> VariantAccess<'de> for EnumAcc<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, _len: usize, _visitorr: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
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
            CellValue::String(s) => visitor.visit_str(s),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
                format!("{other:?}"),
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
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(EnumAcc::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            CellValue::Str(s) => visitor.visit_str(s),
            other => Err(Error::UnexpectedType(
                "CellValue::Str".to_string(),
                format!("{other:?}"),
            )),
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
