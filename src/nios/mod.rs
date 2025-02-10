use crate::usb::{bulk_transfer_in, bulk_transfer_out};
use crate::{BladerfDirection, BladerfVersion, Device};
use anyhow::Result;

mod packet;
pub mod nios_access;