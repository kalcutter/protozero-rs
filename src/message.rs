use crate::encoding::read_varint;
use crate::field::{Field, FieldValue, Fixed32, Fixed64, LengthDelimited, Varint};
use crate::Error;
use core::iter::FusedIterator;

/// A protobuf message.
#[derive(Clone, Copy, Debug)]
pub struct Message<'a> {
    buf: &'a [u8],
}

impl<'a> Message<'a> {
    /// Creates a new [`Message`] from a byte slice.
    #[inline]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    /// Returns an iterator over fields of the message.
    #[inline]
    pub fn fields(&self) -> Fields<'a> {
        Fields { buf: self.buf }
    }
}

impl<'a> From<&'a [u8]> for Message<'a> {
    #[inline]
    fn from(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}

/// An iterator over fields of a message.
///
/// This struct is returned from the [`fields`][Message::fields] method of [`Message`].
#[derive(Clone, Debug)]
pub struct Fields<'a> {
    buf: &'a [u8],
}

#[doc(hidden)]
pub type MessageFields<'a> = Fields<'a>;

impl<'a> Fields<'a> {
    fn try_next(&mut self) -> Result<Option<Field<'a>>, Error> {
        if self.buf.is_empty() {
            return Ok(None);
        }
        let (buf, tag) = read_varint(self.buf)?;
        let number = tag >> 3;
        if number == 0 {
            // Field number 0 is illegal.
            return Err(Error);
        }
        let wire_type = tag as u8 & 0x7;
        let (buf, value) = match wire_type {
            0 => {
                let (buf, value) = read_varint(buf)?;
                (buf, FieldValue::Varint(Varint { value }))
            }
            1 => {
                if buf.len() < 8 {
                    return Err(Error);
                }
                let bytes = buf[..8].try_into().unwrap();
                (&buf[8..], FieldValue::Fixed64(Fixed64 { bytes }))
            }
            2 => {
                let (buf, len) = read_varint(buf)?;
                let len = len.try_into().map_err(|_| Error)?;
                if buf.len() < len {
                    return Err(Error);
                }
                (
                    &buf[len..],
                    FieldValue::LengthDelimited(LengthDelimited { buf: &buf[..len] }),
                )
            }
            3 => (buf, FieldValue::StartGroup),
            4 => (buf, FieldValue::EndGroup),
            5 => {
                if buf.len() < 4 {
                    return Err(Error);
                }
                let bytes = buf[..4].try_into().unwrap();
                (&buf[4..], FieldValue::Fixed32(Fixed32 { bytes }))
            }
            _ => return Err(Error),
        };
        self.buf = buf;
        Ok(Some(Field { number, value }))
    }
}

impl<'a> Iterator for Fields<'a> {
    type Item = Result<Field<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

impl FusedIterator for Fields<'_> {}
