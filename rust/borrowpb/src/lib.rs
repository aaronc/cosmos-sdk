#![feature(test)]

mod wire;
mod buf;

extern crate alloc;

use alloc::borrow::Cow;
use integer_encoding::VarInt;
use cosmos_result::Result;
use crate::buf::{BytesWriter};
use crate::wire::encode_tag;

pub unsafe trait Message<'a> {
    fn message_size(&self) -> usize;
    fn encode_message(&self, buf: &mut BytesWriter) -> Result<()>;
    fn decode_message(buf: &'a [u8]) -> Result<(Self, usize)> where Self: Sized;
}

pub unsafe trait Value<'a> //: Default
{
    fn size(&self, encode_zero: bool) -> usize;
    fn encode(&self, buf: &mut BytesWriter, encode_zero: bool) -> Result<bool>;
    fn decode(buf: &'a [u8]) -> Result<(Self, usize)> where Self: Sized;
}

trait LengthDelimited<'a>: Value<'a> {}
trait Scalar<'a>: Value<'a> {}

unsafe impl <'a, T: Message<'a>> Value<'a> for T {
    fn size(&self, encode_zero: bool) -> usize {
        let size = self.message_size();
        if size == 0 && !encode_zero {
            return 0;
        }

        size + (size as u64).size(true)
    }

    fn encode(&self, buf: &mut BytesWriter, encode_presence: bool) -> Result<bool> {
        let written = buf.written();
        self.encode_message(buf)?;
        let size = buf.written() - written;
        if size == 0 && !encode_presence {
            return Ok(false);
        }

        (size as u64).encode(buf, true)
    }

    fn decode(buf: &'a [u8]) -> Result<(Self, usize)> {
        todo!()
    }
}

impl <'a, T: Message<'a>> LengthDelimited<'a> for T {}

struct MsgSend<'a> {
    from: Cow<'a, str>,
    to: Cow<'a, str>,
    coins: Repeated<'a, Coin<'a>, 16>,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Coin<'a> {
    denom: Cow<'a, str>,
    amount: Cow<'a, str>,
}

enum Repeated<'a, T: 'a, const MAX: usize> {
    Borrowed(&'a [T]),
    Decoded(heapless::Vec<T, MAX>)
}

unsafe impl<'a> Message<'a> for MsgSend<'a> {
    fn message_size(&self) -> usize {
        let mut size = 0;
        let from_size = self.from.size(false);
        if from_size > 0 {
            size += from_size + 1;
        }
        let to_size = self.to.size(false);
        if to_size > 0 {
            size += to_size + 1;
        }
        size += self.coins.size();
        size
    }

    fn encode_message(&self, buf: &mut BytesWriter) -> Result<()> {
        self.coins.encode(buf, 3)?;

        if self.to.encode(buf, false)? {
            encode_tag(2, wire::WireType::LengthDelimited).encode(buf, true)?;
        }

        if self.from.encode(buf, false)? {
            encode_tag(1, wire::WireType::LengthDelimited).encode(buf, true)?;
        }

        Ok(())
    }

    fn decode_message(buf: &'a [u8]) -> Result<(Self, usize)> {
        todo!()
    }
}

unsafe impl<'a> Message<'a> for Coin<'a> {
    fn message_size(&self) -> usize {
        let mut size = 0;
        let denom_size = self.denom.size(false);
        if denom_size > 0 {
            size += denom_size + 1;
        }
        let amount_size = self.amount.size(false);
        if amount_size > 0 {
            size += amount_size + 1;
        }
        size
    }

    fn encode_message(&self, buf: &mut BytesWriter) -> Result<()> {
        if self.amount.encode(buf, false)? {
            encode_tag(2, wire::WireType::LengthDelimited).encode(buf, true)?;
        }


        if self.denom.encode(buf, false)? {
            encode_tag(1, wire::WireType::LengthDelimited).encode(buf, true)?;
        }

        Ok(())
    }

    fn decode_message(buf: &'a [u8]) -> Result<(Self, usize)> {
        let len = buf.len();
        let mut read = 0;
        let mut res: Self = Default::default();
        loop {
            if read >= len {
                return Ok((res, read));
            }
            let (tag, n) = u64::decode(&buf[read..])?;
            read += n;
            let (num, _wt) = wire::decode_tag(tag);
            match num {
                1 => {
                    let (denom, n) = Cow::decode(&buf[read..])?;
                    read += n;
                    res.denom = denom;
                }
                2 => {
                    let (amount, n) = Cow::decode(&buf[read..])?;
                    read += n;
                    res.amount = amount;
                }
                _ => {
                    todo!()
                }
            }
        }
    }
}

