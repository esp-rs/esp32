#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HINF_CFG_DATA0"]
    pub cfg_data0: CFG_DATA0,
    #[doc = "0x04 - HINF_CFG_DATA1"]
    pub cfg_data1: CFG_DATA1,
    _reserved2: [u8; 20usize],
    #[doc = "0x1c - HINF_CFG_DATA7"]
    pub cfg_data7: CFG_DATA7,
    #[doc = "0x20 - HINF_CIS_CONF0"]
    pub cis_conf0: CIS_CONF0,
    #[doc = "0x24 - HINF_CIS_CONF1"]
    pub cis_conf1: CIS_CONF1,
    #[doc = "0x28 - HINF_CIS_CONF2"]
    pub cis_conf2: CIS_CONF2,
    #[doc = "0x2c - HINF_CIS_CONF3"]
    pub cis_conf3: CIS_CONF3,
    #[doc = "0x30 - HINF_CIS_CONF4"]
    pub cis_conf4: CIS_CONF4,
    #[doc = "0x34 - HINF_CIS_CONF5"]
    pub cis_conf5: CIS_CONF5,
    #[doc = "0x38 - HINF_CIS_CONF6"]
    pub cis_conf6: CIS_CONF6,
    #[doc = "0x3c - HINF_CIS_CONF7"]
    pub cis_conf7: CIS_CONF7,
    #[doc = "0x40 - HINF_CFG_DATA16"]
    pub cfg_data16: CFG_DATA16,
    _reserved12: [u8; 184usize],
    #[doc = "0xfc - HINF_DATE"]
    pub date: DATE,
}
#[doc = "HINF_CFG_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_data0](cfg_data0) module"]
pub type CFG_DATA0 = crate::Reg<u32, _CFG_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_DATA0;
#[doc = "`read()` method returns [cfg_data0::R](cfg_data0::R) reader structure"]
impl crate::Readable for CFG_DATA0 {}
#[doc = "`write(|w| ..)` method takes [cfg_data0::W](cfg_data0::W) writer structure"]
impl crate::Writable for CFG_DATA0 {}
#[doc = "HINF_CFG_DATA0"]
pub mod cfg_data0;
#[doc = "HINF_CFG_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_data1](cfg_data1) module"]
pub type CFG_DATA1 = crate::Reg<u32, _CFG_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_DATA1;
#[doc = "`read()` method returns [cfg_data1::R](cfg_data1::R) reader structure"]
impl crate::Readable for CFG_DATA1 {}
#[doc = "`write(|w| ..)` method takes [cfg_data1::W](cfg_data1::W) writer structure"]
impl crate::Writable for CFG_DATA1 {}
#[doc = "HINF_CFG_DATA1"]
pub mod cfg_data1;
#[doc = "HINF_CFG_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_data7](cfg_data7) module"]
pub type CFG_DATA7 = crate::Reg<u32, _CFG_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_DATA7;
#[doc = "`read()` method returns [cfg_data7::R](cfg_data7::R) reader structure"]
impl crate::Readable for CFG_DATA7 {}
#[doc = "`write(|w| ..)` method takes [cfg_data7::W](cfg_data7::W) writer structure"]
impl crate::Writable for CFG_DATA7 {}
#[doc = "HINF_CFG_DATA7"]
pub mod cfg_data7;
#[doc = "HINF_CIS_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf0](cis_conf0) module"]
pub type CIS_CONF0 = crate::Reg<u32, _CIS_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF0;
#[doc = "`read()` method returns [cis_conf0::R](cis_conf0::R) reader structure"]
impl crate::Readable for CIS_CONF0 {}
#[doc = "`write(|w| ..)` method takes [cis_conf0::W](cis_conf0::W) writer structure"]
impl crate::Writable for CIS_CONF0 {}
#[doc = "HINF_CIS_CONF0"]
pub mod cis_conf0;
#[doc = "HINF_CIS_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf1](cis_conf1) module"]
pub type CIS_CONF1 = crate::Reg<u32, _CIS_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF1;
#[doc = "`read()` method returns [cis_conf1::R](cis_conf1::R) reader structure"]
impl crate::Readable for CIS_CONF1 {}
#[doc = "`write(|w| ..)` method takes [cis_conf1::W](cis_conf1::W) writer structure"]
impl crate::Writable for CIS_CONF1 {}
#[doc = "HINF_CIS_CONF1"]
pub mod cis_conf1;
#[doc = "HINF_CIS_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf2](cis_conf2) module"]
pub type CIS_CONF2 = crate::Reg<u32, _CIS_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF2;
#[doc = "`read()` method returns [cis_conf2::R](cis_conf2::R) reader structure"]
impl crate::Readable for CIS_CONF2 {}
#[doc = "`write(|w| ..)` method takes [cis_conf2::W](cis_conf2::W) writer structure"]
impl crate::Writable for CIS_CONF2 {}
#[doc = "HINF_CIS_CONF2"]
pub mod cis_conf2;
#[doc = "HINF_CIS_CONF3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf3](cis_conf3) module"]
pub type CIS_CONF3 = crate::Reg<u32, _CIS_CONF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF3;
#[doc = "`read()` method returns [cis_conf3::R](cis_conf3::R) reader structure"]
impl crate::Readable for CIS_CONF3 {}
#[doc = "`write(|w| ..)` method takes [cis_conf3::W](cis_conf3::W) writer structure"]
impl crate::Writable for CIS_CONF3 {}
#[doc = "HINF_CIS_CONF3"]
pub mod cis_conf3;
#[doc = "HINF_CIS_CONF4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf4](cis_conf4) module"]
pub type CIS_CONF4 = crate::Reg<u32, _CIS_CONF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF4;
#[doc = "`read()` method returns [cis_conf4::R](cis_conf4::R) reader structure"]
impl crate::Readable for CIS_CONF4 {}
#[doc = "`write(|w| ..)` method takes [cis_conf4::W](cis_conf4::W) writer structure"]
impl crate::Writable for CIS_CONF4 {}
#[doc = "HINF_CIS_CONF4"]
pub mod cis_conf4;
#[doc = "HINF_CIS_CONF5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf5](cis_conf5) module"]
pub type CIS_CONF5 = crate::Reg<u32, _CIS_CONF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF5;
#[doc = "`read()` method returns [cis_conf5::R](cis_conf5::R) reader structure"]
impl crate::Readable for CIS_CONF5 {}
#[doc = "`write(|w| ..)` method takes [cis_conf5::W](cis_conf5::W) writer structure"]
impl crate::Writable for CIS_CONF5 {}
#[doc = "HINF_CIS_CONF5"]
pub mod cis_conf5;
#[doc = "HINF_CIS_CONF6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf6](cis_conf6) module"]
pub type CIS_CONF6 = crate::Reg<u32, _CIS_CONF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF6;
#[doc = "`read()` method returns [cis_conf6::R](cis_conf6::R) reader structure"]
impl crate::Readable for CIS_CONF6 {}
#[doc = "`write(|w| ..)` method takes [cis_conf6::W](cis_conf6::W) writer structure"]
impl crate::Writable for CIS_CONF6 {}
#[doc = "HINF_CIS_CONF6"]
pub mod cis_conf6;
#[doc = "HINF_CIS_CONF7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cis_conf7](cis_conf7) module"]
pub type CIS_CONF7 = crate::Reg<u32, _CIS_CONF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS_CONF7;
#[doc = "`read()` method returns [cis_conf7::R](cis_conf7::R) reader structure"]
impl crate::Readable for CIS_CONF7 {}
#[doc = "`write(|w| ..)` method takes [cis_conf7::W](cis_conf7::W) writer structure"]
impl crate::Writable for CIS_CONF7 {}
#[doc = "HINF_CIS_CONF7"]
pub mod cis_conf7;
#[doc = "HINF_CFG_DATA16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_data16](cfg_data16) module"]
pub type CFG_DATA16 = crate::Reg<u32, _CFG_DATA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_DATA16;
#[doc = "`read()` method returns [cfg_data16::R](cfg_data16::R) reader structure"]
impl crate::Readable for CFG_DATA16 {}
#[doc = "`write(|w| ..)` method takes [cfg_data16::W](cfg_data16::W) writer structure"]
impl crate::Writable for CFG_DATA16 {}
#[doc = "HINF_CFG_DATA16"]
pub mod cfg_data16;
#[doc = "HINF_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "HINF_DATE"]
pub mod date;
