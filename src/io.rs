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

use std::io::{self, Read, Write};

/// Trait for serial port communication to allow mocking.
pub trait SerialPort: io::Read + io::Write + Send {
    fn flush(&mut self) -> io::Result<()>;
}

impl SerialPort for serial2::SerialPort {
    fn flush(&mut self) -> io::Result<()> {
        // serial2::SerialPort::flush takes &self, but io::Write::flush takes &mut self
        // We can just call the inherent method or the trait method.
        io::Write::flush(self)
    }
}

/// A mock serial port for testing.
pub struct MockSerial {
    pub read_buffer: Vec<u8>,
    pub written_data: Vec<u8>,
}

impl MockSerial {
    pub fn new() -> Self {
        Self {
            read_buffer: Vec::new(),
            written_data: Vec::new(),
        }
    }

    pub fn push_read(&mut self, data: &[u8]) {
        self.read_buffer.extend_from_slice(data);
    }
    
    pub fn pop_write(&mut self) -> Vec<u8> {
        let data = self.written_data.clone();
        self.written_data.clear();
        data
    }
}

impl Read for MockSerial {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read_buffer.is_empty() {
            return Err(io::Error::new(io::ErrorKind::WouldBlock, "No data"));
        }
        let len = std::cmp::min(buf.len(), self.read_buffer.len());
        buf[..len].copy_from_slice(&self.read_buffer[..len]);
        self.read_buffer.drain(..len);
        Ok(len)
    }
}


impl SerialPort for MockSerial {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// Fix Write impl for MockSerial
impl Write for MockSerial {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.written_data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
