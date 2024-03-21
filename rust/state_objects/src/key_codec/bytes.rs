use cosmossdk_core::{Code, err};
use cosmossdk_core::mem::Ref;
use crate::key_codec::{KeyCodec, Writer};
use crate::key_codec::tuple::KeyPartCodec;

impl KeyCodec for [u8] {
    type Borrowed<'a> = &'a [u8];
    type AsRef<'a> = Ref<'a, &'a [u8]>;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        buf.write(key)
    }

    fn decode<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(Self::Borrowed<'a>, usize)> {
        Ok((buf, buf.len()))
    }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a> {
        r.cast(borrowed)
    }
}

impl KeyPartCodec for [u8] {
    fn decode_non_terminal<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(Self::Borrowed<'a>, usize)> {
        let buf_len = buf.len();
        if buf_len < 2 {
            return err!(Code::OutOfRange, "key too short, expected at least 2 bytes, got {}", buf_len)
        }

        let len = u16::from_be_bytes(buf.try_into().unwrap()) as usize;
        if len > buf_len - 2 {
            return err!(Code::OutOfRange, "key too short, expected at least {} bytes, got {}", len + 2, buf.len())
        }

        Ok((&buf[2..len + 2], len + 2))
    }

    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        let len = key.len();
        if len > u16::MAX as usize {
            return err!(Code::OutOfRange, "key too long, maximum length is 65535, got {}", len)
        }

        buf.write(&(len as u16).to_be_bytes())?;
        buf.write(key)
    }
}