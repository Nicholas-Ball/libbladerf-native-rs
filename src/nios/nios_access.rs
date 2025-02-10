use std::thread::sleep;
use std::time::Duration;
use crate::nios::*;
use crate::usb::{bulk_transfer_in, bulk_transfer_out};
use crate::{BladerfDirection, BladerfVersion, Device};
use anyhow::{Error, Result};
use packet::{pkt_16x64, pkt_32x32, pkt_8x16, pkt_8x32, pkt_8x64, pkt_8x8, pkt_retune};

pub async fn nios_access(
    dev: &Device,
    buf: &[u8; 16],
) -> Result<[u8; 16]> {
    /* Send the command */
    bulk_transfer_out::<0x02>(dev, buf).await?;

    /* Retrieve the request */
    let out = bulk_transfer_in::<0x82, 16>(dev).await?;
    Ok(out)
}

pub async fn nios_8x8_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u8,
) -> Result<u8> {
    let buf = pkt_8x8::pack_8x8(id, false, addr, 0);

    nios_access(dev, &buf).await?;

    let out = pkt_8x8::unpack_8x8(&buf)?.3;

    Ok(out)
}

pub async fn nios_8x8_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u8,
    data: u8,
) -> Result<u8> {
    let buf = pkt_8x8::pack_8x8(id, true, addr, data);

    nios_access(dev, &buf).await?;

    let out = pkt_8x8::unpack_8x8(&buf)?.3;

    Ok(out)
}

pub async fn nios_8x16_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u8,
) -> Result<u16> {
    let buf = pkt_8x16::pack_8x16(id, false, addr, 0);

    nios_access(dev, &buf).await?;

    let out = pkt_8x16::unpack_8x16(&buf)?.3;

    Ok(out)
}

pub async fn nios_8x16_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u8,
    data: u16,
) -> Result<u16> {
    let buf = pkt_8x16::pack_8x16(id, true, addr, data);

    nios_access(dev, &buf).await?;

    let out = pkt_8x16::unpack_8x16(&buf)?.3;

    Ok(out)
}

pub async fn nios_8x32_read(
    dev: &Device,
    id: u8,
    addr: u8,
) -> Result<u32> {
    let buf = pkt_8x32::pack_8x32(id, false, addr, 0);

    nios_access(dev, &buf).await?;

    let out = pkt_8x32::unpack_8x32(&buf)?.3;

    Ok(out)
}

pub async fn nios_8x32_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u8,
    data: u32,
) -> Result<u32> {
    let buf = pkt_8x32::pack_8x32(id, true, addr, data);

    nios_access(dev, &buf).await?;

    let out = pkt_8x32::unpack_8x32(&buf)?.3;

    Ok(out)
}

pub async fn nios_16x64_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u16,
) -> Result<u64> {
    let buf = pkt_16x64::pack_16x64(id, false, addr, 0);

    nios_access(dev, &buf).await?;

    let out = pkt_16x64::unpack_16x64(&buf)?.3;

    Ok(out)
}

pub async fn nios_16x64_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u16,
    data: u64,
) -> Result<u64> {
    let buf = pkt_16x64::pack_16x64(id, true, addr, data);

    nios_access(dev, &buf).await?;

    let out = pkt_16x64::unpack_16x64(&buf)?.3;

    Ok(out)
}

pub async fn nios_32x32_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u32,
) -> Result<u32> {
    let buf = pkt_32x32::pack_32x32(id, false, addr, 0);

    nios_access(dev, &buf).await?;

    let out = pkt_32x32::unpack_32x32(&buf)?.3;

    Ok(out)
}

pub async fn nios_32x32_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    id: u8,
    addr: u32,
    data: u32,
) -> Result<u32> {
    let buf = pkt_32x32::pack_32x32(id, true, addr, data);

    nios_access(dev, &buf).await?;

    let out = pkt_32x32::unpack_32x32(&buf)?.3;

    Ok(out)
}

pub async fn nios_32x32_masked_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    id: u8,
    mask: u32,
) -> Result<u32> {
    let buf = pkt_32x32::pack_32x32(id, false, mask, 0);

    nios_access(dev, &buf).await?;

    let out = pkt_32x32::unpack_32x32(&buf)?.3;

    Ok(out)
}

pub async fn nios_32x32_masked_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    id: u8,
    mask: u32,
    val: u32,
) -> Result<u32> {
    let buf = pkt_32x32::pack_32x32(id, true, mask, val);

    nios_access(dev, &buf).await?;

    let out = pkt_32x32::unpack_32x32(&buf)?.3;

    Ok(out)
}

