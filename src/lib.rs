pub mod robot;

use winapi::shared::windef::{COLORREF, HBITMAP};

use std::collections::HashMap;
use std::cmp;
use std::ptr;

pub struct ItemPos(pub u32, pub u32);

trait ItemEngine {
    fn select_item(&mut self, action: &Ror2Action) -> ItemPos;
}

enum Ror2Action {
    SelectWhite,
    SelectGreen,
    SelectRed,
    SelectLunar,
    SelectUseItem,
    SelectBossItem,
}

trait ActionDetector {
    fn read_screen(&mut self) -> Ror2Action;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemClass {
    White,
    Green,
    Red,
    Lunar,
    UseItem,
    BossItem,
}

/// Describes the ROR2 command chest UI and user's screen
///
/// We assume the placement of ROR2 does not change and game is maximized.
pub struct ScreenInfo {
    pub item_icon_size: u32,
    pub item_icon_margin: u32,
    pub screen_size: (u32, u32),
    pub grid_size: HashMap<ItemClass, (u32, u32)>,
}

pub fn item_to_screen_pos(
    screen: &ScreenInfo,
    class: &ItemClass,
    ItemPos(mut x, mut y): ItemPos
) -> robot::MousePos {
    let icon = screen.item_icon_size;
    let half_icon = icon / 2;
    let margin = screen.item_icon_margin;

    let (grid_width, grid_height) = screen.grid_size.get(class).unwrap();
    let grid_width = grid_width * icon + (grid_width - 1) * margin;
    let grid_height = grid_height * icon + (grid_height - 1) * margin;
    let offset = (grid_width / 2, grid_height / 2);

    let (screen_width, screen_height) = screen.screen_size;
    let min = (screen_width / 2 - offset.0, screen_height / 2 - offset.1);

    x = half_icon + min.0 + (x * icon + (cmp::max(x as i32 - 1, 0) as u32) * margin);
    y = half_icon + min.1 + (y * icon + (cmp::max(y as i32 - 1, 0) as u32) * margin);

    robot::MousePos(x as _, y as _)
}

pub fn screen_cap() {
    use winapi::um::winuser::{
        GetDC,
        GetSystemMetrics,
        ReleaseDC,
        SM_XVIRTUALSCREEN,
        SM_YVIRTUALSCREEN,
        SM_CXVIRTUALSCREEN,
        SM_CYVIRTUALSCREEN,
    };
    use winapi::um::wingdi::{
        CreateCompatibleDC,
        CreateCompatibleBitmap,
        SelectObject,
        BitBlt,
        DeleteDC,
        DeleteObject,
        SRCCOPY,
        CAPTUREBLT,
    };
    // Adapted from:
    // https://stackoverflow.com/questions/3291167/how-can-i-take-a-screenshot-in-a-windows-application

    unsafe {
        // get screen info
        let x  = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y  = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let cx = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let cy = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        // get a screen cap, save to bitmap
        let dc_screen = GetDC(ptr::null_mut());
        let dc_target = CreateCompatibleDC(dc_screen);
        let bmp_target = CreateCompatibleBitmap(dc_screen, cx, cy);
        if bmp_target == ptr::null_mut() {
            panic!("Bitmap creation failed");
        }
        let old_bmp = SelectObject(dc_target, bmp_target as *mut _);
        if BitBlt(dc_target, 0, 0, cx, cy, dc_screen, x, y, SRCCOPY | CAPTUREBLT) == 0 {
            panic!("Bit blitting failed");
        }

        // process the bitmap ////
        {
            // get avg distance from target color across left side
            let green = (118, 237, 34);
            let avg = average_distance2(bmp_target, &green, 671, cy/2, 10);
            println!("average: {:?}", avg);
            // across the right side
            // find the minimum
            // verify they are similar
        }
        
        SelectObject(dc_target, old_bmp);
        DeleteDC(dc_target);
        ReleaseDC(ptr::null_mut(), dc_screen);

        // free the bitmap object
        DeleteObject(bmp_target as *mut _);
    }
}

unsafe fn average_distance2(
    bitmap: HBITMAP,
    target_color: &(i32, i32, i32),
    mut x: i32,
    y: i32,
    span: i32,
) -> i32 {
    use winapi::um::wingdi::{GetPixel, CLR_INVALID};

    let mut sum = 0;
    for i in 0..span {
        let color = GetPixel(bitmap as *mut _, x, y);
        if color == CLR_INVALID {
            panic!("Got invalid color for {}th pixel of span ({}, {}) \
                - position must be out of bounds!", i, x, y);
        }
        sum += color_distance2(color, target_color);
        x += 1;
    }

    sum / span
}

fn color_distance2(color: COLORREF, (r, g, b): &(i32, i32, i32)) -> i32 {
    use winapi::um::wingdi::{GetRValue, GetGValue, GetBValue};

    let src_red = GetRValue(color) as i32;
    let src_green = GetGValue(color) as i32;
    let src_blue = GetBValue(color) as i32;

    (src_red - r).pow(2)
        + (src_green - g).pow(2)
        + (src_blue - b).pow(2)
}
