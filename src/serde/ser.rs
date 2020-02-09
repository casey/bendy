//! Serde bencode serialization.

use crate::serde::common::*;

mod struct_serializer;

pub use struct_serializer::StructSerializer;

/// Serialize an instance of `T` to bencode
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    serializer.into_bytes()
}

/// A serde Bencode serializer
pub struct Serializer {
    encoder: Encoder,
}

impl Serializer {
    /// Create a new `Serializer`
    pub fn new() -> Self {
        Serializer {
            encoder: Encoder::new(),
        }
    }

    /// Create a new `Serializer` with a given maximum serialization depth
    pub fn with_max_depth(max_depth: usize) -> Serializer {
        Serializer {
            encoder: Encoder::new().with_max_depth(max_depth),
        }
    }

    /// Consume this `Serializer`, returning the encoded bencode
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.encoder.get_output()?)
    }

    fn struct_serializer(&mut self) -> StructSerializer {
        let remaining_depth = self.encoder.remaining_depth();
        StructSerializer::new(self, remaining_depth)
    }

    pub(crate) fn emit_struct(&mut self, contents: BTreeMap<&'static str, Vec<u8>>) -> Result<()> {
        self.encoder.emit_token(Token::Dict)?;
        for (key, value) in contents {
            self.encoder.emit_bytes(key.as_bytes())?;
            for result in Decoder::new(&value).tokens() {
                let token = result?;
                self.encoder.emit_token(token)?;
            }
        }
        self.encoder.emit_token(Token::End)?;
        Ok(())
    }
}

impl<'a> serde::ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = StructSerializer<'a>;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        panic!("bendy::Serializer::serialize_bool: not supported");
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.encoder.emit(v)?;
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        panic!("bendy::Serializer::serialize_f32: not supported");
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        panic!("bendy::Serializer::serialize_f64: not supported");
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        panic!("bendy::Serializer::serialize_char: not supported");
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.encoder.emit_bytes(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        panic!("bendy::Serializer::serialize_none: not supported");
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!("bendy::Serializer::serialize_some: not supported");
    }

    fn serialize_unit(self) -> Result<()> {
        panic!("bendy::Serializer::serialize_unit: not supported");
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        panic!("bendy::Serializer::serialize_unit_struct: not supported");
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        panic!("bendy::Serializer::serialize_unit_variant: not supported");
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!("bendy::Serializer::serialize_newtype_variant: not supported");
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.encoder.emit_token(Token::List)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.encoder.emit_token(Token::List)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.encoder.emit_token(Token::List)?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        panic!("bendy::Serializer::serialize_tuple_variant: not supported");
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        panic!("bendy::Serializer::serialize_map: not supported");
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self.struct_serializer())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        panic!("bendy::Serializer::serialize_struct_variant: not supported");
    }
}

impl<'a> SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.encoder.emit_token(Token::End)?;
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.encoder.emit_token(Token::End)?;
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.encoder.emit_token(Token::End)?;
        Ok(())
    }
}

impl<'a> SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl<'a> SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl<'a> SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}
