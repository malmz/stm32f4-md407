use core::convert::Infallible;

use hal::gpio::{Input, Output, PullDown, PushPull};
use nb::Error;

use crate::ports::{Port, PortState};

pub enum KeypadState {}

impl PortState for KeypadState {
    type SP0 = Input<PullDown>;
    type SP1 = Input<PullDown>;
    type SP2 = Input<PullDown>;
    type SP3 = Input<PullDown>;
    type SP4 = Output<PushPull>;
    type SP5 = Output<PushPull>;
    type SP6 = Output<PushPull>;
    type SP7 = Output<PushPull>;
}

pub struct Keypad {
    port: Port<KeypadState>,

    pressed_keys: [[bool; 4]; 4],
    current_frame: [[bool; 4]; 4],
}

const KEYS: [[char; 4]; 4] = [
    ['1', '2', '3', 'A'],
    ['4', '5', '6', 'B'],
    ['7', '8', '9', 'C'],
    ['*', '0', '#', 'D'],
];

impl Keypad {
    pub fn new(port: Port<KeypadState>) -> Self {
        Self {
            port,
            pressed_keys: Default::default(),
            current_frame: Default::default(),
        }
    }

    fn activate_row(&mut self, row: usize) {
        match row {
            0 => self.port.p4.set_high(),
            1 => self.port.p5.set_high(),
            2 => self.port.p6.set_high(),
            3 => self.port.p7.set_high(),
            _ => {}
        }
    }

    fn read_col(&self, col: usize) -> bool {
        match col {
            0 => self.port.p0.is_high(),
            1 => self.port.p1.is_high(),
            2 => self.port.p2.is_high(),
            3 => self.port.p3.is_high(),
            _ => false,
        }
    }

    fn clear_rows(&mut self) {
        self.port.p4.set_low();
        self.port.p5.set_low();
        self.port.p6.set_low();
        self.port.p7.set_low();
    }

    pub fn read_key(&mut self) -> nb::Result<char, Infallible> {
        let mut key_count = 0;
        let mut val = '\0';

        for row in 0..4 {
            self.clear_rows();
            self.activate_row(row);
            for col in 0..4 {
                if self.read_col(col) {
                    self.current_frame[row][col] = true;
                    if !self.pressed_keys[row][col] {
                        self.pressed_keys[row][col] = true;
                        key_count += 1;
                        val = KEYS[row][col];
                    }
                } else {
                    self.current_frame[row][col] = true;
                    self.pressed_keys[row][col] = false;
                }
            }
        }

        let mut debounce = false;
        'outer: for row in 0..4 {
            self.clear_rows();
            self.activate_row(row);
            for col in 0..4 {
                if self.current_frame[row][col] != self.read_col(col) {
                    debounce = true;
                    break 'outer;
                }
            }
        }

        if debounce {
            return Ok('-');
        }

        if key_count == 1 && !debounce {
            Ok(val)
        } else {
            Err(Error::WouldBlock)
        }
    }
}
