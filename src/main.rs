use ror2_command::robot;
use ror2_command::robot::{Robot, MousePos};
use ror2_command::{
    ItemPos,
    ItemClass,
    ScreenInfo,
    item_to_screen_pos,
    screen_cap,
};

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), ()> {
    let mut robot = robot::WinRobot::new();

    let mut grid_size = HashMap::new();
    grid_size.insert(ItemClass::White, (5, 5));
    grid_size.insert(ItemClass::Red, (5, 4));

    let screen = ScreenInfo {
        item_icon_size: 76,
        item_icon_margin: 6,
        screen_size: (1920, 1080),
        grid_size,
    };

    // debug_mouse(&mut robot, &ItemClass::White, &screen);
    screen_cap();

    Ok(())
}

fn debug_mouse<R: Robot>(robot: &mut R, class: &ItemClass, screen: &ScreenInfo) {
    let (width, height) = screen.grid_size.get(class).unwrap();

    for y in 0..*height {
        for x in 0..*width {
            let pos = item_to_screen_pos(screen, class, ItemPos(x, y));
            robot.mouse_to(pos);
            sleep(Duration::from_millis(500));
        }
    }
}
