use std::{ops::Index, marker::PhantomData};

use wasmtime::{Linker, Memory, Store};

pub(crate) fn add_mem_to_linker<T>(linker: &mut Linker<T>) {
    
}

pub struct MemAccess<T> {
    offset: u32,
    len: u32,
    mark: PhantomData<T>,
}

impl<T> MemAccess<T> {
    pub fn new(offset: u32, len: u32) -> Self {
        Self {offset, len, mark: PhantomData }
    }

    pub fn read(&self, index: u32) -> u32 {
        0
    }
}