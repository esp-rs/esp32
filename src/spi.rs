#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CMD"]
    pub cmd: CMD,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - SPI_CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x0c - SPI_CTRL1"]
    pub ctrl1: CTRL1,
    #[doc = "0x10 - SPI_RD_STATUS"]
    pub rd_status: RD_STATUS,
    #[doc = "0x14 - SPI_CTRL2"]
    pub ctrl2: CTRL2,
    #[doc = "0x18 - SPI_CLOCK"]
    pub clock: CLOCK,
    #[doc = "0x1c - SPI_USER"]
    pub user: USER,
    #[doc = "0x20 - SPI_USER1"]
    pub user1: USER1,
    #[doc = "0x24 - SPI_USER2"]
    pub user2: USER2,
    #[doc = "0x28 - SPI_MOSI_DLEN"]
    pub mosi_dlen: MOSI_DLEN,
    #[doc = "0x2c - SPI_MISO_DLEN"]
    pub miso_dlen: MISO_DLEN,
    #[doc = "0x30 - SPI_SLV_WR_STATUS"]
    pub slv_wr_status: SLV_WR_STATUS,
    #[doc = "0x34 - SPI_PIN"]
    pub pin: PIN,
    #[doc = "0x38 - SPI_SLAVE"]
    pub slave: SLAVE,
    #[doc = "0x3c - SPI_SLAVE1"]
    pub slave1: SLAVE1,
    #[doc = "0x40 - SPI_SLAVE2"]
    pub slave2: SLAVE2,
    #[doc = "0x44 - SPI_SLAVE3"]
    pub slave3: SLAVE3,
    #[doc = "0x48 - SPI_SLV_WRBUF_DLEN"]
    pub slv_wrbuf_dlen: SLV_WRBUF_DLEN,
    #[doc = "0x4c - SPI_SLV_RDBUF_DLEN"]
    pub slv_rdbuf_dlen: SLV_RDBUF_DLEN,
    #[doc = "0x50 - SPI_CACHE_FCTRL"]
    pub cache_fctrl: CACHE_FCTRL,
    #[doc = "0x54 - SPI_CACHE_SCTRL"]
    pub cache_sctrl: CACHE_SCTRL,
    #[doc = "0x58 - SPI_SRAM_CMD"]
    pub sram_cmd: SRAM_CMD,
    #[doc = "0x5c - SPI_SRAM_DRD_CMD"]
    pub sram_drd_cmd: SRAM_DRD_CMD,
    #[doc = "0x60 - SPI_SRAM_DWR_CMD"]
    pub sram_dwr_cmd: SRAM_DWR_CMD,
    #[doc = "0x64 - SPI_SLV_RD_BIT"]
    pub slv_rd_bit: SLV_RD_BIT,
    _reserved25: [u8; 24usize],
    #[doc = "0x80 - SPI_W0"]
    pub w0: W0,
    #[doc = "0x84 - SPI_W1"]
    pub w1: W1,
    #[doc = "0x88 - SPI_W2"]
    pub w2: W2,
    #[doc = "0x8c - SPI_W3"]
    pub w3: W3,
    #[doc = "0x90 - SPI_W4"]
    pub w4: W4,
    #[doc = "0x94 - SPI_W5"]
    pub w5: W5,
    #[doc = "0x98 - SPI_W6"]
    pub w6: W6,
    #[doc = "0x9c - SPI_W7"]
    pub w7: W7,
    #[doc = "0xa0 - SPI_W8"]
    pub w8: W8,
    #[doc = "0xa4 - SPI_W9"]
    pub w9: W9,
    #[doc = "0xa8 - SPI_W10"]
    pub w10: W10,
    #[doc = "0xac - SPI_W11"]
    pub w11: W11,
    #[doc = "0xb0 - SPI_W12"]
    pub w12: W12,
    #[doc = "0xb4 - SPI_W13"]
    pub w13: W13,
    #[doc = "0xb8 - SPI_W14"]
    pub w14: W14,
    #[doc = "0xbc - SPI_W15"]
    pub w15: W15,
    #[doc = "0xc0 - SPI_TX_CRC"]
    pub tx_crc: TX_CRC,
    _reserved42: [u8; 44usize],
    #[doc = "0xf0 - SPI_EXT0"]
    pub ext0: EXT0,
    #[doc = "0xf4 - SPI_EXT1"]
    pub ext1: EXT1,
    #[doc = "0xf8 - SPI_EXT2"]
    pub ext2: EXT2,
    #[doc = "0xfc - SPI_EXT3"]
    pub ext3: EXT3,
    #[doc = "0x100 - SPI_DMA_CONF"]
    pub dma_conf: DMA_CONF,
    #[doc = "0x104 - SPI_DMA_OUT_LINK"]
    pub dma_out_link: DMA_OUT_LINK,
    #[doc = "0x108 - SPI_DMA_IN_LINK"]
    pub dma_in_link: DMA_IN_LINK,
    #[doc = "0x10c - SPI_DMA_STATUS"]
    pub dma_status: DMA_STATUS,
    #[doc = "0x110 - SPI_DMA_INT_ENA"]
    pub dma_int_ena: DMA_INT_ENA,
    #[doc = "0x114 - SPI_DMA_INT_RAW"]
    pub dma_int_raw: DMA_INT_RAW,
    #[doc = "0x118 - SPI_DMA_INT_ST"]
    pub dma_int_st: DMA_INT_ST,
    #[doc = "0x11c - SPI_DMA_INT_CLR"]
    pub dma_int_clr: DMA_INT_CLR,
    #[doc = "0x120 - SPI_IN_ERR_EOF_DES_ADDR"]
    pub in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    #[doc = "0x124 - SPI_IN_SUC_EOF_DES_ADDR"]
    pub in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    #[doc = "0x128 - SPI_INLINK_DSCR"]
    pub inlink_dscr: INLINK_DSCR,
    #[doc = "0x12c - SPI_INLINK_DSCR_BF0"]
    pub inlink_dscr_bf0: INLINK_DSCR_BF0,
    #[doc = "0x130 - SPI_INLINK_DSCR_BF1"]
    pub inlink_dscr_bf1: INLINK_DSCR_BF1,
    #[doc = "0x134 - SPI_OUT_EOF_BFR_DES_ADDR"]
    pub out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x138 - SPI_OUT_EOF_DES_ADDR"]
    pub out_eof_des_addr: OUT_EOF_DES_ADDR,
    #[doc = "0x13c - SPI_OUTLINK_DSCR"]
    pub outlink_dscr: OUTLINK_DSCR,
    #[doc = "0x140 - SPI_OUTLINK_DSCR_BF0"]
    pub outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    #[doc = "0x144 - SPI_OUTLINK_DSCR_BF1"]
    pub outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    #[doc = "0x148 - SPI_DMA_RSTATUS"]
    pub dma_rstatus: DMA_RSTATUS,
    #[doc = "0x14c - SPI_DMA_TSTATUS"]
    pub dma_tstatus: DMA_TSTATUS,
    _reserved66: [u8; 684usize],
    #[doc = "0x3fc - SPI_DATE"]
    pub date: DATE,
}
#[doc = "SPI_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "SPI_CMD"]
pub mod cmd;
#[doc = "SPI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SPI_CTRL"]
pub mod ctrl;
#[doc = "SPI_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "SPI_CTRL1"]
pub mod ctrl1;
#[doc = "SPI_RD_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rd_status](rd_status) module"]
pub type RD_STATUS = crate::Reg<u32, _RD_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_STATUS;
#[doc = "`read()` method returns [rd_status::R](rd_status::R) reader structure"]
impl crate::Readable for RD_STATUS {}
#[doc = "`write(|w| ..)` method takes [rd_status::W](rd_status::W) writer structure"]
impl crate::Writable for RD_STATUS {}
#[doc = "SPI_RD_STATUS"]
pub mod rd_status;
#[doc = "SPI_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "SPI_CTRL2"]
pub mod ctrl2;
#[doc = "SPI_CLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "SPI_CLOCK"]
pub mod clock;
#[doc = "SPI_USER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user](user) module"]
pub type USER = crate::Reg<u32, _USER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER;
#[doc = "`read()` method returns [user::R](user::R) reader structure"]
impl crate::Readable for USER {}
#[doc = "`write(|w| ..)` method takes [user::W](user::W) writer structure"]
impl crate::Writable for USER {}
#[doc = "SPI_USER"]
pub mod user;
#[doc = "SPI_USER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user1](user1) module"]
pub type USER1 = crate::Reg<u32, _USER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER1;
#[doc = "`read()` method returns [user1::R](user1::R) reader structure"]
impl crate::Readable for USER1 {}
#[doc = "`write(|w| ..)` method takes [user1::W](user1::W) writer structure"]
impl crate::Writable for USER1 {}
#[doc = "SPI_USER1"]
pub mod user1;
#[doc = "SPI_USER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user2](user2) module"]
pub type USER2 = crate::Reg<u32, _USER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER2;
#[doc = "`read()` method returns [user2::R](user2::R) reader structure"]
impl crate::Readable for USER2 {}
#[doc = "`write(|w| ..)` method takes [user2::W](user2::W) writer structure"]
impl crate::Writable for USER2 {}
#[doc = "SPI_USER2"]
pub mod user2;
#[doc = "SPI_MOSI_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mosi_dlen](mosi_dlen) module"]
pub type MOSI_DLEN = crate::Reg<u32, _MOSI_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOSI_DLEN;
#[doc = "`read()` method returns [mosi_dlen::R](mosi_dlen::R) reader structure"]
impl crate::Readable for MOSI_DLEN {}
#[doc = "`write(|w| ..)` method takes [mosi_dlen::W](mosi_dlen::W) writer structure"]
impl crate::Writable for MOSI_DLEN {}
#[doc = "SPI_MOSI_DLEN"]
pub mod mosi_dlen;
#[doc = "SPI_MISO_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [miso_dlen](miso_dlen) module"]
pub type MISO_DLEN = crate::Reg<u32, _MISO_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISO_DLEN;
#[doc = "`read()` method returns [miso_dlen::R](miso_dlen::R) reader structure"]
impl crate::Readable for MISO_DLEN {}
#[doc = "`write(|w| ..)` method takes [miso_dlen::W](miso_dlen::W) writer structure"]
impl crate::Writable for MISO_DLEN {}
#[doc = "SPI_MISO_DLEN"]
pub mod miso_dlen;
#[doc = "SPI_SLV_WR_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slv_wr_status](slv_wr_status) module"]
pub type SLV_WR_STATUS = crate::Reg<u32, _SLV_WR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLV_WR_STATUS;
#[doc = "`read()` method returns [slv_wr_status::R](slv_wr_status::R) reader structure"]
impl crate::Readable for SLV_WR_STATUS {}
#[doc = "`write(|w| ..)` method takes [slv_wr_status::W](slv_wr_status::W) writer structure"]
impl crate::Writable for SLV_WR_STATUS {}
#[doc = "SPI_SLV_WR_STATUS"]
pub mod slv_wr_status;
#[doc = "SPI_PIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "`write(|w| ..)` method takes [pin::W](pin::W) writer structure"]
impl crate::Writable for PIN {}
#[doc = "SPI_PIN"]
pub mod pin;
#[doc = "SPI_SLAVE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave](slave) module"]
pub type SLAVE = crate::Reg<u32, _SLAVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE;
#[doc = "`read()` method returns [slave::R](slave::R) reader structure"]
impl crate::Readable for SLAVE {}
#[doc = "`write(|w| ..)` method takes [slave::W](slave::W) writer structure"]
impl crate::Writable for SLAVE {}
#[doc = "SPI_SLAVE"]
pub mod slave;
#[doc = "SPI_SLAVE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave1](slave1) module"]
pub type SLAVE1 = crate::Reg<u32, _SLAVE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE1;
#[doc = "`read()` method returns [slave1::R](slave1::R) reader structure"]
impl crate::Readable for SLAVE1 {}
#[doc = "`write(|w| ..)` method takes [slave1::W](slave1::W) writer structure"]
impl crate::Writable for SLAVE1 {}
#[doc = "SPI_SLAVE1"]
pub mod slave1;
#[doc = "SPI_SLAVE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave2](slave2) module"]
pub type SLAVE2 = crate::Reg<u32, _SLAVE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE2;
#[doc = "`read()` method returns [slave2::R](slave2::R) reader structure"]
impl crate::Readable for SLAVE2 {}
#[doc = "`write(|w| ..)` method takes [slave2::W](slave2::W) writer structure"]
impl crate::Writable for SLAVE2 {}
#[doc = "SPI_SLAVE2"]
pub mod slave2;
#[doc = "SPI_SLAVE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave3](slave3) module"]
pub type SLAVE3 = crate::Reg<u32, _SLAVE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE3;
#[doc = "`read()` method returns [slave3::R](slave3::R) reader structure"]
impl crate::Readable for SLAVE3 {}
#[doc = "`write(|w| ..)` method takes [slave3::W](slave3::W) writer structure"]
impl crate::Writable for SLAVE3 {}
#[doc = "SPI_SLAVE3"]
pub mod slave3;
#[doc = "SPI_SLV_WRBUF_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slv_wrbuf_dlen](slv_wrbuf_dlen) module"]
pub type SLV_WRBUF_DLEN = crate::Reg<u32, _SLV_WRBUF_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLV_WRBUF_DLEN;
#[doc = "`read()` method returns [slv_wrbuf_dlen::R](slv_wrbuf_dlen::R) reader structure"]
impl crate::Readable for SLV_WRBUF_DLEN {}
#[doc = "`write(|w| ..)` method takes [slv_wrbuf_dlen::W](slv_wrbuf_dlen::W) writer structure"]
impl crate::Writable for SLV_WRBUF_DLEN {}
#[doc = "SPI_SLV_WRBUF_DLEN"]
pub mod slv_wrbuf_dlen;
#[doc = "SPI_SLV_RDBUF_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slv_rdbuf_dlen](slv_rdbuf_dlen) module"]
pub type SLV_RDBUF_DLEN = crate::Reg<u32, _SLV_RDBUF_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLV_RDBUF_DLEN;
#[doc = "`read()` method returns [slv_rdbuf_dlen::R](slv_rdbuf_dlen::R) reader structure"]
impl crate::Readable for SLV_RDBUF_DLEN {}
#[doc = "`write(|w| ..)` method takes [slv_rdbuf_dlen::W](slv_rdbuf_dlen::W) writer structure"]
impl crate::Writable for SLV_RDBUF_DLEN {}
#[doc = "SPI_SLV_RDBUF_DLEN"]
pub mod slv_rdbuf_dlen;
#[doc = "SPI_CACHE_FCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cache_fctrl](cache_fctrl) module"]
pub type CACHE_FCTRL = crate::Reg<u32, _CACHE_FCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHE_FCTRL;
#[doc = "`read()` method returns [cache_fctrl::R](cache_fctrl::R) reader structure"]
impl crate::Readable for CACHE_FCTRL {}
#[doc = "`write(|w| ..)` method takes [cache_fctrl::W](cache_fctrl::W) writer structure"]
impl crate::Writable for CACHE_FCTRL {}
#[doc = "SPI_CACHE_FCTRL"]
pub mod cache_fctrl;
#[doc = "SPI_CACHE_SCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cache_sctrl](cache_sctrl) module"]
pub type CACHE_SCTRL = crate::Reg<u32, _CACHE_SCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHE_SCTRL;
#[doc = "`read()` method returns [cache_sctrl::R](cache_sctrl::R) reader structure"]
impl crate::Readable for CACHE_SCTRL {}
#[doc = "`write(|w| ..)` method takes [cache_sctrl::W](cache_sctrl::W) writer structure"]
impl crate::Writable for CACHE_SCTRL {}
#[doc = "SPI_CACHE_SCTRL"]
pub mod cache_sctrl;
#[doc = "SPI_SRAM_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_cmd](sram_cmd) module"]
pub type SRAM_CMD = crate::Reg<u32, _SRAM_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_CMD;
#[doc = "`read()` method returns [sram_cmd::R](sram_cmd::R) reader structure"]
impl crate::Readable for SRAM_CMD {}
#[doc = "`write(|w| ..)` method takes [sram_cmd::W](sram_cmd::W) writer structure"]
impl crate::Writable for SRAM_CMD {}
#[doc = "SPI_SRAM_CMD"]
pub mod sram_cmd;
#[doc = "SPI_SRAM_DRD_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_drd_cmd](sram_drd_cmd) module"]
pub type SRAM_DRD_CMD = crate::Reg<u32, _SRAM_DRD_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_DRD_CMD;
#[doc = "`read()` method returns [sram_drd_cmd::R](sram_drd_cmd::R) reader structure"]
impl crate::Readable for SRAM_DRD_CMD {}
#[doc = "`write(|w| ..)` method takes [sram_drd_cmd::W](sram_drd_cmd::W) writer structure"]
impl crate::Writable for SRAM_DRD_CMD {}
#[doc = "SPI_SRAM_DRD_CMD"]
pub mod sram_drd_cmd;
#[doc = "SPI_SRAM_DWR_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_dwr_cmd](sram_dwr_cmd) module"]
pub type SRAM_DWR_CMD = crate::Reg<u32, _SRAM_DWR_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_DWR_CMD;
#[doc = "`read()` method returns [sram_dwr_cmd::R](sram_dwr_cmd::R) reader structure"]
impl crate::Readable for SRAM_DWR_CMD {}
#[doc = "`write(|w| ..)` method takes [sram_dwr_cmd::W](sram_dwr_cmd::W) writer structure"]
impl crate::Writable for SRAM_DWR_CMD {}
#[doc = "SPI_SRAM_DWR_CMD"]
pub mod sram_dwr_cmd;
#[doc = "SPI_SLV_RD_BIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slv_rd_bit](slv_rd_bit) module"]
pub type SLV_RD_BIT = crate::Reg<u32, _SLV_RD_BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLV_RD_BIT;
#[doc = "`read()` method returns [slv_rd_bit::R](slv_rd_bit::R) reader structure"]
impl crate::Readable for SLV_RD_BIT {}
#[doc = "`write(|w| ..)` method takes [slv_rd_bit::W](slv_rd_bit::W) writer structure"]
impl crate::Writable for SLV_RD_BIT {}
#[doc = "SPI_SLV_RD_BIT"]
pub mod slv_rd_bit;
#[doc = "SPI_W0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w0](w0) module"]
pub type W0 = crate::Reg<u32, _W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0;
#[doc = "`read()` method returns [w0::R](w0::R) reader structure"]
impl crate::Readable for W0 {}
#[doc = "`write(|w| ..)` method takes [w0::W](w0::W) writer structure"]
impl crate::Writable for W0 {}
#[doc = "SPI_W0"]
pub mod w0;
#[doc = "SPI_W1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w1](w1) module"]
pub type W1 = crate::Reg<u32, _W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W1;
#[doc = "`read()` method returns [w1::R](w1::R) reader structure"]
impl crate::Readable for W1 {}
#[doc = "`write(|w| ..)` method takes [w1::W](w1::W) writer structure"]
impl crate::Writable for W1 {}
#[doc = "SPI_W1"]
pub mod w1;
#[doc = "SPI_W2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w2](w2) module"]
pub type W2 = crate::Reg<u32, _W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W2;
#[doc = "`read()` method returns [w2::R](w2::R) reader structure"]
impl crate::Readable for W2 {}
#[doc = "`write(|w| ..)` method takes [w2::W](w2::W) writer structure"]
impl crate::Writable for W2 {}
#[doc = "SPI_W2"]
pub mod w2;
#[doc = "SPI_W3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w3](w3) module"]
pub type W3 = crate::Reg<u32, _W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W3;
#[doc = "`read()` method returns [w3::R](w3::R) reader structure"]
impl crate::Readable for W3 {}
#[doc = "`write(|w| ..)` method takes [w3::W](w3::W) writer structure"]
impl crate::Writable for W3 {}
#[doc = "SPI_W3"]
pub mod w3;
#[doc = "SPI_W4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w4](w4) module"]
pub type W4 = crate::Reg<u32, _W4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W4;
#[doc = "`read()` method returns [w4::R](w4::R) reader structure"]
impl crate::Readable for W4 {}
#[doc = "`write(|w| ..)` method takes [w4::W](w4::W) writer structure"]
impl crate::Writable for W4 {}
#[doc = "SPI_W4"]
pub mod w4;
#[doc = "SPI_W5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w5](w5) module"]
pub type W5 = crate::Reg<u32, _W5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W5;
#[doc = "`read()` method returns [w5::R](w5::R) reader structure"]
impl crate::Readable for W5 {}
#[doc = "`write(|w| ..)` method takes [w5::W](w5::W) writer structure"]
impl crate::Writable for W5 {}
#[doc = "SPI_W5"]
pub mod w5;
#[doc = "SPI_W6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w6](w6) module"]
pub type W6 = crate::Reg<u32, _W6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W6;
#[doc = "`read()` method returns [w6::R](w6::R) reader structure"]
impl crate::Readable for W6 {}
#[doc = "`write(|w| ..)` method takes [w6::W](w6::W) writer structure"]
impl crate::Writable for W6 {}
#[doc = "SPI_W6"]
pub mod w6;
#[doc = "SPI_W7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w7](w7) module"]
pub type W7 = crate::Reg<u32, _W7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W7;
#[doc = "`read()` method returns [w7::R](w7::R) reader structure"]
impl crate::Readable for W7 {}
#[doc = "`write(|w| ..)` method takes [w7::W](w7::W) writer structure"]
impl crate::Writable for W7 {}
#[doc = "SPI_W7"]
pub mod w7;
#[doc = "SPI_W8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w8](w8) module"]
pub type W8 = crate::Reg<u32, _W8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W8;
#[doc = "`read()` method returns [w8::R](w8::R) reader structure"]
impl crate::Readable for W8 {}
#[doc = "`write(|w| ..)` method takes [w8::W](w8::W) writer structure"]
impl crate::Writable for W8 {}
#[doc = "SPI_W8"]
pub mod w8;
#[doc = "SPI_W9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w9](w9) module"]
pub type W9 = crate::Reg<u32, _W9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W9;
#[doc = "`read()` method returns [w9::R](w9::R) reader structure"]
impl crate::Readable for W9 {}
#[doc = "`write(|w| ..)` method takes [w9::W](w9::W) writer structure"]
impl crate::Writable for W9 {}
#[doc = "SPI_W9"]
pub mod w9;
#[doc = "SPI_W10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w10](w10) module"]
pub type W10 = crate::Reg<u32, _W10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W10;
#[doc = "`read()` method returns [w10::R](w10::R) reader structure"]
impl crate::Readable for W10 {}
#[doc = "`write(|w| ..)` method takes [w10::W](w10::W) writer structure"]
impl crate::Writable for W10 {}
#[doc = "SPI_W10"]
pub mod w10;
#[doc = "SPI_W11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w11](w11) module"]
pub type W11 = crate::Reg<u32, _W11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W11;
#[doc = "`read()` method returns [w11::R](w11::R) reader structure"]
impl crate::Readable for W11 {}
#[doc = "`write(|w| ..)` method takes [w11::W](w11::W) writer structure"]
impl crate::Writable for W11 {}
#[doc = "SPI_W11"]
pub mod w11;
#[doc = "SPI_W12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w12](w12) module"]
pub type W12 = crate::Reg<u32, _W12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W12;
#[doc = "`read()` method returns [w12::R](w12::R) reader structure"]
impl crate::Readable for W12 {}
#[doc = "`write(|w| ..)` method takes [w12::W](w12::W) writer structure"]
impl crate::Writable for W12 {}
#[doc = "SPI_W12"]
pub mod w12;
#[doc = "SPI_W13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w13](w13) module"]
pub type W13 = crate::Reg<u32, _W13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W13;
#[doc = "`read()` method returns [w13::R](w13::R) reader structure"]
impl crate::Readable for W13 {}
#[doc = "`write(|w| ..)` method takes [w13::W](w13::W) writer structure"]
impl crate::Writable for W13 {}
#[doc = "SPI_W13"]
pub mod w13;
#[doc = "SPI_W14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w14](w14) module"]
pub type W14 = crate::Reg<u32, _W14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W14;
#[doc = "`read()` method returns [w14::R](w14::R) reader structure"]
impl crate::Readable for W14 {}
#[doc = "`write(|w| ..)` method takes [w14::W](w14::W) writer structure"]
impl crate::Writable for W14 {}
#[doc = "SPI_W14"]
pub mod w14;
#[doc = "SPI_W15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [w15](w15) module"]
pub type W15 = crate::Reg<u32, _W15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W15;
#[doc = "`read()` method returns [w15::R](w15::R) reader structure"]
impl crate::Readable for W15 {}
#[doc = "`write(|w| ..)` method takes [w15::W](w15::W) writer structure"]
impl crate::Writable for W15 {}
#[doc = "SPI_W15"]
pub mod w15;
#[doc = "SPI_TX_CRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_crc](tx_crc) module"]
pub type TX_CRC = crate::Reg<u32, _TX_CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CRC;
#[doc = "`read()` method returns [tx_crc::R](tx_crc::R) reader structure"]
impl crate::Readable for TX_CRC {}
#[doc = "`write(|w| ..)` method takes [tx_crc::W](tx_crc::W) writer structure"]
impl crate::Writable for TX_CRC {}
#[doc = "SPI_TX_CRC"]
pub mod tx_crc;
#[doc = "SPI_EXT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext0](ext0) module"]
pub type EXT0 = crate::Reg<u32, _EXT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT0;
#[doc = "`read()` method returns [ext0::R](ext0::R) reader structure"]
impl crate::Readable for EXT0 {}
#[doc = "`write(|w| ..)` method takes [ext0::W](ext0::W) writer structure"]
impl crate::Writable for EXT0 {}
#[doc = "SPI_EXT0"]
pub mod ext0;
#[doc = "SPI_EXT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext1](ext1) module"]
pub type EXT1 = crate::Reg<u32, _EXT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT1;
#[doc = "`read()` method returns [ext1::R](ext1::R) reader structure"]
impl crate::Readable for EXT1 {}
#[doc = "`write(|w| ..)` method takes [ext1::W](ext1::W) writer structure"]
impl crate::Writable for EXT1 {}
#[doc = "SPI_EXT1"]
pub mod ext1;
#[doc = "SPI_EXT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext2](ext2) module"]
pub type EXT2 = crate::Reg<u32, _EXT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT2;
#[doc = "`read()` method returns [ext2::R](ext2::R) reader structure"]
impl crate::Readable for EXT2 {}
#[doc = "`write(|w| ..)` method takes [ext2::W](ext2::W) writer structure"]
impl crate::Writable for EXT2 {}
#[doc = "SPI_EXT2"]
pub mod ext2;
#[doc = "SPI_EXT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext3](ext3) module"]
pub type EXT3 = crate::Reg<u32, _EXT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT3;
#[doc = "`read()` method returns [ext3::R](ext3::R) reader structure"]
impl crate::Readable for EXT3 {}
#[doc = "`write(|w| ..)` method takes [ext3::W](ext3::W) writer structure"]
impl crate::Writable for EXT3 {}
#[doc = "SPI_EXT3"]
pub mod ext3;
#[doc = "SPI_DMA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_conf](dma_conf) module"]
pub type DMA_CONF = crate::Reg<u32, _DMA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CONF;
#[doc = "`read()` method returns [dma_conf::R](dma_conf::R) reader structure"]
impl crate::Readable for DMA_CONF {}
#[doc = "`write(|w| ..)` method takes [dma_conf::W](dma_conf::W) writer structure"]
impl crate::Writable for DMA_CONF {}
#[doc = "SPI_DMA_CONF"]
pub mod dma_conf;
#[doc = "SPI_DMA_OUT_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_link](dma_out_link) module"]
pub type DMA_OUT_LINK = crate::Reg<u32, _DMA_OUT_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_LINK;
#[doc = "`read()` method returns [dma_out_link::R](dma_out_link::R) reader structure"]
impl crate::Readable for DMA_OUT_LINK {}
#[doc = "`write(|w| ..)` method takes [dma_out_link::W](dma_out_link::W) writer structure"]
impl crate::Writable for DMA_OUT_LINK {}
#[doc = "SPI_DMA_OUT_LINK"]
pub mod dma_out_link;
#[doc = "SPI_DMA_IN_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_link](dma_in_link) module"]
pub type DMA_IN_LINK = crate::Reg<u32, _DMA_IN_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_LINK;
#[doc = "`read()` method returns [dma_in_link::R](dma_in_link::R) reader structure"]
impl crate::Readable for DMA_IN_LINK {}
#[doc = "`write(|w| ..)` method takes [dma_in_link::W](dma_in_link::W) writer structure"]
impl crate::Writable for DMA_IN_LINK {}
#[doc = "SPI_DMA_IN_LINK"]
pub mod dma_in_link;
#[doc = "SPI_DMA_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_status](dma_status) module"]
pub type DMA_STATUS = crate::Reg<u32, _DMA_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_STATUS;
#[doc = "`read()` method returns [dma_status::R](dma_status::R) reader structure"]
impl crate::Readable for DMA_STATUS {}
#[doc = "`write(|w| ..)` method takes [dma_status::W](dma_status::W) writer structure"]
impl crate::Writable for DMA_STATUS {}
#[doc = "SPI_DMA_STATUS"]
pub mod dma_status;
#[doc = "SPI_DMA_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_int_ena](dma_int_ena) module"]
pub type DMA_INT_ENA = crate::Reg<u32, _DMA_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ENA;
#[doc = "`read()` method returns [dma_int_ena::R](dma_int_ena::R) reader structure"]
impl crate::Readable for DMA_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [dma_int_ena::W](dma_int_ena::W) writer structure"]
impl crate::Writable for DMA_INT_ENA {}
#[doc = "SPI_DMA_INT_ENA"]
pub mod dma_int_ena;
#[doc = "SPI_DMA_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_int_raw](dma_int_raw) module"]
pub type DMA_INT_RAW = crate::Reg<u32, _DMA_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_RAW;
#[doc = "`read()` method returns [dma_int_raw::R](dma_int_raw::R) reader structure"]
impl crate::Readable for DMA_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [dma_int_raw::W](dma_int_raw::W) writer structure"]
impl crate::Writable for DMA_INT_RAW {}
#[doc = "SPI_DMA_INT_RAW"]
pub mod dma_int_raw;
#[doc = "SPI_DMA_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_int_st](dma_int_st) module"]
pub type DMA_INT_ST = crate::Reg<u32, _DMA_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ST;
#[doc = "`read()` method returns [dma_int_st::R](dma_int_st::R) reader structure"]
impl crate::Readable for DMA_INT_ST {}
#[doc = "`write(|w| ..)` method takes [dma_int_st::W](dma_int_st::W) writer structure"]
impl crate::Writable for DMA_INT_ST {}
#[doc = "SPI_DMA_INT_ST"]
pub mod dma_int_st;
#[doc = "SPI_DMA_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_int_clr](dma_int_clr) module"]
pub type DMA_INT_CLR = crate::Reg<u32, _DMA_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_CLR;
#[doc = "`read()` method returns [dma_int_clr::R](dma_int_clr::R) reader structure"]
impl crate::Readable for DMA_INT_CLR {}
#[doc = "`write(|w| ..)` method takes [dma_int_clr::W](dma_int_clr::W) writer structure"]
impl crate::Writable for DMA_INT_CLR {}
#[doc = "SPI_DMA_INT_CLR"]
pub mod dma_int_clr;
#[doc = "SPI_IN_ERR_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_err_eof_des_addr](in_err_eof_des_addr) module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<u32, _IN_ERR_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_ERR_EOF_DES_ADDR;
#[doc = "`read()` method returns [in_err_eof_des_addr::R](in_err_eof_des_addr::R) reader structure"]
impl crate::Readable for IN_ERR_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [in_err_eof_des_addr::W](in_err_eof_des_addr::W) writer structure"]
impl crate::Writable for IN_ERR_EOF_DES_ADDR {}
#[doc = "SPI_IN_ERR_EOF_DES_ADDR"]
pub mod in_err_eof_des_addr;
#[doc = "SPI_IN_SUC_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_suc_eof_des_addr](in_suc_eof_des_addr) module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<u32, _IN_SUC_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_SUC_EOF_DES_ADDR;
#[doc = "`read()` method returns [in_suc_eof_des_addr::R](in_suc_eof_des_addr::R) reader structure"]
impl crate::Readable for IN_SUC_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [in_suc_eof_des_addr::W](in_suc_eof_des_addr::W) writer structure"]
impl crate::Writable for IN_SUC_EOF_DES_ADDR {}
#[doc = "SPI_IN_SUC_EOF_DES_ADDR"]
pub mod in_suc_eof_des_addr;
#[doc = "SPI_INLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inlink_dscr](inlink_dscr) module"]
pub type INLINK_DSCR = crate::Reg<u32, _INLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INLINK_DSCR;
#[doc = "`read()` method returns [inlink_dscr::R](inlink_dscr::R) reader structure"]
impl crate::Readable for INLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [inlink_dscr::W](inlink_dscr::W) writer structure"]
impl crate::Writable for INLINK_DSCR {}
#[doc = "SPI_INLINK_DSCR"]
pub mod inlink_dscr;
#[doc = "SPI_INLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inlink_dscr_bf0](inlink_dscr_bf0) module"]
pub type INLINK_DSCR_BF0 = crate::Reg<u32, _INLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INLINK_DSCR_BF0;
#[doc = "`read()` method returns [inlink_dscr_bf0::R](inlink_dscr_bf0::R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [inlink_dscr_bf0::W](inlink_dscr_bf0::W) writer structure"]
impl crate::Writable for INLINK_DSCR_BF0 {}
#[doc = "SPI_INLINK_DSCR_BF0"]
pub mod inlink_dscr_bf0;
#[doc = "SPI_INLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inlink_dscr_bf1](inlink_dscr_bf1) module"]
pub type INLINK_DSCR_BF1 = crate::Reg<u32, _INLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INLINK_DSCR_BF1;
#[doc = "`read()` method returns [inlink_dscr_bf1::R](inlink_dscr_bf1::R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [inlink_dscr_bf1::W](inlink_dscr_bf1::W) writer structure"]
impl crate::Writable for INLINK_DSCR_BF1 {}
#[doc = "SPI_INLINK_DSCR_BF1"]
pub mod inlink_dscr_bf1;
#[doc = "SPI_OUT_EOF_BFR_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_eof_bfr_des_addr](out_eof_bfr_des_addr) module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<u32, _OUT_EOF_BFR_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EOF_BFR_DES_ADDR;
#[doc = "`read()` method returns [out_eof_bfr_des_addr::R](out_eof_bfr_des_addr::R) reader structure"]
impl crate::Readable for OUT_EOF_BFR_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [out_eof_bfr_des_addr::W](out_eof_bfr_des_addr::W) writer structure"]
impl crate::Writable for OUT_EOF_BFR_DES_ADDR {}
#[doc = "SPI_OUT_EOF_BFR_DES_ADDR"]
pub mod out_eof_bfr_des_addr;
#[doc = "SPI_OUT_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_eof_des_addr](out_eof_des_addr) module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<u32, _OUT_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EOF_DES_ADDR;
#[doc = "`read()` method returns [out_eof_des_addr::R](out_eof_des_addr::R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [out_eof_des_addr::W](out_eof_des_addr::W) writer structure"]
impl crate::Writable for OUT_EOF_DES_ADDR {}
#[doc = "SPI_OUT_EOF_DES_ADDR"]
pub mod out_eof_des_addr;
#[doc = "SPI_OUTLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outlink_dscr](outlink_dscr) module"]
pub type OUTLINK_DSCR = crate::Reg<u32, _OUTLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTLINK_DSCR;
#[doc = "`read()` method returns [outlink_dscr::R](outlink_dscr::R) reader structure"]
impl crate::Readable for OUTLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [outlink_dscr::W](outlink_dscr::W) writer structure"]
impl crate::Writable for OUTLINK_DSCR {}
#[doc = "SPI_OUTLINK_DSCR"]
pub mod outlink_dscr;
#[doc = "SPI_OUTLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outlink_dscr_bf0](outlink_dscr_bf0) module"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<u32, _OUTLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTLINK_DSCR_BF0;
#[doc = "`read()` method returns [outlink_dscr_bf0::R](outlink_dscr_bf0::R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [outlink_dscr_bf0::W](outlink_dscr_bf0::W) writer structure"]
impl crate::Writable for OUTLINK_DSCR_BF0 {}
#[doc = "SPI_OUTLINK_DSCR_BF0"]
pub mod outlink_dscr_bf0;
#[doc = "SPI_OUTLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outlink_dscr_bf1](outlink_dscr_bf1) module"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<u32, _OUTLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTLINK_DSCR_BF1;
#[doc = "`read()` method returns [outlink_dscr_bf1::R](outlink_dscr_bf1::R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [outlink_dscr_bf1::W](outlink_dscr_bf1::W) writer structure"]
impl crate::Writable for OUTLINK_DSCR_BF1 {}
#[doc = "SPI_OUTLINK_DSCR_BF1"]
pub mod outlink_dscr_bf1;
#[doc = "SPI_DMA_RSTATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_rstatus](dma_rstatus) module"]
pub type DMA_RSTATUS = crate::Reg<u32, _DMA_RSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_RSTATUS;
#[doc = "`read()` method returns [dma_rstatus::R](dma_rstatus::R) reader structure"]
impl crate::Readable for DMA_RSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_rstatus::W](dma_rstatus::W) writer structure"]
impl crate::Writable for DMA_RSTATUS {}
#[doc = "SPI_DMA_RSTATUS"]
pub mod dma_rstatus;
#[doc = "SPI_DMA_TSTATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_tstatus](dma_tstatus) module"]
pub type DMA_TSTATUS = crate::Reg<u32, _DMA_TSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_TSTATUS;
#[doc = "`read()` method returns [dma_tstatus::R](dma_tstatus::R) reader structure"]
impl crate::Readable for DMA_TSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_tstatus::W](dma_tstatus::W) writer structure"]
impl crate::Writable for DMA_TSTATUS {}
#[doc = "SPI_DMA_TSTATUS"]
pub mod dma_tstatus;
#[doc = "SPI_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "SPI_DATE"]
pub mod date;
