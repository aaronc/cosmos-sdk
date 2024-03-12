use cosmossdk_core::bundle::ModuleBundleVisitor;

mod bank;
mod escrow;

include!("types/_includes.rs");
static FILE_DESCRIPTOR_BYTES: &[u8] = include_bytes!("types/file_descriptor_set.bin");

struct ModuleBundle;

impl cosmossdk_core::bundle::ModuleBundle for ModuleBundle {
    fn visit<T: ModuleBundleVisitor>(visitor: &mut T) {
        visitor.visit_module::<bank::Bank>();
    }
}

