use winapi::um::xinput::{
    XInputGetState,
    XINPUT_STATE,
    XINPUT_GAMEPAD_A,
    XINPUT_GAMEPAD_X,
    XINPUT_GAMEPAD_Y,
    XINPUT_GAMEPAD_B,
};

pub struct XboxButtons {
    pub a: bool,
    pub x: bool,
    pub y: bool,
    pub b: bool,
}

impl Default for XboxButtons {
    fn default() -> Self {
        XboxButtons {
            a: false,
            x: false,
            y: false,
            b: false,
        }
    }
}

pub fn get_input(index: u32) -> Option<XboxButtons> {
    let mut state: XINPUT_STATE = Default::default();

    let result = unsafe { XInputGetState(index, &mut state) };
    if result != 0 {
        // Controller is not connected
        return None;
    }

    Some(XboxButtons {
        a: state.Gamepad.wButtons & XINPUT_GAMEPAD_A != 0,
        x: state.Gamepad.wButtons & XINPUT_GAMEPAD_X != 0,
        y: state.Gamepad.wButtons & XINPUT_GAMEPAD_Y != 0,
        b: state.Gamepad.wButtons & XINPUT_GAMEPAD_B != 0,
    })
}

pub fn get_just_pressed(index: u32, prev: &mut XboxButtons) -> Option<XboxButtons> {
    let next = get_input(index)?;

    let r = Some(XboxButtons {
        a: next.a && !prev.a,
        x: next.x && !prev.x,
        y: next.y && !prev.y,
        b: next.b && !prev.b,
    });

    *prev = next;

    r
}
