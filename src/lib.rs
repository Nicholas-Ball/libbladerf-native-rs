#![no_std]

#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use crate::nios::nios_access::nios_lms6_read;
#[cfg(feature = "nusb")]
use ::nusb::{DeviceInfo, Interface};
use usb::*;

pub mod usb;
pub mod nios;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BladerfVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

// const BLADE_USB_CMD_QUERY_VERSION: u8 = 0;
// const BLADE_USB_CMD_QUERY_FPGA_STATUS: u8 = 1;
// const BLADE_USB_CMD_BEGIN_PROG: u8 = 2;
// const BLADE_USB_CMD_END_PROG: u8 = 3;
const BLADE_USB_CMD_RF_RX: u8 = 4;
const BLADE_USB_CMD_RF_TX: u8 = 5;
// const BLADE_USB_CMD_QUERY_DEVICE_READY: u8 = 6;
// const BLADE_USB_CMD_QUERY_FLASH_ID: u8 = 7;
// const BLADE_USB_CMD_QUERY_FPGA_SOURCE: u8 = 8;
// const BLADE_USB_CMD_FLASH_READ: u8 = 100;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BladerfDirection {
    RX,
    TX,
}

pub struct Device {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,

    #[cfg(feature = "nusb")]
    pub(crate) interface: Option<Interface>,

    #[cfg(feature = "nusb")]
    pub(crate) device: DeviceInfo,
}


pub async fn list_devices<const len: usize>() -> anyhow::Result<[Option<Device>; len]> {
    usb::list_devices::<len>().await
}

#[cfg(feature = "nusb")]
impl Device {
    pub fn is_connected(&self) -> bool {
        self.interface.is_some()
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        // Connect to the device
        self.interface = Some(self.device.open()?.claim_interface(0)?);

        Ok(())
    }

    pub async fn enable_rx(&mut self) -> anyhow::Result<()> {
        let test = control_device_to_host::<BLADE_USB_CMD_RF_RX, 1, 0, 4>(self).await?;

        if test == [0, 0, 0, 0] {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Error enabling RX"))
        }
    }

    pub async fn disable_rx(&mut self) -> anyhow::Result<()> {
        let test = control_device_to_host::<BLADE_USB_CMD_RF_RX, 0, 0, 4>(self).await?;

        if test == [64, 0, 0, 0] {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Error disabling RX"))
        }
    }


    pub async fn enable_tx(&self) -> anyhow::Result<()> {
        let test = control_device_to_host::<BLADE_USB_CMD_RF_TX, 1, 0, 4>(self).await?;

        if test == [0, 0, 0, 0] {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Error enabling RX"))
        }
    }

    pub async fn disable_tx(&mut self) -> anyhow::Result<()> {
        let test = control_device_to_host::<BLADE_USB_CMD_RF_TX, 0, 0, 4>(self).await?;

        if test == [64, 0, 0, 0] {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Error disabling RX"))
        }
    }

    pub async fn get_version(&mut self) -> anyhow::Result<BladerfVersion> {
        let version = usb::nusb::nusb_bladerf_to_host::<0, 0, 0, 4>(&<Option<Interface> as Clone>::clone(&self.interface).unwrap()).await?;

        Ok(BladerfVersion {
            major: version[0],
            minor: version[2],
            patch: version[1],
        })
    }
    
    pub async fn get_gain(&mut self, bladerf_direction: BladerfDirection) -> anyhow::Result<f32> {
        let lna = nios_lms6_read(self, 0x75).await?;

        Ok(lna as f32)
    }

    pub fn disconnect(&mut self) -> anyhow::Result<()> {
        // Disconnect from the device
        self.interface = None;
        Ok(())
    }
}