#![feature(test)]

mod wire;
mod buf;

extern crate alloc;
extern crate core;

use alloc::borrow::Cow;
use core::fmt::Debug;
use integer_encoding::VarInt;
use cosmos_result::{bail, Code, new_error, Result};
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

unsafe impl<'a, T: Message<'a>> Value<'a> for T {
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
        let (len, m) = u64::decode(buf)?;
        let (res, n) = T::decode_message(&buf[m..m + len as usize])?;
        Ok((res, m + n))
    }
}

impl<'a, T: Message<'a>> LengthDelimited<'a> for T {}

#[derive(Default, Debug, PartialEq, Eq)]
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

#[derive(Debug, Eq, PartialEq)]
enum Repeated<'a, T: Debug + 'a, const MAX: usize> {
    Borrowed(&'a [T]),
    Owned(heapless::Vec<T, MAX>),
}

impl<'a, T: Debug + 'a, const MAX: usize> Default for Repeated<'a, T, MAX> {
    fn default() -> Self {
        Repeated::Owned(Default::default())
    }
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
        size += self.coins.size(3);
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
                    let (from, n) = Cow::decode(&buf[read..])?;
                    read += n;
                    res.from = from;
                }
                2 => {
                    let (to, n) = Cow::decode(&buf[read..])?;
                    read += n;
                    res.to = to;
                }
                3 => {
                    read += res.coins.decode_one(&buf[read..])?
                }
                _ => {
                    todo!()
                }
            }
        }
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

