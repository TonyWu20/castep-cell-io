use std::str::FromStr;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};

use crate::keywords::{LatticeABC, LatticeCart, LatticeParam};

use super::error::DeError;

mod lattice_abc;
mod lattice_cart;

impl FromStr for LatticeParam {
    type Err = DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut deserializer = LatticeParamDeserializer::from_str(s);
        LatticeParam::deserialize(&mut deserializer)
    }
}

#[derive(Debug)]
pub struct LatticeParamDeserializer<'de> {
    input: &'de str,
}

impl<'de> LatticeParamDeserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Self { input }
    }
}

struct LatVisitor;

impl<'de> Visitor<'de> for LatVisitor {
    type Value = LatticeParam;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting a string representation of `LATTICE_CART` or `LATTICE_ABC`")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let array: Vec<f64> = v
            .split_whitespace()
            .into_iter()
            .filter_map(|value| value.parse::<f64>().ok())
            .collect();
        let array_size = array.len();
        match array_size {
            9 => Ok(LatticeParam::LatticeCart(LatticeCart::from_str(v).unwrap())),
            6 => Ok(LatticeParam::LatticeABC(LatticeABC::from_str(v).unwrap())),
            _ => Err(Error::custom("Incorrect input of lattice parameters")),
        }
    }
}

#[allow(unused_variables)]
impl<'de, 'a> Deserializer<'de> for &'a mut LatticeParamDeserializer<'de> {
    type Error = DeError;

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for LatticeParam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LatVisitor)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::keywords::LatticeParam;

    #[test]
    fn de_lattice_param() {
        let input = r"   18.931530020488704480   -0.000000000000003553    0.000000000000000000
   -9.465765010246645517   16.395185930251127360    0.000000000000000000
    0.000000000000000000    0.000000000000000000    9.999213039981000861
        ";
        let lattice_param = LatticeParam::from_str(input);
        assert!(lattice_param.is_ok());
        println!("{:#?}", lattice_param);
        let input = r"    8.9780000000    5.7400000000	9.9690000000
    90.000000000    90.000000000	90.000000000";
        let lattice_param = LatticeParam::from_str(input);
        assert!(lattice_param.is_ok());
        println!("{:#?}", lattice_param);
    }
}
