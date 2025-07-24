#![allow(dead_code)]

pub use parser::parse_cell_file;
pub use parser::rich_error;
use serde::ser::SerializeSeq;

// Intermediate representation for parsed data
#[derive(Debug, Clone)]
pub enum Cell<'a> {
    KeyValue(&'a str, CellValue<'a>),
    Block(&'a str, Vec<CellValue<'a>>),
    Flag(&'a str),
}

impl<'a> Cell<'a> {
    pub fn key(&self) -> &str {
        match self {
            Cell::KeyValue(key, _cell_value) => key,
            Cell::Block(name, _cell_value) => name,
            Cell::Flag(flag) => flag,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CellValue<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    UInt(u32),
    Int(i32),
    Float(f64),
    Array(Vec<CellValue<'a>>),
}

mod de;
mod error;
mod parser;
mod ser {
    use serde::{
        Serialize, Serializer,
        ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant},
    };

    use crate::{CellValue, error::Error};

    impl<'a> Serialize for CellValue<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                CellValue::Null => serializer.serialize_none(),
                CellValue::Bool(b) => serializer.serialize_bool(*b),
                CellValue::Str(s) => serializer.serialize_str(s),
                CellValue::UInt(u) => serializer.serialize_u32(*u),
                CellValue::Int(i) => serializer.serialize_i32(*i),
                CellValue::Float(f) => serializer.serialize_f64(*f),
                CellValue::Array(cell_values) => {
                    let mut s = serializer.serialize_seq(Some(cell_values.len()))?;
                    cell_values
                        .iter()
                        .try_for_each(|item| s.serialize_element(item))?;
                    s.end()
                }
            }
        }
    }
    #[derive(Debug, Default)]
    struct CellValueSerializer {
        output: String,
    }

    impl<'a> Serializer for &'a mut CellValueSerializer {
        type Ok = ();

        type Error = Error;

        type SerializeSeq = Self;

        type SerializeTuple = Self;

        type SerializeTupleStruct = Self;

        type SerializeTupleVariant = Self;

        type SerializeMap = Self;

        type SerializeStruct = Self;

        type SerializeStructVariant = Self;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:6?}", v);
            Ok(())
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:4}", v);
            Ok(())
        }

        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:20.16}", v);
            Ok(())
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            self.output += &format!("{:20.16}", v);
            Ok(())
        }

        fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            self.output += &format!(" {}", v);
            Ok(())
        }

        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut *self)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
            self.output += name;
            self.output += "\n";
            Ok(())
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.output += &variant.to_lowercase();
            Ok(())
        }

        fn serialize_newtype_struct<T>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.output += name;
            self.output += " : ";
            value.serialize(&mut *self)?;
            Ok(())
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            todo!()
        }

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Ok(self)
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            todo!()
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(self)
        }

        fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            todo!()
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeSeq for &'a mut CellValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.output += "\n";
            Ok(())
        }
    }

    impl<'a> SerializeTuple for &'a mut CellValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.output += "\n";
            Ok(())
        }
    }

    impl<'a> SerializeTupleStruct for &'a mut CellValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.output += "\n";
            Ok(())
        }
    }

    impl<'a> SerializeTupleVariant for &'a mut CellValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }
}
