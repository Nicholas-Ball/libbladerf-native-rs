use anyhow::Result;

const NIOS_PKT_8x32_MAGIC: u8 = b'C';

/* Request packet indices */
const NIOS_PKT_8x32_IDX_MAGIC: usize = 0;
const NIOS_PKT_8x32_IDX_TARGET_ID: usize = 1;
const NIOS_PKT_8x32_IDX_FLAGS: usize = 2;
const NIOS_PKT_8x32_IDX_RESV1: usize = 3;
const NIOS_PKT_8x32_IDX_ADDR: usize = 4;
const NIOS_PKT_8x32_IDX_DATA: usize = 5;
const NIOS_PKT_8x32_IDX_RESV2: usize = 9;


const NIOS_PKT_8x32_FLAG_WRITE: usize = 1;
const NIOS_PKT_8x32_FLAG_SUCCESS: usize = 2;

pub fn pack_8x32(target: u8, write: bool, addr: u8, data: u32) -> [u8; 16] {
    let mut buf = [0; 16];
    
    buf[NIOS_PKT_8x32_IDX_MAGIC] = NIOS_PKT_8x32_MAGIC;
    buf[NIOS_PKT_8x32_IDX_TARGET_ID] = target;

    if write {
        buf[NIOS_PKT_8x32_IDX_FLAGS] = NIOS_PKT_8x32_FLAG_WRITE as u8;
    }

    buf[NIOS_PKT_8x32_IDX_ADDR] = addr;

    buf[NIOS_PKT_8x32_IDX_DATA] = data as u8;
    buf[NIOS_PKT_8x32_IDX_DATA + 1] = (data >> 8) as u8;
    buf[NIOS_PKT_8x32_IDX_DATA + 2] = (data >> 16) as u8;
    buf[NIOS_PKT_8x32_IDX_DATA + 3] = (data >> 24) as u8;
    
    buf
}

pub fn unpack_8x32(packet: &[u8]) -> Result<(u8,bool,u8,u32,bool)>{
    if packet[NIOS_PKT_8x32_IDX_MAGIC] == NIOS_PKT_8x32_MAGIC {
        let target = packet[NIOS_PKT_8x32_IDX_TARGET_ID];
        let write = packet[NIOS_PKT_8x32_IDX_FLAGS] & NIOS_PKT_8x32_FLAG_WRITE as u8 != 0;
        let success = packet[NIOS_PKT_8x32_IDX_FLAGS] & NIOS_PKT_8x32_FLAG_SUCCESS as u8 != 0;
        let addr = packet[NIOS_PKT_8x32_IDX_ADDR];

        let data = (packet[NIOS_PKT_8x32_IDX_DATA] as u32) |
            (packet[NIOS_PKT_8x32_IDX_DATA + 1] as u32) << 8 |
            (packet[NIOS_PKT_8x32_IDX_DATA + 2] as u32) << 16 |
            (packet[NIOS_PKT_8x32_IDX_DATA + 3] as u32) << 24;

        Ok((target,write,addr,data,success))
    }else {
        Err(anyhow::anyhow!("Invalid packet magic"))
    }
}