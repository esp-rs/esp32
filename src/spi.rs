#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CMD_REG(i)"]
    pub spi_cmd_reg: SPI_CMD_REG,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - SPI_CTRL_REG(i)"]
    pub spi_ctrl_reg: SPI_CTRL_REG,
    #[doc = "0x0c - SPI_CTRL1_REG(i)"]
    pub spi_ctrl1_reg: SPI_CTRL1_REG,
    #[doc = "0x10 - SPI_RD_STATUS_REG(i)"]
    pub spi_rd_status_reg: SPI_RD_STATUS_REG,
    #[doc = "0x14 - SPI_CTRL2_REG(i)"]
    pub spi_ctrl2_reg: SPI_CTRL2_REG,
    #[doc = "0x18 - SPI_CLOCK_REG(i)"]
    pub spi_clock_reg: SPI_CLOCK_REG,
    #[doc = "0x1c - SPI_USER_REG(i)"]
    pub spi_user_reg: SPI_USER_REG,
    #[doc = "0x20 - SPI_USER1_REG(i)"]
    pub spi_user1_reg: SPI_USER1_REG,
    #[doc = "0x24 - SPI_USER2_REG(i)"]
    pub spi_user2_reg: SPI_USER2_REG,
    #[doc = "0x28 - SPI_MOSI_DLEN_REG(i)"]
    pub spi_mosi_dlen_reg: SPI_MOSI_DLEN_REG,
    #[doc = "0x2c - SPI_MISO_DLEN_REG(i)"]
    pub spi_miso_dlen_reg: SPI_MISO_DLEN_REG,
    #[doc = "0x30 - SPI_SLV_WR_STATUS_REG(i)"]
    pub spi_slv_wr_status_reg: SPI_SLV_WR_STATUS_REG,
    #[doc = "0x34 - SPI_PIN_REG(i)"]
    pub spi_pin_reg: SPI_PIN_REG,
    #[doc = "0x38 - SPI_SLAVE_REG(i)"]
    pub spi_slave_reg: SPI_SLAVE_REG,
    #[doc = "0x3c - SPI_SLAVE1_REG(i)"]
    pub spi_slave1_reg: SPI_SLAVE1_REG,
    #[doc = "0x40 - SPI_SLAVE2_REG(i)"]
    pub spi_slave2_reg: SPI_SLAVE2_REG,
    #[doc = "0x44 - SPI_SLAVE3_REG(i)"]
    pub spi_slave3_reg: SPI_SLAVE3_REG,
    #[doc = "0x48 - SPI_SLV_WRBUF_DLEN_REG(i)"]
    pub spi_slv_wrbuf_dlen_reg: SPI_SLV_WRBUF_DLEN_REG,
    #[doc = "0x4c - SPI_SLV_RDBUF_DLEN_REG(i)"]
    pub spi_slv_rdbuf_dlen_reg: SPI_SLV_RDBUF_DLEN_REG,
    #[doc = "0x50 - SPI_CACHE_FCTRL_REG(i)"]
    pub spi_cache_fctrl_reg: SPI_CACHE_FCTRL_REG,
    #[doc = "0x54 - SPI_CACHE_SCTRL_REG(i)"]
    pub spi_cache_sctrl_reg: SPI_CACHE_SCTRL_REG,
    #[doc = "0x58 - SPI_SRAM_CMD_REG(i)"]
    pub spi_sram_cmd_reg: SPI_SRAM_CMD_REG,
    #[doc = "0x5c - SPI_SRAM_DRD_CMD_REG(i)"]
    pub spi_sram_drd_cmd_reg: SPI_SRAM_DRD_CMD_REG,
    #[doc = "0x60 - SPI_SRAM_DWR_CMD_REG(i)"]
    pub spi_sram_dwr_cmd_reg: SPI_SRAM_DWR_CMD_REG,
    #[doc = "0x64 - SPI_SLV_RD_BIT_REG(i)"]
    pub spi_slv_rd_bit_reg: SPI_SLV_RD_BIT_REG,
    _reserved25: [u8; 24usize],
    #[doc = "0x80 - SPI_W0_REG(i)"]
    pub spi_w0_reg: SPI_W0_REG,
    #[doc = "0x84 - SPI_W1_REG(i)"]
    pub spi_w1_reg: SPI_W1_REG,
    #[doc = "0x88 - SPI_W2_REG(i)"]
    pub spi_w2_reg: SPI_W2_REG,
    #[doc = "0x8c - SPI_W3_REG(i)"]
    pub spi_w3_reg: SPI_W3_REG,
    #[doc = "0x90 - SPI_W4_REG(i)"]
    pub spi_w4_reg: SPI_W4_REG,
    #[doc = "0x94 - SPI_W5_REG(i)"]
    pub spi_w5_reg: SPI_W5_REG,
    #[doc = "0x98 - SPI_W6_REG(i)"]
    pub spi_w6_reg: SPI_W6_REG,
    #[doc = "0x9c - SPI_W7_REG(i)"]
    pub spi_w7_reg: SPI_W7_REG,
    #[doc = "0xa0 - SPI_W8_REG(i)"]
    pub spi_w8_reg: SPI_W8_REG,
    #[doc = "0xa4 - SPI_W9_REG(i)"]
    pub spi_w9_reg: SPI_W9_REG,
    #[doc = "0xa8 - SPI_W10_REG(i)"]
    pub spi_w10_reg: SPI_W10_REG,
    #[doc = "0xac - SPI_W11_REG(i)"]
    pub spi_w11_reg: SPI_W11_REG,
    #[doc = "0xb0 - SPI_W12_REG(i)"]
    pub spi_w12_reg: SPI_W12_REG,
    #[doc = "0xb4 - SPI_W13_REG(i)"]
    pub spi_w13_reg: SPI_W13_REG,
    #[doc = "0xb8 - SPI_W14_REG(i)"]
    pub spi_w14_reg: SPI_W14_REG,
    #[doc = "0xbc - SPI_W15_REG(i)"]
    pub spi_w15_reg: SPI_W15_REG,
    #[doc = "0xc0 - SPI_TX_CRC_REG(i)"]
    pub spi_tx_crc_reg: SPI_TX_CRC_REG,
    _reserved42: [u8; 44usize],
    #[doc = "0xf0 - SPI_EXT0_REG(i)"]
    pub spi_ext0_reg: SPI_EXT0_REG,
    #[doc = "0xf4 - SPI_EXT1_REG(i)"]
    pub spi_ext1_reg: SPI_EXT1_REG,
    #[doc = "0xf8 - SPI_EXT2_REG(i)"]
    pub spi_ext2_reg: SPI_EXT2_REG,
    #[doc = "0xfc - SPI_EXT3_REG(i)"]
    pub spi_ext3_reg: SPI_EXT3_REG,
    #[doc = "0x100 - SPI_DMA_CONF_REG(i)"]
    pub spi_dma_conf_reg: SPI_DMA_CONF_REG,
    #[doc = "0x104 - SPI_DMA_OUT_LINK_REG(i)"]
    pub spi_dma_out_link_reg: SPI_DMA_OUT_LINK_REG,
    #[doc = "0x108 - SPI_DMA_IN_LINK_REG(i)"]
    pub spi_dma_in_link_reg: SPI_DMA_IN_LINK_REG,
    #[doc = "0x10c - SPI_DMA_STATUS_REG(i)"]
    pub spi_dma_status_reg: SPI_DMA_STATUS_REG,
    #[doc = "0x110 - SPI_DMA_INT_ENA_REG(i)"]
    pub spi_dma_int_ena_reg: SPI_DMA_INT_ENA_REG,
    #[doc = "0x114 - SPI_DMA_INT_RAW_REG(i)"]
    pub spi_dma_int_raw_reg: SPI_DMA_INT_RAW_REG,
    #[doc = "0x118 - SPI_DMA_INT_ST_REG(i)"]
    pub spi_dma_int_st_reg: SPI_DMA_INT_ST_REG,
    #[doc = "0x11c - SPI_DMA_INT_CLR_REG(i)"]
    pub spi_dma_int_clr_reg: SPI_DMA_INT_CLR_REG,
    #[doc = "0x120 - SPI_IN_ERR_EOF_DES_ADDR_REG(i)"]
    pub spi_in_err_eof_des_addr_reg: SPI_IN_ERR_EOF_DES_ADDR_REG,
    #[doc = "0x124 - SPI_IN_SUC_EOF_DES_ADDR_REG(i)"]
    pub spi_in_suc_eof_des_addr_reg: SPI_IN_SUC_EOF_DES_ADDR_REG,
    #[doc = "0x128 - SPI_INLINK_DSCR_REG(i)"]
    pub spi_inlink_dscr_reg: SPI_INLINK_DSCR_REG,
    #[doc = "0x12c - SPI_INLINK_DSCR_BF0_REG(i)"]
    pub spi_inlink_dscr_bf0_reg: SPI_INLINK_DSCR_BF0_REG,
    #[doc = "0x130 - SPI_INLINK_DSCR_BF1_REG(i)"]
    pub spi_inlink_dscr_bf1_reg: SPI_INLINK_DSCR_BF1_REG,
    #[doc = "0x134 - SPI_OUT_EOF_BFR_DES_ADDR_REG(i)"]
    pub spi_out_eof_bfr_des_addr_reg: SPI_OUT_EOF_BFR_DES_ADDR_REG,
    #[doc = "0x138 - SPI_OUT_EOF_DES_ADDR_REG(i)"]
    pub spi_out_eof_des_addr_reg: SPI_OUT_EOF_DES_ADDR_REG,
    #[doc = "0x13c - SPI_OUTLINK_DSCR_REG(i)"]
    pub spi_outlink_dscr_reg: SPI_OUTLINK_DSCR_REG,
    #[doc = "0x140 - SPI_OUTLINK_DSCR_BF0_REG(i)"]
    pub spi_outlink_dscr_bf0_reg: SPI_OUTLINK_DSCR_BF0_REG,
    #[doc = "0x144 - SPI_OUTLINK_DSCR_BF1_REG(i)"]
    pub spi_outlink_dscr_bf1_reg: SPI_OUTLINK_DSCR_BF1_REG,
    #[doc = "0x148 - SPI_DMA_RSTATUS_REG(i)"]
    pub spi_dma_rstatus_reg: SPI_DMA_RSTATUS_REG,
    #[doc = "0x14c - SPI_DMA_TSTATUS_REG(i)"]
    pub spi_dma_tstatus_reg: SPI_DMA_TSTATUS_REG,
    _reserved66: [u8; 684usize],
    #[doc = "0x3fc - SPI_DATE_REG(i)"]
    pub spi_date_reg: SPI_DATE_REG,
}
#[doc = "SPI_CMD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_cmd_reg](spi_cmd_reg) module"]
pub type SPI_CMD_REG = crate::Reg<u32, _SPI_CMD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CMD_REG;
#[doc = "`read()` method returns [spi_cmd_reg::R](spi_cmd_reg::R) reader structure"]
impl crate::Readable for SPI_CMD_REG {}
#[doc = "`write(|w| ..)` method takes [spi_cmd_reg::W](spi_cmd_reg::W) writer structure"]
impl crate::Writable for SPI_CMD_REG {}
#[doc = "SPI_CMD_REG(i)"]
pub mod spi_cmd_reg;
#[doc = "SPI_CTRL_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ctrl_reg](spi_ctrl_reg) module"]
pub type SPI_CTRL_REG = crate::Reg<u32, _SPI_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRL_REG;
#[doc = "`read()` method returns [spi_ctrl_reg::R](spi_ctrl_reg::R) reader structure"]
impl crate::Readable for SPI_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ctrl_reg::W](spi_ctrl_reg::W) writer structure"]
impl crate::Writable for SPI_CTRL_REG {}
#[doc = "SPI_CTRL_REG(i)"]
pub mod spi_ctrl_reg;
#[doc = "SPI_CTRL1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ctrl1_reg](spi_ctrl1_reg) module"]
pub type SPI_CTRL1_REG = crate::Reg<u32, _SPI_CTRL1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRL1_REG;
#[doc = "`read()` method returns [spi_ctrl1_reg::R](spi_ctrl1_reg::R) reader structure"]
impl crate::Readable for SPI_CTRL1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ctrl1_reg::W](spi_ctrl1_reg::W) writer structure"]
impl crate::Writable for SPI_CTRL1_REG {}
#[doc = "SPI_CTRL1_REG(i)"]
pub mod spi_ctrl1_reg;
#[doc = "SPI_RD_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_rd_status_reg](spi_rd_status_reg) module"]
pub type SPI_RD_STATUS_REG = crate::Reg<u32, _SPI_RD_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RD_STATUS_REG;
#[doc = "`read()` method returns [spi_rd_status_reg::R](spi_rd_status_reg::R) reader structure"]
impl crate::Readable for SPI_RD_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [spi_rd_status_reg::W](spi_rd_status_reg::W) writer structure"]
impl crate::Writable for SPI_RD_STATUS_REG {}
#[doc = "SPI_RD_STATUS_REG(i)"]
pub mod spi_rd_status_reg;
#[doc = "SPI_CTRL2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ctrl2_reg](spi_ctrl2_reg) module"]
pub type SPI_CTRL2_REG = crate::Reg<u32, _SPI_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRL2_REG;
#[doc = "`read()` method returns [spi_ctrl2_reg::R](spi_ctrl2_reg::R) reader structure"]
impl crate::Readable for SPI_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ctrl2_reg::W](spi_ctrl2_reg::W) writer structure"]
impl crate::Writable for SPI_CTRL2_REG {}
#[doc = "SPI_CTRL2_REG(i)"]
pub mod spi_ctrl2_reg;
#[doc = "SPI_CLOCK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_clock_reg](spi_clock_reg) module"]
pub type SPI_CLOCK_REG = crate::Reg<u32, _SPI_CLOCK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CLOCK_REG;
#[doc = "`read()` method returns [spi_clock_reg::R](spi_clock_reg::R) reader structure"]
impl crate::Readable for SPI_CLOCK_REG {}
#[doc = "`write(|w| ..)` method takes [spi_clock_reg::W](spi_clock_reg::W) writer structure"]
impl crate::Writable for SPI_CLOCK_REG {}
#[doc = "SPI_CLOCK_REG(i)"]
pub mod spi_clock_reg;
#[doc = "SPI_USER_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_user_reg](spi_user_reg) module"]
pub type SPI_USER_REG = crate::Reg<u32, _SPI_USER_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_USER_REG;
#[doc = "`read()` method returns [spi_user_reg::R](spi_user_reg::R) reader structure"]
impl crate::Readable for SPI_USER_REG {}
#[doc = "`write(|w| ..)` method takes [spi_user_reg::W](spi_user_reg::W) writer structure"]
impl crate::Writable for SPI_USER_REG {}
#[doc = "SPI_USER_REG(i)"]
pub mod spi_user_reg;
#[doc = "SPI_USER1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_user1_reg](spi_user1_reg) module"]
pub type SPI_USER1_REG = crate::Reg<u32, _SPI_USER1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_USER1_REG;
#[doc = "`read()` method returns [spi_user1_reg::R](spi_user1_reg::R) reader structure"]
impl crate::Readable for SPI_USER1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_user1_reg::W](spi_user1_reg::W) writer structure"]
impl crate::Writable for SPI_USER1_REG {}
#[doc = "SPI_USER1_REG(i)"]
pub mod spi_user1_reg;
#[doc = "SPI_USER2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_user2_reg](spi_user2_reg) module"]
pub type SPI_USER2_REG = crate::Reg<u32, _SPI_USER2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_USER2_REG;
#[doc = "`read()` method returns [spi_user2_reg::R](spi_user2_reg::R) reader structure"]
impl crate::Readable for SPI_USER2_REG {}
#[doc = "`write(|w| ..)` method takes [spi_user2_reg::W](spi_user2_reg::W) writer structure"]
impl crate::Writable for SPI_USER2_REG {}
#[doc = "SPI_USER2_REG(i)"]
pub mod spi_user2_reg;
#[doc = "SPI_MOSI_DLEN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_mosi_dlen_reg](spi_mosi_dlen_reg) module"]
pub type SPI_MOSI_DLEN_REG = crate::Reg<u32, _SPI_MOSI_DLEN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MOSI_DLEN_REG;
#[doc = "`read()` method returns [spi_mosi_dlen_reg::R](spi_mosi_dlen_reg::R) reader structure"]
impl crate::Readable for SPI_MOSI_DLEN_REG {}
#[doc = "`write(|w| ..)` method takes [spi_mosi_dlen_reg::W](spi_mosi_dlen_reg::W) writer structure"]
impl crate::Writable for SPI_MOSI_DLEN_REG {}
#[doc = "SPI_MOSI_DLEN_REG(i)"]
pub mod spi_mosi_dlen_reg;
#[doc = "SPI_MISO_DLEN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_miso_dlen_reg](spi_miso_dlen_reg) module"]
pub type SPI_MISO_DLEN_REG = crate::Reg<u32, _SPI_MISO_DLEN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MISO_DLEN_REG;
#[doc = "`read()` method returns [spi_miso_dlen_reg::R](spi_miso_dlen_reg::R) reader structure"]
impl crate::Readable for SPI_MISO_DLEN_REG {}
#[doc = "`write(|w| ..)` method takes [spi_miso_dlen_reg::W](spi_miso_dlen_reg::W) writer structure"]
impl crate::Writable for SPI_MISO_DLEN_REG {}
#[doc = "SPI_MISO_DLEN_REG(i)"]
pub mod spi_miso_dlen_reg;
#[doc = "SPI_SLV_WR_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slv_wr_status_reg](spi_slv_wr_status_reg) module"]
pub type SPI_SLV_WR_STATUS_REG = crate::Reg<u32, _SPI_SLV_WR_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLV_WR_STATUS_REG;
#[doc = "`read()` method returns [spi_slv_wr_status_reg::R](spi_slv_wr_status_reg::R) reader structure"]
impl crate::Readable for SPI_SLV_WR_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slv_wr_status_reg::W](spi_slv_wr_status_reg::W) writer structure"]
impl crate::Writable for SPI_SLV_WR_STATUS_REG {}
#[doc = "SPI_SLV_WR_STATUS_REG(i)"]
pub mod spi_slv_wr_status_reg;
#[doc = "SPI_PIN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_pin_reg](spi_pin_reg) module"]
pub type SPI_PIN_REG = crate::Reg<u32, _SPI_PIN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_PIN_REG;
#[doc = "`read()` method returns [spi_pin_reg::R](spi_pin_reg::R) reader structure"]
impl crate::Readable for SPI_PIN_REG {}
#[doc = "`write(|w| ..)` method takes [spi_pin_reg::W](spi_pin_reg::W) writer structure"]
impl crate::Writable for SPI_PIN_REG {}
#[doc = "SPI_PIN_REG(i)"]
pub mod spi_pin_reg;
#[doc = "SPI_SLAVE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slave_reg](spi_slave_reg) module"]
pub type SPI_SLAVE_REG = crate::Reg<u32, _SPI_SLAVE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLAVE_REG;
#[doc = "`read()` method returns [spi_slave_reg::R](spi_slave_reg::R) reader structure"]
impl crate::Readable for SPI_SLAVE_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slave_reg::W](spi_slave_reg::W) writer structure"]
impl crate::Writable for SPI_SLAVE_REG {}
#[doc = "SPI_SLAVE_REG(i)"]
pub mod spi_slave_reg;
#[doc = "SPI_SLAVE1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slave1_reg](spi_slave1_reg) module"]
pub type SPI_SLAVE1_REG = crate::Reg<u32, _SPI_SLAVE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLAVE1_REG;
#[doc = "`read()` method returns [spi_slave1_reg::R](spi_slave1_reg::R) reader structure"]
impl crate::Readable for SPI_SLAVE1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slave1_reg::W](spi_slave1_reg::W) writer structure"]
impl crate::Writable for SPI_SLAVE1_REG {}
#[doc = "SPI_SLAVE1_REG(i)"]
pub mod spi_slave1_reg;
#[doc = "SPI_SLAVE2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slave2_reg](spi_slave2_reg) module"]
pub type SPI_SLAVE2_REG = crate::Reg<u32, _SPI_SLAVE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLAVE2_REG;
#[doc = "`read()` method returns [spi_slave2_reg::R](spi_slave2_reg::R) reader structure"]
impl crate::Readable for SPI_SLAVE2_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slave2_reg::W](spi_slave2_reg::W) writer structure"]
impl crate::Writable for SPI_SLAVE2_REG {}
#[doc = "SPI_SLAVE2_REG(i)"]
pub mod spi_slave2_reg;
#[doc = "SPI_SLAVE3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slave3_reg](spi_slave3_reg) module"]
pub type SPI_SLAVE3_REG = crate::Reg<u32, _SPI_SLAVE3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLAVE3_REG;
#[doc = "`read()` method returns [spi_slave3_reg::R](spi_slave3_reg::R) reader structure"]
impl crate::Readable for SPI_SLAVE3_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slave3_reg::W](spi_slave3_reg::W) writer structure"]
impl crate::Writable for SPI_SLAVE3_REG {}
#[doc = "SPI_SLAVE3_REG(i)"]
pub mod spi_slave3_reg;
#[doc = "SPI_SLV_WRBUF_DLEN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slv_wrbuf_dlen_reg](spi_slv_wrbuf_dlen_reg) module"]
pub type SPI_SLV_WRBUF_DLEN_REG = crate::Reg<u32, _SPI_SLV_WRBUF_DLEN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLV_WRBUF_DLEN_REG;
#[doc = "`read()` method returns [spi_slv_wrbuf_dlen_reg::R](spi_slv_wrbuf_dlen_reg::R) reader structure"]
impl crate::Readable for SPI_SLV_WRBUF_DLEN_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slv_wrbuf_dlen_reg::W](spi_slv_wrbuf_dlen_reg::W) writer structure"]
impl crate::Writable for SPI_SLV_WRBUF_DLEN_REG {}
#[doc = "SPI_SLV_WRBUF_DLEN_REG(i)"]
pub mod spi_slv_wrbuf_dlen_reg;
#[doc = "SPI_SLV_RDBUF_DLEN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slv_rdbuf_dlen_reg](spi_slv_rdbuf_dlen_reg) module"]
pub type SPI_SLV_RDBUF_DLEN_REG = crate::Reg<u32, _SPI_SLV_RDBUF_DLEN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLV_RDBUF_DLEN_REG;
#[doc = "`read()` method returns [spi_slv_rdbuf_dlen_reg::R](spi_slv_rdbuf_dlen_reg::R) reader structure"]
impl crate::Readable for SPI_SLV_RDBUF_DLEN_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slv_rdbuf_dlen_reg::W](spi_slv_rdbuf_dlen_reg::W) writer structure"]
impl crate::Writable for SPI_SLV_RDBUF_DLEN_REG {}
#[doc = "SPI_SLV_RDBUF_DLEN_REG(i)"]
pub mod spi_slv_rdbuf_dlen_reg;
#[doc = "SPI_CACHE_FCTRL_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_cache_fctrl_reg](spi_cache_fctrl_reg) module"]
pub type SPI_CACHE_FCTRL_REG = crate::Reg<u32, _SPI_CACHE_FCTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CACHE_FCTRL_REG;
#[doc = "`read()` method returns [spi_cache_fctrl_reg::R](spi_cache_fctrl_reg::R) reader structure"]
impl crate::Readable for SPI_CACHE_FCTRL_REG {}
#[doc = "`write(|w| ..)` method takes [spi_cache_fctrl_reg::W](spi_cache_fctrl_reg::W) writer structure"]
impl crate::Writable for SPI_CACHE_FCTRL_REG {}
#[doc = "SPI_CACHE_FCTRL_REG(i)"]
pub mod spi_cache_fctrl_reg;
#[doc = "SPI_CACHE_SCTRL_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_cache_sctrl_reg](spi_cache_sctrl_reg) module"]
pub type SPI_CACHE_SCTRL_REG = crate::Reg<u32, _SPI_CACHE_SCTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CACHE_SCTRL_REG;
#[doc = "`read()` method returns [spi_cache_sctrl_reg::R](spi_cache_sctrl_reg::R) reader structure"]
impl crate::Readable for SPI_CACHE_SCTRL_REG {}
#[doc = "`write(|w| ..)` method takes [spi_cache_sctrl_reg::W](spi_cache_sctrl_reg::W) writer structure"]
impl crate::Writable for SPI_CACHE_SCTRL_REG {}
#[doc = "SPI_CACHE_SCTRL_REG(i)"]
pub mod spi_cache_sctrl_reg;
#[doc = "SPI_SRAM_CMD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_sram_cmd_reg](spi_sram_cmd_reg) module"]
pub type SPI_SRAM_CMD_REG = crate::Reg<u32, _SPI_SRAM_CMD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SRAM_CMD_REG;
#[doc = "`read()` method returns [spi_sram_cmd_reg::R](spi_sram_cmd_reg::R) reader structure"]
impl crate::Readable for SPI_SRAM_CMD_REG {}
#[doc = "`write(|w| ..)` method takes [spi_sram_cmd_reg::W](spi_sram_cmd_reg::W) writer structure"]
impl crate::Writable for SPI_SRAM_CMD_REG {}
#[doc = "SPI_SRAM_CMD_REG(i)"]
pub mod spi_sram_cmd_reg;
#[doc = "SPI_SRAM_DRD_CMD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_sram_drd_cmd_reg](spi_sram_drd_cmd_reg) module"]
pub type SPI_SRAM_DRD_CMD_REG = crate::Reg<u32, _SPI_SRAM_DRD_CMD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SRAM_DRD_CMD_REG;
#[doc = "`read()` method returns [spi_sram_drd_cmd_reg::R](spi_sram_drd_cmd_reg::R) reader structure"]
impl crate::Readable for SPI_SRAM_DRD_CMD_REG {}
#[doc = "`write(|w| ..)` method takes [spi_sram_drd_cmd_reg::W](spi_sram_drd_cmd_reg::W) writer structure"]
impl crate::Writable for SPI_SRAM_DRD_CMD_REG {}
#[doc = "SPI_SRAM_DRD_CMD_REG(i)"]
pub mod spi_sram_drd_cmd_reg;
#[doc = "SPI_SRAM_DWR_CMD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_sram_dwr_cmd_reg](spi_sram_dwr_cmd_reg) module"]
pub type SPI_SRAM_DWR_CMD_REG = crate::Reg<u32, _SPI_SRAM_DWR_CMD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SRAM_DWR_CMD_REG;
#[doc = "`read()` method returns [spi_sram_dwr_cmd_reg::R](spi_sram_dwr_cmd_reg::R) reader structure"]
impl crate::Readable for SPI_SRAM_DWR_CMD_REG {}
#[doc = "`write(|w| ..)` method takes [spi_sram_dwr_cmd_reg::W](spi_sram_dwr_cmd_reg::W) writer structure"]
impl crate::Writable for SPI_SRAM_DWR_CMD_REG {}
#[doc = "SPI_SRAM_DWR_CMD_REG(i)"]
pub mod spi_sram_dwr_cmd_reg;
#[doc = "SPI_SLV_RD_BIT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_slv_rd_bit_reg](spi_slv_rd_bit_reg) module"]
pub type SPI_SLV_RD_BIT_REG = crate::Reg<u32, _SPI_SLV_RD_BIT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLV_RD_BIT_REG;
#[doc = "`read()` method returns [spi_slv_rd_bit_reg::R](spi_slv_rd_bit_reg::R) reader structure"]
impl crate::Readable for SPI_SLV_RD_BIT_REG {}
#[doc = "`write(|w| ..)` method takes [spi_slv_rd_bit_reg::W](spi_slv_rd_bit_reg::W) writer structure"]
impl crate::Writable for SPI_SLV_RD_BIT_REG {}
#[doc = "SPI_SLV_RD_BIT_REG(i)"]
pub mod spi_slv_rd_bit_reg;
#[doc = "SPI_W0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w0_reg](spi_w0_reg) module"]
pub type SPI_W0_REG = crate::Reg<u32, _SPI_W0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W0_REG;
#[doc = "`read()` method returns [spi_w0_reg::R](spi_w0_reg::R) reader structure"]
impl crate::Readable for SPI_W0_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w0_reg::W](spi_w0_reg::W) writer structure"]
impl crate::Writable for SPI_W0_REG {}
#[doc = "SPI_W0_REG(i)"]
pub mod spi_w0_reg;
#[doc = "SPI_W1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w1_reg](spi_w1_reg) module"]
pub type SPI_W1_REG = crate::Reg<u32, _SPI_W1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W1_REG;
#[doc = "`read()` method returns [spi_w1_reg::R](spi_w1_reg::R) reader structure"]
impl crate::Readable for SPI_W1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w1_reg::W](spi_w1_reg::W) writer structure"]
impl crate::Writable for SPI_W1_REG {}
#[doc = "SPI_W1_REG(i)"]
pub mod spi_w1_reg;
#[doc = "SPI_W2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w2_reg](spi_w2_reg) module"]
pub type SPI_W2_REG = crate::Reg<u32, _SPI_W2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W2_REG;
#[doc = "`read()` method returns [spi_w2_reg::R](spi_w2_reg::R) reader structure"]
impl crate::Readable for SPI_W2_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w2_reg::W](spi_w2_reg::W) writer structure"]
impl crate::Writable for SPI_W2_REG {}
#[doc = "SPI_W2_REG(i)"]
pub mod spi_w2_reg;
#[doc = "SPI_W3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w3_reg](spi_w3_reg) module"]
pub type SPI_W3_REG = crate::Reg<u32, _SPI_W3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W3_REG;
#[doc = "`read()` method returns [spi_w3_reg::R](spi_w3_reg::R) reader structure"]
impl crate::Readable for SPI_W3_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w3_reg::W](spi_w3_reg::W) writer structure"]
impl crate::Writable for SPI_W3_REG {}
#[doc = "SPI_W3_REG(i)"]
pub mod spi_w3_reg;
#[doc = "SPI_W4_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w4_reg](spi_w4_reg) module"]
pub type SPI_W4_REG = crate::Reg<u32, _SPI_W4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W4_REG;
#[doc = "`read()` method returns [spi_w4_reg::R](spi_w4_reg::R) reader structure"]
impl crate::Readable for SPI_W4_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w4_reg::W](spi_w4_reg::W) writer structure"]
impl crate::Writable for SPI_W4_REG {}
#[doc = "SPI_W4_REG(i)"]
pub mod spi_w4_reg;
#[doc = "SPI_W5_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w5_reg](spi_w5_reg) module"]
pub type SPI_W5_REG = crate::Reg<u32, _SPI_W5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W5_REG;
#[doc = "`read()` method returns [spi_w5_reg::R](spi_w5_reg::R) reader structure"]
impl crate::Readable for SPI_W5_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w5_reg::W](spi_w5_reg::W) writer structure"]
impl crate::Writable for SPI_W5_REG {}
#[doc = "SPI_W5_REG(i)"]
pub mod spi_w5_reg;
#[doc = "SPI_W6_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w6_reg](spi_w6_reg) module"]
pub type SPI_W6_REG = crate::Reg<u32, _SPI_W6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W6_REG;
#[doc = "`read()` method returns [spi_w6_reg::R](spi_w6_reg::R) reader structure"]
impl crate::Readable for SPI_W6_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w6_reg::W](spi_w6_reg::W) writer structure"]
impl crate::Writable for SPI_W6_REG {}
#[doc = "SPI_W6_REG(i)"]
pub mod spi_w6_reg;
#[doc = "SPI_W7_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w7_reg](spi_w7_reg) module"]
pub type SPI_W7_REG = crate::Reg<u32, _SPI_W7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W7_REG;
#[doc = "`read()` method returns [spi_w7_reg::R](spi_w7_reg::R) reader structure"]
impl crate::Readable for SPI_W7_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w7_reg::W](spi_w7_reg::W) writer structure"]
impl crate::Writable for SPI_W7_REG {}
#[doc = "SPI_W7_REG(i)"]
pub mod spi_w7_reg;
#[doc = "SPI_W8_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w8_reg](spi_w8_reg) module"]
pub type SPI_W8_REG = crate::Reg<u32, _SPI_W8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W8_REG;
#[doc = "`read()` method returns [spi_w8_reg::R](spi_w8_reg::R) reader structure"]
impl crate::Readable for SPI_W8_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w8_reg::W](spi_w8_reg::W) writer structure"]
impl crate::Writable for SPI_W8_REG {}
#[doc = "SPI_W8_REG(i)"]
pub mod spi_w8_reg;
#[doc = "SPI_W9_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w9_reg](spi_w9_reg) module"]
pub type SPI_W9_REG = crate::Reg<u32, _SPI_W9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W9_REG;
#[doc = "`read()` method returns [spi_w9_reg::R](spi_w9_reg::R) reader structure"]
impl crate::Readable for SPI_W9_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w9_reg::W](spi_w9_reg::W) writer structure"]
impl crate::Writable for SPI_W9_REG {}
#[doc = "SPI_W9_REG(i)"]
pub mod spi_w9_reg;
#[doc = "SPI_W10_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w10_reg](spi_w10_reg) module"]
pub type SPI_W10_REG = crate::Reg<u32, _SPI_W10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W10_REG;
#[doc = "`read()` method returns [spi_w10_reg::R](spi_w10_reg::R) reader structure"]
impl crate::Readable for SPI_W10_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w10_reg::W](spi_w10_reg::W) writer structure"]
impl crate::Writable for SPI_W10_REG {}
#[doc = "SPI_W10_REG(i)"]
pub mod spi_w10_reg;
#[doc = "SPI_W11_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w11_reg](spi_w11_reg) module"]
pub type SPI_W11_REG = crate::Reg<u32, _SPI_W11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W11_REG;
#[doc = "`read()` method returns [spi_w11_reg::R](spi_w11_reg::R) reader structure"]
impl crate::Readable for SPI_W11_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w11_reg::W](spi_w11_reg::W) writer structure"]
impl crate::Writable for SPI_W11_REG {}
#[doc = "SPI_W11_REG(i)"]
pub mod spi_w11_reg;
#[doc = "SPI_W12_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w12_reg](spi_w12_reg) module"]
pub type SPI_W12_REG = crate::Reg<u32, _SPI_W12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W12_REG;
#[doc = "`read()` method returns [spi_w12_reg::R](spi_w12_reg::R) reader structure"]
impl crate::Readable for SPI_W12_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w12_reg::W](spi_w12_reg::W) writer structure"]
impl crate::Writable for SPI_W12_REG {}
#[doc = "SPI_W12_REG(i)"]
pub mod spi_w12_reg;
#[doc = "SPI_W13_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w13_reg](spi_w13_reg) module"]
pub type SPI_W13_REG = crate::Reg<u32, _SPI_W13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W13_REG;
#[doc = "`read()` method returns [spi_w13_reg::R](spi_w13_reg::R) reader structure"]
impl crate::Readable for SPI_W13_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w13_reg::W](spi_w13_reg::W) writer structure"]
impl crate::Writable for SPI_W13_REG {}
#[doc = "SPI_W13_REG(i)"]
pub mod spi_w13_reg;
#[doc = "SPI_W14_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w14_reg](spi_w14_reg) module"]
pub type SPI_W14_REG = crate::Reg<u32, _SPI_W14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W14_REG;
#[doc = "`read()` method returns [spi_w14_reg::R](spi_w14_reg::R) reader structure"]
impl crate::Readable for SPI_W14_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w14_reg::W](spi_w14_reg::W) writer structure"]
impl crate::Writable for SPI_W14_REG {}
#[doc = "SPI_W14_REG(i)"]
pub mod spi_w14_reg;
#[doc = "SPI_W15_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_w15_reg](spi_w15_reg) module"]
pub type SPI_W15_REG = crate::Reg<u32, _SPI_W15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W15_REG;
#[doc = "`read()` method returns [spi_w15_reg::R](spi_w15_reg::R) reader structure"]
impl crate::Readable for SPI_W15_REG {}
#[doc = "`write(|w| ..)` method takes [spi_w15_reg::W](spi_w15_reg::W) writer structure"]
impl crate::Writable for SPI_W15_REG {}
#[doc = "SPI_W15_REG(i)"]
pub mod spi_w15_reg;
#[doc = "SPI_TX_CRC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_tx_crc_reg](spi_tx_crc_reg) module"]
pub type SPI_TX_CRC_REG = crate::Reg<u32, _SPI_TX_CRC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_TX_CRC_REG;
#[doc = "`read()` method returns [spi_tx_crc_reg::R](spi_tx_crc_reg::R) reader structure"]
impl crate::Readable for SPI_TX_CRC_REG {}
#[doc = "`write(|w| ..)` method takes [spi_tx_crc_reg::W](spi_tx_crc_reg::W) writer structure"]
impl crate::Writable for SPI_TX_CRC_REG {}
#[doc = "SPI_TX_CRC_REG(i)"]
pub mod spi_tx_crc_reg;
#[doc = "SPI_EXT0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ext0_reg](spi_ext0_reg) module"]
pub type SPI_EXT0_REG = crate::Reg<u32, _SPI_EXT0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_EXT0_REG;
#[doc = "`read()` method returns [spi_ext0_reg::R](spi_ext0_reg::R) reader structure"]
impl crate::Readable for SPI_EXT0_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ext0_reg::W](spi_ext0_reg::W) writer structure"]
impl crate::Writable for SPI_EXT0_REG {}
#[doc = "SPI_EXT0_REG(i)"]
pub mod spi_ext0_reg;
#[doc = "SPI_EXT1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ext1_reg](spi_ext1_reg) module"]
pub type SPI_EXT1_REG = crate::Reg<u32, _SPI_EXT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_EXT1_REG;
#[doc = "`read()` method returns [spi_ext1_reg::R](spi_ext1_reg::R) reader structure"]
impl crate::Readable for SPI_EXT1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ext1_reg::W](spi_ext1_reg::W) writer structure"]
impl crate::Writable for SPI_EXT1_REG {}
#[doc = "SPI_EXT1_REG(i)"]
pub mod spi_ext1_reg;
#[doc = "SPI_EXT2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ext2_reg](spi_ext2_reg) module"]
pub type SPI_EXT2_REG = crate::Reg<u32, _SPI_EXT2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_EXT2_REG;
#[doc = "`read()` method returns [spi_ext2_reg::R](spi_ext2_reg::R) reader structure"]
impl crate::Readable for SPI_EXT2_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ext2_reg::W](spi_ext2_reg::W) writer structure"]
impl crate::Writable for SPI_EXT2_REG {}
#[doc = "SPI_EXT2_REG(i)"]
pub mod spi_ext2_reg;
#[doc = "SPI_EXT3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ext3_reg](spi_ext3_reg) module"]
pub type SPI_EXT3_REG = crate::Reg<u32, _SPI_EXT3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_EXT3_REG;
#[doc = "`read()` method returns [spi_ext3_reg::R](spi_ext3_reg::R) reader structure"]
impl crate::Readable for SPI_EXT3_REG {}
#[doc = "`write(|w| ..)` method takes [spi_ext3_reg::W](spi_ext3_reg::W) writer structure"]
impl crate::Writable for SPI_EXT3_REG {}
#[doc = "SPI_EXT3_REG(i)"]
pub mod spi_ext3_reg;
#[doc = "SPI_DMA_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_conf_reg](spi_dma_conf_reg) module"]
pub type SPI_DMA_CONF_REG = crate::Reg<u32, _SPI_DMA_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_CONF_REG;
#[doc = "`read()` method returns [spi_dma_conf_reg::R](spi_dma_conf_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_conf_reg::W](spi_dma_conf_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_CONF_REG {}
#[doc = "SPI_DMA_CONF_REG(i)"]
pub mod spi_dma_conf_reg;
#[doc = "SPI_DMA_OUT_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_out_link_reg](spi_dma_out_link_reg) module"]
pub type SPI_DMA_OUT_LINK_REG = crate::Reg<u32, _SPI_DMA_OUT_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_OUT_LINK_REG;
#[doc = "`read()` method returns [spi_dma_out_link_reg::R](spi_dma_out_link_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_OUT_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_out_link_reg::W](spi_dma_out_link_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_OUT_LINK_REG {}
#[doc = "SPI_DMA_OUT_LINK_REG(i)"]
pub mod spi_dma_out_link_reg;
#[doc = "SPI_DMA_IN_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_in_link_reg](spi_dma_in_link_reg) module"]
pub type SPI_DMA_IN_LINK_REG = crate::Reg<u32, _SPI_DMA_IN_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_IN_LINK_REG;
#[doc = "`read()` method returns [spi_dma_in_link_reg::R](spi_dma_in_link_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_IN_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_in_link_reg::W](spi_dma_in_link_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_IN_LINK_REG {}
#[doc = "SPI_DMA_IN_LINK_REG(i)"]
pub mod spi_dma_in_link_reg;
#[doc = "SPI_DMA_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_status_reg](spi_dma_status_reg) module"]
pub type SPI_DMA_STATUS_REG = crate::Reg<u32, _SPI_DMA_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_STATUS_REG;
#[doc = "`read()` method returns [spi_dma_status_reg::R](spi_dma_status_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_status_reg::W](spi_dma_status_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_STATUS_REG {}
#[doc = "SPI_DMA_STATUS_REG(i)"]
pub mod spi_dma_status_reg;
#[doc = "SPI_DMA_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_int_ena_reg](spi_dma_int_ena_reg) module"]
pub type SPI_DMA_INT_ENA_REG = crate::Reg<u32, _SPI_DMA_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_ENA_REG;
#[doc = "`read()` method returns [spi_dma_int_ena_reg::R](spi_dma_int_ena_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_ena_reg::W](spi_dma_int_ena_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_ENA_REG {}
#[doc = "SPI_DMA_INT_ENA_REG(i)"]
pub mod spi_dma_int_ena_reg;
#[doc = "SPI_DMA_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_int_raw_reg](spi_dma_int_raw_reg) module"]
pub type SPI_DMA_INT_RAW_REG = crate::Reg<u32, _SPI_DMA_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_RAW_REG;
#[doc = "`read()` method returns [spi_dma_int_raw_reg::R](spi_dma_int_raw_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_raw_reg::W](spi_dma_int_raw_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_RAW_REG {}
#[doc = "SPI_DMA_INT_RAW_REG(i)"]
pub mod spi_dma_int_raw_reg;
#[doc = "SPI_DMA_INT_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_int_st_reg](spi_dma_int_st_reg) module"]
pub type SPI_DMA_INT_ST_REG = crate::Reg<u32, _SPI_DMA_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_ST_REG;
#[doc = "`read()` method returns [spi_dma_int_st_reg::R](spi_dma_int_st_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_st_reg::W](spi_dma_int_st_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_ST_REG {}
#[doc = "SPI_DMA_INT_ST_REG(i)"]
pub mod spi_dma_int_st_reg;
#[doc = "SPI_DMA_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_int_clr_reg](spi_dma_int_clr_reg) module"]
pub type SPI_DMA_INT_CLR_REG = crate::Reg<u32, _SPI_DMA_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_CLR_REG;
#[doc = "`read()` method returns [spi_dma_int_clr_reg::R](spi_dma_int_clr_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_clr_reg::W](spi_dma_int_clr_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_CLR_REG {}
#[doc = "SPI_DMA_INT_CLR_REG(i)"]
pub mod spi_dma_int_clr_reg;
#[doc = "SPI_IN_ERR_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_in_err_eof_des_addr_reg](spi_in_err_eof_des_addr_reg) module"]
pub type SPI_IN_ERR_EOF_DES_ADDR_REG = crate::Reg<u32, _SPI_IN_ERR_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IN_ERR_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [spi_in_err_eof_des_addr_reg::R](spi_in_err_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SPI_IN_ERR_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_in_err_eof_des_addr_reg::W](spi_in_err_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SPI_IN_ERR_EOF_DES_ADDR_REG {}
#[doc = "SPI_IN_ERR_EOF_DES_ADDR_REG(i)"]
pub mod spi_in_err_eof_des_addr_reg;
#[doc = "SPI_IN_SUC_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_in_suc_eof_des_addr_reg](spi_in_suc_eof_des_addr_reg) module"]
pub type SPI_IN_SUC_EOF_DES_ADDR_REG = crate::Reg<u32, _SPI_IN_SUC_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IN_SUC_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [spi_in_suc_eof_des_addr_reg::R](spi_in_suc_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SPI_IN_SUC_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_in_suc_eof_des_addr_reg::W](spi_in_suc_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SPI_IN_SUC_EOF_DES_ADDR_REG {}
#[doc = "SPI_IN_SUC_EOF_DES_ADDR_REG(i)"]
pub mod spi_in_suc_eof_des_addr_reg;
#[doc = "SPI_INLINK_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_inlink_dscr_reg](spi_inlink_dscr_reg) module"]
pub type SPI_INLINK_DSCR_REG = crate::Reg<u32, _SPI_INLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_INLINK_DSCR_REG;
#[doc = "`read()` method returns [spi_inlink_dscr_reg::R](spi_inlink_dscr_reg::R) reader structure"]
impl crate::Readable for SPI_INLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_inlink_dscr_reg::W](spi_inlink_dscr_reg::W) writer structure"]
impl crate::Writable for SPI_INLINK_DSCR_REG {}
#[doc = "SPI_INLINK_DSCR_REG(i)"]
pub mod spi_inlink_dscr_reg;
#[doc = "SPI_INLINK_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_inlink_dscr_bf0_reg](spi_inlink_dscr_bf0_reg) module"]
pub type SPI_INLINK_DSCR_BF0_REG = crate::Reg<u32, _SPI_INLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_INLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [spi_inlink_dscr_bf0_reg::R](spi_inlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for SPI_INLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [spi_inlink_dscr_bf0_reg::W](spi_inlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for SPI_INLINK_DSCR_BF0_REG {}
#[doc = "SPI_INLINK_DSCR_BF0_REG(i)"]
pub mod spi_inlink_dscr_bf0_reg;
#[doc = "SPI_INLINK_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_inlink_dscr_bf1_reg](spi_inlink_dscr_bf1_reg) module"]
pub type SPI_INLINK_DSCR_BF1_REG = crate::Reg<u32, _SPI_INLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_INLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [spi_inlink_dscr_bf1_reg::R](spi_inlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for SPI_INLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_inlink_dscr_bf1_reg::W](spi_inlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for SPI_INLINK_DSCR_BF1_REG {}
#[doc = "SPI_INLINK_DSCR_BF1_REG(i)"]
pub mod spi_inlink_dscr_bf1_reg;
#[doc = "SPI_OUT_EOF_BFR_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_out_eof_bfr_des_addr_reg](spi_out_eof_bfr_des_addr_reg) module"]
pub type SPI_OUT_EOF_BFR_DES_ADDR_REG = crate::Reg<u32, _SPI_OUT_EOF_BFR_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_OUT_EOF_BFR_DES_ADDR_REG;
#[doc = "`read()` method returns [spi_out_eof_bfr_des_addr_reg::R](spi_out_eof_bfr_des_addr_reg::R) reader structure"]
impl crate::Readable for SPI_OUT_EOF_BFR_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_out_eof_bfr_des_addr_reg::W](spi_out_eof_bfr_des_addr_reg::W) writer structure"]
impl crate::Writable for SPI_OUT_EOF_BFR_DES_ADDR_REG {}
#[doc = "SPI_OUT_EOF_BFR_DES_ADDR_REG(i)"]
pub mod spi_out_eof_bfr_des_addr_reg;
#[doc = "SPI_OUT_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_out_eof_des_addr_reg](spi_out_eof_des_addr_reg) module"]
pub type SPI_OUT_EOF_DES_ADDR_REG = crate::Reg<u32, _SPI_OUT_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_OUT_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [spi_out_eof_des_addr_reg::R](spi_out_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SPI_OUT_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_out_eof_des_addr_reg::W](spi_out_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SPI_OUT_EOF_DES_ADDR_REG {}
#[doc = "SPI_OUT_EOF_DES_ADDR_REG(i)"]
pub mod spi_out_eof_des_addr_reg;
#[doc = "SPI_OUTLINK_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_outlink_dscr_reg](spi_outlink_dscr_reg) module"]
pub type SPI_OUTLINK_DSCR_REG = crate::Reg<u32, _SPI_OUTLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_OUTLINK_DSCR_REG;
#[doc = "`read()` method returns [spi_outlink_dscr_reg::R](spi_outlink_dscr_reg::R) reader structure"]
impl crate::Readable for SPI_OUTLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [spi_outlink_dscr_reg::W](spi_outlink_dscr_reg::W) writer structure"]
impl crate::Writable for SPI_OUTLINK_DSCR_REG {}
#[doc = "SPI_OUTLINK_DSCR_REG(i)"]
pub mod spi_outlink_dscr_reg;
#[doc = "SPI_OUTLINK_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_outlink_dscr_bf0_reg](spi_outlink_dscr_bf0_reg) module"]
pub type SPI_OUTLINK_DSCR_BF0_REG = crate::Reg<u32, _SPI_OUTLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_OUTLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [spi_outlink_dscr_bf0_reg::R](spi_outlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for SPI_OUTLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [spi_outlink_dscr_bf0_reg::W](spi_outlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for SPI_OUTLINK_DSCR_BF0_REG {}
#[doc = "SPI_OUTLINK_DSCR_BF0_REG(i)"]
pub mod spi_outlink_dscr_bf0_reg;
#[doc = "SPI_OUTLINK_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_outlink_dscr_bf1_reg](spi_outlink_dscr_bf1_reg) module"]
pub type SPI_OUTLINK_DSCR_BF1_REG = crate::Reg<u32, _SPI_OUTLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_OUTLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [spi_outlink_dscr_bf1_reg::R](spi_outlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for SPI_OUTLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [spi_outlink_dscr_bf1_reg::W](spi_outlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for SPI_OUTLINK_DSCR_BF1_REG {}
#[doc = "SPI_OUTLINK_DSCR_BF1_REG(i)"]
pub mod spi_outlink_dscr_bf1_reg;
#[doc = "SPI_DMA_RSTATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_rstatus_reg](spi_dma_rstatus_reg) module"]
pub type SPI_DMA_RSTATUS_REG = crate::Reg<u32, _SPI_DMA_RSTATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_RSTATUS_REG;
#[doc = "`read()` method returns [spi_dma_rstatus_reg::R](spi_dma_rstatus_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_RSTATUS_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_rstatus_reg::W](spi_dma_rstatus_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_RSTATUS_REG {}
#[doc = "SPI_DMA_RSTATUS_REG(i)"]
pub mod spi_dma_rstatus_reg;
#[doc = "SPI_DMA_TSTATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_tstatus_reg](spi_dma_tstatus_reg) module"]
pub type SPI_DMA_TSTATUS_REG = crate::Reg<u32, _SPI_DMA_TSTATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_TSTATUS_REG;
#[doc = "`read()` method returns [spi_dma_tstatus_reg::R](spi_dma_tstatus_reg::R) reader structure"]
impl crate::Readable for SPI_DMA_TSTATUS_REG {}
#[doc = "`write(|w| ..)` method takes [spi_dma_tstatus_reg::W](spi_dma_tstatus_reg::W) writer structure"]
impl crate::Writable for SPI_DMA_TSTATUS_REG {}
#[doc = "SPI_DMA_TSTATUS_REG(i)"]
pub mod spi_dma_tstatus_reg;
#[doc = "SPI_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_date_reg](spi_date_reg) module"]
pub type SPI_DATE_REG = crate::Reg<u32, _SPI_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DATE_REG;
#[doc = "`read()` method returns [spi_date_reg::R](spi_date_reg::R) reader structure"]
impl crate::Readable for SPI_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [spi_date_reg::W](spi_date_reg::W) writer structure"]
impl crate::Writable for SPI_DATE_REG {}
#[doc = "SPI_DATE_REG(i)"]
pub mod spi_date_reg;
