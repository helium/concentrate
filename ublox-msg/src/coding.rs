//! Types and traits for encoding/decoding u-blox messages into/fromn buffers.

/// Represents the encoded size of a type.
pub enum EncSize {
    /// Type has constant, fixed, encoded size.
    Fixed(usize),
    /// Type has one of two encoded sizes.
    OneOf(usize, usize),
    /// Type has a range of encoded sizes.
    Variable {
        /// Minimum encoded size.
        min: usize,
        /// Maximum encoded size.
        max: usize,
    },
}

/// Error type returned by failed encoding routines.
pub struct EncError;

/// Common `Result` type returned by encoding routines in this crate.
pub type EncResult = Result<(), EncError>;

/// Encoder type used to encode messages into slices.
pub struct Encoder<'a> {
    buf: &'a mut [u8],
    #[doc(hidden)]
    pub pos: usize,
}

impl<'a> Encoder<'a> {
    /// Create a new `Encoder` from a mutable slice.
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Consumes `self` and returns the number of encoded bytes.
    pub fn consume(self) -> usize {
        self.pos
    }
}

/// A trait for types that can be encoded with an `Encoder`.
pub trait Encode {
    /// `Self`'s size when encoded.
    const SIZE: EncSize;

    /// Encodes `self` using `ctx`.
    fn encode(&self, ctx: &mut Encoder) -> EncResult;
}

/// Error type returned by failed decoding routines.
pub struct DecError;

/// Common `Result` type returned by decoding routines in this crate.
pub type DecResult<T> = Result<T, DecError>;

/// Decoder type used to decode messages into slices.
pub struct Decoder<'a> {
    buf: &'a [u8],
    #[doc(hidden)]
    pub pos: usize,
}

impl<'a> Decoder<'a> {
    /// Create a new `Decoder` from a mutable slice.
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Consumes `self` and returns the number of decoded bytes.
    pub fn consume(self) -> usize {
        self.pos
    }
}

/// A trait for types that can be decoded with an `Decoder`.
pub trait Decode: Sized {
    /// `Self`'s size when encoded.
    const SIZE: EncSize;

    /// Decodes `self` using `ctx`.
    fn decode(ctx: &mut Decoder) -> DecResult<Self>;
}

macro_rules! impl_primitive {
    ($T:ty, $ENC_FN:ident, $WRITE_FN:ident, $DEC_FN:ident, $READ_FN:ident) => {
        impl<'a> Encoder<'a> {
            #[allow(missing_docs)]
            pub fn $ENC_FN(&mut self, val: $T) -> EncResult {
                use byteorder::ByteOrder;
                let ty_sz = ::std::mem::size_of::<$T>();
                let buf = &mut self.buf[self.pos..];
                if ty_sz > buf.len() {
                    Err(EncError)
                } else {
                    byteorder::LittleEndian::$WRITE_FN(buf, val);
                    self.pos += ty_sz;
                    Ok(())
                }
            }
        }

        impl<'a> Decoder<'a> {
            #[allow(missing_docs)]
            pub fn $DEC_FN(&mut self) -> DecResult<$T> {
                use byteorder::ByteOrder;
                let ty_sz = ::std::mem::size_of::<$T>();
                let buf = &self.buf[self.pos..];
                if ty_sz > buf.len() {
                    Err(DecError)
                } else {
                    self.pos += ty_sz;
                    Ok(byteorder::LittleEndian::$READ_FN(buf))
                }
            }
        }
    };
}

use crate::primitive::*;

impl_primitive!(I2, encode_i2, write_i16, decode_i2, read_i16);
impl_primitive!(I4, encode_i4, write_i32, decode_i4, read_i32);
impl_primitive!(R4, encode_r4, write_f32, decode_r4, read_f32);
impl_primitive!(R8, encode_r8, write_f64, decode_r8, read_f64);
impl_primitive!(U2, encode_u2, write_u16, decode_u2, read_u16);
impl_primitive!(U4, encode_u4, write_u32, decode_u4, read_u32);
