use crate::module::Module;

pub trait ModuleBundle {
    fn visit<T: ModuleBundleVisitor>(visitor: &T);
}

pub trait ModuleBundleVisitor {
    fn visit_module<T: Module>(&mut self);
}
