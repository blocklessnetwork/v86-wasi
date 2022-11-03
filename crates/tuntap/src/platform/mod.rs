#[cfg(target_os="macos")]
mod macos;
mod posix;

#[cfg(target_os="macos")]
pub use macos::*;

pub use posix::*;