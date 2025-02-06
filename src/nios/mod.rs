use crate::{BladerfDirection, BladerfVersion, Device};
use anyhow::Result;
use crate::nios::packet as packets;
use crate::usb::{bulk_transfer_in, bulk_transfer_out};

mod packet;
pub mod nios_access;

const EP_OUT: u8 = 2;
const EP_IN: u8 = 0x82;
const NIOS_PKT_8x64_TARGET_TIMESTAMP: u8 = 0;

pub async fn nios_access<const RESP_LEN: usize>(device: &Device, buf: &[u8]) -> Result<[u8; RESP_LEN]> {
    bulk_transfer_out::<EP_OUT>(device, buf).await?;
    let resp = bulk_transfer_in::<EP_IN, RESP_LEN>(device).await?;
    Ok(resp)
}

pub async fn nios_8x32_read<const ID:u8, const ADDR: u8>(device: &Device, ) -> Result<u32> {
    let buf = packets::pkt_8x32::pack_8x32(ID, false, ADDR, 0);
    let resp = nios_access::<16>(device, &buf).await?;
    let (_, _, _, data, _) = packet::pkt_8x32::unpack_8x32(&resp)?;
    Ok(data)
}

pub async fn nios_8x32_write<const ID:u8, const ADDR: u8>(device: &Device, data: u32) -> Result<()> {
    let buf = packets::pkt_8x32::pack_8x32(ID, true, ADDR, data as u32);
    nios_access::<16>(device, &buf).await?;
    Ok(())
}

pub async fn nios_get_timestamp(device: &Device, bladerf_direction: BladerfDirection) -> Result<u64> {
    let buf = packets::pkt_8x64::pack_8x64(NIOS_PKT_8x64_TARGET_TIMESTAMP, false, bladerf_direction as u8, 0);
    
    let timestamp_packed = nios_access::<16>(device, &buf).await?;
    let timestamp = timestamp_packed[8..].try_into()?;
    
    Ok(u64::from_le_bytes(timestamp))
}

pub async fn nios_get_fpga_version(device: &Device) -> Result<BladerfVersion> {
    let version = nios_8x32_read::<0, 0>(device).await?;
    
    Ok(BladerfVersion{
        major: (version >> 16) as u8,
        minor: (version >> 8) as u8,
        patch: version as u8,
    })
    
}