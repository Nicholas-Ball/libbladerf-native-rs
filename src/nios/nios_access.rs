
use crate::{BladerfDirection, BladerfVersion, Device};
use anyhow::{Error, Result};
use packet::{pkt_8x8, pkt_8x16, pkt_8x32, pkt_8x64, pkt_16x64, pkt_32x32, pkt_retune};
use crate::nios::*;
use crate::usb::{bulk_transfer_in, bulk_transfer_out};

 /*pub fn print_buf(msg: *const char, buf: *const u8, len: u16)
 {
    let mut i: u16;
     if (msg != NULL) {
         fputs(msg, stderr);
     }
 
     for i in 0..len {
         fprintf(stderr, " %02x", buf[i]);
     }
 
     fputc('\n', stderr);
 }*/
 
 /* Buf is assumed to be NIOS_PKT_LEN bytes */
 pub async fn nios_access<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, buf: &[u8; out_len]) -> Result<[u8; in_len]>
 {
     /* Send the command */
     bulk_transfer_out::<endpoint>(dev, buf).await?;
 
     /* Retrieve the request */
     let out = bulk_transfer_in::<endpoint, in_len>(dev).await?;
     Ok(out)
 }
 
 /* Variant that doesn't output to log_error on error.
 pub async fn nios_access_quiet(dev: &mut bladerf, buf: &mut u8)
 {
     let usb: &mut bladerf_usb = dev.backend_data;
 
     print_buf("NIOS II REQ:", buf, NIOS_PKT_LEN);
 
     /* Send the command */
    nios.bulk_transfer(usb.driver, PERIPHERAL_EP_OUT, buf,
                                     NIOS_PKT_LEN, PERIPHERAL_TIMEOUT_MS).await?;
 
     /* Retrieve the request */
     nios.bulk_transfer(usb.driver, PERIPHERAL_EP_IN, buf,
                                     NIOS_PKT_LEN, PERIPHERAL_TIMEOUT_MS).await?;
 
     print_buf("NIOS II res:", buf, NIOS_PKT_LEN);
 } */
 
 pub async fn nios_8x8_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
                          addr: u8) -> Result<u8>
 {
     let buf = pkt_8x8::pack_8x8(id, false, addr, 0);
 
     nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
     let out = pkt_8x8::unpack_8x8(&buf)?.3;

     Ok(out)
 }
 
 pub async fn nios_8x8_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
                           addr: u8, data: u8) -> Result<u8>
 {
    let buf = pkt_8x8::pack_8x8(id, true, addr, data);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_8x8::unpack_8x8(&buf)?.3;

    Ok(out)
 }
 
 pub async fn nios_8x16_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u8) -> Result<u16>
 {
    let buf = pkt_8x16::pack_8x16(id, false, addr, 0);

    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_8x16::unpack_8x16(&buf)?.3;
 
    Ok(out)
 }
 
 pub async fn nios_8x16_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u8, data: u16) -> Result<u16>
 {
    let buf = pkt_8x16::pack_8x16(id, true, addr, data);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_8x16::unpack_8x16(&buf)?.3;

     Ok(out)
 }
 
