use cosmossdk_core::{Address, Code, Context, err, Handler, ok};
use cosmossdk_core::account::CreateAccountHandler;
use cosmossdk_core::AgentId::Account;
use cosmossdk_macros::AccountHandler;
use state_objects::Item;
use state_objects_macros::State;
use crate::example::bank::v1::{CreateEscrow, MsgSend, RefundEscrow, TransferEscrow};

#[derive(AccountHandler)]
#[account_handler(
    create = CreateEscrow,
    services = [
        Handler<RefundEscrow>,
        Handler<TransferEscrow>
])]
pub struct Escrow {
    state: EscrowState,
    bank_msg_client: crate::example::bank::v1::MsgClient,
}

#[derive(State)]
pub struct EscrowState {
    depositor: Item<Vec<u8>>,
    verifier: Item<Vec<u8>>,
    recipient: Item<Vec<u8>>,
}

impl CreateAccountHandler<CreateEscrow> for Escrow {
    fn create(&self, ctx: &mut Context, req: &CreateEscrow) -> cosmossdk_core::Result<()> {
        self.state.depositor.set(ctx, &req.depositor)?;
        self.state.verifier.set(ctx, &req.verifier)?;
        self.state.recipient.set(ctx, &req.recipient)?;
        ok()
    }
}

impl Escrow {
    fn authenticate_verifier<'b>(&self, ctx: &'b Context) -> cosmossdk_core::Result<&'b Address> {
        let Account(acct_address) = ctx.caller_id() else {
            return err!(Code::PermissionDenied);
        };

        if acct_address.as_slice() != self.state.verifier.get(ctx)?.as_slice() {
            return err!(Code::PermissionDenied);
        }

        Ok(acct_address)
    }
}

impl Handler<RefundEscrow> for Escrow {
    fn handle(&self, ctx: &mut Context, req: &RefundEscrow) -> cosmossdk_core::Result<()> {
        let acct_address = self.authenticate_verifier(ctx)?;

        self.bank_msg_client.send(ctx, &MsgSend {
            from: acct_address.into(),
            to: self.state.depositor.get(ctx)?.into(),
            denom: req.denom.clone(),
            amount: req.amount.clone(),
        })?;

        ok()
    }
}

impl Handler<TransferEscrow> for Escrow {
    fn handle(&self, ctx: &mut Context, req: &TransferEscrow) -> cosmossdk_core::Result<()> {
        let acct_address = self.authenticate_verifier(ctx)?;

        self.bank_msg_client.send(ctx, &MsgSend {
            from: acct_address.into(),
            to: self.state.recipient.get(ctx)?.into(),
            denom: req.denom.clone(),
            amount: req.amount.clone(),
        })?;

        ok()
    }
}