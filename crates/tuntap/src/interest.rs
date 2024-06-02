#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Interest(u8);

const READABLE: u8 = 0b0001;
const WRITABLE: u8 = 0b0010;

impl Interest {

    pub const READABLE: Self = Interest(READABLE);
    
    pub const WRITABLE: Self = Interest(WRITABLE);

    pub const RDWR: Self = Interest(WRITABLE|READABLE);

    #[inline]
    pub fn add(self, i: Interest) -> Self {
        Self(self.0|i.0)
    }

    #[inline]
    pub fn is_readable(self) -> bool {
        (self.0 & READABLE) != 0
    }

    #[inline]
    pub fn is_writable(self) -> bool {
        (self.0 & WRITABLE) != 0
    }

}