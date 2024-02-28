pub mod hook_error;
pub mod keyboard_button;

use crate::hook_error::HookInstallationError;
use crate::keyboard_button::KeyboardButton;
use std::sync::{Arc, Mutex, Once};
use std::{mem, ptr};
pub use winapi::shared::minwindef::{DWORD, HINSTANCE};
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::winuser::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, KBDLLHOOKSTRUCT,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

type Callback = dyn FnMut(KeyboardButton) -> () + Send + Sync;
type CallbackArc = Arc<Mutex<Callback>>;

static mut CALLBACK_INIT: Once = Once::new();
static mut GLOBAL_CALLBACK: Option<CallbackArc> = None;

pub struct Hook {
    hook: HHOOK,
}

extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let p: *const KBDLLHOOKSTRUCT = unsafe { mem::transmute(l_param) };
    if n_code == HC_ACTION && (w_param == WM_SYSKEYDOWN as usize || w_param == WM_KEYDOWN as usize)
    {
        let button = unsafe { (*p).vkCode as u32 };
        if let Some(callback_mutex) = unsafe { &GLOBAL_CALLBACK } {
            let mut callback = callback_mutex.lock().unwrap();
            callback(KeyboardButton::from_u32(button));
        }
    }
    unsafe { CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param) }
}

impl Hook {
    pub fn new() -> Self {
        Hook {
            hook: ptr::null_mut(),
        }
    }
    pub fn install_hook(
        &mut self,
        callback: impl FnMut(KeyboardButton) -> () + Send + Sync + 'static,
        h_instance: Option<HINSTANCE>,
        thread_id: Option<DWORD>,
    ) -> Result<(), HookInstallationError> {
        let h_instance: HINSTANCE = h_instance.unwrap_or(ptr::null_mut());
        let thread_id: DWORD = thread_id.unwrap_or(0);
        unsafe {
            if CALLBACK_INIT.is_completed() {
                return Err(HookInstallationError::new(
                    "Hook already installed somewhere. Only one instance is allowed",
                ));
            }

            CALLBACK_INIT.call_once(|| {
                GLOBAL_CALLBACK = Some(Arc::new(Mutex::new(callback)));
            });
        }

        self.hook =
            unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), h_instance, thread_id) };

        if self.hook.is_null() {
            Err(HookInstallationError::new("Failed to install hook"))
        } else {
            Ok(())
        }
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        unsafe {
            UnhookWindowsHookEx(self.hook);
            CALLBACK_INIT = Once::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::keyboard_button::KeyboardButton;
    use crate::Hook;
    use enigo::{Enigo, Key, KeyboardControllable};
    use std::time::Duration;

    #[test]
    fn test_install_hook() {
        let mut hook = Hook::new();
        hook.install_hook(move |_button| {}, None, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "Hook already installed somewhere. Only one instance is allowed")]
    fn test_install_hook_two_times() {
        let mut hook1 = Hook::new();
        hook1.install_hook(move |_button| {}, None, None).unwrap();
        let mut hook2 = Hook::new();
        hook2.install_hook(move |_button| {}, None, None).unwrap();
    }

    #[test]
    fn test_press_button() {
        let (tx, rx) = std::sync::mpsc::sync_channel::<KeyboardButton>(100);
        let mut hook = Hook::new();
        hook.install_hook(
            move |button| {
                println!("button: {:?}", button);
                tx.send(button).unwrap();
            },
            None,
            None,
        )
        .unwrap();

        let mut enigo = Enigo::new();
        enigo.key_click(Key::Shift);

        let pressed_button = rx.recv_timeout(Duration::from_secs(10)).unwrap();

        assert_eq!(pressed_button, KeyboardButton::LShift);
    }
}
