use cosmos_core::{Context, Address};

pub struct TestApp<'a> {}

impl<'a> TestApp<'a> {
    pub fn client_context(&self, caller: &Address) -> Context<'a> {
        todo!()
    }
}

impl <'a> TestApp {
    pub fn new() -> Self<'a> {
        Self {}
    }

    pub fn add_handler<T>(&mut self) {

    }
}