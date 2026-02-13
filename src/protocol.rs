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

pub const HEADER: [u8; 2] = [0xFE, 0xFE];
pub const FOOTER: u8 = 0xFA;

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    pub command: Command,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(command: Command, payload: Vec<u8>) -> Self {
        Self { command, payload }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&HEADER);
        // Length = command (1) + payload (N) + footer (1)
        let len = 1 + self.payload.len() as u8 + 1;
        bytes.push(len);
        bytes.push(self.command.into());
        bytes.extend_from_slice(&self.payload);
        bytes.push(FOOTER);
        bytes
    }

    /// Tries to parse a packet from the given buffer.
    /// Returns Ok(Some((packet, bytes_consumed))) if a full packet is found.
    /// Returns Ok(None) if more data is needed.
    /// Returns Err if the data is invalid (e.g. wrong header) and should be skipped.
    pub fn parse(buffer: &[u8]) -> Result<Option<(Packet, usize)>, String> {
        if buffer.len() < 2 {
            return Ok(None);
        }

        // Look for header
        if buffer[0] != HEADER[0] || buffer[1] != HEADER[1] {
            // If not starting with header, we should skip one byte to try to find sync
            // But the caller needs to handle skipping. We just say "not a packet at pos 0"
            return Err("Invalid header".to_string());
        }

        if buffer.len() < 3 {
            return Ok(None); // Need length byte
        }

        let len_field = buffer[2];
        let total_frame_len = 2 + 1 + len_field as usize; // Header (2) + LenByte (1) + Body (Length)
        
        // Wait for full frame
        if buffer.len() < total_frame_len {
            return Ok(None);
        }

        // Validate footer
        if buffer[total_frame_len - 1] != FOOTER {
            return Err("Invalid footer".to_string());
        }

        let command_byte = buffer[3];
        let payload_len = len_field as usize - 2; // -1 for command, -1 for footer
        let payload = buffer[4..4+payload_len].to_vec();

        Ok(Some((
            Packet {
                command: Command::from(command_byte),
                payload,
            },
            total_frame_len
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        let packet = Packet::new(Command::GetAngles, vec![]);
        // Header(2) + Len(2: Cmd+Footer) + Cmd(0x20) + Footer(0xFA)
        let bytes = packet.to_bytes();
        assert_eq!(bytes, vec![0xFE, 0xFE, 0x02, 0x20, 0xFA]);
    }
    
    #[test]
    fn test_parse() {
        let data = vec![0xFE, 0xFE, 0x02, 0x20, 0xFA];
        let (packet, consumed) = Packet::parse(&data).unwrap().unwrap();
        assert_eq!(consumed, 5);
        assert_eq!(packet.command, Command::GetAngles);
        assert_eq!(packet.payload.len(), 0);
    }
}
