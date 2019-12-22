#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNTL_OPTIONS0_REG"]
    pub rtc_cntl_options0_reg: RTC_CNTL_OPTIONS0_REG,
    #[doc = "0x04 - RTC_CNTL_SLP_TIMER0_REG"]
    pub rtc_cntl_slp_timer0_reg: RTC_CNTL_SLP_TIMER0_REG,
    #[doc = "0x08 - RTC_CNTL_SLP_TIMER1_REG"]
    pub rtc_cntl_slp_timer1_reg: RTC_CNTL_SLP_TIMER1_REG,
    #[doc = "0x0c - RTC_CNTL_TIME_UPDATE_REG"]
    pub rtc_cntl_time_update_reg: RTC_CNTL_TIME_UPDATE_REG,
    #[doc = "0x10 - RTC_CNTL_TIME0_REG"]
    pub rtc_cntl_time0_reg: RTC_CNTL_TIME0_REG,
    #[doc = "0x14 - RTC_CNTL_TIME1_REG"]
    pub rtc_cntl_time1_reg: RTC_CNTL_TIME1_REG,
    #[doc = "0x18 - RTC_CNTL_STATE0_REG"]
    pub rtc_cntl_state0_reg: RTC_CNTL_STATE0_REG,
    #[doc = "0x1c - RTC_CNTL_TIMER1_REG"]
    pub rtc_cntl_timer1_reg: RTC_CNTL_TIMER1_REG,
    #[doc = "0x20 - RTC_CNTL_TIMER2_REG"]
    pub rtc_cntl_timer2_reg: RTC_CNTL_TIMER2_REG,
    #[doc = "0x24 - RTC_CNTL_TIMER3_REG"]
    pub rtc_cntl_timer3_reg: RTC_CNTL_TIMER3_REG,
    #[doc = "0x28 - RTC_CNTL_TIMER4_REG"]
    pub rtc_cntl_timer4_reg: RTC_CNTL_TIMER4_REG,
    #[doc = "0x2c - RTC_CNTL_TIMER5_REG"]
    pub rtc_cntl_timer5_reg: RTC_CNTL_TIMER5_REG,
    #[doc = "0x30 - RTC_CNTL_ANA_CONF_REG"]
    pub rtc_cntl_ana_conf_reg: RTC_CNTL_ANA_CONF_REG,
    #[doc = "0x34 - RTC_CNTL_RESET_STATE_REG"]
    pub rtc_cntl_reset_state_reg: RTC_CNTL_RESET_STATE_REG,
    #[doc = "0x38 - RTC_CNTL_WAKEUP_STATE_REG"]
    pub rtc_cntl_wakeup_state_reg: RTC_CNTL_WAKEUP_STATE_REG,
    #[doc = "0x3c - RTC_CNTL_INT_ENA_REG"]
    pub rtc_cntl_int_ena_reg: RTC_CNTL_INT_ENA_REG,
    #[doc = "0x40 - RTC_CNTL_INT_RAW_REG"]
    pub rtc_cntl_int_raw_reg: RTC_CNTL_INT_RAW_REG,
    #[doc = "0x44 - RTC_CNTL_INT_ST_REG"]
    pub rtc_cntl_int_st_reg: RTC_CNTL_INT_ST_REG,
    #[doc = "0x48 - RTC_CNTL_INT_CLR_REG"]
    pub rtc_cntl_int_clr_reg: RTC_CNTL_INT_CLR_REG,
    #[doc = "0x4c - RTC_CNTL_STORE0_REG"]
    pub rtc_cntl_store0_reg: RTC_CNTL_STORE0_REG,
    #[doc = "0x50 - RTC_CNTL_STORE1_REG"]
    pub rtc_cntl_store1_reg: RTC_CNTL_STORE1_REG,
    #[doc = "0x54 - RTC_CNTL_STORE2_REG"]
    pub rtc_cntl_store2_reg: RTC_CNTL_STORE2_REG,
    #[doc = "0x58 - RTC_CNTL_STORE3_REG"]
    pub rtc_cntl_store3_reg: RTC_CNTL_STORE3_REG,
    #[doc = "0x5c - RTC_CNTL_EXT_XTL_CONF_REG"]
    pub rtc_cntl_ext_xtl_conf_reg: RTC_CNTL_EXT_XTL_CONF_REG,
    #[doc = "0x60 - RTC_CNTL_EXT_WAKEUP_CONF_REG"]
    pub rtc_cntl_ext_wakeup_conf_reg: RTC_CNTL_EXT_WAKEUP_CONF_REG,
    #[doc = "0x64 - RTC_CNTL_SLP_REJECT_CONF_REG"]
    pub rtc_cntl_slp_reject_conf_reg: RTC_CNTL_SLP_REJECT_CONF_REG,
    #[doc = "0x68 - RTC_CNTL_CPU_PERIOD_CONF_REG"]
    pub rtc_cntl_cpu_period_conf_reg: RTC_CNTL_CPU_PERIOD_CONF_REG,
    #[doc = "0x6c - RTC_CNTL_SDIO_ACT_CONF_REG"]
    pub rtc_cntl_sdio_act_conf_reg: RTC_CNTL_SDIO_ACT_CONF_REG,
    #[doc = "0x70 - RTC_CNTL_CLK_CONF_REG"]
    pub rtc_cntl_clk_conf_reg: RTC_CNTL_CLK_CONF_REG,
    #[doc = "0x74 - RTC_CNTL_SDIO_CONF_REG"]
    pub rtc_cntl_sdio_conf_reg: RTC_CNTL_SDIO_CONF_REG,
    #[doc = "0x78 - RTC_CNTL_BIAS_CONF_REG"]
    pub rtc_cntl_bias_conf_reg: RTC_CNTL_BIAS_CONF_REG,
    _reserved31: [u8; 4usize],
    #[doc = "0x80 - RTC_CNTL_PWC_REG"]
    pub rtc_cntl_pwc_reg: RTC_CNTL_PWC_REG,
    #[doc = "0x84 - RTC_CNTL_DIG_PWC_REG"]
    pub rtc_cntl_dig_pwc_reg: RTC_CNTL_DIG_PWC_REG,
    #[doc = "0x88 - RTC_CNTL_DIG_ISO_REG"]
    pub rtc_cntl_dig_iso_reg: RTC_CNTL_DIG_ISO_REG,
    #[doc = "0x8c - RTC_CNTL_WDTCONFIG0_REG"]
    pub rtc_cntl_wdtconfig0_reg: RTC_CNTL_WDTCONFIG0_REG,
    #[doc = "0x90 - RTC_CNTL_WDTCONFIG1_REG"]
    pub rtc_cntl_wdtconfig1_reg: RTC_CNTL_WDTCONFIG1_REG,
    #[doc = "0x94 - RTC_CNTL_WDTCONFIG2_REG"]
    pub rtc_cntl_wdtconfig2_reg: RTC_CNTL_WDTCONFIG2_REG,
    #[doc = "0x98 - RTC_CNTL_WDTCONFIG3_REG"]
    pub rtc_cntl_wdtconfig3_reg: RTC_CNTL_WDTCONFIG3_REG,
    #[doc = "0x9c - RTC_CNTL_WDTCONFIG4_REG"]
    pub rtc_cntl_wdtconfig4_reg: RTC_CNTL_WDTCONFIG4_REG,
    #[doc = "0xa0 - RTC_CNTL_WDTFEED_REG"]
    pub rtc_cntl_wdtfeed_reg: RTC_CNTL_WDTFEED_REG,
    #[doc = "0xa4 - RTC_CNTL_WDTWPROTECT_REG"]
    pub rtc_cntl_wdtwprotect_reg: RTC_CNTL_WDTWPROTECT_REG,
    #[doc = "0xa8 - RTC_CNTL_TEST_MUX_REG"]
    pub rtc_cntl_test_mux_reg: RTC_CNTL_TEST_MUX_REG,
    #[doc = "0xac - RTC_CNTL_SW_CPU_STALL_REG"]
    pub rtc_cntl_sw_cpu_stall_reg: RTC_CNTL_SW_CPU_STALL_REG,
    #[doc = "0xb0 - RTC_CNTL_STORE4_REG"]
    pub rtc_cntl_store4_reg: RTC_CNTL_STORE4_REG,
    #[doc = "0xb4 - RTC_CNTL_STORE5_REG"]
    pub rtc_cntl_store5_reg: RTC_CNTL_STORE5_REG,
    #[doc = "0xb8 - RTC_CNTL_STORE6_REG"]
    pub rtc_cntl_store6_reg: RTC_CNTL_STORE6_REG,
    #[doc = "0xbc - RTC_CNTL_STORE7_REG"]
    pub rtc_cntl_store7_reg: RTC_CNTL_STORE7_REG,
    _reserved47: [u8; 4usize],
    #[doc = "0xc4 - RTC_CNTL_DIAG1_REG"]
    pub rtc_cntl_diag1_reg: RTC_CNTL_DIAG1_REG,
    #[doc = "0xc8 - RTC_CNTL_HOLD_FORCE_REG"]
    pub rtc_cntl_hold_force_reg: RTC_CNTL_HOLD_FORCE_REG,
    #[doc = "0xcc - RTC_CNTL_EXT_WAKEUP1_REG"]
    pub rtc_cntl_ext_wakeup1_reg: RTC_CNTL_EXT_WAKEUP1_REG,
    #[doc = "0xd0 - RTC_CNTL_EXT_WAKEUP1_STATUS_REG"]
    pub rtc_cntl_ext_wakeup1_status_reg: RTC_CNTL_EXT_WAKEUP1_STATUS_REG,
    #[doc = "0xd4 - RTC_CNTL_BROWN_OUT_REG"]
    pub rtc_cntl_brown_out_reg: RTC_CNTL_BROWN_OUT_REG,
    _reserved52: [u8; 100usize],
    #[doc = "0x13c - RTC_CNTL_DATE_REG"]
    pub rtc_cntl_date_reg: RTC_CNTL_DATE_REG,
}
#[doc = "RTC_CNTL_OPTIONS0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_options0_reg](rtc_cntl_options0_reg) module"]
pub type RTC_CNTL_OPTIONS0_REG = crate::Reg<u32, _RTC_CNTL_OPTIONS0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_OPTIONS0_REG;
#[doc = "`read()` method returns [rtc_cntl_options0_reg::R](rtc_cntl_options0_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_OPTIONS0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_options0_reg::W](rtc_cntl_options0_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_OPTIONS0_REG {}
#[doc = "RTC_CNTL_OPTIONS0_REG"]
pub mod rtc_cntl_options0_reg;
#[doc = "RTC_CNTL_SLP_TIMER0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_slp_timer0_reg](rtc_cntl_slp_timer0_reg) module"]
pub type RTC_CNTL_SLP_TIMER0_REG = crate::Reg<u32, _RTC_CNTL_SLP_TIMER0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_TIMER0_REG;
#[doc = "`read()` method returns [rtc_cntl_slp_timer0_reg::R](rtc_cntl_slp_timer0_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_TIMER0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_timer0_reg::W](rtc_cntl_slp_timer0_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_TIMER0_REG {}
#[doc = "RTC_CNTL_SLP_TIMER0_REG"]
pub mod rtc_cntl_slp_timer0_reg;
#[doc = "RTC_CNTL_SLP_TIMER1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_slp_timer1_reg](rtc_cntl_slp_timer1_reg) module"]
pub type RTC_CNTL_SLP_TIMER1_REG = crate::Reg<u32, _RTC_CNTL_SLP_TIMER1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_TIMER1_REG;
#[doc = "`read()` method returns [rtc_cntl_slp_timer1_reg::R](rtc_cntl_slp_timer1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_TIMER1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_timer1_reg::W](rtc_cntl_slp_timer1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_TIMER1_REG {}
#[doc = "RTC_CNTL_SLP_TIMER1_REG"]
pub mod rtc_cntl_slp_timer1_reg;
#[doc = "RTC_CNTL_TIME_UPDATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_time_update_reg](rtc_cntl_time_update_reg) module"]
pub type RTC_CNTL_TIME_UPDATE_REG = crate::Reg<u32, _RTC_CNTL_TIME_UPDATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_UPDATE_REG;
#[doc = "`read()` method returns [rtc_cntl_time_update_reg::R](rtc_cntl_time_update_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_UPDATE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time_update_reg::W](rtc_cntl_time_update_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME_UPDATE_REG {}
#[doc = "RTC_CNTL_TIME_UPDATE_REG"]
pub mod rtc_cntl_time_update_reg;
#[doc = "RTC_CNTL_TIME0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_time0_reg](rtc_cntl_time0_reg) module"]
pub type RTC_CNTL_TIME0_REG = crate::Reg<u32, _RTC_CNTL_TIME0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME0_REG;
#[doc = "`read()` method returns [rtc_cntl_time0_reg::R](rtc_cntl_time0_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time0_reg::W](rtc_cntl_time0_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME0_REG {}
#[doc = "RTC_CNTL_TIME0_REG"]
pub mod rtc_cntl_time0_reg;
#[doc = "RTC_CNTL_TIME1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_time1_reg](rtc_cntl_time1_reg) module"]
pub type RTC_CNTL_TIME1_REG = crate::Reg<u32, _RTC_CNTL_TIME1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME1_REG;
#[doc = "`read()` method returns [rtc_cntl_time1_reg::R](rtc_cntl_time1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time1_reg::W](rtc_cntl_time1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME1_REG {}
#[doc = "RTC_CNTL_TIME1_REG"]
pub mod rtc_cntl_time1_reg;
#[doc = "RTC_CNTL_STATE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_state0_reg](rtc_cntl_state0_reg) module"]
pub type RTC_CNTL_STATE0_REG = crate::Reg<u32, _RTC_CNTL_STATE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STATE0_REG;
#[doc = "`read()` method returns [rtc_cntl_state0_reg::R](rtc_cntl_state0_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STATE0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_state0_reg::W](rtc_cntl_state0_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STATE0_REG {}
#[doc = "RTC_CNTL_STATE0_REG"]
pub mod rtc_cntl_state0_reg;
#[doc = "RTC_CNTL_TIMER1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer1_reg](rtc_cntl_timer1_reg) module"]
pub type RTC_CNTL_TIMER1_REG = crate::Reg<u32, _RTC_CNTL_TIMER1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER1_REG;
#[doc = "`read()` method returns [rtc_cntl_timer1_reg::R](rtc_cntl_timer1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer1_reg::W](rtc_cntl_timer1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER1_REG {}
#[doc = "RTC_CNTL_TIMER1_REG"]
pub mod rtc_cntl_timer1_reg;
#[doc = "RTC_CNTL_TIMER2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer2_reg](rtc_cntl_timer2_reg) module"]
pub type RTC_CNTL_TIMER2_REG = crate::Reg<u32, _RTC_CNTL_TIMER2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER2_REG;
#[doc = "`read()` method returns [rtc_cntl_timer2_reg::R](rtc_cntl_timer2_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER2_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer2_reg::W](rtc_cntl_timer2_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER2_REG {}
#[doc = "RTC_CNTL_TIMER2_REG"]
pub mod rtc_cntl_timer2_reg;
#[doc = "RTC_CNTL_TIMER3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer3_reg](rtc_cntl_timer3_reg) module"]
pub type RTC_CNTL_TIMER3_REG = crate::Reg<u32, _RTC_CNTL_TIMER3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER3_REG;
#[doc = "`read()` method returns [rtc_cntl_timer3_reg::R](rtc_cntl_timer3_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER3_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer3_reg::W](rtc_cntl_timer3_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER3_REG {}
#[doc = "RTC_CNTL_TIMER3_REG"]
pub mod rtc_cntl_timer3_reg;
#[doc = "RTC_CNTL_TIMER4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer4_reg](rtc_cntl_timer4_reg) module"]
pub type RTC_CNTL_TIMER4_REG = crate::Reg<u32, _RTC_CNTL_TIMER4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER4_REG;
#[doc = "`read()` method returns [rtc_cntl_timer4_reg::R](rtc_cntl_timer4_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER4_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer4_reg::W](rtc_cntl_timer4_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER4_REG {}
#[doc = "RTC_CNTL_TIMER4_REG"]
pub mod rtc_cntl_timer4_reg;
#[doc = "RTC_CNTL_TIMER5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_timer5_reg](rtc_cntl_timer5_reg) module"]
pub type RTC_CNTL_TIMER5_REG = crate::Reg<u32, _RTC_CNTL_TIMER5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER5_REG;
#[doc = "`read()` method returns [rtc_cntl_timer5_reg::R](rtc_cntl_timer5_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER5_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer5_reg::W](rtc_cntl_timer5_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER5_REG {}
#[doc = "RTC_CNTL_TIMER5_REG"]
pub mod rtc_cntl_timer5_reg;
#[doc = "RTC_CNTL_ANA_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ana_conf_reg](rtc_cntl_ana_conf_reg) module"]
pub type RTC_CNTL_ANA_CONF_REG = crate::Reg<u32, _RTC_CNTL_ANA_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_ANA_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_ana_conf_reg::R](rtc_cntl_ana_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_ANA_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ana_conf_reg::W](rtc_cntl_ana_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_ANA_CONF_REG {}
#[doc = "RTC_CNTL_ANA_CONF_REG"]
pub mod rtc_cntl_ana_conf_reg;
#[doc = "RTC_CNTL_RESET_STATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_reset_state_reg](rtc_cntl_reset_state_reg) module"]
pub type RTC_CNTL_RESET_STATE_REG = crate::Reg<u32, _RTC_CNTL_RESET_STATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_RESET_STATE_REG;
#[doc = "`read()` method returns [rtc_cntl_reset_state_reg::R](rtc_cntl_reset_state_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_RESET_STATE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_reset_state_reg::W](rtc_cntl_reset_state_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_RESET_STATE_REG {}
#[doc = "RTC_CNTL_RESET_STATE_REG"]
pub mod rtc_cntl_reset_state_reg;
#[doc = "RTC_CNTL_WAKEUP_STATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wakeup_state_reg](rtc_cntl_wakeup_state_reg) module"]
pub type RTC_CNTL_WAKEUP_STATE_REG = crate::Reg<u32, _RTC_CNTL_WAKEUP_STATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WAKEUP_STATE_REG;
#[doc = "`read()` method returns [rtc_cntl_wakeup_state_reg::R](rtc_cntl_wakeup_state_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WAKEUP_STATE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wakeup_state_reg::W](rtc_cntl_wakeup_state_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WAKEUP_STATE_REG {}
#[doc = "RTC_CNTL_WAKEUP_STATE_REG"]
pub mod rtc_cntl_wakeup_state_reg;
#[doc = "RTC_CNTL_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_ena_reg](rtc_cntl_int_ena_reg) module"]
pub type RTC_CNTL_INT_ENA_REG = crate::Reg<u32, _RTC_CNTL_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ENA_REG;
#[doc = "`read()` method returns [rtc_cntl_int_ena_reg::R](rtc_cntl_int_ena_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_ena_reg::W](rtc_cntl_int_ena_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ENA_REG {}
#[doc = "RTC_CNTL_INT_ENA_REG"]
pub mod rtc_cntl_int_ena_reg;
#[doc = "RTC_CNTL_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_raw_reg](rtc_cntl_int_raw_reg) module"]
pub type RTC_CNTL_INT_RAW_REG = crate::Reg<u32, _RTC_CNTL_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_RAW_REG;
#[doc = "`read()` method returns [rtc_cntl_int_raw_reg::R](rtc_cntl_int_raw_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_raw_reg::W](rtc_cntl_int_raw_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_RAW_REG {}
#[doc = "RTC_CNTL_INT_RAW_REG"]
pub mod rtc_cntl_int_raw_reg;
#[doc = "RTC_CNTL_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_st_reg](rtc_cntl_int_st_reg) module"]
pub type RTC_CNTL_INT_ST_REG = crate::Reg<u32, _RTC_CNTL_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ST_REG;
#[doc = "`read()` method returns [rtc_cntl_int_st_reg::R](rtc_cntl_int_st_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_st_reg::W](rtc_cntl_int_st_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ST_REG {}
#[doc = "RTC_CNTL_INT_ST_REG"]
pub mod rtc_cntl_int_st_reg;
#[doc = "RTC_CNTL_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_int_clr_reg](rtc_cntl_int_clr_reg) module"]
pub type RTC_CNTL_INT_CLR_REG = crate::Reg<u32, _RTC_CNTL_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_CLR_REG;
#[doc = "`read()` method returns [rtc_cntl_int_clr_reg::R](rtc_cntl_int_clr_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_clr_reg::W](rtc_cntl_int_clr_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_CLR_REG {}
#[doc = "RTC_CNTL_INT_CLR_REG"]
pub mod rtc_cntl_int_clr_reg;
#[doc = "RTC_CNTL_STORE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store0_reg](rtc_cntl_store0_reg) module"]
pub type RTC_CNTL_STORE0_REG = crate::Reg<u32, _RTC_CNTL_STORE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE0_REG;
#[doc = "`read()` method returns [rtc_cntl_store0_reg::R](rtc_cntl_store0_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store0_reg::W](rtc_cntl_store0_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE0_REG {}
#[doc = "RTC_CNTL_STORE0_REG"]
pub mod rtc_cntl_store0_reg;
#[doc = "RTC_CNTL_STORE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store1_reg](rtc_cntl_store1_reg) module"]
pub type RTC_CNTL_STORE1_REG = crate::Reg<u32, _RTC_CNTL_STORE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE1_REG;
#[doc = "`read()` method returns [rtc_cntl_store1_reg::R](rtc_cntl_store1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store1_reg::W](rtc_cntl_store1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE1_REG {}
#[doc = "RTC_CNTL_STORE1_REG"]
pub mod rtc_cntl_store1_reg;
#[doc = "RTC_CNTL_STORE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store2_reg](rtc_cntl_store2_reg) module"]
pub type RTC_CNTL_STORE2_REG = crate::Reg<u32, _RTC_CNTL_STORE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE2_REG;
#[doc = "`read()` method returns [rtc_cntl_store2_reg::R](rtc_cntl_store2_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE2_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store2_reg::W](rtc_cntl_store2_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE2_REG {}
#[doc = "RTC_CNTL_STORE2_REG"]
pub mod rtc_cntl_store2_reg;
#[doc = "RTC_CNTL_STORE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store3_reg](rtc_cntl_store3_reg) module"]
pub type RTC_CNTL_STORE3_REG = crate::Reg<u32, _RTC_CNTL_STORE3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE3_REG;
#[doc = "`read()` method returns [rtc_cntl_store3_reg::R](rtc_cntl_store3_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE3_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store3_reg::W](rtc_cntl_store3_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE3_REG {}
#[doc = "RTC_CNTL_STORE3_REG"]
pub mod rtc_cntl_store3_reg;
#[doc = "RTC_CNTL_EXT_XTL_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_xtl_conf_reg](rtc_cntl_ext_xtl_conf_reg) module"]
pub type RTC_CNTL_EXT_XTL_CONF_REG = crate::Reg<u32, _RTC_CNTL_EXT_XTL_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_XTL_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_ext_xtl_conf_reg::R](rtc_cntl_ext_xtl_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_XTL_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_xtl_conf_reg::W](rtc_cntl_ext_xtl_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_XTL_CONF_REG {}
#[doc = "RTC_CNTL_EXT_XTL_CONF_REG"]
pub mod rtc_cntl_ext_xtl_conf_reg;
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_wakeup_conf_reg](rtc_cntl_ext_wakeup_conf_reg) module"]
pub type RTC_CNTL_EXT_WAKEUP_CONF_REG = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup_conf_reg::R](rtc_cntl_ext_wakeup_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup_conf_reg::W](rtc_cntl_ext_wakeup_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP_CONF_REG {}
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF_REG"]
pub mod rtc_cntl_ext_wakeup_conf_reg;
#[doc = "RTC_CNTL_SLP_REJECT_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_slp_reject_conf_reg](rtc_cntl_slp_reject_conf_reg) module"]
pub type RTC_CNTL_SLP_REJECT_CONF_REG = crate::Reg<u32, _RTC_CNTL_SLP_REJECT_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_REJECT_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_slp_reject_conf_reg::R](rtc_cntl_slp_reject_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_REJECT_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_reject_conf_reg::W](rtc_cntl_slp_reject_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_REJECT_CONF_REG {}
#[doc = "RTC_CNTL_SLP_REJECT_CONF_REG"]
pub mod rtc_cntl_slp_reject_conf_reg;
#[doc = "RTC_CNTL_CPU_PERIOD_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_cpu_period_conf_reg](rtc_cntl_cpu_period_conf_reg) module"]
pub type RTC_CNTL_CPU_PERIOD_CONF_REG = crate::Reg<u32, _RTC_CNTL_CPU_PERIOD_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_CPU_PERIOD_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_cpu_period_conf_reg::R](rtc_cntl_cpu_period_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_CPU_PERIOD_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_cpu_period_conf_reg::W](rtc_cntl_cpu_period_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_CPU_PERIOD_CONF_REG {}
#[doc = "RTC_CNTL_CPU_PERIOD_CONF_REG"]
pub mod rtc_cntl_cpu_period_conf_reg;
#[doc = "RTC_CNTL_SDIO_ACT_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_sdio_act_conf_reg](rtc_cntl_sdio_act_conf_reg) module"]
pub type RTC_CNTL_SDIO_ACT_CONF_REG = crate::Reg<u32, _RTC_CNTL_SDIO_ACT_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SDIO_ACT_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_sdio_act_conf_reg::R](rtc_cntl_sdio_act_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_SDIO_ACT_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sdio_act_conf_reg::W](rtc_cntl_sdio_act_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_SDIO_ACT_CONF_REG {}
#[doc = "RTC_CNTL_SDIO_ACT_CONF_REG"]
pub mod rtc_cntl_sdio_act_conf_reg;
#[doc = "RTC_CNTL_CLK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_clk_conf_reg](rtc_cntl_clk_conf_reg) module"]
pub type RTC_CNTL_CLK_CONF_REG = crate::Reg<u32, _RTC_CNTL_CLK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_CLK_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_clk_conf_reg::R](rtc_cntl_clk_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_CLK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_clk_conf_reg::W](rtc_cntl_clk_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_CLK_CONF_REG {}
#[doc = "RTC_CNTL_CLK_CONF_REG"]
pub mod rtc_cntl_clk_conf_reg;
#[doc = "RTC_CNTL_SDIO_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_sdio_conf_reg](rtc_cntl_sdio_conf_reg) module"]
pub type RTC_CNTL_SDIO_CONF_REG = crate::Reg<u32, _RTC_CNTL_SDIO_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SDIO_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_sdio_conf_reg::R](rtc_cntl_sdio_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_SDIO_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sdio_conf_reg::W](rtc_cntl_sdio_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_SDIO_CONF_REG {}
#[doc = "RTC_CNTL_SDIO_CONF_REG"]
pub mod rtc_cntl_sdio_conf_reg;
#[doc = "RTC_CNTL_BIAS_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_bias_conf_reg](rtc_cntl_bias_conf_reg) module"]
pub type RTC_CNTL_BIAS_CONF_REG = crate::Reg<u32, _RTC_CNTL_BIAS_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_BIAS_CONF_REG;
#[doc = "`read()` method returns [rtc_cntl_bias_conf_reg::R](rtc_cntl_bias_conf_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_BIAS_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_bias_conf_reg::W](rtc_cntl_bias_conf_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_BIAS_CONF_REG {}
#[doc = "RTC_CNTL_BIAS_CONF_REG"]
pub mod rtc_cntl_bias_conf_reg;
#[doc = "RTC_CNTL_PWC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_pwc_reg](rtc_cntl_pwc_reg) module"]
pub type RTC_CNTL_PWC_REG = crate::Reg<u32, _RTC_CNTL_PWC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_PWC_REG;
#[doc = "`read()` method returns [rtc_cntl_pwc_reg::R](rtc_cntl_pwc_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_PWC_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_pwc_reg::W](rtc_cntl_pwc_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_PWC_REG {}
#[doc = "RTC_CNTL_PWC_REG"]
pub mod rtc_cntl_pwc_reg;
#[doc = "RTC_CNTL_DIG_PWC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_dig_pwc_reg](rtc_cntl_dig_pwc_reg) module"]
pub type RTC_CNTL_DIG_PWC_REG = crate::Reg<u32, _RTC_CNTL_DIG_PWC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_PWC_REG;
#[doc = "`read()` method returns [rtc_cntl_dig_pwc_reg::R](rtc_cntl_dig_pwc_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_PWC_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_pwc_reg::W](rtc_cntl_dig_pwc_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_PWC_REG {}
#[doc = "RTC_CNTL_DIG_PWC_REG"]
pub mod rtc_cntl_dig_pwc_reg;
#[doc = "RTC_CNTL_DIG_ISO_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_dig_iso_reg](rtc_cntl_dig_iso_reg) module"]
pub type RTC_CNTL_DIG_ISO_REG = crate::Reg<u32, _RTC_CNTL_DIG_ISO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_ISO_REG;
#[doc = "`read()` method returns [rtc_cntl_dig_iso_reg::R](rtc_cntl_dig_iso_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_ISO_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_iso_reg::W](rtc_cntl_dig_iso_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_ISO_REG {}
#[doc = "RTC_CNTL_DIG_ISO_REG"]
pub mod rtc_cntl_dig_iso_reg;
#[doc = "RTC_CNTL_WDTCONFIG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig0_reg](rtc_cntl_wdtconfig0_reg) module"]
pub type RTC_CNTL_WDTCONFIG0_REG = crate::Reg<u32, _RTC_CNTL_WDTCONFIG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG0_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig0_reg::R](rtc_cntl_wdtconfig0_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig0_reg::W](rtc_cntl_wdtconfig0_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG0_REG {}
#[doc = "RTC_CNTL_WDTCONFIG0_REG"]
pub mod rtc_cntl_wdtconfig0_reg;
#[doc = "RTC_CNTL_WDTCONFIG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig1_reg](rtc_cntl_wdtconfig1_reg) module"]
pub type RTC_CNTL_WDTCONFIG1_REG = crate::Reg<u32, _RTC_CNTL_WDTCONFIG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG1_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig1_reg::R](rtc_cntl_wdtconfig1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig1_reg::W](rtc_cntl_wdtconfig1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG1_REG {}
#[doc = "RTC_CNTL_WDTCONFIG1_REG"]
pub mod rtc_cntl_wdtconfig1_reg;
#[doc = "RTC_CNTL_WDTCONFIG2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig2_reg](rtc_cntl_wdtconfig2_reg) module"]
pub type RTC_CNTL_WDTCONFIG2_REG = crate::Reg<u32, _RTC_CNTL_WDTCONFIG2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG2_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig2_reg::R](rtc_cntl_wdtconfig2_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG2_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig2_reg::W](rtc_cntl_wdtconfig2_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG2_REG {}
#[doc = "RTC_CNTL_WDTCONFIG2_REG"]
pub mod rtc_cntl_wdtconfig2_reg;
#[doc = "RTC_CNTL_WDTCONFIG3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig3_reg](rtc_cntl_wdtconfig3_reg) module"]
pub type RTC_CNTL_WDTCONFIG3_REG = crate::Reg<u32, _RTC_CNTL_WDTCONFIG3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG3_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig3_reg::R](rtc_cntl_wdtconfig3_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG3_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig3_reg::W](rtc_cntl_wdtconfig3_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG3_REG {}
#[doc = "RTC_CNTL_WDTCONFIG3_REG"]
pub mod rtc_cntl_wdtconfig3_reg;
#[doc = "RTC_CNTL_WDTCONFIG4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtconfig4_reg](rtc_cntl_wdtconfig4_reg) module"]
pub type RTC_CNTL_WDTCONFIG4_REG = crate::Reg<u32, _RTC_CNTL_WDTCONFIG4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG4_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig4_reg::R](rtc_cntl_wdtconfig4_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG4_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig4_reg::W](rtc_cntl_wdtconfig4_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG4_REG {}
#[doc = "RTC_CNTL_WDTCONFIG4_REG"]
pub mod rtc_cntl_wdtconfig4_reg;
#[doc = "RTC_CNTL_WDTFEED_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtfeed_reg](rtc_cntl_wdtfeed_reg) module"]
pub type RTC_CNTL_WDTFEED_REG = crate::Reg<u32, _RTC_CNTL_WDTFEED_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTFEED_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtfeed_reg::R](rtc_cntl_wdtfeed_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTFEED_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtfeed_reg::W](rtc_cntl_wdtfeed_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTFEED_REG {}
#[doc = "RTC_CNTL_WDTFEED_REG"]
pub mod rtc_cntl_wdtfeed_reg;
#[doc = "RTC_CNTL_WDTWPROTECT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_wdtwprotect_reg](rtc_cntl_wdtwprotect_reg) module"]
pub type RTC_CNTL_WDTWPROTECT_REG = crate::Reg<u32, _RTC_CNTL_WDTWPROTECT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTWPROTECT_REG;
#[doc = "`read()` method returns [rtc_cntl_wdtwprotect_reg::R](rtc_cntl_wdtwprotect_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTWPROTECT_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtwprotect_reg::W](rtc_cntl_wdtwprotect_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTWPROTECT_REG {}
#[doc = "RTC_CNTL_WDTWPROTECT_REG"]
pub mod rtc_cntl_wdtwprotect_reg;
#[doc = "RTC_CNTL_TEST_MUX_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_test_mux_reg](rtc_cntl_test_mux_reg) module"]
pub type RTC_CNTL_TEST_MUX_REG = crate::Reg<u32, _RTC_CNTL_TEST_MUX_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TEST_MUX_REG;
#[doc = "`read()` method returns [rtc_cntl_test_mux_reg::R](rtc_cntl_test_mux_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_TEST_MUX_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_test_mux_reg::W](rtc_cntl_test_mux_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_TEST_MUX_REG {}
#[doc = "RTC_CNTL_TEST_MUX_REG"]
pub mod rtc_cntl_test_mux_reg;
#[doc = "RTC_CNTL_SW_CPU_STALL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_sw_cpu_stall_reg](rtc_cntl_sw_cpu_stall_reg) module"]
pub type RTC_CNTL_SW_CPU_STALL_REG = crate::Reg<u32, _RTC_CNTL_SW_CPU_STALL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SW_CPU_STALL_REG;
#[doc = "`read()` method returns [rtc_cntl_sw_cpu_stall_reg::R](rtc_cntl_sw_cpu_stall_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_SW_CPU_STALL_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sw_cpu_stall_reg::W](rtc_cntl_sw_cpu_stall_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_SW_CPU_STALL_REG {}
#[doc = "RTC_CNTL_SW_CPU_STALL_REG"]
pub mod rtc_cntl_sw_cpu_stall_reg;
#[doc = "RTC_CNTL_STORE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store4_reg](rtc_cntl_store4_reg) module"]
pub type RTC_CNTL_STORE4_REG = crate::Reg<u32, _RTC_CNTL_STORE4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE4_REG;
#[doc = "`read()` method returns [rtc_cntl_store4_reg::R](rtc_cntl_store4_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE4_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store4_reg::W](rtc_cntl_store4_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE4_REG {}
#[doc = "RTC_CNTL_STORE4_REG"]
pub mod rtc_cntl_store4_reg;
#[doc = "RTC_CNTL_STORE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store5_reg](rtc_cntl_store5_reg) module"]
pub type RTC_CNTL_STORE5_REG = crate::Reg<u32, _RTC_CNTL_STORE5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE5_REG;
#[doc = "`read()` method returns [rtc_cntl_store5_reg::R](rtc_cntl_store5_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE5_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store5_reg::W](rtc_cntl_store5_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE5_REG {}
#[doc = "RTC_CNTL_STORE5_REG"]
pub mod rtc_cntl_store5_reg;
#[doc = "RTC_CNTL_STORE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store6_reg](rtc_cntl_store6_reg) module"]
pub type RTC_CNTL_STORE6_REG = crate::Reg<u32, _RTC_CNTL_STORE6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE6_REG;
#[doc = "`read()` method returns [rtc_cntl_store6_reg::R](rtc_cntl_store6_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE6_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store6_reg::W](rtc_cntl_store6_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE6_REG {}
#[doc = "RTC_CNTL_STORE6_REG"]
pub mod rtc_cntl_store6_reg;
#[doc = "RTC_CNTL_STORE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_store7_reg](rtc_cntl_store7_reg) module"]
pub type RTC_CNTL_STORE7_REG = crate::Reg<u32, _RTC_CNTL_STORE7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE7_REG;
#[doc = "`read()` method returns [rtc_cntl_store7_reg::R](rtc_cntl_store7_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE7_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store7_reg::W](rtc_cntl_store7_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE7_REG {}
#[doc = "RTC_CNTL_STORE7_REG"]
pub mod rtc_cntl_store7_reg;
#[doc = "RTC_CNTL_DIAG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_diag1_reg](rtc_cntl_diag1_reg) module"]
pub type RTC_CNTL_DIAG1_REG = crate::Reg<u32, _RTC_CNTL_DIAG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIAG1_REG;
#[doc = "`read()` method returns [rtc_cntl_diag1_reg::R](rtc_cntl_diag1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIAG1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_diag1_reg::W](rtc_cntl_diag1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIAG1_REG {}
#[doc = "RTC_CNTL_DIAG1_REG"]
pub mod rtc_cntl_diag1_reg;
#[doc = "RTC_CNTL_HOLD_FORCE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_hold_force_reg](rtc_cntl_hold_force_reg) module"]
pub type RTC_CNTL_HOLD_FORCE_REG = crate::Reg<u32, _RTC_CNTL_HOLD_FORCE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_HOLD_FORCE_REG;
#[doc = "`read()` method returns [rtc_cntl_hold_force_reg::R](rtc_cntl_hold_force_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_HOLD_FORCE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_hold_force_reg::W](rtc_cntl_hold_force_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_HOLD_FORCE_REG {}
#[doc = "RTC_CNTL_HOLD_FORCE_REG"]
pub mod rtc_cntl_hold_force_reg;
#[doc = "RTC_CNTL_EXT_WAKEUP1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_wakeup1_reg](rtc_cntl_ext_wakeup1_reg) module"]
pub type RTC_CNTL_EXT_WAKEUP1_REG = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP1_REG;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup1_reg::R](rtc_cntl_ext_wakeup1_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup1_reg::W](rtc_cntl_ext_wakeup1_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP1_REG {}
#[doc = "RTC_CNTL_EXT_WAKEUP1_REG"]
pub mod rtc_cntl_ext_wakeup1_reg;
#[doc = "RTC_CNTL_EXT_WAKEUP1_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_ext_wakeup1_status_reg](rtc_cntl_ext_wakeup1_status_reg) module"]
pub type RTC_CNTL_EXT_WAKEUP1_STATUS_REG = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP1_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP1_STATUS_REG;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup1_status_reg::R](rtc_cntl_ext_wakeup1_status_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP1_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup1_status_reg::W](rtc_cntl_ext_wakeup1_status_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP1_STATUS_REG {}
#[doc = "RTC_CNTL_EXT_WAKEUP1_STATUS_REG"]
pub mod rtc_cntl_ext_wakeup1_status_reg;
#[doc = "RTC_CNTL_BROWN_OUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_brown_out_reg](rtc_cntl_brown_out_reg) module"]
pub type RTC_CNTL_BROWN_OUT_REG = crate::Reg<u32, _RTC_CNTL_BROWN_OUT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_BROWN_OUT_REG;
#[doc = "`read()` method returns [rtc_cntl_brown_out_reg::R](rtc_cntl_brown_out_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_BROWN_OUT_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_brown_out_reg::W](rtc_cntl_brown_out_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_BROWN_OUT_REG {}
#[doc = "RTC_CNTL_BROWN_OUT_REG"]
pub mod rtc_cntl_brown_out_reg;
#[doc = "RTC_CNTL_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_cntl_date_reg](rtc_cntl_date_reg) module"]
pub type RTC_CNTL_DATE_REG = crate::Reg<u32, _RTC_CNTL_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DATE_REG;
#[doc = "`read()` method returns [rtc_cntl_date_reg::R](rtc_cntl_date_reg::R) reader structure"]
impl crate::Readable for RTC_CNTL_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_date_reg::W](rtc_cntl_date_reg::W) writer structure"]
impl crate::Writable for RTC_CNTL_DATE_REG {}
#[doc = "RTC_CNTL_DATE_REG"]
pub mod rtc_cntl_date_reg;
