use anyhow::Result;

const NIOS_PKT_8x16_MAGIC: u8 = 'B' as u8;

/* Request packet indices */
const NIOS_PKT_8x16_IDX_MAGIC: usize = 0;
const NIOS_PKT_8x16_IDX_TARGET_ID: usize = 1;
const NIOS_PKT_8x16_IDX_FLAGS: usize = 2;
//const NIOS_PKT_8x16_IDX_RESV1: usize = 3;
const NIOS_PKT_8x16_IDX_ADDR: usize = 4;
const NIOS_PKT_8x16_IDX_DATA: usize = 5;
//const NIOS_PKT_8x16_IDX_RESV2: usize = 7;

pub fn pack_8x16(target: u8, write: bool,
                 addr: u8, data: u16) -> [u8; 16]
{
    let mut buf = [0; 16];
    buf[NIOS_PKT_8x16_IDX_MAGIC] = NIOS_PKT_8x16_MAGIC;
    buf[NIOS_PKT_8x16_IDX_TARGET_ID] = target;

    if (write) {
        buf[NIOS_PKT_8x16_IDX_FLAGS] = 1;
    }

    buf[NIOS_PKT_8x16_IDX_ADDR] = addr;

    buf[NIOS_PKT_8x16_IDX_DATA] = (data & 0xff) as u8;
    buf[NIOS_PKT_8x16_IDX_DATA + 1] = (data >> 8) as u8;
    return buf;
}

pub fn unpack_8x16(packet: &[u8]) -> Result<(u8, bool, u8, u16, bool)> {
    if packet[NIOS_PKT_8x16_IDX_MAGIC] == NIOS_PKT_8x16_MAGIC {
        let target = packet[NIOS_PKT_8x16_IDX_TARGET_ID];
        let write = packet[NIOS_PKT_8x16_IDX_FLAGS] & 1 as u8 != 0;
        let success = packet[NIOS_PKT_8x16_IDX_FLAGS] & 0 as u8 != 0;
        let addr = packet[NIOS_PKT_8x16_IDX_ADDR];

        let data: u16 = (packet[NIOS_PKT_8x16_IDX_DATA]) as u16 |
            (packet[NIOS_PKT_8x16_IDX_DATA + 1] as u16) << 8;

        Ok((target, write, addr, data, success))
    } else {
        Err(anyhow::anyhow!("Invalid packet magic"))
    }
}