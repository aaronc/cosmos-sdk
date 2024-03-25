pub struct BytesWriter<'a> {
    buffer: &'a mut [u8],
    i: usize,
}

impl<'a> BytesWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        let i = buffer.len();
        Self {
            buffer,
            i,
        }
    }

    pub fn write(&mut self, len: usize) -> &mut [u8] {
        if len > self.i {
            panic!("buffer overflow");
        }

        let end = self.i;
        let start = end - len;
        self.i = start;
        &mut self.buffer[start..end]
    }

    pub fn written(&self) -> usize {
        self.buffer.len() - self.i
    }

    pub fn result(self) -> &'a [u8] {
        &self.buffer[self.i..]
    }
}