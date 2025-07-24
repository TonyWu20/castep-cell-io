use serde::{
    Serialize, Serializer,
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
};

use crate::{CResult, Cell, CellValue, error::Error};

pub fn to_string<T: Serialize>(item: &T) -> CResult<String> {
    let mut serializer = CellSerializer::default();
    item.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> Serialize for Cell<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Cell::KeyValue(key, cell_value) => {
                let mut s = serializer.serialize_map(Some(1))?;
                s.serialize_key(key)?;
                s.serialize_value(cell_value)?;
                s.serialize_value("\n")?;
                s.end()
            }
            Cell::Block(block_name, cell_values) => {
                let mut s = serializer.serialize_seq(Some(cell_values.len()))?;
                s.serialize_element(&format!("%BLOCK {}\n", block_name.to_uppercase()))?;
                cell_values
                    .iter()
                    .try_for_each(|line| s.serialize_element(line))?;
                s.serialize_element(&format!("%ENDBLOCK {}\n", block_name.to_uppercase()))?;
                s.end()
            }
            Cell::Flag(key) => serializer.serialize_str(&format!("{key}\n")),
        }
    }
}

impl<'a> Serialize for CellValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CellValue::Null => serializer.serialize_none(),
            CellValue::Bool(b) => serializer.serialize_bool(*b),
            CellValue::Str(s) => {
                let str_len = match s.len() {
                    x if x < 6 => 6,
                    x if (4..20).contains(&x) => 20,
                    x => x + 2,
                };
                serializer.serialize_str(&format!("{s:>str_len$}"))
            }
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
struct CellSerializer {
    output: String,
}

impl Serializer for &mut CellSerializer {
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
        self.output += &format!("{v:6?}");
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:4}");
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:20.16}");
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:20.16}");
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += v;
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

impl SerializeSeq for &mut CellSerializer {
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

impl SerializeTuple for &mut CellSerializer {
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

impl SerializeTupleStruct for &mut CellSerializer {
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

impl SerializeTupleVariant for &mut CellSerializer {
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

impl SerializeMap for &mut CellSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        self.output += " : ";
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "\n";
        Ok(())
    }
}

impl SerializeStruct for &mut CellSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl SerializeStructVariant for &mut CellSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
#[cfg(test)]
mod ser_test {
    use std::fs::read_to_string;

    use crate::{Cell, CellValue, parse_cell_file, ser::to_string};

    #[test]
    fn ser_value() {
        let ion_line = CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Str("SPIN="),
            CellValue::Float(1.0),
        ]);
        let ion_lines = CellValue::Array(vec![
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Str("SPIN="),
                CellValue::Float(1.0),
            ]),
            ion_line.clone(),
        ]);
        let output = to_string(&ion_lines).unwrap();
        println!("{output}");
        let fix_com = Cell::KeyValue("FIX_COM", CellValue::Bool(true));
        println!("{}", to_string(&fix_com).unwrap());
        let block_positions =
            Cell::Block("POSITIONS_FRAC", vec![ion_line.clone(), ion_line.clone()]);
        println!("{}", to_string(&block_positions).unwrap());
    }
    #[test]
    fn ser_round() {
        let example = read_to_string("Mg2SiO4_Cr_1.cell").unwrap();
        let parsed = parse_cell_file(&example).unwrap();
        println!("{}", to_string(&parsed).unwrap());
    }
}
