use crate::StoreT;

pub(crate) trait FileBuffer {
    fn get(&mut self, start: usize, len: usize, cb: Box<dyn FnOnce(&StoreT, &[u8])>);

    fn set(&mut self, offset: usize, sl: &[u8], cb: Box<dyn FnOnce(&StoreT)>);

    fn byte_length(&self) -> usize;
}

pub(crate) struct SyncFileBuffer {
    store: StoreT,
    file_data: Vec<u8>,
}

impl SyncFileBuffer {
    pub fn new(store: StoreT, file_data: Vec<u8>) -> Self {
        Self { store, file_data }
    }
}

impl FileBuffer for SyncFileBuffer {
    fn get(&mut self, start: usize, len: usize, cb: Box<dyn FnOnce(&StoreT, &[u8])>) {
        let bs = &self.file_data[start..start + len];
        cb(&self.store, bs);
    }

    fn set(&mut self, offset: usize, sl: &[u8], cb: Box<dyn FnOnce(&StoreT)>) {
        let dest = &mut self.file_data[offset..offset + sl.len()];
        dest.copy_from_slice(sl);
        cb(&self.store);
    }

    fn byte_length(&self) -> usize {
        self.file_data.len()
    }
}
