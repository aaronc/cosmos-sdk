use dashu_int::UBig;
use cosmossdk_core::{Code, Context, ok, err, Address};
use cosmossdk_core::module::{Module, ModuleContext, ModuleReadContext};
use state_objects::{Index, Map, UBigMap};
use crate::example::bank::v1::{InternalSendLazy, MsgSend, MsgSendResponse, MsgServer, QueryBalance, QueryBalanceResponse, QueryServer};
use core::borrow::Borrow;
use cosmossdk_core::sync::{AsyncInternalHandler, Exec, PrepareContext};
use cosmossdk_macros::Module;
use state_objects_macros::State;

#[derive(Module)]
#[module(name="example.bank.v1.BankModule",
services(MsgServer, QueryServer))]
pub struct Bank {
    state: BankState,

    #[config]
    config: crate::example::bank::v1::BankModule,
}

#[derive(State)]
pub struct BankState {
    #[map(prefix = 1, key(denom), value(enabled))]
    send_enabled: Map<str, bool>,

    #[map(prefix = 2, key(address, denom), value(balance))]
    balances: UBigMap<([u8], str)>,
    //
    #[map(prefix = 3, key(denom), value(supply))]
    supplies: UBigMap<str>,
    // #[map(prefix = 4, key(module, denom), value(balance))]
    // module_balances: UBigMap<(str, str)>,
    //
    // #[index(prefix = 5, on(balances(denom, address)))]
    // balances_by_denom: Index<(str, [u8]), UBig>,
    //
    // #[index(prefix = 6, on(balances(denom, module)))]
    // module_balances_by_denom: Index<(str, str), UBig>,
}

pub enum BankEvent {
    Send{from: Address, to: Address, denom: String, amount: UBig},
    Burn{from: Address, denom: String, amount: UBig},
    Mint{to: Address, denom: String, amount: UBig},
}

// impl Module for Bank {
//     fn route(&self, route_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> Code {
//         // service id is second to last byte of route id
//         let service_id = (route_id >> 8) & 0xffu64;
//         // method id is last byte of route id
//         let method_id = route_id & 0xffu64;
//         match service_id {
//             0x0 => <dyn MsgServer as Server>::route(self, method_id, ctx, req, res),
//             _ => Code::Unimplemented,
//         }
//     }
// }

impl MsgServer for Bank {
    fn send(&self, ctx: &dyn ModuleContext, req: &MsgSend) -> ::cosmossdk_core::Result<MsgSendResponse> {
        // checking send enabled uses last block state so no need to synchronize reads
        if !self.state.send_enabled.get_stale(ctx, req.denom.borrow())? {
            return err!(Code::Unavailable, "send disabled for denom {}", req.denom)
        }

        let amount = UBig::from_le_bytes(&req.amount);

        self.state.balances.safe_sub(ctx, (req.from.borrow(), req.denom.borrow()), &amount)?;
        self.state.balances.add(ctx, (req.to.borrow(), req.denom.borrow()), &amount)?;

        ok()
    }
}

impl QueryServer for Bank {
    fn balance(&self, ctx: &dyn ModuleContext, req: &QueryBalance) -> cosmossdk_core::Result<QueryBalanceResponse> {
        self.state.balances.read(ctx, (req.address.borrow(), req.denom.borrow())).map(|balance| {
            QueryBalanceResponse {
                balance: balance.to_le_bytes().to_vec(),
            }
        })
    }
}

impl AsyncInternalHandler<InternalSendLazy> for Bank {
    fn handle(&self, ctx: PrepareContext, req: &InternalSendLazy) -> cosmossdk_core::Result<Exec<()>> {
        // let safe_sub = self.state.balances.prepare_safe_sub(&ctx, (req.from.borrow(), req.denom.borrow()))?;
        // let add_lazy = self.state.balances.prepare_add_lazy(&ctx, (req.to.borrow(), req.denom.borrow()))?;
        // let amount = UBig::from_le_bytes(&req.amount);
        // ctx.exec(move |ctx| {
        //     safe_sub(ctx, &amount)?;
        //     add_lazy(ctx, &amount)?;
        //     Ok(())
        // })
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use cosmossdk_core::AgentId;
    use super::*;
    use cosmossdk_core::testing::{TestApp, TestClient, TestStore};
    use crate::example::bank::v1::MsgClient;
    // use cosmossdk_core::Server;
    // use cosmossdk_core::store::{MockStore, Store};

    struct Fixture {
        app: Box<TestApp>,
        test_store: TestStore,
        client: TestClient,
        bank_client: MsgClient,
    }

    fn fixture() -> Fixture {
        // let mut mock_store = TestStore::default();
        let mut app = Box::new(TestApp::new());
        app.add_module_default::<Bank>("bank");
        let mock_store = app.add_mock_server(TestStore::default());
        let mut client = app.test_client(AgentId::Account([0; 32].into()));
        let mut bank_client = client.new::<MsgClient>();
        let mut ctx = client.context();
        Fixture {
            app,
            test_store: todo!(),
            client,
            bank_client,
        }
    }

    #[test]
    fn test() {
        let mut f = fixture();
        f.bank_client.send(&mut f.client.context(), &MsgSend {
            from: vec![0; 32],
            to: vec![0; 32],
            denom: "uatom".to_string(),
            amount: vec![0; 32],
        });
    }
}
