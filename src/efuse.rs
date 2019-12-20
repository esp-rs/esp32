#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EFUSE_BLK0_RDATA0_REG"]
    pub efuse_blk0_rdata0_reg: EFUSE_BLK0_RDATA0_REG,
    #[doc = "0x04 - EFUSE_BLK0_RDATA1_REG"]
    pub efuse_blk0_rdata1_reg: EFUSE_BLK0_RDATA1_REG,
    #[doc = "0x08 - EFUSE_BLK0_RDATA2_REG"]
    pub efuse_blk0_rdata2_reg: EFUSE_BLK0_RDATA2_REG,
    #[doc = "0x0c - EFUSE_BLK0_RDATA3_REG"]
    pub efuse_blk0_rdata3_reg: EFUSE_BLK0_RDATA3_REG,
    #[doc = "0x10 - EFUSE_BLK0_RDATA4_REG"]
    pub efuse_blk0_rdata4_reg: EFUSE_BLK0_RDATA4_REG,
    #[doc = "0x14 - EFUSE_BLK0_RDATA5_REG"]
    pub efuse_blk0_rdata5_reg: EFUSE_BLK0_RDATA5_REG,
    #[doc = "0x18 - EFUSE_BLK0_RDATA6_REG"]
    pub efuse_blk0_rdata6_reg: EFUSE_BLK0_RDATA6_REG,
    #[doc = "0x1c - EFUSE_BLK0_WDATA0_REG"]
    pub efuse_blk0_wdata0_reg: EFUSE_BLK0_WDATA0_REG,
    #[doc = "0x20 - EFUSE_BLK0_WDATA1_REG"]
    pub efuse_blk0_wdata1_reg: EFUSE_BLK0_WDATA1_REG,
    #[doc = "0x24 - EFUSE_BLK0_WDATA2_REG"]
    pub efuse_blk0_wdata2_reg: EFUSE_BLK0_WDATA2_REG,
    #[doc = "0x28 - EFUSE_BLK0_WDATA3_REG"]
    pub efuse_blk0_wdata3_reg: EFUSE_BLK0_WDATA3_REG,
    #[doc = "0x2c - EFUSE_BLK0_WDATA4_REG"]
    pub efuse_blk0_wdata4_reg: EFUSE_BLK0_WDATA4_REG,
    #[doc = "0x30 - EFUSE_BLK0_WDATA5_REG"]
    pub efuse_blk0_wdata5_reg: EFUSE_BLK0_WDATA5_REG,
    #[doc = "0x34 - EFUSE_BLK0_WDATA6_REG"]
    pub efuse_blk0_wdata6_reg: EFUSE_BLK0_WDATA6_REG,
    #[doc = "0x38 - EFUSE_BLK1_RDATA0_REG"]
    pub efuse_blk1_rdata0_reg: EFUSE_BLK1_RDATA0_REG,
    #[doc = "0x3c - EFUSE_BLK1_RDATA1_REG"]
    pub efuse_blk1_rdata1_reg: EFUSE_BLK1_RDATA1_REG,
    #[doc = "0x40 - EFUSE_BLK1_RDATA2_REG"]
    pub efuse_blk1_rdata2_reg: EFUSE_BLK1_RDATA2_REG,
    #[doc = "0x44 - EFUSE_BLK1_RDATA3_REG"]
    pub efuse_blk1_rdata3_reg: EFUSE_BLK1_RDATA3_REG,
    #[doc = "0x48 - EFUSE_BLK1_RDATA4_REG"]
    pub efuse_blk1_rdata4_reg: EFUSE_BLK1_RDATA4_REG,
    #[doc = "0x4c - EFUSE_BLK1_RDATA5_REG"]
    pub efuse_blk1_rdata5_reg: EFUSE_BLK1_RDATA5_REG,
    #[doc = "0x50 - EFUSE_BLK1_RDATA6_REG"]
    pub efuse_blk1_rdata6_reg: EFUSE_BLK1_RDATA6_REG,
    #[doc = "0x54 - EFUSE_BLK1_RDATA7_REG"]
    pub efuse_blk1_rdata7_reg: EFUSE_BLK1_RDATA7_REG,
    #[doc = "0x58 - EFUSE_BLK2_RDATA0_REG"]
    pub efuse_blk2_rdata0_reg: EFUSE_BLK2_RDATA0_REG,
    #[doc = "0x5c - EFUSE_BLK2_RDATA1_REG"]
    pub efuse_blk2_rdata1_reg: EFUSE_BLK2_RDATA1_REG,
    #[doc = "0x60 - EFUSE_BLK2_RDATA2_REG"]
    pub efuse_blk2_rdata2_reg: EFUSE_BLK2_RDATA2_REG,
    #[doc = "0x64 - EFUSE_BLK2_RDATA3_REG"]
    pub efuse_blk2_rdata3_reg: EFUSE_BLK2_RDATA3_REG,
    #[doc = "0x68 - EFUSE_BLK2_RDATA4_REG"]
    pub efuse_blk2_rdata4_reg: EFUSE_BLK2_RDATA4_REG,
    #[doc = "0x6c - EFUSE_BLK2_RDATA5_REG"]
    pub efuse_blk2_rdata5_reg: EFUSE_BLK2_RDATA5_REG,
    #[doc = "0x70 - EFUSE_BLK2_RDATA6_REG"]
    pub efuse_blk2_rdata6_reg: EFUSE_BLK2_RDATA6_REG,
    #[doc = "0x74 - EFUSE_BLK2_RDATA7_REG"]
    pub efuse_blk2_rdata7_reg: EFUSE_BLK2_RDATA7_REG,
    #[doc = "0x78 - EFUSE_BLK3_RDATA0_REG"]
    pub efuse_blk3_rdata0_reg: EFUSE_BLK3_RDATA0_REG,
    #[doc = "0x7c - EFUSE_BLK3_RDATA1_REG"]
    pub efuse_blk3_rdata1_reg: EFUSE_BLK3_RDATA1_REG,
    #[doc = "0x80 - EFUSE_BLK3_RDATA2_REG"]
    pub efuse_blk3_rdata2_reg: EFUSE_BLK3_RDATA2_REG,
    #[doc = "0x84 - EFUSE_BLK3_RDATA3_REG"]
    pub efuse_blk3_rdata3_reg: EFUSE_BLK3_RDATA3_REG,
    #[doc = "0x88 - EFUSE_BLK3_RDATA4_REG"]
    pub efuse_blk3_rdata4_reg: EFUSE_BLK3_RDATA4_REG,
    #[doc = "0x8c - EFUSE_BLK3_RDATA5_REG"]
    pub efuse_blk3_rdata5_reg: EFUSE_BLK3_RDATA5_REG,
    #[doc = "0x90 - EFUSE_BLK3_RDATA6_REG"]
    pub efuse_blk3_rdata6_reg: EFUSE_BLK3_RDATA6_REG,
    #[doc = "0x94 - EFUSE_BLK3_RDATA7_REG"]
    pub efuse_blk3_rdata7_reg: EFUSE_BLK3_RDATA7_REG,
    #[doc = "0x98 - EFUSE_BLK1_WDATA0_REG"]
    pub efuse_blk1_wdata0_reg: EFUSE_BLK1_WDATA0_REG,
    #[doc = "0x9c - EFUSE_BLK1_WDATA1_REG"]
    pub efuse_blk1_wdata1_reg: EFUSE_BLK1_WDATA1_REG,
    #[doc = "0xa0 - EFUSE_BLK1_WDATA2_REG"]
    pub efuse_blk1_wdata2_reg: EFUSE_BLK1_WDATA2_REG,
    #[doc = "0xa4 - EFUSE_BLK1_WDATA3_REG"]
    pub efuse_blk1_wdata3_reg: EFUSE_BLK1_WDATA3_REG,
    #[doc = "0xa8 - EFUSE_BLK1_WDATA4_REG"]
    pub efuse_blk1_wdata4_reg: EFUSE_BLK1_WDATA4_REG,
    #[doc = "0xac - EFUSE_BLK1_WDATA5_REG"]
    pub efuse_blk1_wdata5_reg: EFUSE_BLK1_WDATA5_REG,
    #[doc = "0xb0 - EFUSE_BLK1_WDATA6_REG"]
    pub efuse_blk1_wdata6_reg: EFUSE_BLK1_WDATA6_REG,
    #[doc = "0xb4 - EFUSE_BLK1_WDATA7_REG"]
    pub efuse_blk1_wdata7_reg: EFUSE_BLK1_WDATA7_REG,
    #[doc = "0xb8 - EFUSE_BLK2_WDATA0_REG"]
    pub efuse_blk2_wdata0_reg: EFUSE_BLK2_WDATA0_REG,
    #[doc = "0xbc - EFUSE_BLK2_WDATA1_REG"]
    pub efuse_blk2_wdata1_reg: EFUSE_BLK2_WDATA1_REG,
    #[doc = "0xc0 - EFUSE_BLK2_WDATA2_REG"]
    pub efuse_blk2_wdata2_reg: EFUSE_BLK2_WDATA2_REG,
    #[doc = "0xc4 - EFUSE_BLK2_WDATA3_REG"]
    pub efuse_blk2_wdata3_reg: EFUSE_BLK2_WDATA3_REG,
    #[doc = "0xc8 - EFUSE_BLK2_WDATA4_REG"]
    pub efuse_blk2_wdata4_reg: EFUSE_BLK2_WDATA4_REG,
    #[doc = "0xcc - EFUSE_BLK2_WDATA5_REG"]
    pub efuse_blk2_wdata5_reg: EFUSE_BLK2_WDATA5_REG,
    #[doc = "0xd0 - EFUSE_BLK2_WDATA6_REG"]
    pub efuse_blk2_wdata6_reg: EFUSE_BLK2_WDATA6_REG,
    #[doc = "0xd4 - EFUSE_BLK2_WDATA7_REG"]
    pub efuse_blk2_wdata7_reg: EFUSE_BLK2_WDATA7_REG,
    #[doc = "0xd8 - EFUSE_BLK3_WDATA0_REG"]
    pub efuse_blk3_wdata0_reg: EFUSE_BLK3_WDATA0_REG,
    #[doc = "0xdc - EFUSE_BLK3_WDATA1_REG"]
    pub efuse_blk3_wdata1_reg: EFUSE_BLK3_WDATA1_REG,
    #[doc = "0xe0 - EFUSE_BLK3_WDATA2_REG"]
    pub efuse_blk3_wdata2_reg: EFUSE_BLK3_WDATA2_REG,
    #[doc = "0xe4 - EFUSE_BLK3_WDATA3_REG"]
    pub efuse_blk3_wdata3_reg: EFUSE_BLK3_WDATA3_REG,
    #[doc = "0xe8 - EFUSE_BLK3_WDATA4_REG"]
    pub efuse_blk3_wdata4_reg: EFUSE_BLK3_WDATA4_REG,
    #[doc = "0xec - EFUSE_BLK3_WDATA5_REG"]
    pub efuse_blk3_wdata5_reg: EFUSE_BLK3_WDATA5_REG,
    #[doc = "0xf0 - EFUSE_BLK3_WDATA6_REG"]
    pub efuse_blk3_wdata6_reg: EFUSE_BLK3_WDATA6_REG,
    #[doc = "0xf4 - EFUSE_BLK3_WDATA7_REG"]
    pub efuse_blk3_wdata7_reg: EFUSE_BLK3_WDATA7_REG,
    #[doc = "0xf8 - EFUSE_CLK_REG"]
    pub efuse_clk_reg: EFUSE_CLK_REG,
    #[doc = "0xfc - EFUSE_CONF_REG"]
    pub efuse_conf_reg: EFUSE_CONF_REG,
    #[doc = "0x100 - EFUSE_STATUS_REG"]
    pub efuse_status_reg: EFUSE_STATUS_REG,
    #[doc = "0x104 - EFUSE_CMD_REG"]
    pub efuse_cmd_reg: EFUSE_CMD_REG,
    #[doc = "0x108 - EFUSE_INT_RAW_REG"]
    pub efuse_int_raw_reg: EFUSE_INT_RAW_REG,
    #[doc = "0x10c - EFUSE_INT_ST_REG"]
    pub efuse_int_st_reg: EFUSE_INT_ST_REG,
    #[doc = "0x110 - EFUSE_INT_ENA_REG"]
    pub efuse_int_ena_reg: EFUSE_INT_ENA_REG,
    #[doc = "0x114 - EFUSE_INT_CLR_REG"]
    pub efuse_int_clr_reg: EFUSE_INT_CLR_REG,
    #[doc = "0x118 - EFUSE_DAC_CONF_REG"]
    pub efuse_dac_conf_reg: EFUSE_DAC_CONF_REG,
    #[doc = "0x11c - EFUSE_DEC_STATUS_REG"]
    pub efuse_dec_status_reg: EFUSE_DEC_STATUS_REG,
    _reserved72: [u8; 220usize],
    #[doc = "0x1fc - EFUSE_DATE_REG"]
    pub efuse_date_reg: EFUSE_DATE_REG,
}
#[doc = "EFUSE_BLK0_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata0_reg](efuse_blk0_rdata0_reg) module"]
pub type EFUSE_BLK0_RDATA0_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA0_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata0_reg::R](efuse_blk0_rdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata0_reg::W](efuse_blk0_rdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA0_REG {}
#[doc = "EFUSE_BLK0_RDATA0_REG"]
pub mod efuse_blk0_rdata0_reg;
#[doc = "EFUSE_BLK0_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata1_reg](efuse_blk0_rdata1_reg) module"]
pub type EFUSE_BLK0_RDATA1_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA1_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata1_reg::R](efuse_blk0_rdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata1_reg::W](efuse_blk0_rdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA1_REG {}
#[doc = "EFUSE_BLK0_RDATA1_REG"]
pub mod efuse_blk0_rdata1_reg;
#[doc = "EFUSE_BLK0_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata2_reg](efuse_blk0_rdata2_reg) module"]
pub type EFUSE_BLK0_RDATA2_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA2_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata2_reg::R](efuse_blk0_rdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata2_reg::W](efuse_blk0_rdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA2_REG {}
#[doc = "EFUSE_BLK0_RDATA2_REG"]
pub mod efuse_blk0_rdata2_reg;
#[doc = "EFUSE_BLK0_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata3_reg](efuse_blk0_rdata3_reg) module"]
pub type EFUSE_BLK0_RDATA3_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA3_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata3_reg::R](efuse_blk0_rdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata3_reg::W](efuse_blk0_rdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA3_REG {}
#[doc = "EFUSE_BLK0_RDATA3_REG"]
pub mod efuse_blk0_rdata3_reg;
#[doc = "EFUSE_BLK0_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata4_reg](efuse_blk0_rdata4_reg) module"]
pub type EFUSE_BLK0_RDATA4_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA4_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata4_reg::R](efuse_blk0_rdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata4_reg::W](efuse_blk0_rdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA4_REG {}
#[doc = "EFUSE_BLK0_RDATA4_REG"]
pub mod efuse_blk0_rdata4_reg;
#[doc = "EFUSE_BLK0_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata5_reg](efuse_blk0_rdata5_reg) module"]
pub type EFUSE_BLK0_RDATA5_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA5_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata5_reg::R](efuse_blk0_rdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata5_reg::W](efuse_blk0_rdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA5_REG {}
#[doc = "EFUSE_BLK0_RDATA5_REG"]
pub mod efuse_blk0_rdata5_reg;
#[doc = "EFUSE_BLK0_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_rdata6_reg](efuse_blk0_rdata6_reg) module"]
pub type EFUSE_BLK0_RDATA6_REG = crate::Reg<u32, _EFUSE_BLK0_RDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_RDATA6_REG;
#[doc = "`read()` method returns [efuse_blk0_rdata6_reg::R](efuse_blk0_rdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_RDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_rdata6_reg::W](efuse_blk0_rdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_RDATA6_REG {}
#[doc = "EFUSE_BLK0_RDATA6_REG"]
pub mod efuse_blk0_rdata6_reg;
#[doc = "EFUSE_BLK0_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata0_reg](efuse_blk0_wdata0_reg) module"]
pub type EFUSE_BLK0_WDATA0_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA0_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata0_reg::R](efuse_blk0_wdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata0_reg::W](efuse_blk0_wdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA0_REG {}
#[doc = "EFUSE_BLK0_WDATA0_REG"]
pub mod efuse_blk0_wdata0_reg;
#[doc = "EFUSE_BLK0_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata1_reg](efuse_blk0_wdata1_reg) module"]
pub type EFUSE_BLK0_WDATA1_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA1_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata1_reg::R](efuse_blk0_wdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata1_reg::W](efuse_blk0_wdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA1_REG {}
#[doc = "EFUSE_BLK0_WDATA1_REG"]
pub mod efuse_blk0_wdata1_reg;
#[doc = "EFUSE_BLK0_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata2_reg](efuse_blk0_wdata2_reg) module"]
pub type EFUSE_BLK0_WDATA2_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA2_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata2_reg::R](efuse_blk0_wdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata2_reg::W](efuse_blk0_wdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA2_REG {}
#[doc = "EFUSE_BLK0_WDATA2_REG"]
pub mod efuse_blk0_wdata2_reg;
#[doc = "EFUSE_BLK0_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata3_reg](efuse_blk0_wdata3_reg) module"]
pub type EFUSE_BLK0_WDATA3_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA3_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata3_reg::R](efuse_blk0_wdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata3_reg::W](efuse_blk0_wdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA3_REG {}
#[doc = "EFUSE_BLK0_WDATA3_REG"]
pub mod efuse_blk0_wdata3_reg;
#[doc = "EFUSE_BLK0_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata4_reg](efuse_blk0_wdata4_reg) module"]
pub type EFUSE_BLK0_WDATA4_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA4_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata4_reg::R](efuse_blk0_wdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata4_reg::W](efuse_blk0_wdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA4_REG {}
#[doc = "EFUSE_BLK0_WDATA4_REG"]
pub mod efuse_blk0_wdata4_reg;
#[doc = "EFUSE_BLK0_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata5_reg](efuse_blk0_wdata5_reg) module"]
pub type EFUSE_BLK0_WDATA5_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA5_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata5_reg::R](efuse_blk0_wdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata5_reg::W](efuse_blk0_wdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA5_REG {}
#[doc = "EFUSE_BLK0_WDATA5_REG"]
pub mod efuse_blk0_wdata5_reg;
#[doc = "EFUSE_BLK0_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk0_wdata6_reg](efuse_blk0_wdata6_reg) module"]
pub type EFUSE_BLK0_WDATA6_REG = crate::Reg<u32, _EFUSE_BLK0_WDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK0_WDATA6_REG;
#[doc = "`read()` method returns [efuse_blk0_wdata6_reg::R](efuse_blk0_wdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK0_WDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk0_wdata6_reg::W](efuse_blk0_wdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK0_WDATA6_REG {}
#[doc = "EFUSE_BLK0_WDATA6_REG"]
pub mod efuse_blk0_wdata6_reg;
#[doc = "EFUSE_BLK1_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata0_reg](efuse_blk1_rdata0_reg) module"]
pub type EFUSE_BLK1_RDATA0_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA0_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata0_reg::R](efuse_blk1_rdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata0_reg::W](efuse_blk1_rdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA0_REG {}
#[doc = "EFUSE_BLK1_RDATA0_REG"]
pub mod efuse_blk1_rdata0_reg;
#[doc = "EFUSE_BLK1_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata1_reg](efuse_blk1_rdata1_reg) module"]
pub type EFUSE_BLK1_RDATA1_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA1_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata1_reg::R](efuse_blk1_rdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata1_reg::W](efuse_blk1_rdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA1_REG {}
#[doc = "EFUSE_BLK1_RDATA1_REG"]
pub mod efuse_blk1_rdata1_reg;
#[doc = "EFUSE_BLK1_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata2_reg](efuse_blk1_rdata2_reg) module"]
pub type EFUSE_BLK1_RDATA2_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA2_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata2_reg::R](efuse_blk1_rdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata2_reg::W](efuse_blk1_rdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA2_REG {}
#[doc = "EFUSE_BLK1_RDATA2_REG"]
pub mod efuse_blk1_rdata2_reg;
#[doc = "EFUSE_BLK1_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata3_reg](efuse_blk1_rdata3_reg) module"]
pub type EFUSE_BLK1_RDATA3_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA3_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata3_reg::R](efuse_blk1_rdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata3_reg::W](efuse_blk1_rdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA3_REG {}
#[doc = "EFUSE_BLK1_RDATA3_REG"]
pub mod efuse_blk1_rdata3_reg;
#[doc = "EFUSE_BLK1_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata4_reg](efuse_blk1_rdata4_reg) module"]
pub type EFUSE_BLK1_RDATA4_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA4_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata4_reg::R](efuse_blk1_rdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata4_reg::W](efuse_blk1_rdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA4_REG {}
#[doc = "EFUSE_BLK1_RDATA4_REG"]
pub mod efuse_blk1_rdata4_reg;
#[doc = "EFUSE_BLK1_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata5_reg](efuse_blk1_rdata5_reg) module"]
pub type EFUSE_BLK1_RDATA5_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA5_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata5_reg::R](efuse_blk1_rdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata5_reg::W](efuse_blk1_rdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA5_REG {}
#[doc = "EFUSE_BLK1_RDATA5_REG"]
pub mod efuse_blk1_rdata5_reg;
#[doc = "EFUSE_BLK1_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata6_reg](efuse_blk1_rdata6_reg) module"]
pub type EFUSE_BLK1_RDATA6_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA6_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata6_reg::R](efuse_blk1_rdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata6_reg::W](efuse_blk1_rdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA6_REG {}
#[doc = "EFUSE_BLK1_RDATA6_REG"]
pub mod efuse_blk1_rdata6_reg;
#[doc = "EFUSE_BLK1_RDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_rdata7_reg](efuse_blk1_rdata7_reg) module"]
pub type EFUSE_BLK1_RDATA7_REG = crate::Reg<u32, _EFUSE_BLK1_RDATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_RDATA7_REG;
#[doc = "`read()` method returns [efuse_blk1_rdata7_reg::R](efuse_blk1_rdata7_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_RDATA7_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_rdata7_reg::W](efuse_blk1_rdata7_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_RDATA7_REG {}
#[doc = "EFUSE_BLK1_RDATA7_REG"]
pub mod efuse_blk1_rdata7_reg;
#[doc = "EFUSE_BLK2_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata0_reg](efuse_blk2_rdata0_reg) module"]
pub type EFUSE_BLK2_RDATA0_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA0_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata0_reg::R](efuse_blk2_rdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata0_reg::W](efuse_blk2_rdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA0_REG {}
#[doc = "EFUSE_BLK2_RDATA0_REG"]
pub mod efuse_blk2_rdata0_reg;
#[doc = "EFUSE_BLK2_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata1_reg](efuse_blk2_rdata1_reg) module"]
pub type EFUSE_BLK2_RDATA1_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA1_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata1_reg::R](efuse_blk2_rdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata1_reg::W](efuse_blk2_rdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA1_REG {}
#[doc = "EFUSE_BLK2_RDATA1_REG"]
pub mod efuse_blk2_rdata1_reg;
#[doc = "EFUSE_BLK2_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata2_reg](efuse_blk2_rdata2_reg) module"]
pub type EFUSE_BLK2_RDATA2_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA2_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata2_reg::R](efuse_blk2_rdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata2_reg::W](efuse_blk2_rdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA2_REG {}
#[doc = "EFUSE_BLK2_RDATA2_REG"]
pub mod efuse_blk2_rdata2_reg;
#[doc = "EFUSE_BLK2_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata3_reg](efuse_blk2_rdata3_reg) module"]
pub type EFUSE_BLK2_RDATA3_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA3_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata3_reg::R](efuse_blk2_rdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata3_reg::W](efuse_blk2_rdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA3_REG {}
#[doc = "EFUSE_BLK2_RDATA3_REG"]
pub mod efuse_blk2_rdata3_reg;
#[doc = "EFUSE_BLK2_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata4_reg](efuse_blk2_rdata4_reg) module"]
pub type EFUSE_BLK2_RDATA4_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA4_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata4_reg::R](efuse_blk2_rdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata4_reg::W](efuse_blk2_rdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA4_REG {}
#[doc = "EFUSE_BLK2_RDATA4_REG"]
pub mod efuse_blk2_rdata4_reg;
#[doc = "EFUSE_BLK2_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata5_reg](efuse_blk2_rdata5_reg) module"]
pub type EFUSE_BLK2_RDATA5_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA5_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata5_reg::R](efuse_blk2_rdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata5_reg::W](efuse_blk2_rdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA5_REG {}
#[doc = "EFUSE_BLK2_RDATA5_REG"]
pub mod efuse_blk2_rdata5_reg;
#[doc = "EFUSE_BLK2_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata6_reg](efuse_blk2_rdata6_reg) module"]
pub type EFUSE_BLK2_RDATA6_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA6_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata6_reg::R](efuse_blk2_rdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata6_reg::W](efuse_blk2_rdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA6_REG {}
#[doc = "EFUSE_BLK2_RDATA6_REG"]
pub mod efuse_blk2_rdata6_reg;
#[doc = "EFUSE_BLK2_RDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_rdata7_reg](efuse_blk2_rdata7_reg) module"]
pub type EFUSE_BLK2_RDATA7_REG = crate::Reg<u32, _EFUSE_BLK2_RDATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_RDATA7_REG;
#[doc = "`read()` method returns [efuse_blk2_rdata7_reg::R](efuse_blk2_rdata7_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_RDATA7_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_rdata7_reg::W](efuse_blk2_rdata7_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_RDATA7_REG {}
#[doc = "EFUSE_BLK2_RDATA7_REG"]
pub mod efuse_blk2_rdata7_reg;
#[doc = "EFUSE_BLK3_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata0_reg](efuse_blk3_rdata0_reg) module"]
pub type EFUSE_BLK3_RDATA0_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA0_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata0_reg::R](efuse_blk3_rdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata0_reg::W](efuse_blk3_rdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA0_REG {}
#[doc = "EFUSE_BLK3_RDATA0_REG"]
pub mod efuse_blk3_rdata0_reg;
#[doc = "EFUSE_BLK3_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata1_reg](efuse_blk3_rdata1_reg) module"]
pub type EFUSE_BLK3_RDATA1_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA1_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata1_reg::R](efuse_blk3_rdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata1_reg::W](efuse_blk3_rdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA1_REG {}
#[doc = "EFUSE_BLK3_RDATA1_REG"]
pub mod efuse_blk3_rdata1_reg;
#[doc = "EFUSE_BLK3_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata2_reg](efuse_blk3_rdata2_reg) module"]
pub type EFUSE_BLK3_RDATA2_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA2_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata2_reg::R](efuse_blk3_rdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata2_reg::W](efuse_blk3_rdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA2_REG {}
#[doc = "EFUSE_BLK3_RDATA2_REG"]
pub mod efuse_blk3_rdata2_reg;
#[doc = "EFUSE_BLK3_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata3_reg](efuse_blk3_rdata3_reg) module"]
pub type EFUSE_BLK3_RDATA3_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA3_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata3_reg::R](efuse_blk3_rdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata3_reg::W](efuse_blk3_rdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA3_REG {}
#[doc = "EFUSE_BLK3_RDATA3_REG"]
pub mod efuse_blk3_rdata3_reg;
#[doc = "EFUSE_BLK3_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata4_reg](efuse_blk3_rdata4_reg) module"]
pub type EFUSE_BLK3_RDATA4_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA4_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata4_reg::R](efuse_blk3_rdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata4_reg::W](efuse_blk3_rdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA4_REG {}
#[doc = "EFUSE_BLK3_RDATA4_REG"]
pub mod efuse_blk3_rdata4_reg;
#[doc = "EFUSE_BLK3_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata5_reg](efuse_blk3_rdata5_reg) module"]
pub type EFUSE_BLK3_RDATA5_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA5_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata5_reg::R](efuse_blk3_rdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata5_reg::W](efuse_blk3_rdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA5_REG {}
#[doc = "EFUSE_BLK3_RDATA5_REG"]
pub mod efuse_blk3_rdata5_reg;
#[doc = "EFUSE_BLK3_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata6_reg](efuse_blk3_rdata6_reg) module"]
pub type EFUSE_BLK3_RDATA6_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA6_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata6_reg::R](efuse_blk3_rdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata6_reg::W](efuse_blk3_rdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA6_REG {}
#[doc = "EFUSE_BLK3_RDATA6_REG"]
pub mod efuse_blk3_rdata6_reg;
#[doc = "EFUSE_BLK3_RDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_rdata7_reg](efuse_blk3_rdata7_reg) module"]
pub type EFUSE_BLK3_RDATA7_REG = crate::Reg<u32, _EFUSE_BLK3_RDATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_RDATA7_REG;
#[doc = "`read()` method returns [efuse_blk3_rdata7_reg::R](efuse_blk3_rdata7_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_RDATA7_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_rdata7_reg::W](efuse_blk3_rdata7_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_RDATA7_REG {}
#[doc = "EFUSE_BLK3_RDATA7_REG"]
pub mod efuse_blk3_rdata7_reg;
#[doc = "EFUSE_BLK1_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata0_reg](efuse_blk1_wdata0_reg) module"]
pub type EFUSE_BLK1_WDATA0_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA0_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata0_reg::R](efuse_blk1_wdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata0_reg::W](efuse_blk1_wdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA0_REG {}
#[doc = "EFUSE_BLK1_WDATA0_REG"]
pub mod efuse_blk1_wdata0_reg;
#[doc = "EFUSE_BLK1_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata1_reg](efuse_blk1_wdata1_reg) module"]
pub type EFUSE_BLK1_WDATA1_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA1_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata1_reg::R](efuse_blk1_wdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata1_reg::W](efuse_blk1_wdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA1_REG {}
#[doc = "EFUSE_BLK1_WDATA1_REG"]
pub mod efuse_blk1_wdata1_reg;
#[doc = "EFUSE_BLK1_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata2_reg](efuse_blk1_wdata2_reg) module"]
pub type EFUSE_BLK1_WDATA2_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA2_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata2_reg::R](efuse_blk1_wdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata2_reg::W](efuse_blk1_wdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA2_REG {}
#[doc = "EFUSE_BLK1_WDATA2_REG"]
pub mod efuse_blk1_wdata2_reg;
#[doc = "EFUSE_BLK1_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata3_reg](efuse_blk1_wdata3_reg) module"]
pub type EFUSE_BLK1_WDATA3_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA3_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata3_reg::R](efuse_blk1_wdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata3_reg::W](efuse_blk1_wdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA3_REG {}
#[doc = "EFUSE_BLK1_WDATA3_REG"]
pub mod efuse_blk1_wdata3_reg;
#[doc = "EFUSE_BLK1_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata4_reg](efuse_blk1_wdata4_reg) module"]
pub type EFUSE_BLK1_WDATA4_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA4_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata4_reg::R](efuse_blk1_wdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata4_reg::W](efuse_blk1_wdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA4_REG {}
#[doc = "EFUSE_BLK1_WDATA4_REG"]
pub mod efuse_blk1_wdata4_reg;
#[doc = "EFUSE_BLK1_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata5_reg](efuse_blk1_wdata5_reg) module"]
pub type EFUSE_BLK1_WDATA5_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA5_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata5_reg::R](efuse_blk1_wdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata5_reg::W](efuse_blk1_wdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA5_REG {}
#[doc = "EFUSE_BLK1_WDATA5_REG"]
pub mod efuse_blk1_wdata5_reg;
#[doc = "EFUSE_BLK1_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata6_reg](efuse_blk1_wdata6_reg) module"]
pub type EFUSE_BLK1_WDATA6_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA6_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata6_reg::R](efuse_blk1_wdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata6_reg::W](efuse_blk1_wdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA6_REG {}
#[doc = "EFUSE_BLK1_WDATA6_REG"]
pub mod efuse_blk1_wdata6_reg;
#[doc = "EFUSE_BLK1_WDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk1_wdata7_reg](efuse_blk1_wdata7_reg) module"]
pub type EFUSE_BLK1_WDATA7_REG = crate::Reg<u32, _EFUSE_BLK1_WDATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK1_WDATA7_REG;
#[doc = "`read()` method returns [efuse_blk1_wdata7_reg::R](efuse_blk1_wdata7_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK1_WDATA7_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk1_wdata7_reg::W](efuse_blk1_wdata7_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK1_WDATA7_REG {}
#[doc = "EFUSE_BLK1_WDATA7_REG"]
pub mod efuse_blk1_wdata7_reg;
#[doc = "EFUSE_BLK2_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata0_reg](efuse_blk2_wdata0_reg) module"]
pub type EFUSE_BLK2_WDATA0_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA0_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata0_reg::R](efuse_blk2_wdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata0_reg::W](efuse_blk2_wdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA0_REG {}
#[doc = "EFUSE_BLK2_WDATA0_REG"]
pub mod efuse_blk2_wdata0_reg;
#[doc = "EFUSE_BLK2_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata1_reg](efuse_blk2_wdata1_reg) module"]
pub type EFUSE_BLK2_WDATA1_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA1_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata1_reg::R](efuse_blk2_wdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata1_reg::W](efuse_blk2_wdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA1_REG {}
#[doc = "EFUSE_BLK2_WDATA1_REG"]
pub mod efuse_blk2_wdata1_reg;
#[doc = "EFUSE_BLK2_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata2_reg](efuse_blk2_wdata2_reg) module"]
pub type EFUSE_BLK2_WDATA2_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA2_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata2_reg::R](efuse_blk2_wdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata2_reg::W](efuse_blk2_wdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA2_REG {}
#[doc = "EFUSE_BLK2_WDATA2_REG"]
pub mod efuse_blk2_wdata2_reg;
#[doc = "EFUSE_BLK2_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata3_reg](efuse_blk2_wdata3_reg) module"]
pub type EFUSE_BLK2_WDATA3_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA3_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata3_reg::R](efuse_blk2_wdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata3_reg::W](efuse_blk2_wdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA3_REG {}
#[doc = "EFUSE_BLK2_WDATA3_REG"]
pub mod efuse_blk2_wdata3_reg;
#[doc = "EFUSE_BLK2_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata4_reg](efuse_blk2_wdata4_reg) module"]
pub type EFUSE_BLK2_WDATA4_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA4_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata4_reg::R](efuse_blk2_wdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata4_reg::W](efuse_blk2_wdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA4_REG {}
#[doc = "EFUSE_BLK2_WDATA4_REG"]
pub mod efuse_blk2_wdata4_reg;
#[doc = "EFUSE_BLK2_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata5_reg](efuse_blk2_wdata5_reg) module"]
pub type EFUSE_BLK2_WDATA5_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA5_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata5_reg::R](efuse_blk2_wdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata5_reg::W](efuse_blk2_wdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA5_REG {}
#[doc = "EFUSE_BLK2_WDATA5_REG"]
pub mod efuse_blk2_wdata5_reg;
#[doc = "EFUSE_BLK2_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata6_reg](efuse_blk2_wdata6_reg) module"]
pub type EFUSE_BLK2_WDATA6_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA6_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata6_reg::R](efuse_blk2_wdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata6_reg::W](efuse_blk2_wdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA6_REG {}
#[doc = "EFUSE_BLK2_WDATA6_REG"]
pub mod efuse_blk2_wdata6_reg;
#[doc = "EFUSE_BLK2_WDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk2_wdata7_reg](efuse_blk2_wdata7_reg) module"]
pub type EFUSE_BLK2_WDATA7_REG = crate::Reg<u32, _EFUSE_BLK2_WDATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK2_WDATA7_REG;
#[doc = "`read()` method returns [efuse_blk2_wdata7_reg::R](efuse_blk2_wdata7_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK2_WDATA7_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk2_wdata7_reg::W](efuse_blk2_wdata7_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK2_WDATA7_REG {}
#[doc = "EFUSE_BLK2_WDATA7_REG"]
pub mod efuse_blk2_wdata7_reg;
#[doc = "EFUSE_BLK3_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata0_reg](efuse_blk3_wdata0_reg) module"]
pub type EFUSE_BLK3_WDATA0_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA0_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata0_reg::R](efuse_blk3_wdata0_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA0_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata0_reg::W](efuse_blk3_wdata0_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA0_REG {}
#[doc = "EFUSE_BLK3_WDATA0_REG"]
pub mod efuse_blk3_wdata0_reg;
#[doc = "EFUSE_BLK3_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata1_reg](efuse_blk3_wdata1_reg) module"]
pub type EFUSE_BLK3_WDATA1_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA1_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata1_reg::R](efuse_blk3_wdata1_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA1_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata1_reg::W](efuse_blk3_wdata1_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA1_REG {}
#[doc = "EFUSE_BLK3_WDATA1_REG"]
pub mod efuse_blk3_wdata1_reg;
#[doc = "EFUSE_BLK3_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata2_reg](efuse_blk3_wdata2_reg) module"]
pub type EFUSE_BLK3_WDATA2_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA2_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata2_reg::R](efuse_blk3_wdata2_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA2_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata2_reg::W](efuse_blk3_wdata2_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA2_REG {}
#[doc = "EFUSE_BLK3_WDATA2_REG"]
pub mod efuse_blk3_wdata2_reg;
#[doc = "EFUSE_BLK3_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata3_reg](efuse_blk3_wdata3_reg) module"]
pub type EFUSE_BLK3_WDATA3_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA3_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata3_reg::R](efuse_blk3_wdata3_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA3_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata3_reg::W](efuse_blk3_wdata3_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA3_REG {}
#[doc = "EFUSE_BLK3_WDATA3_REG"]
pub mod efuse_blk3_wdata3_reg;
#[doc = "EFUSE_BLK3_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata4_reg](efuse_blk3_wdata4_reg) module"]
pub type EFUSE_BLK3_WDATA4_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA4_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata4_reg::R](efuse_blk3_wdata4_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA4_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata4_reg::W](efuse_blk3_wdata4_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA4_REG {}
#[doc = "EFUSE_BLK3_WDATA4_REG"]
pub mod efuse_blk3_wdata4_reg;
#[doc = "EFUSE_BLK3_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata5_reg](efuse_blk3_wdata5_reg) module"]
pub type EFUSE_BLK3_WDATA5_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA5_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata5_reg::R](efuse_blk3_wdata5_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA5_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata5_reg::W](efuse_blk3_wdata5_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA5_REG {}
#[doc = "EFUSE_BLK3_WDATA5_REG"]
pub mod efuse_blk3_wdata5_reg;
#[doc = "EFUSE_BLK3_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata6_reg](efuse_blk3_wdata6_reg) module"]
pub type EFUSE_BLK3_WDATA6_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA6_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata6_reg::R](efuse_blk3_wdata6_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA6_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata6_reg::W](efuse_blk3_wdata6_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA6_REG {}
#[doc = "EFUSE_BLK3_WDATA6_REG"]
pub mod efuse_blk3_wdata6_reg;
#[doc = "EFUSE_BLK3_WDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_blk3_wdata7_reg](efuse_blk3_wdata7_reg) module"]
pub type EFUSE_BLK3_WDATA7_REG = crate::Reg<u32, _EFUSE_BLK3_WDATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_BLK3_WDATA7_REG;
#[doc = "`read()` method returns [efuse_blk3_wdata7_reg::R](efuse_blk3_wdata7_reg::R) reader structure"]
impl crate::Readable for EFUSE_BLK3_WDATA7_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_blk3_wdata7_reg::W](efuse_blk3_wdata7_reg::W) writer structure"]
impl crate::Writable for EFUSE_BLK3_WDATA7_REG {}
#[doc = "EFUSE_BLK3_WDATA7_REG"]
pub mod efuse_blk3_wdata7_reg;
#[doc = "EFUSE_CLK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_clk_reg](efuse_clk_reg) module"]
pub type EFUSE_CLK_REG = crate::Reg<u32, _EFUSE_CLK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CLK_REG;
#[doc = "`read()` method returns [efuse_clk_reg::R](efuse_clk_reg::R) reader structure"]
impl crate::Readable for EFUSE_CLK_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_clk_reg::W](efuse_clk_reg::W) writer structure"]
impl crate::Writable for EFUSE_CLK_REG {}
#[doc = "EFUSE_CLK_REG"]
pub mod efuse_clk_reg;
#[doc = "EFUSE_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_conf_reg](efuse_conf_reg) module"]
pub type EFUSE_CONF_REG = crate::Reg<u32, _EFUSE_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CONF_REG;
#[doc = "`read()` method returns [efuse_conf_reg::R](efuse_conf_reg::R) reader structure"]
impl crate::Readable for EFUSE_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_conf_reg::W](efuse_conf_reg::W) writer structure"]
impl crate::Writable for EFUSE_CONF_REG {}
#[doc = "EFUSE_CONF_REG"]
pub mod efuse_conf_reg;
#[doc = "EFUSE_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_status_reg](efuse_status_reg) module"]
pub type EFUSE_STATUS_REG = crate::Reg<u32, _EFUSE_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_STATUS_REG;
#[doc = "`read()` method returns [efuse_status_reg::R](efuse_status_reg::R) reader structure"]
impl crate::Readable for EFUSE_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_status_reg::W](efuse_status_reg::W) writer structure"]
impl crate::Writable for EFUSE_STATUS_REG {}
#[doc = "EFUSE_STATUS_REG"]
pub mod efuse_status_reg;
#[doc = "EFUSE_CMD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_cmd_reg](efuse_cmd_reg) module"]
pub type EFUSE_CMD_REG = crate::Reg<u32, _EFUSE_CMD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CMD_REG;
#[doc = "`read()` method returns [efuse_cmd_reg::R](efuse_cmd_reg::R) reader structure"]
impl crate::Readable for EFUSE_CMD_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_cmd_reg::W](efuse_cmd_reg::W) writer structure"]
impl crate::Writable for EFUSE_CMD_REG {}
#[doc = "EFUSE_CMD_REG"]
pub mod efuse_cmd_reg;
#[doc = "EFUSE_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_int_raw_reg](efuse_int_raw_reg) module"]
pub type EFUSE_INT_RAW_REG = crate::Reg<u32, _EFUSE_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_RAW_REG;
#[doc = "`read()` method returns [efuse_int_raw_reg::R](efuse_int_raw_reg::R) reader structure"]
impl crate::Readable for EFUSE_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_int_raw_reg::W](efuse_int_raw_reg::W) writer structure"]
impl crate::Writable for EFUSE_INT_RAW_REG {}
#[doc = "EFUSE_INT_RAW_REG"]
pub mod efuse_int_raw_reg;
#[doc = "EFUSE_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_int_st_reg](efuse_int_st_reg) module"]
pub type EFUSE_INT_ST_REG = crate::Reg<u32, _EFUSE_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_ST_REG;
#[doc = "`read()` method returns [efuse_int_st_reg::R](efuse_int_st_reg::R) reader structure"]
impl crate::Readable for EFUSE_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_int_st_reg::W](efuse_int_st_reg::W) writer structure"]
impl crate::Writable for EFUSE_INT_ST_REG {}
#[doc = "EFUSE_INT_ST_REG"]
pub mod efuse_int_st_reg;
#[doc = "EFUSE_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_int_ena_reg](efuse_int_ena_reg) module"]
pub type EFUSE_INT_ENA_REG = crate::Reg<u32, _EFUSE_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_ENA_REG;
#[doc = "`read()` method returns [efuse_int_ena_reg::R](efuse_int_ena_reg::R) reader structure"]
impl crate::Readable for EFUSE_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_int_ena_reg::W](efuse_int_ena_reg::W) writer structure"]
impl crate::Writable for EFUSE_INT_ENA_REG {}
#[doc = "EFUSE_INT_ENA_REG"]
pub mod efuse_int_ena_reg;
#[doc = "EFUSE_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_int_clr_reg](efuse_int_clr_reg) module"]
pub type EFUSE_INT_CLR_REG = crate::Reg<u32, _EFUSE_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_CLR_REG;
#[doc = "`read()` method returns [efuse_int_clr_reg::R](efuse_int_clr_reg::R) reader structure"]
impl crate::Readable for EFUSE_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_int_clr_reg::W](efuse_int_clr_reg::W) writer structure"]
impl crate::Writable for EFUSE_INT_CLR_REG {}
#[doc = "EFUSE_INT_CLR_REG"]
pub mod efuse_int_clr_reg;
#[doc = "EFUSE_DAC_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_dac_conf_reg](efuse_dac_conf_reg) module"]
pub type EFUSE_DAC_CONF_REG = crate::Reg<u32, _EFUSE_DAC_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_DAC_CONF_REG;
#[doc = "`read()` method returns [efuse_dac_conf_reg::R](efuse_dac_conf_reg::R) reader structure"]
impl crate::Readable for EFUSE_DAC_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_dac_conf_reg::W](efuse_dac_conf_reg::W) writer structure"]
impl crate::Writable for EFUSE_DAC_CONF_REG {}
#[doc = "EFUSE_DAC_CONF_REG"]
pub mod efuse_dac_conf_reg;
#[doc = "EFUSE_DEC_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_dec_status_reg](efuse_dec_status_reg) module"]
pub type EFUSE_DEC_STATUS_REG = crate::Reg<u32, _EFUSE_DEC_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_DEC_STATUS_REG;
#[doc = "`read()` method returns [efuse_dec_status_reg::R](efuse_dec_status_reg::R) reader structure"]
impl crate::Readable for EFUSE_DEC_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_dec_status_reg::W](efuse_dec_status_reg::W) writer structure"]
impl crate::Writable for EFUSE_DEC_STATUS_REG {}
#[doc = "EFUSE_DEC_STATUS_REG"]
pub mod efuse_dec_status_reg;
#[doc = "EFUSE_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuse_date_reg](efuse_date_reg) module"]
pub type EFUSE_DATE_REG = crate::Reg<u32, _EFUSE_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_DATE_REG;
#[doc = "`read()` method returns [efuse_date_reg::R](efuse_date_reg::R) reader structure"]
impl crate::Readable for EFUSE_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [efuse_date_reg::W](efuse_date_reg::W) writer structure"]
impl crate::Writable for EFUSE_DATE_REG {}
#[doc = "EFUSE_DATE_REG"]
pub mod efuse_date_reg;