impl<'a, T: LengthDelimited<'a> + 'a, const N: usize> Repeated<'a, T, N> {
    fn size(&self) -> usize {
        match self {
            Repeated::Borrowed(xs) => {
                if xs.len() == 0 {
                    return 0;
                }

                let mut size = 0;
                for x in xs.iter() {
                    size += x.size(true);
                }
                size
            }
            Repeated::Decoded(xs) => {
                let mut size = 0;
                for x in xs.iter() {
                    size += x.size(true);
                }
                size
            }
        }
    }

    fn encode(&self, buf: &mut BytesWriter, field_num: i32) -> Result<()> {
        let tag = encode_tag(field_num, wire::WireType::LengthDelimited);
        match self {
            Repeated::Borrowed(xs) => {
                for x in xs.iter().rev() {
                    x.encode(buf, true)?;
                    tag.encode(buf, true)?;
                }
            }
            Repeated::Decoded(xs) => {
                for x in xs.iter().rev() {
                    x.encode(buf, true)?;
                    tag.encode(buf, true)?;
                }
            }
        };
        Ok(())
    }

    fn decode_one(&mut self, buf: &'a [u8]) -> Result<usize> {
        todo!()
    }
}

const MAX_VARINT32_BYTES: usize = 5;
const MAX_VARINT64_BYTES: usize = 10;

unsafe impl<'a> Value<'a> for Cow<'a, str> {
    fn size(&self, encode_empty: bool) -> usize {
        let len = self.len();
        if len == 0 && !encode_empty {
            return 0;
        }
        len + len.required_space()
    }

    fn encode(&self, buf: &mut BytesWriter, encode_empty: bool) -> Result<bool> {
        // copy the string to the end of the buffer
        let len = self.len();
        if len == 0 && !encode_empty {
            return Ok(false);
        }

        buf.write(len).copy_from_slice(self.as_bytes());

        // encode length
        let len = len as u64;
        len.encode(buf, true)?;
        Ok(true)
    }

    fn decode(buf: &'a [u8]) -> Result<(Self, usize)> {
        let (len, read) = u64::decode(buf)?;
        let len = len as usize;
        let s = std::str::from_utf8(&buf[read..read + len]).unwrap();
        Ok((Cow::Borrowed(s), read + len))
    }
}

unsafe impl<'a> Value<'a> for u64 {
    fn size(&self, encode_zero: bool) -> usize {
        if *self == 0 && !encode_zero {
            return 0;
        }

        self.required_space()
    }

    fn encode(&self, buf: &mut BytesWriter, encode_zero: bool) -> Result<bool> {
        if *self == 0 && !encode_zero {
            return Ok(false);
        }

        self.encode_var(buf.write(self.required_space()));
        Ok(true)
    }

