#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EFUSE_BLK0_RDATA0_REG"]
    pub blk0_rdata0: BLK0_RDATA0,
    #[doc = "0x04 - EFUSE_BLK0_RDATA1_REG"]
    pub blk0_rdata1: BLK0_RDATA1,
    #[doc = "0x08 - EFUSE_BLK0_RDATA2_REG"]
    pub blk0_rdata2: BLK0_RDATA2,
    #[doc = "0x0c - EFUSE_BLK0_RDATA3_REG"]
    pub blk0_rdata3: BLK0_RDATA3,
    #[doc = "0x10 - EFUSE_BLK0_RDATA4_REG"]
    pub blk0_rdata4: BLK0_RDATA4,
    #[doc = "0x14 - EFUSE_BLK0_RDATA5_REG"]
    pub blk0_rdata5: BLK0_RDATA5,
    #[doc = "0x18 - EFUSE_BLK0_RDATA6_REG"]
    pub blk0_rdata6: BLK0_RDATA6,
    #[doc = "0x1c - EFUSE_BLK0_WDATA0_REG"]
    pub blk0_wdata0: BLK0_WDATA0,
    #[doc = "0x20 - EFUSE_BLK0_WDATA1_REG"]
    pub blk0_wdata1: BLK0_WDATA1,
    #[doc = "0x24 - EFUSE_BLK0_WDATA2_REG"]
    pub blk0_wdata2: BLK0_WDATA2,
    #[doc = "0x28 - EFUSE_BLK0_WDATA3_REG"]
    pub blk0_wdata3: BLK0_WDATA3,
    #[doc = "0x2c - EFUSE_BLK0_WDATA4_REG"]
    pub blk0_wdata4: BLK0_WDATA4,
    #[doc = "0x30 - EFUSE_BLK0_WDATA5_REG"]
    pub blk0_wdata5: BLK0_WDATA5,
    #[doc = "0x34 - EFUSE_BLK0_WDATA6_REG"]
    pub blk0_wdata6: BLK0_WDATA6,
    #[doc = "0x38 - EFUSE_BLK1_RDATA0_REG"]
    pub blk1_rdata0: BLK1_RDATA0,
    #[doc = "0x3c - EFUSE_BLK1_RDATA1_REG"]
    pub blk1_rdata1: BLK1_RDATA1,
    #[doc = "0x40 - EFUSE_BLK1_RDATA2_REG"]
    pub blk1_rdata2: BLK1_RDATA2,
    #[doc = "0x44 - EFUSE_BLK1_RDATA3_REG"]
    pub blk1_rdata3: BLK1_RDATA3,
    #[doc = "0x48 - EFUSE_BLK1_RDATA4_REG"]
    pub blk1_rdata4: BLK1_RDATA4,
    #[doc = "0x4c - EFUSE_BLK1_RDATA5_REG"]
    pub blk1_rdata5: BLK1_RDATA5,
    #[doc = "0x50 - EFUSE_BLK1_RDATA6_REG"]
    pub blk1_rdata6: BLK1_RDATA6,
    #[doc = "0x54 - EFUSE_BLK1_RDATA7_REG"]
    pub blk1_rdata7: BLK1_RDATA7,
    #[doc = "0x58 - EFUSE_BLK2_RDATA0_REG"]
    pub blk2_rdata0: BLK2_RDATA0,
    #[doc = "0x5c - EFUSE_BLK2_RDATA1_REG"]
    pub blk2_rdata1: BLK2_RDATA1,
    #[doc = "0x60 - EFUSE_BLK2_RDATA2_REG"]
    pub blk2_rdata2: BLK2_RDATA2,
    #[doc = "0x64 - EFUSE_BLK2_RDATA3_REG"]
    pub blk2_rdata3: BLK2_RDATA3,
    #[doc = "0x68 - EFUSE_BLK2_RDATA4_REG"]
    pub blk2_rdata4: BLK2_RDATA4,
    #[doc = "0x6c - EFUSE_BLK2_RDATA5_REG"]
    pub blk2_rdata5: BLK2_RDATA5,
    #[doc = "0x70 - EFUSE_BLK2_RDATA6_REG"]
    pub blk2_rdata6: BLK2_RDATA6,
    #[doc = "0x74 - EFUSE_BLK2_RDATA7_REG"]
    pub blk2_rdata7: BLK2_RDATA7,
    #[doc = "0x78 - EFUSE_BLK3_RDATA0_REG"]
    pub blk3_rdata0: BLK3_RDATA0,
    #[doc = "0x7c - EFUSE_BLK3_RDATA1_REG"]
    pub blk3_rdata1: BLK3_RDATA1,
    #[doc = "0x80 - EFUSE_BLK3_RDATA2_REG"]
    pub blk3_rdata2: BLK3_RDATA2,
    #[doc = "0x84 - EFUSE_BLK3_RDATA3_REG"]
    pub blk3_rdata3: BLK3_RDATA3,
    #[doc = "0x88 - EFUSE_BLK3_RDATA4_REG"]
    pub blk3_rdata4: BLK3_RDATA4,
    #[doc = "0x8c - EFUSE_BLK3_RDATA5_REG"]
    pub blk3_rdata5: BLK3_RDATA5,
    #[doc = "0x90 - EFUSE_BLK3_RDATA6_REG"]
    pub blk3_rdata6: BLK3_RDATA6,
    #[doc = "0x94 - EFUSE_BLK3_RDATA7_REG"]
    pub blk3_rdata7: BLK3_RDATA7,
    #[doc = "0x98 - EFUSE_BLK1_WDATA0_REG"]
    pub blk1_wdata0: BLK1_WDATA0,
    #[doc = "0x9c - EFUSE_BLK1_WDATA1_REG"]
    pub blk1_wdata1: BLK1_WDATA1,
    #[doc = "0xa0 - EFUSE_BLK1_WDATA2_REG"]
    pub blk1_wdata2: BLK1_WDATA2,
    #[doc = "0xa4 - EFUSE_BLK1_WDATA3_REG"]
    pub blk1_wdata3: BLK1_WDATA3,
    #[doc = "0xa8 - EFUSE_BLK1_WDATA4_REG"]
    pub blk1_wdata4: BLK1_WDATA4,
    #[doc = "0xac - EFUSE_BLK1_WDATA5_REG"]
    pub blk1_wdata5: BLK1_WDATA5,
    #[doc = "0xb0 - EFUSE_BLK1_WDATA6_REG"]
    pub blk1_wdata6: BLK1_WDATA6,
    #[doc = "0xb4 - EFUSE_BLK1_WDATA7_REG"]
    pub blk1_wdata7: BLK1_WDATA7,
    #[doc = "0xb8 - EFUSE_BLK2_WDATA0_REG"]
    pub blk2_wdata0: BLK2_WDATA0,
    #[doc = "0xbc - EFUSE_BLK2_WDATA1_REG"]
    pub blk2_wdata1: BLK2_WDATA1,
    #[doc = "0xc0 - EFUSE_BLK2_WDATA2_REG"]
    pub blk2_wdata2: BLK2_WDATA2,
    #[doc = "0xc4 - EFUSE_BLK2_WDATA3_REG"]
    pub blk2_wdata3: BLK2_WDATA3,
    #[doc = "0xc8 - EFUSE_BLK2_WDATA4_REG"]
    pub blk2_wdata4: BLK2_WDATA4,
    #[doc = "0xcc - EFUSE_BLK2_WDATA5_REG"]
    pub blk2_wdata5: BLK2_WDATA5,
    #[doc = "0xd0 - EFUSE_BLK2_WDATA6_REG"]
    pub blk2_wdata6: BLK2_WDATA6,
    #[doc = "0xd4 - EFUSE_BLK2_WDATA7_REG"]
    pub blk2_wdata7: BLK2_WDATA7,
    #[doc = "0xd8 - EFUSE_BLK3_WDATA0_REG"]
    pub blk3_wdata0: BLK3_WDATA0,
    #[doc = "0xdc - EFUSE_BLK3_WDATA1_REG"]
    pub blk3_wdata1: BLK3_WDATA1,
    #[doc = "0xe0 - EFUSE_BLK3_WDATA2_REG"]
    pub blk3_wdata2: BLK3_WDATA2,
    #[doc = "0xe4 - EFUSE_BLK3_WDATA3_REG"]
    pub blk3_wdata3: BLK3_WDATA3,
    #[doc = "0xe8 - EFUSE_BLK3_WDATA4_REG"]
    pub blk3_wdata4: BLK3_WDATA4,
    #[doc = "0xec - EFUSE_BLK3_WDATA5_REG"]
    pub blk3_wdata5: BLK3_WDATA5,
    #[doc = "0xf0 - EFUSE_BLK3_WDATA6_REG"]
    pub blk3_wdata6: BLK3_WDATA6,
    #[doc = "0xf4 - EFUSE_BLK3_WDATA7_REG"]
    pub blk3_wdata7: BLK3_WDATA7,
    #[doc = "0xf8 - EFUSE_CLK_REG"]
    pub clk: CLK,
    #[doc = "0xfc - EFUSE_CONF_REG"]
    pub conf: CONF,
    #[doc = "0x100 - EFUSE_STATUS_REG"]
    pub status: STATUS,
    #[doc = "0x104 - EFUSE_CMD_REG"]
    pub cmd: CMD,
    #[doc = "0x108 - EFUSE_INT_RAW_REG"]
    pub int_raw: INT_RAW,
    #[doc = "0x10c - EFUSE_INT_ST_REG"]
    pub int_st: INT_ST,
    #[doc = "0x110 - EFUSE_INT_ENA_REG"]
    pub int_ena: INT_ENA,
    #[doc = "0x114 - EFUSE_INT_CLR_REG"]
    pub int_clr: INT_CLR,
    #[doc = "0x118 - EFUSE_DAC_CONF_REG"]
    pub dac_conf: DAC_CONF,
    #[doc = "0x11c - EFUSE_DEC_STATUS_REG"]
    pub dec_status: DEC_STATUS,
    _reserved72: [u8; 220usize],
    #[doc = "0x1fc - EFUSE_DATE_REG"]
    pub date: DATE,
}
#[doc = "EFUSE_BLK0_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata0](blk0_rdata0) module"]
pub type BLK0_RDATA0 = crate::Reg<u32, _BLK0_RDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA0;
#[doc = "`read()` method returns [blk0_rdata0::R](blk0_rdata0::R) reader structure"]
impl crate::Readable for BLK0_RDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata0::W](blk0_rdata0::W) writer structure"]
impl crate::Writable for BLK0_RDATA0 {}
#[doc = "EFUSE_BLK0_RDATA0_REG"]
pub mod blk0_rdata0;
#[doc = "EFUSE_BLK0_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata1](blk0_rdata1) module"]
pub type BLK0_RDATA1 = crate::Reg<u32, _BLK0_RDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA1;
#[doc = "`read()` method returns [blk0_rdata1::R](blk0_rdata1::R) reader structure"]
impl crate::Readable for BLK0_RDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata1::W](blk0_rdata1::W) writer structure"]
impl crate::Writable for BLK0_RDATA1 {}
#[doc = "EFUSE_BLK0_RDATA1_REG"]
pub mod blk0_rdata1;
#[doc = "EFUSE_BLK0_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata2](blk0_rdata2) module"]
pub type BLK0_RDATA2 = crate::Reg<u32, _BLK0_RDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA2;
#[doc = "`read()` method returns [blk0_rdata2::R](blk0_rdata2::R) reader structure"]
impl crate::Readable for BLK0_RDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata2::W](blk0_rdata2::W) writer structure"]
impl crate::Writable for BLK0_RDATA2 {}
#[doc = "EFUSE_BLK0_RDATA2_REG"]
pub mod blk0_rdata2;
#[doc = "EFUSE_BLK0_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata3](blk0_rdata3) module"]
pub type BLK0_RDATA3 = crate::Reg<u32, _BLK0_RDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA3;
#[doc = "`read()` method returns [blk0_rdata3::R](blk0_rdata3::R) reader structure"]
impl crate::Readable for BLK0_RDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata3::W](blk0_rdata3::W) writer structure"]
impl crate::Writable for BLK0_RDATA3 {}
#[doc = "EFUSE_BLK0_RDATA3_REG"]
pub mod blk0_rdata3;
#[doc = "EFUSE_BLK0_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata4](blk0_rdata4) module"]
pub type BLK0_RDATA4 = crate::Reg<u32, _BLK0_RDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA4;
#[doc = "`read()` method returns [blk0_rdata4::R](blk0_rdata4::R) reader structure"]
impl crate::Readable for BLK0_RDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata4::W](blk0_rdata4::W) writer structure"]
impl crate::Writable for BLK0_RDATA4 {}
#[doc = "EFUSE_BLK0_RDATA4_REG"]
pub mod blk0_rdata4;
#[doc = "EFUSE_BLK0_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata5](blk0_rdata5) module"]
pub type BLK0_RDATA5 = crate::Reg<u32, _BLK0_RDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA5;
#[doc = "`read()` method returns [blk0_rdata5::R](blk0_rdata5::R) reader structure"]
impl crate::Readable for BLK0_RDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata5::W](blk0_rdata5::W) writer structure"]
impl crate::Writable for BLK0_RDATA5 {}
#[doc = "EFUSE_BLK0_RDATA5_REG"]
pub mod blk0_rdata5;
#[doc = "EFUSE_BLK0_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_rdata6](blk0_rdata6) module"]
pub type BLK0_RDATA6 = crate::Reg<u32, _BLK0_RDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_RDATA6;
#[doc = "`read()` method returns [blk0_rdata6::R](blk0_rdata6::R) reader structure"]
impl crate::Readable for BLK0_RDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk0_rdata6::W](blk0_rdata6::W) writer structure"]
impl crate::Writable for BLK0_RDATA6 {}
#[doc = "EFUSE_BLK0_RDATA6_REG"]
pub mod blk0_rdata6;
#[doc = "EFUSE_BLK0_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata0](blk0_wdata0) module"]
pub type BLK0_WDATA0 = crate::Reg<u32, _BLK0_WDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA0;
#[doc = "`read()` method returns [blk0_wdata0::R](blk0_wdata0::R) reader structure"]
impl crate::Readable for BLK0_WDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata0::W](blk0_wdata0::W) writer structure"]
impl crate::Writable for BLK0_WDATA0 {}
#[doc = "EFUSE_BLK0_WDATA0_REG"]
pub mod blk0_wdata0;
#[doc = "EFUSE_BLK0_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata1](blk0_wdata1) module"]
pub type BLK0_WDATA1 = crate::Reg<u32, _BLK0_WDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA1;
#[doc = "`read()` method returns [blk0_wdata1::R](blk0_wdata1::R) reader structure"]
impl crate::Readable for BLK0_WDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata1::W](blk0_wdata1::W) writer structure"]
impl crate::Writable for BLK0_WDATA1 {}
#[doc = "EFUSE_BLK0_WDATA1_REG"]
pub mod blk0_wdata1;
#[doc = "EFUSE_BLK0_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata2](blk0_wdata2) module"]
pub type BLK0_WDATA2 = crate::Reg<u32, _BLK0_WDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA2;
#[doc = "`read()` method returns [blk0_wdata2::R](blk0_wdata2::R) reader structure"]
impl crate::Readable for BLK0_WDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata2::W](blk0_wdata2::W) writer structure"]
impl crate::Writable for BLK0_WDATA2 {}
#[doc = "EFUSE_BLK0_WDATA2_REG"]
pub mod blk0_wdata2;
#[doc = "EFUSE_BLK0_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata3](blk0_wdata3) module"]
pub type BLK0_WDATA3 = crate::Reg<u32, _BLK0_WDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA3;
#[doc = "`read()` method returns [blk0_wdata3::R](blk0_wdata3::R) reader structure"]
impl crate::Readable for BLK0_WDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata3::W](blk0_wdata3::W) writer structure"]
impl crate::Writable for BLK0_WDATA3 {}
#[doc = "EFUSE_BLK0_WDATA3_REG"]
pub mod blk0_wdata3;
#[doc = "EFUSE_BLK0_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata4](blk0_wdata4) module"]
pub type BLK0_WDATA4 = crate::Reg<u32, _BLK0_WDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA4;
#[doc = "`read()` method returns [blk0_wdata4::R](blk0_wdata4::R) reader structure"]
impl crate::Readable for BLK0_WDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata4::W](blk0_wdata4::W) writer structure"]
impl crate::Writable for BLK0_WDATA4 {}
#[doc = "EFUSE_BLK0_WDATA4_REG"]
pub mod blk0_wdata4;
#[doc = "EFUSE_BLK0_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata5](blk0_wdata5) module"]
pub type BLK0_WDATA5 = crate::Reg<u32, _BLK0_WDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA5;
#[doc = "`read()` method returns [blk0_wdata5::R](blk0_wdata5::R) reader structure"]
impl crate::Readable for BLK0_WDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata5::W](blk0_wdata5::W) writer structure"]
impl crate::Writable for BLK0_WDATA5 {}
#[doc = "EFUSE_BLK0_WDATA5_REG"]
pub mod blk0_wdata5;
#[doc = "EFUSE_BLK0_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk0_wdata6](blk0_wdata6) module"]
pub type BLK0_WDATA6 = crate::Reg<u32, _BLK0_WDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK0_WDATA6;
#[doc = "`read()` method returns [blk0_wdata6::R](blk0_wdata6::R) reader structure"]
impl crate::Readable for BLK0_WDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk0_wdata6::W](blk0_wdata6::W) writer structure"]
impl crate::Writable for BLK0_WDATA6 {}
#[doc = "EFUSE_BLK0_WDATA6_REG"]
pub mod blk0_wdata6;
#[doc = "EFUSE_BLK1_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata0](blk1_rdata0) module"]
pub type BLK1_RDATA0 = crate::Reg<u32, _BLK1_RDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA0;
#[doc = "`read()` method returns [blk1_rdata0::R](blk1_rdata0::R) reader structure"]
impl crate::Readable for BLK1_RDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata0::W](blk1_rdata0::W) writer structure"]
impl crate::Writable for BLK1_RDATA0 {}
#[doc = "EFUSE_BLK1_RDATA0_REG"]
pub mod blk1_rdata0;
#[doc = "EFUSE_BLK1_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata1](blk1_rdata1) module"]
pub type BLK1_RDATA1 = crate::Reg<u32, _BLK1_RDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA1;
#[doc = "`read()` method returns [blk1_rdata1::R](blk1_rdata1::R) reader structure"]
impl crate::Readable for BLK1_RDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata1::W](blk1_rdata1::W) writer structure"]
impl crate::Writable for BLK1_RDATA1 {}
#[doc = "EFUSE_BLK1_RDATA1_REG"]
pub mod blk1_rdata1;
#[doc = "EFUSE_BLK1_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata2](blk1_rdata2) module"]
pub type BLK1_RDATA2 = crate::Reg<u32, _BLK1_RDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA2;
#[doc = "`read()` method returns [blk1_rdata2::R](blk1_rdata2::R) reader structure"]
impl crate::Readable for BLK1_RDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata2::W](blk1_rdata2::W) writer structure"]
impl crate::Writable for BLK1_RDATA2 {}
#[doc = "EFUSE_BLK1_RDATA2_REG"]
pub mod blk1_rdata2;
#[doc = "EFUSE_BLK1_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata3](blk1_rdata3) module"]
pub type BLK1_RDATA3 = crate::Reg<u32, _BLK1_RDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA3;
#[doc = "`read()` method returns [blk1_rdata3::R](blk1_rdata3::R) reader structure"]
impl crate::Readable for BLK1_RDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata3::W](blk1_rdata3::W) writer structure"]
impl crate::Writable for BLK1_RDATA3 {}
#[doc = "EFUSE_BLK1_RDATA3_REG"]
pub mod blk1_rdata3;
#[doc = "EFUSE_BLK1_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata4](blk1_rdata4) module"]
pub type BLK1_RDATA4 = crate::Reg<u32, _BLK1_RDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA4;
#[doc = "`read()` method returns [blk1_rdata4::R](blk1_rdata4::R) reader structure"]
impl crate::Readable for BLK1_RDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata4::W](blk1_rdata4::W) writer structure"]
impl crate::Writable for BLK1_RDATA4 {}
#[doc = "EFUSE_BLK1_RDATA4_REG"]
pub mod blk1_rdata4;
#[doc = "EFUSE_BLK1_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata5](blk1_rdata5) module"]
pub type BLK1_RDATA5 = crate::Reg<u32, _BLK1_RDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA5;
#[doc = "`read()` method returns [blk1_rdata5::R](blk1_rdata5::R) reader structure"]
impl crate::Readable for BLK1_RDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata5::W](blk1_rdata5::W) writer structure"]
impl crate::Writable for BLK1_RDATA5 {}
#[doc = "EFUSE_BLK1_RDATA5_REG"]
pub mod blk1_rdata5;
#[doc = "EFUSE_BLK1_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata6](blk1_rdata6) module"]
pub type BLK1_RDATA6 = crate::Reg<u32, _BLK1_RDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA6;
#[doc = "`read()` method returns [blk1_rdata6::R](blk1_rdata6::R) reader structure"]
impl crate::Readable for BLK1_RDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata6::W](blk1_rdata6::W) writer structure"]
impl crate::Writable for BLK1_RDATA6 {}
#[doc = "EFUSE_BLK1_RDATA6_REG"]
pub mod blk1_rdata6;
#[doc = "EFUSE_BLK1_RDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_rdata7](blk1_rdata7) module"]
pub type BLK1_RDATA7 = crate::Reg<u32, _BLK1_RDATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_RDATA7;
#[doc = "`read()` method returns [blk1_rdata7::R](blk1_rdata7::R) reader structure"]
impl crate::Readable for BLK1_RDATA7 {}
#[doc = "`write(|w| ..)` method takes [blk1_rdata7::W](blk1_rdata7::W) writer structure"]
impl crate::Writable for BLK1_RDATA7 {}
#[doc = "EFUSE_BLK1_RDATA7_REG"]
pub mod blk1_rdata7;
#[doc = "EFUSE_BLK2_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata0](blk2_rdata0) module"]
pub type BLK2_RDATA0 = crate::Reg<u32, _BLK2_RDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA0;
#[doc = "`read()` method returns [blk2_rdata0::R](blk2_rdata0::R) reader structure"]
impl crate::Readable for BLK2_RDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata0::W](blk2_rdata0::W) writer structure"]
impl crate::Writable for BLK2_RDATA0 {}
#[doc = "EFUSE_BLK2_RDATA0_REG"]
pub mod blk2_rdata0;
#[doc = "EFUSE_BLK2_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata1](blk2_rdata1) module"]
pub type BLK2_RDATA1 = crate::Reg<u32, _BLK2_RDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA1;
#[doc = "`read()` method returns [blk2_rdata1::R](blk2_rdata1::R) reader structure"]
impl crate::Readable for BLK2_RDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata1::W](blk2_rdata1::W) writer structure"]
impl crate::Writable for BLK2_RDATA1 {}
#[doc = "EFUSE_BLK2_RDATA1_REG"]
pub mod blk2_rdata1;
#[doc = "EFUSE_BLK2_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata2](blk2_rdata2) module"]
pub type BLK2_RDATA2 = crate::Reg<u32, _BLK2_RDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA2;
#[doc = "`read()` method returns [blk2_rdata2::R](blk2_rdata2::R) reader structure"]
impl crate::Readable for BLK2_RDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata2::W](blk2_rdata2::W) writer structure"]
impl crate::Writable for BLK2_RDATA2 {}
#[doc = "EFUSE_BLK2_RDATA2_REG"]
pub mod blk2_rdata2;
#[doc = "EFUSE_BLK2_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata3](blk2_rdata3) module"]
pub type BLK2_RDATA3 = crate::Reg<u32, _BLK2_RDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA3;
#[doc = "`read()` method returns [blk2_rdata3::R](blk2_rdata3::R) reader structure"]
impl crate::Readable for BLK2_RDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata3::W](blk2_rdata3::W) writer structure"]
impl crate::Writable for BLK2_RDATA3 {}
#[doc = "EFUSE_BLK2_RDATA3_REG"]
pub mod blk2_rdata3;
#[doc = "EFUSE_BLK2_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata4](blk2_rdata4) module"]
pub type BLK2_RDATA4 = crate::Reg<u32, _BLK2_RDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA4;
#[doc = "`read()` method returns [blk2_rdata4::R](blk2_rdata4::R) reader structure"]
impl crate::Readable for BLK2_RDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata4::W](blk2_rdata4::W) writer structure"]
impl crate::Writable for BLK2_RDATA4 {}
#[doc = "EFUSE_BLK2_RDATA4_REG"]
pub mod blk2_rdata4;
#[doc = "EFUSE_BLK2_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata5](blk2_rdata5) module"]
pub type BLK2_RDATA5 = crate::Reg<u32, _BLK2_RDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA5;
#[doc = "`read()` method returns [blk2_rdata5::R](blk2_rdata5::R) reader structure"]
impl crate::Readable for BLK2_RDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata5::W](blk2_rdata5::W) writer structure"]
impl crate::Writable for BLK2_RDATA5 {}
#[doc = "EFUSE_BLK2_RDATA5_REG"]
pub mod blk2_rdata5;
#[doc = "EFUSE_BLK2_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata6](blk2_rdata6) module"]
pub type BLK2_RDATA6 = crate::Reg<u32, _BLK2_RDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA6;
#[doc = "`read()` method returns [blk2_rdata6::R](blk2_rdata6::R) reader structure"]
impl crate::Readable for BLK2_RDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata6::W](blk2_rdata6::W) writer structure"]
impl crate::Writable for BLK2_RDATA6 {}
#[doc = "EFUSE_BLK2_RDATA6_REG"]
pub mod blk2_rdata6;
#[doc = "EFUSE_BLK2_RDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_rdata7](blk2_rdata7) module"]
pub type BLK2_RDATA7 = crate::Reg<u32, _BLK2_RDATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_RDATA7;
#[doc = "`read()` method returns [blk2_rdata7::R](blk2_rdata7::R) reader structure"]
impl crate::Readable for BLK2_RDATA7 {}
#[doc = "`write(|w| ..)` method takes [blk2_rdata7::W](blk2_rdata7::W) writer structure"]
impl crate::Writable for BLK2_RDATA7 {}
#[doc = "EFUSE_BLK2_RDATA7_REG"]
pub mod blk2_rdata7;
#[doc = "EFUSE_BLK3_RDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata0](blk3_rdata0) module"]
pub type BLK3_RDATA0 = crate::Reg<u32, _BLK3_RDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA0;
#[doc = "`read()` method returns [blk3_rdata0::R](blk3_rdata0::R) reader structure"]
impl crate::Readable for BLK3_RDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata0::W](blk3_rdata0::W) writer structure"]
impl crate::Writable for BLK3_RDATA0 {}
#[doc = "EFUSE_BLK3_RDATA0_REG"]
pub mod blk3_rdata0;
#[doc = "EFUSE_BLK3_RDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata1](blk3_rdata1) module"]
pub type BLK3_RDATA1 = crate::Reg<u32, _BLK3_RDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA1;
#[doc = "`read()` method returns [blk3_rdata1::R](blk3_rdata1::R) reader structure"]
impl crate::Readable for BLK3_RDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata1::W](blk3_rdata1::W) writer structure"]
impl crate::Writable for BLK3_RDATA1 {}
#[doc = "EFUSE_BLK3_RDATA1_REG"]
pub mod blk3_rdata1;
#[doc = "EFUSE_BLK3_RDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata2](blk3_rdata2) module"]
pub type BLK3_RDATA2 = crate::Reg<u32, _BLK3_RDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA2;
#[doc = "`read()` method returns [blk3_rdata2::R](blk3_rdata2::R) reader structure"]
impl crate::Readable for BLK3_RDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata2::W](blk3_rdata2::W) writer structure"]
impl crate::Writable for BLK3_RDATA2 {}
#[doc = "EFUSE_BLK3_RDATA2_REG"]
pub mod blk3_rdata2;
#[doc = "EFUSE_BLK3_RDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata3](blk3_rdata3) module"]
pub type BLK3_RDATA3 = crate::Reg<u32, _BLK3_RDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA3;
#[doc = "`read()` method returns [blk3_rdata3::R](blk3_rdata3::R) reader structure"]
impl crate::Readable for BLK3_RDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata3::W](blk3_rdata3::W) writer structure"]
impl crate::Writable for BLK3_RDATA3 {}
#[doc = "EFUSE_BLK3_RDATA3_REG"]
pub mod blk3_rdata3;
#[doc = "EFUSE_BLK3_RDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata4](blk3_rdata4) module"]
pub type BLK3_RDATA4 = crate::Reg<u32, _BLK3_RDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA4;
#[doc = "`read()` method returns [blk3_rdata4::R](blk3_rdata4::R) reader structure"]
impl crate::Readable for BLK3_RDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata4::W](blk3_rdata4::W) writer structure"]
impl crate::Writable for BLK3_RDATA4 {}
#[doc = "EFUSE_BLK3_RDATA4_REG"]
pub mod blk3_rdata4;
#[doc = "EFUSE_BLK3_RDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata5](blk3_rdata5) module"]
pub type BLK3_RDATA5 = crate::Reg<u32, _BLK3_RDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA5;
#[doc = "`read()` method returns [blk3_rdata5::R](blk3_rdata5::R) reader structure"]
impl crate::Readable for BLK3_RDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata5::W](blk3_rdata5::W) writer structure"]
impl crate::Writable for BLK3_RDATA5 {}
#[doc = "EFUSE_BLK3_RDATA5_REG"]
pub mod blk3_rdata5;
#[doc = "EFUSE_BLK3_RDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata6](blk3_rdata6) module"]
pub type BLK3_RDATA6 = crate::Reg<u32, _BLK3_RDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA6;
#[doc = "`read()` method returns [blk3_rdata6::R](blk3_rdata6::R) reader structure"]
impl crate::Readable for BLK3_RDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata6::W](blk3_rdata6::W) writer structure"]
impl crate::Writable for BLK3_RDATA6 {}
#[doc = "EFUSE_BLK3_RDATA6_REG"]
pub mod blk3_rdata6;
#[doc = "EFUSE_BLK3_RDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_rdata7](blk3_rdata7) module"]
pub type BLK3_RDATA7 = crate::Reg<u32, _BLK3_RDATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_RDATA7;
#[doc = "`read()` method returns [blk3_rdata7::R](blk3_rdata7::R) reader structure"]
impl crate::Readable for BLK3_RDATA7 {}
#[doc = "`write(|w| ..)` method takes [blk3_rdata7::W](blk3_rdata7::W) writer structure"]
impl crate::Writable for BLK3_RDATA7 {}
#[doc = "EFUSE_BLK3_RDATA7_REG"]
pub mod blk3_rdata7;
#[doc = "EFUSE_BLK1_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata0](blk1_wdata0) module"]
pub type BLK1_WDATA0 = crate::Reg<u32, _BLK1_WDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA0;
#[doc = "`read()` method returns [blk1_wdata0::R](blk1_wdata0::R) reader structure"]
impl crate::Readable for BLK1_WDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata0::W](blk1_wdata0::W) writer structure"]
impl crate::Writable for BLK1_WDATA0 {}
#[doc = "EFUSE_BLK1_WDATA0_REG"]
pub mod blk1_wdata0;
#[doc = "EFUSE_BLK1_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata1](blk1_wdata1) module"]
pub type BLK1_WDATA1 = crate::Reg<u32, _BLK1_WDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA1;
#[doc = "`read()` method returns [blk1_wdata1::R](blk1_wdata1::R) reader structure"]
impl crate::Readable for BLK1_WDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata1::W](blk1_wdata1::W) writer structure"]
impl crate::Writable for BLK1_WDATA1 {}
#[doc = "EFUSE_BLK1_WDATA1_REG"]
pub mod blk1_wdata1;
#[doc = "EFUSE_BLK1_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata2](blk1_wdata2) module"]
pub type BLK1_WDATA2 = crate::Reg<u32, _BLK1_WDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA2;
#[doc = "`read()` method returns [blk1_wdata2::R](blk1_wdata2::R) reader structure"]
impl crate::Readable for BLK1_WDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata2::W](blk1_wdata2::W) writer structure"]
impl crate::Writable for BLK1_WDATA2 {}
#[doc = "EFUSE_BLK1_WDATA2_REG"]
pub mod blk1_wdata2;
#[doc = "EFUSE_BLK1_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata3](blk1_wdata3) module"]
pub type BLK1_WDATA3 = crate::Reg<u32, _BLK1_WDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA3;
#[doc = "`read()` method returns [blk1_wdata3::R](blk1_wdata3::R) reader structure"]
impl crate::Readable for BLK1_WDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata3::W](blk1_wdata3::W) writer structure"]
impl crate::Writable for BLK1_WDATA3 {}
#[doc = "EFUSE_BLK1_WDATA3_REG"]
pub mod blk1_wdata3;
#[doc = "EFUSE_BLK1_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata4](blk1_wdata4) module"]
pub type BLK1_WDATA4 = crate::Reg<u32, _BLK1_WDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA4;
#[doc = "`read()` method returns [blk1_wdata4::R](blk1_wdata4::R) reader structure"]
impl crate::Readable for BLK1_WDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata4::W](blk1_wdata4::W) writer structure"]
impl crate::Writable for BLK1_WDATA4 {}
#[doc = "EFUSE_BLK1_WDATA4_REG"]
pub mod blk1_wdata4;
#[doc = "EFUSE_BLK1_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata5](blk1_wdata5) module"]
pub type BLK1_WDATA5 = crate::Reg<u32, _BLK1_WDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA5;
#[doc = "`read()` method returns [blk1_wdata5::R](blk1_wdata5::R) reader structure"]
impl crate::Readable for BLK1_WDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata5::W](blk1_wdata5::W) writer structure"]
impl crate::Writable for BLK1_WDATA5 {}
#[doc = "EFUSE_BLK1_WDATA5_REG"]
pub mod blk1_wdata5;
#[doc = "EFUSE_BLK1_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata6](blk1_wdata6) module"]
pub type BLK1_WDATA6 = crate::Reg<u32, _BLK1_WDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA6;
#[doc = "`read()` method returns [blk1_wdata6::R](blk1_wdata6::R) reader structure"]
impl crate::Readable for BLK1_WDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata6::W](blk1_wdata6::W) writer structure"]
impl crate::Writable for BLK1_WDATA6 {}
#[doc = "EFUSE_BLK1_WDATA6_REG"]
pub mod blk1_wdata6;
#[doc = "EFUSE_BLK1_WDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk1_wdata7](blk1_wdata7) module"]
pub type BLK1_WDATA7 = crate::Reg<u32, _BLK1_WDATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK1_WDATA7;
#[doc = "`read()` method returns [blk1_wdata7::R](blk1_wdata7::R) reader structure"]
impl crate::Readable for BLK1_WDATA7 {}
#[doc = "`write(|w| ..)` method takes [blk1_wdata7::W](blk1_wdata7::W) writer structure"]
impl crate::Writable for BLK1_WDATA7 {}
#[doc = "EFUSE_BLK1_WDATA7_REG"]
pub mod blk1_wdata7;
#[doc = "EFUSE_BLK2_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata0](blk2_wdata0) module"]
pub type BLK2_WDATA0 = crate::Reg<u32, _BLK2_WDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA0;
#[doc = "`read()` method returns [blk2_wdata0::R](blk2_wdata0::R) reader structure"]
impl crate::Readable for BLK2_WDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata0::W](blk2_wdata0::W) writer structure"]
impl crate::Writable for BLK2_WDATA0 {}
#[doc = "EFUSE_BLK2_WDATA0_REG"]
pub mod blk2_wdata0;
#[doc = "EFUSE_BLK2_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata1](blk2_wdata1) module"]
pub type BLK2_WDATA1 = crate::Reg<u32, _BLK2_WDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA1;
#[doc = "`read()` method returns [blk2_wdata1::R](blk2_wdata1::R) reader structure"]
impl crate::Readable for BLK2_WDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata1::W](blk2_wdata1::W) writer structure"]
impl crate::Writable for BLK2_WDATA1 {}
#[doc = "EFUSE_BLK2_WDATA1_REG"]
pub mod blk2_wdata1;
#[doc = "EFUSE_BLK2_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata2](blk2_wdata2) module"]
pub type BLK2_WDATA2 = crate::Reg<u32, _BLK2_WDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA2;
#[doc = "`read()` method returns [blk2_wdata2::R](blk2_wdata2::R) reader structure"]
impl crate::Readable for BLK2_WDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata2::W](blk2_wdata2::W) writer structure"]
impl crate::Writable for BLK2_WDATA2 {}
#[doc = "EFUSE_BLK2_WDATA2_REG"]
pub mod blk2_wdata2;
#[doc = "EFUSE_BLK2_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata3](blk2_wdata3) module"]
pub type BLK2_WDATA3 = crate::Reg<u32, _BLK2_WDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA3;
#[doc = "`read()` method returns [blk2_wdata3::R](blk2_wdata3::R) reader structure"]
impl crate::Readable for BLK2_WDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata3::W](blk2_wdata3::W) writer structure"]
impl crate::Writable for BLK2_WDATA3 {}
#[doc = "EFUSE_BLK2_WDATA3_REG"]
pub mod blk2_wdata3;
#[doc = "EFUSE_BLK2_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata4](blk2_wdata4) module"]
pub type BLK2_WDATA4 = crate::Reg<u32, _BLK2_WDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA4;
#[doc = "`read()` method returns [blk2_wdata4::R](blk2_wdata4::R) reader structure"]
impl crate::Readable for BLK2_WDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata4::W](blk2_wdata4::W) writer structure"]
impl crate::Writable for BLK2_WDATA4 {}
#[doc = "EFUSE_BLK2_WDATA4_REG"]
pub mod blk2_wdata4;
#[doc = "EFUSE_BLK2_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata5](blk2_wdata5) module"]
pub type BLK2_WDATA5 = crate::Reg<u32, _BLK2_WDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA5;
#[doc = "`read()` method returns [blk2_wdata5::R](blk2_wdata5::R) reader structure"]
impl crate::Readable for BLK2_WDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata5::W](blk2_wdata5::W) writer structure"]
impl crate::Writable for BLK2_WDATA5 {}
#[doc = "EFUSE_BLK2_WDATA5_REG"]
pub mod blk2_wdata5;
#[doc = "EFUSE_BLK2_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata6](blk2_wdata6) module"]
pub type BLK2_WDATA6 = crate::Reg<u32, _BLK2_WDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA6;
#[doc = "`read()` method returns [blk2_wdata6::R](blk2_wdata6::R) reader structure"]
impl crate::Readable for BLK2_WDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata6::W](blk2_wdata6::W) writer structure"]
impl crate::Writable for BLK2_WDATA6 {}
#[doc = "EFUSE_BLK2_WDATA6_REG"]
pub mod blk2_wdata6;
#[doc = "EFUSE_BLK2_WDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk2_wdata7](blk2_wdata7) module"]
pub type BLK2_WDATA7 = crate::Reg<u32, _BLK2_WDATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK2_WDATA7;
#[doc = "`read()` method returns [blk2_wdata7::R](blk2_wdata7::R) reader structure"]
impl crate::Readable for BLK2_WDATA7 {}
#[doc = "`write(|w| ..)` method takes [blk2_wdata7::W](blk2_wdata7::W) writer structure"]
impl crate::Writable for BLK2_WDATA7 {}
#[doc = "EFUSE_BLK2_WDATA7_REG"]
pub mod blk2_wdata7;
#[doc = "EFUSE_BLK3_WDATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata0](blk3_wdata0) module"]
pub type BLK3_WDATA0 = crate::Reg<u32, _BLK3_WDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA0;
#[doc = "`read()` method returns [blk3_wdata0::R](blk3_wdata0::R) reader structure"]
impl crate::Readable for BLK3_WDATA0 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata0::W](blk3_wdata0::W) writer structure"]
impl crate::Writable for BLK3_WDATA0 {}
#[doc = "EFUSE_BLK3_WDATA0_REG"]
pub mod blk3_wdata0;
#[doc = "EFUSE_BLK3_WDATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata1](blk3_wdata1) module"]
pub type BLK3_WDATA1 = crate::Reg<u32, _BLK3_WDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA1;
#[doc = "`read()` method returns [blk3_wdata1::R](blk3_wdata1::R) reader structure"]
impl crate::Readable for BLK3_WDATA1 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata1::W](blk3_wdata1::W) writer structure"]
impl crate::Writable for BLK3_WDATA1 {}
#[doc = "EFUSE_BLK3_WDATA1_REG"]
pub mod blk3_wdata1;
#[doc = "EFUSE_BLK3_WDATA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata2](blk3_wdata2) module"]
pub type BLK3_WDATA2 = crate::Reg<u32, _BLK3_WDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA2;
#[doc = "`read()` method returns [blk3_wdata2::R](blk3_wdata2::R) reader structure"]
impl crate::Readable for BLK3_WDATA2 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata2::W](blk3_wdata2::W) writer structure"]
impl crate::Writable for BLK3_WDATA2 {}
#[doc = "EFUSE_BLK3_WDATA2_REG"]
pub mod blk3_wdata2;
#[doc = "EFUSE_BLK3_WDATA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata3](blk3_wdata3) module"]
pub type BLK3_WDATA3 = crate::Reg<u32, _BLK3_WDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA3;
#[doc = "`read()` method returns [blk3_wdata3::R](blk3_wdata3::R) reader structure"]
impl crate::Readable for BLK3_WDATA3 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata3::W](blk3_wdata3::W) writer structure"]
impl crate::Writable for BLK3_WDATA3 {}
#[doc = "EFUSE_BLK3_WDATA3_REG"]
pub mod blk3_wdata3;
#[doc = "EFUSE_BLK3_WDATA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata4](blk3_wdata4) module"]
pub type BLK3_WDATA4 = crate::Reg<u32, _BLK3_WDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA4;
#[doc = "`read()` method returns [blk3_wdata4::R](blk3_wdata4::R) reader structure"]
impl crate::Readable for BLK3_WDATA4 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata4::W](blk3_wdata4::W) writer structure"]
impl crate::Writable for BLK3_WDATA4 {}
#[doc = "EFUSE_BLK3_WDATA4_REG"]
pub mod blk3_wdata4;
#[doc = "EFUSE_BLK3_WDATA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata5](blk3_wdata5) module"]
pub type BLK3_WDATA5 = crate::Reg<u32, _BLK3_WDATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA5;
#[doc = "`read()` method returns [blk3_wdata5::R](blk3_wdata5::R) reader structure"]
impl crate::Readable for BLK3_WDATA5 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata5::W](blk3_wdata5::W) writer structure"]
impl crate::Writable for BLK3_WDATA5 {}
#[doc = "EFUSE_BLK3_WDATA5_REG"]
pub mod blk3_wdata5;
#[doc = "EFUSE_BLK3_WDATA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata6](blk3_wdata6) module"]
pub type BLK3_WDATA6 = crate::Reg<u32, _BLK3_WDATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA6;
#[doc = "`read()` method returns [blk3_wdata6::R](blk3_wdata6::R) reader structure"]
impl crate::Readable for BLK3_WDATA6 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata6::W](blk3_wdata6::W) writer structure"]
impl crate::Writable for BLK3_WDATA6 {}
#[doc = "EFUSE_BLK3_WDATA6_REG"]
pub mod blk3_wdata6;
#[doc = "EFUSE_BLK3_WDATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk3_wdata7](blk3_wdata7) module"]
pub type BLK3_WDATA7 = crate::Reg<u32, _BLK3_WDATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK3_WDATA7;
#[doc = "`read()` method returns [blk3_wdata7::R](blk3_wdata7::R) reader structure"]
impl crate::Readable for BLK3_WDATA7 {}
#[doc = "`write(|w| ..)` method takes [blk3_wdata7::W](blk3_wdata7::W) writer structure"]
impl crate::Writable for BLK3_WDATA7 {}
#[doc = "EFUSE_BLK3_WDATA7_REG"]
pub mod blk3_wdata7;
#[doc = "EFUSE_CLK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "EFUSE_CLK_REG"]
pub mod clk;
#[doc = "EFUSE_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "EFUSE_CONF_REG"]
pub mod conf;
#[doc = "EFUSE_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "EFUSE_STATUS_REG"]
pub mod status;
#[doc = "EFUSE_CMD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "EFUSE_CMD_REG"]
pub mod cmd;
#[doc = "EFUSE_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "EFUSE_INT_RAW_REG"]
pub mod int_raw;
#[doc = "EFUSE_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "EFUSE_INT_ST_REG"]
pub mod int_st;
#[doc = "EFUSE_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "EFUSE_INT_ENA_REG"]
pub mod int_ena;
#[doc = "EFUSE_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "EFUSE_INT_CLR_REG"]
pub mod int_clr;
#[doc = "EFUSE_DAC_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac_conf](dac_conf) module"]
pub type DAC_CONF = crate::Reg<u32, _DAC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CONF;
#[doc = "`read()` method returns [dac_conf::R](dac_conf::R) reader structure"]
impl crate::Readable for DAC_CONF {}
#[doc = "`write(|w| ..)` method takes [dac_conf::W](dac_conf::W) writer structure"]
impl crate::Writable for DAC_CONF {}
#[doc = "EFUSE_DAC_CONF_REG"]
pub mod dac_conf;
#[doc = "EFUSE_DEC_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dec_status](dec_status) module"]
pub type DEC_STATUS = crate::Reg<u32, _DEC_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEC_STATUS;
#[doc = "`read()` method returns [dec_status::R](dec_status::R) reader structure"]
impl crate::Readable for DEC_STATUS {}
#[doc = "`write(|w| ..)` method takes [dec_status::W](dec_status::W) writer structure"]
impl crate::Writable for DEC_STATUS {}
#[doc = "EFUSE_DEC_STATUS_REG"]
pub mod dec_status;
#[doc = "EFUSE_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "EFUSE_DATE_REG"]
pub mod date;
