use winapi::um::winuser::{
    SetCursorPos,
    SendInput,
    INPUT,
    INPUT_MOUSE,
    MOUSEINPUT,
    MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MOVE,
};

use std::mem;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct MousePos(pub i32, pub i32);

impl ops::Sub for MousePos {
    type Output = MousePos;

    fn sub(self, rhs: MousePos) -> Self::Output {
        MousePos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub trait Robot {
    fn mouse_to(&mut self, pos: MousePos);
    fn mouse_relative(&mut self, pos: MousePos);
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

    fn mouse_relative(&mut self, pos: MousePos) {
        unsafe {

            let mut input = INPUT {
                type_: INPUT_MOUSE,
                u: std::mem::zeroed(),
            };
            *input.u.mi_mut() = MOUSEINPUT {
                dx: pos.0,
                dy: pos.1,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0,
            };
            SendInput(1, &mut input as *mut _, mem::size_of::<INPUT>() as _);
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
