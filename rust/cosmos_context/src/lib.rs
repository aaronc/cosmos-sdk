use core::ops::Deref;
use cosmos_error::Error;
use cosmos_message_api::{Address, Code, MessagePacket, StateToken};
use cosmos_message_api::handler::HostCallbacks;

pub struct Context<'a> {
    packet: MessagePacket,
    backend: &'a ContextBackend<'a>,
}

#[derive(Clone)]
pub struct ContextBackend<'a> {
    host_callbacks: &'a HostCallbacks,
    default_layout: core::alloc::Layout,
    alloc: fn(&core::alloc::Layout) -> *mut u8,
    free: fn(*mut u8, &core::alloc::Layout),
}

impl<'a> Context<'a> {
    pub fn self_address(&self) -> &Address {
        unsafe { &self.packet.header().target_address }
    }
    pub fn sender(&self) -> &Address {
        unsafe { &self.packet.header().sender_address }
    }

    pub fn state_token(&self) -> &StateToken {
        unsafe { &self.packet.header().state_token }
    }

    pub unsafe fn new_context(&self) -> Context {
        self.new_context_with_layout(&self.backend.default_layout)
    }

    pub unsafe fn new_context_with_layout(&self, layout: &core::alloc::Layout) -> Context {
        if layout.size() < 512 {
            panic!("layout size must be at least 512 bytes");
        }
        if layout.align() < 8 {
            panic!("layout alignment must be at least 8 bytes");
        }
        let packet_data = (self.backend.alloc)(layout);
        // TODO is this the best way to zeroize the header
        let packet_header = core::slice::from_raw_parts_mut(packet_data, 512);
        packet_header.fill(0);
        let mut packet = MessagePacket::new(packet_data, layout.size());
        let mut header = packet.header_mut();
        let self_header = self.packet.header();
        header.context_token = self_header.context_token;
        header.sender_address = self_header.target_address;
        header.state_token = self_header.state_token;
        Context {
            packet,
            backend: self.backend,
        }
    }

    pub unsafe fn raw_packet(&self) -> &MessagePacket {
        &self.packet
    }

    pub unsafe fn raw_packet_mut(&mut self) -> &mut MessagePacket {
        &mut self.packet
    }

    pub unsafe fn raw_invoke(&mut self) -> Code {
        (self.backend.host_callbacks.invoke)(&mut self.packet)
    }

    pub fn ok<T, E>() -> Response<'a, T, E> {
        todo!()
    }
}

pub type Response<'a, T, E = ()> = Result<ResponseBody<'a, T>, Error<E>>;

pub struct ResponseBody<'a, T> {
    packet: &'a mut MessagePacket,
}

impl <'a, T> Deref for ResponseBody<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}