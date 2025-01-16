#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod usb;
mod nios;