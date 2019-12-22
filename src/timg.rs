#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG_REG(i)"]
    pub timg_t0config_reg: TIMG_T0CONFIG_REG,
    #[doc = "0x04 - TIMG_T0LO_REG(i)"]
    pub timg_t0lo_reg: TIMG_T0LO_REG,
    #[doc = "0x08 - TIMG_T0HI_REG(i)"]
    pub timg_t0hi_reg: TIMG_T0HI_REG,
    #[doc = "0x0c - TIMG_T0UPDATE_REG(i)"]
    pub timg_t0update_reg: TIMG_T0UPDATE_REG,
    #[doc = "0x10 - TIMG_T0ALARMLO_REG(i)"]
    pub timg_t0alarmlo_reg: TIMG_T0ALARMLO_REG,
    #[doc = "0x14 - TIMG_T0ALARMHI_REG(i)"]
    pub timg_t0alarmhi_reg: TIMG_T0ALARMHI_REG,
    #[doc = "0x18 - TIMG_T0LOADLO_REG(i)"]
    pub timg_t0loadlo_reg: TIMG_T0LOADLO_REG,
    #[doc = "0x1c - TIMG_T0LOADHI_REG(i)"]
    pub timg_t0loadhi_reg: TIMG_T0LOADHI_REG,
    #[doc = "0x20 - TIMG_T0LOAD_REG(i)"]
    pub timg_t0load_reg: TIMG_T0LOAD_REG,
    #[doc = "0x24 - TIMG_T1CONFIG_REG(i)"]
    pub timg_t1config_reg: TIMG_T1CONFIG_REG,
    #[doc = "0x28 - TIMG_T1LO_REG(i)"]
    pub timg_t1lo_reg: TIMG_T1LO_REG,
    #[doc = "0x2c - TIMG_T1HI_REG(i)"]
    pub timg_t1hi_reg: TIMG_T1HI_REG,
    #[doc = "0x30 - TIMG_T1UPDATE_REG(i)"]
    pub timg_t1update_reg: TIMG_T1UPDATE_REG,
    #[doc = "0x34 - TIMG_T1ALARMLO_REG(i)"]
    pub timg_t1alarmlo_reg: TIMG_T1ALARMLO_REG,
    #[doc = "0x38 - TIMG_T1ALARMHI_REG(i)"]
    pub timg_t1alarmhi_reg: TIMG_T1ALARMHI_REG,
    #[doc = "0x3c - TIMG_T1LOADLO_REG(i)"]
    pub timg_t1loadlo_reg: TIMG_T1LOADLO_REG,
    #[doc = "0x40 - TIMG_T1LOADHI_REG(i)"]
    pub timg_t1loadhi_reg: TIMG_T1LOADHI_REG,
    #[doc = "0x44 - TIMG_T1LOAD_REG(i)"]
    pub timg_t1load_reg: TIMG_T1LOAD_REG,
    #[doc = "0x48 - TIMG_WDTCONFIG0_REG(i)"]
    pub timg_wdtconfig0_reg: TIMG_WDTCONFIG0_REG,
    #[doc = "0x4c - TIMG_WDTCONFIG1_REG(i)"]
    pub timg_wdtconfig1_reg: TIMG_WDTCONFIG1_REG,
    #[doc = "0x50 - TIMG_WDTCONFIG2_REG(i)"]
    pub timg_wdtconfig2_reg: TIMG_WDTCONFIG2_REG,
    #[doc = "0x54 - TIMG_WDTCONFIG3_REG(i)"]
    pub timg_wdtconfig3_reg: TIMG_WDTCONFIG3_REG,
    #[doc = "0x58 - TIMG_WDTCONFIG4_REG(i)"]
    pub timg_wdtconfig4_reg: TIMG_WDTCONFIG4_REG,
    #[doc = "0x5c - TIMG_WDTCONFIG5_REG(i)"]
    pub timg_wdtconfig5_reg: TIMG_WDTCONFIG5_REG,
    #[doc = "0x60 - TIMG_WDTFEED_REG(i)"]
    pub timg_wdtfeed_reg: TIMG_WDTFEED_REG,
    #[doc = "0x64 - TIMG_WDTWPROTECT_REG(i)"]
    pub timg_wdtwprotect_reg: TIMG_WDTWPROTECT_REG,
    #[doc = "0x68 - TIMG_RTCCALICFG_REG(i)"]
    pub timg_rtccalicfg_reg: TIMG_RTCCALICFG_REG,
    #[doc = "0x6c - TIMG_RTCCALICFG1_REG(i)"]
    pub timg_rtccalicfg1_reg: TIMG_RTCCALICFG1_REG,
    #[doc = "0x70 - TIMG_LACTCONFIG_REG(i)"]
    pub timg_lactconfig_reg: TIMG_LACTCONFIG_REG,
    #[doc = "0x74 - TIMG_LACTRTC_REG(i)"]
    pub timg_lactrtc_reg: TIMG_LACTRTC_REG,
    #[doc = "0x78 - TIMG_LACTLO_REG(i)"]
    pub timg_lactlo_reg: TIMG_LACTLO_REG,
    #[doc = "0x7c - TIMG_LACTHI_REG(i)"]
    pub timg_lacthi_reg: TIMG_LACTHI_REG,
    #[doc = "0x80 - TIMG_LACTUPDATE_REG(i)"]
    pub timg_lactupdate_reg: TIMG_LACTUPDATE_REG,
    #[doc = "0x84 - TIMG_LACTALARMLO_REG(i)"]
    pub timg_lactalarmlo_reg: TIMG_LACTALARMLO_REG,
    #[doc = "0x88 - TIMG_LACTALARMHI_REG(i)"]
    pub timg_lactalarmhi_reg: TIMG_LACTALARMHI_REG,
    #[doc = "0x8c - TIMG_LACTLOADLO_REG(i)"]
    pub timg_lactloadlo_reg: TIMG_LACTLOADLO_REG,
    #[doc = "0x90 - TIMG_LACTLOADHI_REG(i)"]
    pub timg_lactloadhi_reg: TIMG_LACTLOADHI_REG,
    #[doc = "0x94 - TIMG_LACTLOAD_REG(i)"]
    pub timg_lactload_reg: TIMG_LACTLOAD_REG,
    #[doc = "0x98 - TIMG_INT_ENA_TIMERS_REG(i)"]
    pub timg_int_ena_timers_reg: TIMG_INT_ENA_TIMERS_REG,
    #[doc = "0x9c - TIMG_INT_RAW_TIMERS_REG(i)"]
    pub timg_int_raw_timers_reg: TIMG_INT_RAW_TIMERS_REG,
    #[doc = "0xa0 - TIMG_INT_ST_TIMERS_REG(i)"]
    pub timg_int_st_timers_reg: TIMG_INT_ST_TIMERS_REG,
    #[doc = "0xa4 - TIMG_INT_CLR_TIMERS_REG(i)"]
    pub timg_int_clr_timers_reg: TIMG_INT_CLR_TIMERS_REG,
    _reserved42: [u8; 80usize],
    #[doc = "0xf8 - TIMG_NTIMERS_DATE_REG(i)"]
    pub timg_ntimers_date_reg: TIMG_NTIMERS_DATE_REG,
    #[doc = "0xfc - TIMGCLK_REG(i)"]
    pub timgclk_reg: TIMGCLK_REG,
}
#[doc = "TIMG_T0CONFIG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0config_reg](timg_t0config_reg) module"]
pub type TIMG_T0CONFIG_REG = crate::Reg<u32, _TIMG_T0CONFIG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0CONFIG_REG;
#[doc = "`read()` method returns [timg_t0config_reg::R](timg_t0config_reg::R) reader structure"]
impl crate::Readable for TIMG_T0CONFIG_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0config_reg::W](timg_t0config_reg::W) writer structure"]
impl crate::Writable for TIMG_T0CONFIG_REG {}
#[doc = "TIMG_T0CONFIG_REG(i)"]
pub mod timg_t0config_reg;
#[doc = "TIMG_T0LO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0lo_reg](timg_t0lo_reg) module"]
pub type TIMG_T0LO_REG = crate::Reg<u32, _TIMG_T0LO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LO_REG;
#[doc = "`read()` method returns [timg_t0lo_reg::R](timg_t0lo_reg::R) reader structure"]
impl crate::Readable for TIMG_T0LO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0lo_reg::W](timg_t0lo_reg::W) writer structure"]
impl crate::Writable for TIMG_T0LO_REG {}
#[doc = "TIMG_T0LO_REG(i)"]
pub mod timg_t0lo_reg;
#[doc = "TIMG_T0HI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0hi_reg](timg_t0hi_reg) module"]
pub type TIMG_T0HI_REG = crate::Reg<u32, _TIMG_T0HI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0HI_REG;
#[doc = "`read()` method returns [timg_t0hi_reg::R](timg_t0hi_reg::R) reader structure"]
impl crate::Readable for TIMG_T0HI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0hi_reg::W](timg_t0hi_reg::W) writer structure"]
impl crate::Writable for TIMG_T0HI_REG {}
#[doc = "TIMG_T0HI_REG(i)"]
pub mod timg_t0hi_reg;
#[doc = "TIMG_T0UPDATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0update_reg](timg_t0update_reg) module"]
pub type TIMG_T0UPDATE_REG = crate::Reg<u32, _TIMG_T0UPDATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0UPDATE_REG;
#[doc = "`read()` method returns [timg_t0update_reg::R](timg_t0update_reg::R) reader structure"]
impl crate::Readable for TIMG_T0UPDATE_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0update_reg::W](timg_t0update_reg::W) writer structure"]
impl crate::Writable for TIMG_T0UPDATE_REG {}
#[doc = "TIMG_T0UPDATE_REG(i)"]
pub mod timg_t0update_reg;
#[doc = "TIMG_T0ALARMLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0alarmlo_reg](timg_t0alarmlo_reg) module"]
pub type TIMG_T0ALARMLO_REG = crate::Reg<u32, _TIMG_T0ALARMLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0ALARMLO_REG;
#[doc = "`read()` method returns [timg_t0alarmlo_reg::R](timg_t0alarmlo_reg::R) reader structure"]
impl crate::Readable for TIMG_T0ALARMLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0alarmlo_reg::W](timg_t0alarmlo_reg::W) writer structure"]
impl crate::Writable for TIMG_T0ALARMLO_REG {}
#[doc = "TIMG_T0ALARMLO_REG(i)"]
pub mod timg_t0alarmlo_reg;
#[doc = "TIMG_T0ALARMHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0alarmhi_reg](timg_t0alarmhi_reg) module"]
pub type TIMG_T0ALARMHI_REG = crate::Reg<u32, _TIMG_T0ALARMHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0ALARMHI_REG;
#[doc = "`read()` method returns [timg_t0alarmhi_reg::R](timg_t0alarmhi_reg::R) reader structure"]
impl crate::Readable for TIMG_T0ALARMHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0alarmhi_reg::W](timg_t0alarmhi_reg::W) writer structure"]
impl crate::Writable for TIMG_T0ALARMHI_REG {}
#[doc = "TIMG_T0ALARMHI_REG(i)"]
pub mod timg_t0alarmhi_reg;
#[doc = "TIMG_T0LOADLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0loadlo_reg](timg_t0loadlo_reg) module"]
pub type TIMG_T0LOADLO_REG = crate::Reg<u32, _TIMG_T0LOADLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LOADLO_REG;
#[doc = "`read()` method returns [timg_t0loadlo_reg::R](timg_t0loadlo_reg::R) reader structure"]
impl crate::Readable for TIMG_T0LOADLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0loadlo_reg::W](timg_t0loadlo_reg::W) writer structure"]
impl crate::Writable for TIMG_T0LOADLO_REG {}
#[doc = "TIMG_T0LOADLO_REG(i)"]
pub mod timg_t0loadlo_reg;
#[doc = "TIMG_T0LOADHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0loadhi_reg](timg_t0loadhi_reg) module"]
pub type TIMG_T0LOADHI_REG = crate::Reg<u32, _TIMG_T0LOADHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LOADHI_REG;
#[doc = "`read()` method returns [timg_t0loadhi_reg::R](timg_t0loadhi_reg::R) reader structure"]
impl crate::Readable for TIMG_T0LOADHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0loadhi_reg::W](timg_t0loadhi_reg::W) writer structure"]
impl crate::Writable for TIMG_T0LOADHI_REG {}
#[doc = "TIMG_T0LOADHI_REG(i)"]
pub mod timg_t0loadhi_reg;
#[doc = "TIMG_T0LOAD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t0load_reg](timg_t0load_reg) module"]
pub type TIMG_T0LOAD_REG = crate::Reg<u32, _TIMG_T0LOAD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LOAD_REG;
#[doc = "`read()` method returns [timg_t0load_reg::R](timg_t0load_reg::R) reader structure"]
impl crate::Readable for TIMG_T0LOAD_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t0load_reg::W](timg_t0load_reg::W) writer structure"]
impl crate::Writable for TIMG_T0LOAD_REG {}
#[doc = "TIMG_T0LOAD_REG(i)"]
pub mod timg_t0load_reg;
#[doc = "TIMG_T1CONFIG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1config_reg](timg_t1config_reg) module"]
pub type TIMG_T1CONFIG_REG = crate::Reg<u32, _TIMG_T1CONFIG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1CONFIG_REG;
#[doc = "`read()` method returns [timg_t1config_reg::R](timg_t1config_reg::R) reader structure"]
impl crate::Readable for TIMG_T1CONFIG_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1config_reg::W](timg_t1config_reg::W) writer structure"]
impl crate::Writable for TIMG_T1CONFIG_REG {}
#[doc = "TIMG_T1CONFIG_REG(i)"]
pub mod timg_t1config_reg;
#[doc = "TIMG_T1LO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1lo_reg](timg_t1lo_reg) module"]
pub type TIMG_T1LO_REG = crate::Reg<u32, _TIMG_T1LO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1LO_REG;
#[doc = "`read()` method returns [timg_t1lo_reg::R](timg_t1lo_reg::R) reader structure"]
impl crate::Readable for TIMG_T1LO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1lo_reg::W](timg_t1lo_reg::W) writer structure"]
impl crate::Writable for TIMG_T1LO_REG {}
#[doc = "TIMG_T1LO_REG(i)"]
pub mod timg_t1lo_reg;
#[doc = "TIMG_T1HI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1hi_reg](timg_t1hi_reg) module"]
pub type TIMG_T1HI_REG = crate::Reg<u32, _TIMG_T1HI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1HI_REG;
#[doc = "`read()` method returns [timg_t1hi_reg::R](timg_t1hi_reg::R) reader structure"]
impl crate::Readable for TIMG_T1HI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1hi_reg::W](timg_t1hi_reg::W) writer structure"]
impl crate::Writable for TIMG_T1HI_REG {}
#[doc = "TIMG_T1HI_REG(i)"]
pub mod timg_t1hi_reg;
#[doc = "TIMG_T1UPDATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1update_reg](timg_t1update_reg) module"]
pub type TIMG_T1UPDATE_REG = crate::Reg<u32, _TIMG_T1UPDATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1UPDATE_REG;
#[doc = "`read()` method returns [timg_t1update_reg::R](timg_t1update_reg::R) reader structure"]
impl crate::Readable for TIMG_T1UPDATE_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1update_reg::W](timg_t1update_reg::W) writer structure"]
impl crate::Writable for TIMG_T1UPDATE_REG {}
#[doc = "TIMG_T1UPDATE_REG(i)"]
pub mod timg_t1update_reg;
#[doc = "TIMG_T1ALARMLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1alarmlo_reg](timg_t1alarmlo_reg) module"]
pub type TIMG_T1ALARMLO_REG = crate::Reg<u32, _TIMG_T1ALARMLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1ALARMLO_REG;
#[doc = "`read()` method returns [timg_t1alarmlo_reg::R](timg_t1alarmlo_reg::R) reader structure"]
impl crate::Readable for TIMG_T1ALARMLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1alarmlo_reg::W](timg_t1alarmlo_reg::W) writer structure"]
impl crate::Writable for TIMG_T1ALARMLO_REG {}
#[doc = "TIMG_T1ALARMLO_REG(i)"]
pub mod timg_t1alarmlo_reg;
#[doc = "TIMG_T1ALARMHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1alarmhi_reg](timg_t1alarmhi_reg) module"]
pub type TIMG_T1ALARMHI_REG = crate::Reg<u32, _TIMG_T1ALARMHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1ALARMHI_REG;
#[doc = "`read()` method returns [timg_t1alarmhi_reg::R](timg_t1alarmhi_reg::R) reader structure"]
impl crate::Readable for TIMG_T1ALARMHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1alarmhi_reg::W](timg_t1alarmhi_reg::W) writer structure"]
impl crate::Writable for TIMG_T1ALARMHI_REG {}
#[doc = "TIMG_T1ALARMHI_REG(i)"]
pub mod timg_t1alarmhi_reg;
#[doc = "TIMG_T1LOADLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1loadlo_reg](timg_t1loadlo_reg) module"]
pub type TIMG_T1LOADLO_REG = crate::Reg<u32, _TIMG_T1LOADLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1LOADLO_REG;
#[doc = "`read()` method returns [timg_t1loadlo_reg::R](timg_t1loadlo_reg::R) reader structure"]
impl crate::Readable for TIMG_T1LOADLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1loadlo_reg::W](timg_t1loadlo_reg::W) writer structure"]
impl crate::Writable for TIMG_T1LOADLO_REG {}
#[doc = "TIMG_T1LOADLO_REG(i)"]
pub mod timg_t1loadlo_reg;
#[doc = "TIMG_T1LOADHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1loadhi_reg](timg_t1loadhi_reg) module"]
pub type TIMG_T1LOADHI_REG = crate::Reg<u32, _TIMG_T1LOADHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1LOADHI_REG;
#[doc = "`read()` method returns [timg_t1loadhi_reg::R](timg_t1loadhi_reg::R) reader structure"]
impl crate::Readable for TIMG_T1LOADHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1loadhi_reg::W](timg_t1loadhi_reg::W) writer structure"]
impl crate::Writable for TIMG_T1LOADHI_REG {}
#[doc = "TIMG_T1LOADHI_REG(i)"]
pub mod timg_t1loadhi_reg;
#[doc = "TIMG_T1LOAD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_t1load_reg](timg_t1load_reg) module"]
pub type TIMG_T1LOAD_REG = crate::Reg<u32, _TIMG_T1LOAD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T1LOAD_REG;
#[doc = "`read()` method returns [timg_t1load_reg::R](timg_t1load_reg::R) reader structure"]
impl crate::Readable for TIMG_T1LOAD_REG {}
#[doc = "`write(|w| ..)` method takes [timg_t1load_reg::W](timg_t1load_reg::W) writer structure"]
impl crate::Writable for TIMG_T1LOAD_REG {}
#[doc = "TIMG_T1LOAD_REG(i)"]
pub mod timg_t1load_reg;
#[doc = "TIMG_WDTCONFIG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtconfig0_reg](timg_wdtconfig0_reg) module"]
pub type TIMG_WDTCONFIG0_REG = crate::Reg<u32, _TIMG_WDTCONFIG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG0_REG;
#[doc = "`read()` method returns [timg_wdtconfig0_reg::R](timg_wdtconfig0_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG0_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig0_reg::W](timg_wdtconfig0_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG0_REG {}
#[doc = "TIMG_WDTCONFIG0_REG(i)"]
pub mod timg_wdtconfig0_reg;
#[doc = "TIMG_WDTCONFIG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtconfig1_reg](timg_wdtconfig1_reg) module"]
pub type TIMG_WDTCONFIG1_REG = crate::Reg<u32, _TIMG_WDTCONFIG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG1_REG;
#[doc = "`read()` method returns [timg_wdtconfig1_reg::R](timg_wdtconfig1_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG1_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig1_reg::W](timg_wdtconfig1_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG1_REG {}
#[doc = "TIMG_WDTCONFIG1_REG(i)"]
pub mod timg_wdtconfig1_reg;
#[doc = "TIMG_WDTCONFIG2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtconfig2_reg](timg_wdtconfig2_reg) module"]
pub type TIMG_WDTCONFIG2_REG = crate::Reg<u32, _TIMG_WDTCONFIG2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG2_REG;
#[doc = "`read()` method returns [timg_wdtconfig2_reg::R](timg_wdtconfig2_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG2_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig2_reg::W](timg_wdtconfig2_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG2_REG {}
#[doc = "TIMG_WDTCONFIG2_REG(i)"]
pub mod timg_wdtconfig2_reg;
#[doc = "TIMG_WDTCONFIG3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtconfig3_reg](timg_wdtconfig3_reg) module"]
pub type TIMG_WDTCONFIG3_REG = crate::Reg<u32, _TIMG_WDTCONFIG3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG3_REG;
#[doc = "`read()` method returns [timg_wdtconfig3_reg::R](timg_wdtconfig3_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG3_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig3_reg::W](timg_wdtconfig3_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG3_REG {}
#[doc = "TIMG_WDTCONFIG3_REG(i)"]
pub mod timg_wdtconfig3_reg;
#[doc = "TIMG_WDTCONFIG4_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtconfig4_reg](timg_wdtconfig4_reg) module"]
pub type TIMG_WDTCONFIG4_REG = crate::Reg<u32, _TIMG_WDTCONFIG4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG4_REG;
#[doc = "`read()` method returns [timg_wdtconfig4_reg::R](timg_wdtconfig4_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG4_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig4_reg::W](timg_wdtconfig4_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG4_REG {}
#[doc = "TIMG_WDTCONFIG4_REG(i)"]
pub mod timg_wdtconfig4_reg;
#[doc = "TIMG_WDTCONFIG5_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtconfig5_reg](timg_wdtconfig5_reg) module"]
pub type TIMG_WDTCONFIG5_REG = crate::Reg<u32, _TIMG_WDTCONFIG5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG5_REG;
#[doc = "`read()` method returns [timg_wdtconfig5_reg::R](timg_wdtconfig5_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG5_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig5_reg::W](timg_wdtconfig5_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG5_REG {}
#[doc = "TIMG_WDTCONFIG5_REG(i)"]
pub mod timg_wdtconfig5_reg;
#[doc = "TIMG_WDTFEED_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtfeed_reg](timg_wdtfeed_reg) module"]
pub type TIMG_WDTFEED_REG = crate::Reg<u32, _TIMG_WDTFEED_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTFEED_REG;
#[doc = "`read()` method returns [timg_wdtfeed_reg::R](timg_wdtfeed_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTFEED_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtfeed_reg::W](timg_wdtfeed_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTFEED_REG {}
#[doc = "TIMG_WDTFEED_REG(i)"]
pub mod timg_wdtfeed_reg;
#[doc = "TIMG_WDTWPROTECT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_wdtwprotect_reg](timg_wdtwprotect_reg) module"]
pub type TIMG_WDTWPROTECT_REG = crate::Reg<u32, _TIMG_WDTWPROTECT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTWPROTECT_REG;
#[doc = "`read()` method returns [timg_wdtwprotect_reg::R](timg_wdtwprotect_reg::R) reader structure"]
impl crate::Readable for TIMG_WDTWPROTECT_REG {}
#[doc = "`write(|w| ..)` method takes [timg_wdtwprotect_reg::W](timg_wdtwprotect_reg::W) writer structure"]
impl crate::Writable for TIMG_WDTWPROTECT_REG {}
#[doc = "TIMG_WDTWPROTECT_REG(i)"]
pub mod timg_wdtwprotect_reg;
#[doc = "TIMG_RTCCALICFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_rtccalicfg_reg](timg_rtccalicfg_reg) module"]
pub type TIMG_RTCCALICFG_REG = crate::Reg<u32, _TIMG_RTCCALICFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_RTCCALICFG_REG;
#[doc = "`read()` method returns [timg_rtccalicfg_reg::R](timg_rtccalicfg_reg::R) reader structure"]
impl crate::Readable for TIMG_RTCCALICFG_REG {}
#[doc = "`write(|w| ..)` method takes [timg_rtccalicfg_reg::W](timg_rtccalicfg_reg::W) writer structure"]
impl crate::Writable for TIMG_RTCCALICFG_REG {}
#[doc = "TIMG_RTCCALICFG_REG(i)"]
pub mod timg_rtccalicfg_reg;
#[doc = "TIMG_RTCCALICFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_rtccalicfg1_reg](timg_rtccalicfg1_reg) module"]
pub type TIMG_RTCCALICFG1_REG = crate::Reg<u32, _TIMG_RTCCALICFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_RTCCALICFG1_REG;
#[doc = "`read()` method returns [timg_rtccalicfg1_reg::R](timg_rtccalicfg1_reg::R) reader structure"]
impl crate::Readable for TIMG_RTCCALICFG1_REG {}
#[doc = "`write(|w| ..)` method takes [timg_rtccalicfg1_reg::W](timg_rtccalicfg1_reg::W) writer structure"]
impl crate::Writable for TIMG_RTCCALICFG1_REG {}
#[doc = "TIMG_RTCCALICFG1_REG(i)"]
pub mod timg_rtccalicfg1_reg;
#[doc = "TIMG_LACTCONFIG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactconfig_reg](timg_lactconfig_reg) module"]
pub type TIMG_LACTCONFIG_REG = crate::Reg<u32, _TIMG_LACTCONFIG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTCONFIG_REG;
#[doc = "`read()` method returns [timg_lactconfig_reg::R](timg_lactconfig_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTCONFIG_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactconfig_reg::W](timg_lactconfig_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTCONFIG_REG {}
#[doc = "TIMG_LACTCONFIG_REG(i)"]
pub mod timg_lactconfig_reg;
#[doc = "TIMG_LACTRTC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactrtc_reg](timg_lactrtc_reg) module"]
pub type TIMG_LACTRTC_REG = crate::Reg<u32, _TIMG_LACTRTC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTRTC_REG;
#[doc = "`read()` method returns [timg_lactrtc_reg::R](timg_lactrtc_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTRTC_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactrtc_reg::W](timg_lactrtc_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTRTC_REG {}
#[doc = "TIMG_LACTRTC_REG(i)"]
pub mod timg_lactrtc_reg;
#[doc = "TIMG_LACTLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactlo_reg](timg_lactlo_reg) module"]
pub type TIMG_LACTLO_REG = crate::Reg<u32, _TIMG_LACTLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTLO_REG;
#[doc = "`read()` method returns [timg_lactlo_reg::R](timg_lactlo_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactlo_reg::W](timg_lactlo_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTLO_REG {}
#[doc = "TIMG_LACTLO_REG(i)"]
pub mod timg_lactlo_reg;
#[doc = "TIMG_LACTHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lacthi_reg](timg_lacthi_reg) module"]
pub type TIMG_LACTHI_REG = crate::Reg<u32, _TIMG_LACTHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTHI_REG;
#[doc = "`read()` method returns [timg_lacthi_reg::R](timg_lacthi_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lacthi_reg::W](timg_lacthi_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTHI_REG {}
#[doc = "TIMG_LACTHI_REG(i)"]
pub mod timg_lacthi_reg;
#[doc = "TIMG_LACTUPDATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactupdate_reg](timg_lactupdate_reg) module"]
pub type TIMG_LACTUPDATE_REG = crate::Reg<u32, _TIMG_LACTUPDATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTUPDATE_REG;
#[doc = "`read()` method returns [timg_lactupdate_reg::R](timg_lactupdate_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTUPDATE_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactupdate_reg::W](timg_lactupdate_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTUPDATE_REG {}
#[doc = "TIMG_LACTUPDATE_REG(i)"]
pub mod timg_lactupdate_reg;
#[doc = "TIMG_LACTALARMLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactalarmlo_reg](timg_lactalarmlo_reg) module"]
pub type TIMG_LACTALARMLO_REG = crate::Reg<u32, _TIMG_LACTALARMLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTALARMLO_REG;
#[doc = "`read()` method returns [timg_lactalarmlo_reg::R](timg_lactalarmlo_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTALARMLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactalarmlo_reg::W](timg_lactalarmlo_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTALARMLO_REG {}
#[doc = "TIMG_LACTALARMLO_REG(i)"]
pub mod timg_lactalarmlo_reg;
#[doc = "TIMG_LACTALARMHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactalarmhi_reg](timg_lactalarmhi_reg) module"]
pub type TIMG_LACTALARMHI_REG = crate::Reg<u32, _TIMG_LACTALARMHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTALARMHI_REG;
#[doc = "`read()` method returns [timg_lactalarmhi_reg::R](timg_lactalarmhi_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTALARMHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactalarmhi_reg::W](timg_lactalarmhi_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTALARMHI_REG {}
#[doc = "TIMG_LACTALARMHI_REG(i)"]
pub mod timg_lactalarmhi_reg;
#[doc = "TIMG_LACTLOADLO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactloadlo_reg](timg_lactloadlo_reg) module"]
pub type TIMG_LACTLOADLO_REG = crate::Reg<u32, _TIMG_LACTLOADLO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTLOADLO_REG;
#[doc = "`read()` method returns [timg_lactloadlo_reg::R](timg_lactloadlo_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTLOADLO_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactloadlo_reg::W](timg_lactloadlo_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTLOADLO_REG {}
#[doc = "TIMG_LACTLOADLO_REG(i)"]
pub mod timg_lactloadlo_reg;
#[doc = "TIMG_LACTLOADHI_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactloadhi_reg](timg_lactloadhi_reg) module"]
pub type TIMG_LACTLOADHI_REG = crate::Reg<u32, _TIMG_LACTLOADHI_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTLOADHI_REG;
#[doc = "`read()` method returns [timg_lactloadhi_reg::R](timg_lactloadhi_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTLOADHI_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactloadhi_reg::W](timg_lactloadhi_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTLOADHI_REG {}
#[doc = "TIMG_LACTLOADHI_REG(i)"]
pub mod timg_lactloadhi_reg;
#[doc = "TIMG_LACTLOAD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_lactload_reg](timg_lactload_reg) module"]
pub type TIMG_LACTLOAD_REG = crate::Reg<u32, _TIMG_LACTLOAD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_LACTLOAD_REG;
#[doc = "`read()` method returns [timg_lactload_reg::R](timg_lactload_reg::R) reader structure"]
impl crate::Readable for TIMG_LACTLOAD_REG {}
#[doc = "`write(|w| ..)` method takes [timg_lactload_reg::W](timg_lactload_reg::W) writer structure"]
impl crate::Writable for TIMG_LACTLOAD_REG {}
#[doc = "TIMG_LACTLOAD_REG(i)"]
pub mod timg_lactload_reg;
#[doc = "TIMG_INT_ENA_TIMERS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_int_ena_timers_reg](timg_int_ena_timers_reg) module"]
pub type TIMG_INT_ENA_TIMERS_REG = crate::Reg<u32, _TIMG_INT_ENA_TIMERS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_ENA_TIMERS_REG;
#[doc = "`read()` method returns [timg_int_ena_timers_reg::R](timg_int_ena_timers_reg::R) reader structure"]
impl crate::Readable for TIMG_INT_ENA_TIMERS_REG {}
#[doc = "`write(|w| ..)` method takes [timg_int_ena_timers_reg::W](timg_int_ena_timers_reg::W) writer structure"]
impl crate::Writable for TIMG_INT_ENA_TIMERS_REG {}
#[doc = "TIMG_INT_ENA_TIMERS_REG(i)"]
pub mod timg_int_ena_timers_reg;
#[doc = "TIMG_INT_RAW_TIMERS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_int_raw_timers_reg](timg_int_raw_timers_reg) module"]
pub type TIMG_INT_RAW_TIMERS_REG = crate::Reg<u32, _TIMG_INT_RAW_TIMERS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_RAW_TIMERS_REG;
#[doc = "`read()` method returns [timg_int_raw_timers_reg::R](timg_int_raw_timers_reg::R) reader structure"]
impl crate::Readable for TIMG_INT_RAW_TIMERS_REG {}
#[doc = "`write(|w| ..)` method takes [timg_int_raw_timers_reg::W](timg_int_raw_timers_reg::W) writer structure"]
impl crate::Writable for TIMG_INT_RAW_TIMERS_REG {}
#[doc = "TIMG_INT_RAW_TIMERS_REG(i)"]
pub mod timg_int_raw_timers_reg;
#[doc = "TIMG_INT_ST_TIMERS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_int_st_timers_reg](timg_int_st_timers_reg) module"]
pub type TIMG_INT_ST_TIMERS_REG = crate::Reg<u32, _TIMG_INT_ST_TIMERS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_ST_TIMERS_REG;
#[doc = "`read()` method returns [timg_int_st_timers_reg::R](timg_int_st_timers_reg::R) reader structure"]
impl crate::Readable for TIMG_INT_ST_TIMERS_REG {}
#[doc = "`write(|w| ..)` method takes [timg_int_st_timers_reg::W](timg_int_st_timers_reg::W) writer structure"]
impl crate::Writable for TIMG_INT_ST_TIMERS_REG {}
#[doc = "TIMG_INT_ST_TIMERS_REG(i)"]
pub mod timg_int_st_timers_reg;
#[doc = "TIMG_INT_CLR_TIMERS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_int_clr_timers_reg](timg_int_clr_timers_reg) module"]
pub type TIMG_INT_CLR_TIMERS_REG = crate::Reg<u32, _TIMG_INT_CLR_TIMERS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_CLR_TIMERS_REG;
#[doc = "`read()` method returns [timg_int_clr_timers_reg::R](timg_int_clr_timers_reg::R) reader structure"]
impl crate::Readable for TIMG_INT_CLR_TIMERS_REG {}
#[doc = "`write(|w| ..)` method takes [timg_int_clr_timers_reg::W](timg_int_clr_timers_reg::W) writer structure"]
impl crate::Writable for TIMG_INT_CLR_TIMERS_REG {}
#[doc = "TIMG_INT_CLR_TIMERS_REG(i)"]
pub mod timg_int_clr_timers_reg;
#[doc = "TIMG_NTIMERS_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timg_ntimers_date_reg](timg_ntimers_date_reg) module"]
pub type TIMG_NTIMERS_DATE_REG = crate::Reg<u32, _TIMG_NTIMERS_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_NTIMERS_DATE_REG;
#[doc = "`read()` method returns [timg_ntimers_date_reg::R](timg_ntimers_date_reg::R) reader structure"]
impl crate::Readable for TIMG_NTIMERS_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [timg_ntimers_date_reg::W](timg_ntimers_date_reg::W) writer structure"]
impl crate::Writable for TIMG_NTIMERS_DATE_REG {}
#[doc = "TIMG_NTIMERS_DATE_REG(i)"]
pub mod timg_ntimers_date_reg;
#[doc = "TIMGCLK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timgclk_reg](timgclk_reg) module"]
pub type TIMGCLK_REG = crate::Reg<u32, _TIMGCLK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMGCLK_REG;
#[doc = "`read()` method returns [timgclk_reg::R](timgclk_reg::R) reader structure"]
impl crate::Readable for TIMGCLK_REG {}
#[doc = "`write(|w| ..)` method takes [timgclk_reg::W](timgclk_reg::W) writer structure"]
impl crate::Writable for TIMGCLK_REG {}
#[doc = "TIMGCLK_REG(i)"]
pub mod timgclk_reg;
