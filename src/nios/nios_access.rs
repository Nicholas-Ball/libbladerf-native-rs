use crate::{BladerfDirection, BladerfVersion, Device};
use anyhow::Result;
use crate::nios::packet as packets;
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
 pub async fn nios_access<const len: usize>(dev: &mut bladerf, buf: &[u8; len]) -> Result<[u8; len]>
 {
     let usb: &mut bladerf_usb = dev.backend_data;
 
     /* Send the command */
     bulk_transfer_out(usb.driver, buf).await?;
 
     /* Retrieve the request */
     let out = bulk_transfer_in(usb.driver).await?;
     Ok(out)
 }
 
 /* Variant that doesn't output to log_error on error. */
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
 }
 
 pub async fn nios_8x8_read(dev: &mut bladerf, id: u8,
                          addr: u8, data: &mut u8)
 {
     let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_8x8_pack(buf, id, false, addr, 0);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_8x8_resp_unpack(buf, NULL, NULL, NULL, data)?;
 }
 
 pub async fn nios_8x8_write(dev: &mut bladerf, id: u8,
                           addr: u8, data: u8)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_8x8_pack(buf, id, true, addr, data);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_8x8_resp_unpack(buf, NULL, NULL, NULL, NULL)?;
 }
 
 pub async fn nios_8x16_read(dev: &mut bladerf, id: u8,
    addr: u8, data: &mut u16)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
    let mut tmp: u16 = 0;
 
    nios_pkt_8x16_pack(buf, id, false, addr, 0);
 
    nios_access(dev, buf).await?;
 
    nios_pkt_8x16_resp_unpack(buf, NULL, NULL, NULL, &tmp)?;
 
    unsafe {*data = tmp;}
 }
 
 pub async fn nios_8x16_write(dev: &mut bladerf, id: u8,
    addr: u8, data: u16)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_8x16_pack(buf, id, true, addr, data);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_8x16_resp_unpack(buf, NULL, NULL, NULL, NULL)?;
 }
 
 pub fn log_debug_8x32_pkt(buf: *const u8) {
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
 }
 
 pub async fn nios_8x32_read(dev: &mut bladerf, id: u8,
    addr: u8, data: &mut u32)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_8x32_pack(buf, id, false, addr, 0);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_8x32_resp_unpack(buf, NULL, NULL, NULL, data)?;
 }
 
 pub async fn nios_8x32_write(dev: &mut bladerf, id: u8,
    addr: u8, data: u32)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_8x32_pack(buf, id, true, addr, data);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_8x32_resp_unpack(buf, NULL, NULL, NULL, NULL)?;
 }
 
 pub async fn nios_16x64_read(dev: &mut bladerf, id: u8,
    addr: u16, data: &mut u64)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_16x64_pack(buf, id, false, addr, 0);
 
     if (NIOS_PKT_16x64_TARGET_RFIC == id) {
         nios_access_quiet(dev, buf).await?;
     } else {
         nios_access(dev, buf).await?;
     }
 
     nios_pkt_16x64_resp_unpack(buf, NULL, NULL, NULL, data)?;
 }
 
 pub async fn nios_16x64_write(dev: &mut bladerf, id: u8,
    addr: u16, data: u64)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_16x64_pack(buf, id, true, addr, data);
 
     /* RFIC access times out occasionally, and this is fine. */
     if (NIOS_PKT_16x64_TARGET_RFIC == id) {
        nios_access_quiet(dev, buf).await?;
    } else {
        nios_access(dev, buf).await?;
    }

     nios_pkt_16x64_resp_unpack(buf, NULL, NULL, NULL, NULL)?;
 }
 
 pub async fn nios_32x32_read(dev: &mut bladerf, id: u8,
    addr: u32, data: &mut u32)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_32x32_pack(buf, id, false, addr, 0);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_32x32_resp_unpack(buf, NULL, NULL, NULL, data)?;
 }
 
 pub async fn nios_32x32_write(dev: &mut bladerf, id: u8,
    addr: u32, data: u32)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
    nios_pkt_32x32_pack(buf, id, true, addr, data);
 
    nios_access(dev, buf).await?;
 
     nios_pkt_32x32_resp_unpack(buf, NULL, NULL, NULL, NULL)?;
 }
 
 pub async fn nios_32x32_masked_read(dev: &mut bladerf, id: u8,
                                   mask: u32, val: &mut u32)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     /* The address is used as a mask of bits to read and return */
     nios_pkt_32x32_pack(buf, id, false, mask, 0);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_32x32_resp_unpack(buf, NULL, NULL, NULL, val)?;
 }
 
 pub async fn nios_32x32_masked_write(dev: &mut bladerf, id: u8,
    mask: u32, val: u32)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     nios_pkt_32x32_pack(buf, id, true, mask, val);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_32x32_resp_unpack(buf, NULL, NULL, NULL, NULL)?;
 
 }
 
 pub async fn nios_config_read(dev: &mut bladerf, val: &mut u32)
 {
     nios_8x32_read(dev, NIOS_PKT_8x32_TARGET_CONTROL, 0, val).await?;
 }
 
 pub async fn nios_config_write(dev: &mut bladerf, val: u32)
 {
    nios_8x32_write(dev, NIOS_PKT_8x32_TARGET_CONTROL, 0, val).await?;
 
    log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, val);
 }
 
 pub async fn nios_get_fpga_version(dev: &mut bladerf, ver: &mut bladerf_version)
 {
    let mut regval: u32 = 0;
    nios_8x32_read(dev, NIOS_PKT_8x32_TARGET_VERSION, 0, &regval).await?;
 
    log_verbose("%s: Read FPGA version word: 0x%08x\n", __FUNCTION__, regval);
 
    ver.major = (regval >> 24) & 0xff;
    ver.minor = (regval >> 16) & 0xff;
    ver.patch = LE16_TO_HOST(regval & 0xffff);
 
    snprintf(ver.describe, BLADERF_VERSION_STR_MAX, "%d.%d.%d", ver.major, ver.minor, ver.patch);
 }
 
 pub async fn nios_get_timestamp(dev: &mut bladerf,
                        dir: bladerf_direction,
                        timestamp: &mut u64)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
     let mut addr: u8 = 0;
 
     match (dir){
         BLADERF_RX => {
             addr = NIOS_PKT_8x64_TIMESTAMP_RX;
         }
 
         BLADERF_TX => {
             addr = NIOS_PKT_8x64_TIMESTAMP_TX;
         }
         _ => {
             log_debug("Invalid direction: %d\n", dir);
             error(BLADERF_ERR_INVAL);
         }
     }
 
     nios_pkt_8x64_pack(buf, NIOS_PKT_8x64_TARGET_TIMESTAMP, false, addr, 0);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_8x64_resp_unpack(buf, NULL, NULL, NULL, timestamp)?;
 }
 
 pub async fn nios_si5338_read(dev: &mut bladerf, addr: u8, data: &mut u8)
 {
    nios_8x8_read(dev, NIOS_PKT_8x8_TARGET_SI5338, addr, data).await?;
 }
 
 pub async fn nios_si5338_write(dev: &mut bladerf, addr: u8, data: u8)
 {
    nios_8x8_write(dev, NIOS_PKT_8x8_TARGET_SI5338, addr, data).await?;
 
 if (ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE){
     if (status == 0) {
         log_verbose("%s: Wrote 0x%02x to addr 0x%02x\n",
                     __FUNCTION__, data, addr);
     }
    }
 }
 
 pub async fn nios_lms6_read(dev: &mut bladerf, addr: u8, data: &mut u8)
 {
    nios_8x8_read(dev, NIOS_PKT_8x8_TARGET_LMS6, addr, data).await?;
 
 }
 
 pub async fn nios_lms6_write(dev: &mut bladerf, addr: u8, data: u8)
 {
    nios_8x8_write(dev, NIOS_PKT_8x8_TARGET_LMS6, addr, data).await?;
 
 if(ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE){
     if (status == 0) {
         log_verbose("%s: Wrote 0x%02x to addr 0x%02x\n",
                     __FUNCTION__, data, addr);
     }
    }
 }
 
 pub async fn nios_ina219_read(dev: &mut bladerf, addr: u8, data: &mut u16)
 {
    nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_INA219, addr, data).await?;
 }
 
 pub async fn nios_ina219_write(dev: &mut bladerf, addr: u8, data: u16)
 {
    nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_INA219, addr, data).await?;
 }
 
 const VERBOSE_OUT_SINGLEBYTE: str = "%s: %s 0x%02x @ addr 0x%04x\n"; //TODO: literally everything involving string operations 'cause I don't think they're that important at the moment
 const VERBOSE_OUT_MULTIBYTE: str  = "%s: %s 0x%02x @ addr 0x%04x (%d/%d)\n";
 
 pub async fn nios_ad9361_spi_read(dev: &mut bladerf, cmd: u16, data: &mut u64)
 {
    nios_16x64_read(dev, NIOS_PKT_16x64_TARGET_AD9361, cmd, data).await?;
 
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
    }*/ //look to be honest we can deal with this later; it's all debug code
 }
 
 pub async fn nios_ad9361_spi_write(dev: &mut bladerf, cmd: u16, data: u64)
 {
    nios_16x64_write(dev, NIOS_PKT_16x64_TARGET_AD9361, cmd, data).await?;
 
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
 #endif*/ //same business here
 }
 
 pub async fn nios_adi_axi_read(dev: &mut bladerf, addr: u32, data: &mut u32)
 {
    nios_32x32_read(dev, NIOS_PKT_32x32_TARGET_ADI_AXI, addr, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s:  Read  0x%08" PRIx32 " from addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, *data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_adi_axi_write(dev: &mut bladerf, addr: u32, data: u32)
 {
    nios_32x32_write(dev, NIOS_PKT_32x32_TARGET_ADI_AXI, addr, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08" PRIx32 " to   addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_wishbone_master_read(dev: &mut bladerf, addr: u32, data: &mut u32)
 {
     nios_32x32_read(dev, NIOS_PKT_32x32_TARGET_WB_MSTR, addr, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s:  Read  0x%08" PRIx32 " from addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, *data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_wishbone_master_write(dev: &mut bladerf, addr: u32, data: u32)
 {
    nios_32x32_write(dev, NIOS_PKT_32x32_TARGET_WB_MSTR, addr, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08" PRIx32 " to   addr 0x%04" PRIx32 "\n",
                     __FUNCTION__, data, addr);
     }
 #endif*/
 }
 
 pub async fn nios_rfic_command_read(dev: &mut bladerf, cmd: u16, data: &mut u64)
 {
    nios_16x64_read(dev, NIOS_PKT_16x64_TARGET_RFIC, cmd, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Read 0x%04x 0x%08x\n", __FUNCTION__, cmd, *data);
     }
 #endif*/
 }
 
 pub async fn nios_rfic_command_write(dev: &mut bladerf, cmd: u16, data: u64)
 {
    nios_16x64_write(dev, NIOS_PKT_16x64_TARGET_RFIC, cmd, data);
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Write 0x%04x 0x%08x\n", __FUNCTION__, cmd, data);
     }
 #endif*/
 }
 
 pub async fn nios_rffe_control_read(dev: &mut bladerf, value: &mut u32)
 {
    nios_8x32_read(dev, NIOS_PKT_8x32_TARGET_RFFE_CSR, 0, value).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Read 0x%08x\n", __FUNCTION__, *value);
     }
 #endif*/
 }
 
 pub async fn nios_rffe_control_write(dev: &mut bladerf, value: u32)
 {
    nios_8x32_write(dev, NIOS_PKT_8x32_TARGET_RFFE_CSR, 0, value).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, value);
     }
 #endif*/
 
 }
 
 pub async fn nios_rffe_fastlock_save(dev: &mut bladerf, is_tx: bool,
                             rffe_profile: u8, nios_profile: u16)
 {
     let mut addr: u8 = 0;
     let data: u32 = 0;
 
     if is_tx { addr = 1; }
     data = (rffe_profile << 16) | nios_profile;
 
    nios_8x32_write(dev, NIOS_PKT_8x32_TARGET_FASTLOCK, addr, data).await?;
 
 /*#ifdef ENABLE_LIBBLADERF_NIOS_ACCESS_LOG_VERBOSE
     if (status == 0) {
         log_verbose("%s: Wrote 0x%08x\n", __FUNCTION__, data);
     }
 #endif*/
 }
 
 pub async fn nios_ad56x1_vctcxo_trim_dac_read(dev: &mut bladerf, value: &mut u16)
 {
    nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_AD56X1_DAC, 0, value).await?;
 }
 
 pub async fn nios_ad56x1_vctcxo_trim_dac_write(dev: &mut bladerf, value: u16)
 {
    nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AD56X1_DAC, 0, value).await?;
 }
 
 pub async fn nios_adf400x_read(dev: &mut bladerf, addr: u8, data: &mut u32)
 {
    nios_8x32_read(dev, NIOS_PKT_8x32_TARGET_ADF400X, addr, data).await?;
 }
 
 pub async fn nios_adf400x_write(dev: &mut bladerf, addr: u8, data: u32)
 {
     data &= !(0x3);
 
     nios_8x32_write(dev, NIOS_PKT_8x32_TARGET_ADF400X, 0, data | (addr & 0x3)).await?;
 }
 
 pub async fn nios_vctcxo_trim_dac_write(dev: &mut bladerf, addr: u8, value: u16)
 {
     return nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_VCTCXO_DAC, addr, value);
 }
 
 pub async fn nios_vctcxo_trim_dac_read(dev: &mut bladerf, addr: u8, value: &mut u16)
 {
     return nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_VCTCXO_DAC, addr, value);
 }
 
 pub async fn nios_set_vctcxo_tamer_mode(dev: &mut bladerf, mode: bladerf_vctcxo_tamer_mode)
 {
    nios_8x8_write(dev,NIOS_PKT_8x8_TARGET_VCTCXO_TAMER, 0xff,mode as u8).await?;
 }
 
 pub async fn nios_get_vctcxo_tamer_mode(dev: &mut bladerf,
                                mode: &mut bladerf_vctcxo_tamer_mode)
 {
     let mut tmp: u8;
 
    unsafe {*mode = BLADERF_VCTCXO_TAMER_INVALID;}
 
     nios_8x8_read(dev, NIOS_PKT_8x8_TARGET_VCTCXO_TAMER, 0xff, &tmp).await?;

        match (tmp as bladerf_vctcxo_tamer_mode) {
            BLADERF_VCTCXO_TAMER_DISABLED |
            BLADERF_VCTCXO_TAMER_1_PPS |
            BLADERF_VCTCXO_TAMER_10_MHZ =>
                {unsafe {*mode = tmp as bladerf_vctcxo_tamer_mode;}}

            _ =>
                {status = BLADERF_ERR_UNEXPECTED;}
        }
 }
 
 
 pub async fn nios_get_iq_gain_correction(dev: &mut bladerf, ch: bladerf_channel,
                                 value: &mut i16)
 {
     let mut tmp: u16 = 0;
 
     match (ch) {
         BLADERF_CHANNEL_RX(0) => {
             nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                     NIOS_PKT_8x16_ADDR_IQ_CORR_RX_GAIN, &tmp).await?;
        }
 
        BLADERF_CHANNEL_TX(0) => {
        nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                     NIOS_PKT_8x16_ADDR_IQ_CORR_TX_GAIN, &tmp).await?;
         }
 
         _ => { log_debug("Invalid channel: 0x%x\n", ch); }
     }
 
    unsafe {*value = tmp as i16;}
 }
 
 pub async fn nios_get_iq_phase_correction(dev: &mut bladerf, ch: bladerf_channel,
                                  value: &mut i16)
 {
     let mut tmp: u16 = 0;
 
     match (ch) {
        BLADERF_CHANNEL_RX(0) => {
             nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                     NIOS_PKT_8x16_ADDR_IQ_CORR_RX_PHASE, &tmp).await?;
        }
 
        BLADERF_CHANNEL_TX(0) => {
             nios_8x16_read(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                     NIOS_PKT_8x16_ADDR_IQ_CORR_TX_PHASE, &tmp).await?;
             }
 
        _ =>
             {log_debug("Invalid channel: 0x%x\n", ch);}
     }
 
    unsafe{*value = tmp as i16;}
 }
 
 pub async fn nios_set_iq_gain_correction(dev: &mut bladerf, ch: bladerf_channel,
                                 value: i16)
 {
     match (ch) {
         BLADERF_CHANNEL_RX(0) => {
            nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                      NIOS_PKT_8x16_ADDR_IQ_CORR_RX_GAIN, value).await?;
         }
 
         BLADERF_CHANNEL_TX(0) => {
             nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                      NIOS_PKT_8x16_ADDR_IQ_CORR_TX_GAIN, value).await?;
             }
 
        _ =>
            {log_debug("Invalid channel: 0x%x\n", ch);}
     }
 }
 
 pub async fn nios_set_iq_phase_correction(dev: &mut bladerf, ch: bladerf_chanel,
                                  value: i16)
 {
 
     match (ch) {
        BLADERF_CHANNEL_RX(0) => {
             nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                      NIOS_PKT_8x16_ADDR_IQ_CORR_RX_PHASE, value).await?;
        }
 
         BLADERF_CHANNEL_TX(0) => {
             nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_IQ_CORR,
                                      NIOS_PKT_8x16_ADDR_IQ_CORR_TX_PHASE, value).await?;
         }
 
         _ => {log_debug("Invalid channel: 0x%x\n", ch);}
     }
 }
 
 pub async fn nios_set_agc_dc_correction(dev: &mut bladerf, q_max: i16, i_max: i16,
                                q_mid: i16, i_mid: i16,
                                q_low: i16, i_low: i16)
 {
 
     nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AGC_CORR, NIOS_PKT_8x16_ADDR_AGC_DC_Q_MAX, q_max).await?;
 
     nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AGC_CORR, NIOS_PKT_8x16_ADDR_AGC_DC_I_MAX, i_max).await?;
     nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AGC_CORR, NIOS_PKT_8x16_ADDR_AGC_DC_Q_MID, q_mid).await?;
     nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AGC_CORR, NIOS_PKT_8x16_ADDR_AGC_DC_I_MID, i_mid).await?;
     nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AGC_CORR, NIOS_PKT_8x16_ADDR_AGC_DC_Q_MIN, q_low).await?;
     nios_8x16_write(dev, NIOS_PKT_8x16_TARGET_AGC_CORR, NIOS_PKT_8x16_ADDR_AGC_DC_I_MIN, i_low).await?;
 }
 
 pub async fn nios_xb200_synth_write(dev: &mut bladerf, value: u32)
 {
    nios_8x32_write(dev, NIOS_PKT_8x32_TARGET_ADF4351, 0, value).await?;
 }
 
 pub async fn nios_expansion_gpio_read(dev: &mut bladerf, val: &mut u32)
 {
    nios_32x32_masked_read(dev, NIOS_PKT_32x32_TARGET_EXP,
                                         0xffffffff, val).await?;
 }
 
 pub async fn nios_expansion_gpio_write(dev: &mut bladerf, mask: u32, val: u32)
 {
    nios_32x32_masked_write(dev, NIOS_PKT_32x32_TARGET_EXP,
                                          mask, val).await?;
 }
 
 pub async fn nios_expansion_gpio_dir_read(dev: &mut bladerf, val: &mut u32)
 {
    nios_32x32_masked_read(dev, NIOS_PKT_32x32_TARGET_EXP_DIR,
                                         0xffffffff, val).await?;
 }
 
 pub async fn nios_expansion_gpio_dir_write(dev: &mut bladerf,
                                   mask: u32, val: u32)
 {
    nios_32x32_masked_write(dev, NIOS_PKT_32x32_TARGET_EXP_DIR,
                                          mask, val).await?;
 }
 
 pub async fn nios_retune(dev: &mut bladerf, ch: bladerf_channel,
                 timestamp: u64, nint: u16, nfrac: u32,
                 freqsel: u8, vcocap: u8, low_band: bool,
                 xb_gpio: u8, quick_tune: bool)
 {
    let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     let resp_flags: u8;
     let duration: u64;
 
     /*if (timestamp == NIOS_PKT_RETUNE_CLEAR_QUEUE) {
         log_verbose("Clearing %s retune queue.\n", channel2str(ch));
     } else {
         log_verbose("%s: channel=%s timestamp=%"PRIu64" nint=%u nfrac=%u\n\t\t\t\t"
                     "freqsel=0x%02x vcocap=0x%02x low_band=%d quick_tune=%d\n",
                     __FUNCTION__, channel2str(ch), timestamp, nint, nfrac,
                     freqsel, vcocap, low_band, quick_tune);
     }*/
 
     nios_pkt_retune_pack(buf, ch, timestamp,
                          nint, nfrac, freqsel, vcocap, low_band,
                          xb_gpio, quick_tune);
 
    nios_access(dev, buf).await?;
 
     nios_pkt_retune_resp_unpack(buf, &duration, &vcocap, &resp_flags);
 
     /*if (resp_flags & NIOS_PKT_RETUNERESP_FLAG_TSVTUNE_VALID) {
         log_verbose("%s retune operation: vcocap=%u, duration=%"PRIu64"\n",
                     channel2str(ch), vcocap, duration);
     } else {
         log_verbose("%s operation duration: %"PRIu64"\n",
                     channel2str(ch), duration);
     }*/
 
     if ((resp_flags & NIOS_PKT_RETUNERESP_FLAG_SUCCESS) == 0) {
         if (timestamp == BLADERF_RETUNE_NOW) {
             log_debug("FPGA tuning reported failure.\n");
             anyhow::Error(BLADERF_ERR_UNEXPECTED);
         } else {
             log_debug("The FPGA's retune queue is full. Try again after a previous request has completed.\n");
             anyhow::Error(BLADERF_ERR_QUEUE_FULL);
         }
     }
 }
 
 pub async fn nios_retune2(dev: &mut bladerf, ch: bladerf_channel,
                  timestamp: u64, nios_profile: u16,
                  rffe_profile: u8, port: u8,
                  spdt: u8)
 {
     let mut buf: &mut u8 = u8[NIOS_PKT_LEN];
 
     let resp_flags: u8;
     let duration: u64;
 
     /*if (timestamp == NIOS_PKT_RETUNE2_CLEAR_QUEUE) {
        //log_verbose("Clearing %s retune queue.\n", channel2str(ch));
     } else {
        /* log_verbose("%s: channel=%s timestamp=%"PRIu64" nios_profile=%u "
                     "rffe_profile=%u\n\t\t\t\tport=0x%02x spdt=0x%02x\n",
                     __FUNCTION__, channel2str(ch), timestamp, nios_profile,
                     rffe_profile, port, spdt);*/
     }*/
 
     nios_pkt_retune2_pack(buf, ch, timestamp, nios_profile, rffe_profile,
                           port, spdt);
 
     nios_access(dev, buf).await?;
 
     nios_pkt_retune2_resp_unpack(buf, &duration, &resp_flags);
 
     /*if (resp_flags & NIOS_PKT_RETUNE2_RESP_FLAG_TSVTUNE_VALID) {
         log_verbose("%s retune operation: duration=%"PRIu64"\n",
                     channel2str(ch), duration);
     } else {
         log_verbose("%s operation duration: %"PRIu64"\n",
                     channel2str(ch), duration);
     }*/
 
     if ((resp_flags & NIOS_PKT_RETUNE2_RESP_FLAG_SUCCESS) == 0) {
         if (timestamp == BLADERF_RETUNE_NOW) {
             log_debug("FPGA tuning reported failure.\n");
             anyhow::Error(BLADERF_ERR_UNEXPECTED);
         } else {
             log_debug("The FPGA's retune queue is full. Try again after a previous request has completed.\n");
             anyhow::Error(BLADERF_ERR_QUEUE_FULL);
         }
     }
 }
 
 pub async fn nios_read_trigger(dev: &mut bladerf, ch: bladerf_channel,
                       trigger: bladerf_trigger_signal, value: &mut u8)
 {
    let nios_id: u8;
 
     match (ch) {
        BLADERF_CHANNEL_TX(0) => {
             nios_id = NIOS_PKT_8x8_TX_TRIGGER_CTL;
        }
 
        BLADERF_CHANNEL_RX(0) => {
             nios_id = NIOS_PKT_8x8_RX_TRIGGER_CTL;
        }
 
        _ => {
             //log_debug("Invalid channel: 0x%x\n", ch);
            anyhow::Error(BLADERF_ERR_INVAL);
        }
     }
 
     /* Only 1 external trigger is currently supported */
     match (trigger) {
         BLADERF_TRIGGER_J71_4 |
         BLADERF_TRIGGER_J51_1 |
         BLADERF_TRIGGER_MINI_EXP_1 =>
             {}
 
         _ => {
             //log_debug("Invalid trigger: %d\n", trigger);
             return BLADERF_ERR_INVAL;
         }
     }
 
     nios_8x8_read(dev, nios_id, 0, value).await?;
 }
 
 pub async fn nios_write_trigger(dev: &mut bladerf, ch: bladerf_channel,
                        trigger: bladerf_trigger_signal, value: u8)
 {
    let nios_id: u8;
 
     match (ch) {
        BLADERF_CHANNEL_TX(0) => {
             nios_id = NIOS_PKT_8x8_TX_TRIGGER_CTL;
        }
 
        BLADERF_CHANNEL_RX(0) => {
             nios_id = NIOS_PKT_8x8_RX_TRIGGER_CTL;
        }
 
        _ => {
             anyhow::Error::new(BLADERF_ERR_INVAL);
        }
     }
 
     /* Only 1 external trigger is currently supported */
     match (trigger) {
         BLADERF_TRIGGER_J71_4 |
         BLADERF_TRIGGER_J51_1 |
         BLADERF_TRIGGER_MINI_EXP_1 =>
            {}
 
         _ => {
             log_debug("Invalid trigger: %d\n", trigger);
             anyhow::Error::new(BLADERF_ERR_INVALID);
         }
     }
 
     nios_8x8_write(dev, nios_id, 0, value).await?;
 }