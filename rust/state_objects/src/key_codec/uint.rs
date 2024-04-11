use cosmossdk_core::{Code, err};
use cosmossdk_core::mem::Ref;
use crate::buffer::Writer;
use crate::key_codec::KeyCodec;
use crate::dynamic::DynamicValue;

impl KeyCodec for u32 {
    fn encode<B: Writer>(buf: &mut B, key: u32) -> cosmossdk_core::Result<()> {
        buf.write(&key.to_be_bytes())
    }

    fn decode<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(u32, usize)> {
        if buf.len() < 4 {
            return err!(Code::OutOfRange)
        }
        Ok((u32::from_be_bytes(buf.try_into().unwrap()), 4))
    }

    fn size_hint(_key: &Self::Borrowed<'_>) -> Option<usize> {
        Some(4)
    }

    fn to_dynamic(key: Self::Borrowed<'_>) -> DynamicValue {
        DynamicValue::Text(key.to_string())
    }

    fn from_value(value: DynamicValue) -> cosmossdk_core::Result<Self::Borrowed<'static>> {
        match value {
            DynamicValue::Text(text) => Ok(text.parse().map_err(|_| err!(Code::InvalidArgument, "invalid number: {:?}", text))?),
            _ => err!(Code::InvalidArgument, "expected text, got {:?}", value)
        }
    }
}
