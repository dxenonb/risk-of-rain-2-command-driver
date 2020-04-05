use image::{
    Pixel,
    RgbImage,
    Rgb,
};

use std::collections::HashMap;
use std::cmp;
use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use std::convert::TryInto;
use std::borrow::Cow;
use std::fmt;

pub mod robot;
pub mod xinput;

pub struct ItemPos(pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemClass {
    White,
    Green,
    Red,
    Lunar,
    UseItem,
    BossItem,
}

pub struct AnalysisOptions {
    pub left: i32,
    /// Should be the right edge of the spot to start checking
    ///
    /// You will want to factor in the value of `span`.
    pub right: i32,
    pub y: i32,
    pub span: i32,
    /// How different the right can be from the left, as a percentage
    ///
    /// E.g. 0.1 means the right can be +/-10% from the right side
    pub permitted_deviation: f32,
    pub max_distance: i32,
}

type ColorCheck<'a, T> = &'a [((i32, i32, i32), T)];

pub struct Win32Color(u32);
pub struct Win32Bitmap(pub Vec<u8>, (usize, usize));

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

pub trait ColorSrc {
    type C: Color;

    fn get_pixel(&self, x: i32, y: i32) -> Self::C;
}

pub trait Color {
    fn get_red(&self) -> u8;
    fn get_blue(&self) -> u8;
    fn get_green(&self) -> u8;
}

impl ColorSrc for Win32Bitmap {
    type C = Win32Color;

    fn get_pixel(&self, x: i32, y: i32) -> Self::C {
        // buffer is layed out from top to bottom, left to right, 32bits per pixel
        // row alignment and endianness is unknown - assume 0 and little endian for now
        let (width, _) = self.1;
        let index = 4 * ((y as usize) * width + (x as usize));
        let bytes: [u8; 4] = self.0[index..index+4].try_into().unwrap();
        let pixel = u32::from_le_bytes(bytes);
        Win32Color(pixel)
    }
}

impl Color for Win32Color {
    fn get_red(&self) -> u8 {
        ((self.0 & 0x00FF00FF) >> 16) as u8
    }

    fn get_green(&self) -> u8 {
        ((self.0 & 0x0000FF00) >> 8) as u8
    }

    fn get_blue(&self) -> u8 {
        ((self.0 & 0x000000FF) >> 0) as u8
    }
}

impl ColorSrc for &RgbImage
{
    type C = Rgb<u8>;

    fn get_pixel(&self, x: i32, y: i32) -> Self::C {
        *RgbImage::get_pixel(self, x as _, y as _)
    }
}

impl<P: Pixel<Subpixel=u8>> Color for P {
    fn get_red(&self) -> u8 {
        self.to_rgb().0[0]
    }

    fn get_green(&self) -> u8 {
        self.to_rgb().0[1]
    }

    fn get_blue(&self) -> u8 {
        self.to_rgb().0[2]
    }
}

pub fn analyze_screencap<T: Clone + fmt::Debug>(
    opts: &AnalysisOptions,
    checking: ColorCheck<T>,
    log: bool,
) -> Result<Option<T>, Cow<'static, str>> {
    let win32bitmap = screencap()?;
    Ok(analyze_bitmap(opts, checking, &win32bitmap, log))
}

pub fn analyze_bitmap<T: Clone + fmt::Debug, C: ColorSrc>(
    opts: &AnalysisOptions,
    checking: ColorCheck<T>,
    bitmap: &C,
    log: bool,
) -> Option<T> {
    let mut min = None;

    // debug_color_src("analyzed.bmp", bitmap, opts.left, opts.y - 1, opts.right - opts.left + opts.span, 3);

    for (color, result) in checking {
        let left_dist = average_distance2(
            bitmap,
            &color,
            opts.left,
            opts.y,
            opts.span,
        );
        let right_dist = average_distance2(
            bitmap,
            &color,
            opts.right,
            opts.y,
            opts.span,
        );
        if log {
            println!("For {:?}, got dists: {}, {}", result, left_dist, right_dist);
        }
        let diff = (1.0 - left_dist as f32 / (right_dist as f32)).abs();
        if diff > opts.permitted_deviation || left_dist > opts.max_distance {
            continue;
        }
        let dist = left_dist;
        match min {
            None => {
                min = Some((dist, result))
            },
            Some((prev_dist, _)) => {
                if dist < prev_dist {
                    min = Some((dist, result));
                }
            }
        }
    }

    min.map(|(_, val)| val.clone())
}

pub fn average_distance2<C: ColorSrc>(
    bitmap: &C,
    target_color: &(i32, i32, i32),
    mut x: i32,
    y: i32,
    span: i32,
) -> i32 {
    let mut sum = 0;
    for _ in 0..span {
        let color = bitmap.get_pixel(x, y);
        sum += color_distance2(color, target_color);
        x += 1;
    }

    sum / span
}

pub fn color_distance2<C: Color>(c: C, (r, g, b): &(i32, i32, i32)) -> i32 {
    let src_red = c.get_red() as i32;
    let src_green = c.get_green() as i32;
    let src_blue = c.get_blue() as i32;

    (src_red - r).pow(2)
        + (src_green - g).pow(2)
        + (src_blue - b).pow(2)
}

