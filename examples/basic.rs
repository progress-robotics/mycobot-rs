/*
 * Copyright (C) 2026 Progress Robotics UG
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use mycobot_rs::MyCobot;
use serial2::SerialPort as SysSerial;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = SysSerial::open("/dev/ttyAMA0", 1_000_000)?;
    let mut robot = MyCobot::new(port);

    robot.power_on()?;

    let angles = robot.get_angles()?;
    println!("Angles: {:?}", angles);

    Ok(())
}
