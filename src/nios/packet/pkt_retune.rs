const NIOS_PKT_RETUNE_IDX_MAGIC: usize = 0;
const NIOS_PKT_RETUNE_IDX_TIME: usize = 1;
const NIOS_PKT_RETUNE_IDX_INTFRAC: usize = 9;
const NIOS_PKT_RETUNE_IDX_FREQSEL: usize = 13;
const NIOS_PKT_RETUNE_IDX_BANDSEL: usize = 14;
const NIOS_PKT_RETUNE_IDX_RESV: usize = 15;
const NIOS_PKT_RETUNE_MAGIC: u8 = 'T' as u8;

const NIOS_PKT_RETUNERESP_IDX_VCOCAP: usize = 9;
const NIOS_PKT_RETUNERESP_IDX_FLAGS: usize = 10;


const FLAG_QUICK_TUNE: u8 = (1 << 6);
const FLAG_RX: u8 = (1 << 6);
const FLAG_TX: u8 = (1 << 7);
const FLAG_LOW_BAND: u8 = (1 << 7);

pub fn pack_retune(
    module: u8,
    timestamp: u64,
    nint: u16,
    nfrac: u32,
    freqsel: u8,
    vcocap: u8,
    low_band: bool,
    xb_gpio: u8,
    quick_tune: bool) -> [u8; 16]
{
    let mut buf = [0; 16];

    buf[NIOS_PKT_RETUNE_IDX_MAGIC] = NIOS_PKT_RETUNE_MAGIC;

    buf[NIOS_PKT_RETUNE_IDX_TIME + 0] = (timestamp & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 1] = ((timestamp >> 8) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 2] = ((timestamp >> 16) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 3] = ((timestamp >> 24) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 4] = ((timestamp >> 32) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 5] = ((timestamp >> 40) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 6] = ((timestamp >> 48) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_TIME + 7] = ((timestamp >> 56) & 0xff) as u8;

    buf[NIOS_PKT_RETUNE_IDX_INTFRAC + 0] = ((nint >> 1) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_INTFRAC + 1] = ((nint & 0x1) << 7) as u8;
    buf[NIOS_PKT_RETUNE_IDX_INTFRAC + 1] |= ((nfrac >> 16) & 0x7f) as u8;
    buf[NIOS_PKT_RETUNE_IDX_INTFRAC + 2] = ((nfrac >> 8) & 0xff) as u8;
    buf[NIOS_PKT_RETUNE_IDX_INTFRAC + 3] = (nfrac & 0xff) as u8;

    buf[NIOS_PKT_RETUNE_IDX_FREQSEL] = freqsel & 0xff;

    match (module) {
        1 => {
            buf[NIOS_PKT_RETUNE_IDX_FREQSEL] |= FLAG_TX;
        }

        0 => {
            buf[NIOS_PKT_RETUNE_IDX_FREQSEL] |= FLAG_RX;
        }

        _ => {}
    }

    if (low_band) {
        buf[NIOS_PKT_RETUNE_IDX_BANDSEL] = FLAG_LOW_BAND;
    } else {
        buf[NIOS_PKT_RETUNE_IDX_BANDSEL] = 0x00;
    }

    if (quick_tune) {
        buf[NIOS_PKT_RETUNE_IDX_BANDSEL] |= FLAG_QUICK_TUNE;
    }

    buf[NIOS_PKT_RETUNE_IDX_BANDSEL] |= vcocap;

    buf[NIOS_PKT_RETUNE_IDX_RESV] = xb_gpio;

    return buf;
}

const NIOS_PKT_RETUNE2_IDX_MAGIC: usize = 0;
const NIOS_PKT_RETUNE2_IDX_TIME: usize = 1;
const NIOS_PKT_RETUNE2_IDX_NIOS_PROFILE: usize = 9;
const NIOS_PKT_RETUNE2_IDX_RFFE_PROFILE: usize = 11;
const NIOS_PKT_RETUNE2_IDX_RFFE_PORT: usize = 12;
const NIOS_PKT_RETUNE2_IDX_SPDT: usize = 13;
const NIOS_PKT_RETUNE2_IDX_RESV: usize = 14;

const NIOS_PKT_RETUNE2_MAGIC: u8 = 'U' as u8;

const NIOS_PKT_RETUNE2_RESP_IDX_TIME: usize = 1;
const NIOS_PKT_RETUNE2_RESP_IDX_FLAGS: usize = 9;

pub fn pack_retune2(
    module: u8,
    timestamp: u64,
    nios_profile: u16,
    rffe_profile: u8,
    port: u8,
    spdt: u8) -> [u8; 16]
{
    let mut buf: [u8; 16] = [0; 16];
    let mut pkt_port: u8;

    let retune_mask = 0x1 << 7;
    /* Clear the IS_RX bit of the port parameter */
    pkt_port = port & (!retune_mask);

    /* Set the IS_RX bit (if needed) */
    if (pkt_port != 0) | (module == 1) {
        pkt_port = 0;
    } else {
        pkt_port = retune_mask;
    }

    buf[NIOS_PKT_RETUNE2_IDX_MAGIC] = NIOS_PKT_RETUNE2_MAGIC;

    buf[NIOS_PKT_RETUNE2_IDX_TIME + 0] = (timestamp >> 0) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 1] = (timestamp >> 8) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 2] = (timestamp >> 16) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 3] = (timestamp >> 24) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 4] = (timestamp >> 32) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 5] = (timestamp >> 40) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 6] = (timestamp >> 48) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_TIME + 7] = (timestamp >> 56) as u8;

    buf[NIOS_PKT_RETUNE2_IDX_NIOS_PROFILE + 0] = (nios_profile >> 0) as u8;
    buf[NIOS_PKT_RETUNE2_IDX_NIOS_PROFILE + 1] = (nios_profile >> 8) as u8;

    buf[NIOS_PKT_RETUNE2_IDX_RFFE_PROFILE] = rffe_profile & 0xff;

    buf[NIOS_PKT_RETUNE2_IDX_RFFE_PORT] = pkt_port;

    buf[NIOS_PKT_RETUNE2_IDX_SPDT] = spdt & 0xff;

    buf[NIOS_PKT_RETUNE2_IDX_RESV + 0] = 0x00;
    buf[NIOS_PKT_RETUNE2_IDX_RESV + 1] = 0x00;

    return buf;
}

pub fn unpack_retune(buf: &[u8]) -> (u64, u8, u8)
{
    let mut duration: u64 = buf[NIOS_PKT_RETUNE_IDX_TIME] as u64;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 1] as u64) << 8;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 2] as u64) << 16;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 3] as u64) << 24;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 4] as u64) << 32;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 5] as u64) << 40;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 6] as u64) << 48;
    duration |= (buf[NIOS_PKT_RETUNE_IDX_TIME + 7] as u64) << 56;

    let vcocap = buf[NIOS_PKT_RETUNERESP_IDX_VCOCAP];
    let flags = buf[NIOS_PKT_RETUNERESP_IDX_FLAGS];

    return (duration, vcocap, flags);
}

pub fn unpack_retune2(buf: &[u8]) -> (u64, u8)
{
    let mut duration: u64 = buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 0] as u64;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 1] as u64) << 8;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 2] as u64) << 16;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 3] as u64) << 24;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 4] as u64) << 32;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 5] as u64) << 40;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 6] as u64) << 48;
    duration |= (buf[NIOS_PKT_RETUNE2_RESP_IDX_TIME + 7] as u64) << 56;

    let flags = buf[NIOS_PKT_RETUNE2_RESP_IDX_FLAGS];

    return (duration, flags);
}