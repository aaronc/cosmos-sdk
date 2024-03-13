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

    pub fn add_mock_server<T>(&mut self, server: T) -> &T {
        todo!()
    }

    pub fn test_client(&mut self, identity: AgentId) -> TestClient {
        TestClient {
            app: self,
            identity,
        }
    }
}


pub struct TestClient {
    app: TestApp,
    identity: AgentId,
}

impl TestClient {
    pub fn new<T: Client>(&mut self) -> T {
        todo!()
    }

    pub fn identity(&self) -> &AgentId {
        &self.identity
    }

    pub fn context(&self) -> &dyn Context {
        todo!()
    }
}