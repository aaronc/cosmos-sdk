use cosmos_message_api::{Address, MessagePacket, StateToken};

pub struct Context {
    packet: MessagePacket,
    backend: ContextBackend,
}

#[derive(Clone)]
pub struct ContextBackend {
    invoke_fn: fn(&mut MessagePacket) -> u32,
    default_layout: core::alloc::Layout,
    alloc: fn(&core::alloc::Layout) -> *mut u8,
    free: fn(*mut u8, &core::alloc::Layout),
}

impl Context {
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
        let mut header= packet.header_mut();
        let self_header = self.packet.header();
        header.context_token = self_header.context_token;
        header.sender_address = self_header.target_address;
        header.state_token = self_header.state_token;
        Context {
            packet,
            backend: self.backend.clone(),
        }
    }

    pub unsafe fn raw_packet(&self) -> &MessagePacket {
        &self.packet
    }

    pub unsafe fn raw_packet_mut(&mut self) -> &mut MessagePacket {
        &mut self.packet
    }

    pub unsafe fn raw_invoke(&mut self) -> u32 {
        (self.backend.invoke_fn)(&mut self.packet)
    }
}