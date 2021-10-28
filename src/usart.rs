use core::{convert::Infallible, fmt};

use hal::{hal::serial::Write, pac::USART1};

/// USART1 is already initialized when debugging over usb
pub struct USerial;

impl Write<u8> for USerial {
    type Error = nb::Error<Infallible>;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART1::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            unsafe { &*USART1::ptr() }
                .dr
                .write(|w| w.dr().bits(word as u16));

            if word as char == '\n' {
                self.write(word)?
            }

            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART1::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl fmt::Write for USerial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.bytes()
            .try_for_each(|c| nb::block!(self.write(c)))
            .map_err(|_| fmt::Error)
    }
}
/// Prints to [`USART1`]
///
/// Equivalent to the [`uprintln!`] macro except that a newline is not printed at
/// the end of the message.
///
/// Note that when using dbgArm, [`USART1`] is already intitialized
#[macro_export]
macro_rules! uprint {
    ($($arg:tt)*) => {
        core::fmt::Write::write_fmt(&mut $crate::usart::USerial, format_args!($($arg)*)).ok()
    };
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! uprintln {
    () => ($crate::uprint!("\n"));
    ($fmt:expr) => {
        $crate::uprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::uprint!(concat!($fmt, "\n"), $($arg)*)
    };
}
