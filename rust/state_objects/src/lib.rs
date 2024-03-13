// mod async;

use dashu_int::UBig;
use cosmossdk_core::{Address, Code, Context, ReadContext, Result};
use cosmossdk_core::routing::Client;
use cosmossdk_core::store::{StoreClient};
use cosmossdk_core::sync::{Completer, Completer1, PrepareContext};

pub trait State: Client {}

pub trait KeyCodec {
    type In<'a>;
    type Out;
    type Keys<'a>;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()>;

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out>;
}

pub trait KeyPartCodec: KeyCodec {
    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        Self::encode(buf, key)
    }

    fn decode_non_terminal<B: Reader>(buf: &B) -> Result<Self::Out> {
        Self::decode(buf)
    }
}

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> Result<()>;
}

pub trait Reader {
    fn read(&self, n: usize) -> Result<&[u8]>;
    fn read_all(&self) -> Result<&[u8]>;
}

pub trait ValueCodec {
    type In<'a>;
    type Out;
    type Keys<'a>;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()>;

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out>;
}

// impl KeyCodec for u64 {
//     type In = u64;
// }

impl KeyCodec for Vec<u8> {
    type In<'a> = &'a [u8];
    type Out = Vec<u8>;
    type Keys<'a> = &'a str;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        buf.write(key)
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        buf.read_all().map(|x| x.to_vec())
    }
}

impl ValueCodec for Vec<u8> {
    type In<'a> = &'a [u8];
    type Out = Vec<u8>;
    type Keys<'a> = &'a str;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        buf.write(key)
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        buf.read_all().map(|x| x.to_vec())
    }
}

impl KeyPartCodec for Vec<u8> {
    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        // TODO variant encode length
        let len = key.len() as u16;
        buf.write(&len.to_le_bytes())?;
        buf.write(key)
    }

    fn decode_non_terminal<B: Reader>(buf: &B) -> Result<Self::Out> {
        let len = u16::from_le_bytes(buf.read(2)?.try_into().unwrap());
        buf.read(len as usize).map(|x| x.to_vec())
    }
}

impl<P1: KeyPartCodec, P2: KeyPartCodec> KeyCodec for (P1, P2) {
    type In<'a> = (P1::In<'a>, P2::In<'a>);
    type Out = (P1::Out, P2::Out);
    type Keys<'a> = (P1::Keys<'a>, P2::Keys<'a>);

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        P1::encode_non_terminal(buf, key.0)?;
        P2::encode(buf, key.1)
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        let p1 = P1::decode_non_terminal(buf)?;
        let p2 = P2::decode(buf)?;
        Ok((p1, p2))
    }
}

impl<P1: KeyPartCodec, P2: KeyPartCodec> ValueCodec for (P1, P2) {
    type In<'a> = <(P1, P2) as KeyCodec>::In<'a>;
    type Out = <(P1, P2) as KeyCodec>::Out;
    type Keys<'a> = <(P1, P2) as KeyCodec>::Keys<'a>;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        <(P1, P2) as KeyCodec>::encode(buf, key)
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        <(P1, P2) as KeyCodec>::decode(buf)
    }
}

// pub struct Pair<P1, P2>(pub P1, pub P2);
//
// impl <P1:KeyCodec, P2:KeyCodec> KeyCodec for Pair<P1, P2> {
//     // type In<'a> = Pair<P1::In<'a>, P2::In<'a>>;
//     type In<'a> = (&'a P1::In<'a>, &'a P2::In<'a>) where <P1 as KeyCodec>::In<'a>: 'a, <P2 as KeyCodec>::In<'a>: 'a;
//     type Out<'a> = Pair<P1::Out<'a>, P2::Out<'a>>;
//     type Keys<'a> = (&'a P1::Keys, &'a P2::Keys) where <P1 as KeyCodec>::Keys: 'a, <P2 as KeyCodec>::Keys: 'a;
//
//     fn encode<B: Writer>(buf: &mut B, key: &Self::In<'_>) -> Result<()> {
//         todo!()
//     }
//
//     fn encode_non_terminal<B: Writer>(buf: &mut B, key: &Self::In<'_>) -> Result<()> {
//         Err(Code::Unimplemented.into())
//     }
//
//     fn decode<B: Reader>(buf: &B) -> Result<Self::Out<'_>> {
//         todo!()
//     }
//
//     fn decode_non_terminal<B: Reader>(buf: &B) -> Result<Self::Out<'_>> {
//         Err(Code::Unimplemented.into())
//     }
// }

impl KeyCodec for String {
    type In<'a> = &'a str;
    type Out = String;
    type Keys<'a> = &'a str;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        todo!()
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        todo!()
    }
}

impl KeyPartCodec for String {
    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        todo!()
    }

    fn decode_non_terminal<B: Reader>(buf: &B) -> Result<Self::Out> {
        todo!()
    }
}

impl ValueCodec for bool {
    type In<'a> = bool;
    type Out = bool;
    type Keys<'a> = &'a str;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        todo!()
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        todo!()
    }
}

// impl <P1:KeyCodec, P2:KeyCodec> KeyCodec for (P1, P2) {
//     type In<'a> = (P1::In<'a>, P2::In<'a>);
// }
//
// impl <P1:KeyCodec, P2:KeyCodec, P3: KeyCodec> KeyCodec for (P1, P2, P3) {
//     type In = (P1::In, P2::In, P3::In);
// }

