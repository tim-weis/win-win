use windows::runtime::Handle;
use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateAcceleratorW, TranslateMessage, HACCEL,
};

/// A basic winapi runloop.
///
/// This runloop blocks on receiving messages and dispatches them to windows. It exits
/// on [`WM_QUIT`].
///
/// It is tempting to try to get fancier with runloops, for example waiting on semaphores
/// or other events, but these strategies are risky. In particular, the main runloop is not
/// always in control; when the window is being resized, or a modal dialog is open, then
/// that runloop takes precedence. For waking the UI thread from another thread,
/// [`SendMessage`] is probably the best bet.
///
/// # Safety
///
/// The `accel` argument must be a valid HACCEL handle (though `null_mut()` is valid).
///
/// [`WM_QUIT`]: https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit
/// [`SendMessage`]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage
pub unsafe fn runloop(accel: HACCEL) -> BOOL {
    loop {
        let mut msg = Default::default();
        let res = GetMessageW(&mut msg, None, 0, 0);
        if res.0 <= 0 {
            return res;
        }

        let msg = msg;
        if accel.is_invalid() || TranslateAcceleratorW(msg.hwnd, accel, &msg) == 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
