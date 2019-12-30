use crate::{Decode, EncodingFormat, SharedData, VariantError};

/// Simpler sibling of [`Decode`].
///
/// This trait is implemented by all the types (mostly basic) whose signature is always the same
/// regardless of their actual value.
///
/// [`Decode`]: trait.Decode.html
pub trait SimpleDecode: Decode {
    /// Same as [`Decode::slice_data`], except you don't have to pass any signature to it.
    ///
    /// [`Decode::slice_data`]: trait.Decode.html#method.slice_data
    fn slice_data_simple(
        data: impl Into<SharedData>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::slice_data(data, Self::SIGNATURE_STR, format)
    }

    /// Same as [`Decode::decode`], except you don't have to pass any signature to it.
    ///
    /// [`Decode::decode`]: trait.Decode.html#method.decode
    fn decode_simple(
        data: impl Into<SharedData>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError>
    where
        Self: Sized,
    {
        Self::decode(data, Self::SIGNATURE_STR, format)
    }
}

impl SimpleDecode for u8 {}
impl SimpleDecode for bool {}
impl SimpleDecode for i16 {}
impl SimpleDecode for u16 {}
impl SimpleDecode for i32 {}
impl SimpleDecode for u32 {}
impl SimpleDecode for i64 {}
impl SimpleDecode for u64 {}
impl SimpleDecode for f64 {}
