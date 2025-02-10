use anyhow::Result;

const NIOS_PKT_32x32_MAGIC: u8 = b'K';

/* Request packet indices */
const NIOS_PKT_32x32_IDX_MAGIC: usize = 0;
const NIOS_PKT_32x32_IDX_TARGET_ID: usize = 1;
const NIOS_PKT_32x32_IDX_FLAGS: usize = 2;
const NIOS_PKT_32x32_IDX_RESV1: usize = 3;
const NIOS_PKT_32x32_IDX_ADDR: usize = 4;
const NIOS_PKT_32x32_IDX_DATA: usize = 8;
const NIOS_PKT_32x32_IDX_RESV2: usize = 12;


const NIOS_PKT_32x32_FLAG_WRITE: u8 = 1;
const NIOS_PKT_32x32_FLAG_SUCCESS: u8 = 2;

pub fn pack_32x32(target: u8, write: bool, addr: u32, data: u32) -> [u8; 16] {
    let mut buf = [0; 16];

    buf[NIOS_PKT_32x32_IDX_MAGIC] = NIOS_PKT_32x32_MAGIC;
    buf[NIOS_PKT_32x32_IDX_TARGET_ID] = target;

    if write {
        buf[NIOS_PKT_32x32_IDX_FLAGS] = NIOS_PKT_32x32_FLAG_WRITE;
    }

    buf[NIOS_PKT_32x32_IDX_ADDR] = addr as u8;
    buf[NIOS_PKT_32x32_IDX_ADDR + 1] = (addr >> 8) as u8;
    buf[NIOS_PKT_32x32_IDX_ADDR + 2] = (addr >> 16) as u8;
    buf[NIOS_PKT_32x32_IDX_ADDR + 3] = (addr >> 24) as u8;

    buf[NIOS_PKT_32x32_IDX_DATA] = data as u8;
    buf[NIOS_PKT_32x32_IDX_DATA + 1] = (data >> 8) as u8;
    buf[NIOS_PKT_32x32_IDX_DATA + 2] = (data >> 16) as u8;
    buf[NIOS_PKT_32x32_IDX_DATA + 3] = (data >> 24) as u8;

    buf
}

pub fn unpack_32x32(packet: &[u8]) -> Result<(u8, bool, u32, u32, bool)> {
    if packet[NIOS_PKT_32x32_IDX_MAGIC] == NIOS_PKT_32x32_MAGIC {
        let target = packet[NIOS_PKT_32x32_IDX_TARGET_ID];
        let write = packet[NIOS_PKT_32x32_IDX_FLAGS] & NIOS_PKT_32x32_FLAG_WRITE != 0;
        let success = packet[NIOS_PKT_32x32_IDX_FLAGS] & NIOS_PKT_32x32_FLAG_SUCCESS != 0;
        let addr = u32::from_le_bytes([packet[NIOS_PKT_32x32_IDX_ADDR], packet[NIOS_PKT_32x32_IDX_ADDR + 1], packet[NIOS_PKT_32x32_IDX_ADDR + 2], packet[NIOS_PKT_32x32_IDX_ADDR + 3]]);

        let data = (packet[NIOS_PKT_32x32_IDX_DATA] as u32) |
            (packet[NIOS_PKT_32x32_IDX_DATA + 1] as u32) << 8 |
            (packet[NIOS_PKT_32x32_IDX_DATA + 2] as u32) << 16 |
            (packet[NIOS_PKT_32x32_IDX_DATA + 3] as u32) << 24;

        Ok((target, write, addr, data, success))
    } else {
        Err(anyhow::anyhow!("Invalid packet magic"))
    }
}

pub fn pack_32x32_resp(target: u8, write: bool, addr: u32, success: bool, data: u32) -> [u8; 16] {
    let mut buf = pack_32x32(target, write, addr, data);

    if success {
        buf[NIOS_PKT_32x32_IDX_FLAGS] |= NIOS_PKT_32x32_FLAG_SUCCESS;
    }

    buf
}

pub fn unpack_32x32_resp(packet: &[u8]) -> (u8, bool, u32, u32, bool) {
    let unpacked = unpack_32x32(packet).unwrap();

    (unpacked.0, unpacked.1, unpacked.2, unpacked.3, unpacked.4)
}