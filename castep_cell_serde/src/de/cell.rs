use std::collections::HashMap;

use serde::{
    Deserializer,
    de::{
        DeserializeOwned, EnumAccess, MapAccess, SeqAccess, VariantAccess, value::StrDeserializer,
    },
    forward_to_deserialize_any,
};

use crate::{Cell, CellValue, error::Error, parse_cell_file};

use super::primitive::CellValueDeserializer;

/// Deserialize struct from `&str` of a `.cell` file
pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let tokens = parse_cell_file(input).unwrap();
    T::deserialize(&mut CellFileDeserializer::new(&tokens))
}

/// Deserialize struct from tokens `&'a [Cell<'a>]`
pub fn from_tokens<'a, T>(input: &'a [Cell<'a>]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(&mut CellFileDeserializer::new(input))
}

struct CellStructDeserializer<'a, 'de> {
    contents: &'a [CellValue<'de>],
}

impl<'a, 'de: 'a> CellStructDeserializer<'a, 'de> {
    pub fn new(contents: &'a [CellValue<'de>]) -> Self {
        Self { contents }
    }
}

impl<'a, 'de: 'a> Deserializer<'de> for &'a mut CellStructDeserializer<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64
        f32 f64 char str string bytes byte_buf unit unit_struct tuple tuple_struct map struct enum identifier
        newtype_struct ignored_any option
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(BlockSeqAccess::new(self.contents))
    }
}

struct BlockSeqAccess<'a, 'de> {
    iter: std::slice::Iter<'a, CellValue<'de>>,
}

impl<'a, 'de> BlockSeqAccess<'a, 'de> {
    pub fn new(contents: &'a [CellValue<'de>]) -> Self {
        Self {
            iter: contents.iter(),
        }
    }
}

impl<'a, 'de> SeqAccess<'de> for BlockSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<S>(&mut self, seed: S) -> Result<Option<S::Value>, Self::Error>
    where
        S: serde::de::DeserializeSeed<'de>,
    {
        self.iter
            .next()
            .map(|cell_value| {
                let mut deserializer = CellValueDeserializer::new(cell_value);
                seed.deserialize(&mut deserializer)
            })
            .transpose()
    }
}

struct CellFileDeserializer<'de> {
    entries: HashMap<&'de str, &'de Cell<'de>>,
}

impl<'de> CellFileDeserializer<'de> {
    pub fn new(entries: &'de [Cell]) -> Self {
        Self {
            entries: entries.iter().map(|entry| (entry.key(), entry)).collect(),
        }
    }
}

struct CellFileMapAccess<'a, 'de> {
    inner: &'a mut CellFileDeserializer<'de>,
    field_entrys_iter: std::slice::Iter<'static, &'static str>,
    current_item: Option<&'static str>,
}

impl<'a, 'de> CellFileMapAccess<'a, 'de> {
    fn new(
        inner: &'a mut CellFileDeserializer<'de>,
        field_entrys: &'static [&'static str],
    ) -> Self {
        Self {
            inner,
            field_entrys_iter: field_entrys.iter(),
            current_item: None,
        }
    }
}

struct CellFileEnumAccess<'a, 'de> {
    enum_name: &'static str,
    de: &'a mut CellFileDeserializer<'de>,
}

impl<'a, 'de> CellFileEnumAccess<'a, 'de> {
    fn new(enum_name: &'static str, de: &'a mut CellFileDeserializer<'de>) -> Self {
        Self { enum_name, de }
    }
}

impl<'a, 'de> EnumAccess<'de> for CellFileEnumAccess<'a, 'de> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.de.entries.get(self.enum_name) {
            Some(item) => match item {
                Cell::KeyValue(_, cell_value) => {
                    let val = seed.deserialize(&mut CellValueDeserializer::new(cell_value))?;
                    Ok((val, self))
                }
                Cell::Block(_, _cell_values) => Err(Error::UnexpectedType(
                    "KeyValue".to_string(),
                    "Block".to_string(),
                )),
                Cell::Flag(_) => Err(Error::UnexpectedType("KeyValue".into(), "Flag".into())),
            },
            None => Err(Error::KeyNotFound(self.enum_name.to_string())),
        }
    }
}

impl<'a, 'de> VariantAccess<'de> for CellFileEnumAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        todo!()
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
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

impl<'a, 'de> MapAccess<'de> for CellFileMapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.field_entrys_iter.next().and_then(|key| {
            self.inner.entries.get(key).map(|_| {
                self.current_item = Some(key);
                seed.deserialize(StrDeserializer::new(key)).map(Some)
            })
        }) {
            Some(k) => k,
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.current_item
            .take()
            .and_then(|key| {
                self.inner.entries.get(key).map(|item| match item {
                    Cell::KeyValue(_, cell_value) => {
                        seed.deserialize(&mut CellValueDeserializer::new(cell_value))
                    }
                    Cell::Block(_, cell_values) => {
                        seed.deserialize(&mut CellStructDeserializer::new(cell_values))
                    }
                    Cell::Flag(_) => {
                        seed.deserialize(&mut CellValueDeserializer::new(&CellValue::Bool(true)))
                    }
                })
            })
            .ok_or(Error::Message("Item not found".to_string()))?
    }
}

impl<'de, 'a: 'de> Deserializer<'de> for &'a mut CellFileDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /// For the top level `CellFile`
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(CellFileMapAccess::new(&mut *self, fields))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(CellFileEnumAccess::new(name, self))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64
        f32 f64 char str string bytes byte_buf unit_struct unit identifier
        newtype_struct ignored_any option map seq tuple tuple_struct
    }
}