/// Use this to quickly print out images
///
/// Very handy for debugging.
#[allow(unused)]
fn debug_color_src<C: ColorSrc>(path: &str, c: &C, left: i32, top: i32, width: i32, height: i32) {
    let buffer_size = (3 * width * height) as usize;
    let buffer = vec![0; buffer_size];
    let mut img = RgbImage::from_raw(width as _, height as _, buffer).unwrap();

    for yi in 0..height {
        for xi in 0..width {
            let output = img.get_pixel_mut(xi as _, yi as _);
            let input = c.get_pixel(xi + left, yi + top);
            output.0[0] = input.get_red();
            output.0[1] = input.get_green();
            output.0[2] = input.get_blue();
        }
    }

    img.save(path).expect("failed to write debug image");
}

fn screencap() -> Result<Win32Bitmap, Cow<'static, str>> {
    // Adapted from:
    // https://stackoverflow.com/questions/3291167/how-can-i-take-a-screenshot-in-a-windows-application
    use winapi::um::winuser::ReleaseDC;
    use winapi::um::wingdi::{
        DeleteObject,
        DeleteDC,
    };

    unsafe {
        let mut dc_screen = ptr::null_mut();
        let mut dc_target = ptr::null_mut();
        let mut bmp_target = ptr::null_mut();

        let result = screencap_inner(&mut dc_screen, &mut dc_target, &mut bmp_target);

        DeleteObject(bmp_target as *mut _);
        DeleteDC(dc_target);
        ReleaseDC(ptr::null_mut(), dc_screen);

        result
    }
}

fn screencap_inner(
    dc_screen: &mut winapi::shared::windef::HDC,
    dc_target: &mut winapi::shared::windef::HDC,
    bmp_target: &mut winapi::shared::windef::HBITMAP,
) -> Result<Win32Bitmap, Cow<'static, str>> {
    use winapi::shared::minwindef::WORD;
    use winapi::um::winuser::{
        GetDC,
        GetSystemMetrics,
        SM_XVIRTUALSCREEN,
        SM_YVIRTUALSCREEN,
        SM_CXVIRTUALSCREEN,
        SM_CYVIRTUALSCREEN,
    };
    use winapi::um::wingdi::{
        CreateCompatibleDC,
        CreateCompatibleBitmap,
        GetObjectA,
        SelectObject,
        BitBlt,
        GetDIBits,
        SRCCOPY,
        CAPTUREBLT,
        BITMAP,
        BI_RGB,
        BITMAPINFO,
        BITMAPINFOHEADER,
        DIB_RGB_COLORS,
    };

    unsafe {
        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let cx = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let cy = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        *dc_screen = GetDC(ptr::null_mut());
        let dc_screen = *dc_screen;
        if dc_screen == ptr::null_mut() {
            return Err(Cow::from("failed to get screen DC"));
        }

        *dc_target = CreateCompatibleDC(dc_screen);
        let dc_target = *dc_target;
        if dc_target == ptr::null_mut() {
            return Err(Cow::from("failed to create target DC"));
        }

        *bmp_target = CreateCompatibleBitmap(dc_screen, cx, cy);
        let bmp_target = *bmp_target;
        if bmp_target == ptr::null_mut() {
            return Err(Cow::from("bitmap creation failed"));
        }

        let old_bmp = SelectObject(dc_target, bmp_target as *mut _);

        if BitBlt(dc_target, 0, 0, cx, cy, dc_screen, x, y, SRCCOPY | CAPTUREBLT) == 0 {
            return Err(Cow::from("Bit blitting failed"));
        }

        // extract bitmap buffer (using winapi `GetPixel` was not working)
        let mut buffer: MaybeUninit<BITMAP> = MaybeUninit::uninit();
        let bitmap_size = mem::size_of::<BITMAP>() as _;
        let result = GetObjectA(bmp_target as *mut _, bitmap_size, buffer.as_mut_ptr() as *mut _);
        if result == 0 || result != bitmap_size {
            return Err(Cow::from("failed to get object"));
        }
        let bmp = buffer.assume_init();
        let clr_bits: WORD = (bmp.bmPlanes * bmp.bmBitsPixel) as _;
        if clr_bits != 32 {
            return Err(Cow::from(format!("expected 32 bit image, got {}", clr_bits)));
        }
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as _,
                biWidth: bmp.bmWidth,
                biHeight: bmp.bmHeight,
                biPlanes: bmp.bmPlanes,
                biBitCount: bmp.bmBitsPixel,
                biCompression: BI_RGB,
                ..Default::default()
            },
            bmiColors: Default::default(),
        };
        let size_image = (((bmp.bmWidth * (clr_bits as i32) + 31) & !31) / 8 * bmp.bmHeight) as usize;
        bmi.bmiHeader.biSizeImage = size_image as _;
        bmi.bmiHeader.biClrImportant = 0;

        let mut bits: Vec<u8> = vec![0; size_image];
        let r = GetDIBits(
            dc_screen,
            bmp_target,
            0,
            cy as _,
            bits.as_mut_ptr() as *mut _,
            &mut bmi as *mut _,
            DIB_RGB_COLORS,
        );
        if r != cy {
            return Err(Cow::from(format!("GetDIBits is: {}, expected # of scanlines: {}", r, cy)));
        }

        SelectObject(dc_target, old_bmp);

        Ok(Win32Bitmap(bits, (cx as _, cy as _)))
    }
}
