fn main() {
    ::windows::build!(
        windows::win32::com::HRESULT,
        windows::win32::gdi::HBRUSH,
        windows::win32::menus_and_resources::{HACCEL, HICON, HCURSOR, HMENU},
        windows::win32::system_services::{BOOL, LRESULT, HINSTANCE, PWSTR, WINDOWSCLASS_STYLES},
        windows::win32::windows_and_messaging::{
            GetMessageW, MSG, HWND, TranslateAcceleratorW, TranslateMessage, DispatchMessageW,
            WPARAM, LPARAM, RegisterClassExW, WNDCLASSEXW, CreateWindowExW, WINDOWS_EX_STYLE,
            WINDOWS_STYLE, WM_CREATE, WM_NCDESTROY, CREATESTRUCTW, DefWindowProcW
        },
    )
}
