use cosmos_core_macros::service;

pub use cosmos_message_api::{Address, StateToken};

pub struct Context {}

impl Context {
    pub fn self_address(&self) -> Address {
        todo!()
    }
    pub fn sender(&self) -> Address {
        todo!()
    }

    pub fn state_token(&self) -> StateToken {
        todo!()
    }

    pub fn derived_context(&self) -> Context {
        todo!()
    }

    pub fn set_state_token(&mut self, state_token: StateToken) {
        todo!()
    }

    pub fn set_sender(&mut self, sender: Address) {
        todo!()
    }

    // consume_gas consumes gas from the gas meter.
    // It returns an error if the gas meter has run out.
    // This method uses interior mutability to update the gas meter so that it can
    // work from read-only references to the context.
    pub fn consume_gas(&self, gas: u64) -> Result<()> {
        todo!()
    }
}

pub type Result<T> = core::result::Result<T, String>;

#[derive(Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Time(u64);

#[service]
pub trait BlockService {
    fn current_time(&self, ctx: &Context) -> crate::Result<Time>;
}

pub trait Account {}

pub trait Module {}

pub trait Serializable {}

pub struct Item<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl <T: Default> Item<T> {
    pub fn get(&self, ctx: &Context) -> Result<T> {
        todo!()
    }

    pub fn set(&self, ctx: &mut Context, value: &T) -> Result<()> {
        todo!()
    }
}

pub struct Map<K, V> {
    _phantom: core::marker::PhantomData<(K, V)>,
}

impl <K, V> Map<K, V> {
    pub fn get(&self, ctx: &Context, key: &K) -> Result<Option<V>> {
        todo!()
    }

    pub fn set(&self, ctx: &mut Context, key: &K, value: &V) -> Result<()> {
        todo!()
    }
}


pub trait Table {}

pub trait OnCreate {
    type InitMessage;

    fn on_create(&self, ctx: &mut Context, msg: &Self::InitMessage) -> Result<()>;
}

pub trait State {}

pub trait Service {
    fn client() -> Box<Self>;
    fn client_with_ctx<F>(ctx_fn: F) -> Box<Self>
        where F: FnOnce(&mut Context);
}