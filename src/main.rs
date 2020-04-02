use ror2_command::robot;
use ror2_command::robot::{Robot, MousePos};

fn main() -> Result<(), ()> {
    let mut robot = robot::WinRobot::new();
    robot.click_on(MousePos(500, 200));
    robot.click_on(MousePos(500, 200));
    Ok(())
}
