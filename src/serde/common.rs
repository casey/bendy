/// Standard library
pub(crate) use std::{
    collections::BTreeMap,
    fmt::{self, Debug, Display, Formatter},
    iter::Peekable,
    num::ParseIntError,
    str::{self, Utf8Error},
};

/// Dependencies
pub(crate) use serde::{
    de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
    ser::{
        Serialize, SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Deserialize,
};

/// Structs and enums
pub(crate) use crate::{
    decoding::{self, Decoder, Tokens},
    encoding::{self, Encoder},
    serde::{ser::Serializer, Error, Result},
    state_tracker::{StructureError, Token},
};