pub async fn nios_config_read<const endpoint: u8>(
    dev: &Device,
) -> Result<u32> {
    let out = nios_8x32_read(dev, 1, 0).await?;

    Ok(out)
}

pub async fn nios_config_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    val: u32,
) -> Result<u32> {
    let out = nios_8x32_write::<in_len, out_len, endpoint>(dev, 1, 0, val).await?;
    Ok(out)

    //log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, val);
}

pub async fn nios_get_fpga_version(
    dev: &Device,
) -> Result<BladerfVersion> {
    let regval: u32 = nios_8x32_read(dev, 0, 0).await?;

    Ok(BladerfVersion {
        major: ((regval >> 24) & 0xff) as u8,
        minor: ((regval >> 16) & 0xff) as u8,
        patch: (regval & 0xffff) as u8,
    })
}

pub async fn nios_get_timestamp<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    dir: BladerfDirection,
) -> Result<u64> {
    let addr = match dir {
        BladerfDirection::RX => 0,

        BladerfDirection::TX => 1,
    };

    let buf = pkt_8x64::pack_8x64(0, false, addr, 0);

    nios_access(dev, &buf).await?;

    let timestamp = pkt_8x64::unpack_8x64(&buf)?.3;

    Ok(timestamp)
}

pub async fn nios_si5338_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
) -> Result<u8> {
    let out = nios_8x8_read::<in_len, 15, endpoint>(dev, 1, addr).await?;
    Ok(out)
}

pub async fn nios_si5338_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
    data: u8,
) -> Result<u8> {
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, 1, addr, data).await?;

    Ok(out)
}

pub async fn nios_lms6_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
) -> Result<u8> {
    let out = nios_8x8_read::<in_len, 15, endpoint>(dev, 0, addr).await?;
    Ok(out)
}

pub async fn nios_lms6_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
    data: u8,
) -> Result<u8> {
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, 0, addr, data).await?;

    Ok(out)
}

pub async fn nios_ina219_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
) -> Result<u16> {
    let out = nios_8x16_read::<in_len, 16, endpoint>(dev, 4, addr).await?;

    Ok(out)
}

pub async fn nios_ina219_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
    data: u16,
) -> Result<u16> {
    let out = nios_8x16_write::<in_len, 16, endpoint>(dev, 4, addr, data).await?;

    Ok(out)
}

pub async fn nios_ad9361_spi_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    cmd: u16,
) -> Result<u64> {
    let out = nios_16x64_read::<in_len, 16, endpoint>(dev, 0, cmd).await?;

    Ok(out)
}

pub async fn nios_ad9361_spi_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    cmd: u16,
    data: u64,
) -> Result<u64> {
    let out = nios_16x64_write::<in_len, 16, endpoint>(dev, 0, cmd, data).await?;

    Ok(out)
}

pub async fn nios_adi_axi_read<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u32,
) -> Result<u32> {
    let out = nios_32x32_read::<in_len, 16, endpoint>(dev, 2, addr).await?;

    Ok(out)
}

pub async fn nios_adi_axi_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u32,
    data: u32,
) -> Result<u32> {
    let out = nios_32x32_write::<in_len, out_len, endpoint>(dev, 2, addr, data).await?;

    Ok(out)
}

pub async fn nios_wishbone_master_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    addr: u32,
) -> Result<u32> {
    let out = nios_32x32_read::<in_len, 16, endpoint>(dev, 3, addr).await?;

    Ok(out)
}

pub async fn nios_wishbone_master_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    addr: u32,
    data: u32,
) -> Result<u32> {
    let out = nios_32x32_write::<in_len, out_len, endpoint>(dev, 3, addr, data).await?;

    Ok(out)
}

pub async fn nios_rfic_command_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    cmd: u16,
) -> Result<u64> {
    let out = nios_16x64_read::<in_len, 16, endpoint>(dev, 1, cmd).await?;

    Ok(out)
}

pub async fn nios_rfic_command_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    cmd: u16,
    data: u64,
) -> Result<u64> {
    let out = nios_16x64_write::<in_len, 16, endpoint>(dev, 1, cmd, data).await?;

    Ok(out)
}

pub async fn nios_rffe_control_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
) -> Result<u32> {
    let out = nios_8x32_read(dev, 3, 0).await?;

    Ok(out)
}

pub async fn nios_rffe_control_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    value: u32,
) -> Result<u32> {
    let out = nios_8x32_write::<in_len, 16, endpoint>(dev, 3, 0, value).await?;

    Ok(out)
}

