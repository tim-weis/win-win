use crate::bindings::windows::win32::{
    menus_and_resources::HACCEL,
    system_services::BOOL,
    windows_and_messaging::{
        DispatchMessageW, GetMessageW, TranslateAcceleratorW, TranslateMessage, HWND, MSG,
    },
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
/// The `accel` argument must be a valid HACCEL handle (though `default()` is valid).
///
/// [`WM_QUIT`]: https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit
/// [`SendMessage`]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage
pub unsafe fn runloop(accel: HACCEL) -> BOOL {
    loop {
        let mut msg = MSG::default();
        let res = GetMessageW(&mut msg, HWND(0), 0, 0);
        if res.0 <= 0 {
            return res;
        }
        if accel.0 == 0 || TranslateAcceleratorW(msg.hwnd, accel, &mut msg) == 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
