use crate::{BladerfDirection, BladerfVersion, Device};
use anyhow::Result;
use crate::nios::packet as packets;
use crate::usb::{bulk_transfer_in, bulk_transfer_out};

mod packet;
pub mod nios_access;