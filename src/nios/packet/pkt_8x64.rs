use anyhow::Result;

const NIOS_PKT_8x64_MAGIC: u8 = 'D' as u8;

/* Request packet indices */
const NIOS_PKT_8x64_IDX_MAGIC: usize = 0;
const NIOS_PKT_8x64_IDX_TARGET_ID: usize = 1;
const NIOS_PKT_8x64_IDX_FLAGS: usize = 2;
const NIOS_PKT_8x64_IDX_RESV1: usize = 3;
const NIOS_PKT_8x64_IDX_ADDR: usize = 4;
const NIOS_PKT_8x64_IDX_DATA: usize = 5;
const NIOS_PKT_8x64_IDX_RESV2: usize = 13;
const NIOS_PKT_8x64_FLAG_WRITE: u8 = 1;

pub fn pack_8x64(target: u8, write: bool, addr: u8, data: u64) -> [u8; 16] {
    let mut buf = [0; 16];
    
    buf[NIOS_PKT_8x64_IDX_MAGIC] = NIOS_PKT_8x64_MAGIC;
    buf[NIOS_PKT_8x64_IDX_TARGET_ID] = target;

    if write {
        buf[NIOS_PKT_8x64_IDX_FLAGS] = NIOS_PKT_8x64_FLAG_WRITE;
    }

    buf[NIOS_PKT_8x64_IDX_ADDR] = addr;

    buf[NIOS_PKT_8x64_IDX_DATA] = data as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 1] = (data >> 8) as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 2] = (data >> 16) as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 3] = (data >> 24) as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 4] = (data >> 32) as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 5] = (data >> 40) as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 6] = (data >> 48) as u8;
    buf[NIOS_PKT_8x64_IDX_DATA + 7] = (data >> 56) as u8;
    
    buf
}

pub fn unpack_8x64(packet: &[u8]) -> Result<(u8,bool,u8,u64)>{
    if packet[NIOS_PKT_8x64_IDX_MAGIC] == NIOS_PKT_8x64_MAGIC {
        let target = packet[NIOS_PKT_8x64_IDX_TARGET_ID];
        let write = packet[NIOS_PKT_8x64_IDX_FLAGS] & NIOS_PKT_8x64_FLAG_WRITE != 0;
        let addr = packet[NIOS_PKT_8x64_IDX_ADDR];

        let data = (packet[NIOS_PKT_8x64_IDX_DATA] as u64) |
            (packet[NIOS_PKT_8x64_IDX_DATA + 1] as u64) << 8 |
            (packet[NIOS_PKT_8x64_IDX_DATA + 2] as u64) << 16 |
            (packet[NIOS_PKT_8x64_IDX_DATA + 3] as u64) << 24 |
            (packet[NIOS_PKT_8x64_IDX_DATA + 4] as u64) << 32 |
            (packet[NIOS_PKT_8x64_IDX_DATA + 5] as u64) << 40 |
            (packet[NIOS_PKT_8x64_IDX_DATA + 6] as u64) << 48 |
            (packet[NIOS_PKT_8x64_IDX_DATA + 7] as u64) << 56;

        Ok((target,write,addr,data))
    }else {
        Err(anyhow::anyhow!("Invalid packet magic"))
    }
}