use anyhow::Result;
use crate::Device;
use crate::usb::nusb::nusb_host_to_bladerf;

#[cfg(feature = "nusb")]
mod nusb;

#[cfg(feature = "nusb")]
pub async fn control_device_to_host <const request: u8, const value: u16, const index: u16,const len: usize>(device: &Device) -> Result<[u8; len]> {
    if let Some(int) = &device.interface {
        nusb::nusb_bladerf_to_host::<request,value,index,len>(int).await
    } else {
        Err(anyhow::anyhow!("Device not connected"))
    }
}

#[cfg(feature = "nusb")]
pub async fn control_host_to_device<const request: u8, const value: u16, const index: u16,const len: usize>(device: &Device, data: &[u8]) -> Result<()> {
    if let Some(int) = &device.interface {
        nusb_host_to_bladerf::<request,value,index>(int, data).await
    } else {
        Err(anyhow::anyhow!("Device not connected"))
    }
}

#[cfg(feature = "nusb")]
pub async fn bulk_transfer_in<const endpoint: u8, const len: usize>(device: &Device) -> Result<[u8; len]> {
    if let Some(int) = &device.interface {
        Ok(nusb::nusb_bulk_transfer_in::<endpoint,len>(int).await?[..len].try_into()?)
    } else {
        Err(anyhow::anyhow!("Device not connected"))
    }
}

#[cfg(feature = "nusb")]
pub async fn bulk_transfer_out<const endpoint: u8>(device: &Device, buf: &[u8]) -> Result<()> {
    if let Some(int) = &device.interface {
        nusb::nusb_bulk_transfer_out::<endpoint>(int, buf).await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Device not connected"))
    }
}

#[cfg(feature = "nusb")]
pub async fn list_devices<const len: usize>() -> Result<[Option<Device>; len]> {
    nusb::list_devices::<len, 0x2CF0>().await
}
