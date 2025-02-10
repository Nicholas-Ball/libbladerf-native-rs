use nusb::{Interface};
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient, RequestBuffer, ResponseBuffer};
use crate::usb::Device;
use alloc::vec::Vec;
use anyhow::Error;

pub async fn nusb_bladerf_to_host<const request: u8, const value: u16, const index: u16, const len: usize>(interface: &Interface) -> anyhow::Result<[u8;len]>{
    let buf = interface.control_in(ControlIn{
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request,
        value,
        index,
        length: len as u16,
    }).await;

    if buf.status.is_ok() {
        Ok(buf.data[0..len].try_into()?)
    }else {
        Err(anyhow::anyhow!("Error reading from device"))
    }
}

pub async fn nusb_host_to_bladerf<const request: u8, const value: u16, const index: u16>(interface: &Interface, data: &[u8]) -> anyhow::Result<()>{
    let buf = interface.control_out(ControlOut{
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request,
        value,
        index,
        data,
    }).await;

    if buf.status.is_ok() {
        Ok(())
    }else {
        Err(anyhow::anyhow!("Error writing to device"))
    }
}

pub async fn nusb_bulk_transfer_in<const endpoint: u8, const len: usize>(interface: &Interface) -> anyhow::Result<Vec<u8>>{
    
    let buff = RequestBuffer::new(len);
    let out = interface.bulk_in(endpoint, buff).await;
    out.status?;
    
    Ok(out.data)
}

pub async fn nusb_bulk_transfer_out<const endpoint: u8>(interface: &Interface, buf: &[u8]) -> anyhow::Result<ResponseBuffer>{
    let resp =  interface.bulk_out(endpoint, buf.to_vec()).await;
    resp.status?;
    
    Ok(
        resp.data
    )
}

pub async fn list_devices<const len: usize, const vid: u16>() -> anyhow::Result<[Option<Device>; len]>{
    let mut to_return = [const { None }; len];
    let mut count = 0;

    let devices = nusb::list_devices()?;

    for device in devices{
        if device.vendor_id() == vid{
            to_return[count] = Some(
                Device{
                    vendor_id: device.vendor_id(),
                    product_id: device.product_id(),
                    interface: None,
                    device,
                });
            count += 1;
        }
    }

    Ok(to_return)
}