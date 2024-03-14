use crate::module::Module;

pub trait ModuleBundle {
    fn visit<T: ModuleBundleVisitor>(visitor: &mut T) -> crate::Result<()>;
}

pub trait ModuleBundleVisitor {
    fn visit_module<T: Module + 'static>(&mut self) -> crate::Result<()>;
}