pub async fn nios_rffe_fastlock_save<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    is_tx: bool,
    rffe_profile: u8,
    nios_profile: u16,
) -> Result<u32> {
    let mut addr: u8 = 0;
    let data: u32 = ((rffe_profile as u32) << 16) | nios_profile as u32;

    if is_tx {
        addr = 1;
    }

    let out = nios_8x32_write::<in_len, 16, endpoint>(dev, 5, addr, data).await?;

    Ok(out)
}

pub async fn nios_ad56x1_vctcxo_trim_dac_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
) -> Result<u16> {
    let out = nios_8x16_read::<in_len, 16, endpoint>(dev, 3, 0).await?;

    Ok(out)
}

pub async fn nios_ad56x1_vctcxo_trim_dac_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    value: u16,
) -> Result<u16> {
    let out = nios_8x16_write::<in_len, 16, endpoint>(dev, 3, 0, value).await?;

    Ok(out)
}

pub async fn nios_adf400x_read<const endpoint: u8>(
    dev: &Device,
    addr: u8,
) -> Result<u32> {
    let out = nios_8x32_read(dev, 4, addr).await?;

    Ok(out)
}

pub async fn nios_adf400x_write<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    addr: u8,
    mut data: u32,
) -> Result<u32> {
    data &= !0x3;

    let out =
        nios_8x32_write::<in_len, 16, endpoint>(dev, 4, 0, data | (addr as u32 & 0x3)).await?;

    Ok(out)
}

pub async fn nios_vctcxo_trim_dac_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    addr: u8,
    value: u16,
) -> Result<u16> {
    let out = nios_8x16_write::<in_len, 16, endpoint>(dev, 0, addr, value).await?;

    Ok(out)
}

pub async fn nios_vctcxo_trim_dac_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    addr: u8,
) -> Result<u16> {
    let out = nios_8x16_read::<in_len, 16, endpoint>(dev, 0, addr).await?;

    Ok(out)
}

pub async fn nios_set_vctcxo_tamer_mode<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    mode: u8,
) -> Result<u8>
//In the original code "mode" was its own type, but it seems to just be a fancy u8 when all's said and done.
{
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, 2, 0xff, mode).await?;

    Ok(out)
}

pub async fn nios_get_vctcxo_tamer_mode<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
) -> Result<u8> {
    let mode_detected = nios_8x8_read::<in_len, 15, endpoint>(dev, 2, 0xff).await?;
    let mode = match mode_detected {
        0..=2 => mode_detected,
        _ => return Err(Error::msg("Error -1: Unexpected mode detected")), //If it's not one of these - bail!
    };
    Ok(mode)
}

pub async fn nios_get_iq_gain_correction<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    ch: BladerfDirection,
) -> Result<u16> {
    let tmp: u16 = match ch {
        BladerfDirection::RX => nios_8x16_read::<in_len, 16, endpoint>(dev, 1, 0).await?,

        BladerfDirection::TX => nios_8x16_read::<in_len, 16, endpoint>(dev, 1, 2).await?,
    };

    Ok(tmp)
}

pub async fn nios_get_iq_phase_correction<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    ch: BladerfDirection,
) -> Result<u16> {
    let tmp: u16 = match ch {
        BladerfDirection::RX => nios_8x16_read::<in_len, 16, endpoint>(dev, 1, 1).await?,

        BladerfDirection::TX => nios_8x16_read::<in_len, 16, endpoint>(dev, 1, 3).await?,
    };

    Ok(tmp)
}

pub async fn nios_set_iq_gain_correction<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    ch: BladerfDirection,
    value: i16,
) -> Result<u16> {
    let tmp = match ch {
        BladerfDirection::RX => {
            nios_8x16_write::<in_len, 16, endpoint>(dev, 1, 0, value as u16).await?
        }

        BladerfDirection::TX => {
            nios_8x16_write::<in_len, 16, endpoint>(dev, 1, 2, value as u16).await?
        }
    };

    Ok(tmp)
}

pub async fn nios_set_iq_phase_correction<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    ch: BladerfDirection,
    value: i16,
) -> Result<u16> {
    let tmp = match ch {
        BladerfDirection::RX => {
            nios_8x16_write::<in_len, 16, endpoint>(dev, 1, 1, value as u16).await?
        }

        BladerfDirection::TX => {
            nios_8x16_write::<in_len, 16, endpoint>(dev, 1, 3, value as u16).await?
        }
    };

    Ok(tmp)
}

