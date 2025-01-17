use ::nusb::{DeviceInfo, Interface};
use anyhow::Result;
use crate::Device;
use crate::usb::nusb::nusb_host_to_bladerf;

#[cfg(feature = "nusb")]
mod nusb;

const BLADE_USB_CMD_QUERY_VERSION: u8 = 0;
const BLADE_USB_CMD_QUERY_FPGA_STATUS: u8 = 1;
const BLADE_USB_CMD_BEGIN_PROG: u8 = 2;
const BLADE_USB_CMD_END_PROG: u8 = 3;
const BLADE_USB_CMD_RF_RX: u8 = 4;
const BLADE_USB_CMD_RF_TX: u8 = 5;
const BLADE_USB_CMD_QUERY_DEVICE_READY: u8 = 6;
const BLADE_USB_CMD_QUERY_FLASH_ID: u8 = 7;
const BLADE_USB_CMD_QUERY_FPGA_SOURCE: u8 = 8;
const BLADE_USB_CMD_FLASH_READ: u8 = 100;

#[cfg(feature = "nusb")]
mod nusb;

pub struct Device {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,

#[cfg(feature = "nusb")]
pub async fn control_device_to_host<const request: u8, const value: u16, const index: u16,const len: usize>(device: &mut Device) -> Result<[u8; len]> {
    if let Some(int) = &mut device.interface {
        nusb::nusb_bladerf_to_host::<request,value,index,len>(int).await
    } else {
        Err(anyhow::anyhow!("Device not connected"))
    }
}

#[cfg(feature = "nusb")]
pub async fn control_host_to_device<const request: u8, const value: u16, const index: u16,const len: usize>(device: &mut Device, data: &[u8]) -> Result<()> {
    if let Some(int) = &mut device.interface {
        nusb_host_to_bladerf::<request,value,index>(int, data).await
    } else {
        Err(anyhow::anyhow!("Device not connected"))
    }
}3