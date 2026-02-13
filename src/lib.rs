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

pub mod io;
pub mod protocol;
pub mod commands;
pub mod robot;

pub use io::{SerialPort, MockSerial};
pub use robot::{MyCobot, Error, Result};
pub use commands::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_on() {
        let mock = MockSerial::new();
        let mut robot = MyCobot::new(mock);
        
        robot.power_on().unwrap();
        
        // We need to access the mock inside the robot to verify writes.
        // But MyCobot consumes the port.
        // We can't access `port` field because it's private.
        // We should add a method to decompose or access inner? 
        // Or make MockSerial split into verified channels.
        // For now, let's just make `port` public for crate or provide a `into_inner`.
    }
}
