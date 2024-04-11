mod parallel;
mod credits;

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

pub struct MsgBurn {
    from: Address,
    denom: String,
    amount: UBig,
}

impl Handler<MsgSend, BankEvent> for BankModule {
    fn handle(&self, ctx: &Context<BankEvent>, msg: &MsgSend) -> cosmos_result::Result<()> {
        if !self.state.send_enabled.get_stale(ctx, msg.denom.borrow())? {
            bail!(Code::Unavailable, "send disabled for denom {}", msg.denom)
        }

        ctx.emit(BankEvent::Send {
            from: msg.from.clone(),
            to: msg.to.clone(),
            denom: msg.denom.clone(),
            amount: msg.amount.clone(),
        })
    }
}

impl Handler<MsgBurn, BankEvent> for BankModule {
    fn handle(&self, ctx: &Context<BankEvent>, msg: &MsgBurn) -> cosmos_result::Result<()> {
        ctx.emit(BankEvent::Burn {
            from: msg.from.clone(),
            denom: msg.denom.clone(),
            amount: msg.amount.clone(),
        })}
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
    fn reduce(&self, event: &BankEvent, store: &Store) -> cosmos_result::Result<()> {
        match event {
            BankEvent::Send { from, to, denom, amount } => {
                self.balances.safe_sub(store, (from, denom), &amount)?;
                self.balances.add(store, (to, denom), &amount)
            }
            BankEvent::Burn { from, denom, amount } => {
                self.balances.safe_sub(store, (from, denom), &amount)?;
                self.supply.safe_sub(store, denom, &amount)
            },
            BankEvent::Mint { to, denom, amount } => {
                self.balances.add(store, (to, denom), &amount)?;
                self.supply.add(store, denom, &amount)
            }
        }
    }
}
