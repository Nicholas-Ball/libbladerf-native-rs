#![no_std]
#![feature(impl_trait_in_fn_trait_return)]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod usb;