    fn decode(buf: &'a [u8]) -> Result<(Self, usize)> {
        let (num, n) = u64::decode_var(buf).unwrap();
        Ok((num, n))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use prost::Message;
    use super::*;
    use test::{Bencher, black_box};
    use zeropb::ZeroCopy;


    #[derive(Message, Clone, PartialEq)]
    struct ProstMsgSend {
        #[prost(string, tag = "1")]
        from: String,

        #[prost(string, tag = "2")]
        to: String,

        #[prost(message, repeated, tag = "3")]
        amount: Vec<ProstCoin>,
    }

    #[derive(Message, Clone, PartialEq)]
    struct ProstCoin {
        #[prost(string, tag = "1")]
        denom: String,
        #[prost(string, tag = "2")]
        amount: String,
    }

    struct ZeropbCoin {
        denom: zeropb::Str,
        amount: zeropb::Str,
    }

    unsafe impl ZeroCopy for ZeropbCoin {}

    #[test]
    fn test_encode_decode() {
        let msg_send = MsgSend {
            from: Cow::Borrowed("bob"),
            to: Cow::Borrowed("sally"),
            coins: Repeated::Borrowed(&[
                Coin {
                    denom: Cow::Borrowed("uatom"),
                    amount: Cow::Borrowed("1000"),
                },
                Coin {
                    denom: Cow::Borrowed("foo"),
                    amount: Cow::Borrowed("100"),
                },
                Coin {
                    denom: Cow::Borrowed("bar"),
                    amount: Cow::Borrowed("200"),
                },
            ]),

        };

        let mut bytes = vec![0; msg_send.size(false)];
        let mut buf = BytesWriter::new(&mut bytes);
        msg_send.encode(&mut buf, false).unwrap();

        let prost_decoded = ProstMsgSend::decode(&*bytes).unwrap();
        println!("{:?}", prost_decoded);

        // let (decoded_coins, _) = Coin::decode(&*bytes).unwrap();
        // println!("{:?}", decoded_coins);
        // assert_eq!(coin, decoded_coins);
    }

    #[bench]
    fn bench_borrowpb_decode(b: &mut test::Bencher) {
        let coin = Coin {
            denom: Cow::Borrowed("uatom"),
            amount: Cow::Borrowed("1000"),
        };

        let mut bytes = vec![0; coin.size(false)];
        let mut buf = BytesWriter::new(&mut bytes);
        coin.encode(&mut buf, false).unwrap();
        b.iter(|| {
            let (decoded, _) = Coin::decode(&*bytes).unwrap();
            black_box(format!("{:?}{:?}", decoded.amount, decoded.denom))
        });
    }

    #[bench]
    fn bench_borrowpb_encode(b: &mut test::Bencher) {
        let coin = Coin {
            denom: Cow::Borrowed("uatom"),
            amount: Cow::Borrowed("1000"),
        };

        b.iter(|| {
            let mut bytes = vec![0; coin.size(false)];
            let mut buf = BytesWriter::new(&mut bytes);
            coin.encode(&mut buf, false).unwrap();
            black_box(bytes);
        });
    }

    #[bench]
    fn bench_prost_encode(b: &mut test::Bencher) {
        let coin = ProstCoin {
            denom: "uatom".to_string(),
            amount: "1000".to_string(),
        };


        b.iter(|| {
            let mut bytes = vec![0; coin.encoded_len()];
            coin.encode(&mut bytes).unwrap();
            black_box(bytes);
        });
    }

    #[bench]
    fn bench_zeropb_encode(b: &mut test::Bencher) {
        b.iter(|| {
            let mut coin = zeropb::Root::<ZeropbCoin>::new();
            black_box(coin.amount.set("1000").unwrap());
            black_box(coin.denom.set("uatom").unwrap());
        })
    }

    #[bench]
    fn bench_prost_decode(b: &mut test::Bencher) {
        let coin = Coin {
            denom: Cow::Borrowed("uatom"),
            amount: Cow::Borrowed("1000"),
        };

        let mut bytes = vec![0; coin.size(false)];
        let mut buf = BytesWriter::new(&mut bytes);
        coin.encode(&mut buf, false).unwrap();
        b.iter(|| {
            let decoded = ProstCoin::decode(&*bytes).unwrap();
            black_box(format!("{}{}", decoded.amount, decoded.denom))
        });
    }

    #[test]
    fn test_zeropb() {
        let mut coin = zeropb::Root::<ZeropbCoin>::new();
        coin.denom.set("uatom").unwrap();
        coin.amount.set("1000").unwrap();

        println!("{}{}", coin.amount.as_str(), coin.denom.as_str())
    }

    #[bench]
    fn bench_zeropb_decode(b: &mut test::Bencher) {
        let mut coin = zeropb::Root::<ZeropbCoin>::new();
        coin.denom.set("uatom").unwrap();
        coin.amount.set("1000").unwrap();

        b.iter(|| {
            black_box(format!("{}{}", coin.amount.as_str(), coin.denom.as_str()))
        });
    }
}