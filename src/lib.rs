pub mod hook_error;
pub mod keyboard_button;

extern crate winapi;

use crate::hook_error::HookInstallationError;
use crate::keyboard_button::KeyboardButton;
use std::sync::{Arc, Mutex, Once};
use std::{mem, ptr};
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::um::winuser::{
    CallNextHookEx, SetWindowsHookExW, HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_SYSKEYDOWN,
};

type Callback = dyn FnMut(KeyboardButton) -> () + Send + Sync;
type CallbackArc = Arc<Mutex<Callback>>;

static CALLBACK_INIT: Once = Once::new();
static mut GLOBAL_CALLBACK: Option<CallbackArc> = None;

extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let p: *const KBDLLHOOKSTRUCT = unsafe { mem::transmute(l_param) };
    if n_code == HC_ACTION && (w_param == WM_SYSKEYDOWN as usize || w_param == WM_KEYDOWN as usize)
    {
        let button = unsafe { (*p).vkCode as u32 };
        if let Some(callback_mutex) = unsafe { &GLOBAL_CALLBACK } {
            let mut callback = callback_mutex.lock().unwrap();
            callback(KeyboardButton::from_u32(button));
        }

        unsafe { CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param) }
    } else {
        unsafe { CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param) }
    }
}

pub fn install_hook(
    callback: impl FnMut(KeyboardButton) -> () + Send + Sync + 'static,
) -> Result<(), HookInstallationError> {
    let h_instance: HINSTANCE = ptr::null_mut();
    let thread_id: DWORD = 0;

    CALLBACK_INIT.call_once(|| unsafe {
        GLOBAL_CALLBACK = Some(Arc::new(Mutex::new(callback)));
    });

    let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), h_instance, thread_id) };

    if hook.is_null() {
        Err(HookInstallationError::new("Failed to install hook"))
    } else {
        Ok(())
    }
    // UnhookWindowsHookEx(hook);
}
