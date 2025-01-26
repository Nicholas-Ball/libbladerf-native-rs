use anyhow::Result;

const NIOS_PKT_8x8_MAGIC: u8 = 'A' as u8;

/* Request packet indices */
const NIOS_PKT_8x8_IDX_MAGIC: usize = 0;
const NIOS_PKT_8x8_IDX_TARGET_ID: usize = 1;
const NIOS_PKT_8x8_IDX_FLAGS: usize = 2;
//const NIOS_PKT_8x8_IDX_RESV1: usize = 3;
const NIOS_PKT_8x8_IDX_ADDR: usize = 4;
const NIOS_PKT_8x8_IDX_DATA: usize = 5;
//const NIOS_PKT_8x8_IDX_RESV2: usize = 6;

pub fn pack_8x8(target: u8, write: bool,
    addr: u8, data: u8) -> [u8; 15]
{
    let mut buf = [0; 15];
    buf[NIOS_PKT_8x8_IDX_MAGIC] = NIOS_PKT_8x8_MAGIC;
    buf[NIOS_PKT_8x8_IDX_TARGET_ID] = target;

    if (write) {
    buf[NIOS_PKT_8x8_IDX_FLAGS] = 1;
    }

    buf[NIOS_PKT_8x8_IDX_ADDR] = addr;
    buf[NIOS_PKT_8x8_IDX_DATA] = data;

    return buf;
}
pub fn unpack_8x8(packet: &[u8]) -> Result<(u8,bool,u8,u8,bool)>{
    if packet[NIOS_PKT_8x8_IDX_MAGIC] == NIOS_PKT_8x8_MAGIC {
        let target = packet[NIOS_PKT_8x8_IDX_TARGET_ID];
        let write = packet[NIOS_PKT_8x8_IDX_FLAGS] & 1 as u8 != 0;
        let success = packet[NIOS_PKT_8x8_IDX_FLAGS] != 0;
        let addr = packet[NIOS_PKT_8x8_IDX_ADDR];

        let data = packet[NIOS_PKT_8x8_IDX_DATA];

        Ok((target,write,addr,data,success))
    }else {
        Err(anyhow::anyhow!("Invalid packet magic"))
    }
}