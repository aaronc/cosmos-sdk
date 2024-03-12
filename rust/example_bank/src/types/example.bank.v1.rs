#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BankModule {}
impl ::prost::Name for BankModule {
    const NAME: &'static str = "BankModule";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    #[prost(bytes = "vec", tag = "1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MsgSend {
    const NAME: &'static str = "MsgSend";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
impl ::prost::Name for MsgSendResponse {
    const NAME: &'static str = "MsgSendResponse";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalance {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBalance {
    const NAME: &'static str = "QueryBalance";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub balance: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for QueryBalanceResponse {
    const NAME: &'static str = "QueryBalanceResponse";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalSend {
    #[prost(bytes = "vec", tag = "1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for InternalSend {
    const NAME: &'static str = "InternalSend";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalSendLazy {
    #[prost(bytes = "vec", tag = "1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for InternalSendLazy {
    const NAME: &'static str = "InternalSendLazy";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
pub struct MsgClient<'a> {
    conn: ::cosmossdk_core::routing::ClientConnection<'a>,
}
impl<'a> MsgClient<'a> {
    pub fn send(
        &self,
        ctx: &mut ::cosmossdk_core::Context,
        req: &MsgSend,
    ) -> ::cosmossdk_core::Result<MsgSendResponse> {
        todo!()
    }
}
impl<'a> ::cosmossdk_core::routing::Client<'a> for MsgClient<'a> {
    fn new(conn: ::cosmossdk_core::routing::ClientConnection<'a>) -> Self {
        Self { conn }
    }
    fn describe(
        helper: &mut dyn ::cosmossdk_core::routing::ClientDescriptorHelper,
    ) -> ::cosmossdk_core::routing::ClientDescriptor {
        ::cosmossdk_core::routing::ClientDescriptor::ServiceClient(
            "example.bank.v1.Msg".to_string(),
        )
    }
}
impl<'a> ::cosmossdk_core::encoding::prost::ProstClient<'a> for MsgClient<'a> {}
pub trait MsgServer {
    fn send(
        &self,
        ctx: &mut ::cosmossdk_core::Context,
        req: &MsgSend,
    ) -> ::cosmossdk_core::Result<MsgSendResponse>;
}
pub struct QueryClient<'a> {
    conn: ::cosmossdk_core::routing::ClientConnection<'a>,
}
impl<'a> QueryClient<'a> {
    pub fn balance(
        &self,
        ctx: &mut ::cosmossdk_core::Context,
        req: &QueryBalance,
    ) -> ::cosmossdk_core::Result<QueryBalanceResponse> {
        todo!()
    }
}
impl<'a> ::cosmossdk_core::routing::Client<'a> for QueryClient<'a> {
    fn new(conn: ::cosmossdk_core::routing::ClientConnection<'a>) -> Self {
        Self { conn }
    }
    fn describe(
        helper: &mut dyn ::cosmossdk_core::routing::ClientDescriptorHelper,
    ) -> ::cosmossdk_core::routing::ClientDescriptor {
        ::cosmossdk_core::routing::ClientDescriptor::ServiceClient(
            "example.bank.v1.Query".to_string(),
        )
    }
}
impl<'a> ::cosmossdk_core::encoding::prost::ProstClient<'a> for QueryClient<'a> {}
pub trait QueryServer {
    fn balance(
        &self,
        ctx: &mut ::cosmossdk_core::Context,
        req: &QueryBalance,
    ) -> ::cosmossdk_core::Result<QueryBalanceResponse>;
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEscrow {
    #[prost(bytes = "vec", tag = "1")]
    pub depositor: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub verifier: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CreateEscrow {
    const NAME: &'static str = "CreateEscrow";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefundEscrow {
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for RefundEscrow {
    const NAME: &'static str = "RefundEscrow";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEscrow {
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for TransferEscrow {
    const NAME: &'static str = "TransferEscrow";
    const PACKAGE: &'static str = "example.bank.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("example.bank.v1.{}", Self::NAME)
    }
}
