extern crate core;
use core::ops::Fn;
use crate::{Context, Result};

pub struct PrepareContext();

pub struct ExecContext();

pub struct Exec<T> {
    exec: Box<dyn FnOnce(&mut ExecContext) -> Result<T>>
}

pub type Completer<R> = Box<dyn FnOnce(&mut ExecContext) -> Result<R>>;

pub type Completer1<P1, R> = Box<dyn FnOnce(&mut ExecContext, P1) -> Result<R>>;

pub trait AsyncInternalHandler<Request, Response = ()> {
    fn handle(&self, ctx: PrepareContext, req: &Request) -> Result<Exec<Response>>;
}

impl PrepareContext {
    pub fn exec<T, F: FnOnce(&mut ExecContext) -> Result<T> + 'static>(self, f: F) -> Result<Exec<T>> {
        Ok(Exec { exec: Box::new(f) })
    }
}