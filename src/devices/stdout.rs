use std::io::{stdout, Write};

use crate::serial::SerialDevice;

pub struct StdoutDevice {
    flush: bool,
    callback: fn(image_buffer: &Vec<u8>),
}

impl StdoutDevice {
    pub fn new(flush: bool) -> Self {
        Self {
            flush,
            callback: |_| {},
        }
    }

    pub fn set_callback(&mut self, callback: fn(image_buffer: &Vec<u8>)) {
        self.callback = callback;
    }
}

impl SerialDevice for StdoutDevice {
    fn send(&mut self) -> u8 {
        0xff
    }

    fn receive(&mut self, byte: u8) {
        print!("{}", byte as char);
        if self.flush {
            stdout().flush().unwrap();
        }
        let data = vec![byte];
        (self.callback)(&data);
    }

    fn allow_slave(&self) -> bool {
        false
    }
}

impl Default for StdoutDevice {
    fn default() -> Self {
        Self::new(true)
    }
}
