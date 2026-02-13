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

use mycobot_rs::{MockSerial, MyCobot};

#[test]
fn test_get_angles() {
    let mut mock = MockSerial::new();

    // Commands are:
    // Request: FE FE 02 20 FA
    // Response: FE FE 0E 20 [12 bytes payload] FA

    // Prepare fake response for GetAngles
    // Angles: 0.0 for all.
    // 0.0 * 100 = 0 -> 0x0000
    let mut response = vec![0xFE, 0xFE, 0x0E, 0x20];
    for _ in 0..12 {
        response.push(0);
    }
    response.push(0xFA);

    mock.push_read(&response);

    let mut robot = MyCobot::new(mock);
    let angles = robot.get_angles().unwrap();

    // Use approx comparison if needed, but 0.0 is exact
    assert_eq!(angles, [0.0; 6]);

    // Verify request
    // We need to access the mock from the robot.
    // Since we made `port` public in previous step, this should work IF `MockSerial` implements `SerialPort` correctly
    // AND `MyCobot` exposes it.
    // However, `MyCobot` takes `P: SerialPort`.
    // To access specific methods of MockSerial using `robot.port`, we might need to cast or just know it's MockSerial.
    // Since `robot.port` is type `P` (which is MockSerial here), we can call MockSerial methods directly!
    // BUT only if those methods are available on the struct.
    // We defined `pop_write` on `MockSerial` implementation, not the trait.
    // So yes, `robot.port.pop_write()` works.

    let written = robot.port.pop_write();
    assert_eq!(written, vec![0xFE, 0xFE, 0x02, 0x20, 0xFA]);
}

#[test]
fn test_write_coords() {
    let mock = MockSerial::new();
    let mut robot = MyCobot::new(mock);

    let coords = [10.0, 20.0, 30.0, 0.0, 0.0, 0.0];
    robot.write_coords(coords, 50, 2).unwrap();

    let written = robot.port.pop_write();

    // Verify written data
    // Header(2) + Length(1) + Cmd(1) + Payload(12+1+1=14) + Footer(1)
    // Total Length field = 1 + 14 + 1 = 16 (0x10)

    assert_eq!(written[0], 0xFE);
    assert_eq!(written[1], 0xFE);
    assert_eq!(written[2], 0x10);
    assert_eq!(written[3], 0x25); // WriteCoords
    // ... we could verify payload details if needed
    assert_eq!(written.last(), Some(&0xFA));
}
