#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNTL_OPTIONS0_REG"]
    pub rtc_cntl_options0: RTC_CNTL_OPTIONS0,
    #[doc = "0x04 - RTC_CNTL_SLP_TIMER0_REG"]
    pub rtc_cntl_slp_timer0: RTC_CNTL_SLP_TIMER0,
    #[doc = "0x08 - RTC_CNTL_SLP_TIMER1_REG"]
    pub rtc_cntl_slp_timer1: RTC_CNTL_SLP_TIMER1,
    #[doc = "0x0c - RTC_CNTL_TIME_UPDATE_REG"]
    pub rtc_cntl_time_update: RTC_CNTL_TIME_UPDATE,
    #[doc = "0x10 - RTC_CNTL_TIME0_REG"]
    pub rtc_cntl_time0: RTC_CNTL_TIME0,
    #[doc = "0x14 - RTC_CNTL_TIME1_REG"]
    pub rtc_cntl_time1: RTC_CNTL_TIME1,
    #[doc = "0x18 - RTC_CNTL_STATE0_REG"]
    pub rtc_cntl_state0: RTC_CNTL_STATE0,
    #[doc = "0x1c - RTC_CNTL_TIMER1_REG"]
    pub rtc_cntl_timer1: RTC_CNTL_TIMER1,
    #[doc = "0x20 - RTC_CNTL_TIMER2_REG"]
    pub rtc_cntl_timer2: RTC_CNTL_TIMER2,
    #[doc = "0x24 - RTC_CNTL_TIMER3_REG"]
    pub rtc_cntl_timer3: RTC_CNTL_TIMER3,
    #[doc = "0x28 - RTC_CNTL_TIMER4_REG"]
    pub rtc_cntl_timer4: RTC_CNTL_TIMER4,
    #[doc = "0x2c - RTC_CNTL_TIMER5_REG"]
    pub rtc_cntl_timer5: RTC_CNTL_TIMER5,
    #[doc = "0x30 - RTC_CNTL_ANA_CONF_REG"]
    pub rtc_cntl_ana_conf: RTC_CNTL_ANA_CONF,
    #[doc = "0x34 - RTC_CNTL_RESET_STATE_REG"]
    pub rtc_cntl_reset_state: RTC_CNTL_RESET_STATE,
    #[doc = "0x38 - RTC_CNTL_WAKEUP_STATE_REG"]
    pub rtc_cntl_wakeup_state: RTC_CNTL_WAKEUP_STATE,
    #[doc = "0x3c - RTC_CNTL_INT_ENA_REG"]
    pub rtc_cntl_int_ena: RTC_CNTL_INT_ENA,
    #[doc = "0x40 - RTC_CNTL_INT_RAW_REG"]
    pub rtc_cntl_int_raw: RTC_CNTL_INT_RAW,
    #[doc = "0x44 - RTC_CNTL_INT_ST_REG"]
    pub rtc_cntl_int_st: RTC_CNTL_INT_ST,
    #[doc = "0x48 - RTC_CNTL_INT_CLR_REG"]
    pub rtc_cntl_int_clr: RTC_CNTL_INT_CLR,
    #[doc = "0x4c - RTC_CNTL_STORE0_REG"]
    pub rtc_cntl_store0: RTC_CNTL_STORE0,
    #[doc = "0x50 - RTC_CNTL_STORE1_REG"]
    pub rtc_cntl_store1: RTC_CNTL_STORE1,
    #[doc = "0x54 - RTC_CNTL_STORE2_REG"]
    pub rtc_cntl_store2: RTC_CNTL_STORE2,
    #[doc = "0x58 - RTC_CNTL_STORE3_REG"]
    pub rtc_cntl_store3: RTC_CNTL_STORE3,
    #[doc = "0x5c - RTC_CNTL_EXT_XTL_CONF_REG"]
    pub rtc_cntl_ext_xtl_conf: RTC_CNTL_EXT_XTL_CONF,
    #[doc = "0x60 - RTC_CNTL_EXT_WAKEUP_CONF_REG"]
    pub rtc_cntl_ext_wakeup_conf: RTC_CNTL_EXT_WAKEUP_CONF,
    #[doc = "0x64 - RTC_CNTL_SLP_REJECT_CONF_REG"]
    pub rtc_cntl_slp_reject_conf: RTC_CNTL_SLP_REJECT_CONF,
    #[doc = "0x68 - RTC_CNTL_CPU_PERIOD_CONF_REG"]
    pub rtc_cntl_cpu_period_conf: RTC_CNTL_CPU_PERIOD_CONF,
    #[doc = "0x6c - RTC_CNTL_SDIO_ACT_CONF_REG"]
    pub rtc_cntl_sdio_act_conf: RTC_CNTL_SDIO_ACT_CONF,
    #[doc = "0x70 - RTC_CNTL_CLK_CONF_REG"]
    pub rtc_cntl_clk_conf: RTC_CNTL_CLK_CONF,
    #[doc = "0x74 - RTC_CNTL_SDIO_CONF_REG"]
    pub rtc_cntl_sdio_conf: RTC_CNTL_SDIO_CONF,
    #[doc = "0x78 - RTC_CNTL_BIAS_CONF_REG"]
    pub rtc_cntl_bias_conf: RTC_CNTL_BIAS_CONF,
    _reserved31: [u8; 4usize],
    #[doc = "0x80 - RTC_CNTL_PWC_REG"]
    pub rtc_cntl_pwc: RTC_CNTL_PWC,
    #[doc = "0x84 - RTC_CNTL_DIG_PWC_REG"]
    pub rtc_cntl_dig_pwc: RTC_CNTL_DIG_PWC,
    #[doc = "0x88 - RTC_CNTL_DIG_ISO_REG"]
    pub rtc_cntl_dig_iso: RTC_CNTL_DIG_ISO,
    #[doc = "0x8c - RTC_CNTL_WDTCONFIG0_REG"]
    pub rtc_cntl_wdtconfig0: RTC_CNTL_WDTCONFIG0,
    #[doc = "0x90 - RTC_CNTL_WDTCONFIG1_REG"]
    pub rtc_cntl_wdtconfig1: RTC_CNTL_WDTCONFIG1,
    #[doc = "0x94 - RTC_CNTL_WDTCONFIG2_REG"]
    pub rtc_cntl_wdtconfig2: RTC_CNTL_WDTCONFIG2,
    #[doc = "0x98 - RTC_CNTL_WDTCONFIG3_REG"]
    pub rtc_cntl_wdtconfig3: RTC_CNTL_WDTCONFIG3,
    #[doc = "0x9c - RTC_CNTL_WDTCONFIG4_REG"]
    pub rtc_cntl_wdtconfig4: RTC_CNTL_WDTCONFIG4,
    #[doc = "0xa0 - RTC_CNTL_WDTFEED_REG"]
    pub rtc_cntl_wdtfeed: RTC_CNTL_WDTFEED,
    #[doc = "0xa4 - RTC_CNTL_WDTWPROTECT_REG"]
    pub rtc_cntl_wdtwprotect: RTC_CNTL_WDTWPROTECT,
    #[doc = "0xa8 - RTC_CNTL_TEST_MUX_REG"]
    pub rtc_cntl_test_mux: RTC_CNTL_TEST_MUX,
    #[doc = "0xac - RTC_CNTL_SW_CPU_STALL_REG"]
    pub rtc_cntl_sw_cpu_stall: RTC_CNTL_SW_CPU_STALL,
    #[doc = "0xb0 - RTC_CNTL_STORE4_REG"]
    pub rtc_cntl_store4: RTC_CNTL_STORE4,
    #[doc = "0xb4 - RTC_CNTL_STORE5_REG"]
    pub rtc_cntl_store5: RTC_CNTL_STORE5,
    #[doc = "0xb8 - RTC_CNTL_STORE6_REG"]
    pub rtc_cntl_store6: RTC_CNTL_STORE6,
    #[doc = "0xbc - RTC_CNTL_STORE7_REG"]
    pub rtc_cntl_store7: RTC_CNTL_STORE7,
    _reserved47: [u8; 4usize],
    #[doc = "0xc4 - RTC_CNTL_DIAG1_REG"]
    pub rtc_cntl_diag1: RTC_CNTL_DIAG1,
    #[doc = "0xc8 - RTC_CNTL_HOLD_FORCE_REG"]
    pub rtc_cntl_hold_force: RTC_CNTL_HOLD_FORCE,
    #[doc = "0xcc - RTC_CNTL_EXT_WAKEUP1_REG"]
    pub rtc_cntl_ext_wakeup1: RTC_CNTL_EXT_WAKEUP1,
    #[doc = "0xd0 - RTC_CNTL_EXT_WAKEUP1_STATUS_REG"]
    pub rtc_cntl_ext_wakeup1_status: RTC_CNTL_EXT_WAKEUP1_STATUS,
    #[doc = "0xd4 - RTC_CNTL_BROWN_OUT_REG"]
    pub rtc_cntl_brown_out: RTC_CNTL_BROWN_OUT,
    _reserved52: [u8; 100usize],
    #[doc = "0x13c - RTC_CNTL_DATE_REG"]
    pub rtc_cntl_date: RTC_CNTL_DATE,
}
#[doc = "RTC_CNTL_OPTIONS0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_options0](rtc_cntl_options0) module"]
pub type RTC_CNTL_OPTIONS0 = crate::Reg<u32, _RTC_CNTL_OPTIONS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_OPTIONS0;
#[doc = "`read()` method returns [rtc_cntl_options0::R](rtc_cntl_options0::R) reader structure"]
impl crate::Readable for RTC_CNTL_OPTIONS0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_options0::W](rtc_cntl_options0::W) writer structure"]
impl crate::Writable for RTC_CNTL_OPTIONS0 {}
#[doc = "RTC_CNTL_OPTIONS0_REG"]
pub mod rtc_cntl_options0;
#[doc = "RTC_CNTL_SLP_TIMER0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_slp_timer0](rtc_cntl_slp_timer0) module"]
pub type RTC_CNTL_SLP_TIMER0 = crate::Reg<u32, _RTC_CNTL_SLP_TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_TIMER0;
#[doc = "`read()` method returns [rtc_cntl_slp_timer0::R](rtc_cntl_slp_timer0::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_TIMER0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_timer0::W](rtc_cntl_slp_timer0::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_TIMER0 {}
#[doc = "RTC_CNTL_SLP_TIMER0_REG"]
pub mod rtc_cntl_slp_timer0;
#[doc = "RTC_CNTL_SLP_TIMER1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_slp_timer1](rtc_cntl_slp_timer1) module"]
pub type RTC_CNTL_SLP_TIMER1 = crate::Reg<u32, _RTC_CNTL_SLP_TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_TIMER1;
#[doc = "`read()` method returns [rtc_cntl_slp_timer1::R](rtc_cntl_slp_timer1::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_TIMER1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_timer1::W](rtc_cntl_slp_timer1::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_TIMER1 {}
#[doc = "RTC_CNTL_SLP_TIMER1_REG"]
pub mod rtc_cntl_slp_timer1;
#[doc = "RTC_CNTL_TIME_UPDATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_time_update](rtc_cntl_time_update) module"]
pub type RTC_CNTL_TIME_UPDATE = crate::Reg<u32, _RTC_CNTL_TIME_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_UPDATE;
#[doc = "`read()` method returns [rtc_cntl_time_update::R](rtc_cntl_time_update::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_UPDATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time_update::W](rtc_cntl_time_update::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME_UPDATE {}
#[doc = "RTC_CNTL_TIME_UPDATE_REG"]
pub mod rtc_cntl_time_update;
#[doc = "RTC_CNTL_TIME0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_time0](rtc_cntl_time0) module"]
pub type RTC_CNTL_TIME0 = crate::Reg<u32, _RTC_CNTL_TIME0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME0;
#[doc = "`read()` method returns [rtc_cntl_time0::R](rtc_cntl_time0::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time0::W](rtc_cntl_time0::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME0 {}
#[doc = "RTC_CNTL_TIME0_REG"]
pub mod rtc_cntl_time0;
#[doc = "RTC_CNTL_TIME1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_time1](rtc_cntl_time1) module"]
pub type RTC_CNTL_TIME1 = crate::Reg<u32, _RTC_CNTL_TIME1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME1;
#[doc = "`read()` method returns [rtc_cntl_time1::R](rtc_cntl_time1::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time1::W](rtc_cntl_time1::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME1 {}
#[doc = "RTC_CNTL_TIME1_REG"]
pub mod rtc_cntl_time1;
#[doc = "RTC_CNTL_STATE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_state0](rtc_cntl_state0) module"]
pub type RTC_CNTL_STATE0 = crate::Reg<u32, _RTC_CNTL_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STATE0;
#[doc = "`read()` method returns [rtc_cntl_state0::R](rtc_cntl_state0::R) reader structure"]
impl crate::Readable for RTC_CNTL_STATE0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_state0::W](rtc_cntl_state0::W) writer structure"]
impl crate::Writable for RTC_CNTL_STATE0 {}
#[doc = "RTC_CNTL_STATE0_REG"]
pub mod rtc_cntl_state0;
#[doc = "RTC_CNTL_TIMER1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer1](rtc_cntl_timer1) module"]
pub type RTC_CNTL_TIMER1 = crate::Reg<u32, _RTC_CNTL_TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER1;
#[doc = "`read()` method returns [rtc_cntl_timer1::R](rtc_cntl_timer1::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer1::W](rtc_cntl_timer1::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER1 {}
#[doc = "RTC_CNTL_TIMER1_REG"]
pub mod rtc_cntl_timer1;
#[doc = "RTC_CNTL_TIMER2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer2](rtc_cntl_timer2) module"]
pub type RTC_CNTL_TIMER2 = crate::Reg<u32, _RTC_CNTL_TIMER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER2;
#[doc = "`read()` method returns [rtc_cntl_timer2::R](rtc_cntl_timer2::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER2 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer2::W](rtc_cntl_timer2::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER2 {}
#[doc = "RTC_CNTL_TIMER2_REG"]
pub mod rtc_cntl_timer2;
#[doc = "RTC_CNTL_TIMER3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer3](rtc_cntl_timer3) module"]
pub type RTC_CNTL_TIMER3 = crate::Reg<u32, _RTC_CNTL_TIMER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER3;
#[doc = "`read()` method returns [rtc_cntl_timer3::R](rtc_cntl_timer3::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER3 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer3::W](rtc_cntl_timer3::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER3 {}
#[doc = "RTC_CNTL_TIMER3_REG"]
pub mod rtc_cntl_timer3;
#[doc = "RTC_CNTL_TIMER4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer4](rtc_cntl_timer4) module"]
pub type RTC_CNTL_TIMER4 = crate::Reg<u32, _RTC_CNTL_TIMER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER4;
#[doc = "`read()` method returns [rtc_cntl_timer4::R](rtc_cntl_timer4::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER4 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer4::W](rtc_cntl_timer4::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER4 {}
#[doc = "RTC_CNTL_TIMER4_REG"]
pub mod rtc_cntl_timer4;
#[doc = "RTC_CNTL_TIMER5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer5](rtc_cntl_timer5) module"]
pub type RTC_CNTL_TIMER5 = crate::Reg<u32, _RTC_CNTL_TIMER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER5;
#[doc = "`read()` method returns [rtc_cntl_timer5::R](rtc_cntl_timer5::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER5 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer5::W](rtc_cntl_timer5::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER5 {}
#[doc = "RTC_CNTL_TIMER5_REG"]
pub mod rtc_cntl_timer5;
#[doc = "RTC_CNTL_ANA_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ana_conf](rtc_cntl_ana_conf) module"]
pub type RTC_CNTL_ANA_CONF = crate::Reg<u32, _RTC_CNTL_ANA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_ANA_CONF;
#[doc = "`read()` method returns [rtc_cntl_ana_conf::R](rtc_cntl_ana_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_ANA_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ana_conf::W](rtc_cntl_ana_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_ANA_CONF {}
#[doc = "RTC_CNTL_ANA_CONF_REG"]
pub mod rtc_cntl_ana_conf;
#[doc = "RTC_CNTL_RESET_STATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_reset_state](rtc_cntl_reset_state) module"]
pub type RTC_CNTL_RESET_STATE = crate::Reg<u32, _RTC_CNTL_RESET_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_RESET_STATE;
#[doc = "`read()` method returns [rtc_cntl_reset_state::R](rtc_cntl_reset_state::R) reader structure"]
impl crate::Readable for RTC_CNTL_RESET_STATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_reset_state::W](rtc_cntl_reset_state::W) writer structure"]
impl crate::Writable for RTC_CNTL_RESET_STATE {}
#[doc = "RTC_CNTL_RESET_STATE_REG"]
pub mod rtc_cntl_reset_state;
#[doc = "RTC_CNTL_WAKEUP_STATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wakeup_state](rtc_cntl_wakeup_state) module"]
pub type RTC_CNTL_WAKEUP_STATE = crate::Reg<u32, _RTC_CNTL_WAKEUP_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WAKEUP_STATE;
#[doc = "`read()` method returns [rtc_cntl_wakeup_state::R](rtc_cntl_wakeup_state::R) reader structure"]
impl crate::Readable for RTC_CNTL_WAKEUP_STATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wakeup_state::W](rtc_cntl_wakeup_state::W) writer structure"]
impl crate::Writable for RTC_CNTL_WAKEUP_STATE {}
#[doc = "RTC_CNTL_WAKEUP_STATE_REG"]
pub mod rtc_cntl_wakeup_state;
#[doc = "RTC_CNTL_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_ena](rtc_cntl_int_ena) module"]
pub type RTC_CNTL_INT_ENA = crate::Reg<u32, _RTC_CNTL_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ENA;
#[doc = "`read()` method returns [rtc_cntl_int_ena::R](rtc_cntl_int_ena::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_ena::W](rtc_cntl_int_ena::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ENA {}
#[doc = "RTC_CNTL_INT_ENA_REG"]
pub mod rtc_cntl_int_ena;
#[doc = "RTC_CNTL_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_raw](rtc_cntl_int_raw) module"]
pub type RTC_CNTL_INT_RAW = crate::Reg<u32, _RTC_CNTL_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_RAW;
#[doc = "`read()` method returns [rtc_cntl_int_raw::R](rtc_cntl_int_raw::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_raw::W](rtc_cntl_int_raw::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_RAW {}
#[doc = "RTC_CNTL_INT_RAW_REG"]
pub mod rtc_cntl_int_raw;
#[doc = "RTC_CNTL_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_st](rtc_cntl_int_st) module"]
pub type RTC_CNTL_INT_ST = crate::Reg<u32, _RTC_CNTL_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ST;
#[doc = "`read()` method returns [rtc_cntl_int_st::R](rtc_cntl_int_st::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_ST {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_st::W](rtc_cntl_int_st::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ST {}
#[doc = "RTC_CNTL_INT_ST_REG"]
pub mod rtc_cntl_int_st;
#[doc = "RTC_CNTL_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_clr](rtc_cntl_int_clr) module"]
pub type RTC_CNTL_INT_CLR = crate::Reg<u32, _RTC_CNTL_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_CLR;
#[doc = "`read()` method returns [rtc_cntl_int_clr::R](rtc_cntl_int_clr::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_CLR {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_clr::W](rtc_cntl_int_clr::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_CLR {}
#[doc = "RTC_CNTL_INT_CLR_REG"]
pub mod rtc_cntl_int_clr;
#[doc = "RTC_CNTL_STORE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store0](rtc_cntl_store0) module"]
pub type RTC_CNTL_STORE0 = crate::Reg<u32, _RTC_CNTL_STORE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE0;
#[doc = "`read()` method returns [rtc_cntl_store0::R](rtc_cntl_store0::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store0::W](rtc_cntl_store0::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE0 {}
#[doc = "RTC_CNTL_STORE0_REG"]
pub mod rtc_cntl_store0;
#[doc = "RTC_CNTL_STORE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store1](rtc_cntl_store1) module"]
pub type RTC_CNTL_STORE1 = crate::Reg<u32, _RTC_CNTL_STORE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE1;
#[doc = "`read()` method returns [rtc_cntl_store1::R](rtc_cntl_store1::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store1::W](rtc_cntl_store1::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE1 {}
#[doc = "RTC_CNTL_STORE1_REG"]
pub mod rtc_cntl_store1;
#[doc = "RTC_CNTL_STORE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store2](rtc_cntl_store2) module"]
pub type RTC_CNTL_STORE2 = crate::Reg<u32, _RTC_CNTL_STORE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE2;
#[doc = "`read()` method returns [rtc_cntl_store2::R](rtc_cntl_store2::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE2 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store2::W](rtc_cntl_store2::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE2 {}
#[doc = "RTC_CNTL_STORE2_REG"]
pub mod rtc_cntl_store2;
#[doc = "RTC_CNTL_STORE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store3](rtc_cntl_store3) module"]
pub type RTC_CNTL_STORE3 = crate::Reg<u32, _RTC_CNTL_STORE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE3;
#[doc = "`read()` method returns [rtc_cntl_store3::R](rtc_cntl_store3::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE3 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store3::W](rtc_cntl_store3::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE3 {}
#[doc = "RTC_CNTL_STORE3_REG"]
pub mod rtc_cntl_store3;
#[doc = "RTC_CNTL_EXT_XTL_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_xtl_conf](rtc_cntl_ext_xtl_conf) module"]
pub type RTC_CNTL_EXT_XTL_CONF = crate::Reg<u32, _RTC_CNTL_EXT_XTL_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_XTL_CONF;
#[doc = "`read()` method returns [rtc_cntl_ext_xtl_conf::R](rtc_cntl_ext_xtl_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_XTL_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_xtl_conf::W](rtc_cntl_ext_xtl_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_XTL_CONF {}
#[doc = "RTC_CNTL_EXT_XTL_CONF_REG"]
pub mod rtc_cntl_ext_xtl_conf;
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_wakeup_conf](rtc_cntl_ext_wakeup_conf) module"]
pub type RTC_CNTL_EXT_WAKEUP_CONF = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP_CONF;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup_conf::R](rtc_cntl_ext_wakeup_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup_conf::W](rtc_cntl_ext_wakeup_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP_CONF {}
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF_REG"]
pub mod rtc_cntl_ext_wakeup_conf;
#[doc = "RTC_CNTL_SLP_REJECT_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_slp_reject_conf](rtc_cntl_slp_reject_conf) module"]
pub type RTC_CNTL_SLP_REJECT_CONF = crate::Reg<u32, _RTC_CNTL_SLP_REJECT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_REJECT_CONF;
#[doc = "`read()` method returns [rtc_cntl_slp_reject_conf::R](rtc_cntl_slp_reject_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_REJECT_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_reject_conf::W](rtc_cntl_slp_reject_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_REJECT_CONF {}
#[doc = "RTC_CNTL_SLP_REJECT_CONF_REG"]
pub mod rtc_cntl_slp_reject_conf;
#[doc = "RTC_CNTL_CPU_PERIOD_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_cpu_period_conf](rtc_cntl_cpu_period_conf) module"]
pub type RTC_CNTL_CPU_PERIOD_CONF = crate::Reg<u32, _RTC_CNTL_CPU_PERIOD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_CPU_PERIOD_CONF;
#[doc = "`read()` method returns [rtc_cntl_cpu_period_conf::R](rtc_cntl_cpu_period_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_CPU_PERIOD_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_cpu_period_conf::W](rtc_cntl_cpu_period_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_CPU_PERIOD_CONF {}
#[doc = "RTC_CNTL_CPU_PERIOD_CONF_REG"]
pub mod rtc_cntl_cpu_period_conf;
#[doc = "RTC_CNTL_SDIO_ACT_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_sdio_act_conf](rtc_cntl_sdio_act_conf) module"]
pub type RTC_CNTL_SDIO_ACT_CONF = crate::Reg<u32, _RTC_CNTL_SDIO_ACT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SDIO_ACT_CONF;
#[doc = "`read()` method returns [rtc_cntl_sdio_act_conf::R](rtc_cntl_sdio_act_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SDIO_ACT_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sdio_act_conf::W](rtc_cntl_sdio_act_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SDIO_ACT_CONF {}
#[doc = "RTC_CNTL_SDIO_ACT_CONF_REG"]
pub mod rtc_cntl_sdio_act_conf;
#[doc = "RTC_CNTL_CLK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_clk_conf](rtc_cntl_clk_conf) module"]
pub type RTC_CNTL_CLK_CONF = crate::Reg<u32, _RTC_CNTL_CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_CLK_CONF;
#[doc = "`read()` method returns [rtc_cntl_clk_conf::R](rtc_cntl_clk_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_clk_conf::W](rtc_cntl_clk_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_CLK_CONF {}
#[doc = "RTC_CNTL_CLK_CONF_REG"]
pub mod rtc_cntl_clk_conf;
#[doc = "RTC_CNTL_SDIO_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_sdio_conf](rtc_cntl_sdio_conf) module"]
pub type RTC_CNTL_SDIO_CONF = crate::Reg<u32, _RTC_CNTL_SDIO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SDIO_CONF;
#[doc = "`read()` method returns [rtc_cntl_sdio_conf::R](rtc_cntl_sdio_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SDIO_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sdio_conf::W](rtc_cntl_sdio_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SDIO_CONF {}
#[doc = "RTC_CNTL_SDIO_CONF_REG"]
pub mod rtc_cntl_sdio_conf;
#[doc = "RTC_CNTL_BIAS_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_bias_conf](rtc_cntl_bias_conf) module"]
pub type RTC_CNTL_BIAS_CONF = crate::Reg<u32, _RTC_CNTL_BIAS_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_BIAS_CONF;
#[doc = "`read()` method returns [rtc_cntl_bias_conf::R](rtc_cntl_bias_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_BIAS_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_bias_conf::W](rtc_cntl_bias_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_BIAS_CONF {}
#[doc = "RTC_CNTL_BIAS_CONF_REG"]
pub mod rtc_cntl_bias_conf;
#[doc = "RTC_CNTL_PWC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_pwc](rtc_cntl_pwc) module"]
pub type RTC_CNTL_PWC = crate::Reg<u32, _RTC_CNTL_PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_PWC;
#[doc = "`read()` method returns [rtc_cntl_pwc::R](rtc_cntl_pwc::R) reader structure"]
impl crate::Readable for RTC_CNTL_PWC {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_pwc::W](rtc_cntl_pwc::W) writer structure"]
impl crate::Writable for RTC_CNTL_PWC {}
#[doc = "RTC_CNTL_PWC_REG"]
pub mod rtc_cntl_pwc;
#[doc = "RTC_CNTL_DIG_PWC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_dig_pwc](rtc_cntl_dig_pwc) module"]
pub type RTC_CNTL_DIG_PWC = crate::Reg<u32, _RTC_CNTL_DIG_PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_PWC;
#[doc = "`read()` method returns [rtc_cntl_dig_pwc::R](rtc_cntl_dig_pwc::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_PWC {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_pwc::W](rtc_cntl_dig_pwc::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_PWC {}
#[doc = "RTC_CNTL_DIG_PWC_REG"]
pub mod rtc_cntl_dig_pwc;
#[doc = "RTC_CNTL_DIG_ISO_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_dig_iso](rtc_cntl_dig_iso) module"]
pub type RTC_CNTL_DIG_ISO = crate::Reg<u32, _RTC_CNTL_DIG_ISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_ISO;
#[doc = "`read()` method returns [rtc_cntl_dig_iso::R](rtc_cntl_dig_iso::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_ISO {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_iso::W](rtc_cntl_dig_iso::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_ISO {}
#[doc = "RTC_CNTL_DIG_ISO_REG"]
pub mod rtc_cntl_dig_iso;
#[doc = "RTC_CNTL_WDTCONFIG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig0](rtc_cntl_wdtconfig0) module"]
pub type RTC_CNTL_WDTCONFIG0 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG0;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig0::R](rtc_cntl_wdtconfig0::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig0::W](rtc_cntl_wdtconfig0::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG0 {}
#[doc = "RTC_CNTL_WDTCONFIG0_REG"]
pub mod rtc_cntl_wdtconfig0;
#[doc = "RTC_CNTL_WDTCONFIG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig1](rtc_cntl_wdtconfig1) module"]
pub type RTC_CNTL_WDTCONFIG1 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG1;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig1::R](rtc_cntl_wdtconfig1::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig1::W](rtc_cntl_wdtconfig1::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG1 {}
#[doc = "RTC_CNTL_WDTCONFIG1_REG"]
pub mod rtc_cntl_wdtconfig1;
#[doc = "RTC_CNTL_WDTCONFIG2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig2](rtc_cntl_wdtconfig2) module"]
pub type RTC_CNTL_WDTCONFIG2 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG2;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig2::R](rtc_cntl_wdtconfig2::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig2::W](rtc_cntl_wdtconfig2::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG2 {}
#[doc = "RTC_CNTL_WDTCONFIG2_REG"]
pub mod rtc_cntl_wdtconfig2;
#[doc = "RTC_CNTL_WDTCONFIG3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig3](rtc_cntl_wdtconfig3) module"]
pub type RTC_CNTL_WDTCONFIG3 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG3;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig3::R](rtc_cntl_wdtconfig3::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig3::W](rtc_cntl_wdtconfig3::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG3 {}
#[doc = "RTC_CNTL_WDTCONFIG3_REG"]
pub mod rtc_cntl_wdtconfig3;
#[doc = "RTC_CNTL_WDTCONFIG4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig4](rtc_cntl_wdtconfig4) module"]
pub type RTC_CNTL_WDTCONFIG4 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG4;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig4::R](rtc_cntl_wdtconfig4::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig4::W](rtc_cntl_wdtconfig4::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG4 {}
#[doc = "RTC_CNTL_WDTCONFIG4_REG"]
pub mod rtc_cntl_wdtconfig4;
#[doc = "RTC_CNTL_WDTFEED_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtfeed](rtc_cntl_wdtfeed) module"]
pub type RTC_CNTL_WDTFEED = crate::Reg<u32, _RTC_CNTL_WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTFEED;
#[doc = "`read()` method returns [rtc_cntl_wdtfeed::R](rtc_cntl_wdtfeed::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTFEED {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtfeed::W](rtc_cntl_wdtfeed::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTFEED {}
#[doc = "RTC_CNTL_WDTFEED_REG"]
pub mod rtc_cntl_wdtfeed;
#[doc = "RTC_CNTL_WDTWPROTECT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtwprotect](rtc_cntl_wdtwprotect) module"]
pub type RTC_CNTL_WDTWPROTECT = crate::Reg<u32, _RTC_CNTL_WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTWPROTECT;
#[doc = "`read()` method returns [rtc_cntl_wdtwprotect::R](rtc_cntl_wdtwprotect::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtwprotect::W](rtc_cntl_wdtwprotect::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTWPROTECT {}
#[doc = "RTC_CNTL_WDTWPROTECT_REG"]
pub mod rtc_cntl_wdtwprotect;
#[doc = "RTC_CNTL_TEST_MUX_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_test_mux](rtc_cntl_test_mux) module"]
pub type RTC_CNTL_TEST_MUX = crate::Reg<u32, _RTC_CNTL_TEST_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TEST_MUX;
#[doc = "`read()` method returns [rtc_cntl_test_mux::R](rtc_cntl_test_mux::R) reader structure"]
impl crate::Readable for RTC_CNTL_TEST_MUX {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_test_mux::W](rtc_cntl_test_mux::W) writer structure"]
impl crate::Writable for RTC_CNTL_TEST_MUX {}
#[doc = "RTC_CNTL_TEST_MUX_REG"]
pub mod rtc_cntl_test_mux;
#[doc = "RTC_CNTL_SW_CPU_STALL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_sw_cpu_stall](rtc_cntl_sw_cpu_stall) module"]
pub type RTC_CNTL_SW_CPU_STALL = crate::Reg<u32, _RTC_CNTL_SW_CPU_STALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SW_CPU_STALL;
#[doc = "`read()` method returns [rtc_cntl_sw_cpu_stall::R](rtc_cntl_sw_cpu_stall::R) reader structure"]
impl crate::Readable for RTC_CNTL_SW_CPU_STALL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sw_cpu_stall::W](rtc_cntl_sw_cpu_stall::W) writer structure"]
impl crate::Writable for RTC_CNTL_SW_CPU_STALL {}
#[doc = "RTC_CNTL_SW_CPU_STALL_REG"]
pub mod rtc_cntl_sw_cpu_stall;
#[doc = "RTC_CNTL_STORE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store4](rtc_cntl_store4) module"]
pub type RTC_CNTL_STORE4 = crate::Reg<u32, _RTC_CNTL_STORE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE4;
#[doc = "`read()` method returns [rtc_cntl_store4::R](rtc_cntl_store4::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE4 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store4::W](rtc_cntl_store4::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE4 {}
#[doc = "RTC_CNTL_STORE4_REG"]
pub mod rtc_cntl_store4;
#[doc = "RTC_CNTL_STORE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store5](rtc_cntl_store5) module"]
pub type RTC_CNTL_STORE5 = crate::Reg<u32, _RTC_CNTL_STORE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE5;
#[doc = "`read()` method returns [rtc_cntl_store5::R](rtc_cntl_store5::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE5 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store5::W](rtc_cntl_store5::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE5 {}
#[doc = "RTC_CNTL_STORE5_REG"]
pub mod rtc_cntl_store5;
#[doc = "RTC_CNTL_STORE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store6](rtc_cntl_store6) module"]
pub type RTC_CNTL_STORE6 = crate::Reg<u32, _RTC_CNTL_STORE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE6;
#[doc = "`read()` method returns [rtc_cntl_store6::R](rtc_cntl_store6::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE6 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store6::W](rtc_cntl_store6::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE6 {}
#[doc = "RTC_CNTL_STORE6_REG"]
pub mod rtc_cntl_store6;
#[doc = "RTC_CNTL_STORE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store7](rtc_cntl_store7) module"]
pub type RTC_CNTL_STORE7 = crate::Reg<u32, _RTC_CNTL_STORE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE7;
#[doc = "`read()` method returns [rtc_cntl_store7::R](rtc_cntl_store7::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE7 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store7::W](rtc_cntl_store7::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE7 {}
#[doc = "RTC_CNTL_STORE7_REG"]
pub mod rtc_cntl_store7;
#[doc = "RTC_CNTL_DIAG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_diag1](rtc_cntl_diag1) module"]
pub type RTC_CNTL_DIAG1 = crate::Reg<u32, _RTC_CNTL_DIAG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIAG1;
#[doc = "`read()` method returns [rtc_cntl_diag1::R](rtc_cntl_diag1::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIAG1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_diag1::W](rtc_cntl_diag1::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIAG1 {}
#[doc = "RTC_CNTL_DIAG1_REG"]
pub mod rtc_cntl_diag1;
#[doc = "RTC_CNTL_HOLD_FORCE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_hold_force](rtc_cntl_hold_force) module"]
pub type RTC_CNTL_HOLD_FORCE = crate::Reg<u32, _RTC_CNTL_HOLD_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_HOLD_FORCE;
#[doc = "`read()` method returns [rtc_cntl_hold_force::R](rtc_cntl_hold_force::R) reader structure"]
impl crate::Readable for RTC_CNTL_HOLD_FORCE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_hold_force::W](rtc_cntl_hold_force::W) writer structure"]
impl crate::Writable for RTC_CNTL_HOLD_FORCE {}
#[doc = "RTC_CNTL_HOLD_FORCE_REG"]
pub mod rtc_cntl_hold_force;
#[doc = "RTC_CNTL_EXT_WAKEUP1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_wakeup1](rtc_cntl_ext_wakeup1) module"]
pub type RTC_CNTL_EXT_WAKEUP1 = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP1;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup1::R](rtc_cntl_ext_wakeup1::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup1::W](rtc_cntl_ext_wakeup1::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP1 {}
#[doc = "RTC_CNTL_EXT_WAKEUP1_REG"]
pub mod rtc_cntl_ext_wakeup1;
#[doc = "RTC_CNTL_EXT_WAKEUP1_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_wakeup1_status](rtc_cntl_ext_wakeup1_status) module"]
pub type RTC_CNTL_EXT_WAKEUP1_STATUS = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP1_STATUS;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup1_status::R](rtc_cntl_ext_wakeup1_status::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP1_STATUS {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup1_status::W](rtc_cntl_ext_wakeup1_status::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP1_STATUS {}
#[doc = "RTC_CNTL_EXT_WAKEUP1_STATUS_REG"]
pub mod rtc_cntl_ext_wakeup1_status;
#[doc = "RTC_CNTL_BROWN_OUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_brown_out](rtc_cntl_brown_out) module"]
pub type RTC_CNTL_BROWN_OUT = crate::Reg<u32, _RTC_CNTL_BROWN_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_BROWN_OUT;
#[doc = "`read()` method returns [rtc_cntl_brown_out::R](rtc_cntl_brown_out::R) reader structure"]
impl crate::Readable for RTC_CNTL_BROWN_OUT {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_brown_out::W](rtc_cntl_brown_out::W) writer structure"]
impl crate::Writable for RTC_CNTL_BROWN_OUT {}
#[doc = "RTC_CNTL_BROWN_OUT_REG"]
pub mod rtc_cntl_brown_out;
#[doc = "RTC_CNTL_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_date](rtc_cntl_date) module"]
pub type RTC_CNTL_DATE = crate::Reg<u32, _RTC_CNTL_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DATE;
#[doc = "`read()` method returns [rtc_cntl_date::R](rtc_cntl_date::R) reader structure"]
impl crate::Readable for RTC_CNTL_DATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_date::W](rtc_cntl_date::W) writer structure"]
impl crate::Writable for RTC_CNTL_DATE {}
#[doc = "RTC_CNTL_DATE_REG"]
pub mod rtc_cntl_date;
