use crate::module::Module;

pub trait ModuleBundle {
    fn visit<T: ModuleBundleVisitor>(visitor: &mut T);
}

pub trait ModuleBundleVisitor {
    fn visit_module<T: Module>(&mut self);
}
