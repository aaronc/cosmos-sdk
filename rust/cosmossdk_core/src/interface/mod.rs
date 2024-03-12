use crate::routing::ModuleServiceResolver;

pub trait InterfaceImplementation: ModuleServiceResolver {
    type Parameter;
}