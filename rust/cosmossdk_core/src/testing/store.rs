extern crate core;
extern crate alloc;
extern crate std;
use core::option::{Option};
use core::option::Option::{Some, None};
use core::result::Result::{Ok, Err};
use core::ops::FnOnce;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use crate::{Code, Context, Result};
use crate::store::{StoreServer};
use core::todo;
use core::convert::{From, Into};
use crate::id::AgentId;
use core::default::Default;
use alloc::sync::{Arc};
use std::sync::{RwLock};
use alloc::boxed::Box;
use alloc::vec;

pub struct TestStore {
    data: Arc<RwLock<Data>>
}

#[derive(Default)]
struct Data {
    committed: BTreeMap<Vec<u8>, Vec<u8>>,
    writes: BTreeMap<Vec<u8>, Option<Vec<u8>>>,
    lazy: BTreeMap<Vec<u8>, Vec<Box<dyn FnOnce(Option<&[u8]>) -> Option<Vec<u8>>>>>,
}

impl Default for TestStore {
    fn default() -> Self {
        TestStore {
            data: Arc::new(RwLock::new(Data::default()))
        }
    }
}

fn prepend_id(id: &AgentId, key: &[u8]) -> Vec<u8> {
    match id {
        AgentId::Module(name) => {
            let mut result = Vec::with_capacity(name.len() + key.len() + 5);
            result.push(b'0');
            let len = name.len() as u32;
            result.extend_from_slice(&len.to_be_bytes());
            result.extend_from_slice(name.as_bytes());
            result.extend_from_slice(key);
            result
        }
        AgentId::Account(account) => {
            let mut result = Vec::with_capacity(account.len() + key.len() + 5);
            result.push(b'1');
            let len = account.len() as u32;
            result.extend_from_slice(&len.to_be_bytes());
            result.extend_from_slice(account);
            result.extend_from_slice(key);
            result
        }
    }
}

impl TestStore {
    pub fn commit(&self) {
        // let mut data = self.data.write().unwrap();
        // let writes = &data.writes;
        // let mut committed = &mut data.committed;
        // for (key, value) in writes.iter() {
        //     if let Some(value) = value {
        //         committed.insert(key.clone(), value.clone());
        //     } else {
        //         committed.remove(key);
        //     }
        // }
        // data.writes.clear();
        // for (key, mut fns) in data.lazy.iter() {
        //     let value = data.committed.get(key).cloned();
        //     for f in fns.iter_mut() {
        //         if let Some(value) = value.as_ref() {
        //             let value = Some(value.as_slice());
        //             let new_value = f(value);
        //             if let Some(new_value) = new_value {
        //                 data.committed.insert(key.clone(), new_value.into());
        //             } else {
        //                 data.committed.remove(key);
        //             }
        //         } else {
        //             let new_value = f(None);
        //             if let Some(new_value) = new_value {
        //                 data.committed.insert(key.clone(), new_value.into());
        //             } else {
        //                 data.committed.remove(key);
        //             }
        //         }
        //     }
        // }
    }
}

impl StoreServer for TestStore {
    fn get(&self, ctx: &mut Context, caller: &AgentId, key: &[u8]) -> Result<Vec<u8>> {
        let key = prepend_id(caller, key);
        let data = self.data.read().unwrap();
        if let Some(value) = data.writes.get(&key) {
            if let Some(value) = value {
                return Ok(value.clone().into());
           } else {
                return Err(Code::NotFound.into());
            }
        } else {
            if let Some(value) = data.committed.get(&key) {
                return Ok(value.clone().into());
            } else {
                return Err(Code::NotFound.into());
            }
        }
    }

    fn set(&self, ctx: &mut Context, caller: &AgentId, key: &[u8], value: &[u8]) -> Result<()> {
        let key = prepend_id(caller, key);
        let mut data = self.data.write().unwrap();
        data.writes.insert(key, Some(value.to_vec()));
        Ok(())
    }

    fn delete(&self, ctx: &mut Context, caller: &AgentId, key: &[u8]) -> Result<()> {
        let key = prepend_id(caller, key);
        let mut data = self.data.write().unwrap();
        data.writes.insert(key, None);
        Ok(())
    }

    fn has(&self, ctx: &mut Context, caller: &AgentId, key: &[u8]) -> Result<bool> {
        let key = prepend_id(caller, key);
        let data = self.data.read().unwrap();
        if let Some(value) = data.writes.get(&key) {
            return Ok(value.is_some());
        } else {
            return Ok(data.committed.contains_key(&key));
        }
    }

    fn get_stale(&self, ctx: &mut Context, caller: &AgentId, key: &[u8]) -> Result<Vec<u8>> {
        let key = prepend_id(caller, key);
        let data = self.data.read().unwrap();
        if let Some(value) = data.committed.get(&key) {
            return Ok(value.clone().into());
        } else {
            return Err(Code::NotFound.into());
        }
    }

    fn set_lazy<F: FnOnce(Option<&[u8]>) -> Option<Vec<u8>>>(&self, ctx: &mut Context, caller: &AgentId, key: &[u8], value_fn: F) -> Result<()> {
        // let key = prepend_id(caller, key);
        // let mut data = self.data.write().unwrap();
        // if let Some(value) = data.lazy.get_mut(&key) {
        //     value.push(Box::new(value_fn));
        // } else {
        //     data.lazy.insert(key, vec![Box::new(value_fn)]);
        // }
        // Ok(())
        todo!()
    }
}