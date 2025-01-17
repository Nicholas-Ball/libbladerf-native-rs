use crate::Device;
use anyhow::Result;
use crate::usb::{bulk_transfer_in, bulk_transfer_out};

mod packet;

const EP_OUT: u8 = 2;
const EP_IN: u8 = 0x82;


pub async fn nios_access<const RESP_LEN: usize>(device: &Device, buf: &[u8]) -> Result<[u8; RESP_LEN]> {
    bulk_transfer_out::<EP_OUT>(device, buf).await?;
    let resp = bulk_transfer_in::<EP_IN, RESP_LEN>(device).await?;
    Ok(resp)
}