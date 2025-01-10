use std::future::Future;
use ::nusb::{DeviceInfo, Interface};
use anyhow::Result;

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
            nusb::nusb_bladerf_to_host::<0,0,0,4>(int).await
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