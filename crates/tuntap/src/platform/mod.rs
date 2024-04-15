#[cfg(target_os="macos")]
mod macos;
mod posix;

#[cfg(target_os="macos")]
pub use macos::*;

#[cfg(any(target_os="linux", target_os="macos"))]
pub use posix::*;