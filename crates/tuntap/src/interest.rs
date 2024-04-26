#[derive(Clone, Copy)]
pub struct Interest(u8);

const READABLE: u8 = 0b0001;
const WRITEABLE: u8 = 0b0010;

impl Interest {

    pub const READABLE: Self = Interest(READABLE);
    
    pub const WRITEABLE: Self = Interest(WRITEABLE);

    #[inline]
    pub fn add(self, i: Interest) -> Self {
        Self(self.0|i.0)
    }

    #[inline]
    pub fn is_readable(self) -> bool {
        (self.0 & READABLE) != 0
    }

    #[inline]
    pub fn is_writeable(self) -> bool {
        (self.0 & WRITEABLE) != 0
    }

}