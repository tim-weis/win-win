//! This module serves to consolidate changes that need to be applied to use the
//! *windows-rs* crate in its current state.
//!
//! Both the *windows-rs* crate as well as the Win32Metadata project that provides the
//! source for the code generation are still in preview. Consequently, the set of symbols
//! and their respective paths are in flux, and occasionally disappear.

#![allow(dead_code)]

use crate::bindings::windows::win32::windows_and_messaging::HWND;

// GetWindowsLongPtr isn't currently included in the Win32Metadata (see
// [SetWindowLongPtr/GetWindowLongPtr](https://github.com/microsoft/win32metadata/issues/142)).
//
// For 32-bit architectures, the `Ptr`-variants aren't exported, and we need to use the
// plain versions instead.
//
// The workaround here follows the C header approach that maps the `Ptr`-variants to the
// plain versions in the preprocessor. The following is doing the same mapping.
//
// Only the `Ptr`-variants are exposed from here on.
#[cfg(target_arch = "x86_64")]
#[link(name = "user32")]
extern "system" {
    pub(crate) fn SetWindowLongPtrW(window: HWND, index: i32, value: isize) -> isize;
    pub(crate) fn GetWindowLongPtrW(window: HWND, index: i32) -> isize;
}

#[cfg(target_arch = "x86")]
#[link(name = "user32")]
extern "system" {
    fn SetWindowLongW(window: HWND, index: i32, value: isize) -> isize;
    fn GetWindowLongW(window: HWND, index: i32) -> isize;
}
#[cfg(target_arch = "x86")]
pub(crate) fn SetWindowLongPtrW(window: HWND, index: i32, value: isize) -> isize {
    SetWindowLongW(window, index, value)
}
#[cfg(target_arch = "x86")]
pub(crate) fn GetWindowLongPtrW(window: HWND, index: i32) -> isize {
    GetWindowLongW(window, index)
}

// Make available the most common index constants as well
pub(crate) const GWL_EXSTYLE: i32 = -20;
pub(crate) const GWLP_HINSTANCE: i32 = -6;
pub(crate) const GWLP_HWNDPARENT: i32 = -8;
pub(crate) const GWLP_ID: i32 = -12;
pub(crate) const GWL_STYLE: i32 = -16;
pub(crate) const GWLP_USERDATA: i32 = -21;
pub(crate) const GWLP_WNDPROC: i32 = -4;

// [CW_USEDEFAULT has disappeared (regression)](https://github.com/microsoft/win32metadata/issues/353)
// This has been fixed in the Win32Metadata already, but other regressions in preview.68
// prevent *windows-rs* from picking that fix up at this time.
pub const CW_USEDEFAULT: i32 = 0x8000_0000_u32 as i32;
