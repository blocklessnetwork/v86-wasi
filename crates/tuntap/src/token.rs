
/// Token is wrap the usize used in register
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token(pub usize);

impl From<usize> for Token {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
