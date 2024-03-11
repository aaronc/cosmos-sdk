extern crate alloc;
extern crate core;

mod store;

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::default::Default;
use crate::{Context, Module};

pub use store::{TestStore};
use crate::id::AgentId;
use crate::routing::Client;

pub struct TestApp {
    client_routes: BTreeMap<u64, String>,
    server_routes: BTreeMap<String, u64>,
}

impl TestApp {
    pub fn new() -> Self {
        TestApp {
            client_routes: Default::default(),
            server_routes: Default::default(),
        }
    }

    pub fn add_module<T: Module>(&mut self, name: &str, config: T::Config) {
    }

    pub fn add_module_default<T: Module>(&mut self, name: &str)
    where
        T::Config: Default,
    {
    }

    pub fn add_mock_server<'a, T>(&'a mut self, server: &'a T) {

    }

    pub fn test_client(&mut self, identity: AgentId) -> TestClient {
        TestClient {
            app: self,
            identity,
        }
    }
}


pub struct TestClient<'a> {
    app: &'a mut TestApp,
    identity: AgentId,
}

impl <'a> TestClient<'a> {
    pub fn new<T: Client<'a>>(&mut self) -> T {
        todo!()
    }

    pub fn context(&self) -> Context {
        todo!()
    }
}