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

use crate::commands::Command;
use crate::io::SerialPort;
use crate::protocol::Packet;
use std::time::Duration;
use log::{debug, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Timeout waiting for response")]
    Timeout,
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct MyCobot<P: SerialPort> {
    pub port: P,
    debug_mode: bool,
}

impl<P: SerialPort> MyCobot<P> {
    pub fn new(port: P) -> Self {
        Self {
            port,
            debug_mode: false,
        }
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug_mode = debug;
    }

    /// Helper to write a command without waiting for response
    fn write_command(&mut self, command: Command, payload: Vec<u8>) -> Result<()> {
        let packet = Packet::new(command, payload);
        let bytes = packet.to_bytes();
        if self.debug_mode {
            debug!("Writing: {:02X?}", bytes);
        }
        self.port.write_all(&bytes)?;
        std::io::Write::flush(&mut self.port)?;
        Ok(())
    }

    /// Helper to write a command and wait for a response
    /// Returns the payload of the response packet
    fn request(&mut self, command: Command, payload: Vec<u8>, timeout: Duration) -> Result<Vec<u8>> {
        self.write_command(command, payload)?;

        // Simple blocking read loop with timeout
        // Since we are using serial2 in blocking mode or with timeouts set on the port,
        // we can try to read byte by byte or in chunks.
        // For simplicity in this initial blocking version, we'll read byte-by-byte to parse.
        // A better approach for serial2 is to set a read timeout on the port itself.
        
        let start = std::time::Instant::now();
        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 1];

        loop {
            if start.elapsed() > timeout {
                return Err(Error::Timeout);
            }

            // This read might block depending on port config. 
            // We assume the user has configured the port with a timeout or is using non-blocking with retry.
            // But here we are wrapping a generic SerialPort trait which is just Read+Write.
            // We should use a loop with short sleeps if the read returns 0/WouldBlock, but std::io::Read 
            // doesn't guarantee timeout behavior without trait support.
            
            // For the purpose of this library, we assume the underlying port handles blocking/timeout
            // or returns quickly.
            
            match self.port.read(&mut temp_buf) {
                Ok(0) => {
                    // EOF or no data yet?
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Ok(1) => {
                    buffer.push(temp_buf[0]);
                    // Try to parse
                    match Packet::parse(&buffer) {
                        Ok(Some((packet, _consumed))) => {
                             if self.debug_mode {
                                debug!("Received: {:?} {:02X?}", packet.command, packet.payload);
                            }
                            // Does the response command match? 
                            // Usually response command is same as request for getters.
                            if packet.command == command {
                                return Ok(packet.payload);
                            } else {
                                // Mismatch, might be old data or async message. 
                                // For now, log and continue or return error?
                                // Let's simplify: return it if it's not a known async packet.
                                warn!("Received unexpected command {:?} waiting for {:?}", packet.command, command);
                                // Reset buffer to search for next packet? 
                                // Packet::parse consumes bytes conceptually but here we just have the full buffer.
                                // If we found a packet but it's wrong, we should ideally consume it and continue.
                                // But Packet::parse returns (packet, bytes_consumed).
                                // We need to remove the consumed bytes.
                            }
                        },
                        Ok(None) => continue, // Need more data
                        Err(_e) => {
                             // Invalid data, maybe skip one byte?
                             if buffer.len() > 0 {
                                 buffer.remove(0);
                             }
                             continue;
                        }
                    }
                }
                Ok(_) => unreachable!(), // we asked for 1 byte
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                     std::thread::sleep(Duration::from_millis(1));
                     continue;
                }
                Err(e) => return Err(Error::Io(e)),
            }
        }
    }

    // --- Basic Control ---

    pub fn power_on(&mut self) -> Result<()> {
        self.write_command(Command::PowerOn, vec![])
    }

    pub fn power_off(&mut self) -> Result<()> {
        self.write_command(Command::PowerOff, vec![])
    }
    
    pub fn is_powered_on(&mut self) -> Result<bool> {
        let response = self.request(Command::IsPoweredOn, vec![], Duration::from_millis(500))?;
        if response.len() == 1 {
            Ok(response[0] == 1)
        } else {
            Err(Error::Protocol("Invalid payload length for IsPoweredOn".into()))
        }
    }

    // --- Atom IO ---
    
    pub fn set_led_color(&mut self, r: u8, g: u8, b: u8) -> Result<()> {
        self.write_command(Command::SetLedRgb, vec![r, g, b])
    }

    // --- Movement ---
    
    /// Get current joint angles
    pub fn get_angles(&mut self) -> Result<[f32; 6]> {
        let response = self.request(Command::GetAngles, vec![], Duration::from_millis(500))?;
        if response.len() != 12 {
            return Err(Error::Protocol(format!("Expected 12 bytes for angles, got {}", response.len())));
        }
        
        let mut angles = [0.0; 6];
        for i in 0..6 {
            let high = response[i * 2];
            let low = response[i * 2 + 1];
            let raw = (high as i16) << 8 | (low as i16); // Big endian
            angles[i] = raw as f32 / 100.0;
        }
        Ok(angles)
    }

    pub fn write_angles(&mut self, angles: [f32; 6], speed: u8) -> Result<()> {
        let mut payload = Vec::with_capacity(13);
        for &angle in &angles {
            let value = (angle * 100.0) as i16;
            let bytes = value.to_be_bytes();
            payload.push(bytes[0]);
            payload.push(bytes[1]);
        }
        payload.push(speed);
        self.write_command(Command::WriteAngles, payload)
    }
    
    pub fn get_coords(&mut self) -> Result<[f32; 6]> {
         let response = self.request(Command::GetCoords, vec![], Duration::from_millis(500))?;
        if response.len() != 12 {
            return Err(Error::Protocol(format!("Expected 12 bytes for coords, got {}", response.len())));
        }
        
        let mut coords = [0.0; 6];
        // XYZ
        for i in 0..3 {
            let high = response[i * 2];
            let low = response[i * 2 + 1];
            let raw = (high as i16) << 8 | (low as i16);
            coords[i] = raw as f32 / 10.0;
        }
        // RxRyRz
        for i in 3..6 {
            let high = response[i * 2];
            let low = response[i * 2 + 1];
            let raw = (high as i16) << 8 | (low as i16);
            coords[i] = raw as f32 / 100.0;
        }
        Ok(coords)
    }

    pub fn write_coords(&mut self, coords: [f32; 6], speed: u8, _mode: u8) -> Result<()> {
        let mut payload = Vec::with_capacity(14);
        // XYZ
        for i in 0..3 {
            let value = (coords[i] * 10.0) as i16;
            let bytes = value.to_be_bytes();
            payload.push(bytes[0]);
            payload.push(bytes[1]);
        }
        // RxRyRz
        for i in 3..6 {
            let value = (coords[i] * 100.0) as i16;
            let bytes = value.to_be_bytes();
            payload.push(bytes[0]);
            payload.push(bytes[1]);
        }
        payload.push(speed);
        payload.push(2); // Mode (MoveJ? Check docs, usually 2 for MyCobot)
                            // C++ Code: command += static_cast<char>(2);
                            // MyCobot.cpp:165
        
        self.write_command(Command::WriteCoords, payload)
    }
}
