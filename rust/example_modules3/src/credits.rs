extern crate alloc;

use alloc::borrow::Cow;
use cosmos_core_api::Address;

pub struct CreditModule {
    state: CreditState<'_>,
}

pub struct MsgCreateClass<'a> {
    name: Cow<'a, str>,
}

impl <'a> Handler<MsgCreateClass<'a>, CreditEvent> for CreditModule {
    fn handle(&self, ctx: &dyn Context<CreditEvent>, msg: &MsgCreateClass<'a>) -> cosmos_result::Result<()> {
        let class_id = self.state.class_seq.next(ctx)?;
        ctx.emit(CreditEvent::ClassCreated { class_id, name: msg.name.clone(), admin: ctx.caller().clone() })
        ctx.emit(CreditEvent::ClassIssuerSet { class_id, issuer: ctx.caller().clone() })
    }
}

pub struct MsgCreateBatch<'a> {
    denom: Cow<'a, str>,
    project_id: u64,
    class_id: u64,
    to: Address,
    amount: UInt,
}

impl <'a> Handler<MsgCreateBatch<'a>, CreditEvent> for CreditModule {
    fn handle(&self, ctx: &dyn Context<CreditEvent>, msg: &MsgCreateBatch<'a>) -> cosmos_result::Result<()> {
        if !self.state.class_issuers.has(ctx, (msg.class_id, ctx.sender()))? {
            bail!(Code::Unauthorized, "only class issuer can create batches")
        }

        let batch_id = self.state.batch_seq.next(ctx)?;
        ctx.emit(CreditEvent::BatchCreated { batch_id, denom: msg.denom.clone(), project_id: msg.project_id, class_id: msg.class_id })?
        ctx.emit(CreditEvent::CreditsMinted { batch_id, to: msg.to.clone(), amount: msg.amount })
    }
}


pub struct CreditState<'a> {
    // classes: Table<Class<'a>>,
    // class_issuers: Table<ClassIssuer<'a>>,
    // projects: Table<Project<'a>>,
    // batches: Table<Batch<'a>>,
    // batches_by_denom: TableIndex<str, Batch<'a>>,
    #[map(prefix(1), key(id), value(name))]
    classes: Map<u64, str>,

    #[seq(prefix(11))]
    class_seq: Seq,

    #[map(prefix(2), key(class_id), value(admin))]
    class_admins: Map<u64, Address>,

    #[set(prefix(4), key(class_id, issuer))]
    class_issuers: Set<(u64, Address)>,

    #[unique_index(prefix(15), on(classes(name)))]
    classes_by_name: UniqueIndex<str, u64>,

    #[map(prefix(5), key(id), value(name))]
    projects: Map<u64, str>,

    #[unique_index(prefix(17), on(projects(name)))]
    projects_by_name: UniqueIndex<str, u64>,

    #[seq(prefix(12))]
    project_seq: Seq,

    #[map(prefix(6), key(project_id), value(admin))]
    project_admins: Map<u64, Address>,

    #[map(prefix(8), key(id), value(denom))]
    batches: Map<u64, str>,

    #[seq(prefix(13))]
    batch_seq: Seq,

    #[map(prefix(9), key(batch_id), value(project_id))]
    batch_projects: Map<u64, u64>,

    #[map(prefix(10), key(batch_id), value(class_id))]
    batch_classes: Map<u64, u64>,

    #[map(prefix(14), key(batch_id), value(balance))]
    batch_balances: UMap<(Address, u64)>,

    #[unique_index(prefix(16), on(batches(denom)))]
    batches_by_denom: UniqueIndex<str, u64>,
}

pub enum CreditEvent<'a> {
    ClassCreated { class_id: u64, name: Cow<'a, str>, admin: Address },
    ClassAdminSet { class_id: u64, admin: Address },
    ClassIssuerSet { class_id: u64, issuer: Address },
    ClassIssuerRemoved { class_id: u64, issuer: Address },
    ProjectCreated { project_id: u64, name: Cow<'a, str> },
    ProjectAdminSet { project_id: u64, admin: Address },
    BatchCreated { batch_id: u64, denom: Cow<'a, str>, project_id: u64, class_id: u64 },
    CreditsMinted { batch_id: u64, to: Address, amount: UInt },
}

impl<'a> EventReducer<CreditEvent<'a>> for CreditState {
    fn reduce(&self, event: &CreditEvent<'a>, store: &WriteStore) -> cosmos_result::Result<()> {
        match event {
            CreditEvent::ClassCreated { class_id, name, admin } => {
                self.class_seq.set_next(store, *class_id)?;
                self.classes.insert(store, class_id.borrow(), name)?;
                self.class_admins.insert(store, class_id.borrow(), admin)
            }
            CreditEvent::ClassAdminSet { class_id, admin } => {
                self.class_admins.insert(store, class_id.borrow(), admin)
            }
            CreditEvent::ClassIssuerSet { class_id, issuer } => {
                self.class_issuers.insert(store, (class_id.borrow(), issuer))
            }
            CreditEvent::ClassIssuerRemoved { class_id, issuer } => {
                self.class_issuers.remove(store, (class_id.borrow(), issuer))
            }
            CreditEvent::ProjectCreated { project_id, name } => {
                self.project_seq.set_next(store, *project_id)?;
                self.projects.insert(store, project_id.borrow(), name)
            }
            CreditEvent::ProjectAdminSet { .. } => {}
            CreditEvent::BatchCreated { .. } => {}
            CreditEvent::CreditsMinted { batch_id, to, amount } => {
                self.batch_balances.add(store, (to, batch_id.borrow()), amount)
            }
        }
    }
}

trait EventReducer<E> {
    fn reduce(&self, event: &E, store: &WriteStore) -> cosmos_result::Result<()>;
}

trait Handler<M, E> {
    fn handle(&self, ctx: &dyn Context<E>, msg: &M) -> cosmos_result::Result<()>;
}

struct WriteStore {}

trait ReadStore {
    fn has(&self, key: &[u8]) -> cosmos_result::Result<bool> { todo!() }
    fn get(&self, key: &[u8]) -> cosmos_result::Result<Cow<[u8]>> { todo!() }
}

impl ReadStore for WriteStore {

}

impl WriteStore {

    fn set(&self, key: &[u8], value: &[u8]) -> cosmos_result::Result<()> { todo!() }
    fn delete(&self, key: &[u8]) -> cosmos_result::Result<()> { todo!() }
}

struct Map<K, V> {}

impl <K, V> Map<K, V> {
    fn insert(&self, store: &WriteStore, key: K, value: V) -> cosmos_result::Result<()> { todo!() }
    fn get(&self, store: &dyn ReadStore, key: K) -> cosmos_result::Result<V> { todo!() }
    fn delete(&self, store: &WriteStore, key: K) -> cosmos_result::Result<()> { todo!() }
}

struct Seq {}

impl Seq {
    fn next(&self, store: &dyn ReadStore) -> cosmos_result::Result<u64> { todo!() }
    fn set_next(&self, store: &WriteStore, expected: u64) -> cosmos_result::Result<()> { todo!() }
}

struct Set<T> {}

impl <T> Set<T> {

    fn has(&self, store: &dyn ReadStore, value: T) -> cosmos_result::Result<bool> { todo!() }
    fn insert(&self, store: &WriteStore, value: T) -> cosmos_result::Result<()> { todo!() }
    fn remove(&self, store: &WriteStore, value: T) -> cosmos_result::Result<()> { todo!() }
}

trait Context<E>: ReadStore {
    fn sender(&self) -> &Address;
    fn emit(&self, event: E) -> cosmos_result::Result<()>;
}