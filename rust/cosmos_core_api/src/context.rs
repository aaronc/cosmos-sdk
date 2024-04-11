use alloc::borrow::Cow;
use crate::Address;

trait Context: ReadStore {
    fn op_id(&self) -> u64;
    fn caller_opt(&self) -> &Option<Address>;
    fn self_address(&self) -> &Address;
}

trait Event {}

trait EventReducer<E: Event, Store: WriteStore> {
    fn reduce(&self, store: &Store);
}

trait ExecContext: Context {
    fn caller(&self) -> &Address;

    // fn emit(&self, evt: )
}

trait ReadStore {
    fn has(&self, key: &[u8]) -> cosmos_result::Result<bool>;
    fn get(&self, key: &[u8]) -> cosmos_result::Result<Cow<[u8]>>;
}

trait OrderedStore: ReadStore {
    // TODO
}

trait WriteStore: ReadStore {
    fn set(&self, key: &[u8], value: &[u8]) -> cosmos_result::Result<()>;
    fn delete(&self, key: &[u8]) -> cosmos_result::Result<()>;
}