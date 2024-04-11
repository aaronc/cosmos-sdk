pub trait Param<'a> {
    fn bytes(&self) -> alloc::borrow::Cow<'a, [u8]>;
    fn set_bytes(&mut self, data: alloc::borrow::Cow<'a, [u8]>);
}