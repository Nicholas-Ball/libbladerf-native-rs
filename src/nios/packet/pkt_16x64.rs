use anyhow::Result;

const NIOS_PKT_16x64_MAGIC: u8 = b'E';

/* Request packet indices */
const NIOS_PKT_16x64_IDX_MAGIC: usize = 0;
const NIOS_PKT_16x64_IDX_TARGET_ID: usize = 1;
const NIOS_PKT_16x64_IDX_FLAGS: usize = 2;
const NIOS_PKT_16x64_IDX_RESV1: usize = 3;
const NIOS_PKT_16x64_IDX_ADDR: usize = 4;
const NIOS_PKT_16x64_IDX_DATA: usize = 6;
const NIOS_PKT_16x64_IDX_RESV2: usize = 14;


const NIOS_PKT_16x64_FLAG_WRITE: u8 = 1;
const NIOS_PKT_16x64_FLAG_SUCCESS: u8 = 2;


pub fn pack_16x64(target: u8, write: bool, addr: u16, data: u64) -> [u8; 16] {
    let mut buf = [0; 16];

    buf[NIOS_PKT_16x64_IDX_MAGIC] = NIOS_PKT_16x64_MAGIC;
    buf[NIOS_PKT_16x64_IDX_TARGET_ID] = target;

    if write {
        buf[NIOS_PKT_16x64_IDX_FLAGS] = NIOS_PKT_16x64_FLAG_WRITE as u8;
    }

    buf[NIOS_PKT_16x64_IDX_ADDR] = addr as u8;
    buf[NIOS_PKT_16x64_IDX_ADDR + 1] = (addr >> 8) as u8;

    buf[NIOS_PKT_16x64_IDX_DATA] = data as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 1] = (data >> 8) as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 2] = (data >> 16) as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 3] = (data >> 24) as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 4] = (data >> 32) as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 5] = (data >> 40) as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 6] = (data >> 48) as u8;
    buf[NIOS_PKT_16x64_IDX_DATA + 7] = (data >> 56) as u8;

    buf
}

pub fn unpack_16x64(packet: &[u8]) -> Result<(u8, bool, u16, u64, bool)> {
    if packet[NIOS_PKT_16x64_IDX_MAGIC] == NIOS_PKT_16x64_MAGIC {
        let target = packet[NIOS_PKT_16x64_IDX_TARGET_ID];
        let write = packet[NIOS_PKT_16x64_IDX_FLAGS] & NIOS_PKT_16x64_FLAG_WRITE as u8 != 0;
        let success = packet[NIOS_PKT_16x64_IDX_FLAGS] & NIOS_PKT_16x64_FLAG_SUCCESS as u8 != 0;
        let addr = u16::from_le_bytes([packet[NIOS_PKT_16x64_IDX_ADDR], packet[NIOS_PKT_16x64_IDX_ADDR + 1]]);

        let data = (packet[NIOS_PKT_16x64_IDX_DATA] as u64) |
            (packet[NIOS_PKT_16x64_IDX_DATA + 1] as u64) << 8 |
            (packet[NIOS_PKT_16x64_IDX_DATA + 2] as u64) << 16 |
            (packet[NIOS_PKT_16x64_IDX_DATA + 3] as u64) << 24 |
            (packet[NIOS_PKT_16x64_IDX_DATA + 4] as u64) << 32 |
            (packet[NIOS_PKT_16x64_IDX_DATA + 5] as u64) << 40 |
            (packet[NIOS_PKT_16x64_IDX_DATA + 6] as u64) << 48 |
            (packet[NIOS_PKT_16x64_IDX_DATA + 7] as u64) << 56;

        Ok((target, write, addr, data, success))
    } else {
        Err(anyhow::anyhow!("Invalid packet magic"))
    }
}

pub fn pack_16x64_resp_data(target: u8, write: bool, addr: u16, data: u64) -> [u8; 16] {
    let mut pkt = pack_16x64(target, write, addr, data);

    pkt[NIOS_PKT_16x64_IDX_FLAGS] |= NIOS_PKT_16x64_FLAG_SUCCESS;

    pkt
}

pub fn unpack_16x64_resp_data(packet: &[u8]) -> Result<(u8, bool, u16, u64)> {
    let (target, write, addr, data, success) = unpack_16x64(packet)?;

    if success {
        Ok((target, write, addr, data))
    } else {
        Err(anyhow::anyhow!("Packet failed"))
    }
}
