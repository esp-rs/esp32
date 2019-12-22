#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HINF_CFG_DATA0_REG"]
    pub hinf_cfg_data0_reg: HINF_CFG_DATA0_REG,
    #[doc = "0x04 - HINF_CFG_DATA1_REG"]
    pub hinf_cfg_data1_reg: HINF_CFG_DATA1_REG,
    _reserved2: [u8; 20usize],
    #[doc = "0x1c - HINF_CFG_DATA7_REG"]
    pub hinf_cfg_data7_reg: HINF_CFG_DATA7_REG,
    #[doc = "0x20 - HINF_CIS_CONF0_REG"]
    pub hinf_cis_conf0_reg: HINF_CIS_CONF0_REG,
    #[doc = "0x24 - HINF_CIS_CONF1_REG"]
    pub hinf_cis_conf1_reg: HINF_CIS_CONF1_REG,
    #[doc = "0x28 - HINF_CIS_CONF2_REG"]
    pub hinf_cis_conf2_reg: HINF_CIS_CONF2_REG,
    #[doc = "0x2c - HINF_CIS_CONF3_REG"]
    pub hinf_cis_conf3_reg: HINF_CIS_CONF3_REG,
    #[doc = "0x30 - HINF_CIS_CONF4_REG"]
    pub hinf_cis_conf4_reg: HINF_CIS_CONF4_REG,
    #[doc = "0x34 - HINF_CIS_CONF5_REG"]
    pub hinf_cis_conf5_reg: HINF_CIS_CONF5_REG,
    #[doc = "0x38 - HINF_CIS_CONF6_REG"]
    pub hinf_cis_conf6_reg: HINF_CIS_CONF6_REG,
    #[doc = "0x3c - HINF_CIS_CONF7_REG"]
    pub hinf_cis_conf7_reg: HINF_CIS_CONF7_REG,
    #[doc = "0x40 - HINF_CFG_DATA16_REG"]
    pub hinf_cfg_data16_reg: HINF_CFG_DATA16_REG,
    _reserved12: [u8; 184usize],
    #[doc = "0xfc - HINF_DATE_REG"]
    pub hinf_date_reg: HINF_DATE_REG,
}
#[doc = "HINF_CFG_DATA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cfg_data0_reg](hinf_cfg_data0_reg) module"]
pub type HINF_CFG_DATA0_REG = crate::Reg<u32, _HINF_CFG_DATA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CFG_DATA0_REG;
#[doc = "`read()` method returns [hinf_cfg_data0_reg::R](hinf_cfg_data0_reg::R) reader structure"]
impl crate::Readable for HINF_CFG_DATA0_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cfg_data0_reg::W](hinf_cfg_data0_reg::W) writer structure"]
impl crate::Writable for HINF_CFG_DATA0_REG {}
#[doc = "HINF_CFG_DATA0_REG"]
pub mod hinf_cfg_data0_reg;
#[doc = "HINF_CFG_DATA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cfg_data1_reg](hinf_cfg_data1_reg) module"]
pub type HINF_CFG_DATA1_REG = crate::Reg<u32, _HINF_CFG_DATA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CFG_DATA1_REG;
#[doc = "`read()` method returns [hinf_cfg_data1_reg::R](hinf_cfg_data1_reg::R) reader structure"]
impl crate::Readable for HINF_CFG_DATA1_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cfg_data1_reg::W](hinf_cfg_data1_reg::W) writer structure"]
impl crate::Writable for HINF_CFG_DATA1_REG {}
#[doc = "HINF_CFG_DATA1_REG"]
pub mod hinf_cfg_data1_reg;
#[doc = "HINF_CFG_DATA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cfg_data7_reg](hinf_cfg_data7_reg) module"]
pub type HINF_CFG_DATA7_REG = crate::Reg<u32, _HINF_CFG_DATA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CFG_DATA7_REG;
#[doc = "`read()` method returns [hinf_cfg_data7_reg::R](hinf_cfg_data7_reg::R) reader structure"]
impl crate::Readable for HINF_CFG_DATA7_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cfg_data7_reg::W](hinf_cfg_data7_reg::W) writer structure"]
impl crate::Writable for HINF_CFG_DATA7_REG {}
#[doc = "HINF_CFG_DATA7_REG"]
pub mod hinf_cfg_data7_reg;
#[doc = "HINF_CIS_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf0_reg](hinf_cis_conf0_reg) module"]
pub type HINF_CIS_CONF0_REG = crate::Reg<u32, _HINF_CIS_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF0_REG;
#[doc = "`read()` method returns [hinf_cis_conf0_reg::R](hinf_cis_conf0_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf0_reg::W](hinf_cis_conf0_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF0_REG {}
#[doc = "HINF_CIS_CONF0_REG"]
pub mod hinf_cis_conf0_reg;
#[doc = "HINF_CIS_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf1_reg](hinf_cis_conf1_reg) module"]
pub type HINF_CIS_CONF1_REG = crate::Reg<u32, _HINF_CIS_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF1_REG;
#[doc = "`read()` method returns [hinf_cis_conf1_reg::R](hinf_cis_conf1_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf1_reg::W](hinf_cis_conf1_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF1_REG {}
#[doc = "HINF_CIS_CONF1_REG"]
pub mod hinf_cis_conf1_reg;
#[doc = "HINF_CIS_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf2_reg](hinf_cis_conf2_reg) module"]
pub type HINF_CIS_CONF2_REG = crate::Reg<u32, _HINF_CIS_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF2_REG;
#[doc = "`read()` method returns [hinf_cis_conf2_reg::R](hinf_cis_conf2_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf2_reg::W](hinf_cis_conf2_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF2_REG {}
#[doc = "HINF_CIS_CONF2_REG"]
pub mod hinf_cis_conf2_reg;
#[doc = "HINF_CIS_CONF3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf3_reg](hinf_cis_conf3_reg) module"]
pub type HINF_CIS_CONF3_REG = crate::Reg<u32, _HINF_CIS_CONF3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF3_REG;
#[doc = "`read()` method returns [hinf_cis_conf3_reg::R](hinf_cis_conf3_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF3_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf3_reg::W](hinf_cis_conf3_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF3_REG {}
#[doc = "HINF_CIS_CONF3_REG"]
pub mod hinf_cis_conf3_reg;
#[doc = "HINF_CIS_CONF4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf4_reg](hinf_cis_conf4_reg) module"]
pub type HINF_CIS_CONF4_REG = crate::Reg<u32, _HINF_CIS_CONF4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF4_REG;
#[doc = "`read()` method returns [hinf_cis_conf4_reg::R](hinf_cis_conf4_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF4_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf4_reg::W](hinf_cis_conf4_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF4_REG {}
#[doc = "HINF_CIS_CONF4_REG"]
pub mod hinf_cis_conf4_reg;
#[doc = "HINF_CIS_CONF5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf5_reg](hinf_cis_conf5_reg) module"]
pub type HINF_CIS_CONF5_REG = crate::Reg<u32, _HINF_CIS_CONF5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF5_REG;
#[doc = "`read()` method returns [hinf_cis_conf5_reg::R](hinf_cis_conf5_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF5_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf5_reg::W](hinf_cis_conf5_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF5_REG {}
#[doc = "HINF_CIS_CONF5_REG"]
pub mod hinf_cis_conf5_reg;
#[doc = "HINF_CIS_CONF6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf6_reg](hinf_cis_conf6_reg) module"]
pub type HINF_CIS_CONF6_REG = crate::Reg<u32, _HINF_CIS_CONF6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF6_REG;
#[doc = "`read()` method returns [hinf_cis_conf6_reg::R](hinf_cis_conf6_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF6_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf6_reg::W](hinf_cis_conf6_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF6_REG {}
#[doc = "HINF_CIS_CONF6_REG"]
pub mod hinf_cis_conf6_reg;
#[doc = "HINF_CIS_CONF7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cis_conf7_reg](hinf_cis_conf7_reg) module"]
pub type HINF_CIS_CONF7_REG = crate::Reg<u32, _HINF_CIS_CONF7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CIS_CONF7_REG;
#[doc = "`read()` method returns [hinf_cis_conf7_reg::R](hinf_cis_conf7_reg::R) reader structure"]
impl crate::Readable for HINF_CIS_CONF7_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cis_conf7_reg::W](hinf_cis_conf7_reg::W) writer structure"]
impl crate::Writable for HINF_CIS_CONF7_REG {}
#[doc = "HINF_CIS_CONF7_REG"]
pub mod hinf_cis_conf7_reg;
#[doc = "HINF_CFG_DATA16_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_cfg_data16_reg](hinf_cfg_data16_reg) module"]
pub type HINF_CFG_DATA16_REG = crate::Reg<u32, _HINF_CFG_DATA16_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_CFG_DATA16_REG;
#[doc = "`read()` method returns [hinf_cfg_data16_reg::R](hinf_cfg_data16_reg::R) reader structure"]
impl crate::Readable for HINF_CFG_DATA16_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_cfg_data16_reg::W](hinf_cfg_data16_reg::W) writer structure"]
impl crate::Writable for HINF_CFG_DATA16_REG {}
#[doc = "HINF_CFG_DATA16_REG"]
pub mod hinf_cfg_data16_reg;
#[doc = "HINF_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hinf_date_reg](hinf_date_reg) module"]
pub type HINF_DATE_REG = crate::Reg<u32, _HINF_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINF_DATE_REG;
#[doc = "`read()` method returns [hinf_date_reg::R](hinf_date_reg::R) reader structure"]
impl crate::Readable for HINF_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [hinf_date_reg::W](hinf_date_reg::W) writer structure"]
impl crate::Writable for HINF_DATE_REG {}
#[doc = "HINF_DATE_REG"]
pub mod hinf_date_reg;
