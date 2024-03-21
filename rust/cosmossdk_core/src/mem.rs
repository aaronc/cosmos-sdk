use core::borrow::Borrow;
use core::ops::Deref;

pub struct Ref<'a, T: 'a + ?Sized> {
    handle: BytesRef<'a>,
    pub data: T,
}

impl<'a, T: 'a + ?Sized> Ref<'a, T> {}

// impl<'a, T: 'a + ?Sized> Borrow<T> for Ref<'a, T>
// where
// {
//     fn borrow(&self) -> &T {
//         &self.data
//     }
// }
//
// impl<'a, T: ?Sized> Deref for Ref<'a, T> {
//     type Target = T;
//
//     fn deref(&self) -> &'a Self::Target {
//         self.data
//     }
// }

impl<'a, T> Ref<'a, T> {
    pub fn cast<U: 'a>(self, u: U) -> Ref<'a, U> {
        Ref {
            handle: self.handle,
            data: u,
        }
    }
}

pub struct BytesRef<'a> {
    ptr: *mut u8,
    len: usize,
    free: fn(*mut u8, usize),
    _phantom: core::marker::PhantomData<&'a *mut u8>,
}

impl<'a> BytesRef<'a> {
    pub fn new(data: &'a [u8], free: fn(*mut u8, usize)) -> BytesRef<'a> {
        let len = core::mem::size_of_val(&data);
        let ptr = Box::into_raw(Box::new(data)) as *mut u8;
        BytesRef {
            ptr,
            len,
            free,
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn cast<U: 'a>(self, u: U) -> Ref<'a, U> {
        Ref {
            handle: self,
            data: u,
        }
    }
}

impl<'a> Drop for BytesRef<'a> {
    fn drop(&mut self) {
        (self.free)(self.ptr, self.len)
    }
}