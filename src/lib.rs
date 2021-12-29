//! Protozero is a low-level [Protocol Buffers][Protocol Buffers] decoder.
//!
//! [Protocol Buffers]: https://developers.google.com/protocol-buffers/

#![no_std]

mod encoding;
/// Protobuf fields and values.
pub mod field;
/// Protobuf message API.
pub mod message;

/// The error type.
#[derive(Debug, PartialEq)]
pub struct Error;

pub use message::Message;
