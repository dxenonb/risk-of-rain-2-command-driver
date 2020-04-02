pub mod robot;

use std::collections::HashMap;
use std::cmp;

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
