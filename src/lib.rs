#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use nusb::{DeviceInfo, Interface};

pub mod usb;
mod nios;


pub struct Device {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,

    #[cfg(feature = "nusb")]
    pub(crate) interface: Option<Interface>,

    #[cfg(feature = "nusb")]
    pub(crate) device: DeviceInfo,
}

#[cfg(feature = "nusb")]
pub async fn list_devices<const len: usize>() -> anyhow::Result<[Option<Device>; len]> {
    usb::list_devices::<len, 0x2CF0>().await
}

#[cfg(feature = "nusb")]
impl Device{
    pub fn is_connected(&self) -> bool {
        self.interface.is_some()
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        // Connect to the device
        self.interface = Some(self.device.open()?.claim_interface(0)?);

        Ok(())
    }

    pub async fn enable_rx(&mut self) -> anyhow::Result<()> {
        if let Some(int) = &mut self.interface {
            let test = nusb::nusb_bladerf_to_host::<crate::usb::BLADE_USB_CMD_RF_RX,1,0,4>(int).await?;

            if test == [0,0,0,0] {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Error enabling RX"))
            }
        } else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }

    pub async fn disable_rx(&mut self) -> anyhow::Result<()> {
        if let Some(int) = &mut self.interface {
            let test = nusb::nusb_bladerf_to_host::<crate::usb::BLADE_USB_CMD_RF_RX,0,0,4>(int).await?;

            if test == [64,0,0,0] {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Error disabling RX"))
            }
        } else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }


    pub async fn enable_tx(&mut self) -> anyhow::Result<()> {
        if let Some(int) = &mut self.interface {
            let test = nusb::nusb_bladerf_to_host::<crate::usb::BLADE_USB_CMD_RF_TX,1,0,4>(int).await?;

            if test == [0,0,0,0] {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Error enabling RX"))
            }
        } else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }

    pub async fn disable_tx(&mut self) -> anyhow::Result<()> {
        if let Some(int) = &mut self.interface {
            let test = nusb::nusb_bladerf_to_host::<crate::usb::BLADE_USB_CMD_RF_TX,0,0,4>(int).await?;

            if test == [64,0,0,0] {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Error disabling RX"))
            }
        } else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }

    pub async fn get_version(&mut self) -> anyhow::Result<[u8; 4]> {
        if let Some(int) = &mut self.interface {
            nusb::nusb_bladerf_to_host::<crate::usb::BLADE_USB_CMD_QUERY_VERSION,0,0,4>(int).await
        } else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }

    pub fn disconnect(&mut self) -> anyhow::Result<()> {
        // Disconnect from the device
        self.interface = None;
        Ok(())
    }
}