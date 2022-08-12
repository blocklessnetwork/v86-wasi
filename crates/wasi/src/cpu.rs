
// Resources:
// https://pdos.csail.mit.edu/6.828/2006/readings/i386/toc.htm
// https://www-ssl.intel.com/content/www/us/en/processors/architectures-software-developer-manuals.html
// http://ref.x86asm.net/geek32.html

use wasmtime::Memory;
use crate::io::IO;

pub(crate) struct CPU {
    memory: Memory,
    
    io: IO,
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        CPU { memory, io: IO::new() }
    }
}