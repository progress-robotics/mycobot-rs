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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    // System definitions
    Undefined = 0x00,
    Version = 0x01,
    
    // Power & status
    PowerOn = 0x10,
    PowerOff = 0x11,
    IsPoweredOn = 0x12,
    ReleaseAllServos = 0x13,
    IsControllerConnected = 0x14,
    ReadNextError = 0x15,
    SetFreeMoveMode = 0x1A,
    IsFreeMoveMode = 0x1B,

    // MDI & Operation
    GetAngles = 0x20,
    WriteAngle = 0x21,
    WriteAngles = 0x22,
    GetCoords = 0x23,
    WriteCoord = 0x24,
    WriteCoords = 0x25,
    ProgramPause = 0x26,
    IsProgramPaused = 0x27,
    ProgramResume = 0x28,
    TaskStop = 0x29,
    IsInPosition = 0x2A,
    CheckRunning = 0x2B,

    // Jogging
    JogAngle = 0x30,
    JogAbsolute = 0x31,
    JogCoord = 0x32,
    SendJogIncrement = 0x33,
    JogStop = 0x34,
    
    // Encoder
    SetEncoder = 0x3A,
    GetEncoder = 0x3B,
    SetEncoders = 0x3C,
    GetEncoders = 0x3D,

    // Speed
    GetSpeed = 0x40,
    SetSpeed = 0x41,
    
    // IO
    SetPinMode = 0x60,
    SetDigitalOut = 0x61,
    GetDigitalIn = 0x62,
    
    // Gripper / LED
    GripperMode = 0x66,
    SetLedRgb = 0x6A,
    
    // Basic
    SetBasicOut = 0xA0,
    GetBasicIn = 0xA1,
    
    // Fallback
    Unknown(u8),
}

impl From<u8> for Command {
    fn from(byte: u8) -> Self {
        match byte {
            0x10 => Command::PowerOn,
            0x11 => Command::PowerOff,
            0x12 => Command::IsPoweredOn,
            0x13 => Command::ReleaseAllServos,
            0x14 => Command::IsControllerConnected,
            0x20 => Command::GetAngles,
            0x21 => Command::WriteAngle,
            0x22 => Command::WriteAngles,
            0x23 => Command::GetCoords,
            0x24 => Command::WriteCoord,
            0x25 => Command::WriteCoords,
            0x2A => Command::IsInPosition,
            0x2B => Command::CheckRunning,
            0x40 => Command::GetSpeed,
            0x41 => Command::SetSpeed,
            0x6A => Command::SetLedRgb,
            0xA0 => Command::SetBasicOut,
            0xA1 => Command::GetBasicIn,
            // ... add others as needed
            b => Command::Unknown(b),
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::PowerOn => 0x10,
            Command::PowerOff => 0x11,
            Command::IsPoweredOn => 0x12,
            Command::ReleaseAllServos => 0x13,
            Command::IsControllerConnected => 0x14,
            Command::GetAngles => 0x20,
            Command::WriteAngle => 0x21,
            Command::WriteAngles => 0x22,
            Command::GetCoords => 0x23,
            Command::WriteCoord => 0x24,
            Command::WriteCoords => 0x25,
            Command::IsInPosition => 0x2A,
            Command::CheckRunning => 0x2B,
            Command::GetSpeed => 0x40,
            Command::SetSpeed => 0x41,
            Command::SetLedRgb => 0x6A,
            Command::SetBasicOut => 0xA0,
            Command::GetBasicIn => 0xA1,
            // ...
            Command::Unknown(b) => b,
            _ => 0x00, // TODO: map all
        }
    }
}
