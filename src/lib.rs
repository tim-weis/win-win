//! Window creation for Windows.

mod bindings {
    ::windows::include_bindings!();
}

mod error;
#[cfg(feature = "kb")]
mod keyboard;
mod runloop;
mod window;

pub use error::Error;
pub use runloop::runloop;
pub use window::{WindowBuilder, WindowClass, WindowClassBuilder, WindowProc};

#[cfg(feature = "kb")]
pub use keyboard::{key_to_vk, KeyboardState};

mod win32impl;
pub use win32impl::CW_USEDEFAULT;
