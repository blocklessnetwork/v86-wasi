#[macro_use]
mod macros;

#[cfg(target_os="macos")]
mod macos;
#[cfg(any(target_os="linux", target_os="macos"))]
mod posix;

#[cfg(target_os="linux")]
mod linux;

#[cfg(target_os="macos")]
pub use macos::*;

#[cfg(target_os="linux")]
pub use linux::*;

#[cfg(any(target_os="linux", target_os="macos"))]
pub use posix::*;
