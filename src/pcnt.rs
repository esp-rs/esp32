#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCNT_U0_CONF0"]
    pub u0_conf0: U0_CONF0,
    #[doc = "0x04 - PCNT_U0_CONF1"]
    pub u0_conf1: U0_CONF1,
    #[doc = "0x08 - PCNT_U0_CONF2"]
    pub u0_conf2: U0_CONF2,
    #[doc = "0x0c - PCNT_U1_CONF0"]
    pub u1_conf0: U1_CONF0,
    #[doc = "0x10 - PCNT_U1_CONF1"]
    pub u1_conf1: U1_CONF1,
    #[doc = "0x14 - PCNT_U1_CONF2"]
    pub u1_conf2: U1_CONF2,
    #[doc = "0x18 - PCNT_U2_CONF0"]
    pub u2_conf0: U2_CONF0,
    #[doc = "0x1c - PCNT_U2_CONF1"]
    pub u2_conf1: U2_CONF1,
    #[doc = "0x20 - PCNT_U2_CONF2"]
    pub u2_conf2: U2_CONF2,
    #[doc = "0x24 - PCNT_U3_CONF0"]
    pub u3_conf0: U3_CONF0,
    #[doc = "0x28 - PCNT_U3_CONF1"]
    pub u3_conf1: U3_CONF1,
    #[doc = "0x2c - PCNT_U3_CONF2"]
    pub u3_conf2: U3_CONF2,
    #[doc = "0x30 - PCNT_U4_CONF0"]
    pub u4_conf0: U4_CONF0,
    #[doc = "0x34 - PCNT_U4_CONF1"]
    pub u4_conf1: U4_CONF1,
    #[doc = "0x38 - PCNT_U4_CONF2"]
    pub u4_conf2: U4_CONF2,
    #[doc = "0x3c - PCNT_U5_CONF0"]
    pub u5_conf0: U5_CONF0,
    #[doc = "0x40 - PCNT_U5_CONF1"]
    pub u5_conf1: U5_CONF1,
    #[doc = "0x44 - PCNT_U5_CONF2"]
    pub u5_conf2: U5_CONF2,
    #[doc = "0x48 - PCNT_U6_CONF0"]
    pub u6_conf0: U6_CONF0,
    #[doc = "0x4c - PCNT_U6_CONF1"]
    pub u6_conf1: U6_CONF1,
    #[doc = "0x50 - PCNT_U6_CONF2"]
    pub u6_conf2: U6_CONF2,
    #[doc = "0x54 - PCNT_U7_CONF0"]
    pub u7_conf0: U7_CONF0,
    #[doc = "0x58 - PCNT_U7_CONF1"]
    pub u7_conf1: U7_CONF1,
    #[doc = "0x5c - PCNT_U7_CONF2"]
    pub u7_conf2: U7_CONF2,
    #[doc = "0x60 - PCNT_U0_CNT"]
    pub u0_cnt: U0_CNT,
    #[doc = "0x64 - PCNT_U1_CNT"]
    pub u1_cnt: U1_CNT,
    #[doc = "0x68 - PCNT_U2_CNT"]
    pub u2_cnt: U2_CNT,
    #[doc = "0x6c - PCNT_U3_CNT"]
    pub u3_cnt: U3_CNT,
    #[doc = "0x70 - PCNT_U4_CNT"]
    pub u4_cnt: U4_CNT,
    #[doc = "0x74 - PCNT_U5_CNT"]
    pub u5_cnt: U5_CNT,
    #[doc = "0x78 - PCNT_U6_CNT"]
    pub u6_cnt: U6_CNT,
    #[doc = "0x7c - PCNT_U7_CNT"]
    pub u7_cnt: U7_CNT,
    #[doc = "0x80 - PCNT_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x84 - PCNT_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x88 - PCNT_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x8c - PCNT_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x90 - PCNT_U0_STATUS"]
    pub u0_status: U0_STATUS,
    #[doc = "0x94 - PCNT_U1_STATUS"]
    pub u1_status: U1_STATUS,
    #[doc = "0x98 - PCNT_U2_STATUS"]
    pub u2_status: U2_STATUS,
    #[doc = "0x9c - PCNT_U3_STATUS"]
    pub u3_status: U3_STATUS,
    #[doc = "0xa0 - PCNT_U4_STATUS"]
    pub u4_status: U4_STATUS,
    #[doc = "0xa4 - PCNT_U5_STATUS"]
    pub u5_status: U5_STATUS,
    #[doc = "0xa8 - PCNT_U6_STATUS"]
    pub u6_status: U6_STATUS,
    #[doc = "0xac - PCNT_U7_STATUS"]
    pub u7_status: U7_STATUS,
    #[doc = "0xb0 - PCNT_CTRL"]
    pub ctrl: CTRL,
    _reserved45: [u8; 72usize],
    #[doc = "0xfc - PCNT_DATE"]
    pub date: DATE,
}
#[doc = "PCNT_U0_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u0_conf0](u0_conf0) module"]
pub type U0_CONF0 = crate::Reg<u32, _U0_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0_CONF0;
#[doc = "`read()` method returns [u0_conf0::R](u0_conf0::R) reader structure"]
impl crate::Readable for U0_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u0_conf0::W](u0_conf0::W) writer structure"]
impl crate::Writable for U0_CONF0 {}
#[doc = "PCNT_U0_CONF0"]
pub mod u0_conf0;
#[doc = "PCNT_U0_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u0_conf1](u0_conf1) module"]
pub type U0_CONF1 = crate::Reg<u32, _U0_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0_CONF1;
#[doc = "`read()` method returns [u0_conf1::R](u0_conf1::R) reader structure"]
impl crate::Readable for U0_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u0_conf1::W](u0_conf1::W) writer structure"]
impl crate::Writable for U0_CONF1 {}
#[doc = "PCNT_U0_CONF1"]
pub mod u0_conf1;
#[doc = "PCNT_U0_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u0_conf2](u0_conf2) module"]
pub type U0_CONF2 = crate::Reg<u32, _U0_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0_CONF2;
#[doc = "`read()` method returns [u0_conf2::R](u0_conf2::R) reader structure"]
impl crate::Readable for U0_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u0_conf2::W](u0_conf2::W) writer structure"]
impl crate::Writable for U0_CONF2 {}
#[doc = "PCNT_U0_CONF2"]
pub mod u0_conf2;
#[doc = "PCNT_U1_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u1_conf0](u1_conf0) module"]
pub type U1_CONF0 = crate::Reg<u32, _U1_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1_CONF0;
#[doc = "`read()` method returns [u1_conf0::R](u1_conf0::R) reader structure"]
impl crate::Readable for U1_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u1_conf0::W](u1_conf0::W) writer structure"]
impl crate::Writable for U1_CONF0 {}
#[doc = "PCNT_U1_CONF0"]
pub mod u1_conf0;
#[doc = "PCNT_U1_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u1_conf1](u1_conf1) module"]
pub type U1_CONF1 = crate::Reg<u32, _U1_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1_CONF1;
#[doc = "`read()` method returns [u1_conf1::R](u1_conf1::R) reader structure"]
impl crate::Readable for U1_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u1_conf1::W](u1_conf1::W) writer structure"]
impl crate::Writable for U1_CONF1 {}
#[doc = "PCNT_U1_CONF1"]
pub mod u1_conf1;
#[doc = "PCNT_U1_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u1_conf2](u1_conf2) module"]
pub type U1_CONF2 = crate::Reg<u32, _U1_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1_CONF2;
#[doc = "`read()` method returns [u1_conf2::R](u1_conf2::R) reader structure"]
impl crate::Readable for U1_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u1_conf2::W](u1_conf2::W) writer structure"]
impl crate::Writable for U1_CONF2 {}
#[doc = "PCNT_U1_CONF2"]
pub mod u1_conf2;
#[doc = "PCNT_U2_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u2_conf0](u2_conf0) module"]
pub type U2_CONF0 = crate::Reg<u32, _U2_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2_CONF0;
#[doc = "`read()` method returns [u2_conf0::R](u2_conf0::R) reader structure"]
impl crate::Readable for U2_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u2_conf0::W](u2_conf0::W) writer structure"]
impl crate::Writable for U2_CONF0 {}
#[doc = "PCNT_U2_CONF0"]
pub mod u2_conf0;
#[doc = "PCNT_U2_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u2_conf1](u2_conf1) module"]
pub type U2_CONF1 = crate::Reg<u32, _U2_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2_CONF1;
#[doc = "`read()` method returns [u2_conf1::R](u2_conf1::R) reader structure"]
impl crate::Readable for U2_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u2_conf1::W](u2_conf1::W) writer structure"]
impl crate::Writable for U2_CONF1 {}
#[doc = "PCNT_U2_CONF1"]
pub mod u2_conf1;
#[doc = "PCNT_U2_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u2_conf2](u2_conf2) module"]
pub type U2_CONF2 = crate::Reg<u32, _U2_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2_CONF2;
#[doc = "`read()` method returns [u2_conf2::R](u2_conf2::R) reader structure"]
impl crate::Readable for U2_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u2_conf2::W](u2_conf2::W) writer structure"]
impl crate::Writable for U2_CONF2 {}
#[doc = "PCNT_U2_CONF2"]
pub mod u2_conf2;
#[doc = "PCNT_U3_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u3_conf0](u3_conf0) module"]
pub type U3_CONF0 = crate::Reg<u32, _U3_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U3_CONF0;
#[doc = "`read()` method returns [u3_conf0::R](u3_conf0::R) reader structure"]
impl crate::Readable for U3_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u3_conf0::W](u3_conf0::W) writer structure"]
impl crate::Writable for U3_CONF0 {}
#[doc = "PCNT_U3_CONF0"]
pub mod u3_conf0;
#[doc = "PCNT_U3_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u3_conf1](u3_conf1) module"]
pub type U3_CONF1 = crate::Reg<u32, _U3_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U3_CONF1;
#[doc = "`read()` method returns [u3_conf1::R](u3_conf1::R) reader structure"]
impl crate::Readable for U3_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u3_conf1::W](u3_conf1::W) writer structure"]
impl crate::Writable for U3_CONF1 {}
#[doc = "PCNT_U3_CONF1"]
pub mod u3_conf1;
#[doc = "PCNT_U3_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u3_conf2](u3_conf2) module"]
pub type U3_CONF2 = crate::Reg<u32, _U3_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U3_CONF2;
#[doc = "`read()` method returns [u3_conf2::R](u3_conf2::R) reader structure"]
impl crate::Readable for U3_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u3_conf2::W](u3_conf2::W) writer structure"]
impl crate::Writable for U3_CONF2 {}
#[doc = "PCNT_U3_CONF2"]
pub mod u3_conf2;
#[doc = "PCNT_U4_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u4_conf0](u4_conf0) module"]
pub type U4_CONF0 = crate::Reg<u32, _U4_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U4_CONF0;
#[doc = "`read()` method returns [u4_conf0::R](u4_conf0::R) reader structure"]
impl crate::Readable for U4_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u4_conf0::W](u4_conf0::W) writer structure"]
impl crate::Writable for U4_CONF0 {}
#[doc = "PCNT_U4_CONF0"]
pub mod u4_conf0;
#[doc = "PCNT_U4_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u4_conf1](u4_conf1) module"]
pub type U4_CONF1 = crate::Reg<u32, _U4_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U4_CONF1;
#[doc = "`read()` method returns [u4_conf1::R](u4_conf1::R) reader structure"]
impl crate::Readable for U4_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u4_conf1::W](u4_conf1::W) writer structure"]
impl crate::Writable for U4_CONF1 {}
#[doc = "PCNT_U4_CONF1"]
pub mod u4_conf1;
#[doc = "PCNT_U4_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u4_conf2](u4_conf2) module"]
pub type U4_CONF2 = crate::Reg<u32, _U4_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U4_CONF2;
#[doc = "`read()` method returns [u4_conf2::R](u4_conf2::R) reader structure"]
impl crate::Readable for U4_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u4_conf2::W](u4_conf2::W) writer structure"]
impl crate::Writable for U4_CONF2 {}
#[doc = "PCNT_U4_CONF2"]
pub mod u4_conf2;
#[doc = "PCNT_U5_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u5_conf0](u5_conf0) module"]
pub type U5_CONF0 = crate::Reg<u32, _U5_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U5_CONF0;
#[doc = "`read()` method returns [u5_conf0::R](u5_conf0::R) reader structure"]
impl crate::Readable for U5_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u5_conf0::W](u5_conf0::W) writer structure"]
impl crate::Writable for U5_CONF0 {}
#[doc = "PCNT_U5_CONF0"]
pub mod u5_conf0;
#[doc = "PCNT_U5_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u5_conf1](u5_conf1) module"]
pub type U5_CONF1 = crate::Reg<u32, _U5_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U5_CONF1;
#[doc = "`read()` method returns [u5_conf1::R](u5_conf1::R) reader structure"]
impl crate::Readable for U5_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u5_conf1::W](u5_conf1::W) writer structure"]
impl crate::Writable for U5_CONF1 {}
#[doc = "PCNT_U5_CONF1"]
pub mod u5_conf1;
#[doc = "PCNT_U5_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u5_conf2](u5_conf2) module"]
pub type U5_CONF2 = crate::Reg<u32, _U5_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U5_CONF2;
#[doc = "`read()` method returns [u5_conf2::R](u5_conf2::R) reader structure"]
impl crate::Readable for U5_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u5_conf2::W](u5_conf2::W) writer structure"]
impl crate::Writable for U5_CONF2 {}
#[doc = "PCNT_U5_CONF2"]
pub mod u5_conf2;
#[doc = "PCNT_U6_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u6_conf0](u6_conf0) module"]
pub type U6_CONF0 = crate::Reg<u32, _U6_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U6_CONF0;
#[doc = "`read()` method returns [u6_conf0::R](u6_conf0::R) reader structure"]
impl crate::Readable for U6_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u6_conf0::W](u6_conf0::W) writer structure"]
impl crate::Writable for U6_CONF0 {}
#[doc = "PCNT_U6_CONF0"]
pub mod u6_conf0;
#[doc = "PCNT_U6_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u6_conf1](u6_conf1) module"]
pub type U6_CONF1 = crate::Reg<u32, _U6_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U6_CONF1;
#[doc = "`read()` method returns [u6_conf1::R](u6_conf1::R) reader structure"]
impl crate::Readable for U6_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u6_conf1::W](u6_conf1::W) writer structure"]
impl crate::Writable for U6_CONF1 {}
#[doc = "PCNT_U6_CONF1"]
pub mod u6_conf1;
#[doc = "PCNT_U6_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u6_conf2](u6_conf2) module"]
pub type U6_CONF2 = crate::Reg<u32, _U6_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U6_CONF2;
#[doc = "`read()` method returns [u6_conf2::R](u6_conf2::R) reader structure"]
impl crate::Readable for U6_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u6_conf2::W](u6_conf2::W) writer structure"]
impl crate::Writable for U6_CONF2 {}
#[doc = "PCNT_U6_CONF2"]
pub mod u6_conf2;
#[doc = "PCNT_U7_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u7_conf0](u7_conf0) module"]
pub type U7_CONF0 = crate::Reg<u32, _U7_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U7_CONF0;
#[doc = "`read()` method returns [u7_conf0::R](u7_conf0::R) reader structure"]
impl crate::Readable for U7_CONF0 {}
#[doc = "`write(|w| ..)` method takes [u7_conf0::W](u7_conf0::W) writer structure"]
impl crate::Writable for U7_CONF0 {}
#[doc = "PCNT_U7_CONF0"]
pub mod u7_conf0;
#[doc = "PCNT_U7_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u7_conf1](u7_conf1) module"]
pub type U7_CONF1 = crate::Reg<u32, _U7_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U7_CONF1;
#[doc = "`read()` method returns [u7_conf1::R](u7_conf1::R) reader structure"]
impl crate::Readable for U7_CONF1 {}
#[doc = "`write(|w| ..)` method takes [u7_conf1::W](u7_conf1::W) writer structure"]
impl crate::Writable for U7_CONF1 {}
#[doc = "PCNT_U7_CONF1"]
pub mod u7_conf1;
#[doc = "PCNT_U7_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u7_conf2](u7_conf2) module"]
pub type U7_CONF2 = crate::Reg<u32, _U7_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U7_CONF2;
#[doc = "`read()` method returns [u7_conf2::R](u7_conf2::R) reader structure"]
impl crate::Readable for U7_CONF2 {}
#[doc = "`write(|w| ..)` method takes [u7_conf2::W](u7_conf2::W) writer structure"]
impl crate::Writable for U7_CONF2 {}
#[doc = "PCNT_U7_CONF2"]
pub mod u7_conf2;
#[doc = "PCNT_U0_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u0_cnt](u0_cnt) module"]
pub type U0_CNT = crate::Reg<u32, _U0_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0_CNT;
#[doc = "`read()` method returns [u0_cnt::R](u0_cnt::R) reader structure"]
impl crate::Readable for U0_CNT {}
#[doc = "`write(|w| ..)` method takes [u0_cnt::W](u0_cnt::W) writer structure"]
impl crate::Writable for U0_CNT {}
#[doc = "PCNT_U0_CNT"]
pub mod u0_cnt;
#[doc = "PCNT_U1_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u1_cnt](u1_cnt) module"]
pub type U1_CNT = crate::Reg<u32, _U1_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1_CNT;
#[doc = "`read()` method returns [u1_cnt::R](u1_cnt::R) reader structure"]
impl crate::Readable for U1_CNT {}
#[doc = "`write(|w| ..)` method takes [u1_cnt::W](u1_cnt::W) writer structure"]
impl crate::Writable for U1_CNT {}
#[doc = "PCNT_U1_CNT"]
pub mod u1_cnt;
#[doc = "PCNT_U2_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u2_cnt](u2_cnt) module"]
pub type U2_CNT = crate::Reg<u32, _U2_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2_CNT;
#[doc = "`read()` method returns [u2_cnt::R](u2_cnt::R) reader structure"]
impl crate::Readable for U2_CNT {}
#[doc = "`write(|w| ..)` method takes [u2_cnt::W](u2_cnt::W) writer structure"]
impl crate::Writable for U2_CNT {}
#[doc = "PCNT_U2_CNT"]
pub mod u2_cnt;
#[doc = "PCNT_U3_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u3_cnt](u3_cnt) module"]
pub type U3_CNT = crate::Reg<u32, _U3_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U3_CNT;
#[doc = "`read()` method returns [u3_cnt::R](u3_cnt::R) reader structure"]
impl crate::Readable for U3_CNT {}
#[doc = "`write(|w| ..)` method takes [u3_cnt::W](u3_cnt::W) writer structure"]
impl crate::Writable for U3_CNT {}
#[doc = "PCNT_U3_CNT"]
pub mod u3_cnt;
#[doc = "PCNT_U4_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u4_cnt](u4_cnt) module"]
pub type U4_CNT = crate::Reg<u32, _U4_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U4_CNT;
#[doc = "`read()` method returns [u4_cnt::R](u4_cnt::R) reader structure"]
impl crate::Readable for U4_CNT {}
#[doc = "`write(|w| ..)` method takes [u4_cnt::W](u4_cnt::W) writer structure"]
impl crate::Writable for U4_CNT {}
#[doc = "PCNT_U4_CNT"]
pub mod u4_cnt;
#[doc = "PCNT_U5_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u5_cnt](u5_cnt) module"]
pub type U5_CNT = crate::Reg<u32, _U5_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U5_CNT;
#[doc = "`read()` method returns [u5_cnt::R](u5_cnt::R) reader structure"]
impl crate::Readable for U5_CNT {}
#[doc = "`write(|w| ..)` method takes [u5_cnt::W](u5_cnt::W) writer structure"]
impl crate::Writable for U5_CNT {}
#[doc = "PCNT_U5_CNT"]
pub mod u5_cnt;
#[doc = "PCNT_U6_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u6_cnt](u6_cnt) module"]
pub type U6_CNT = crate::Reg<u32, _U6_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U6_CNT;
#[doc = "`read()` method returns [u6_cnt::R](u6_cnt::R) reader structure"]
impl crate::Readable for U6_CNT {}
#[doc = "`write(|w| ..)` method takes [u6_cnt::W](u6_cnt::W) writer structure"]
impl crate::Writable for U6_CNT {}
#[doc = "PCNT_U6_CNT"]
pub mod u6_cnt;
#[doc = "PCNT_U7_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u7_cnt](u7_cnt) module"]
pub type U7_CNT = crate::Reg<u32, _U7_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U7_CNT;
#[doc = "`read()` method returns [u7_cnt::R](u7_cnt::R) reader structure"]
impl crate::Readable for U7_CNT {}
#[doc = "`write(|w| ..)` method takes [u7_cnt::W](u7_cnt::W) writer structure"]
impl crate::Writable for U7_CNT {}
#[doc = "PCNT_U7_CNT"]
pub mod u7_cnt;
#[doc = "PCNT_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "PCNT_INT_RAW"]
pub mod int_raw;
#[doc = "PCNT_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "PCNT_INT_ST"]
pub mod int_st;
#[doc = "PCNT_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "PCNT_INT_ENA"]
pub mod int_ena;
#[doc = "PCNT_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "PCNT_INT_CLR"]
pub mod int_clr;
#[doc = "PCNT_U0_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u0_status](u0_status) module"]
pub type U0_STATUS = crate::Reg<u32, _U0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0_STATUS;
#[doc = "`read()` method returns [u0_status::R](u0_status::R) reader structure"]
impl crate::Readable for U0_STATUS {}
#[doc = "`write(|w| ..)` method takes [u0_status::W](u0_status::W) writer structure"]
impl crate::Writable for U0_STATUS {}
#[doc = "PCNT_U0_STATUS"]
pub mod u0_status;
#[doc = "PCNT_U1_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u1_status](u1_status) module"]
pub type U1_STATUS = crate::Reg<u32, _U1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1_STATUS;
#[doc = "`read()` method returns [u1_status::R](u1_status::R) reader structure"]
impl crate::Readable for U1_STATUS {}
#[doc = "`write(|w| ..)` method takes [u1_status::W](u1_status::W) writer structure"]
impl crate::Writable for U1_STATUS {}
#[doc = "PCNT_U1_STATUS"]
pub mod u1_status;
#[doc = "PCNT_U2_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u2_status](u2_status) module"]
pub type U2_STATUS = crate::Reg<u32, _U2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2_STATUS;
#[doc = "`read()` method returns [u2_status::R](u2_status::R) reader structure"]
impl crate::Readable for U2_STATUS {}
#[doc = "`write(|w| ..)` method takes [u2_status::W](u2_status::W) writer structure"]
impl crate::Writable for U2_STATUS {}
#[doc = "PCNT_U2_STATUS"]
pub mod u2_status;
#[doc = "PCNT_U3_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u3_status](u3_status) module"]
pub type U3_STATUS = crate::Reg<u32, _U3_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U3_STATUS;
#[doc = "`read()` method returns [u3_status::R](u3_status::R) reader structure"]
impl crate::Readable for U3_STATUS {}
#[doc = "`write(|w| ..)` method takes [u3_status::W](u3_status::W) writer structure"]
impl crate::Writable for U3_STATUS {}
#[doc = "PCNT_U3_STATUS"]
pub mod u3_status;
#[doc = "PCNT_U4_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u4_status](u4_status) module"]
pub type U4_STATUS = crate::Reg<u32, _U4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U4_STATUS;
#[doc = "`read()` method returns [u4_status::R](u4_status::R) reader structure"]
impl crate::Readable for U4_STATUS {}
#[doc = "`write(|w| ..)` method takes [u4_status::W](u4_status::W) writer structure"]
impl crate::Writable for U4_STATUS {}
#[doc = "PCNT_U4_STATUS"]
pub mod u4_status;
#[doc = "PCNT_U5_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u5_status](u5_status) module"]
pub type U5_STATUS = crate::Reg<u32, _U5_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U5_STATUS;
#[doc = "`read()` method returns [u5_status::R](u5_status::R) reader structure"]
impl crate::Readable for U5_STATUS {}
#[doc = "`write(|w| ..)` method takes [u5_status::W](u5_status::W) writer structure"]
impl crate::Writable for U5_STATUS {}
#[doc = "PCNT_U5_STATUS"]
pub mod u5_status;
#[doc = "PCNT_U6_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u6_status](u6_status) module"]
pub type U6_STATUS = crate::Reg<u32, _U6_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U6_STATUS;
#[doc = "`read()` method returns [u6_status::R](u6_status::R) reader structure"]
impl crate::Readable for U6_STATUS {}
#[doc = "`write(|w| ..)` method takes [u6_status::W](u6_status::W) writer structure"]
impl crate::Writable for U6_STATUS {}
#[doc = "PCNT_U6_STATUS"]
pub mod u6_status;
#[doc = "PCNT_U7_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [u7_status](u7_status) module"]
pub type U7_STATUS = crate::Reg<u32, _U7_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U7_STATUS;
#[doc = "`read()` method returns [u7_status::R](u7_status::R) reader structure"]
impl crate::Readable for U7_STATUS {}
#[doc = "`write(|w| ..)` method takes [u7_status::W](u7_status::W) writer structure"]
impl crate::Writable for U7_STATUS {}
#[doc = "PCNT_U7_STATUS"]
pub mod u7_status;
#[doc = "PCNT_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "PCNT_CTRL"]
pub mod ctrl;
#[doc = "PCNT_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "PCNT_DATE"]
pub mod date;
