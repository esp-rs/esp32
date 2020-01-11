#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG"]
    pub t0config: T0CONFIG,
    #[doc = "0x04 - TIMG_T0LO"]
    pub t0lo: T0LO,
    #[doc = "0x08 - TIMG_T0HI"]
    pub t0hi: T0HI,
    #[doc = "0x0c - TIMG_T0UPDATE"]
    pub t0update: T0UPDATE,
    #[doc = "0x10 - TIMG_T0ALARMLO"]
    pub t0alarmlo: T0ALARMLO,
    #[doc = "0x14 - TIMG_T0ALARMHI"]
    pub t0alarmhi: T0ALARMHI,
    #[doc = "0x18 - TIMG_T0LOADLO"]
    pub t0loadlo: T0LOADLO,
    #[doc = "0x1c - TIMG_T0LOADHI"]
    pub t0loadhi: T0LOADHI,
    #[doc = "0x20 - TIMG_T0LOAD"]
    pub t0load: T0LOAD,
    #[doc = "0x24 - TIMG_T1CONFIG"]
    pub t1config: T1CONFIG,
    #[doc = "0x28 - TIMG_T1LO"]
    pub t1lo: T1LO,
    #[doc = "0x2c - TIMG_T1HI"]
    pub t1hi: T1HI,
    #[doc = "0x30 - TIMG_T1UPDATE"]
    pub t1update: T1UPDATE,
    #[doc = "0x34 - TIMG_T1ALARMLO"]
    pub t1alarmlo: T1ALARMLO,
    #[doc = "0x38 - TIMG_T1ALARMHI"]
    pub t1alarmhi: T1ALARMHI,
    #[doc = "0x3c - TIMG_T1LOADLO"]
    pub t1loadlo: T1LOADLO,
    #[doc = "0x40 - TIMG_T1LOADHI"]
    pub t1loadhi: T1LOADHI,
    #[doc = "0x44 - TIMG_T1LOAD"]
    pub t1load: T1LOAD,
    #[doc = "0x48 - TIMG_WDTCONFIG0"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x4c - TIMG_WDTCONFIG1"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x50 - TIMG_WDTCONFIG2"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x54 - TIMG_WDTCONFIG3"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x58 - TIMG_WDTCONFIG4"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0x5c - TIMG_WDTCONFIG5"]
    pub wdtconfig5: WDTCONFIG5,
    #[doc = "0x60 - TIMG_WDTFEED"]
    pub wdtfeed: WDTFEED,
    #[doc = "0x64 - TIMG_WDTWPROTECT"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0x68 - TIMG_RTCCALICFG"]
    pub rtccalicfg: RTCCALICFG,
    #[doc = "0x6c - TIMG_RTCCALICFG1"]
    pub rtccalicfg1: RTCCALICFG1,
    #[doc = "0x70 - TIMG_LACTCONFIG"]
    pub lactconfig: LACTCONFIG,
    #[doc = "0x74 - TIMG_LACTRTC"]
    pub lactrtc: LACTRTC,
    #[doc = "0x78 - TIMG_LACTLO"]
    pub lactlo: LACTLO,
    #[doc = "0x7c - TIMG_LACTHI"]
    pub lacthi: LACTHI,
    #[doc = "0x80 - TIMG_LACTUPDATE"]
    pub lactupdate: LACTUPDATE,
    #[doc = "0x84 - TIMG_LACTALARMLO"]
    pub lactalarmlo: LACTALARMLO,
    #[doc = "0x88 - TIMG_LACTALARMHI"]
    pub lactalarmhi: LACTALARMHI,
    #[doc = "0x8c - TIMG_LACTLOADLO"]
    pub lactloadlo: LACTLOADLO,
    #[doc = "0x90 - TIMG_LACTLOADHI"]
    pub lactloadhi: LACTLOADHI,
    #[doc = "0x94 - TIMG_LACTLOAD"]
    pub lactload: LACTLOAD,
    #[doc = "0x98 - TIMG_INT_ENA_TIMERS"]
    pub int_ena_timers: INT_ENA_TIMERS,
    #[doc = "0x9c - TIMG_INT_RAW_TIMERS"]
    pub int_raw_timers: INT_RAW_TIMERS,
    #[doc = "0xa0 - TIMG_INT_ST_TIMERS"]
    pub int_st_timers: INT_ST_TIMERS,
    #[doc = "0xa4 - TIMG_INT_CLR_TIMERS"]
    pub int_clr_timers: INT_CLR_TIMERS,
    _reserved42: [u8; 80usize],
    #[doc = "0xf8 - TIMG_NTIMERS_DATE"]
    pub ntimers_date: NTIMERS_DATE,
    #[doc = "0xfc - TIMGCLK"]
    pub timgclk: TIMGCLK,
}
#[doc = "TIMG_T0CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0config](t0config) module"]
pub type T0CONFIG = crate::Reg<u32, _T0CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CONFIG;
#[doc = "`read()` method returns [t0config::R](t0config::R) reader structure"]
impl crate::Readable for T0CONFIG {}
#[doc = "`write(|w| ..)` method takes [t0config::W](t0config::W) writer structure"]
impl crate::Writable for T0CONFIG {}
#[doc = "TIMG_T0CONFIG"]
pub mod t0config;
#[doc = "TIMG_T0LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0lo](t0lo) module"]
pub type T0LO = crate::Reg<u32, _T0LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LO;
#[doc = "`read()` method returns [t0lo::R](t0lo::R) reader structure"]
impl crate::Readable for T0LO {}
#[doc = "`write(|w| ..)` method takes [t0lo::W](t0lo::W) writer structure"]
impl crate::Writable for T0LO {}
#[doc = "TIMG_T0LO"]
pub mod t0lo;
#[doc = "TIMG_T0HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0hi](t0hi) module"]
pub type T0HI = crate::Reg<u32, _T0HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0HI;
#[doc = "`read()` method returns [t0hi::R](t0hi::R) reader structure"]
impl crate::Readable for T0HI {}
#[doc = "`write(|w| ..)` method takes [t0hi::W](t0hi::W) writer structure"]
impl crate::Writable for T0HI {}
#[doc = "TIMG_T0HI"]
pub mod t0hi;
#[doc = "TIMG_T0UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0update](t0update) module"]
pub type T0UPDATE = crate::Reg<u32, _T0UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0UPDATE;
#[doc = "`read()` method returns [t0update::R](t0update::R) reader structure"]
impl crate::Readable for T0UPDATE {}
#[doc = "`write(|w| ..)` method takes [t0update::W](t0update::W) writer structure"]
impl crate::Writable for T0UPDATE {}
#[doc = "TIMG_T0UPDATE"]
pub mod t0update;
#[doc = "TIMG_T0ALARMLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0alarmlo](t0alarmlo) module"]
pub type T0ALARMLO = crate::Reg<u32, _T0ALARMLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0ALARMLO;
#[doc = "`read()` method returns [t0alarmlo::R](t0alarmlo::R) reader structure"]
impl crate::Readable for T0ALARMLO {}
#[doc = "`write(|w| ..)` method takes [t0alarmlo::W](t0alarmlo::W) writer structure"]
impl crate::Writable for T0ALARMLO {}
#[doc = "TIMG_T0ALARMLO"]
pub mod t0alarmlo;
#[doc = "TIMG_T0ALARMHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0alarmhi](t0alarmhi) module"]
pub type T0ALARMHI = crate::Reg<u32, _T0ALARMHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0ALARMHI;
#[doc = "`read()` method returns [t0alarmhi::R](t0alarmhi::R) reader structure"]
impl crate::Readable for T0ALARMHI {}
#[doc = "`write(|w| ..)` method takes [t0alarmhi::W](t0alarmhi::W) writer structure"]
impl crate::Writable for T0ALARMHI {}
#[doc = "TIMG_T0ALARMHI"]
pub mod t0alarmhi;
#[doc = "TIMG_T0LOADLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0loadlo](t0loadlo) module"]
pub type T0LOADLO = crate::Reg<u32, _T0LOADLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LOADLO;
#[doc = "`read()` method returns [t0loadlo::R](t0loadlo::R) reader structure"]
impl crate::Readable for T0LOADLO {}
#[doc = "`write(|w| ..)` method takes [t0loadlo::W](t0loadlo::W) writer structure"]
impl crate::Writable for T0LOADLO {}
#[doc = "TIMG_T0LOADLO"]
pub mod t0loadlo;
#[doc = "TIMG_T0LOADHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0loadhi](t0loadhi) module"]
pub type T0LOADHI = crate::Reg<u32, _T0LOADHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LOADHI;
#[doc = "`read()` method returns [t0loadhi::R](t0loadhi::R) reader structure"]
impl crate::Readable for T0LOADHI {}
#[doc = "`write(|w| ..)` method takes [t0loadhi::W](t0loadhi::W) writer structure"]
impl crate::Writable for T0LOADHI {}
#[doc = "TIMG_T0LOADHI"]
pub mod t0loadhi;
#[doc = "TIMG_T0LOAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t0load](t0load) module"]
pub type T0LOAD = crate::Reg<u32, _T0LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LOAD;
#[doc = "`read()` method returns [t0load::R](t0load::R) reader structure"]
impl crate::Readable for T0LOAD {}
#[doc = "`write(|w| ..)` method takes [t0load::W](t0load::W) writer structure"]
impl crate::Writable for T0LOAD {}
#[doc = "TIMG_T0LOAD"]
pub mod t0load;
#[doc = "TIMG_T1CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1config](t1config) module"]
pub type T1CONFIG = crate::Reg<u32, _T1CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CONFIG;
#[doc = "`read()` method returns [t1config::R](t1config::R) reader structure"]
impl crate::Readable for T1CONFIG {}
#[doc = "`write(|w| ..)` method takes [t1config::W](t1config::W) writer structure"]
impl crate::Writable for T1CONFIG {}
#[doc = "TIMG_T1CONFIG"]
pub mod t1config;
#[doc = "TIMG_T1LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1lo](t1lo) module"]
pub type T1LO = crate::Reg<u32, _T1LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1LO;
#[doc = "`read()` method returns [t1lo::R](t1lo::R) reader structure"]
impl crate::Readable for T1LO {}
#[doc = "`write(|w| ..)` method takes [t1lo::W](t1lo::W) writer structure"]
impl crate::Writable for T1LO {}
#[doc = "TIMG_T1LO"]
pub mod t1lo;
#[doc = "TIMG_T1HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1hi](t1hi) module"]
pub type T1HI = crate::Reg<u32, _T1HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1HI;
#[doc = "`read()` method returns [t1hi::R](t1hi::R) reader structure"]
impl crate::Readable for T1HI {}
#[doc = "`write(|w| ..)` method takes [t1hi::W](t1hi::W) writer structure"]
impl crate::Writable for T1HI {}
#[doc = "TIMG_T1HI"]
pub mod t1hi;
#[doc = "TIMG_T1UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1update](t1update) module"]
pub type T1UPDATE = crate::Reg<u32, _T1UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1UPDATE;
#[doc = "`read()` method returns [t1update::R](t1update::R) reader structure"]
impl crate::Readable for T1UPDATE {}
#[doc = "`write(|w| ..)` method takes [t1update::W](t1update::W) writer structure"]
impl crate::Writable for T1UPDATE {}
#[doc = "TIMG_T1UPDATE"]
pub mod t1update;
#[doc = "TIMG_T1ALARMLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1alarmlo](t1alarmlo) module"]
pub type T1ALARMLO = crate::Reg<u32, _T1ALARMLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1ALARMLO;
#[doc = "`read()` method returns [t1alarmlo::R](t1alarmlo::R) reader structure"]
impl crate::Readable for T1ALARMLO {}
#[doc = "`write(|w| ..)` method takes [t1alarmlo::W](t1alarmlo::W) writer structure"]
impl crate::Writable for T1ALARMLO {}
#[doc = "TIMG_T1ALARMLO"]
pub mod t1alarmlo;
#[doc = "TIMG_T1ALARMHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1alarmhi](t1alarmhi) module"]
pub type T1ALARMHI = crate::Reg<u32, _T1ALARMHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1ALARMHI;
#[doc = "`read()` method returns [t1alarmhi::R](t1alarmhi::R) reader structure"]
impl crate::Readable for T1ALARMHI {}
#[doc = "`write(|w| ..)` method takes [t1alarmhi::W](t1alarmhi::W) writer structure"]
impl crate::Writable for T1ALARMHI {}
#[doc = "TIMG_T1ALARMHI"]
pub mod t1alarmhi;
#[doc = "TIMG_T1LOADLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1loadlo](t1loadlo) module"]
pub type T1LOADLO = crate::Reg<u32, _T1LOADLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1LOADLO;
#[doc = "`read()` method returns [t1loadlo::R](t1loadlo::R) reader structure"]
impl crate::Readable for T1LOADLO {}
#[doc = "`write(|w| ..)` method takes [t1loadlo::W](t1loadlo::W) writer structure"]
impl crate::Writable for T1LOADLO {}
#[doc = "TIMG_T1LOADLO"]
pub mod t1loadlo;
#[doc = "TIMG_T1LOADHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1loadhi](t1loadhi) module"]
pub type T1LOADHI = crate::Reg<u32, _T1LOADHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1LOADHI;
#[doc = "`read()` method returns [t1loadhi::R](t1loadhi::R) reader structure"]
impl crate::Readable for T1LOADHI {}
#[doc = "`write(|w| ..)` method takes [t1loadhi::W](t1loadhi::W) writer structure"]
impl crate::Writable for T1LOADHI {}
#[doc = "TIMG_T1LOADHI"]
pub mod t1loadhi;
#[doc = "TIMG_T1LOAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t1load](t1load) module"]
pub type T1LOAD = crate::Reg<u32, _T1LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1LOAD;
#[doc = "`read()` method returns [t1load::R](t1load::R) reader structure"]
impl crate::Readable for T1LOAD {}
#[doc = "`write(|w| ..)` method takes [t1load::W](t1load::W) writer structure"]
impl crate::Writable for T1LOAD {}
#[doc = "TIMG_T1LOAD"]
pub mod t1load;
#[doc = "TIMG_WDTCONFIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig0](wdtconfig0) module"]
pub type WDTCONFIG0 = crate::Reg<u32, _WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG0;
#[doc = "`read()` method returns [wdtconfig0::R](wdtconfig0::R) reader structure"]
impl crate::Readable for WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](wdtconfig0::W) writer structure"]
impl crate::Writable for WDTCONFIG0 {}
#[doc = "TIMG_WDTCONFIG0"]
pub mod wdtconfig0;
#[doc = "TIMG_WDTCONFIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig1](wdtconfig1) module"]
pub type WDTCONFIG1 = crate::Reg<u32, _WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG1;
#[doc = "`read()` method returns [wdtconfig1::R](wdtconfig1::R) reader structure"]
impl crate::Readable for WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig1::W](wdtconfig1::W) writer structure"]
impl crate::Writable for WDTCONFIG1 {}
#[doc = "TIMG_WDTCONFIG1"]
pub mod wdtconfig1;
#[doc = "TIMG_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig2](wdtconfig2) module"]
pub type WDTCONFIG2 = crate::Reg<u32, _WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG2;
#[doc = "`read()` method returns [wdtconfig2::R](wdtconfig2::R) reader structure"]
impl crate::Readable for WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig2::W](wdtconfig2::W) writer structure"]
impl crate::Writable for WDTCONFIG2 {}
#[doc = "TIMG_WDTCONFIG2"]
pub mod wdtconfig2;
#[doc = "TIMG_WDTCONFIG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig3](wdtconfig3) module"]
pub type WDTCONFIG3 = crate::Reg<u32, _WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG3;
#[doc = "`read()` method returns [wdtconfig3::R](wdtconfig3::R) reader structure"]
impl crate::Readable for WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig3::W](wdtconfig3::W) writer structure"]
impl crate::Writable for WDTCONFIG3 {}
#[doc = "TIMG_WDTCONFIG3"]
pub mod wdtconfig3;
#[doc = "TIMG_WDTCONFIG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig4](wdtconfig4) module"]
pub type WDTCONFIG4 = crate::Reg<u32, _WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG4;
#[doc = "`read()` method returns [wdtconfig4::R](wdtconfig4::R) reader structure"]
impl crate::Readable for WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig4::W](wdtconfig4::W) writer structure"]
impl crate::Writable for WDTCONFIG4 {}
#[doc = "TIMG_WDTCONFIG4"]
pub mod wdtconfig4;
#[doc = "TIMG_WDTCONFIG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig5](wdtconfig5) module"]
pub type WDTCONFIG5 = crate::Reg<u32, _WDTCONFIG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG5;
#[doc = "`read()` method returns [wdtconfig5::R](wdtconfig5::R) reader structure"]
impl crate::Readable for WDTCONFIG5 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig5::W](wdtconfig5::W) writer structure"]
impl crate::Writable for WDTCONFIG5 {}
#[doc = "TIMG_WDTCONFIG5"]
pub mod wdtconfig5;
#[doc = "TIMG_WDTFEED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtfeed](wdtfeed) module"]
pub type WDTFEED = crate::Reg<u32, _WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTFEED;
#[doc = "`read()` method returns [wdtfeed::R](wdtfeed::R) reader structure"]
impl crate::Readable for WDTFEED {}
#[doc = "`write(|w| ..)` method takes [wdtfeed::W](wdtfeed::W) writer structure"]
impl crate::Writable for WDTFEED {}
#[doc = "TIMG_WDTFEED"]
pub mod wdtfeed;
#[doc = "TIMG_WDTWPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtwprotect](wdtwprotect) module"]
pub type WDTWPROTECT = crate::Reg<u32, _WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTWPROTECT;
#[doc = "`read()` method returns [wdtwprotect::R](wdtwprotect::R) reader structure"]
impl crate::Readable for WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](wdtwprotect::W) writer structure"]
impl crate::Writable for WDTWPROTECT {}
#[doc = "TIMG_WDTWPROTECT"]
pub mod wdtwprotect;
#[doc = "TIMG_RTCCALICFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtccalicfg](rtccalicfg) module"]
pub type RTCCALICFG = crate::Reg<u32, _RTCCALICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCALICFG;
#[doc = "`read()` method returns [rtccalicfg::R](rtccalicfg::R) reader structure"]
impl crate::Readable for RTCCALICFG {}
#[doc = "`write(|w| ..)` method takes [rtccalicfg::W](rtccalicfg::W) writer structure"]
impl crate::Writable for RTCCALICFG {}
#[doc = "TIMG_RTCCALICFG"]
pub mod rtccalicfg;
#[doc = "TIMG_RTCCALICFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtccalicfg1](rtccalicfg1) module"]
pub type RTCCALICFG1 = crate::Reg<u32, _RTCCALICFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCALICFG1;
#[doc = "`read()` method returns [rtccalicfg1::R](rtccalicfg1::R) reader structure"]
impl crate::Readable for RTCCALICFG1 {}
#[doc = "`write(|w| ..)` method takes [rtccalicfg1::W](rtccalicfg1::W) writer structure"]
impl crate::Writable for RTCCALICFG1 {}
#[doc = "TIMG_RTCCALICFG1"]
pub mod rtccalicfg1;
#[doc = "TIMG_LACTCONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactconfig](lactconfig) module"]
pub type LACTCONFIG = crate::Reg<u32, _LACTCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTCONFIG;
#[doc = "`read()` method returns [lactconfig::R](lactconfig::R) reader structure"]
impl crate::Readable for LACTCONFIG {}
#[doc = "`write(|w| ..)` method takes [lactconfig::W](lactconfig::W) writer structure"]
impl crate::Writable for LACTCONFIG {}
#[doc = "TIMG_LACTCONFIG"]
pub mod lactconfig;
#[doc = "TIMG_LACTRTC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactrtc](lactrtc) module"]
pub type LACTRTC = crate::Reg<u32, _LACTRTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTRTC;
#[doc = "`read()` method returns [lactrtc::R](lactrtc::R) reader structure"]
impl crate::Readable for LACTRTC {}
#[doc = "`write(|w| ..)` method takes [lactrtc::W](lactrtc::W) writer structure"]
impl crate::Writable for LACTRTC {}
#[doc = "TIMG_LACTRTC"]
pub mod lactrtc;
#[doc = "TIMG_LACTLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactlo](lactlo) module"]
pub type LACTLO = crate::Reg<u32, _LACTLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTLO;
#[doc = "`read()` method returns [lactlo::R](lactlo::R) reader structure"]
impl crate::Readable for LACTLO {}
#[doc = "`write(|w| ..)` method takes [lactlo::W](lactlo::W) writer structure"]
impl crate::Writable for LACTLO {}
#[doc = "TIMG_LACTLO"]
pub mod lactlo;
#[doc = "TIMG_LACTHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lacthi](lacthi) module"]
pub type LACTHI = crate::Reg<u32, _LACTHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTHI;
#[doc = "`read()` method returns [lacthi::R](lacthi::R) reader structure"]
impl crate::Readable for LACTHI {}
#[doc = "`write(|w| ..)` method takes [lacthi::W](lacthi::W) writer structure"]
impl crate::Writable for LACTHI {}
#[doc = "TIMG_LACTHI"]
pub mod lacthi;
#[doc = "TIMG_LACTUPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactupdate](lactupdate) module"]
pub type LACTUPDATE = crate::Reg<u32, _LACTUPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTUPDATE;
#[doc = "`read()` method returns [lactupdate::R](lactupdate::R) reader structure"]
impl crate::Readable for LACTUPDATE {}
#[doc = "`write(|w| ..)` method takes [lactupdate::W](lactupdate::W) writer structure"]
impl crate::Writable for LACTUPDATE {}
#[doc = "TIMG_LACTUPDATE"]
pub mod lactupdate;
#[doc = "TIMG_LACTALARMLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactalarmlo](lactalarmlo) module"]
pub type LACTALARMLO = crate::Reg<u32, _LACTALARMLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTALARMLO;
#[doc = "`read()` method returns [lactalarmlo::R](lactalarmlo::R) reader structure"]
impl crate::Readable for LACTALARMLO {}
#[doc = "`write(|w| ..)` method takes [lactalarmlo::W](lactalarmlo::W) writer structure"]
impl crate::Writable for LACTALARMLO {}
#[doc = "TIMG_LACTALARMLO"]
pub mod lactalarmlo;
#[doc = "TIMG_LACTALARMHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactalarmhi](lactalarmhi) module"]
pub type LACTALARMHI = crate::Reg<u32, _LACTALARMHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTALARMHI;
#[doc = "`read()` method returns [lactalarmhi::R](lactalarmhi::R) reader structure"]
impl crate::Readable for LACTALARMHI {}
#[doc = "`write(|w| ..)` method takes [lactalarmhi::W](lactalarmhi::W) writer structure"]
impl crate::Writable for LACTALARMHI {}
#[doc = "TIMG_LACTALARMHI"]
pub mod lactalarmhi;
#[doc = "TIMG_LACTLOADLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactloadlo](lactloadlo) module"]
pub type LACTLOADLO = crate::Reg<u32, _LACTLOADLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTLOADLO;
#[doc = "`read()` method returns [lactloadlo::R](lactloadlo::R) reader structure"]
impl crate::Readable for LACTLOADLO {}
#[doc = "`write(|w| ..)` method takes [lactloadlo::W](lactloadlo::W) writer structure"]
impl crate::Writable for LACTLOADLO {}
#[doc = "TIMG_LACTLOADLO"]
pub mod lactloadlo;
#[doc = "TIMG_LACTLOADHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactloadhi](lactloadhi) module"]
pub type LACTLOADHI = crate::Reg<u32, _LACTLOADHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTLOADHI;
#[doc = "`read()` method returns [lactloadhi::R](lactloadhi::R) reader structure"]
impl crate::Readable for LACTLOADHI {}
#[doc = "`write(|w| ..)` method takes [lactloadhi::W](lactloadhi::W) writer structure"]
impl crate::Writable for LACTLOADHI {}
#[doc = "TIMG_LACTLOADHI"]
pub mod lactloadhi;
#[doc = "TIMG_LACTLOAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lactload](lactload) module"]
pub type LACTLOAD = crate::Reg<u32, _LACTLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LACTLOAD;
#[doc = "`read()` method returns [lactload::R](lactload::R) reader structure"]
impl crate::Readable for LACTLOAD {}
#[doc = "`write(|w| ..)` method takes [lactload::W](lactload::W) writer structure"]
impl crate::Writable for LACTLOAD {}
#[doc = "TIMG_LACTLOAD"]
pub mod lactload;
#[doc = "TIMG_INT_ENA_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena_timers](int_ena_timers) module"]
pub type INT_ENA_TIMERS = crate::Reg<u32, _INT_ENA_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA_TIMERS;
#[doc = "`read()` method returns [int_ena_timers::R](int_ena_timers::R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS {}
#[doc = "`write(|w| ..)` method takes [int_ena_timers::W](int_ena_timers::W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS {}
#[doc = "TIMG_INT_ENA_TIMERS"]
pub mod int_ena_timers;
#[doc = "TIMG_INT_RAW_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw_timers](int_raw_timers) module"]
pub type INT_RAW_TIMERS = crate::Reg<u32, _INT_RAW_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW_TIMERS;
#[doc = "`read()` method returns [int_raw_timers::R](int_raw_timers::R) reader structure"]
impl crate::Readable for INT_RAW_TIMERS {}
#[doc = "`write(|w| ..)` method takes [int_raw_timers::W](int_raw_timers::W) writer structure"]
impl crate::Writable for INT_RAW_TIMERS {}
#[doc = "TIMG_INT_RAW_TIMERS"]
pub mod int_raw_timers;
#[doc = "TIMG_INT_ST_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st_timers](int_st_timers) module"]
pub type INT_ST_TIMERS = crate::Reg<u32, _INT_ST_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST_TIMERS;
#[doc = "`read()` method returns [int_st_timers::R](int_st_timers::R) reader structure"]
impl crate::Readable for INT_ST_TIMERS {}
#[doc = "`write(|w| ..)` method takes [int_st_timers::W](int_st_timers::W) writer structure"]
impl crate::Writable for INT_ST_TIMERS {}
#[doc = "TIMG_INT_ST_TIMERS"]
pub mod int_st_timers;
#[doc = "TIMG_INT_CLR_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr_timers](int_clr_timers) module"]
pub type INT_CLR_TIMERS = crate::Reg<u32, _INT_CLR_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR_TIMERS;
#[doc = "`read()` method returns [int_clr_timers::R](int_clr_timers::R) reader structure"]
impl crate::Readable for INT_CLR_TIMERS {}
#[doc = "`write(|w| ..)` method takes [int_clr_timers::W](int_clr_timers::W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS {}
#[doc = "TIMG_INT_CLR_TIMERS"]
pub mod int_clr_timers;
#[doc = "TIMG_NTIMERS_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ntimers_date](ntimers_date) module"]
pub type NTIMERS_DATE = crate::Reg<u32, _NTIMERS_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NTIMERS_DATE;
#[doc = "`read()` method returns [ntimers_date::R](ntimers_date::R) reader structure"]
impl crate::Readable for NTIMERS_DATE {}
#[doc = "`write(|w| ..)` method takes [ntimers_date::W](ntimers_date::W) writer structure"]
impl crate::Writable for NTIMERS_DATE {}
#[doc = "TIMG_NTIMERS_DATE"]
pub mod ntimers_date;
#[doc = "TIMGCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timgclk](timgclk) module"]
pub type TIMGCLK = crate::Reg<u32, _TIMGCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMGCLK;
#[doc = "`read()` method returns [timgclk::R](timgclk::R) reader structure"]
impl crate::Readable for TIMGCLK {}
#[doc = "`write(|w| ..)` method takes [timgclk::W](timgclk::W) writer structure"]
impl crate::Writable for TIMGCLK {}
#[doc = "TIMGCLK"]
pub mod timgclk;
