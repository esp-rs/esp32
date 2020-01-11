#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNTL_OPTIONS0"]
    pub options0: OPTIONS0,
    #[doc = "0x04 - RTC_CNTL_SLP_TIMER0"]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - RTC_CNTL_SLP_TIMER1"]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - RTC_CNTL_TIME_UPDATE"]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - RTC_CNTL_TIME0"]
    pub time0: TIME0,
    #[doc = "0x14 - RTC_CNTL_TIME1"]
    pub time1: TIME1,
    #[doc = "0x18 - RTC_CNTL_STATE0"]
    pub state0: STATE0,
    #[doc = "0x1c - RTC_CNTL_TIMER1"]
    pub timer1: TIMER1,
    #[doc = "0x20 - RTC_CNTL_TIMER2"]
    pub timer2: TIMER2,
    #[doc = "0x24 - RTC_CNTL_TIMER3"]
    pub timer3: TIMER3,
    #[doc = "0x28 - RTC_CNTL_TIMER4"]
    pub timer4: TIMER4,
    #[doc = "0x2c - RTC_CNTL_TIMER5"]
    pub timer5: TIMER5,
    #[doc = "0x30 - RTC_CNTL_ANA_CONF"]
    pub ana_conf: ANA_CONF,
    #[doc = "0x34 - RTC_CNTL_RESET_STATE"]
    pub reset_state: RESET_STATE,
    #[doc = "0x38 - RTC_CNTL_WAKEUP_STATE"]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x3c - RTC_CNTL_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x40 - RTC_CNTL_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x44 - RTC_CNTL_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x48 - RTC_CNTL_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x4c - RTC_CNTL_STORE0"]
    pub store0: STORE0,
    #[doc = "0x50 - RTC_CNTL_STORE1"]
    pub store1: STORE1,
    #[doc = "0x54 - RTC_CNTL_STORE2"]
    pub store2: STORE2,
    #[doc = "0x58 - RTC_CNTL_STORE3"]
    pub store3: STORE3,
    #[doc = "0x5c - RTC_CNTL_EXT_XTL_CONF"]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x60 - RTC_CNTL_EXT_WAKEUP_CONF"]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x64 - RTC_CNTL_SLP_REJECT_CONF"]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x68 - RTC_CNTL_CPU_PERIOD_CONF"]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x6c - RTC_CNTL_SDIO_ACT_CONF"]
    pub sdio_act_conf: SDIO_ACT_CONF,
    #[doc = "0x70 - RTC_CNTL_CLK_CONF"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x74 - RTC_CNTL_SDIO_CONF"]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x78 - RTC_CNTL_BIAS_CONF"]
    pub bias_conf: BIAS_CONF,
    _reserved31: [u8; 4usize],
    #[doc = "0x80 - RTC_CNTL_PWC"]
    pub pwc: PWC,
    #[doc = "0x84 - RTC_CNTL_DIG_PWC"]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x88 - RTC_CNTL_DIG_ISO"]
    pub dig_iso: DIG_ISO,
    #[doc = "0x8c - RTC_CNTL_WDTCONFIG0"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x90 - RTC_CNTL_WDTCONFIG1"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x94 - RTC_CNTL_WDTCONFIG2"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x98 - RTC_CNTL_WDTCONFIG3"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x9c - RTC_CNTL_WDTCONFIG4"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xa0 - RTC_CNTL_WDTFEED"]
    pub wdtfeed: WDTFEED,
    #[doc = "0xa4 - RTC_CNTL_WDTWPROTECT"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xa8 - RTC_CNTL_TEST_MUX"]
    pub test_mux: TEST_MUX,
    #[doc = "0xac - RTC_CNTL_SW_CPU_STALL"]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xb0 - RTC_CNTL_STORE4"]
    pub store4: STORE4,
    #[doc = "0xb4 - RTC_CNTL_STORE5"]
    pub store5: STORE5,
    #[doc = "0xb8 - RTC_CNTL_STORE6"]
    pub store6: STORE6,
    #[doc = "0xbc - RTC_CNTL_STORE7"]
    pub store7: STORE7,
    _reserved47: [u8; 4usize],
    #[doc = "0xc4 - RTC_CNTL_DIAG1"]
    pub diag1: DIAG1,
    #[doc = "0xc8 - RTC_CNTL_HOLD_FORCE"]
    pub hold_force: HOLD_FORCE,
    #[doc = "0xcc - RTC_CNTL_EXT_WAKEUP1"]
    pub ext_wakeup1: EXT_WAKEUP1,
    #[doc = "0xd0 - RTC_CNTL_EXT_WAKEUP1_STATUS"]
    pub ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    #[doc = "0xd4 - RTC_CNTL_BROWN_OUT"]
    pub brown_out: BROWN_OUT,
    _reserved52: [u8; 100usize],
    #[doc = "0x13c - RTC_CNTL_DATE"]
    pub date: DATE,
}
#[doc = "RTC_CNTL_OPTIONS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [options0](options0) module"]
pub type OPTIONS0 = crate::Reg<u32, _OPTIONS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTIONS0;
#[doc = "`read()` method returns [options0::R](options0::R) reader structure"]
impl crate::Readable for OPTIONS0 {}
#[doc = "`write(|w| ..)` method takes [options0::W](options0::W) writer structure"]
impl crate::Writable for OPTIONS0 {}
#[doc = "RTC_CNTL_OPTIONS0"]
pub mod options0;
#[doc = "RTC_CNTL_SLP_TIMER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slp_timer0](slp_timer0) module"]
pub type SLP_TIMER0 = crate::Reg<u32, _SLP_TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_TIMER0;
#[doc = "`read()` method returns [slp_timer0::R](slp_timer0::R) reader structure"]
impl crate::Readable for SLP_TIMER0 {}
#[doc = "`write(|w| ..)` method takes [slp_timer0::W](slp_timer0::W) writer structure"]
impl crate::Writable for SLP_TIMER0 {}
#[doc = "RTC_CNTL_SLP_TIMER0"]
pub mod slp_timer0;
#[doc = "RTC_CNTL_SLP_TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slp_timer1](slp_timer1) module"]
pub type SLP_TIMER1 = crate::Reg<u32, _SLP_TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_TIMER1;
#[doc = "`read()` method returns [slp_timer1::R](slp_timer1::R) reader structure"]
impl crate::Readable for SLP_TIMER1 {}
#[doc = "`write(|w| ..)` method takes [slp_timer1::W](slp_timer1::W) writer structure"]
impl crate::Writable for SLP_TIMER1 {}
#[doc = "RTC_CNTL_SLP_TIMER1"]
pub mod slp_timer1;
#[doc = "RTC_CNTL_TIME_UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [time_update](time_update) module"]
pub type TIME_UPDATE = crate::Reg<u32, _TIME_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME_UPDATE;
#[doc = "`read()` method returns [time_update::R](time_update::R) reader structure"]
impl crate::Readable for TIME_UPDATE {}
#[doc = "`write(|w| ..)` method takes [time_update::W](time_update::W) writer structure"]
impl crate::Writable for TIME_UPDATE {}
#[doc = "RTC_CNTL_TIME_UPDATE"]
pub mod time_update;
#[doc = "RTC_CNTL_TIME0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [time0](time0) module"]
pub type TIME0 = crate::Reg<u32, _TIME0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME0;
#[doc = "`read()` method returns [time0::R](time0::R) reader structure"]
impl crate::Readable for TIME0 {}
#[doc = "`write(|w| ..)` method takes [time0::W](time0::W) writer structure"]
impl crate::Writable for TIME0 {}
#[doc = "RTC_CNTL_TIME0"]
pub mod time0;
#[doc = "RTC_CNTL_TIME1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [time1](time1) module"]
pub type TIME1 = crate::Reg<u32, _TIME1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME1;
#[doc = "`read()` method returns [time1::R](time1::R) reader structure"]
impl crate::Readable for TIME1 {}
#[doc = "`write(|w| ..)` method takes [time1::W](time1::W) writer structure"]
impl crate::Writable for TIME1 {}
#[doc = "RTC_CNTL_TIME1"]
pub mod time1;
#[doc = "RTC_CNTL_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [state0](state0) module"]
pub type STATE0 = crate::Reg<u32, _STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE0;
#[doc = "`read()` method returns [state0::R](state0::R) reader structure"]
impl crate::Readable for STATE0 {}
#[doc = "`write(|w| ..)` method takes [state0::W](state0::W) writer structure"]
impl crate::Writable for STATE0 {}
#[doc = "RTC_CNTL_STATE0"]
pub mod state0;
#[doc = "RTC_CNTL_TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1](timer1) module"]
pub type TIMER1 = crate::Reg<u32, _TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1;
#[doc = "`read()` method returns [timer1::R](timer1::R) reader structure"]
impl crate::Readable for TIMER1 {}
#[doc = "`write(|w| ..)` method takes [timer1::W](timer1::W) writer structure"]
impl crate::Writable for TIMER1 {}
#[doc = "RTC_CNTL_TIMER1"]
pub mod timer1;
#[doc = "RTC_CNTL_TIMER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2](timer2) module"]
pub type TIMER2 = crate::Reg<u32, _TIMER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2;
#[doc = "`read()` method returns [timer2::R](timer2::R) reader structure"]
impl crate::Readable for TIMER2 {}
#[doc = "`write(|w| ..)` method takes [timer2::W](timer2::W) writer structure"]
impl crate::Writable for TIMER2 {}
#[doc = "RTC_CNTL_TIMER2"]
pub mod timer2;
#[doc = "RTC_CNTL_TIMER3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer3](timer3) module"]
pub type TIMER3 = crate::Reg<u32, _TIMER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER3;
#[doc = "`read()` method returns [timer3::R](timer3::R) reader structure"]
impl crate::Readable for TIMER3 {}
#[doc = "`write(|w| ..)` method takes [timer3::W](timer3::W) writer structure"]
impl crate::Writable for TIMER3 {}
#[doc = "RTC_CNTL_TIMER3"]
pub mod timer3;
#[doc = "RTC_CNTL_TIMER4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer4](timer4) module"]
pub type TIMER4 = crate::Reg<u32, _TIMER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER4;
#[doc = "`read()` method returns [timer4::R](timer4::R) reader structure"]
impl crate::Readable for TIMER4 {}
#[doc = "`write(|w| ..)` method takes [timer4::W](timer4::W) writer structure"]
impl crate::Writable for TIMER4 {}
#[doc = "RTC_CNTL_TIMER4"]
pub mod timer4;
#[doc = "RTC_CNTL_TIMER5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer5](timer5) module"]
pub type TIMER5 = crate::Reg<u32, _TIMER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER5;
#[doc = "`read()` method returns [timer5::R](timer5::R) reader structure"]
impl crate::Readable for TIMER5 {}
#[doc = "`write(|w| ..)` method takes [timer5::W](timer5::W) writer structure"]
impl crate::Writable for TIMER5 {}
#[doc = "RTC_CNTL_TIMER5"]
pub mod timer5;
#[doc = "RTC_CNTL_ANA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ana_conf](ana_conf) module"]
pub type ANA_CONF = crate::Reg<u32, _ANA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_CONF;
#[doc = "`read()` method returns [ana_conf::R](ana_conf::R) reader structure"]
impl crate::Readable for ANA_CONF {}
#[doc = "`write(|w| ..)` method takes [ana_conf::W](ana_conf::W) writer structure"]
impl crate::Writable for ANA_CONF {}
#[doc = "RTC_CNTL_ANA_CONF"]
pub mod ana_conf;
#[doc = "RTC_CNTL_RESET_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset_state](reset_state) module"]
pub type RESET_STATE = crate::Reg<u32, _RESET_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_STATE;
#[doc = "`read()` method returns [reset_state::R](reset_state::R) reader structure"]
impl crate::Readable for RESET_STATE {}
#[doc = "`write(|w| ..)` method takes [reset_state::W](reset_state::W) writer structure"]
impl crate::Writable for RESET_STATE {}
#[doc = "RTC_CNTL_RESET_STATE"]
pub mod reset_state;
#[doc = "RTC_CNTL_WAKEUP_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakeup_state](wakeup_state) module"]
pub type WAKEUP_STATE = crate::Reg<u32, _WAKEUP_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP_STATE;
#[doc = "`read()` method returns [wakeup_state::R](wakeup_state::R) reader structure"]
impl crate::Readable for WAKEUP_STATE {}
#[doc = "`write(|w| ..)` method takes [wakeup_state::W](wakeup_state::W) writer structure"]
impl crate::Writable for WAKEUP_STATE {}
#[doc = "RTC_CNTL_WAKEUP_STATE"]
pub mod wakeup_state;
#[doc = "RTC_CNTL_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "RTC_CNTL_INT_ENA"]
pub mod int_ena;
#[doc = "RTC_CNTL_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "RTC_CNTL_INT_RAW"]
pub mod int_raw;
#[doc = "RTC_CNTL_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "RTC_CNTL_INT_ST"]
pub mod int_st;
#[doc = "RTC_CNTL_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "RTC_CNTL_INT_CLR"]
pub mod int_clr;
#[doc = "RTC_CNTL_STORE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store0](store0) module"]
pub type STORE0 = crate::Reg<u32, _STORE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE0;
#[doc = "`read()` method returns [store0::R](store0::R) reader structure"]
impl crate::Readable for STORE0 {}
#[doc = "`write(|w| ..)` method takes [store0::W](store0::W) writer structure"]
impl crate::Writable for STORE0 {}
#[doc = "RTC_CNTL_STORE0"]
pub mod store0;
#[doc = "RTC_CNTL_STORE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store1](store1) module"]
pub type STORE1 = crate::Reg<u32, _STORE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE1;
#[doc = "`read()` method returns [store1::R](store1::R) reader structure"]
impl crate::Readable for STORE1 {}
#[doc = "`write(|w| ..)` method takes [store1::W](store1::W) writer structure"]
impl crate::Writable for STORE1 {}
#[doc = "RTC_CNTL_STORE1"]
pub mod store1;
#[doc = "RTC_CNTL_STORE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store2](store2) module"]
pub type STORE2 = crate::Reg<u32, _STORE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE2;
#[doc = "`read()` method returns [store2::R](store2::R) reader structure"]
impl crate::Readable for STORE2 {}
#[doc = "`write(|w| ..)` method takes [store2::W](store2::W) writer structure"]
impl crate::Writable for STORE2 {}
#[doc = "RTC_CNTL_STORE2"]
pub mod store2;
#[doc = "RTC_CNTL_STORE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store3](store3) module"]
pub type STORE3 = crate::Reg<u32, _STORE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE3;
#[doc = "`read()` method returns [store3::R](store3::R) reader structure"]
impl crate::Readable for STORE3 {}
#[doc = "`write(|w| ..)` method takes [store3::W](store3::W) writer structure"]
impl crate::Writable for STORE3 {}
#[doc = "RTC_CNTL_STORE3"]
pub mod store3;
#[doc = "RTC_CNTL_EXT_XTL_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_xtl_conf](ext_xtl_conf) module"]
pub type EXT_XTL_CONF = crate::Reg<u32, _EXT_XTL_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_XTL_CONF;
#[doc = "`read()` method returns [ext_xtl_conf::R](ext_xtl_conf::R) reader structure"]
impl crate::Readable for EXT_XTL_CONF {}
#[doc = "`write(|w| ..)` method takes [ext_xtl_conf::W](ext_xtl_conf::W) writer structure"]
impl crate::Writable for EXT_XTL_CONF {}
#[doc = "RTC_CNTL_EXT_XTL_CONF"]
pub mod ext_xtl_conf;
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_wakeup_conf](ext_wakeup_conf) module"]
pub type EXT_WAKEUP_CONF = crate::Reg<u32, _EXT_WAKEUP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_WAKEUP_CONF;
#[doc = "`read()` method returns [ext_wakeup_conf::R](ext_wakeup_conf::R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF {}
#[doc = "`write(|w| ..)` method takes [ext_wakeup_conf::W](ext_wakeup_conf::W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF {}
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF"]
pub mod ext_wakeup_conf;
#[doc = "RTC_CNTL_SLP_REJECT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slp_reject_conf](slp_reject_conf) module"]
pub type SLP_REJECT_CONF = crate::Reg<u32, _SLP_REJECT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_REJECT_CONF;
#[doc = "`read()` method returns [slp_reject_conf::R](slp_reject_conf::R) reader structure"]
impl crate::Readable for SLP_REJECT_CONF {}
#[doc = "`write(|w| ..)` method takes [slp_reject_conf::W](slp_reject_conf::W) writer structure"]
impl crate::Writable for SLP_REJECT_CONF {}
#[doc = "RTC_CNTL_SLP_REJECT_CONF"]
pub mod slp_reject_conf;
#[doc = "RTC_CNTL_CPU_PERIOD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu_period_conf](cpu_period_conf) module"]
pub type CPU_PERIOD_CONF = crate::Reg<u32, _CPU_PERIOD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_PERIOD_CONF;
#[doc = "`read()` method returns [cpu_period_conf::R](cpu_period_conf::R) reader structure"]
impl crate::Readable for CPU_PERIOD_CONF {}
#[doc = "`write(|w| ..)` method takes [cpu_period_conf::W](cpu_period_conf::W) writer structure"]
impl crate::Writable for CPU_PERIOD_CONF {}
#[doc = "RTC_CNTL_CPU_PERIOD_CONF"]
pub mod cpu_period_conf;
#[doc = "RTC_CNTL_SDIO_ACT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_act_conf](sdio_act_conf) module"]
pub type SDIO_ACT_CONF = crate::Reg<u32, _SDIO_ACT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_ACT_CONF;
#[doc = "`read()` method returns [sdio_act_conf::R](sdio_act_conf::R) reader structure"]
impl crate::Readable for SDIO_ACT_CONF {}
#[doc = "`write(|w| ..)` method takes [sdio_act_conf::W](sdio_act_conf::W) writer structure"]
impl crate::Writable for SDIO_ACT_CONF {}
#[doc = "RTC_CNTL_SDIO_ACT_CONF"]
pub mod sdio_act_conf;
#[doc = "RTC_CNTL_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_conf](clk_conf) module"]
pub type CLK_CONF = crate::Reg<u32, _CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CONF;
#[doc = "`read()` method returns [clk_conf::R](clk_conf::R) reader structure"]
impl crate::Readable for CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](clk_conf::W) writer structure"]
impl crate::Writable for CLK_CONF {}
#[doc = "RTC_CNTL_CLK_CONF"]
pub mod clk_conf;
#[doc = "RTC_CNTL_SDIO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_conf](sdio_conf) module"]
pub type SDIO_CONF = crate::Reg<u32, _SDIO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_CONF;
#[doc = "`read()` method returns [sdio_conf::R](sdio_conf::R) reader structure"]
impl crate::Readable for SDIO_CONF {}
#[doc = "`write(|w| ..)` method takes [sdio_conf::W](sdio_conf::W) writer structure"]
impl crate::Writable for SDIO_CONF {}
#[doc = "RTC_CNTL_SDIO_CONF"]
pub mod sdio_conf;
#[doc = "RTC_CNTL_BIAS_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bias_conf](bias_conf) module"]
pub type BIAS_CONF = crate::Reg<u32, _BIAS_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIAS_CONF;
#[doc = "`read()` method returns [bias_conf::R](bias_conf::R) reader structure"]
impl crate::Readable for BIAS_CONF {}
#[doc = "`write(|w| ..)` method takes [bias_conf::W](bias_conf::W) writer structure"]
impl crate::Writable for BIAS_CONF {}
#[doc = "RTC_CNTL_BIAS_CONF"]
pub mod bias_conf;
#[doc = "RTC_CNTL_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwc](pwc) module"]
pub type PWC = crate::Reg<u32, _PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWC;
#[doc = "`read()` method returns [pwc::R](pwc::R) reader structure"]
impl crate::Readable for PWC {}
#[doc = "`write(|w| ..)` method takes [pwc::W](pwc::W) writer structure"]
impl crate::Writable for PWC {}
#[doc = "RTC_CNTL_PWC"]
pub mod pwc;
#[doc = "RTC_CNTL_DIG_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dig_pwc](dig_pwc) module"]
pub type DIG_PWC = crate::Reg<u32, _DIG_PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_PWC;
#[doc = "`read()` method returns [dig_pwc::R](dig_pwc::R) reader structure"]
impl crate::Readable for DIG_PWC {}
#[doc = "`write(|w| ..)` method takes [dig_pwc::W](dig_pwc::W) writer structure"]
impl crate::Writable for DIG_PWC {}
#[doc = "RTC_CNTL_DIG_PWC"]
pub mod dig_pwc;
#[doc = "RTC_CNTL_DIG_ISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dig_iso](dig_iso) module"]
pub type DIG_ISO = crate::Reg<u32, _DIG_ISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_ISO;
#[doc = "`read()` method returns [dig_iso::R](dig_iso::R) reader structure"]
impl crate::Readable for DIG_ISO {}
#[doc = "`write(|w| ..)` method takes [dig_iso::W](dig_iso::W) writer structure"]
impl crate::Writable for DIG_ISO {}
#[doc = "RTC_CNTL_DIG_ISO"]
pub mod dig_iso;
#[doc = "RTC_CNTL_WDTCONFIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig0](wdtconfig0) module"]
pub type WDTCONFIG0 = crate::Reg<u32, _WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG0;
#[doc = "`read()` method returns [wdtconfig0::R](wdtconfig0::R) reader structure"]
impl crate::Readable for WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](wdtconfig0::W) writer structure"]
impl crate::Writable for WDTCONFIG0 {}
#[doc = "RTC_CNTL_WDTCONFIG0"]
pub mod wdtconfig0;
#[doc = "RTC_CNTL_WDTCONFIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig1](wdtconfig1) module"]
pub type WDTCONFIG1 = crate::Reg<u32, _WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG1;
#[doc = "`read()` method returns [wdtconfig1::R](wdtconfig1::R) reader structure"]
impl crate::Readable for WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig1::W](wdtconfig1::W) writer structure"]
impl crate::Writable for WDTCONFIG1 {}
#[doc = "RTC_CNTL_WDTCONFIG1"]
pub mod wdtconfig1;
#[doc = "RTC_CNTL_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig2](wdtconfig2) module"]
pub type WDTCONFIG2 = crate::Reg<u32, _WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG2;
#[doc = "`read()` method returns [wdtconfig2::R](wdtconfig2::R) reader structure"]
impl crate::Readable for WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig2::W](wdtconfig2::W) writer structure"]
impl crate::Writable for WDTCONFIG2 {}
#[doc = "RTC_CNTL_WDTCONFIG2"]
pub mod wdtconfig2;
#[doc = "RTC_CNTL_WDTCONFIG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig3](wdtconfig3) module"]
pub type WDTCONFIG3 = crate::Reg<u32, _WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG3;
#[doc = "`read()` method returns [wdtconfig3::R](wdtconfig3::R) reader structure"]
impl crate::Readable for WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig3::W](wdtconfig3::W) writer structure"]
impl crate::Writable for WDTCONFIG3 {}
#[doc = "RTC_CNTL_WDTCONFIG3"]
pub mod wdtconfig3;
#[doc = "RTC_CNTL_WDTCONFIG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtconfig4](wdtconfig4) module"]
pub type WDTCONFIG4 = crate::Reg<u32, _WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG4;
#[doc = "`read()` method returns [wdtconfig4::R](wdtconfig4::R) reader structure"]
impl crate::Readable for WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig4::W](wdtconfig4::W) writer structure"]
impl crate::Writable for WDTCONFIG4 {}
#[doc = "RTC_CNTL_WDTCONFIG4"]
pub mod wdtconfig4;
#[doc = "RTC_CNTL_WDTFEED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtfeed](wdtfeed) module"]
pub type WDTFEED = crate::Reg<u32, _WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTFEED;
#[doc = "`read()` method returns [wdtfeed::R](wdtfeed::R) reader structure"]
impl crate::Readable for WDTFEED {}
#[doc = "`write(|w| ..)` method takes [wdtfeed::W](wdtfeed::W) writer structure"]
impl crate::Writable for WDTFEED {}
#[doc = "RTC_CNTL_WDTFEED"]
pub mod wdtfeed;
#[doc = "RTC_CNTL_WDTWPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtwprotect](wdtwprotect) module"]
pub type WDTWPROTECT = crate::Reg<u32, _WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTWPROTECT;
#[doc = "`read()` method returns [wdtwprotect::R](wdtwprotect::R) reader structure"]
impl crate::Readable for WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](wdtwprotect::W) writer structure"]
impl crate::Writable for WDTWPROTECT {}
#[doc = "RTC_CNTL_WDTWPROTECT"]
pub mod wdtwprotect;
#[doc = "RTC_CNTL_TEST_MUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [test_mux](test_mux) module"]
pub type TEST_MUX = crate::Reg<u32, _TEST_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST_MUX;
#[doc = "`read()` method returns [test_mux::R](test_mux::R) reader structure"]
impl crate::Readable for TEST_MUX {}
#[doc = "`write(|w| ..)` method takes [test_mux::W](test_mux::W) writer structure"]
impl crate::Writable for TEST_MUX {}
#[doc = "RTC_CNTL_TEST_MUX"]
pub mod test_mux;
#[doc = "RTC_CNTL_SW_CPU_STALL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_cpu_stall](sw_cpu_stall) module"]
pub type SW_CPU_STALL = crate::Reg<u32, _SW_CPU_STALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CPU_STALL;
#[doc = "`read()` method returns [sw_cpu_stall::R](sw_cpu_stall::R) reader structure"]
impl crate::Readable for SW_CPU_STALL {}
#[doc = "`write(|w| ..)` method takes [sw_cpu_stall::W](sw_cpu_stall::W) writer structure"]
impl crate::Writable for SW_CPU_STALL {}
#[doc = "RTC_CNTL_SW_CPU_STALL"]
pub mod sw_cpu_stall;
#[doc = "RTC_CNTL_STORE4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store4](store4) module"]
pub type STORE4 = crate::Reg<u32, _STORE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE4;
#[doc = "`read()` method returns [store4::R](store4::R) reader structure"]
impl crate::Readable for STORE4 {}
#[doc = "`write(|w| ..)` method takes [store4::W](store4::W) writer structure"]
impl crate::Writable for STORE4 {}
#[doc = "RTC_CNTL_STORE4"]
pub mod store4;
#[doc = "RTC_CNTL_STORE5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store5](store5) module"]
pub type STORE5 = crate::Reg<u32, _STORE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE5;
#[doc = "`read()` method returns [store5::R](store5::R) reader structure"]
impl crate::Readable for STORE5 {}
#[doc = "`write(|w| ..)` method takes [store5::W](store5::W) writer structure"]
impl crate::Writable for STORE5 {}
#[doc = "RTC_CNTL_STORE5"]
pub mod store5;
#[doc = "RTC_CNTL_STORE6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store6](store6) module"]
pub type STORE6 = crate::Reg<u32, _STORE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE6;
#[doc = "`read()` method returns [store6::R](store6::R) reader structure"]
impl crate::Readable for STORE6 {}
#[doc = "`write(|w| ..)` method takes [store6::W](store6::W) writer structure"]
impl crate::Writable for STORE6 {}
#[doc = "RTC_CNTL_STORE6"]
pub mod store6;
#[doc = "RTC_CNTL_STORE7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [store7](store7) module"]
pub type STORE7 = crate::Reg<u32, _STORE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE7;
#[doc = "`read()` method returns [store7::R](store7::R) reader structure"]
impl crate::Readable for STORE7 {}
#[doc = "`write(|w| ..)` method takes [store7::W](store7::W) writer structure"]
impl crate::Writable for STORE7 {}
#[doc = "RTC_CNTL_STORE7"]
pub mod store7;
#[doc = "RTC_CNTL_DIAG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diag1](diag1) module"]
pub type DIAG1 = crate::Reg<u32, _DIAG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIAG1;
#[doc = "`read()` method returns [diag1::R](diag1::R) reader structure"]
impl crate::Readable for DIAG1 {}
#[doc = "`write(|w| ..)` method takes [diag1::W](diag1::W) writer structure"]
impl crate::Writable for DIAG1 {}
#[doc = "RTC_CNTL_DIAG1"]
pub mod diag1;
#[doc = "RTC_CNTL_HOLD_FORCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hold_force](hold_force) module"]
pub type HOLD_FORCE = crate::Reg<u32, _HOLD_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOLD_FORCE;
#[doc = "`read()` method returns [hold_force::R](hold_force::R) reader structure"]
impl crate::Readable for HOLD_FORCE {}
#[doc = "`write(|w| ..)` method takes [hold_force::W](hold_force::W) writer structure"]
impl crate::Writable for HOLD_FORCE {}
#[doc = "RTC_CNTL_HOLD_FORCE"]
pub mod hold_force;
#[doc = "RTC_CNTL_EXT_WAKEUP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_wakeup1](ext_wakeup1) module"]
pub type EXT_WAKEUP1 = crate::Reg<u32, _EXT_WAKEUP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_WAKEUP1;
#[doc = "`read()` method returns [ext_wakeup1::R](ext_wakeup1::R) reader structure"]
impl crate::Readable for EXT_WAKEUP1 {}
#[doc = "`write(|w| ..)` method takes [ext_wakeup1::W](ext_wakeup1::W) writer structure"]
impl crate::Writable for EXT_WAKEUP1 {}
#[doc = "RTC_CNTL_EXT_WAKEUP1"]
pub mod ext_wakeup1;
#[doc = "RTC_CNTL_EXT_WAKEUP1_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_wakeup1_status](ext_wakeup1_status) module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<u32, _EXT_WAKEUP1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_WAKEUP1_STATUS;
#[doc = "`read()` method returns [ext_wakeup1_status::R](ext_wakeup1_status::R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_STATUS {}
#[doc = "`write(|w| ..)` method takes [ext_wakeup1_status::W](ext_wakeup1_status::W) writer structure"]
impl crate::Writable for EXT_WAKEUP1_STATUS {}
#[doc = "RTC_CNTL_EXT_WAKEUP1_STATUS"]
pub mod ext_wakeup1_status;
#[doc = "RTC_CNTL_BROWN_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [brown_out](brown_out) module"]
pub type BROWN_OUT = crate::Reg<u32, _BROWN_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BROWN_OUT;
#[doc = "`read()` method returns [brown_out::R](brown_out::R) reader structure"]
impl crate::Readable for BROWN_OUT {}
#[doc = "`write(|w| ..)` method takes [brown_out::W](brown_out::W) writer structure"]
impl crate::Writable for BROWN_OUT {}
#[doc = "RTC_CNTL_BROWN_OUT"]
pub mod brown_out;
#[doc = "RTC_CNTL_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "RTC_CNTL_DATE"]
pub mod date;
