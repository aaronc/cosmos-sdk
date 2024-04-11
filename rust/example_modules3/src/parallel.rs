mod parallel;

use cosmos_result::{bail, Code};

pub struct BankModule {
    state: BankState
}

pub struct MsgSend {
    from: Address,
    to: Address,
    denom: String,
    amount: UBig,
}

impl Handler<MsgSend, BankEvent> for BankModule {
    fn handle(&self, ctx: &PrepareContext<BankEvent>, msg: &MsgSend) -> cosmos_result::Result<()> {
        if !self.state.send_enabled.get_stale(ctx, msg.denom.borrow())? {
            bail!(Code::Unavailable, "send disabled for denom {}", msg.denom)
        }

        let emit = ctx.prepare_emit(BankEvent::Send {
            from: msg.from.clone(),
            to: msg.to.clone(),
            denom: msg.denom.clone(),
            amount: msg.amount.clone(),
        })?;

        ctx.exec(move |ctx| {
            emit(ctx)?;
            Ok(())
        })
    }
}

pub struct BankState {
    send_enabled: Map<str, bool>,
    balances: UMap<([u8], str)>,
    supply: UMap<str>,
}

pub enum BankEvent {
    Send{from: Address, to: Address, denom: String, amount: UBig},
    Burn{from: Address, denom: String, amount: UBig},
    Mint{to: Address, denom: String, amount: UBig},
}

impl EventReducer<BankEvent> for BankState {
    fn reduce(&self, event: &BankEvent, store: &PrepareStore) -> cosmos_result::Result<()> {
        match event {
            BankEvent::Send { from, to, denom, amount } => {
                let safe_sub = self.balances.prepare_safe_sub(store, (from, denom))?;
                let add = self.balances.prepare_add(store, (to, denom))?;
                store.exec(move |store| {
                    safe_sub(store, &amount)?;
                    add(store, &amount)?;
                    Ok(())
                })
            }
            BankEvent::Burn { from, denom, amount } => {
                let balance_sub = self.balances.prepare_safe_sub(store, (from, denom))?;
                let supply_sub = self.supply.prepare_safe_sub(store, denom)?;
                store.exec(move |store| {
                    balance_sub(store, &amount)?;
                    supply_sub(store, &amount)?;
                    Ok(())
                })

            },
            BankEvent::Mint { to, denom, amount } => {
                let add = self.balances.prepare_add(store, (to, denom))?;
                let supply_add = self.supply.prepare_add(store, denom)?;
                store.exec(move |store| {
                    add(store, &amount)?;
                    supply_add(store, &amount)?;
                    Ok(())
                })
            }
        }
        Ok(())
    }
}