impl<'a, T: LengthDelimited<'a> + Debug + 'a, const N: usize> Repeated<'a, T, N> {
    fn size(&self, field_num: i32) -> usize {
        let tag = encode_tag(field_num, wire::WireType::LengthDelimited);
        let tag_len = tag.size(true);
        match self {
            Repeated::Borrowed(xs) => {
                if xs.len() == 0 {
                    return 0;
                }

                let mut size = 0;
                for x in xs.iter() {
                    size += x.size(true);
                    size += tag_len;
                }
                size
            }
            Repeated::Owned(xs) => {
                let mut size = 0;
                for x in xs.iter() {
                    size += x.size(true);
                    size += tag_len;
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
            Repeated::Owned(xs) => {
                for x in xs.iter().rev() {
                    x.encode(buf, true)?;
                    tag.encode(buf, true)?;
                }
            }
        };
        Ok(())
    }

    fn decode_one(&mut self, buf: &'a [u8]) -> Result<usize> {
        let (x, n) = T::decode(buf)?;
        let Repeated::Owned(xs) = self else {
            bail!(Code::Internal, "expected Decoded");
        };
        xs.push(x).map_err(|e| new_error!(Code::Internal, "can't push to vec: {:?}", e))?;
        Ok(n)
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

    use alloc::fmt::format;
    use super::*;
    use test::{Bencher, black_box};
    use zeropb::ZeroCopy;
    use crate::Message;


    #[derive(::prost::Message, Clone, PartialEq)]
    struct ProstMsgSend {
        #[prost(string, tag = "1")]
        from: String,

        #[prost(string, tag = "2")]
        to: String,

        #[prost(message, repeated, tag = "3")]
        amount: Vec<ProstCoin>,
    }

    #[derive(::prost::Message, Clone, PartialEq)]
    struct ProstCoin {
        #[prost(string, tag = "1")]
        denom: String,
        #[prost(string, tag = "2")]
        amount: String,
    }

    struct ZeropbMsgSend {
        from: zeropb::Str,
        to: zeropb::Str,
        amount: zeropb::Repeated<ZeropbCoin>,
    }

    unsafe impl ZeroCopy for ZeropbMsgSend {}

    struct ZeropbCoin {
        denom: zeropb::Str,
        amount: zeropb::Str,
    }

    unsafe impl ZeroCopy for ZeropbCoin {}

    #[test]
    fn test_encode_decode_coin() {
        let coin = Coin {
            denom: Cow::Borrowed("uatom"),
            amount: Cow::Borrowed("1000"),
        };

        let mut bytes = vec![0; coin.message_size()];
        let mut buf = BytesWriter::new(&mut bytes);
        coin.encode_message(&mut buf).unwrap();

        let prost_decoded = <ProstMsgSend as prost::Message>::decode(&*bytes).unwrap();
        println!("{:?}", prost_decoded);

        let (decoded_coins, _) = Coin::decode_message(&*bytes).unwrap();
        println!("{:?}", decoded_coins);
        assert_eq!(coin, decoded_coins);
    }

    fn msg_send1() -> MsgSend<'static> {
        MsgSend {
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
        }
    }

    #[test]
    fn test_encode_decode_msg_send() {
        let msg_send = msg_send1();

        let size = msg_send.message_size();
        let mut bytes = vec![0; size * 2];
        let mut buf = BytesWriter::new(&mut bytes);
        msg_send.encode_message(&mut buf).unwrap();
        let res = buf.result();

        let prost_decoded = <ProstMsgSend as prost::Message>::decode(res).unwrap();
        println!("{:?}", prost_decoded);

        let (decoded, _) = MsgSend::decode_message(res).unwrap();
        println!("{:?}", decoded);
        // assert_eq!(msg_send, decoded);
    }

    #[bench]
    fn bench_borrowpb_decode(b: &mut test::Bencher) {
        let msg = msg_send1();

        let mut bytes = vec![0; msg.message_size() * 2];
        let mut buf = BytesWriter::new(&mut bytes);
        msg.encode_message(&mut buf).unwrap();
        let res = buf.result();
        b.iter(|| {
            let (decoded, _) = MsgSend::decode_message(res).unwrap();
            black_box(format!("{:?}{:?}", decoded.from, decoded.to));
            let Repeated::Owned(coins) = decoded.coins else {
                panic!("expected owned")
            };
            for coin in coins {
                black_box(format!("{}{}", coin.amount, coin.denom));
            }
        });
    }

    #[bench]
    fn bench_borrowpb_encode(b: &mut test::Bencher) {
        let msg = msg_send1();

        b.iter(|| {
            let mut bytes = vec![0; msg.message_size() + 1];
            let mut buf = BytesWriter::new(&mut bytes);
            msg.encode(&mut buf, false).unwrap();
            black_box(bytes);
        });
    }

    #[bench]
    fn bench_prost_encode(b: &mut test::Bencher) {
        let msg = ProstMsgSend {
            from: "bob".to_string(),
            to: "sally".to_string(),
            amount: vec![ProstCoin {
                denom: "uatom".to_string(),
                amount: "1000".to_string(),
            }, ProstCoin {
                denom: "foo".to_string(),
                amount: "100".to_string(),
            }, ProstCoin {
                denom: "bar".to_string(),
                amount: "200".to_string(),
            }],
        };
        b.iter(|| {
            let mut bytes = vec![0; <ProstMsgSend as ::prost::Message>::encoded_len(&msg)];
            <ProstMsgSend as ::prost::Message>::encode(&msg, &mut bytes).unwrap();
            black_box(bytes);
        });
    }

    fn zero1() -> zeropb::Root<ZeropbMsgSend> {
        let mut msg = zeropb::Root::<ZeropbMsgSend>::new();
        msg.from.set("bob").unwrap();
        msg.to.set("sally").unwrap();
        let mut coins = msg.amount.start_write().unwrap();
        let mut coin = coins.append().unwrap();
        coin.amount.set("1000").unwrap();
        coin.denom.set("uatom").unwrap();
        let mut coin = coins.append().unwrap();
        coin.amount.set("100").unwrap();
        coin.denom.set("foo").unwrap();
        let mut coin = coins.append().unwrap();
        coin.amount.set("200").unwrap();
        coin.denom.set("bar").unwrap();
        msg
    }

    #[bench]
    fn bench_zeropb_encode(b: &mut test::Bencher) {
        b.iter(|| {
            black_box(zero1())
        })
    }

    #[bench]
    fn bench_prost_decode(b: &mut test::Bencher) {
        let msg = msg_send1();

        let mut bytes = vec![0; msg.message_size() * 2];
        let mut buf = BytesWriter::new(&mut bytes);
        msg.encode_message(&mut buf).unwrap();
        let res = buf.result();
        b.iter(|| {
            let decoded = <ProstMsgSend as ::prost::Message>::decode(res).unwrap();
            black_box(format!("{}{}", decoded.from, decoded.to));
            for coin in decoded.amount {
                black_box(format!("{}{}", coin.amount, coin.denom));
            }
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
        let mut msg = zero1();

        b.iter(|| {
            black_box(format!("{}{}", msg.from.as_str(), msg.to.as_str()));
            for coin in msg.amount.into_iter() {
                black_box(format!("{}{}", coin.amount, coin.denom));
            }
        });
    }
}