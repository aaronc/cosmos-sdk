pub struct BorrowAllocator {

}

impl BorrowAllocator {
    fn allocate(&mut self, size: usize) -> (&mut [u8], AllocError) {
        todo!()
    }
}

struct AllocError;