/*pub fn log_debug_8x32_pkt(buf: *const u8) {
     if (buf == NULL) {
         log_debug("Failed to log packet: packet buffer is NULL\n");
         return;
     }
 
     let target_id: u8 = 0;
     let write: bool = false;
     let addr: u8 = 0;
     let data: u32 = 0;
 
     nios_pkt_8x32_resp_unpack(buf, &target_id, &write, &addr, &data)?;
 
     let operation: *const str = "Read";
     if write {
        operation = "Write";
     }
     let status: *const str = "Success";
 
     log_debug("Packet Magic:      0x%02x\n", buf[NIOS_PKT_8x32_IDX_MAGIC]);
     log_debug("Packet Target:     %s\n", target2str(target_id));
     log_debug("Packet Operation:  %s\n", operation);
     log_debug("Packet Address:    0x%02x\n", addr);
     log_debug("Packet Data:       0x%08x\n", data);
     log_debug("Packet Status:     %s\n", status);
 } */
 
 pub async fn nios_8x32_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u8) -> Result<u32>
 {
    let buf = pkt_8x32::pack_8x32(id, false, addr, 0);

    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_8x32::unpack_8x32(&buf)?.3;
 
    Ok(out)
 }
 
 pub async fn nios_8x32_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u8, data: u32) -> Result<u32>
 {
    let buf = pkt_8x32::pack_8x32(id, true, addr, data);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_8x32::unpack_8x32(&buf)?.3;

     Ok(out)
 }
 
 pub async fn nios_16x64_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u16) -> Result<u64>
 {
    let buf = pkt_16x64::pack_16x64(id, false, addr, 0);

    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_16x64::unpack_16x64(&buf)?.3;
 
    Ok(out)
 
     /*if (NIOS_PKT_16x64_TARGET_RFIC == id) {
         nios_access_quiet(dev, buf).await?;
     } else {
         nios_access(dev, buf).await?;
     }*/
 }
 
 pub async fn nios_16x64_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u16, data: u64) -> Result<u64>
 {
    let buf = pkt_16x64::pack_16x64(id, true, addr, data);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_16x64::unpack_16x64(&buf)?.3;

     Ok(out)
 
     /* RFIC access times out occasionally, and this is fine. 
     if (NIOS_PKT_16x64_TARGET_RFIC == id) {
        nios_access_quiet(dev, buf).await?;
    } else {
        nios_access(dev, buf).await?;
    }*/
 }
 
 pub async fn nios_32x32_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u32) -> Result<u32>
 {
    let buf = pkt_32x32::pack_32x32(id, false, addr, 0);

    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_32x32::unpack_32x32(&buf)?.3;
 
    Ok(out)
 }
 
 pub async fn nios_32x32_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    addr: u32, data: u32) -> Result<u32>
 {
    let buf = pkt_32x32::pack_32x32(id, true, addr, data);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_32x32::unpack_32x32(&buf)?.3;

     Ok(out)
 }
 
 pub async fn nios_32x32_masked_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
                                   mask: u32) -> Result<u32>
 {
    let buf = pkt_32x32::pack_32x32(id, false, mask, 0);
 
     /* The address is used as a mask of bits to read and return */
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_32x32::unpack_32x32(&buf)?.3;
 
    Ok(out)
 }
 
 pub async fn nios_32x32_masked_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, id: u8,
    mask: u32, val: u32) -> Result<u32>
 {
    let buf = pkt_32x32::pack_32x32(id, true, mask, val);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_32x32::unpack_32x32(&buf)?.3;

    Ok(out)
 
 }
 
 pub async fn nios_config_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device) -> Result<u32>
 {
    let out = nios_8x32_read::<in_len, out_len, endpoint>(dev, 1, 0).await?;

    Ok(out)
 }
 
 pub async fn nios_config_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, val: u32) -> Result<u32>
 {
    let out = nios_8x32_write::<in_len, out_len, endpoint>(dev, 1, 0, val).await?;
    Ok(out)

    //log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, val);
 }
 
 pub async fn nios_get_fpga_version<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device) -> Result<BladerfVersion>
 {
    let regval: u32 = nios_8x32_read::<in_len, out_len, endpoint>(dev, 0, 0).await?;
 
    //snprintf(ver.describe, BLADERF_VERSION_STR_MAX, "%d.%d.%d", ver.major, ver.minor, ver.patch);
    Ok(BladerfVersion{
        major: ((regval >> 24) & 0xff) as u8,
        minor: ((regval >> 16) & 0xff) as u8,
        patch: (regval & 0xffff) as u8
    })
 }
 
 pub async fn nios_get_timestamp<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device,
                        dir: BladerfDirection) -> Result<u64>
 {
     let mut addr: u8 = 0;
 
     match (dir){
         BLADERF_RX => {
             addr = 0;
         }
 
         BLADERF_TX => {
             addr = 1;
         }
         _ => {
             //log_debug("Invalid direction: %d\n", dir);
             Error::msg("Error #3: Invalid Direction (Neither RX nor TX)");
         }
     }
 
     let buf = pkt_8x64::pack_8x64(0, false, addr, 0);
 
     nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
     let timestamp = pkt_8x64::unpack_8x64(&buf)?.3;

     Ok(timestamp)
 }
 
 pub async fn nios_si5338_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8) -> Result<u8>
 {
    let out = nios_8x8_read::<in_len, 15, endpoint>(dev, 1, addr).await?;
    Ok(out)
 }
 
 pub async fn nios_si5338_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8, data: u8) -> Result<u8>
 {
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, 1, addr, data).await?;
 
 /*if (ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE){
     if (status == 0) {
         log_verbose("%s: Wrote 0x%02x to addr 0x%02x\n",
                     __FUNCTION__, data, addr);
     }
    }*/

    Ok(out)
 }
 
 pub async fn nios_lms6_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8) -> Result<u8>
 {
    let out = nios_8x8_read::<in_len, 15, endpoint>(dev, 0, addr).await?;
    Ok(out)
 }
 
 pub async fn nios_lms6_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8, data: u8) -> Result<u8>
 {
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, 0, addr, data).await?;
 
 /*if(ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE){
     if (status == 0) {
         log_verbose("%s: Wrote 0x%02x to addr 0x%02x\n",
                     __FUNCTION__, data, addr);
     }
    }*/ //Nonuka miyayi ku paci ai noktes sohayi katu

    Ok(out)
 }
 
 pub async fn nios_ina219_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8) -> Result<u16>
 {
    let out = nios_8x16_read::<in_len, 16, endpoint>(dev, 4, addr).await?;
    Ok(out)
 }
 
 pub async fn nios_ina219_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8, data: u16) -> Result<u16>
 {
    let out = nios_8x16_write::<in_len, 16, endpoint>(dev, 4, addr, data).await?;
    Ok(out)
 }
 
 //const VERBOSE_OUT_SINGLEBYTE: str = "%s: %s 0x%02x @ addr 0x%04x\n"; //TODO: literally everything involving string operations 'cause I don't think they're that important at the moment
 //const VERBOSE_OUT_MULTIBYTE: str  = "%s: %s 0x%02x @ addr 0x%04x (%d/%d)\n"; //TODO RESOLVED: We don't need the damn things lol
 //Pou kasa duamou (A xiela sovexi ki kuaduela pasa u mudu soyidu xiexi ji iasa i fika maixidu)
 
 pub async fn nios_ad9361_spi_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, cmd: u16) -> Result<u64>
 {
    let out = nios_16x64_read::<in_len, 16, endpoint>(dev, 0, cmd).await?;
 
 /*if ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE {
     if (log_get_verbosity() == BLADERF_LOG_LEVEL_VERBOSE && status == 0) {
         size_t bytes = (((cmd >> 12) & 0x7) + 1);
         size_t addr  = cmd & 0xFFF;
 
         if (bytes > 1) {
             size_t i;
             for (i = 0; i < bytes; ++i) {
                 uint8_t byte = (*data >> (56 - 8 * i)) & 0xFF;
                 log_verbose(VERBOSE_OUT_MULTIBYTE, "ad9361_spi", " MRead", byte,
                             addr - i, i + 1, bytes);
             }
         } else {
             uint8_t byte = (*data >> 56) & 0xFF;
             log_verbose(VERBOSE_OUT_SINGLEBYTE, "ad9361_spi", "  Read", byte,
                         addr);
         }
     }
    }*/ //look to be honest we can deal with this later; it's all debug code. apeitaduasi koladasi kaladuasi
    Ok(out)
 }
 
 pub async fn nios_ad9361_spi_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, cmd: u16, data: u64) -> Result<u64>
 {
    let out = nios_16x64_write::<in_len, 16, endpoint>(dev, 0, cmd, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (log_get_verbosity() == BLADERF_LOG_LEVEL_VERBOSE && status == 0) {
         size_t bytes = (((cmd >> 12) & 0x7) + 1) & 0xFF;
         size_t addr  = cmd & 0xFFF;
 
         if (bytes > 1) {
             size_t i;
             for (i = 0; i < bytes; ++i) {
                 uint8_t byte = (data >> (56 - 8 * i)) & 0xFF;
                 log_verbose(VERBOSE_OUT_MULTIBYTE, "ad9361_spi", "MWrite", byte,
                             addr - i, i + 1, bytes);
             }
         } else {
             uint8_t byte = (data >> 56) & 0xFF;
             log_verbose(VERBOSE_OUT_SINGLEBYTE, "ad9361_spi", " Write", byte,
                         addr);
         }
     }
 #endif*/ //same business here. Tu kaptux ex es vuakix sed seciyu koplekteix kai meia fuit mi ieveneyit ak noci duktavit mi sohu videvu te

    Ok(out)
 }
 
 pub async fn nios_adi_axi_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u32) -> Result<u32>
 {
    let out = nios_32x32_read::<in_len, 16, endpoint>(dev, 2, addr).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s:  Read  0x%08" PRIx32 " from addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, *data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_adi_axi_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u32, data: u32) -> Result<u32>
 {
    let out = nios_32x32_write::<in_len, out_len, endpoint>(dev, 2, addr, data).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08" PRIx32 " to   addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_wishbone_master_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u32) -> Result<u32>
 {
    let out = nios_32x32_read::<in_len, 16, endpoint>(dev, 3, addr).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s:  Read  0x%08" PRIx32 " from addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, *data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_wishbone_master_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u32, data: u32) -> Result<u32>
 {
    let out = nios_32x32_write::<in_len, out_len, endpoint>(dev, 3, addr, data).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08" PRIx32 " to   addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_rfic_command_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, cmd: u16) -> Result<u64>
 {
    let out = nios_16x64_read::<in_len, 16, endpoint>(dev, 1, cmd).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Read 0x%04x 0x%08x\n", __FUNCTION__, cmd, *data);
     }
 #endif*/
 }
 
 pub async fn nios_rfic_command_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, cmd: u16, data: u64) -> Result<u64>
 {
    let out = nios_16x64_write::<in_len, 16, endpoint>(dev, 1, cmd, data).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Write 0x%04x 0x%08x\n", __FUNCTION__, cmd, data);
     }
 #endif*/
 }
 
 pub async fn nios_rffe_control_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device) -> Result<u32>
 {
    let out = nios_8x32_read::<in_len, 16, endpoint>(dev, 3, 0).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Read 0x%08x\n", __FUNCTION__, *value);
     }
 #endif*/
 }
 
 pub async fn nios_rffe_control_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, value: u32) -> Result<u32>
 {
    let out = nios_8x32_write::<in_len, 16, endpoint>(dev, 3, 0, value).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, value);
     }
 #endif*/
 
 }
 
 pub async fn nios_rffe_fastlock_save<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, is_tx: bool,
                             rffe_profile: u8, nios_profile: u16) -> Result<u32>
 {
     let mut addr: u8 = 0;
     let data: u32 = ((rffe_profile as u32) << 16) | nios_profile as u32;
 
     if is_tx { addr = 1; }
 
    let out = nios_8x32_write::<in_len, 16, endpoint>(dev, 5, addr, data).await?;

    Ok(out)
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, data);
     }
 #endif*/
 }
 
 pub async fn nios_ad56x1_vctcxo_trim_dac_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device) -> Result<u16>
 {
    let out = nios_8x16_read::<in_len, 16, endpoint>(dev, 3, 0).await?;

    Ok(out)
 }
 
 pub async fn nios_ad56x1_vctcxo_trim_dac_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, value: u16) -> Result<u16>
 {
    let out = nios_8x16_write::<in_len, 16, endpoint>(dev, 3, 0, value).await?;

    Ok(out)
 }
 
 pub async fn nios_adf400x_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8) -> Result<u32>
 {
    let out = nios_8x32_read::<in_len, 16, endpoint>(dev, 4, addr).await?;

    Ok(out)
 }
 
 pub async fn nios_adf400x_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8, mut data: u32) -> Result<u32>
 {
     data &= !(0x3);
 
     let out = nios_8x32_write::<in_len, 16, endpoint>(dev, 4, 0, data | (addr as u32 & 0x3)).await?;

     Ok(out)
 }
 
 pub async fn nios_vctcxo_trim_dac_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8, value: u16) -> Result<u16>
 {
    let out = nios_8x16_write::<in_len, 16, endpoint>(dev, 0, addr, value).await?;
    
    Ok(out)
 }
 
 pub async fn nios_vctcxo_trim_dac_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, addr: u8) -> Result<u16>
 {
     let out = nios_8x16_read::<in_len, 16, endpoint>(dev, 0, addr).await?;
     
     Ok(out)
 }
 
 pub async fn nios_set_vctcxo_tamer_mode<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, mode: u8) -> Result<u8>
 //In the original code "mode" was its own type, but it seems to just be a fancy u8 when all's said and done.
 {
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev,2, 0xff,mode).await?;
    
    Ok(out)
 }
 
 pub async fn nios_get_vctcxo_tamer_mode<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device,
                                ) -> Result<u8>
 {
    
    let mut mode: u8 = 255; //Initialize as invalid code - if everything goes right, this should change ere the function returns.

     let mode_detected = nios_8x8_read::<in_len, 15, endpoint>(dev, 2, 0xff).await?;

        match (mode_detected) {
            0 | //Disabled,
            1 | //1 PPS,
            2 => //or 10 MHz.
                {mode = mode_detected}

            _ =>
                {Error::msg("Error -1: Unexpected mode detected");} //If it's not one of these - bail!
        }
    Ok(mode)
 }
 
 
 pub async fn nios_get_iq_gain_correction<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: i32, //"ch" was originally "bladerf_channel", but that seems to just be a plain i32.
                                ) -> Result<i16>
 {
     let mut tmp: u16 = 0;

     match ch {
        0 => {
             tmp = nios_8x16_read::<in_len, 16, endpoint>(dev, 1,
                                     0).await?;
        }
 
        1 => {
            tmp = nios_8x16_read::<in_len, 16, endpoint>(dev, 1,
                                     2).await?;
         }
 
         _ => { /*log_debug("Invalid channel: 0x%x\n", ch);*/ }
     }

     Ok(tmp as i16)
 }
 
 pub async fn nios_get_iq_phase_correction<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: i32,
                                  ) -> Result<i16>
 {
     let mut tmp: u16 = 0;
     match ch {
        0 => {
             tmp = nios_8x16_read::<in_len, 16, endpoint>(dev, 1,
                                     1).await?;
        }
 
        1 => {
             tmp = nios_8x16_read::<in_len, 16, endpoint>(dev, 1,
                                     3).await?;
             }
 
        _ =>
             {/*log_debug("Invalid channel: 0x%x\n", ch);*/}
     }
 
    Ok(tmp as i16)
 }
 
 pub async fn nios_set_iq_gain_correction<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: u8,
                                 value: i16) -> Result<u16>
 {
    let mut tmp = 0;
     match ch {
         0 => {
            tmp = nios_8x16_write::<in_len, 16, endpoint>(dev, 1,
                                      0, value as u16).await?;
         }
 
         1 => {
             tmp = nios_8x16_write::<in_len, 16, endpoint>(dev, 1,
                                      2, value as u16).await?;
             }
 
        _ =>
            {/*log_debug("Invalid channel: 0x%x\n", ch);*/}
     }

     Ok(tmp)
 }
 
 pub async fn nios_set_iq_phase_correction<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: u8,
                                  value: i16) -> Result<u16>
 {
    let mut tmp = 0;

     match ch {
        0 => {
             tmp = nios_8x16_write::<in_len, 16, endpoint>(dev, 1,
                                      1, value as u16).await?;
        }
 
         1 => {
             tmp = nios_8x16_write::<in_len, 16, endpoint>(dev, 1,
                                      3, value as u16).await?;
         }
 
         _ => {/*log_debug("Invalid channel: 0x%x\n", ch);*/}
     }

     Ok(tmp)
 }
 
 pub async fn nios_set_agc_dc_correction<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, q_max: i16, i_max: i16,
                                q_mid: i16, i_mid: i16,
                                q_low: i16, i_low: i16) -> Result<u16>
 {
 
     nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 0, q_max as u16).await?;
     nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 1, i_max as u16).await?;
     nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 2, q_mid as u16).await?;
     nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 3, i_mid as u16).await?;
     nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 4, q_low as u16).await?;
     let tmp = nios_8x16_write::<in_len, 16, endpoint>(dev, 2, 5, i_low as u16).await?;
    
    Ok(tmp)
 }
 
 pub async fn nios_xb200_synth_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, value: u32) -> Result<u32>
 {
    let out = nios_8x32_write::<in_len, 16, endpoint>(dev,  2, 0, value).await?;
    Ok(out)
 }
 
 pub async fn nios_expansion_gpio_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device) -> Result<u32>
 {
    let out = nios_32x32_masked_read::<in_len, 16, endpoint>(dev, 0,
                                         0xffffffff).await?;
    Ok(out)
 }
 
 pub async fn nios_expansion_gpio_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, mask: u32, val: u32) -> Result<u32>
 {
    let out = nios_32x32_masked_write::<in_len, 16, endpoint>(dev, 0, mask, val).await?;
    Ok(out)
 }
 
 pub async fn nios_expansion_gpio_dir_read<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device) -> Result<u32>
 {
    let out = nios_32x32_masked_read::<in_len, 16, endpoint>(dev, 1,
                                         0xffffffff).await?;
    Ok(out)
 }
 
 pub async fn nios_expansion_gpio_dir_write<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device,
                                   mask: u32, val: u32) -> Result<u32>
 {
    let out = nios_32x32_masked_write::<in_len, 16, endpoint>(dev, 1, mask, val).await?;
    Ok(out)
 }
 
 pub async fn nios_retune<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: u8,
                 timestamp: u64, nint: u16, nfrac: u32,
                 freqsel: u8, vcocap: u8, low_band: bool,
                 xb_gpio: u8, quick_tune: bool) -> Result<u64>
 {
 
     /*if (timestamp == NIOS_PKT_RETUNE_CLEAR_QUEUE) {
         log_verbose("Clearing %s retune queue.\n", channel2str(ch));
     } else {
         log_verbose("%s: channel=%s timestamp=%"PRIu64" nint=%u nfrac=%u\n\t\t\t\t"
                     "freqsel=0x%02x vcocap=0x%02x low_band=%d quick_tune=%d\n",
                     __FUNCTION__, channel2str(ch), timestamp, nint, nfrac,
                     freqsel, vcocap, low_band, quick_tune);
     }*/
 
    let buf = pkt_retune::pack_retune(ch, timestamp,
                          nint, nfrac, freqsel, vcocap, low_band,
                          xb_gpio, quick_tune);
 
    nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
    let out = pkt_retune::unpack_retune(&buf);
 
     /*if (resp_flags & NIOS_PKT_RETUNERESP_FLAG_TSVTUNE_VALID) {
         log_verbose("%s retune operation: vcocap=%u, duration=%"PRIu64"\n",
                     channel2str(ch), vcocap, duration);
     } else {
         log_verbose("%s operation duration: %"PRIu64"\n",
                     channel2str(ch), duration);
     }*/
 
     if out.2 & 0b00000010 == 0 {
         if out.0 == 0 {
             //log_debug("FPGA tuning reported failure.\n");
             anyhow::Error::msg("Error -1: FPGA tuning reported failure.");
         } else {
             //log_debug("The FPGA's retune queue is full. Try again after a previous request has completed.\n");
             anyhow::Error::msg("Error -15: BladeRF retune queue is full. Try again later.");
         }
     }

     Ok(out.0)
 }
 
 pub async fn nios_retune2<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: u8,
                  timestamp: u64, nios_profile: u16,
                  rffe_profile: u8, port: u8,
                  spdt: u8) -> Result<u64>
 {
 
     /*if (timestamp == NIOS_PKT_RETUNE2_CLEAR_QUEUE) {
        //log_verbose("Clearing %s retune queue.\n", channel2str(ch));
     } else {
        /* log_verbose("%s: channel=%s timestamp=%"PRIu64" nios_profile=%u "
                     "rffe_profile=%u\n\t\t\t\tport=0x%02x spdt=0x%02x\n",
                     __FUNCTION__, channel2str(ch), timestamp, nios_profile,
                     rffe_profile, port, spdt);*/
     }*/
 
     let buf = pkt_retune::pack_retune2(ch, timestamp, nios_profile, rffe_profile,
                           port, spdt);
 
     nios_access::<in_len, 16, endpoint>(dev, &buf).await?;
 
     let out = pkt_retune::unpack_retune2(&buf);
 
     /*if (resp_flags & NIOS_PKT_RETUNE2_RESP_FLAG_TSVTUNE_VALID) {
         log_verbose("%s retune operation: duration=%"PRIu64"\n",
                     channel2str(ch), duration);
     } else {
         log_verbose("%s operation duration: %"PRIu64"\n",
                     channel2str(ch), duration);
     }*/
 
     if out.1 & 0b00000010 == 0 {
        if out.0 == 0 {
            //log_debug("FPGA tuning reported failure.\n");
            anyhow::Error::msg("Error -1: FPGA tuning reported failure.");
        } else {
            //log_debug("The FPGA's retune queue is full. Try again after a previous request has completed.\n");
            anyhow::Error::msg("Error -15: BladeRF retune queue is full. Try again later.");
        }
    }
    Ok(out.0)
 }
 
 pub async fn nios_read_trigger<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: u8,
                       trigger: u8) -> Result<u8>
 {
    let nios_id: u8;
 
     match (ch) {
        1 => {
             nios_id = 3;
        }
 
        0 => {
             nios_id = 4;
        }
 
        _ => {
            nios_id = 0xff; //We must do this, lest Rust scream at us for "possible uninitalization".
            //log_debug("Invalid channel: 0x%x\n", ch);
            anyhow::Error::msg("Error -1: Invalid Channel.");
        }
     }
 
     /* Only 1 external trigger is currently supported */
     match (trigger) {
         0 |
         1 |
         2 =>
             {}
 
         _ => {
             //log_debug("Invalid trigger: %d\n", trigger);
             anyhow::Error::msg("Error -1: Invalid Trigger.");
         }
     }
 
     let out = nios_8x8_read::<in_len, 15, endpoint>(dev, nios_id, 0).await?;

     Ok(out)
 }
 
 pub async fn nios_write_trigger<const in_len: usize, const out_len: usize, const endpoint: u8>(dev: &Device, ch: u8,
                        trigger: u8, value: u8) -> Result<u8>
 {
    let nios_id: u8;
 
     match (ch) {
        1 => {
             nios_id = 3;
        }
 
        0 => {
             nios_id = 4;
        }
 
        _ => {
            nios_id = 0xff; //Cur obligare sic facere!?
            anyhow::Error::msg("Error -1: Invalid Channel.");
        }
     }
 
     /* Only 1 external trigger is currently supported */
     //Translator dicit: Dixere modo est ramulus unus, sed hic tres optiones validae est.
     match (trigger) {
         0 |
         1 |
         2 =>
            {}
 
         _ => {
             //log_debug("Invalid trigger: %d\n", trigger);
             anyhow::Error::msg("Error -1: Invalid Trigger.");
         }
     }
 
    let out = nios_8x8_write::<in_len, 15, endpoint>(dev, nios_id, 0, value).await?;

    Ok(out)
 }