// struct CompactU64;
//
// impl KeyCodec for CompactU64 {
//     type In = u64;
// }

pub struct Map<K, V> {
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,

    name: String,
    prefix: Vec<u8>,
}

impl<K: KeyCodec, V: ValueCodec> Map<K, V> {
    pub fn new(store: StoreClient, prefix: &[u8], name: String, keys_names: K::Keys<'_>, values_names: &V::Keys<'_>) -> Map<K, V> {
        Self {
            _k: std::marker::PhantomData,
            _v: std::marker::PhantomData,
            name,
            prefix: prefix.to_vec(),
        }
    }

    pub fn get(&self, ctx: &dyn ReadContext, key: K::In<'_>) -> cosmossdk_core::Result<V::Out> {
        todo!()
    }

    pub fn get_stale(&self, ctx: &dyn Context, key: K::In<'_>) -> cosmossdk_core::Result<V::Out> {
        todo!()
    }

    pub fn set(&self, ctx: &dyn Context, key: K::In<'_>, value: &V::In<'_>) -> cosmossdk_core::Result<()> {
        todo!()
    }
}

//
// struct MyModule {
//     myMap: Map<CompactU64, u64>
// }
//
impl ValueCodec for UBig {
    type In<'a> = &'a UBig;
    type Out = UBig;
    type Keys<'a> = &'a str;

    fn encode<B: Writer>(buf: &mut B, key: &UBig) -> Result<()> {
        // let bytes = key.to_le_bytes();
        // let len = bytes.len() as u16;
        // buf.write(&len.to_le_bytes())?;
        // buf.write(&bytes)
        todo!()
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        // let len = u16::from_le_bytes(buf.read(2)?.try_into().unwrap());
        // let bytes = buf.read(len as usize)?;
        // Ok(UBig::from_bytes_le(bytes))
        todo!()
    }
}

pub struct UBigMap<K> {
    _k: std::marker::PhantomData<K>,
}

impl<K: KeyCodec> UBigMap<K> {
    pub fn has(&self, ctx: &dyn ReadContext, key: K::In<'_>) -> cosmossdk_core::Result<bool> {
        todo!()
    }

    pub fn read(&self, ctx: &dyn ReadContext, key: K::In<'_>) -> cosmossdk_core::Result<UBig> {
        todo!()
    }

    pub fn delete(&self, ctx: &dyn Context, key: K::In<'_>) -> cosmossdk_core::Result<()> {
        todo!()
    }

    pub fn safe_sub(&self, ctx: &dyn  Context, key: K::In<'_>, value: &UBig) -> cosmossdk_core::Result<UBig> {
        todo!()
    }

    pub fn add(&self, ctx: &dyn Context, key: K::In<'_>, value: &UBig) -> cosmossdk_core::Result<UBig> {
        todo!()
    }

    pub fn add_lazy(&self, ctx: &dyn Context, key: K::In<'_>, value: &UBig) {
        todo!()
    }

    pub fn prepare_safe_sub(&self, ctx: &PrepareContext, key: K::In<'_>) -> Result<Completer1<&UBig, ()>> {
        todo!()
    }

    pub fn prepare_add(&self, ctx: &PrepareContext, key: K::In<'_>) -> Result<Completer1<&UBig, ()>> {
        todo!()
    }

    pub fn prepare_add_lazy(&self, ctx: &PrepareContext, key: K::In<'_>) -> Result<Completer1<&UBig, ()>> {
        todo!()
    }
}

pub struct Index<K, V> {
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,
}

pub struct ProstBinary<T> {
    _t: std::marker::PhantomData<T>,
}

impl<T: 'static> ValueCodec for ProstBinary<T> where T: prost::Message + prost::Name {
    type In<'a> = &'a T;
    type Out = T;
    type Keys<'a> = &'a str;

    fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
        todo!()
    }

    fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
        todo!()
    }
}

pub struct Item<T> {
    _t: std::marker::PhantomData<T>,
}

impl <T: ValueCodec> Item<T> {
    pub fn get(&self, ctx: &dyn ReadContext) -> Result<T::Out> {
        todo!()
    }

    pub fn set(&self, ctx: &dyn Context, value: T::In<'_>) -> Result<()> {
        todo!()
    }
}

// impl KeyCodec for Address {
//     type In<'a> = &'a Address;
//     type Out = Address;
//     type Keys<'a> = &'a str;
//
//     fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
//         todo!()
//     }
//
//     fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
//         todo!()
//     }
// }
//
// impl KeyPartCodec for Address {
//     fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
//         todo!()
//     }
//
//     fn decode_non_terminal<B: Reader>(buf: &B) -> Result<Self::Out> {
//         todo!()
//     }
// }
//
// impl ValueCodec for Address {
//     type In<'a> = &'a Address;
//     type Out = Address;
//     type Keys<'a> = &'a str;
//
//     fn encode<B: Writer>(buf: &mut B, key: Self::In<'_>) -> Result<()> {
//         todo!()
//     }
//
//     fn decode<B: Reader>(buf: &B) -> Result<Self::Out> {
//         todo!()
//     }
// }