use ror2_command::robot;
use ror2_command::robot::{Robot};
use ror2_command::{
    AnalysisOptions,
    ItemPos,
    ItemClass,
    ScreenInfo,
    item_to_screen_pos,
    analyze_screencap,
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

    let opts = AnalysisOptions {
        left: 672,
        right: 1244,
        y: 1080 / 2,
        span: 4,
        permitted_deviation: 0.03,
        max_distance: 40_000,
    };
    let checking = &[
        ((242, 246, 232), ItemClass::White),
        ((118, 237, 34), ItemClass::Green),
        ((212, 83, 54), ItemClass::Red),
    ];

    let mut xbox_buttons = Default::default();
    loop {
        let pressed = ror2_command::xinput::get_just_pressed(0, &mut xbox_buttons)
            .expect("controller was disconnected");
        if pressed.x {
            let result = analyze_screencap(&opts, checking, false);
            match result {
                Err(err) => {
                    println!("analysis ended with error: {}", err);
                },
                Ok(t) => {
                    println!("detected item: {:?}", t);
                }
            }
        }
    }

    Ok(())
}

#[allow(unused)]
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
