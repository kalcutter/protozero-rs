use crate::encoding::read_varint;
use crate::encoding::zigzag;
use crate::message::Message;
use crate::Error;
use core::str;

/// Instance of a message field.
///
/// A protobuf message is a series of key-value pairs. `Field` represents a single key-value pair
/// of a message. See [Message Structure].
///
/// [Message Structure]: https://developers.google.com/protocol-buffers/docs/encoding#structure
#[derive(Debug)]
pub struct Field<'a> {
    /// The field number.
    pub number: u64,
    /// The field value.
    pub value: FieldValue<'a>,
}

/// Value of a field.
///
/// Each enum variant corresponds to a [wire type][Message Structure].
///
/// [Message Structure]: https://developers.google.com/protocol-buffers/docs/encoding#structure
#[derive(Debug)]
pub enum FieldValue<'a> {
    /// A varint value.
    Varint(Varint),
    /// A non-varint 64-bit number.
    Fixed64(Fixed64),
    /// A length-delimited value.
    LengthDelimited(LengthDelimited<'a>),
    /// Start of a group.
    StartGroup,
    /// End of a group.
    EndGroup,
    /// A non-varint 32-bit number.
    Fixed32(Fixed32),
}

impl<'a> FieldValue<'a> {
    /// Returns the value of a `bool` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_bool(&self) -> Result<bool, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_bool()),
            _ => Err(Error),
        }
    }

    /// Returns the value of an `enum` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_enum(&self) -> Result<i32, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_enum()),
            _ => Err(Error),
        }
    }

    /// Returns the value of an `int32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_int32(&self) -> Result<i32, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_int32()),
            _ => Err(Error),
        }
    }

    /// Returns the value of an `int64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_int64(&self) -> Result<i64, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_int64()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `sint32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_sint32(&self) -> Result<i32, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_sint32()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `sint64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_sint64(&self) -> Result<i64, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_sint64()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `uint32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_uint32(&self) -> Result<u32, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_uint32()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `uint64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_uint64(&self) -> Result<u64, Error> {
        match self {
            FieldValue::Varint(f) => Ok(f.get_uint64()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `fixed64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_fixed64(&self) -> Result<u64, Error> {
        match self {
            FieldValue::Fixed64(f) => Ok(f.get_fixed64()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `sfixed64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_sfixed64(&self) -> Result<i64, Error> {
        match self {
            FieldValue::Fixed64(f) => Ok(f.get_sfixed64()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `double` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_double(&self) -> Result<f64, Error> {
        match self {
            FieldValue::Fixed64(f) => Ok(f.get_double()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `bytes` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_bytes(&self) -> Result<&'a [u8], Error> {
        match self {
            FieldValue::LengthDelimited(f) => Ok(f.get_bytes()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `message` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_message(&self) -> Result<Message<'a>, Error> {
        match self {
            FieldValue::LengthDelimited(f) => Ok(f.get_message()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `string` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_string(&self) -> Result<&'a str, Error> {
        match self {
            FieldValue::LengthDelimited(f) => f.get_string(),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `fixed32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_fixed32(&self) -> Result<u32, Error> {
        match self {
            FieldValue::Fixed32(f) => Ok(f.get_fixed32()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `sfixed32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_sfixed32(&self) -> Result<i32, Error> {
        match self {
            FieldValue::Fixed32(f) => Ok(f.get_sfixed32()),
            _ => Err(Error),
        }
    }

    /// Returns the value of a `float` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_float(&self) -> Result<f32, Error> {
        match self {
            FieldValue::Fixed32(f) => Ok(f.get_float()),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `bool` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_bool(&self) -> Result<Repeated<bool, PackedBool<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_bool()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_bool())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `enum` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_enum(&self) -> Result<Repeated<i32, PackedEnum<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_enum()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_enum())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `int32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_int32(&self) -> Result<Repeated<i32, PackedInt32<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_int32()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_int32())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `int64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_int64(&self) -> Result<Repeated<i64, PackedInt64<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_int64()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_int64())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `sint32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_sint32(&self) -> Result<Repeated<i32, PackedSint32<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_sint32()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_sint32())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `sint64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_sint64(&self) -> Result<Repeated<i64, PackedSint64<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_sint64()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_sint64())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `uint32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_uint32(&self) -> Result<Repeated<u32, PackedUint32<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_uint32()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_uint32())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `uint64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_uint64(&self) -> Result<Repeated<u64, PackedUint64<'_>>, Error> {
        match self {
            FieldValue::Varint(f) => Ok(Repeated::Value(Some(f.get_uint64()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_uint64())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `fixed64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_fixed64(&self) -> Result<Repeated<u64, PackedFixed64<'_>>, Error> {
        match self {
            FieldValue::Fixed64(f) => Ok(Repeated::Value(Some(f.get_fixed64()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_fixed64())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `sfixed64` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_sfixed64(&self) -> Result<Repeated<i64, PackedSfixed64<'_>>, Error> {
        match self {
            FieldValue::Fixed64(f) => Ok(Repeated::Value(Some(f.get_sfixed64()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_sfixed64())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `double` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_double(&self) -> Result<Repeated<f64, PackedDouble<'_>>, Error> {
        match self {
            FieldValue::Fixed64(f) => Ok(Repeated::Value(Some(f.get_double()))),
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_double())),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `fixed32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_fixed32(&self) -> Result<Repeated<u32, PackedFixed32<'_>>, Error> {
        match self {
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_fixed32())),
            FieldValue::Fixed32(f) => Ok(Repeated::Value(Some(f.get_fixed32()))),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `sfixed32` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_sfixed32(&self) -> Result<Repeated<i32, PackedSfixed32<'_>>, Error> {
        match self {
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_sfixed32())),
            FieldValue::Fixed32(f) => Ok(Repeated::Value(Some(f.get_sfixed32()))),
            _ => Err(Error),
        }
    }

    /// Returns an iterator over values of a repeated `float` field.
    ///
    /// If the wire type is not compatible, `Err` is returned.
    #[inline]
    pub fn get_repeated_float(&self) -> Result<Repeated<f32, PackedFloat<'_>>, Error> {
        match self {
            FieldValue::LengthDelimited(f) => Ok(Repeated::Packed(f.get_packed_float())),
            FieldValue::Fixed32(f) => Ok(Repeated::Value(Some(f.get_float()))),
            _ => Err(Error),
        }
    }
}

/// A non-varint 32-bit number.
///
/// `Fixed32` is a numeric value stored in 32 bits. See [Non-varint Numbers].
///
/// [Non-varint Numbers]: https://developers.google.com/protocol-buffers/docs/encoding#non-varint_numbers
#[derive(Clone, Copy, Debug)]
pub struct Fixed32 {
    pub(crate) bytes: [u8; 4],
}

impl Fixed32 {
    /// Returns the value of a `fixed32` field.
    #[inline]
    pub fn get_fixed32(&self) -> u32 {
        u32::from_le_bytes(self.bytes)
    }

    /// Returns the value of a `sfixed32` field.
    #[inline]
    pub fn get_sfixed32(&self) -> i32 {
        i32::from_le_bytes(self.bytes)
    }

    /// Returns the value of a `float` field.
    #[inline]
    pub fn get_float(&self) -> f32 {
        f32::from_le_bytes(self.bytes)
    }
}

/// A non-varint 64-bit number.
///
/// `Fixed64` is a numeric value stored in 64 bits. See [Non-varint Numbers].
///
/// [Non-varint Numbers]: https://developers.google.com/protocol-buffers/docs/encoding#non-varint_numbers
#[derive(Clone, Copy, Debug)]
pub struct Fixed64 {
    pub(crate) bytes: [u8; 8],
}

impl Fixed64 {
    /// Returns the value of a `fixed64` field.
    #[inline]
    pub fn get_fixed64(&self) -> u64 {
        u64::from_le_bytes(self.bytes)
    }

    /// Returns the value of a `sfixed64` field.
    #[inline]
    pub fn get_sfixed64(&self) -> i64 {
        i64::from_le_bytes(self.bytes)
    }

    /// Returns the value of a `double` field.
    #[inline]
    pub fn get_double(&self) -> f64 {
        f64::from_le_bytes(self.bytes)
    }
}

/// A length-delimited value.
#[derive(Clone, Copy, Debug)]
pub struct LengthDelimited<'a> {
    pub(crate) buf: &'a [u8],
}

impl<'a> LengthDelimited<'a> {
    /// Returns the value of a `bytes` field.
    #[inline]
    pub fn get_bytes(&self) -> &'a [u8] {
        self.buf
    }

    /// Returns the value of a `message` field.
    #[inline]
    pub fn get_message(&self) -> Message<'a> {
        Message::new(self.buf)
    }

    /// Returns the value of a `string` field.
    ///
    /// If the string is not valid UTF-8, `Err` is returned.
    #[inline]
    pub fn get_string(&self) -> Result<&'a str, Error> {
        str::from_utf8(self.buf).map_err(|_| Error)
    }

    /// Returns an iterator over packed repeated `bool`s.
    #[inline]
    pub fn get_packed_bool(&self) -> PackedBool<'a> {
        PackedBool { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `enum`s.
    #[inline]
    pub fn get_packed_enum(&self) -> PackedEnum<'a> {
        PackedEnum { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `int32`s.
    #[inline]
    pub fn get_packed_int32(&self) -> PackedInt32<'a> {
        PackedInt32 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `int64`s.
    #[inline]
    pub fn get_packed_int64(&self) -> PackedInt64<'a> {
        PackedInt64 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `sint32`s.
    #[inline]
    pub fn get_packed_sint32(&self) -> PackedSint32<'a> {
        PackedSint32 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `sint64`s.
    #[inline]
    pub fn get_packed_sint64(&self) -> PackedSint64<'a> {
        PackedSint64 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `uint32`s.
    #[inline]
    pub fn get_packed_uint32(&self) -> PackedUint32<'a> {
        PackedUint32 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `uint64`s.
    #[inline]
    pub fn get_packed_uint64(&self) -> PackedUint64<'a> {
        PackedUint64 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `fixed64`s.
    #[inline]
    pub fn get_packed_fixed64(&self) -> PackedFixed64<'a> {
        PackedFixed64 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `sfixed64`s.
    #[inline]
    pub fn get_packed_sfixed64(&self) -> PackedSfixed64<'a> {
        PackedSfixed64 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `double`s.
    #[inline]
    pub fn get_packed_double(&self) -> PackedDouble<'a> {
        PackedDouble { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `fixed32`s.
    #[inline]
    pub fn get_packed_fixed32(&self) -> PackedFixed32<'a> {
        PackedFixed32 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `sfixed32`s.
    #[inline]
    pub fn get_packed_sfixed32(&self) -> PackedSfixed32<'a> {
        PackedSfixed32 { buf: self.buf }
    }

    /// Returns an iterator over packed repeated `float`s.
    #[inline]
    pub fn get_packed_float(&self) -> PackedFloat<'a> {
        PackedFloat { buf: self.buf }
    }
}

/// A varint value.
///
/// Varints are values encoded as [LEB128] variable length integers. See [Encoding].
///
/// [Encoding]: https://developers.google.com/protocol-buffers/docs/encoding#varints
/// [LEB128]: https://en.wikipedia.org/wiki/LEB128
#[derive(Clone, Copy, Debug)]
pub struct Varint {
    pub(crate) value: u64,
}

impl Varint {
    /// Returns the value of a `bool` field.
    #[inline]
    pub fn get_bool(&self) -> bool {
        self.value != 0
    }

    /// Returns the value of an `enum` field.
    #[inline]
    pub fn get_enum(&self) -> i32 {
        self.value as i32
    }

    /// Returns the value of an `int32` field.
    #[inline]
    pub fn get_int32(&self) -> i32 {
        self.value as i32
    }

    /// Returns the value of an `int64` field.
    #[inline]
    pub fn get_int64(&self) -> i64 {
        self.value as i64
    }

    /// Returns the value of a `sint32` field.
    #[inline]
    pub fn get_sint32(&self) -> i32 {
        zigzag::decode_32(self.value as u32)
    }

    /// Returns the value of a `sint64` field.
    #[inline]
    pub fn get_sint64(&self) -> i64 {
        zigzag::decode_64(self.value)
    }

    /// Returns the value of a `uint32` field.
    #[inline]
    pub fn get_uint32(&self) -> u32 {
        self.value as u32
    }

    /// Returns the value of a `uint64` field.
    #[inline]
    pub fn get_uint64(&self) -> u64 {
        self.value
    }
}

macro_rules! impl_packed {
    ($(#[$meta:meta])* $name:ident, Varint, $get_fn:ident, $return_type:ty) => {
        $(#[$meta])*
        pub struct $name<'a> {
            buf: &'a [u8],
        }

        impl<'a> Iterator for $name<'a> {
            type Item = Result<$return_type, Error>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.buf.is_empty() {
                    return None;
                }
                match read_varint(self.buf) {
                    Ok((buf, value)) => {
                        self.buf = buf;
                        Some(Ok(Varint { value }.$get_fn()))
                    }
                    Err(e) => Some(Err(e)),
                }
            }
        }
    };
    ($(#[$meta:meta])* $name:ident, Fixed64, $get_fn:ident, $return_type:ty) => {
        $(#[$meta])*
        pub struct $name<'a> {
            buf: &'a [u8],
        }

        impl<'a> Iterator for $name<'a> {
            type Item = Result<$return_type, Error>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.buf.is_empty() {
                    return None;
                }
                if self.buf.len() < 8 {
                    return Some(Err(Error));
                }
                let bytes = self.buf[..8].try_into().unwrap();
                self.buf = &self.buf[8..];
                Some(Ok(Fixed64 { bytes }.$get_fn()))
            }
        }
    };
    ($(#[$meta:meta])* $name:ident, Fixed32, $get_fn:ident, $return_type:ty) => {
        $(#[$meta])*
        pub struct $name<'a> {
            buf: &'a [u8],
        }

        impl<'a> Iterator for $name<'a> {
            type Item = Result<$return_type, Error>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.buf.is_empty() {
                    return None;
                }
                if self.buf.len() < 4 {
                    return Some(Err(Error));
                }
                let bytes = self.buf[..4].try_into().unwrap();
                self.buf = &self.buf[4..];
                Some(Ok(Fixed32 { bytes }.$get_fn()))
            }
        }
    };
}
impl_packed!(
    /// An iterator over values of a packed `bool` field.
    ///
    /// This struct is returned from the [`get_packed_bool`][LengthDelimited::get_packed_bool]
    /// method of [`LengthDelimited`].
    PackedBool,
    Varint,
    get_bool,
    bool
);
impl_packed!(
    /// An iterator over values of a packed `enum` field.
    ///
    /// This struct is returned from the [`get_packed_enum`][LengthDelimited::get_packed_enum]
    /// method of [`LengthDelimited`].
    PackedEnum,
    Varint,
    get_enum,
    i32
);
impl_packed!(
    /// An iterator over values of a packed `int32` field.
    ///
    /// This struct is returned from the [`get_packed_int32`][LengthDelimited::get_packed_int32]
    /// method of [`LengthDelimited`].
    PackedInt32,
    Varint,
    get_int32,
    i32
);
impl_packed!(
    /// An iterator over values of a packed `int64` field.
    ///
    /// This struct is returned from the [`get_packed_int64`][LengthDelimited::get_packed_int64]
    /// method of [`LengthDelimited`].
    PackedInt64,
    Varint,
    get_int64,
    i64
);
impl_packed!(
    /// An iterator over values of a packed `sint32` field.
    ///
    /// This struct is returned from the [`get_packed_sint32`][LengthDelimited::get_packed_sint32]
    /// method of [`LengthDelimited`].
    PackedSint32,
    Varint,
    get_sint32,
    i32
);
impl_packed!(
    /// An iterator over values of a packed `sint64` field.
    ///
    /// This struct is returned from the [`get_packed_sint64`][LengthDelimited::get_packed_sint64]
    /// method of [`LengthDelimited`].
    PackedSint64,
    Varint,
    get_sint64,
    i64
);
impl_packed!(
    /// An iterator over values of a packed `uint32` field.
    ///
    /// This struct is returned from the [`get_packed_uint32`][LengthDelimited::get_packed_uint32]
    /// method of [`LengthDelimited`].
    PackedUint32,
    Varint,
    get_uint32,
    u32
);
impl_packed!(
    /// An iterator over values of a packed `uint64` field.
    ///
    /// This struct is returned from the [`get_packed_uint64`][LengthDelimited::get_packed_uint64]
    /// method of [`LengthDelimited`].
    PackedUint64,
    Varint,
    get_uint64,
    u64
);

impl_packed!(
    /// An iterator over values of a packed `fixed64` field.
    ///
    /// This struct is returned from the [`get_packed_fixed64`][LengthDelimited::get_packed_fixed64]
    /// method of [`LengthDelimited`].
    PackedFixed64,
    Fixed64,
    get_fixed64,
    u64
);
impl_packed!(
    /// An iterator over values of a packed `sfixed64` field.
    ///
    /// This struct is returned from the [`get_packed_sfixed64`][LengthDelimited::get_packed_sfixed64]
    /// method of [`LengthDelimited`].
    PackedSfixed64,
    Fixed64,
    get_sfixed64,
    i64
);
impl_packed!(
    /// An iterator over values of a packed `double` field.
    ///
    /// This struct is returned from the [`get_packed_double`][LengthDelimited::get_packed_double]
    /// method of [`LengthDelimited`].
    PackedDouble,
    Fixed64,
    get_double,
    f64
);

impl_packed!(
    /// An iterator over values of a packed `fixed32` field.
    ///
    /// This struct is returned from the [`get_packed_fixed32`][LengthDelimited::get_packed_fixed32]
    /// method of [`LengthDelimited`].
    PackedFixed32,
    Fixed32,
    get_fixed32,
    u32
);
impl_packed!(
    /// An iterator over values of a packed `sfixed32` field.
    ///
    /// This struct is returned from the [`get_packed_sfixed32`][LengthDelimited::get_packed_sfixed32]
    /// method of [`LengthDelimited`].
    PackedSfixed32,
    Fixed32,
    get_sfixed32,
    i32
);
impl_packed!(
    /// An iterator over values of a packed `float` field.
    ///
    /// This struct is returned from the [`get_packed_float`][LengthDelimited::get_packed_float]
    /// method of [`LengthDelimited`].
    PackedFloat,
    Fixed32,
    get_float,
    f32
);

/// An iterator over repeated values of a field.
#[derive(Debug)]
pub enum Repeated<T, P> {
    Value(Option<T>),
    Packed(P),
}

impl<T, P> Iterator for Repeated<T, P>
where
    T: Copy,
    P: Iterator<Item = Result<T, Error>>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Repeated::Value(v) => v.take().map(Ok),
            Repeated::Packed(p) => p.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldValue, PackedInt32, Repeated, Varint};
    use crate::message::Message;

    #[test]
    fn repeated_value() {
        let mut iter = Repeated::<i32, PackedInt32>::Value(Some(137_i32));
        assert_eq!(iter.next(), Some(Ok(137_i32)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn varint() {
        let message = Message::new(&b"\x08\x01"[..]);
        let field = message.fields().next().unwrap().unwrap();
        assert_eq!(field.number, 1);
        assert!(matches!(
            field.value,
            FieldValue::Varint(Varint { value: 1 })
        ));
    }
}
