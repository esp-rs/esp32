#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC_HSCH0_CONF0_REG"]
    pub hsch0_conf0: HSCH0_CONF0,
    #[doc = "0x04 - LEDC_HSCH0_HPOINT_REG"]
    pub hsch0_hpoint: HSCH0_HPOINT,
    #[doc = "0x08 - LEDC_HSCH0_DUTY_REG"]
    pub hsch0_duty: HSCH0_DUTY,
    #[doc = "0x0c - LEDC_HSCH0_CONF1_REG"]
    pub hsch0_conf1: HSCH0_CONF1,
    #[doc = "0x10 - LEDC_HSCH0_DUTY_R_REG"]
    pub hsch0_duty_r: HSCH0_DUTY_R,
    #[doc = "0x14 - LEDC_HSCH1_CONF0_REG"]
    pub hsch1_conf0: HSCH1_CONF0,
    #[doc = "0x18 - LEDC_HSCH1_HPOINT_REG"]
    pub hsch1_hpoint: HSCH1_HPOINT,
    #[doc = "0x1c - LEDC_HSCH1_DUTY_REG"]
    pub hsch1_duty: HSCH1_DUTY,
    #[doc = "0x20 - LEDC_HSCH1_CONF1_REG"]
    pub hsch1_conf1: HSCH1_CONF1,
    #[doc = "0x24 - LEDC_HSCH1_DUTY_R_REG"]
    pub hsch1_duty_r: HSCH1_DUTY_R,
    #[doc = "0x28 - LEDC_HSCH2_CONF0_REG"]
    pub hsch2_conf0: HSCH2_CONF0,
    #[doc = "0x2c - LEDC_HSCH2_HPOINT_REG"]
    pub hsch2_hpoint: HSCH2_HPOINT,
    #[doc = "0x30 - LEDC_HSCH2_DUTY_REG"]
    pub hsch2_duty: HSCH2_DUTY,
    #[doc = "0x34 - LEDC_HSCH2_CONF1_REG"]
    pub hsch2_conf1: HSCH2_CONF1,
    #[doc = "0x38 - LEDC_HSCH2_DUTY_R_REG"]
    pub hsch2_duty_r: HSCH2_DUTY_R,
    #[doc = "0x3c - LEDC_HSCH3_CONF0_REG"]
    pub hsch3_conf0: HSCH3_CONF0,
    #[doc = "0x40 - LEDC_HSCH3_HPOINT_REG"]
    pub hsch3_hpoint: HSCH3_HPOINT,
    #[doc = "0x44 - LEDC_HSCH3_DUTY_REG"]
    pub hsch3_duty: HSCH3_DUTY,
    #[doc = "0x48 - LEDC_HSCH3_CONF1_REG"]
    pub hsch3_conf1: HSCH3_CONF1,
    #[doc = "0x4c - LEDC_HSCH3_DUTY_R_REG"]
    pub hsch3_duty_r: HSCH3_DUTY_R,
    #[doc = "0x50 - LEDC_HSCH4_CONF0_REG"]
    pub hsch4_conf0: HSCH4_CONF0,
    #[doc = "0x54 - LEDC_HSCH4_HPOINT_REG"]
    pub hsch4_hpoint: HSCH4_HPOINT,
    #[doc = "0x58 - LEDC_HSCH4_DUTY_REG"]
    pub hsch4_duty: HSCH4_DUTY,
    #[doc = "0x5c - LEDC_HSCH4_CONF1_REG"]
    pub hsch4_conf1: HSCH4_CONF1,
    #[doc = "0x60 - LEDC_HSCH4_DUTY_R_REG"]
    pub hsch4_duty_r: HSCH4_DUTY_R,
    #[doc = "0x64 - LEDC_HSCH5_CONF0_REG"]
    pub hsch5_conf0: HSCH5_CONF0,
    #[doc = "0x68 - LEDC_HSCH5_HPOINT_REG"]
    pub hsch5_hpoint: HSCH5_HPOINT,
    #[doc = "0x6c - LEDC_HSCH5_DUTY_REG"]
    pub hsch5_duty: HSCH5_DUTY,
    #[doc = "0x70 - LEDC_HSCH5_CONF1_REG"]
    pub hsch5_conf1: HSCH5_CONF1,
    #[doc = "0x74 - LEDC_HSCH5_DUTY_R_REG"]
    pub hsch5_duty_r: HSCH5_DUTY_R,
    #[doc = "0x78 - LEDC_HSCH6_CONF0_REG"]
    pub hsch6_conf0: HSCH6_CONF0,
    #[doc = "0x7c - LEDC_HSCH6_HPOINT_REG"]
    pub hsch6_hpoint: HSCH6_HPOINT,
    #[doc = "0x80 - LEDC_HSCH6_DUTY_REG"]
    pub hsch6_duty: HSCH6_DUTY,
    #[doc = "0x84 - LEDC_HSCH6_CONF1_REG"]
    pub hsch6_conf1: HSCH6_CONF1,
    #[doc = "0x88 - LEDC_HSCH6_DUTY_R_REG"]
    pub hsch6_duty_r: HSCH6_DUTY_R,
    #[doc = "0x8c - LEDC_HSCH7_CONF0_REG"]
    pub hsch7_conf0: HSCH7_CONF0,
    #[doc = "0x90 - LEDC_HSCH7_HPOINT_REG"]
    pub hsch7_hpoint: HSCH7_HPOINT,
    #[doc = "0x94 - LEDC_HSCH7_DUTY_REG"]
    pub hsch7_duty: HSCH7_DUTY,
    #[doc = "0x98 - LEDC_HSCH7_CONF1_REG"]
    pub hsch7_conf1: HSCH7_CONF1,
    #[doc = "0x9c - LEDC_HSCH7_DUTY_R_REG"]
    pub hsch7_duty_r: HSCH7_DUTY_R,
    #[doc = "0xa0 - LEDC_LSCH0_CONF0_REG"]
    pub lsch0_conf0: LSCH0_CONF0,
    #[doc = "0xa4 - LEDC_LSCH0_HPOINT_REG"]
    pub lsch0_hpoint: LSCH0_HPOINT,
    #[doc = "0xa8 - LEDC_LSCH0_DUTY_REG"]
    pub lsch0_duty: LSCH0_DUTY,
    #[doc = "0xac - LEDC_LSCH0_CONF1_REG"]
    pub lsch0_conf1: LSCH0_CONF1,
    #[doc = "0xb0 - LEDC_LSCH0_DUTY_R_REG"]
    pub lsch0_duty_r: LSCH0_DUTY_R,
    #[doc = "0xb4 - LEDC_LSCH1_CONF0_REG"]
    pub lsch1_conf0: LSCH1_CONF0,
    #[doc = "0xb8 - LEDC_LSCH1_HPOINT_REG"]
    pub lsch1_hpoint: LSCH1_HPOINT,
    #[doc = "0xbc - LEDC_LSCH1_DUTY_REG"]
    pub lsch1_duty: LSCH1_DUTY,
    #[doc = "0xc0 - LEDC_LSCH1_CONF1_REG"]
    pub lsch1_conf1: LSCH1_CONF1,
    #[doc = "0xc4 - LEDC_LSCH1_DUTY_R_REG"]
    pub lsch1_duty_r: LSCH1_DUTY_R,
    #[doc = "0xc8 - LEDC_LSCH2_CONF0_REG"]
    pub lsch2_conf0: LSCH2_CONF0,
    #[doc = "0xcc - LEDC_LSCH2_HPOINT_REG"]
    pub lsch2_hpoint: LSCH2_HPOINT,
    #[doc = "0xd0 - LEDC_LSCH2_DUTY_REG"]
    pub lsch2_duty: LSCH2_DUTY,
    #[doc = "0xd4 - LEDC_LSCH2_CONF1_REG"]
    pub lsch2_conf1: LSCH2_CONF1,
    #[doc = "0xd8 - LEDC_LSCH2_DUTY_R_REG"]
    pub lsch2_duty_r: LSCH2_DUTY_R,
    #[doc = "0xdc - LEDC_LSCH3_CONF0_REG"]
    pub lsch3_conf0: LSCH3_CONF0,
    #[doc = "0xe0 - LEDC_LSCH3_HPOINT_REG"]
    pub lsch3_hpoint: LSCH3_HPOINT,
    #[doc = "0xe4 - LEDC_LSCH3_DUTY_REG"]
    pub lsch3_duty: LSCH3_DUTY,
    #[doc = "0xe8 - LEDC_LSCH3_CONF1_REG"]
    pub lsch3_conf1: LSCH3_CONF1,
    #[doc = "0xec - LEDC_LSCH3_DUTY_R_REG"]
    pub lsch3_duty_r: LSCH3_DUTY_R,
    #[doc = "0xf0 - LEDC_LSCH4_CONF0_REG"]
    pub lsch4_conf0: LSCH4_CONF0,
    #[doc = "0xf4 - LEDC_LSCH4_HPOINT_REG"]
    pub lsch4_hpoint: LSCH4_HPOINT,
    #[doc = "0xf8 - LEDC_LSCH4_DUTY_REG"]
    pub lsch4_duty: LSCH4_DUTY,
    #[doc = "0xfc - LEDC_LSCH4_CONF1_REG"]
    pub lsch4_conf1: LSCH4_CONF1,
    #[doc = "0x100 - LEDC_LSCH4_DUTY_R_REG"]
    pub lsch4_duty_r: LSCH4_DUTY_R,
    #[doc = "0x104 - LEDC_LSCH5_CONF0_REG"]
    pub lsch5_conf0: LSCH5_CONF0,
    #[doc = "0x108 - LEDC_LSCH5_HPOINT_REG"]
    pub lsch5_hpoint: LSCH5_HPOINT,
    #[doc = "0x10c - LEDC_LSCH5_DUTY_REG"]
    pub lsch5_duty: LSCH5_DUTY,
    #[doc = "0x110 - LEDC_LSCH5_CONF1_REG"]
    pub lsch5_conf1: LSCH5_CONF1,
    #[doc = "0x114 - LEDC_LSCH5_DUTY_R_REG"]
    pub lsch5_duty_r: LSCH5_DUTY_R,
    #[doc = "0x118 - LEDC_LSCH6_CONF0_REG"]
    pub lsch6_conf0: LSCH6_CONF0,
    #[doc = "0x11c - LEDC_LSCH6_HPOINT_REG"]
    pub lsch6_hpoint: LSCH6_HPOINT,
    #[doc = "0x120 - LEDC_LSCH6_DUTY_REG"]
    pub lsch6_duty: LSCH6_DUTY,
    #[doc = "0x124 - LEDC_LSCH6_CONF1_REG"]
    pub lsch6_conf1: LSCH6_CONF1,
    #[doc = "0x128 - LEDC_LSCH6_DUTY_R_REG"]
    pub lsch6_duty_r: LSCH6_DUTY_R,
    #[doc = "0x12c - LEDC_LSCH7_CONF0_REG"]
    pub lsch7_conf0: LSCH7_CONF0,
    #[doc = "0x130 - LEDC_LSCH7_HPOINT_REG"]
    pub lsch7_hpoint: LSCH7_HPOINT,
    #[doc = "0x134 - LEDC_LSCH7_DUTY_REG"]
    pub lsch7_duty: LSCH7_DUTY,
    #[doc = "0x138 - LEDC_LSCH7_CONF1_REG"]
    pub lsch7_conf1: LSCH7_CONF1,
    #[doc = "0x13c - LEDC_LSCH7_DUTY_R_REG"]
    pub lsch7_duty_r: LSCH7_DUTY_R,
    #[doc = "0x140 - LEDC_HSTIMER0_CONF_REG"]
    pub hstimer0_conf: HSTIMER0_CONF,
    #[doc = "0x144 - LEDC_HSTIMER0_VALUE_REG"]
    pub hstimer0_value: HSTIMER0_VALUE,
    #[doc = "0x148 - LEDC_HSTIMER1_CONF_REG"]
    pub hstimer1_conf: HSTIMER1_CONF,
    #[doc = "0x14c - LEDC_HSTIMER1_VALUE_REG"]
    pub hstimer1_value: HSTIMER1_VALUE,
    #[doc = "0x150 - LEDC_HSTIMER2_CONF_REG"]
    pub hstimer2_conf: HSTIMER2_CONF,
    #[doc = "0x154 - LEDC_HSTIMER2_VALUE_REG"]
    pub hstimer2_value: HSTIMER2_VALUE,
    #[doc = "0x158 - LEDC_HSTIMER3_CONF_REG"]
    pub hstimer3_conf: HSTIMER3_CONF,
    #[doc = "0x15c - LEDC_HSTIMER3_VALUE_REG"]
    pub hstimer3_value: HSTIMER3_VALUE,
    #[doc = "0x160 - LEDC_LSTIMER0_CONF_REG"]
    pub lstimer0_conf: LSTIMER0_CONF,
    #[doc = "0x164 - LEDC_LSTIMER0_VALUE_REG"]
    pub lstimer0_value: LSTIMER0_VALUE,
    #[doc = "0x168 - LEDC_LSTIMER1_CONF_REG"]
    pub lstimer1_conf: LSTIMER1_CONF,
    #[doc = "0x16c - LEDC_LSTIMER1_VALUE_REG"]
    pub lstimer1_value: LSTIMER1_VALUE,
    #[doc = "0x170 - LEDC_LSTIMER2_CONF_REG"]
    pub lstimer2_conf: LSTIMER2_CONF,
    #[doc = "0x174 - LEDC_LSTIMER2_VALUE_REG"]
    pub lstimer2_value: LSTIMER2_VALUE,
    #[doc = "0x178 - LEDC_LSTIMER3_CONF_REG"]
    pub lstimer3_conf: LSTIMER3_CONF,
    #[doc = "0x17c - LEDC_LSTIMER3_VALUE_REG"]
    pub lstimer3_value: LSTIMER3_VALUE,
    #[doc = "0x180 - LEDC_INT_RAW_REG"]
    pub int_raw: INT_RAW,
    #[doc = "0x184 - LEDC_INT_ST_REG"]
    pub int_st: INT_ST,
    #[doc = "0x188 - LEDC_INT_ENA_REG"]
    pub int_ena: INT_ENA,
    #[doc = "0x18c - LEDC_INT_CLR_REG"]
    pub int_clr: INT_CLR,
    #[doc = "0x190 - LEDC_CONF_REG"]
    pub conf: CONF,
    _reserved101: [u8; 104usize],
    #[doc = "0x1fc - LEDC_DATE_REG"]
    pub date: DATE,
}
#[doc = "LEDC_HSCH0_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch0_conf0](hsch0_conf0) module"]
pub type HSCH0_CONF0 = crate::Reg<u32, _HSCH0_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH0_CONF0;
#[doc = "`read()` method returns [hsch0_conf0::R](hsch0_conf0::R) reader structure"]
impl crate::Readable for HSCH0_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch0_conf0::W](hsch0_conf0::W) writer structure"]
impl crate::Writable for HSCH0_CONF0 {}
#[doc = "LEDC_HSCH0_CONF0_REG"]
pub mod hsch0_conf0;
#[doc = "LEDC_HSCH0_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch0_hpoint](hsch0_hpoint) module"]
pub type HSCH0_HPOINT = crate::Reg<u32, _HSCH0_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH0_HPOINT;
#[doc = "`read()` method returns [hsch0_hpoint::R](hsch0_hpoint::R) reader structure"]
impl crate::Readable for HSCH0_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch0_hpoint::W](hsch0_hpoint::W) writer structure"]
impl crate::Writable for HSCH0_HPOINT {}
#[doc = "LEDC_HSCH0_HPOINT_REG"]
pub mod hsch0_hpoint;
#[doc = "LEDC_HSCH0_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch0_duty](hsch0_duty) module"]
pub type HSCH0_DUTY = crate::Reg<u32, _HSCH0_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH0_DUTY;
#[doc = "`read()` method returns [hsch0_duty::R](hsch0_duty::R) reader structure"]
impl crate::Readable for HSCH0_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch0_duty::W](hsch0_duty::W) writer structure"]
impl crate::Writable for HSCH0_DUTY {}
#[doc = "LEDC_HSCH0_DUTY_REG"]
pub mod hsch0_duty;
#[doc = "LEDC_HSCH0_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch0_conf1](hsch0_conf1) module"]
pub type HSCH0_CONF1 = crate::Reg<u32, _HSCH0_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH0_CONF1;
#[doc = "`read()` method returns [hsch0_conf1::R](hsch0_conf1::R) reader structure"]
impl crate::Readable for HSCH0_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch0_conf1::W](hsch0_conf1::W) writer structure"]
impl crate::Writable for HSCH0_CONF1 {}
#[doc = "LEDC_HSCH0_CONF1_REG"]
pub mod hsch0_conf1;
#[doc = "LEDC_HSCH0_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch0_duty_r](hsch0_duty_r) module"]
pub type HSCH0_DUTY_R = crate::Reg<u32, _HSCH0_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH0_DUTY_R;
#[doc = "`read()` method returns [hsch0_duty_r::R](hsch0_duty_r::R) reader structure"]
impl crate::Readable for HSCH0_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch0_duty_r::W](hsch0_duty_r::W) writer structure"]
impl crate::Writable for HSCH0_DUTY_R {}
#[doc = "LEDC_HSCH0_DUTY_R_REG"]
pub mod hsch0_duty_r;
#[doc = "LEDC_HSCH1_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch1_conf0](hsch1_conf0) module"]
pub type HSCH1_CONF0 = crate::Reg<u32, _HSCH1_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH1_CONF0;
#[doc = "`read()` method returns [hsch1_conf0::R](hsch1_conf0::R) reader structure"]
impl crate::Readable for HSCH1_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch1_conf0::W](hsch1_conf0::W) writer structure"]
impl crate::Writable for HSCH1_CONF0 {}
#[doc = "LEDC_HSCH1_CONF0_REG"]
pub mod hsch1_conf0;
#[doc = "LEDC_HSCH1_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch1_hpoint](hsch1_hpoint) module"]
pub type HSCH1_HPOINT = crate::Reg<u32, _HSCH1_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH1_HPOINT;
#[doc = "`read()` method returns [hsch1_hpoint::R](hsch1_hpoint::R) reader structure"]
impl crate::Readable for HSCH1_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch1_hpoint::W](hsch1_hpoint::W) writer structure"]
impl crate::Writable for HSCH1_HPOINT {}
#[doc = "LEDC_HSCH1_HPOINT_REG"]
pub mod hsch1_hpoint;
#[doc = "LEDC_HSCH1_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch1_duty](hsch1_duty) module"]
pub type HSCH1_DUTY = crate::Reg<u32, _HSCH1_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH1_DUTY;
#[doc = "`read()` method returns [hsch1_duty::R](hsch1_duty::R) reader structure"]
impl crate::Readable for HSCH1_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch1_duty::W](hsch1_duty::W) writer structure"]
impl crate::Writable for HSCH1_DUTY {}
#[doc = "LEDC_HSCH1_DUTY_REG"]
pub mod hsch1_duty;
#[doc = "LEDC_HSCH1_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch1_conf1](hsch1_conf1) module"]
pub type HSCH1_CONF1 = crate::Reg<u32, _HSCH1_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH1_CONF1;
#[doc = "`read()` method returns [hsch1_conf1::R](hsch1_conf1::R) reader structure"]
impl crate::Readable for HSCH1_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch1_conf1::W](hsch1_conf1::W) writer structure"]
impl crate::Writable for HSCH1_CONF1 {}
#[doc = "LEDC_HSCH1_CONF1_REG"]
pub mod hsch1_conf1;
#[doc = "LEDC_HSCH1_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch1_duty_r](hsch1_duty_r) module"]
pub type HSCH1_DUTY_R = crate::Reg<u32, _HSCH1_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH1_DUTY_R;
#[doc = "`read()` method returns [hsch1_duty_r::R](hsch1_duty_r::R) reader structure"]
impl crate::Readable for HSCH1_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch1_duty_r::W](hsch1_duty_r::W) writer structure"]
impl crate::Writable for HSCH1_DUTY_R {}
#[doc = "LEDC_HSCH1_DUTY_R_REG"]
pub mod hsch1_duty_r;
#[doc = "LEDC_HSCH2_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch2_conf0](hsch2_conf0) module"]
pub type HSCH2_CONF0 = crate::Reg<u32, _HSCH2_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH2_CONF0;
#[doc = "`read()` method returns [hsch2_conf0::R](hsch2_conf0::R) reader structure"]
impl crate::Readable for HSCH2_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch2_conf0::W](hsch2_conf0::W) writer structure"]
impl crate::Writable for HSCH2_CONF0 {}
#[doc = "LEDC_HSCH2_CONF0_REG"]
pub mod hsch2_conf0;
#[doc = "LEDC_HSCH2_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch2_hpoint](hsch2_hpoint) module"]
pub type HSCH2_HPOINT = crate::Reg<u32, _HSCH2_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH2_HPOINT;
#[doc = "`read()` method returns [hsch2_hpoint::R](hsch2_hpoint::R) reader structure"]
impl crate::Readable for HSCH2_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch2_hpoint::W](hsch2_hpoint::W) writer structure"]
impl crate::Writable for HSCH2_HPOINT {}
#[doc = "LEDC_HSCH2_HPOINT_REG"]
pub mod hsch2_hpoint;
#[doc = "LEDC_HSCH2_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch2_duty](hsch2_duty) module"]
pub type HSCH2_DUTY = crate::Reg<u32, _HSCH2_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH2_DUTY;
#[doc = "`read()` method returns [hsch2_duty::R](hsch2_duty::R) reader structure"]
impl crate::Readable for HSCH2_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch2_duty::W](hsch2_duty::W) writer structure"]
impl crate::Writable for HSCH2_DUTY {}
#[doc = "LEDC_HSCH2_DUTY_REG"]
pub mod hsch2_duty;
#[doc = "LEDC_HSCH2_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch2_conf1](hsch2_conf1) module"]
pub type HSCH2_CONF1 = crate::Reg<u32, _HSCH2_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH2_CONF1;
#[doc = "`read()` method returns [hsch2_conf1::R](hsch2_conf1::R) reader structure"]
impl crate::Readable for HSCH2_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch2_conf1::W](hsch2_conf1::W) writer structure"]
impl crate::Writable for HSCH2_CONF1 {}
#[doc = "LEDC_HSCH2_CONF1_REG"]
pub mod hsch2_conf1;
#[doc = "LEDC_HSCH2_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch2_duty_r](hsch2_duty_r) module"]
pub type HSCH2_DUTY_R = crate::Reg<u32, _HSCH2_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH2_DUTY_R;
#[doc = "`read()` method returns [hsch2_duty_r::R](hsch2_duty_r::R) reader structure"]
impl crate::Readable for HSCH2_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch2_duty_r::W](hsch2_duty_r::W) writer structure"]
impl crate::Writable for HSCH2_DUTY_R {}
#[doc = "LEDC_HSCH2_DUTY_R_REG"]
pub mod hsch2_duty_r;
#[doc = "LEDC_HSCH3_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch3_conf0](hsch3_conf0) module"]
pub type HSCH3_CONF0 = crate::Reg<u32, _HSCH3_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH3_CONF0;
#[doc = "`read()` method returns [hsch3_conf0::R](hsch3_conf0::R) reader structure"]
impl crate::Readable for HSCH3_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch3_conf0::W](hsch3_conf0::W) writer structure"]
impl crate::Writable for HSCH3_CONF0 {}
#[doc = "LEDC_HSCH3_CONF0_REG"]
pub mod hsch3_conf0;
#[doc = "LEDC_HSCH3_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch3_hpoint](hsch3_hpoint) module"]
pub type HSCH3_HPOINT = crate::Reg<u32, _HSCH3_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH3_HPOINT;
#[doc = "`read()` method returns [hsch3_hpoint::R](hsch3_hpoint::R) reader structure"]
impl crate::Readable for HSCH3_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch3_hpoint::W](hsch3_hpoint::W) writer structure"]
impl crate::Writable for HSCH3_HPOINT {}
#[doc = "LEDC_HSCH3_HPOINT_REG"]
pub mod hsch3_hpoint;
#[doc = "LEDC_HSCH3_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch3_duty](hsch3_duty) module"]
pub type HSCH3_DUTY = crate::Reg<u32, _HSCH3_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH3_DUTY;
#[doc = "`read()` method returns [hsch3_duty::R](hsch3_duty::R) reader structure"]
impl crate::Readable for HSCH3_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch3_duty::W](hsch3_duty::W) writer structure"]
impl crate::Writable for HSCH3_DUTY {}
#[doc = "LEDC_HSCH3_DUTY_REG"]
pub mod hsch3_duty;
#[doc = "LEDC_HSCH3_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch3_conf1](hsch3_conf1) module"]
pub type HSCH3_CONF1 = crate::Reg<u32, _HSCH3_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH3_CONF1;
#[doc = "`read()` method returns [hsch3_conf1::R](hsch3_conf1::R) reader structure"]
impl crate::Readable for HSCH3_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch3_conf1::W](hsch3_conf1::W) writer structure"]
impl crate::Writable for HSCH3_CONF1 {}
#[doc = "LEDC_HSCH3_CONF1_REG"]
pub mod hsch3_conf1;
#[doc = "LEDC_HSCH3_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch3_duty_r](hsch3_duty_r) module"]
pub type HSCH3_DUTY_R = crate::Reg<u32, _HSCH3_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH3_DUTY_R;
#[doc = "`read()` method returns [hsch3_duty_r::R](hsch3_duty_r::R) reader structure"]
impl crate::Readable for HSCH3_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch3_duty_r::W](hsch3_duty_r::W) writer structure"]
impl crate::Writable for HSCH3_DUTY_R {}
#[doc = "LEDC_HSCH3_DUTY_R_REG"]
pub mod hsch3_duty_r;
#[doc = "LEDC_HSCH4_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch4_conf0](hsch4_conf0) module"]
pub type HSCH4_CONF0 = crate::Reg<u32, _HSCH4_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH4_CONF0;
#[doc = "`read()` method returns [hsch4_conf0::R](hsch4_conf0::R) reader structure"]
impl crate::Readable for HSCH4_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch4_conf0::W](hsch4_conf0::W) writer structure"]
impl crate::Writable for HSCH4_CONF0 {}
#[doc = "LEDC_HSCH4_CONF0_REG"]
pub mod hsch4_conf0;
#[doc = "LEDC_HSCH4_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch4_hpoint](hsch4_hpoint) module"]
pub type HSCH4_HPOINT = crate::Reg<u32, _HSCH4_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH4_HPOINT;
#[doc = "`read()` method returns [hsch4_hpoint::R](hsch4_hpoint::R) reader structure"]
impl crate::Readable for HSCH4_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch4_hpoint::W](hsch4_hpoint::W) writer structure"]
impl crate::Writable for HSCH4_HPOINT {}
#[doc = "LEDC_HSCH4_HPOINT_REG"]
pub mod hsch4_hpoint;
#[doc = "LEDC_HSCH4_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch4_duty](hsch4_duty) module"]
pub type HSCH4_DUTY = crate::Reg<u32, _HSCH4_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH4_DUTY;
#[doc = "`read()` method returns [hsch4_duty::R](hsch4_duty::R) reader structure"]
impl crate::Readable for HSCH4_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch4_duty::W](hsch4_duty::W) writer structure"]
impl crate::Writable for HSCH4_DUTY {}
#[doc = "LEDC_HSCH4_DUTY_REG"]
pub mod hsch4_duty;
#[doc = "LEDC_HSCH4_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch4_conf1](hsch4_conf1) module"]
pub type HSCH4_CONF1 = crate::Reg<u32, _HSCH4_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH4_CONF1;
#[doc = "`read()` method returns [hsch4_conf1::R](hsch4_conf1::R) reader structure"]
impl crate::Readable for HSCH4_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch4_conf1::W](hsch4_conf1::W) writer structure"]
impl crate::Writable for HSCH4_CONF1 {}
#[doc = "LEDC_HSCH4_CONF1_REG"]
pub mod hsch4_conf1;
#[doc = "LEDC_HSCH4_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch4_duty_r](hsch4_duty_r) module"]
pub type HSCH4_DUTY_R = crate::Reg<u32, _HSCH4_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH4_DUTY_R;
#[doc = "`read()` method returns [hsch4_duty_r::R](hsch4_duty_r::R) reader structure"]
impl crate::Readable for HSCH4_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch4_duty_r::W](hsch4_duty_r::W) writer structure"]
impl crate::Writable for HSCH4_DUTY_R {}
#[doc = "LEDC_HSCH4_DUTY_R_REG"]
pub mod hsch4_duty_r;
#[doc = "LEDC_HSCH5_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch5_conf0](hsch5_conf0) module"]
pub type HSCH5_CONF0 = crate::Reg<u32, _HSCH5_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH5_CONF0;
#[doc = "`read()` method returns [hsch5_conf0::R](hsch5_conf0::R) reader structure"]
impl crate::Readable for HSCH5_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch5_conf0::W](hsch5_conf0::W) writer structure"]
impl crate::Writable for HSCH5_CONF0 {}
#[doc = "LEDC_HSCH5_CONF0_REG"]
pub mod hsch5_conf0;
#[doc = "LEDC_HSCH5_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch5_hpoint](hsch5_hpoint) module"]
pub type HSCH5_HPOINT = crate::Reg<u32, _HSCH5_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH5_HPOINT;
#[doc = "`read()` method returns [hsch5_hpoint::R](hsch5_hpoint::R) reader structure"]
impl crate::Readable for HSCH5_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch5_hpoint::W](hsch5_hpoint::W) writer structure"]
impl crate::Writable for HSCH5_HPOINT {}
#[doc = "LEDC_HSCH5_HPOINT_REG"]
pub mod hsch5_hpoint;
#[doc = "LEDC_HSCH5_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch5_duty](hsch5_duty) module"]
pub type HSCH5_DUTY = crate::Reg<u32, _HSCH5_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH5_DUTY;
#[doc = "`read()` method returns [hsch5_duty::R](hsch5_duty::R) reader structure"]
impl crate::Readable for HSCH5_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch5_duty::W](hsch5_duty::W) writer structure"]
impl crate::Writable for HSCH5_DUTY {}
#[doc = "LEDC_HSCH5_DUTY_REG"]
pub mod hsch5_duty;
#[doc = "LEDC_HSCH5_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch5_conf1](hsch5_conf1) module"]
pub type HSCH5_CONF1 = crate::Reg<u32, _HSCH5_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH5_CONF1;
#[doc = "`read()` method returns [hsch5_conf1::R](hsch5_conf1::R) reader structure"]
impl crate::Readable for HSCH5_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch5_conf1::W](hsch5_conf1::W) writer structure"]
impl crate::Writable for HSCH5_CONF1 {}
#[doc = "LEDC_HSCH5_CONF1_REG"]
pub mod hsch5_conf1;
#[doc = "LEDC_HSCH5_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch5_duty_r](hsch5_duty_r) module"]
pub type HSCH5_DUTY_R = crate::Reg<u32, _HSCH5_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH5_DUTY_R;
#[doc = "`read()` method returns [hsch5_duty_r::R](hsch5_duty_r::R) reader structure"]
impl crate::Readable for HSCH5_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch5_duty_r::W](hsch5_duty_r::W) writer structure"]
impl crate::Writable for HSCH5_DUTY_R {}
#[doc = "LEDC_HSCH5_DUTY_R_REG"]
pub mod hsch5_duty_r;
#[doc = "LEDC_HSCH6_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch6_conf0](hsch6_conf0) module"]
pub type HSCH6_CONF0 = crate::Reg<u32, _HSCH6_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH6_CONF0;
#[doc = "`read()` method returns [hsch6_conf0::R](hsch6_conf0::R) reader structure"]
impl crate::Readable for HSCH6_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch6_conf0::W](hsch6_conf0::W) writer structure"]
impl crate::Writable for HSCH6_CONF0 {}
#[doc = "LEDC_HSCH6_CONF0_REG"]
pub mod hsch6_conf0;
#[doc = "LEDC_HSCH6_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch6_hpoint](hsch6_hpoint) module"]
pub type HSCH6_HPOINT = crate::Reg<u32, _HSCH6_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH6_HPOINT;
#[doc = "`read()` method returns [hsch6_hpoint::R](hsch6_hpoint::R) reader structure"]
impl crate::Readable for HSCH6_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch6_hpoint::W](hsch6_hpoint::W) writer structure"]
impl crate::Writable for HSCH6_HPOINT {}
#[doc = "LEDC_HSCH6_HPOINT_REG"]
pub mod hsch6_hpoint;
#[doc = "LEDC_HSCH6_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch6_duty](hsch6_duty) module"]
pub type HSCH6_DUTY = crate::Reg<u32, _HSCH6_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH6_DUTY;
#[doc = "`read()` method returns [hsch6_duty::R](hsch6_duty::R) reader structure"]
impl crate::Readable for HSCH6_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch6_duty::W](hsch6_duty::W) writer structure"]
impl crate::Writable for HSCH6_DUTY {}
#[doc = "LEDC_HSCH6_DUTY_REG"]
pub mod hsch6_duty;
#[doc = "LEDC_HSCH6_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch6_conf1](hsch6_conf1) module"]
pub type HSCH6_CONF1 = crate::Reg<u32, _HSCH6_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH6_CONF1;
#[doc = "`read()` method returns [hsch6_conf1::R](hsch6_conf1::R) reader structure"]
impl crate::Readable for HSCH6_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch6_conf1::W](hsch6_conf1::W) writer structure"]
impl crate::Writable for HSCH6_CONF1 {}
#[doc = "LEDC_HSCH6_CONF1_REG"]
pub mod hsch6_conf1;
#[doc = "LEDC_HSCH6_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch6_duty_r](hsch6_duty_r) module"]
pub type HSCH6_DUTY_R = crate::Reg<u32, _HSCH6_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH6_DUTY_R;
#[doc = "`read()` method returns [hsch6_duty_r::R](hsch6_duty_r::R) reader structure"]
impl crate::Readable for HSCH6_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch6_duty_r::W](hsch6_duty_r::W) writer structure"]
impl crate::Writable for HSCH6_DUTY_R {}
#[doc = "LEDC_HSCH6_DUTY_R_REG"]
pub mod hsch6_duty_r;
#[doc = "LEDC_HSCH7_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch7_conf0](hsch7_conf0) module"]
pub type HSCH7_CONF0 = crate::Reg<u32, _HSCH7_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH7_CONF0;
#[doc = "`read()` method returns [hsch7_conf0::R](hsch7_conf0::R) reader structure"]
impl crate::Readable for HSCH7_CONF0 {}
#[doc = "`write(|w| ..)` method takes [hsch7_conf0::W](hsch7_conf0::W) writer structure"]
impl crate::Writable for HSCH7_CONF0 {}
#[doc = "LEDC_HSCH7_CONF0_REG"]
pub mod hsch7_conf0;
#[doc = "LEDC_HSCH7_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch7_hpoint](hsch7_hpoint) module"]
pub type HSCH7_HPOINT = crate::Reg<u32, _HSCH7_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH7_HPOINT;
#[doc = "`read()` method returns [hsch7_hpoint::R](hsch7_hpoint::R) reader structure"]
impl crate::Readable for HSCH7_HPOINT {}
#[doc = "`write(|w| ..)` method takes [hsch7_hpoint::W](hsch7_hpoint::W) writer structure"]
impl crate::Writable for HSCH7_HPOINT {}
#[doc = "LEDC_HSCH7_HPOINT_REG"]
pub mod hsch7_hpoint;
#[doc = "LEDC_HSCH7_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch7_duty](hsch7_duty) module"]
pub type HSCH7_DUTY = crate::Reg<u32, _HSCH7_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH7_DUTY;
#[doc = "`read()` method returns [hsch7_duty::R](hsch7_duty::R) reader structure"]
impl crate::Readable for HSCH7_DUTY {}
#[doc = "`write(|w| ..)` method takes [hsch7_duty::W](hsch7_duty::W) writer structure"]
impl crate::Writable for HSCH7_DUTY {}
#[doc = "LEDC_HSCH7_DUTY_REG"]
pub mod hsch7_duty;
#[doc = "LEDC_HSCH7_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch7_conf1](hsch7_conf1) module"]
pub type HSCH7_CONF1 = crate::Reg<u32, _HSCH7_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH7_CONF1;
#[doc = "`read()` method returns [hsch7_conf1::R](hsch7_conf1::R) reader structure"]
impl crate::Readable for HSCH7_CONF1 {}
#[doc = "`write(|w| ..)` method takes [hsch7_conf1::W](hsch7_conf1::W) writer structure"]
impl crate::Writable for HSCH7_CONF1 {}
#[doc = "LEDC_HSCH7_CONF1_REG"]
pub mod hsch7_conf1;
#[doc = "LEDC_HSCH7_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsch7_duty_r](hsch7_duty_r) module"]
pub type HSCH7_DUTY_R = crate::Reg<u32, _HSCH7_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCH7_DUTY_R;
#[doc = "`read()` method returns [hsch7_duty_r::R](hsch7_duty_r::R) reader structure"]
impl crate::Readable for HSCH7_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [hsch7_duty_r::W](hsch7_duty_r::W) writer structure"]
impl crate::Writable for HSCH7_DUTY_R {}
#[doc = "LEDC_HSCH7_DUTY_R_REG"]
pub mod hsch7_duty_r;
#[doc = "LEDC_LSCH0_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch0_conf0](lsch0_conf0) module"]
pub type LSCH0_CONF0 = crate::Reg<u32, _LSCH0_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_CONF0;
#[doc = "`read()` method returns [lsch0_conf0::R](lsch0_conf0::R) reader structure"]
impl crate::Readable for LSCH0_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch0_conf0::W](lsch0_conf0::W) writer structure"]
impl crate::Writable for LSCH0_CONF0 {}
#[doc = "LEDC_LSCH0_CONF0_REG"]
pub mod lsch0_conf0;
#[doc = "LEDC_LSCH0_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch0_hpoint](lsch0_hpoint) module"]
pub type LSCH0_HPOINT = crate::Reg<u32, _LSCH0_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_HPOINT;
#[doc = "`read()` method returns [lsch0_hpoint::R](lsch0_hpoint::R) reader structure"]
impl crate::Readable for LSCH0_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch0_hpoint::W](lsch0_hpoint::W) writer structure"]
impl crate::Writable for LSCH0_HPOINT {}
#[doc = "LEDC_LSCH0_HPOINT_REG"]
pub mod lsch0_hpoint;
#[doc = "LEDC_LSCH0_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch0_duty](lsch0_duty) module"]
pub type LSCH0_DUTY = crate::Reg<u32, _LSCH0_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_DUTY;
#[doc = "`read()` method returns [lsch0_duty::R](lsch0_duty::R) reader structure"]
impl crate::Readable for LSCH0_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch0_duty::W](lsch0_duty::W) writer structure"]
impl crate::Writable for LSCH0_DUTY {}
#[doc = "LEDC_LSCH0_DUTY_REG"]
pub mod lsch0_duty;
#[doc = "LEDC_LSCH0_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch0_conf1](lsch0_conf1) module"]
pub type LSCH0_CONF1 = crate::Reg<u32, _LSCH0_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_CONF1;
#[doc = "`read()` method returns [lsch0_conf1::R](lsch0_conf1::R) reader structure"]
impl crate::Readable for LSCH0_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch0_conf1::W](lsch0_conf1::W) writer structure"]
impl crate::Writable for LSCH0_CONF1 {}
#[doc = "LEDC_LSCH0_CONF1_REG"]
pub mod lsch0_conf1;
#[doc = "LEDC_LSCH0_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch0_duty_r](lsch0_duty_r) module"]
pub type LSCH0_DUTY_R = crate::Reg<u32, _LSCH0_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_DUTY_R;
#[doc = "`read()` method returns [lsch0_duty_r::R](lsch0_duty_r::R) reader structure"]
impl crate::Readable for LSCH0_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch0_duty_r::W](lsch0_duty_r::W) writer structure"]
impl crate::Writable for LSCH0_DUTY_R {}
#[doc = "LEDC_LSCH0_DUTY_R_REG"]
pub mod lsch0_duty_r;
#[doc = "LEDC_LSCH1_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch1_conf0](lsch1_conf0) module"]
pub type LSCH1_CONF0 = crate::Reg<u32, _LSCH1_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_CONF0;
#[doc = "`read()` method returns [lsch1_conf0::R](lsch1_conf0::R) reader structure"]
impl crate::Readable for LSCH1_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch1_conf0::W](lsch1_conf0::W) writer structure"]
impl crate::Writable for LSCH1_CONF0 {}
#[doc = "LEDC_LSCH1_CONF0_REG"]
pub mod lsch1_conf0;
#[doc = "LEDC_LSCH1_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch1_hpoint](lsch1_hpoint) module"]
pub type LSCH1_HPOINT = crate::Reg<u32, _LSCH1_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_HPOINT;
#[doc = "`read()` method returns [lsch1_hpoint::R](lsch1_hpoint::R) reader structure"]
impl crate::Readable for LSCH1_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch1_hpoint::W](lsch1_hpoint::W) writer structure"]
impl crate::Writable for LSCH1_HPOINT {}
#[doc = "LEDC_LSCH1_HPOINT_REG"]
pub mod lsch1_hpoint;
#[doc = "LEDC_LSCH1_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch1_duty](lsch1_duty) module"]
pub type LSCH1_DUTY = crate::Reg<u32, _LSCH1_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_DUTY;
#[doc = "`read()` method returns [lsch1_duty::R](lsch1_duty::R) reader structure"]
impl crate::Readable for LSCH1_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch1_duty::W](lsch1_duty::W) writer structure"]
impl crate::Writable for LSCH1_DUTY {}
#[doc = "LEDC_LSCH1_DUTY_REG"]
pub mod lsch1_duty;
#[doc = "LEDC_LSCH1_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch1_conf1](lsch1_conf1) module"]
pub type LSCH1_CONF1 = crate::Reg<u32, _LSCH1_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_CONF1;
#[doc = "`read()` method returns [lsch1_conf1::R](lsch1_conf1::R) reader structure"]
impl crate::Readable for LSCH1_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch1_conf1::W](lsch1_conf1::W) writer structure"]
impl crate::Writable for LSCH1_CONF1 {}
#[doc = "LEDC_LSCH1_CONF1_REG"]
pub mod lsch1_conf1;
#[doc = "LEDC_LSCH1_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch1_duty_r](lsch1_duty_r) module"]
pub type LSCH1_DUTY_R = crate::Reg<u32, _LSCH1_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_DUTY_R;
#[doc = "`read()` method returns [lsch1_duty_r::R](lsch1_duty_r::R) reader structure"]
impl crate::Readable for LSCH1_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch1_duty_r::W](lsch1_duty_r::W) writer structure"]
impl crate::Writable for LSCH1_DUTY_R {}
#[doc = "LEDC_LSCH1_DUTY_R_REG"]
pub mod lsch1_duty_r;
#[doc = "LEDC_LSCH2_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch2_conf0](lsch2_conf0) module"]
pub type LSCH2_CONF0 = crate::Reg<u32, _LSCH2_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_CONF0;
#[doc = "`read()` method returns [lsch2_conf0::R](lsch2_conf0::R) reader structure"]
impl crate::Readable for LSCH2_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch2_conf0::W](lsch2_conf0::W) writer structure"]
impl crate::Writable for LSCH2_CONF0 {}
#[doc = "LEDC_LSCH2_CONF0_REG"]
pub mod lsch2_conf0;
#[doc = "LEDC_LSCH2_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch2_hpoint](lsch2_hpoint) module"]
pub type LSCH2_HPOINT = crate::Reg<u32, _LSCH2_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_HPOINT;
#[doc = "`read()` method returns [lsch2_hpoint::R](lsch2_hpoint::R) reader structure"]
impl crate::Readable for LSCH2_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch2_hpoint::W](lsch2_hpoint::W) writer structure"]
impl crate::Writable for LSCH2_HPOINT {}
#[doc = "LEDC_LSCH2_HPOINT_REG"]
pub mod lsch2_hpoint;
#[doc = "LEDC_LSCH2_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch2_duty](lsch2_duty) module"]
pub type LSCH2_DUTY = crate::Reg<u32, _LSCH2_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_DUTY;
#[doc = "`read()` method returns [lsch2_duty::R](lsch2_duty::R) reader structure"]
impl crate::Readable for LSCH2_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch2_duty::W](lsch2_duty::W) writer structure"]
impl crate::Writable for LSCH2_DUTY {}
#[doc = "LEDC_LSCH2_DUTY_REG"]
pub mod lsch2_duty;
#[doc = "LEDC_LSCH2_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch2_conf1](lsch2_conf1) module"]
pub type LSCH2_CONF1 = crate::Reg<u32, _LSCH2_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_CONF1;
#[doc = "`read()` method returns [lsch2_conf1::R](lsch2_conf1::R) reader structure"]
impl crate::Readable for LSCH2_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch2_conf1::W](lsch2_conf1::W) writer structure"]
impl crate::Writable for LSCH2_CONF1 {}
#[doc = "LEDC_LSCH2_CONF1_REG"]
pub mod lsch2_conf1;
#[doc = "LEDC_LSCH2_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch2_duty_r](lsch2_duty_r) module"]
pub type LSCH2_DUTY_R = crate::Reg<u32, _LSCH2_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_DUTY_R;
#[doc = "`read()` method returns [lsch2_duty_r::R](lsch2_duty_r::R) reader structure"]
impl crate::Readable for LSCH2_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch2_duty_r::W](lsch2_duty_r::W) writer structure"]
impl crate::Writable for LSCH2_DUTY_R {}
#[doc = "LEDC_LSCH2_DUTY_R_REG"]
pub mod lsch2_duty_r;
#[doc = "LEDC_LSCH3_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch3_conf0](lsch3_conf0) module"]
pub type LSCH3_CONF0 = crate::Reg<u32, _LSCH3_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_CONF0;
#[doc = "`read()` method returns [lsch3_conf0::R](lsch3_conf0::R) reader structure"]
impl crate::Readable for LSCH3_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch3_conf0::W](lsch3_conf0::W) writer structure"]
impl crate::Writable for LSCH3_CONF0 {}
#[doc = "LEDC_LSCH3_CONF0_REG"]
pub mod lsch3_conf0;
#[doc = "LEDC_LSCH3_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch3_hpoint](lsch3_hpoint) module"]
pub type LSCH3_HPOINT = crate::Reg<u32, _LSCH3_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_HPOINT;
#[doc = "`read()` method returns [lsch3_hpoint::R](lsch3_hpoint::R) reader structure"]
impl crate::Readable for LSCH3_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch3_hpoint::W](lsch3_hpoint::W) writer structure"]
impl crate::Writable for LSCH3_HPOINT {}
#[doc = "LEDC_LSCH3_HPOINT_REG"]
pub mod lsch3_hpoint;
#[doc = "LEDC_LSCH3_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch3_duty](lsch3_duty) module"]
pub type LSCH3_DUTY = crate::Reg<u32, _LSCH3_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_DUTY;
#[doc = "`read()` method returns [lsch3_duty::R](lsch3_duty::R) reader structure"]
impl crate::Readable for LSCH3_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch3_duty::W](lsch3_duty::W) writer structure"]
impl crate::Writable for LSCH3_DUTY {}
#[doc = "LEDC_LSCH3_DUTY_REG"]
pub mod lsch3_duty;
#[doc = "LEDC_LSCH3_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch3_conf1](lsch3_conf1) module"]
pub type LSCH3_CONF1 = crate::Reg<u32, _LSCH3_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_CONF1;
#[doc = "`read()` method returns [lsch3_conf1::R](lsch3_conf1::R) reader structure"]
impl crate::Readable for LSCH3_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch3_conf1::W](lsch3_conf1::W) writer structure"]
impl crate::Writable for LSCH3_CONF1 {}
#[doc = "LEDC_LSCH3_CONF1_REG"]
pub mod lsch3_conf1;
#[doc = "LEDC_LSCH3_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch3_duty_r](lsch3_duty_r) module"]
pub type LSCH3_DUTY_R = crate::Reg<u32, _LSCH3_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_DUTY_R;
#[doc = "`read()` method returns [lsch3_duty_r::R](lsch3_duty_r::R) reader structure"]
impl crate::Readable for LSCH3_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch3_duty_r::W](lsch3_duty_r::W) writer structure"]
impl crate::Writable for LSCH3_DUTY_R {}
#[doc = "LEDC_LSCH3_DUTY_R_REG"]
pub mod lsch3_duty_r;
#[doc = "LEDC_LSCH4_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch4_conf0](lsch4_conf0) module"]
pub type LSCH4_CONF0 = crate::Reg<u32, _LSCH4_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_CONF0;
#[doc = "`read()` method returns [lsch4_conf0::R](lsch4_conf0::R) reader structure"]
impl crate::Readable for LSCH4_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch4_conf0::W](lsch4_conf0::W) writer structure"]
impl crate::Writable for LSCH4_CONF0 {}
#[doc = "LEDC_LSCH4_CONF0_REG"]
pub mod lsch4_conf0;
#[doc = "LEDC_LSCH4_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch4_hpoint](lsch4_hpoint) module"]
pub type LSCH4_HPOINT = crate::Reg<u32, _LSCH4_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_HPOINT;
#[doc = "`read()` method returns [lsch4_hpoint::R](lsch4_hpoint::R) reader structure"]
impl crate::Readable for LSCH4_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch4_hpoint::W](lsch4_hpoint::W) writer structure"]
impl crate::Writable for LSCH4_HPOINT {}
#[doc = "LEDC_LSCH4_HPOINT_REG"]
pub mod lsch4_hpoint;
#[doc = "LEDC_LSCH4_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch4_duty](lsch4_duty) module"]
pub type LSCH4_DUTY = crate::Reg<u32, _LSCH4_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_DUTY;
#[doc = "`read()` method returns [lsch4_duty::R](lsch4_duty::R) reader structure"]
impl crate::Readable for LSCH4_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch4_duty::W](lsch4_duty::W) writer structure"]
impl crate::Writable for LSCH4_DUTY {}
#[doc = "LEDC_LSCH4_DUTY_REG"]
pub mod lsch4_duty;
#[doc = "LEDC_LSCH4_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch4_conf1](lsch4_conf1) module"]
pub type LSCH4_CONF1 = crate::Reg<u32, _LSCH4_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_CONF1;
#[doc = "`read()` method returns [lsch4_conf1::R](lsch4_conf1::R) reader structure"]
impl crate::Readable for LSCH4_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch4_conf1::W](lsch4_conf1::W) writer structure"]
impl crate::Writable for LSCH4_CONF1 {}
#[doc = "LEDC_LSCH4_CONF1_REG"]
pub mod lsch4_conf1;
#[doc = "LEDC_LSCH4_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch4_duty_r](lsch4_duty_r) module"]
pub type LSCH4_DUTY_R = crate::Reg<u32, _LSCH4_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_DUTY_R;
#[doc = "`read()` method returns [lsch4_duty_r::R](lsch4_duty_r::R) reader structure"]
impl crate::Readable for LSCH4_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch4_duty_r::W](lsch4_duty_r::W) writer structure"]
impl crate::Writable for LSCH4_DUTY_R {}
#[doc = "LEDC_LSCH4_DUTY_R_REG"]
pub mod lsch4_duty_r;
#[doc = "LEDC_LSCH5_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch5_conf0](lsch5_conf0) module"]
pub type LSCH5_CONF0 = crate::Reg<u32, _LSCH5_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_CONF0;
#[doc = "`read()` method returns [lsch5_conf0::R](lsch5_conf0::R) reader structure"]
impl crate::Readable for LSCH5_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch5_conf0::W](lsch5_conf0::W) writer structure"]
impl crate::Writable for LSCH5_CONF0 {}
#[doc = "LEDC_LSCH5_CONF0_REG"]
pub mod lsch5_conf0;
#[doc = "LEDC_LSCH5_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch5_hpoint](lsch5_hpoint) module"]
pub type LSCH5_HPOINT = crate::Reg<u32, _LSCH5_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_HPOINT;
#[doc = "`read()` method returns [lsch5_hpoint::R](lsch5_hpoint::R) reader structure"]
impl crate::Readable for LSCH5_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch5_hpoint::W](lsch5_hpoint::W) writer structure"]
impl crate::Writable for LSCH5_HPOINT {}
#[doc = "LEDC_LSCH5_HPOINT_REG"]
pub mod lsch5_hpoint;
#[doc = "LEDC_LSCH5_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch5_duty](lsch5_duty) module"]
pub type LSCH5_DUTY = crate::Reg<u32, _LSCH5_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_DUTY;
#[doc = "`read()` method returns [lsch5_duty::R](lsch5_duty::R) reader structure"]
impl crate::Readable for LSCH5_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch5_duty::W](lsch5_duty::W) writer structure"]
impl crate::Writable for LSCH5_DUTY {}
#[doc = "LEDC_LSCH5_DUTY_REG"]
pub mod lsch5_duty;
#[doc = "LEDC_LSCH5_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch5_conf1](lsch5_conf1) module"]
pub type LSCH5_CONF1 = crate::Reg<u32, _LSCH5_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_CONF1;
#[doc = "`read()` method returns [lsch5_conf1::R](lsch5_conf1::R) reader structure"]
impl crate::Readable for LSCH5_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch5_conf1::W](lsch5_conf1::W) writer structure"]
impl crate::Writable for LSCH5_CONF1 {}
#[doc = "LEDC_LSCH5_CONF1_REG"]
pub mod lsch5_conf1;
#[doc = "LEDC_LSCH5_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch5_duty_r](lsch5_duty_r) module"]
pub type LSCH5_DUTY_R = crate::Reg<u32, _LSCH5_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_DUTY_R;
#[doc = "`read()` method returns [lsch5_duty_r::R](lsch5_duty_r::R) reader structure"]
impl crate::Readable for LSCH5_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch5_duty_r::W](lsch5_duty_r::W) writer structure"]
impl crate::Writable for LSCH5_DUTY_R {}
#[doc = "LEDC_LSCH5_DUTY_R_REG"]
pub mod lsch5_duty_r;
#[doc = "LEDC_LSCH6_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch6_conf0](lsch6_conf0) module"]
pub type LSCH6_CONF0 = crate::Reg<u32, _LSCH6_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH6_CONF0;
#[doc = "`read()` method returns [lsch6_conf0::R](lsch6_conf0::R) reader structure"]
impl crate::Readable for LSCH6_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch6_conf0::W](lsch6_conf0::W) writer structure"]
impl crate::Writable for LSCH6_CONF0 {}
#[doc = "LEDC_LSCH6_CONF0_REG"]
pub mod lsch6_conf0;
#[doc = "LEDC_LSCH6_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch6_hpoint](lsch6_hpoint) module"]
pub type LSCH6_HPOINT = crate::Reg<u32, _LSCH6_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH6_HPOINT;
#[doc = "`read()` method returns [lsch6_hpoint::R](lsch6_hpoint::R) reader structure"]
impl crate::Readable for LSCH6_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch6_hpoint::W](lsch6_hpoint::W) writer structure"]
impl crate::Writable for LSCH6_HPOINT {}
#[doc = "LEDC_LSCH6_HPOINT_REG"]
pub mod lsch6_hpoint;
#[doc = "LEDC_LSCH6_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch6_duty](lsch6_duty) module"]
pub type LSCH6_DUTY = crate::Reg<u32, _LSCH6_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH6_DUTY;
#[doc = "`read()` method returns [lsch6_duty::R](lsch6_duty::R) reader structure"]
impl crate::Readable for LSCH6_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch6_duty::W](lsch6_duty::W) writer structure"]
impl crate::Writable for LSCH6_DUTY {}
#[doc = "LEDC_LSCH6_DUTY_REG"]
pub mod lsch6_duty;
#[doc = "LEDC_LSCH6_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch6_conf1](lsch6_conf1) module"]
pub type LSCH6_CONF1 = crate::Reg<u32, _LSCH6_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH6_CONF1;
#[doc = "`read()` method returns [lsch6_conf1::R](lsch6_conf1::R) reader structure"]
impl crate::Readable for LSCH6_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch6_conf1::W](lsch6_conf1::W) writer structure"]
impl crate::Writable for LSCH6_CONF1 {}
#[doc = "LEDC_LSCH6_CONF1_REG"]
pub mod lsch6_conf1;
#[doc = "LEDC_LSCH6_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch6_duty_r](lsch6_duty_r) module"]
pub type LSCH6_DUTY_R = crate::Reg<u32, _LSCH6_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH6_DUTY_R;
#[doc = "`read()` method returns [lsch6_duty_r::R](lsch6_duty_r::R) reader structure"]
impl crate::Readable for LSCH6_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch6_duty_r::W](lsch6_duty_r::W) writer structure"]
impl crate::Writable for LSCH6_DUTY_R {}
#[doc = "LEDC_LSCH6_DUTY_R_REG"]
pub mod lsch6_duty_r;
#[doc = "LEDC_LSCH7_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch7_conf0](lsch7_conf0) module"]
pub type LSCH7_CONF0 = crate::Reg<u32, _LSCH7_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH7_CONF0;
#[doc = "`read()` method returns [lsch7_conf0::R](lsch7_conf0::R) reader structure"]
impl crate::Readable for LSCH7_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch7_conf0::W](lsch7_conf0::W) writer structure"]
impl crate::Writable for LSCH7_CONF0 {}
#[doc = "LEDC_LSCH7_CONF0_REG"]
pub mod lsch7_conf0;
#[doc = "LEDC_LSCH7_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch7_hpoint](lsch7_hpoint) module"]
pub type LSCH7_HPOINT = crate::Reg<u32, _LSCH7_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH7_HPOINT;
#[doc = "`read()` method returns [lsch7_hpoint::R](lsch7_hpoint::R) reader structure"]
impl crate::Readable for LSCH7_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch7_hpoint::W](lsch7_hpoint::W) writer structure"]
impl crate::Writable for LSCH7_HPOINT {}
#[doc = "LEDC_LSCH7_HPOINT_REG"]
pub mod lsch7_hpoint;
#[doc = "LEDC_LSCH7_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch7_duty](lsch7_duty) module"]
pub type LSCH7_DUTY = crate::Reg<u32, _LSCH7_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH7_DUTY;
#[doc = "`read()` method returns [lsch7_duty::R](lsch7_duty::R) reader structure"]
impl crate::Readable for LSCH7_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch7_duty::W](lsch7_duty::W) writer structure"]
impl crate::Writable for LSCH7_DUTY {}
#[doc = "LEDC_LSCH7_DUTY_REG"]
pub mod lsch7_duty;
#[doc = "LEDC_LSCH7_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch7_conf1](lsch7_conf1) module"]
pub type LSCH7_CONF1 = crate::Reg<u32, _LSCH7_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH7_CONF1;
#[doc = "`read()` method returns [lsch7_conf1::R](lsch7_conf1::R) reader structure"]
impl crate::Readable for LSCH7_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch7_conf1::W](lsch7_conf1::W) writer structure"]
impl crate::Writable for LSCH7_CONF1 {}
#[doc = "LEDC_LSCH7_CONF1_REG"]
pub mod lsch7_conf1;
#[doc = "LEDC_LSCH7_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsch7_duty_r](lsch7_duty_r) module"]
pub type LSCH7_DUTY_R = crate::Reg<u32, _LSCH7_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH7_DUTY_R;
#[doc = "`read()` method returns [lsch7_duty_r::R](lsch7_duty_r::R) reader structure"]
impl crate::Readable for LSCH7_DUTY_R {}
#[doc = "`write(|w| ..)` method takes [lsch7_duty_r::W](lsch7_duty_r::W) writer structure"]
impl crate::Writable for LSCH7_DUTY_R {}
#[doc = "LEDC_LSCH7_DUTY_R_REG"]
pub mod lsch7_duty_r;
#[doc = "LEDC_HSTIMER0_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer0_conf](hstimer0_conf) module"]
pub type HSTIMER0_CONF = crate::Reg<u32, _HSTIMER0_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER0_CONF;
#[doc = "`read()` method returns [hstimer0_conf::R](hstimer0_conf::R) reader structure"]
impl crate::Readable for HSTIMER0_CONF {}
#[doc = "`write(|w| ..)` method takes [hstimer0_conf::W](hstimer0_conf::W) writer structure"]
impl crate::Writable for HSTIMER0_CONF {}
#[doc = "LEDC_HSTIMER0_CONF_REG"]
pub mod hstimer0_conf;
#[doc = "LEDC_HSTIMER0_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer0_value](hstimer0_value) module"]
pub type HSTIMER0_VALUE = crate::Reg<u32, _HSTIMER0_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER0_VALUE;
#[doc = "`read()` method returns [hstimer0_value::R](hstimer0_value::R) reader structure"]
impl crate::Readable for HSTIMER0_VALUE {}
#[doc = "`write(|w| ..)` method takes [hstimer0_value::W](hstimer0_value::W) writer structure"]
impl crate::Writable for HSTIMER0_VALUE {}
#[doc = "LEDC_HSTIMER0_VALUE_REG"]
pub mod hstimer0_value;
#[doc = "LEDC_HSTIMER1_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer1_conf](hstimer1_conf) module"]
pub type HSTIMER1_CONF = crate::Reg<u32, _HSTIMER1_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER1_CONF;
#[doc = "`read()` method returns [hstimer1_conf::R](hstimer1_conf::R) reader structure"]
impl crate::Readable for HSTIMER1_CONF {}
#[doc = "`write(|w| ..)` method takes [hstimer1_conf::W](hstimer1_conf::W) writer structure"]
impl crate::Writable for HSTIMER1_CONF {}
#[doc = "LEDC_HSTIMER1_CONF_REG"]
pub mod hstimer1_conf;
#[doc = "LEDC_HSTIMER1_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer1_value](hstimer1_value) module"]
pub type HSTIMER1_VALUE = crate::Reg<u32, _HSTIMER1_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER1_VALUE;
#[doc = "`read()` method returns [hstimer1_value::R](hstimer1_value::R) reader structure"]
impl crate::Readable for HSTIMER1_VALUE {}
#[doc = "`write(|w| ..)` method takes [hstimer1_value::W](hstimer1_value::W) writer structure"]
impl crate::Writable for HSTIMER1_VALUE {}
#[doc = "LEDC_HSTIMER1_VALUE_REG"]
pub mod hstimer1_value;
#[doc = "LEDC_HSTIMER2_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer2_conf](hstimer2_conf) module"]
pub type HSTIMER2_CONF = crate::Reg<u32, _HSTIMER2_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER2_CONF;
#[doc = "`read()` method returns [hstimer2_conf::R](hstimer2_conf::R) reader structure"]
impl crate::Readable for HSTIMER2_CONF {}
#[doc = "`write(|w| ..)` method takes [hstimer2_conf::W](hstimer2_conf::W) writer structure"]
impl crate::Writable for HSTIMER2_CONF {}
#[doc = "LEDC_HSTIMER2_CONF_REG"]
pub mod hstimer2_conf;
#[doc = "LEDC_HSTIMER2_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer2_value](hstimer2_value) module"]
pub type HSTIMER2_VALUE = crate::Reg<u32, _HSTIMER2_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER2_VALUE;
#[doc = "`read()` method returns [hstimer2_value::R](hstimer2_value::R) reader structure"]
impl crate::Readable for HSTIMER2_VALUE {}
#[doc = "`write(|w| ..)` method takes [hstimer2_value::W](hstimer2_value::W) writer structure"]
impl crate::Writable for HSTIMER2_VALUE {}
#[doc = "LEDC_HSTIMER2_VALUE_REG"]
pub mod hstimer2_value;
#[doc = "LEDC_HSTIMER3_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer3_conf](hstimer3_conf) module"]
pub type HSTIMER3_CONF = crate::Reg<u32, _HSTIMER3_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER3_CONF;
#[doc = "`read()` method returns [hstimer3_conf::R](hstimer3_conf::R) reader structure"]
impl crate::Readable for HSTIMER3_CONF {}
#[doc = "`write(|w| ..)` method takes [hstimer3_conf::W](hstimer3_conf::W) writer structure"]
impl crate::Writable for HSTIMER3_CONF {}
#[doc = "LEDC_HSTIMER3_CONF_REG"]
pub mod hstimer3_conf;
#[doc = "LEDC_HSTIMER3_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimer3_value](hstimer3_value) module"]
pub type HSTIMER3_VALUE = crate::Reg<u32, _HSTIMER3_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMER3_VALUE;
#[doc = "`read()` method returns [hstimer3_value::R](hstimer3_value::R) reader structure"]
impl crate::Readable for HSTIMER3_VALUE {}
#[doc = "`write(|w| ..)` method takes [hstimer3_value::W](hstimer3_value::W) writer structure"]
impl crate::Writable for HSTIMER3_VALUE {}
#[doc = "LEDC_HSTIMER3_VALUE_REG"]
pub mod hstimer3_value;
#[doc = "LEDC_LSTIMER0_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer0_conf](lstimer0_conf) module"]
pub type LSTIMER0_CONF = crate::Reg<u32, _LSTIMER0_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER0_CONF;
#[doc = "`read()` method returns [lstimer0_conf::R](lstimer0_conf::R) reader structure"]
impl crate::Readable for LSTIMER0_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer0_conf::W](lstimer0_conf::W) writer structure"]
impl crate::Writable for LSTIMER0_CONF {}
#[doc = "LEDC_LSTIMER0_CONF_REG"]
pub mod lstimer0_conf;
#[doc = "LEDC_LSTIMER0_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer0_value](lstimer0_value) module"]
pub type LSTIMER0_VALUE = crate::Reg<u32, _LSTIMER0_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER0_VALUE;
#[doc = "`read()` method returns [lstimer0_value::R](lstimer0_value::R) reader structure"]
impl crate::Readable for LSTIMER0_VALUE {}
#[doc = "`write(|w| ..)` method takes [lstimer0_value::W](lstimer0_value::W) writer structure"]
impl crate::Writable for LSTIMER0_VALUE {}
#[doc = "LEDC_LSTIMER0_VALUE_REG"]
pub mod lstimer0_value;
#[doc = "LEDC_LSTIMER1_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer1_conf](lstimer1_conf) module"]
pub type LSTIMER1_CONF = crate::Reg<u32, _LSTIMER1_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER1_CONF;
#[doc = "`read()` method returns [lstimer1_conf::R](lstimer1_conf::R) reader structure"]
impl crate::Readable for LSTIMER1_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer1_conf::W](lstimer1_conf::W) writer structure"]
impl crate::Writable for LSTIMER1_CONF {}
#[doc = "LEDC_LSTIMER1_CONF_REG"]
pub mod lstimer1_conf;
#[doc = "LEDC_LSTIMER1_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer1_value](lstimer1_value) module"]
pub type LSTIMER1_VALUE = crate::Reg<u32, _LSTIMER1_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER1_VALUE;
#[doc = "`read()` method returns [lstimer1_value::R](lstimer1_value::R) reader structure"]
impl crate::Readable for LSTIMER1_VALUE {}
#[doc = "`write(|w| ..)` method takes [lstimer1_value::W](lstimer1_value::W) writer structure"]
impl crate::Writable for LSTIMER1_VALUE {}
#[doc = "LEDC_LSTIMER1_VALUE_REG"]
pub mod lstimer1_value;
#[doc = "LEDC_LSTIMER2_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer2_conf](lstimer2_conf) module"]
pub type LSTIMER2_CONF = crate::Reg<u32, _LSTIMER2_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER2_CONF;
#[doc = "`read()` method returns [lstimer2_conf::R](lstimer2_conf::R) reader structure"]
impl crate::Readable for LSTIMER2_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer2_conf::W](lstimer2_conf::W) writer structure"]
impl crate::Writable for LSTIMER2_CONF {}
#[doc = "LEDC_LSTIMER2_CONF_REG"]
pub mod lstimer2_conf;
#[doc = "LEDC_LSTIMER2_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer2_value](lstimer2_value) module"]
pub type LSTIMER2_VALUE = crate::Reg<u32, _LSTIMER2_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER2_VALUE;
#[doc = "`read()` method returns [lstimer2_value::R](lstimer2_value::R) reader structure"]
impl crate::Readable for LSTIMER2_VALUE {}
#[doc = "`write(|w| ..)` method takes [lstimer2_value::W](lstimer2_value::W) writer structure"]
impl crate::Writable for LSTIMER2_VALUE {}
#[doc = "LEDC_LSTIMER2_VALUE_REG"]
pub mod lstimer2_value;
#[doc = "LEDC_LSTIMER3_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer3_conf](lstimer3_conf) module"]
pub type LSTIMER3_CONF = crate::Reg<u32, _LSTIMER3_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER3_CONF;
#[doc = "`read()` method returns [lstimer3_conf::R](lstimer3_conf::R) reader structure"]
impl crate::Readable for LSTIMER3_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer3_conf::W](lstimer3_conf::W) writer structure"]
impl crate::Writable for LSTIMER3_CONF {}
#[doc = "LEDC_LSTIMER3_CONF_REG"]
pub mod lstimer3_conf;
#[doc = "LEDC_LSTIMER3_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstimer3_value](lstimer3_value) module"]
pub type LSTIMER3_VALUE = crate::Reg<u32, _LSTIMER3_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER3_VALUE;
#[doc = "`read()` method returns [lstimer3_value::R](lstimer3_value::R) reader structure"]
impl crate::Readable for LSTIMER3_VALUE {}
#[doc = "`write(|w| ..)` method takes [lstimer3_value::W](lstimer3_value::W) writer structure"]
impl crate::Writable for LSTIMER3_VALUE {}
#[doc = "LEDC_LSTIMER3_VALUE_REG"]
pub mod lstimer3_value;
#[doc = "LEDC_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "LEDC_INT_RAW_REG"]
pub mod int_raw;
#[doc = "LEDC_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "LEDC_INT_ST_REG"]
pub mod int_st;
#[doc = "LEDC_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "LEDC_INT_ENA_REG"]
pub mod int_ena;
#[doc = "LEDC_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "LEDC_INT_CLR_REG"]
pub mod int_clr;
#[doc = "LEDC_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "LEDC_CONF_REG"]
pub mod conf;
#[doc = "LEDC_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "LEDC_DATE_REG"]
pub mod date;
