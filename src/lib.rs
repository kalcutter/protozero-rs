//! Protozero is a low-level [Protocol Buffers][Protocol Buffers] decoder.
//!
//! [Protocol Buffers]: https://developers.google.com/protocol-buffers/

#![no_std]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![forbid(unsafe_code)]

mod encoding;
/// Protobuf fields and values.
pub mod field;
/// Protobuf message API.
pub mod message;

/// The error type.
#[derive(Debug, Eq, PartialEq)]
pub struct Error;

pub use message::Message;