pub async fn nios_set_agc_dc_correction<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    q_max: i16,
    i_max: i16,
    q_mid: i16,
    i_mid: i16,
    q_low: i16,
    i_low: i16,
) -> Result<u16> {
    nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 0, q_max as u16).await?;
    nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 1, i_max as u16).await?;
    nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 2, q_mid as u16).await?;
    nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 3, i_mid as u16).await?;
    nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 4, q_low as u16).await?;
    let tmp = nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 5, i_low as u16).await?;

    Ok(tmp)
}

pub async fn nios_xb200_synth_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    value: u32,
) -> Result<u32> {
    let out = nios_8x32_write::<in_len, 16, endpoint>(dev, 2, 0, value).await?;
    Ok(out)
}

pub async fn nios_expansion_gpio_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
) -> Result<u32> {
    let out = nios_32x32_masked_read::<in_len, 16, endpoint>(dev, 0, 0xffffffff).await?;
    Ok(out)
}

pub async fn nios_expansion_gpio_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    mask: u32,
    val: u32,
) -> Result<u32> {
    let out = nios_32x32_masked_write::<in_len, 16, endpoint>(dev, 0, mask, val).await?;
    Ok(out)
}

pub async fn nios_expansion_gpio_dir_read<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
) -> Result<u32> {
    let out = nios_32x32_masked_read::<in_len, 16, endpoint>(dev, 1, 0xffffffff).await?;
    Ok(out)
}

pub async fn nios_expansion_gpio_dir_write<
    const in_len: usize,
    const out_len: usize,
    const endpoint: u8,
>(
    dev: &Device,
    mask: u32,
    val: u32,
) -> Result<u32> {
    let out = nios_32x32_masked_write::<in_len, 16, endpoint>(dev, 1, mask, val).await?;
    Ok(out)
}

pub async fn nios_retune<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    ch: u8,
    timestamp: u64,
    nint: u16,
    nfrac: u32,
    freqsel: u8,
    vcocap: u8,
    low_band: bool,
    xb_gpio: u8,
    quick_tune: bool,
) -> Result<u64> {
    let buf = pkt_retune::pack_retune(
        ch, timestamp, nint, nfrac, freqsel, vcocap, low_band, xb_gpio, quick_tune,
    );

    nios_access(dev, &buf).await?;

    let out = pkt_retune::unpack_retune(&buf);

    if out.2 & 0b00000010 == 0 {
        let err = if out.0 == 0 {
            Error::msg("Error -1: FPGA tuning reported failure.")
        } else {
            Error::msg("Error -15: BladeRF retune queue is full. Try again later.")
        };
        return Err(err);
    }

    Ok(out.0)
}

pub async fn nios_retune2<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    ch: u8,
    timestamp: u64,
    nios_profile: u16,
    rffe_profile: u8,
    port: u8,
    spdt: u8,
) -> Result<u64> {
    let buf = pkt_retune::pack_retune2(ch, timestamp, nios_profile, rffe_profile, port, spdt);

    nios_access(dev, &buf).await?;

    let out = pkt_retune::unpack_retune2(&buf);

    if out.1 & 0b00000010 == 0 {
        let err = if out.0 == 0 {
            Error::msg("Error -1: FPGA tuning reported failure.")
        } else {
            Error::msg("Error -15: BladeRF retune queue is full. Try again later.")
        };
        return Err(err);
    }
    Ok(out.0)
}

pub async fn nios_read_trigger<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    ch: BladerfDirection,
    trigger: u8,
) -> Result<u8> {
    let nios_id: u8 = match ch {
        BladerfDirection::TX => 3,

        BladerfDirection::RX => 4,
    };

    /* Only 1 external trigger is currently supported */
    match (trigger) {
        0..=2 => {}

        _ => return Err(Error::msg("Error -1: Invalid Trigger.")),
    }

    let out = nios_8x8_read::<in_len, 15, endpoint>(dev, nios_id, 0).await?;

    Ok(out)
}

pub async fn nios_write_trigger<const in_len: usize, const out_len: usize, const endpoint: u8>(
    dev: &Device,
    ch: BladerfDirection,
    trigger: u8,
    value: u8,
) -> Result<u8> {
    let nios_id: u8 = match ch {
        BladerfDirection::TX => 3,

        BladerfDirection::RX => 4,
    };

    if trigger > 2 {
        return Err(Error::msg("Error -1: Invalid Trigger."));
    }

    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, nios_id, 0, value).await?;

    Ok(out)
}
