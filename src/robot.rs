use winapi::um::winuser::{
    SetCursorPos,
    SendInput,
    INPUT,
    INPUT_MOUSE,
    MOUSEINPUT,
    MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP,
};

use std::mem;

pub struct MousePos(pub u32, pub u32);

pub trait Robot {
    fn mouse_to(&mut self, pos: MousePos);
    fn click_on(&mut self, pos: MousePos);
}

pub struct WinRobot;

impl WinRobot {
    pub fn new() -> Self {
        WinRobot
    }
}

impl Robot for WinRobot {
    fn mouse_to(&mut self, MousePos(x, y): MousePos) {
        unsafe {
            SetCursorPos(x as _, y as _);
        }
    }

    fn click_on(&mut self, pos: MousePos) {
        self.mouse_to(pos);

        unsafe {
            let mut input;

            input = INPUT {
                type_: INPUT_MOUSE,
                u: std::mem::zeroed(),
            };
            *input.u.mi_mut() = MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_LEFTDOWN,
                time: 0,
                dwExtraInfo: 0,
            };
            SendInput(1, &mut input as *mut _, mem::size_of::<INPUT>() as _);

            *input.u.mi_mut() = MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_LEFTUP,
                time: 0,
                dwExtraInfo: 0,
            };
            SendInput(1, &mut input as *mut _, mem::size_of::<INPUT>() as _);
        }
    }
}
