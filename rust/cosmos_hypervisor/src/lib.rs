use std::collections::HashMap;
use cosmos_message_api::handler::Handler;
use cosmos_message_api::{Address, MessagePacket};
use cosmos_vm_api::{AuthorizationHandler, StateHandler, VMFactory, VM};

pub struct Hypervisor {
    // initialized at startup
    vms: HashMap<String, Box<dyn VM>>, // TODO: can we use an integer ID to avoid a hash map lookup?
    modules: HashMap<String, Box<dyn Handler>>,
    module_message_map: HashMap<String, Box<dyn Handler>>,
    state_handler: Box<dyn StateHandler>,
    authorization_handler: Option<Box<dyn AuthorizationHandler>>,

    // stateful:
    account_handler_ids: state_objects::Map<Address, String>, // TODO integer handler ID?
}


pub struct InitParams {
}

impl Hypervisor {
    pub fn new(state_handler: Box<dyn StateHandler>, vms: HashMap<String, Box<dyn VMFactory>>, authorization_handler: Option<Box<dyn AuthorizationHandler>>) -> Self {
        Self {
            vms,
            state_handler,
            authorization_handler,
            modules: HashMap::new(),
            module_message_map: HashMap::new(),
            account_handler_ids: todo!(),
        }

    }

    pub fn init_module(&mut self, handler_id: &str, init_data: &[u8]) {
        todo!()
    }

    pub fn invoke(&mut self, message: &mut MessagePacket) {
        todo!()
    }
}