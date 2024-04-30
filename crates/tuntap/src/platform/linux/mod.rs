mod dev;
mod sys;
mod epoll;
mod event;

pub use event::{Event, Events};

pub use dev::Tap;