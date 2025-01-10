use std::future::Future;
use ::nusb::{DeviceInfo, Interface};
use anyhow::Result;

const BLADE_USB_CMD_QUERY_VERSION: u8 = 0;
const BLADE_USB_CMD_QUERY_FPGA_STATUS: u8 = 2;
const BLADE_USB_CMD_BEGIN_PROG: u8 = 3;
const BLADE_USB_CMD_END_PROG: u8 = 4;
const BLADE_USB_CMD_RF_RX: u8 = 5;
const BLADE_USB_CMD_RF_TX: u8 = 6;
const BLADE_USB_CMD_QUERY_DEVICE_READY: u8 = 7;
const BLADE_USB_CMD_QUERY_FLASH_ID: u8 = 8;
const BLADE_USB_CMD_QUERY_FPGA_SOURCE: u8 = 9;
const BLADE_USB_CMD_FLASH_READ: u8 = 100;

#[cfg(feature = "nusb")]
mod nusb;

pub struct Device {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,

    #[cfg(feature = "nusb")]
    pub(crate) interface: Option<Interface>,

    #[cfg(feature = "nusb")]
    pub(crate) device: DeviceInfo,
}

#[cfg(feature = "nusb")]
pub async fn get_version(interface: &mut Interface) -> Result<[u8;4]> {
    nusb::nusb_bladerf_to_host::<0,0,0,4>(interface).await
}

#[cfg(feature = "nusb")]
pub async fn list_devices<const len: usize>() -> Result<[Option<Device>; len]> {
    nusb::list_devices::<len, 0x2CF0>().await
}

impl Device{
    #[cfg(feature = "nusb")]
    pub fn is_connected(&self) -> bool {
        self.interface.is_some()
    }
    
    #[cfg(feature = "nusb")]
    pub async fn connect(&mut self) -> Result<()> {
        // Connect to the device
        self.interface = Some(self.device.open()?.claim_interface(0)?);
        
        Ok(())
    }

    #[cfg(feature = "nusb")]
    pub async fn get_version(&mut self) -> Result<[u8;4]>{
        if let Some(int) = &mut self.interface {
            nusb::nusb_bladerf_to_host::<BLADE_USB_CMD_QUERY_VERSION,0,0,4>(int).await
        }else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }


    #[cfg(feature = "nusb")]
    pub fn disconnect(&mut self) -> Result<()> {
        // Disconnect from the device
        self.interface = None;
        Ok(())
    }
}