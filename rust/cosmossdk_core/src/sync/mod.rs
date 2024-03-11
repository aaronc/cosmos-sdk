extern crate core;
use core::ops::Fn;
use crate::{Context, Result};

pub struct PrepareContext(Context);

pub struct ExecContext(Context);

pub struct Exec<T> {
    exec: Box<dyn FnOnce(&mut ExecContext) -> Result<T>>
}

pub type Completer<'a, R> = &'a dyn FnOnce(&mut ExecContext) -> Result<R>;

pub type Completer1<'a, P1, R> = &'a dyn FnOnce(&mut ExecContext, P1) -> Result<R>;

pub trait AsyncInternalHandler<Request, Response = ()> {
    fn handle(&self, ctx: PrepareContext, req: &Request) -> Result<Exec<Response>>;
}

impl PrepareContext {
    pub fn exec<T, F: FnOnce(&mut ExecContext) -> Result<T> + 'static>(self, f: F) -> Result<Exec<T>> {
        Ok(Exec { exec: Box::new(f) })
    }
}