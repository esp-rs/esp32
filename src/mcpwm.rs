#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCPWM_CLK_CFG_REG(i)"]
    pub mcpwm_clk_cfg_reg: MCPWM_CLK_CFG_REG,
    #[doc = "0x04 - MCPWM_TIMER0_CFG0_REG(i)"]
    pub mcpwm_timer0_cfg0_reg: MCPWM_TIMER0_CFG0_REG,
    #[doc = "0x08 - MCPWM_TIMER0_CFG1_REG(i)"]
    pub mcpwm_timer0_cfg1_reg: MCPWM_TIMER0_CFG1_REG,
    #[doc = "0x0c - MCPWM_TIMER0_SYNC_REG(i)"]
    pub mcpwm_timer0_sync_reg: MCPWM_TIMER0_SYNC_REG,
    #[doc = "0x10 - MCPWM_TIMER0_STATUS_REG(i)"]
    pub mcpwm_timer0_status_reg: MCPWM_TIMER0_STATUS_REG,
    #[doc = "0x14 - MCPWM_TIMER1_CFG0_REG(i)"]
    pub mcpwm_timer1_cfg0_reg: MCPWM_TIMER1_CFG0_REG,
    #[doc = "0x18 - MCPWM_TIMER1_CFG1_REG(i)"]
    pub mcpwm_timer1_cfg1_reg: MCPWM_TIMER1_CFG1_REG,
    #[doc = "0x1c - MCPWM_TIMER1_SYNC_REG(i)"]
    pub mcpwm_timer1_sync_reg: MCPWM_TIMER1_SYNC_REG,
    #[doc = "0x20 - MCPWM_TIMER1_STATUS_REG(i)"]
    pub mcpwm_timer1_status_reg: MCPWM_TIMER1_STATUS_REG,
    #[doc = "0x24 - MCPWM_TIMER2_CFG0_REG(i)"]
    pub mcpwm_timer2_cfg0_reg: MCPWM_TIMER2_CFG0_REG,
    #[doc = "0x28 - MCPWM_TIMER2_CFG1_REG(i)"]
    pub mcpwm_timer2_cfg1_reg: MCPWM_TIMER2_CFG1_REG,
    #[doc = "0x2c - MCPWM_TIMER2_SYNC_REG(i)"]
    pub mcpwm_timer2_sync_reg: MCPWM_TIMER2_SYNC_REG,
    #[doc = "0x30 - MCPWM_TIMER2_STATUS_REG(i)"]
    pub mcpwm_timer2_status_reg: MCPWM_TIMER2_STATUS_REG,
    #[doc = "0x34 - MCPWM_TIMER_SYNCI_CFG_REG(i)"]
    pub mcpwm_timer_synci_cfg_reg: MCPWM_TIMER_SYNCI_CFG_REG,
    #[doc = "0x38 - MCPWM_OPERATOR_TIMERSEL_REG(i)"]
    pub mcpwm_operator_timersel_reg: MCPWM_OPERATOR_TIMERSEL_REG,
    #[doc = "0x3c - MCPWM_GEN0_STMP_CFG_REG(i)"]
    pub mcpwm_gen0_stmp_cfg_reg: MCPWM_GEN0_STMP_CFG_REG,
    #[doc = "0x40 - MCPWM_GEN0_TSTMP_A_REG(i)"]
    pub mcpwm_gen0_tstmp_a_reg: MCPWM_GEN0_TSTMP_A_REG,
    #[doc = "0x44 - MCPWM_GEN0_TSTMP_B_REG(i)"]
    pub mcpwm_gen0_tstmp_b_reg: MCPWM_GEN0_TSTMP_B_REG,
    #[doc = "0x48 - MCPWM_GEN0_CFG0_REG(i)"]
    pub mcpwm_gen0_cfg0_reg: MCPWM_GEN0_CFG0_REG,
    #[doc = "0x4c - MCPWM_GEN0_FORCE_REG(i)"]
    pub mcpwm_gen0_force_reg: MCPWM_GEN0_FORCE_REG,
    #[doc = "0x50 - MCPWM_GEN0_A_REG(i)"]
    pub mcpwm_gen0_a_reg: MCPWM_GEN0_A_REG,
    #[doc = "0x54 - MCPWM_GEN0_B_REG(i)"]
    pub mcpwm_gen0_b_reg: MCPWM_GEN0_B_REG,
    #[doc = "0x58 - MCPWM_DT0_CFG_REG(i)"]
    pub mcpwm_dt0_cfg_reg: MCPWM_DT0_CFG_REG,
    #[doc = "0x5c - MCPWM_DT0_FED_CFG_REG(i)"]
    pub mcpwm_dt0_fed_cfg_reg: MCPWM_DT0_FED_CFG_REG,
    #[doc = "0x60 - MCPWM_DT0_RED_CFG_REG(i)"]
    pub mcpwm_dt0_red_cfg_reg: MCPWM_DT0_RED_CFG_REG,
    #[doc = "0x64 - MCPWM_CARRIER0_CFG_REG(i)"]
    pub mcpwm_carrier0_cfg_reg: MCPWM_CARRIER0_CFG_REG,
    #[doc = "0x68 - MCPWM_FH0_CFG0_REG(i)"]
    pub mcpwm_fh0_cfg0_reg: MCPWM_FH0_CFG0_REG,
    #[doc = "0x6c - MCPWM_FH0_CFG1_REG(i)"]
    pub mcpwm_fh0_cfg1_reg: MCPWM_FH0_CFG1_REG,
    #[doc = "0x70 - MCPWM_FH0_STATUS_REG(i)"]
    pub mcpwm_fh0_status_reg: MCPWM_FH0_STATUS_REG,
    #[doc = "0x74 - MCPWM_GEN1_STMP_CFG_REG(i)"]
    pub mcpwm_gen1_stmp_cfg_reg: MCPWM_GEN1_STMP_CFG_REG,
    #[doc = "0x78 - MCPWM_GEN1_TSTMP_A_REG(i)"]
    pub mcpwm_gen1_tstmp_a_reg: MCPWM_GEN1_TSTMP_A_REG,
    #[doc = "0x7c - MCPWM_GEN1_TSTMP_B_REG(i)"]
    pub mcpwm_gen1_tstmp_b_reg: MCPWM_GEN1_TSTMP_B_REG,
    #[doc = "0x80 - MCPWM_GEN1_CFG0_REG(i)"]
    pub mcpwm_gen1_cfg0_reg: MCPWM_GEN1_CFG0_REG,
    #[doc = "0x84 - MCPWM_GEN1_FORCE_REG(i)"]
    pub mcpwm_gen1_force_reg: MCPWM_GEN1_FORCE_REG,
    #[doc = "0x88 - MCPWM_GEN1_A_REG(i)"]
    pub mcpwm_gen1_a_reg: MCPWM_GEN1_A_REG,
    #[doc = "0x8c - MCPWM_GEN1_B_REG(i)"]
    pub mcpwm_gen1_b_reg: MCPWM_GEN1_B_REG,
    #[doc = "0x90 - MCPWM_DT1_CFG_REG(i)"]
    pub mcpwm_dt1_cfg_reg: MCPWM_DT1_CFG_REG,
    #[doc = "0x94 - MCPWM_DT1_FED_CFG_REG(i)"]
    pub mcpwm_dt1_fed_cfg_reg: MCPWM_DT1_FED_CFG_REG,
    #[doc = "0x98 - MCPWM_DT1_RED_CFG_REG(i)"]
    pub mcpwm_dt1_red_cfg_reg: MCPWM_DT1_RED_CFG_REG,
    #[doc = "0x9c - MCPWM_CARRIER1_CFG_REG(i)"]
    pub mcpwm_carrier1_cfg_reg: MCPWM_CARRIER1_CFG_REG,
    #[doc = "0xa0 - MCPWM_FH1_CFG0_REG(i)"]
    pub mcpwm_fh1_cfg0_reg: MCPWM_FH1_CFG0_REG,
    #[doc = "0xa4 - MCPWM_FH1_CFG1_REG(i)"]
    pub mcpwm_fh1_cfg1_reg: MCPWM_FH1_CFG1_REG,
    #[doc = "0xa8 - MCPWM_FH1_STATUS_REG(i)"]
    pub mcpwm_fh1_status_reg: MCPWM_FH1_STATUS_REG,
    #[doc = "0xac - MCPWM_GEN2_STMP_CFG_REG(i)"]
    pub mcpwm_gen2_stmp_cfg_reg: MCPWM_GEN2_STMP_CFG_REG,
    #[doc = "0xb0 - MCPWM_GEN2_TSTMP_A_REG(i)"]
    pub mcpwm_gen2_tstmp_a_reg: MCPWM_GEN2_TSTMP_A_REG,
    #[doc = "0xb4 - MCPWM_GEN2_TSTMP_B_REG(i)"]
    pub mcpwm_gen2_tstmp_b_reg: MCPWM_GEN2_TSTMP_B_REG,
    #[doc = "0xb8 - MCPWM_GEN2_CFG0_REG(i)"]
    pub mcpwm_gen2_cfg0_reg: MCPWM_GEN2_CFG0_REG,
    #[doc = "0xbc - MCPWM_GEN2_FORCE_REG(i)"]
    pub mcpwm_gen2_force_reg: MCPWM_GEN2_FORCE_REG,
    #[doc = "0xc0 - MCPWM_GEN2_A_REG(i)"]
    pub mcpwm_gen2_a_reg: MCPWM_GEN2_A_REG,
    #[doc = "0xc4 - MCPWM_GEN2_B_REG(i)"]
    pub mcpwm_gen2_b_reg: MCPWM_GEN2_B_REG,
    #[doc = "0xc8 - MCPWM_DT2_CFG_REG(i)"]
    pub mcpwm_dt2_cfg_reg: MCPWM_DT2_CFG_REG,
    #[doc = "0xcc - MCPWM_DT2_FED_CFG_REG(i)"]
    pub mcpwm_dt2_fed_cfg_reg: MCPWM_DT2_FED_CFG_REG,
    #[doc = "0xd0 - MCPWM_DT2_RED_CFG_REG(i)"]
    pub mcpwm_dt2_red_cfg_reg: MCPWM_DT2_RED_CFG_REG,
    #[doc = "0xd4 - MCPWM_CARRIER2_CFG_REG(i)"]
    pub mcpwm_carrier2_cfg_reg: MCPWM_CARRIER2_CFG_REG,
    #[doc = "0xd8 - MCPWM_FH2_CFG0_REG(i)"]
    pub mcpwm_fh2_cfg0_reg: MCPWM_FH2_CFG0_REG,
    #[doc = "0xdc - MCPWM_FH2_CFG1_REG(i)"]
    pub mcpwm_fh2_cfg1_reg: MCPWM_FH2_CFG1_REG,
    #[doc = "0xe0 - MCPWM_FH2_STATUS_REG(i)"]
    pub mcpwm_fh2_status_reg: MCPWM_FH2_STATUS_REG,
    #[doc = "0xe4 - MCPWM_FAULT_DETECT_REG(i)"]
    pub mcpwm_fault_detect_reg: MCPWM_FAULT_DETECT_REG,
    #[doc = "0xe8 - MCPWM_CAP_TIMER_CFG_REG(i)"]
    pub mcpwm_cap_timer_cfg_reg: MCPWM_CAP_TIMER_CFG_REG,
    #[doc = "0xec - MCPWM_CAP_TIMER_PHASE_REG(i)"]
    pub mcpwm_cap_timer_phase_reg: MCPWM_CAP_TIMER_PHASE_REG,
    #[doc = "0xf0 - MCPWM_CAP_CH0_CFG_REG(i)"]
    pub mcpwm_cap_ch0_cfg_reg: MCPWM_CAP_CH0_CFG_REG,
    #[doc = "0xf4 - MCPWM_CAP_CH1_CFG_REG(i)"]
    pub mcpwm_cap_ch1_cfg_reg: MCPWM_CAP_CH1_CFG_REG,
    #[doc = "0xf8 - MCPWM_CAP_CH2_CFG_REG(i)"]
    pub mcpwm_cap_ch2_cfg_reg: MCPWM_CAP_CH2_CFG_REG,
    #[doc = "0xfc - MCPWM_CAP_CH0_REG(i)"]
    pub mcpwm_cap_ch0_reg: MCPWM_CAP_CH0_REG,
    #[doc = "0x100 - MCPWM_CAP_CH1_REG(i)"]
    pub mcpwm_cap_ch1_reg: MCPWM_CAP_CH1_REG,
    #[doc = "0x104 - MCPWM_CAP_CH2_REG(i)"]
    pub mcpwm_cap_ch2_reg: MCPWM_CAP_CH2_REG,
    #[doc = "0x108 - MCPWM_CAP_STATUS_REG(i)"]
    pub mcpwm_cap_status_reg: MCPWM_CAP_STATUS_REG,
    #[doc = "0x10c - MCPWM_UPDATE_CFG_REG(i)"]
    pub mcpwm_update_cfg_reg: MCPWM_UPDATE_CFG_REG,
    #[doc = "0x110 - MCMCPWM_INT_ENA_MCPWM_REG(i)"]
    pub mcmcpwm_int_ena_mcpwm_reg: MCMCPWM_INT_ENA_MCPWM_REG,
    #[doc = "0x114 - MCMCPWM_INT_RAW_MCPWM_REG(i)"]
    pub mcmcpwm_int_raw_mcpwm_reg: MCMCPWM_INT_RAW_MCPWM_REG,
    #[doc = "0x118 - MCMCPWM_INT_ST_MCPWM_REG(i)"]
    pub mcmcpwm_int_st_mcpwm_reg: MCMCPWM_INT_ST_MCPWM_REG,
    #[doc = "0x11c - MCMCPWM_INT_CLR_MCPWM_REG(i)"]
    pub mcmcpwm_int_clr_mcpwm_reg: MCMCPWM_INT_CLR_MCPWM_REG,
    #[doc = "0x120 - MCPWM_CLK_REG(i)"]
    pub mcpwm_clk_reg: MCPWM_CLK_REG,
    #[doc = "0x124 - MCPWM_VERSION_REG(i)"]
    pub mcpwm_version_reg: MCPWM_VERSION_REG,
}
#[doc = "MCPWM_CLK_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_clk_cfg_reg](mcpwm_clk_cfg_reg) module"]
pub type MCPWM_CLK_CFG_REG = crate::Reg<u32, _MCPWM_CLK_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CLK_CFG_REG;
#[doc = "`read()` method returns [mcpwm_clk_cfg_reg::R](mcpwm_clk_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CLK_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_clk_cfg_reg::W](mcpwm_clk_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CLK_CFG_REG {}
#[doc = "MCPWM_CLK_CFG_REG(i)"]
pub mod mcpwm_clk_cfg_reg;
#[doc = "MCPWM_TIMER0_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer0_cfg0_reg](mcpwm_timer0_cfg0_reg) module"]
pub type MCPWM_TIMER0_CFG0_REG = crate::Reg<u32, _MCPWM_TIMER0_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER0_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_timer0_cfg0_reg::R](mcpwm_timer0_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER0_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer0_cfg0_reg::W](mcpwm_timer0_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER0_CFG0_REG {}
#[doc = "MCPWM_TIMER0_CFG0_REG(i)"]
pub mod mcpwm_timer0_cfg0_reg;
#[doc = "MCPWM_TIMER0_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer0_cfg1_reg](mcpwm_timer0_cfg1_reg) module"]
pub type MCPWM_TIMER0_CFG1_REG = crate::Reg<u32, _MCPWM_TIMER0_CFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER0_CFG1_REG;
#[doc = "`read()` method returns [mcpwm_timer0_cfg1_reg::R](mcpwm_timer0_cfg1_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER0_CFG1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer0_cfg1_reg::W](mcpwm_timer0_cfg1_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER0_CFG1_REG {}
#[doc = "MCPWM_TIMER0_CFG1_REG(i)"]
pub mod mcpwm_timer0_cfg1_reg;
#[doc = "MCPWM_TIMER0_SYNC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer0_sync_reg](mcpwm_timer0_sync_reg) module"]
pub type MCPWM_TIMER0_SYNC_REG = crate::Reg<u32, _MCPWM_TIMER0_SYNC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER0_SYNC_REG;
#[doc = "`read()` method returns [mcpwm_timer0_sync_reg::R](mcpwm_timer0_sync_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER0_SYNC_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer0_sync_reg::W](mcpwm_timer0_sync_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER0_SYNC_REG {}
#[doc = "MCPWM_TIMER0_SYNC_REG(i)"]
pub mod mcpwm_timer0_sync_reg;
#[doc = "MCPWM_TIMER0_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer0_status_reg](mcpwm_timer0_status_reg) module"]
pub type MCPWM_TIMER0_STATUS_REG = crate::Reg<u32, _MCPWM_TIMER0_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER0_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_timer0_status_reg::R](mcpwm_timer0_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER0_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer0_status_reg::W](mcpwm_timer0_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER0_STATUS_REG {}
#[doc = "MCPWM_TIMER0_STATUS_REG(i)"]
pub mod mcpwm_timer0_status_reg;
#[doc = "MCPWM_TIMER1_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer1_cfg0_reg](mcpwm_timer1_cfg0_reg) module"]
pub type MCPWM_TIMER1_CFG0_REG = crate::Reg<u32, _MCPWM_TIMER1_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER1_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_timer1_cfg0_reg::R](mcpwm_timer1_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER1_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer1_cfg0_reg::W](mcpwm_timer1_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER1_CFG0_REG {}
#[doc = "MCPWM_TIMER1_CFG0_REG(i)"]
pub mod mcpwm_timer1_cfg0_reg;
#[doc = "MCPWM_TIMER1_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer1_cfg1_reg](mcpwm_timer1_cfg1_reg) module"]
pub type MCPWM_TIMER1_CFG1_REG = crate::Reg<u32, _MCPWM_TIMER1_CFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER1_CFG1_REG;
#[doc = "`read()` method returns [mcpwm_timer1_cfg1_reg::R](mcpwm_timer1_cfg1_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER1_CFG1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer1_cfg1_reg::W](mcpwm_timer1_cfg1_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER1_CFG1_REG {}
#[doc = "MCPWM_TIMER1_CFG1_REG(i)"]
pub mod mcpwm_timer1_cfg1_reg;
#[doc = "MCPWM_TIMER1_SYNC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer1_sync_reg](mcpwm_timer1_sync_reg) module"]
pub type MCPWM_TIMER1_SYNC_REG = crate::Reg<u32, _MCPWM_TIMER1_SYNC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER1_SYNC_REG;
#[doc = "`read()` method returns [mcpwm_timer1_sync_reg::R](mcpwm_timer1_sync_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER1_SYNC_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer1_sync_reg::W](mcpwm_timer1_sync_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER1_SYNC_REG {}
#[doc = "MCPWM_TIMER1_SYNC_REG(i)"]
pub mod mcpwm_timer1_sync_reg;
#[doc = "MCPWM_TIMER1_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer1_status_reg](mcpwm_timer1_status_reg) module"]
pub type MCPWM_TIMER1_STATUS_REG = crate::Reg<u32, _MCPWM_TIMER1_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER1_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_timer1_status_reg::R](mcpwm_timer1_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER1_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer1_status_reg::W](mcpwm_timer1_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER1_STATUS_REG {}
#[doc = "MCPWM_TIMER1_STATUS_REG(i)"]
pub mod mcpwm_timer1_status_reg;
#[doc = "MCPWM_TIMER2_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer2_cfg0_reg](mcpwm_timer2_cfg0_reg) module"]
pub type MCPWM_TIMER2_CFG0_REG = crate::Reg<u32, _MCPWM_TIMER2_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER2_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_timer2_cfg0_reg::R](mcpwm_timer2_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER2_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer2_cfg0_reg::W](mcpwm_timer2_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER2_CFG0_REG {}
#[doc = "MCPWM_TIMER2_CFG0_REG(i)"]
pub mod mcpwm_timer2_cfg0_reg;
#[doc = "MCPWM_TIMER2_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer2_cfg1_reg](mcpwm_timer2_cfg1_reg) module"]
pub type MCPWM_TIMER2_CFG1_REG = crate::Reg<u32, _MCPWM_TIMER2_CFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER2_CFG1_REG;
#[doc = "`read()` method returns [mcpwm_timer2_cfg1_reg::R](mcpwm_timer2_cfg1_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER2_CFG1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer2_cfg1_reg::W](mcpwm_timer2_cfg1_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER2_CFG1_REG {}
#[doc = "MCPWM_TIMER2_CFG1_REG(i)"]
pub mod mcpwm_timer2_cfg1_reg;
#[doc = "MCPWM_TIMER2_SYNC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer2_sync_reg](mcpwm_timer2_sync_reg) module"]
pub type MCPWM_TIMER2_SYNC_REG = crate::Reg<u32, _MCPWM_TIMER2_SYNC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER2_SYNC_REG;
#[doc = "`read()` method returns [mcpwm_timer2_sync_reg::R](mcpwm_timer2_sync_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER2_SYNC_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer2_sync_reg::W](mcpwm_timer2_sync_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER2_SYNC_REG {}
#[doc = "MCPWM_TIMER2_SYNC_REG(i)"]
pub mod mcpwm_timer2_sync_reg;
#[doc = "MCPWM_TIMER2_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer2_status_reg](mcpwm_timer2_status_reg) module"]
pub type MCPWM_TIMER2_STATUS_REG = crate::Reg<u32, _MCPWM_TIMER2_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER2_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_timer2_status_reg::R](mcpwm_timer2_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER2_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer2_status_reg::W](mcpwm_timer2_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER2_STATUS_REG {}
#[doc = "MCPWM_TIMER2_STATUS_REG(i)"]
pub mod mcpwm_timer2_status_reg;
#[doc = "MCPWM_TIMER_SYNCI_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_timer_synci_cfg_reg](mcpwm_timer_synci_cfg_reg) module"]
pub type MCPWM_TIMER_SYNCI_CFG_REG = crate::Reg<u32, _MCPWM_TIMER_SYNCI_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_TIMER_SYNCI_CFG_REG;
#[doc = "`read()` method returns [mcpwm_timer_synci_cfg_reg::R](mcpwm_timer_synci_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_TIMER_SYNCI_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_timer_synci_cfg_reg::W](mcpwm_timer_synci_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_TIMER_SYNCI_CFG_REG {}
#[doc = "MCPWM_TIMER_SYNCI_CFG_REG(i)"]
pub mod mcpwm_timer_synci_cfg_reg;
#[doc = "MCPWM_OPERATOR_TIMERSEL_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_operator_timersel_reg](mcpwm_operator_timersel_reg) module"]
pub type MCPWM_OPERATOR_TIMERSEL_REG = crate::Reg<u32, _MCPWM_OPERATOR_TIMERSEL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_OPERATOR_TIMERSEL_REG;
#[doc = "`read()` method returns [mcpwm_operator_timersel_reg::R](mcpwm_operator_timersel_reg::R) reader structure"]
impl crate::Readable for MCPWM_OPERATOR_TIMERSEL_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_operator_timersel_reg::W](mcpwm_operator_timersel_reg::W) writer structure"]
impl crate::Writable for MCPWM_OPERATOR_TIMERSEL_REG {}
#[doc = "MCPWM_OPERATOR_TIMERSEL_REG(i)"]
pub mod mcpwm_operator_timersel_reg;
#[doc = "MCPWM_GEN0_STMP_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_stmp_cfg_reg](mcpwm_gen0_stmp_cfg_reg) module"]
pub type MCPWM_GEN0_STMP_CFG_REG = crate::Reg<u32, _MCPWM_GEN0_STMP_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_STMP_CFG_REG;
#[doc = "`read()` method returns [mcpwm_gen0_stmp_cfg_reg::R](mcpwm_gen0_stmp_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_STMP_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_stmp_cfg_reg::W](mcpwm_gen0_stmp_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_STMP_CFG_REG {}
#[doc = "MCPWM_GEN0_STMP_CFG_REG(i)"]
pub mod mcpwm_gen0_stmp_cfg_reg;
#[doc = "MCPWM_GEN0_TSTMP_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_tstmp_a_reg](mcpwm_gen0_tstmp_a_reg) module"]
pub type MCPWM_GEN0_TSTMP_A_REG = crate::Reg<u32, _MCPWM_GEN0_TSTMP_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_TSTMP_A_REG;
#[doc = "`read()` method returns [mcpwm_gen0_tstmp_a_reg::R](mcpwm_gen0_tstmp_a_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_TSTMP_A_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_tstmp_a_reg::W](mcpwm_gen0_tstmp_a_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_TSTMP_A_REG {}
#[doc = "MCPWM_GEN0_TSTMP_A_REG(i)"]
pub mod mcpwm_gen0_tstmp_a_reg;
#[doc = "MCPWM_GEN0_TSTMP_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_tstmp_b_reg](mcpwm_gen0_tstmp_b_reg) module"]
pub type MCPWM_GEN0_TSTMP_B_REG = crate::Reg<u32, _MCPWM_GEN0_TSTMP_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_TSTMP_B_REG;
#[doc = "`read()` method returns [mcpwm_gen0_tstmp_b_reg::R](mcpwm_gen0_tstmp_b_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_TSTMP_B_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_tstmp_b_reg::W](mcpwm_gen0_tstmp_b_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_TSTMP_B_REG {}
#[doc = "MCPWM_GEN0_TSTMP_B_REG(i)"]
pub mod mcpwm_gen0_tstmp_b_reg;
#[doc = "MCPWM_GEN0_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_cfg0_reg](mcpwm_gen0_cfg0_reg) module"]
pub type MCPWM_GEN0_CFG0_REG = crate::Reg<u32, _MCPWM_GEN0_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_gen0_cfg0_reg::R](mcpwm_gen0_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_cfg0_reg::W](mcpwm_gen0_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_CFG0_REG {}
#[doc = "MCPWM_GEN0_CFG0_REG(i)"]
pub mod mcpwm_gen0_cfg0_reg;
#[doc = "MCPWM_GEN0_FORCE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_force_reg](mcpwm_gen0_force_reg) module"]
pub type MCPWM_GEN0_FORCE_REG = crate::Reg<u32, _MCPWM_GEN0_FORCE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_FORCE_REG;
#[doc = "`read()` method returns [mcpwm_gen0_force_reg::R](mcpwm_gen0_force_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_FORCE_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_force_reg::W](mcpwm_gen0_force_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_FORCE_REG {}
#[doc = "MCPWM_GEN0_FORCE_REG(i)"]
pub mod mcpwm_gen0_force_reg;
#[doc = "MCPWM_GEN0_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_a_reg](mcpwm_gen0_a_reg) module"]
pub type MCPWM_GEN0_A_REG = crate::Reg<u32, _MCPWM_GEN0_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_A_REG;
#[doc = "`read()` method returns [mcpwm_gen0_a_reg::R](mcpwm_gen0_a_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_A_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_a_reg::W](mcpwm_gen0_a_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_A_REG {}
#[doc = "MCPWM_GEN0_A_REG(i)"]
pub mod mcpwm_gen0_a_reg;
#[doc = "MCPWM_GEN0_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen0_b_reg](mcpwm_gen0_b_reg) module"]
pub type MCPWM_GEN0_B_REG = crate::Reg<u32, _MCPWM_GEN0_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN0_B_REG;
#[doc = "`read()` method returns [mcpwm_gen0_b_reg::R](mcpwm_gen0_b_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN0_B_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen0_b_reg::W](mcpwm_gen0_b_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN0_B_REG {}
#[doc = "MCPWM_GEN0_B_REG(i)"]
pub mod mcpwm_gen0_b_reg;
#[doc = "MCPWM_DT0_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt0_cfg_reg](mcpwm_dt0_cfg_reg) module"]
pub type MCPWM_DT0_CFG_REG = crate::Reg<u32, _MCPWM_DT0_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT0_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt0_cfg_reg::R](mcpwm_dt0_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT0_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt0_cfg_reg::W](mcpwm_dt0_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT0_CFG_REG {}
#[doc = "MCPWM_DT0_CFG_REG(i)"]
pub mod mcpwm_dt0_cfg_reg;
#[doc = "MCPWM_DT0_FED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt0_fed_cfg_reg](mcpwm_dt0_fed_cfg_reg) module"]
pub type MCPWM_DT0_FED_CFG_REG = crate::Reg<u32, _MCPWM_DT0_FED_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT0_FED_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt0_fed_cfg_reg::R](mcpwm_dt0_fed_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT0_FED_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt0_fed_cfg_reg::W](mcpwm_dt0_fed_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT0_FED_CFG_REG {}
#[doc = "MCPWM_DT0_FED_CFG_REG(i)"]
pub mod mcpwm_dt0_fed_cfg_reg;
#[doc = "MCPWM_DT0_RED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt0_red_cfg_reg](mcpwm_dt0_red_cfg_reg) module"]
pub type MCPWM_DT0_RED_CFG_REG = crate::Reg<u32, _MCPWM_DT0_RED_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT0_RED_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt0_red_cfg_reg::R](mcpwm_dt0_red_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT0_RED_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt0_red_cfg_reg::W](mcpwm_dt0_red_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT0_RED_CFG_REG {}
#[doc = "MCPWM_DT0_RED_CFG_REG(i)"]
pub mod mcpwm_dt0_red_cfg_reg;
#[doc = "MCPWM_CARRIER0_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_carrier0_cfg_reg](mcpwm_carrier0_cfg_reg) module"]
pub type MCPWM_CARRIER0_CFG_REG = crate::Reg<u32, _MCPWM_CARRIER0_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CARRIER0_CFG_REG;
#[doc = "`read()` method returns [mcpwm_carrier0_cfg_reg::R](mcpwm_carrier0_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CARRIER0_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_carrier0_cfg_reg::W](mcpwm_carrier0_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CARRIER0_CFG_REG {}
#[doc = "MCPWM_CARRIER0_CFG_REG(i)"]
pub mod mcpwm_carrier0_cfg_reg;
#[doc = "MCPWM_FH0_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh0_cfg0_reg](mcpwm_fh0_cfg0_reg) module"]
pub type MCPWM_FH0_CFG0_REG = crate::Reg<u32, _MCPWM_FH0_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH0_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_fh0_cfg0_reg::R](mcpwm_fh0_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH0_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh0_cfg0_reg::W](mcpwm_fh0_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH0_CFG0_REG {}
#[doc = "MCPWM_FH0_CFG0_REG(i)"]
pub mod mcpwm_fh0_cfg0_reg;
#[doc = "MCPWM_FH0_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh0_cfg1_reg](mcpwm_fh0_cfg1_reg) module"]
pub type MCPWM_FH0_CFG1_REG = crate::Reg<u32, _MCPWM_FH0_CFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH0_CFG1_REG;
#[doc = "`read()` method returns [mcpwm_fh0_cfg1_reg::R](mcpwm_fh0_cfg1_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH0_CFG1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh0_cfg1_reg::W](mcpwm_fh0_cfg1_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH0_CFG1_REG {}
#[doc = "MCPWM_FH0_CFG1_REG(i)"]
pub mod mcpwm_fh0_cfg1_reg;
#[doc = "MCPWM_FH0_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh0_status_reg](mcpwm_fh0_status_reg) module"]
pub type MCPWM_FH0_STATUS_REG = crate::Reg<u32, _MCPWM_FH0_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH0_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_fh0_status_reg::R](mcpwm_fh0_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH0_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh0_status_reg::W](mcpwm_fh0_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH0_STATUS_REG {}
#[doc = "MCPWM_FH0_STATUS_REG(i)"]
pub mod mcpwm_fh0_status_reg;
#[doc = "MCPWM_GEN1_STMP_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_stmp_cfg_reg](mcpwm_gen1_stmp_cfg_reg) module"]
pub type MCPWM_GEN1_STMP_CFG_REG = crate::Reg<u32, _MCPWM_GEN1_STMP_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_STMP_CFG_REG;
#[doc = "`read()` method returns [mcpwm_gen1_stmp_cfg_reg::R](mcpwm_gen1_stmp_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_STMP_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_stmp_cfg_reg::W](mcpwm_gen1_stmp_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_STMP_CFG_REG {}
#[doc = "MCPWM_GEN1_STMP_CFG_REG(i)"]
pub mod mcpwm_gen1_stmp_cfg_reg;
#[doc = "MCPWM_GEN1_TSTMP_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_tstmp_a_reg](mcpwm_gen1_tstmp_a_reg) module"]
pub type MCPWM_GEN1_TSTMP_A_REG = crate::Reg<u32, _MCPWM_GEN1_TSTMP_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_TSTMP_A_REG;
#[doc = "`read()` method returns [mcpwm_gen1_tstmp_a_reg::R](mcpwm_gen1_tstmp_a_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_TSTMP_A_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_tstmp_a_reg::W](mcpwm_gen1_tstmp_a_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_TSTMP_A_REG {}
#[doc = "MCPWM_GEN1_TSTMP_A_REG(i)"]
pub mod mcpwm_gen1_tstmp_a_reg;
#[doc = "MCPWM_GEN1_TSTMP_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_tstmp_b_reg](mcpwm_gen1_tstmp_b_reg) module"]
pub type MCPWM_GEN1_TSTMP_B_REG = crate::Reg<u32, _MCPWM_GEN1_TSTMP_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_TSTMP_B_REG;
#[doc = "`read()` method returns [mcpwm_gen1_tstmp_b_reg::R](mcpwm_gen1_tstmp_b_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_TSTMP_B_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_tstmp_b_reg::W](mcpwm_gen1_tstmp_b_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_TSTMP_B_REG {}
#[doc = "MCPWM_GEN1_TSTMP_B_REG(i)"]
pub mod mcpwm_gen1_tstmp_b_reg;
#[doc = "MCPWM_GEN1_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_cfg0_reg](mcpwm_gen1_cfg0_reg) module"]
pub type MCPWM_GEN1_CFG0_REG = crate::Reg<u32, _MCPWM_GEN1_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_gen1_cfg0_reg::R](mcpwm_gen1_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_cfg0_reg::W](mcpwm_gen1_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_CFG0_REG {}
#[doc = "MCPWM_GEN1_CFG0_REG(i)"]
pub mod mcpwm_gen1_cfg0_reg;
#[doc = "MCPWM_GEN1_FORCE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_force_reg](mcpwm_gen1_force_reg) module"]
pub type MCPWM_GEN1_FORCE_REG = crate::Reg<u32, _MCPWM_GEN1_FORCE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_FORCE_REG;
#[doc = "`read()` method returns [mcpwm_gen1_force_reg::R](mcpwm_gen1_force_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_FORCE_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_force_reg::W](mcpwm_gen1_force_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_FORCE_REG {}
#[doc = "MCPWM_GEN1_FORCE_REG(i)"]
pub mod mcpwm_gen1_force_reg;
#[doc = "MCPWM_GEN1_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_a_reg](mcpwm_gen1_a_reg) module"]
pub type MCPWM_GEN1_A_REG = crate::Reg<u32, _MCPWM_GEN1_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_A_REG;
#[doc = "`read()` method returns [mcpwm_gen1_a_reg::R](mcpwm_gen1_a_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_A_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_a_reg::W](mcpwm_gen1_a_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_A_REG {}
#[doc = "MCPWM_GEN1_A_REG(i)"]
pub mod mcpwm_gen1_a_reg;
#[doc = "MCPWM_GEN1_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen1_b_reg](mcpwm_gen1_b_reg) module"]
pub type MCPWM_GEN1_B_REG = crate::Reg<u32, _MCPWM_GEN1_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN1_B_REG;
#[doc = "`read()` method returns [mcpwm_gen1_b_reg::R](mcpwm_gen1_b_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN1_B_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen1_b_reg::W](mcpwm_gen1_b_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN1_B_REG {}
#[doc = "MCPWM_GEN1_B_REG(i)"]
pub mod mcpwm_gen1_b_reg;
#[doc = "MCPWM_DT1_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt1_cfg_reg](mcpwm_dt1_cfg_reg) module"]
pub type MCPWM_DT1_CFG_REG = crate::Reg<u32, _MCPWM_DT1_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT1_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt1_cfg_reg::R](mcpwm_dt1_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT1_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt1_cfg_reg::W](mcpwm_dt1_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT1_CFG_REG {}
#[doc = "MCPWM_DT1_CFG_REG(i)"]
pub mod mcpwm_dt1_cfg_reg;
#[doc = "MCPWM_DT1_FED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt1_fed_cfg_reg](mcpwm_dt1_fed_cfg_reg) module"]
pub type MCPWM_DT1_FED_CFG_REG = crate::Reg<u32, _MCPWM_DT1_FED_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT1_FED_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt1_fed_cfg_reg::R](mcpwm_dt1_fed_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT1_FED_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt1_fed_cfg_reg::W](mcpwm_dt1_fed_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT1_FED_CFG_REG {}
#[doc = "MCPWM_DT1_FED_CFG_REG(i)"]
pub mod mcpwm_dt1_fed_cfg_reg;
#[doc = "MCPWM_DT1_RED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt1_red_cfg_reg](mcpwm_dt1_red_cfg_reg) module"]
pub type MCPWM_DT1_RED_CFG_REG = crate::Reg<u32, _MCPWM_DT1_RED_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT1_RED_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt1_red_cfg_reg::R](mcpwm_dt1_red_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT1_RED_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt1_red_cfg_reg::W](mcpwm_dt1_red_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT1_RED_CFG_REG {}
#[doc = "MCPWM_DT1_RED_CFG_REG(i)"]
pub mod mcpwm_dt1_red_cfg_reg;
#[doc = "MCPWM_CARRIER1_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_carrier1_cfg_reg](mcpwm_carrier1_cfg_reg) module"]
pub type MCPWM_CARRIER1_CFG_REG = crate::Reg<u32, _MCPWM_CARRIER1_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CARRIER1_CFG_REG;
#[doc = "`read()` method returns [mcpwm_carrier1_cfg_reg::R](mcpwm_carrier1_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CARRIER1_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_carrier1_cfg_reg::W](mcpwm_carrier1_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CARRIER1_CFG_REG {}
#[doc = "MCPWM_CARRIER1_CFG_REG(i)"]
pub mod mcpwm_carrier1_cfg_reg;
#[doc = "MCPWM_FH1_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh1_cfg0_reg](mcpwm_fh1_cfg0_reg) module"]
pub type MCPWM_FH1_CFG0_REG = crate::Reg<u32, _MCPWM_FH1_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH1_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_fh1_cfg0_reg::R](mcpwm_fh1_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH1_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh1_cfg0_reg::W](mcpwm_fh1_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH1_CFG0_REG {}
#[doc = "MCPWM_FH1_CFG0_REG(i)"]
pub mod mcpwm_fh1_cfg0_reg;
#[doc = "MCPWM_FH1_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh1_cfg1_reg](mcpwm_fh1_cfg1_reg) module"]
pub type MCPWM_FH1_CFG1_REG = crate::Reg<u32, _MCPWM_FH1_CFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH1_CFG1_REG;
#[doc = "`read()` method returns [mcpwm_fh1_cfg1_reg::R](mcpwm_fh1_cfg1_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH1_CFG1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh1_cfg1_reg::W](mcpwm_fh1_cfg1_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH1_CFG1_REG {}
#[doc = "MCPWM_FH1_CFG1_REG(i)"]
pub mod mcpwm_fh1_cfg1_reg;
#[doc = "MCPWM_FH1_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh1_status_reg](mcpwm_fh1_status_reg) module"]
pub type MCPWM_FH1_STATUS_REG = crate::Reg<u32, _MCPWM_FH1_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH1_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_fh1_status_reg::R](mcpwm_fh1_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH1_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh1_status_reg::W](mcpwm_fh1_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH1_STATUS_REG {}
#[doc = "MCPWM_FH1_STATUS_REG(i)"]
pub mod mcpwm_fh1_status_reg;
#[doc = "MCPWM_GEN2_STMP_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_stmp_cfg_reg](mcpwm_gen2_stmp_cfg_reg) module"]
pub type MCPWM_GEN2_STMP_CFG_REG = crate::Reg<u32, _MCPWM_GEN2_STMP_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_STMP_CFG_REG;
#[doc = "`read()` method returns [mcpwm_gen2_stmp_cfg_reg::R](mcpwm_gen2_stmp_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_STMP_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_stmp_cfg_reg::W](mcpwm_gen2_stmp_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_STMP_CFG_REG {}
#[doc = "MCPWM_GEN2_STMP_CFG_REG(i)"]
pub mod mcpwm_gen2_stmp_cfg_reg;
#[doc = "MCPWM_GEN2_TSTMP_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_tstmp_a_reg](mcpwm_gen2_tstmp_a_reg) module"]
pub type MCPWM_GEN2_TSTMP_A_REG = crate::Reg<u32, _MCPWM_GEN2_TSTMP_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_TSTMP_A_REG;
#[doc = "`read()` method returns [mcpwm_gen2_tstmp_a_reg::R](mcpwm_gen2_tstmp_a_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_TSTMP_A_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_tstmp_a_reg::W](mcpwm_gen2_tstmp_a_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_TSTMP_A_REG {}
#[doc = "MCPWM_GEN2_TSTMP_A_REG(i)"]
pub mod mcpwm_gen2_tstmp_a_reg;
#[doc = "MCPWM_GEN2_TSTMP_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_tstmp_b_reg](mcpwm_gen2_tstmp_b_reg) module"]
pub type MCPWM_GEN2_TSTMP_B_REG = crate::Reg<u32, _MCPWM_GEN2_TSTMP_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_TSTMP_B_REG;
#[doc = "`read()` method returns [mcpwm_gen2_tstmp_b_reg::R](mcpwm_gen2_tstmp_b_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_TSTMP_B_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_tstmp_b_reg::W](mcpwm_gen2_tstmp_b_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_TSTMP_B_REG {}
#[doc = "MCPWM_GEN2_TSTMP_B_REG(i)"]
pub mod mcpwm_gen2_tstmp_b_reg;
#[doc = "MCPWM_GEN2_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_cfg0_reg](mcpwm_gen2_cfg0_reg) module"]
pub type MCPWM_GEN2_CFG0_REG = crate::Reg<u32, _MCPWM_GEN2_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_gen2_cfg0_reg::R](mcpwm_gen2_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_cfg0_reg::W](mcpwm_gen2_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_CFG0_REG {}
#[doc = "MCPWM_GEN2_CFG0_REG(i)"]
pub mod mcpwm_gen2_cfg0_reg;
#[doc = "MCPWM_GEN2_FORCE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_force_reg](mcpwm_gen2_force_reg) module"]
pub type MCPWM_GEN2_FORCE_REG = crate::Reg<u32, _MCPWM_GEN2_FORCE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_FORCE_REG;
#[doc = "`read()` method returns [mcpwm_gen2_force_reg::R](mcpwm_gen2_force_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_FORCE_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_force_reg::W](mcpwm_gen2_force_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_FORCE_REG {}
#[doc = "MCPWM_GEN2_FORCE_REG(i)"]
pub mod mcpwm_gen2_force_reg;
#[doc = "MCPWM_GEN2_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_a_reg](mcpwm_gen2_a_reg) module"]
pub type MCPWM_GEN2_A_REG = crate::Reg<u32, _MCPWM_GEN2_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_A_REG;
#[doc = "`read()` method returns [mcpwm_gen2_a_reg::R](mcpwm_gen2_a_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_A_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_a_reg::W](mcpwm_gen2_a_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_A_REG {}
#[doc = "MCPWM_GEN2_A_REG(i)"]
pub mod mcpwm_gen2_a_reg;
#[doc = "MCPWM_GEN2_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_gen2_b_reg](mcpwm_gen2_b_reg) module"]
pub type MCPWM_GEN2_B_REG = crate::Reg<u32, _MCPWM_GEN2_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_GEN2_B_REG;
#[doc = "`read()` method returns [mcpwm_gen2_b_reg::R](mcpwm_gen2_b_reg::R) reader structure"]
impl crate::Readable for MCPWM_GEN2_B_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_gen2_b_reg::W](mcpwm_gen2_b_reg::W) writer structure"]
impl crate::Writable for MCPWM_GEN2_B_REG {}
#[doc = "MCPWM_GEN2_B_REG(i)"]
pub mod mcpwm_gen2_b_reg;
#[doc = "MCPWM_DT2_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt2_cfg_reg](mcpwm_dt2_cfg_reg) module"]
pub type MCPWM_DT2_CFG_REG = crate::Reg<u32, _MCPWM_DT2_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT2_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt2_cfg_reg::R](mcpwm_dt2_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT2_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt2_cfg_reg::W](mcpwm_dt2_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT2_CFG_REG {}
#[doc = "MCPWM_DT2_CFG_REG(i)"]
pub mod mcpwm_dt2_cfg_reg;
#[doc = "MCPWM_DT2_FED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt2_fed_cfg_reg](mcpwm_dt2_fed_cfg_reg) module"]
pub type MCPWM_DT2_FED_CFG_REG = crate::Reg<u32, _MCPWM_DT2_FED_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT2_FED_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt2_fed_cfg_reg::R](mcpwm_dt2_fed_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT2_FED_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt2_fed_cfg_reg::W](mcpwm_dt2_fed_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT2_FED_CFG_REG {}
#[doc = "MCPWM_DT2_FED_CFG_REG(i)"]
pub mod mcpwm_dt2_fed_cfg_reg;
#[doc = "MCPWM_DT2_RED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_dt2_red_cfg_reg](mcpwm_dt2_red_cfg_reg) module"]
pub type MCPWM_DT2_RED_CFG_REG = crate::Reg<u32, _MCPWM_DT2_RED_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_DT2_RED_CFG_REG;
#[doc = "`read()` method returns [mcpwm_dt2_red_cfg_reg::R](mcpwm_dt2_red_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_DT2_RED_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_dt2_red_cfg_reg::W](mcpwm_dt2_red_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_DT2_RED_CFG_REG {}
#[doc = "MCPWM_DT2_RED_CFG_REG(i)"]
pub mod mcpwm_dt2_red_cfg_reg;
#[doc = "MCPWM_CARRIER2_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_carrier2_cfg_reg](mcpwm_carrier2_cfg_reg) module"]
pub type MCPWM_CARRIER2_CFG_REG = crate::Reg<u32, _MCPWM_CARRIER2_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CARRIER2_CFG_REG;
#[doc = "`read()` method returns [mcpwm_carrier2_cfg_reg::R](mcpwm_carrier2_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CARRIER2_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_carrier2_cfg_reg::W](mcpwm_carrier2_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CARRIER2_CFG_REG {}
#[doc = "MCPWM_CARRIER2_CFG_REG(i)"]
pub mod mcpwm_carrier2_cfg_reg;
#[doc = "MCPWM_FH2_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh2_cfg0_reg](mcpwm_fh2_cfg0_reg) module"]
pub type MCPWM_FH2_CFG0_REG = crate::Reg<u32, _MCPWM_FH2_CFG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH2_CFG0_REG;
#[doc = "`read()` method returns [mcpwm_fh2_cfg0_reg::R](mcpwm_fh2_cfg0_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH2_CFG0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh2_cfg0_reg::W](mcpwm_fh2_cfg0_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH2_CFG0_REG {}
#[doc = "MCPWM_FH2_CFG0_REG(i)"]
pub mod mcpwm_fh2_cfg0_reg;
#[doc = "MCPWM_FH2_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh2_cfg1_reg](mcpwm_fh2_cfg1_reg) module"]
pub type MCPWM_FH2_CFG1_REG = crate::Reg<u32, _MCPWM_FH2_CFG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH2_CFG1_REG;
#[doc = "`read()` method returns [mcpwm_fh2_cfg1_reg::R](mcpwm_fh2_cfg1_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH2_CFG1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh2_cfg1_reg::W](mcpwm_fh2_cfg1_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH2_CFG1_REG {}
#[doc = "MCPWM_FH2_CFG1_REG(i)"]
pub mod mcpwm_fh2_cfg1_reg;
#[doc = "MCPWM_FH2_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fh2_status_reg](mcpwm_fh2_status_reg) module"]
pub type MCPWM_FH2_STATUS_REG = crate::Reg<u32, _MCPWM_FH2_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FH2_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_fh2_status_reg::R](mcpwm_fh2_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_FH2_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fh2_status_reg::W](mcpwm_fh2_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_FH2_STATUS_REG {}
#[doc = "MCPWM_FH2_STATUS_REG(i)"]
pub mod mcpwm_fh2_status_reg;
#[doc = "MCPWM_FAULT_DETECT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_fault_detect_reg](mcpwm_fault_detect_reg) module"]
pub type MCPWM_FAULT_DETECT_REG = crate::Reg<u32, _MCPWM_FAULT_DETECT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_FAULT_DETECT_REG;
#[doc = "`read()` method returns [mcpwm_fault_detect_reg::R](mcpwm_fault_detect_reg::R) reader structure"]
impl crate::Readable for MCPWM_FAULT_DETECT_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_fault_detect_reg::W](mcpwm_fault_detect_reg::W) writer structure"]
impl crate::Writable for MCPWM_FAULT_DETECT_REG {}
#[doc = "MCPWM_FAULT_DETECT_REG(i)"]
pub mod mcpwm_fault_detect_reg;
#[doc = "MCPWM_CAP_TIMER_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_timer_cfg_reg](mcpwm_cap_timer_cfg_reg) module"]
pub type MCPWM_CAP_TIMER_CFG_REG = crate::Reg<u32, _MCPWM_CAP_TIMER_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_TIMER_CFG_REG;
#[doc = "`read()` method returns [mcpwm_cap_timer_cfg_reg::R](mcpwm_cap_timer_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_TIMER_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_timer_cfg_reg::W](mcpwm_cap_timer_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_TIMER_CFG_REG {}
#[doc = "MCPWM_CAP_TIMER_CFG_REG(i)"]
pub mod mcpwm_cap_timer_cfg_reg;
#[doc = "MCPWM_CAP_TIMER_PHASE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_timer_phase_reg](mcpwm_cap_timer_phase_reg) module"]
pub type MCPWM_CAP_TIMER_PHASE_REG = crate::Reg<u32, _MCPWM_CAP_TIMER_PHASE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_TIMER_PHASE_REG;
#[doc = "`read()` method returns [mcpwm_cap_timer_phase_reg::R](mcpwm_cap_timer_phase_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_TIMER_PHASE_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_timer_phase_reg::W](mcpwm_cap_timer_phase_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_TIMER_PHASE_REG {}
#[doc = "MCPWM_CAP_TIMER_PHASE_REG(i)"]
pub mod mcpwm_cap_timer_phase_reg;
#[doc = "MCPWM_CAP_CH0_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_ch0_cfg_reg](mcpwm_cap_ch0_cfg_reg) module"]
pub type MCPWM_CAP_CH0_CFG_REG = crate::Reg<u32, _MCPWM_CAP_CH0_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_CH0_CFG_REG;
#[doc = "`read()` method returns [mcpwm_cap_ch0_cfg_reg::R](mcpwm_cap_ch0_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_CH0_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_ch0_cfg_reg::W](mcpwm_cap_ch0_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_CH0_CFG_REG {}
#[doc = "MCPWM_CAP_CH0_CFG_REG(i)"]
pub mod mcpwm_cap_ch0_cfg_reg;
#[doc = "MCPWM_CAP_CH1_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_ch1_cfg_reg](mcpwm_cap_ch1_cfg_reg) module"]
pub type MCPWM_CAP_CH1_CFG_REG = crate::Reg<u32, _MCPWM_CAP_CH1_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_CH1_CFG_REG;
#[doc = "`read()` method returns [mcpwm_cap_ch1_cfg_reg::R](mcpwm_cap_ch1_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_CH1_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_ch1_cfg_reg::W](mcpwm_cap_ch1_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_CH1_CFG_REG {}
#[doc = "MCPWM_CAP_CH1_CFG_REG(i)"]
pub mod mcpwm_cap_ch1_cfg_reg;
#[doc = "MCPWM_CAP_CH2_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_ch2_cfg_reg](mcpwm_cap_ch2_cfg_reg) module"]
pub type MCPWM_CAP_CH2_CFG_REG = crate::Reg<u32, _MCPWM_CAP_CH2_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_CH2_CFG_REG;
#[doc = "`read()` method returns [mcpwm_cap_ch2_cfg_reg::R](mcpwm_cap_ch2_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_CH2_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_ch2_cfg_reg::W](mcpwm_cap_ch2_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_CH2_CFG_REG {}
#[doc = "MCPWM_CAP_CH2_CFG_REG(i)"]
pub mod mcpwm_cap_ch2_cfg_reg;
#[doc = "MCPWM_CAP_CH0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_ch0_reg](mcpwm_cap_ch0_reg) module"]
pub type MCPWM_CAP_CH0_REG = crate::Reg<u32, _MCPWM_CAP_CH0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_CH0_REG;
#[doc = "`read()` method returns [mcpwm_cap_ch0_reg::R](mcpwm_cap_ch0_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_CH0_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_ch0_reg::W](mcpwm_cap_ch0_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_CH0_REG {}
#[doc = "MCPWM_CAP_CH0_REG(i)"]
pub mod mcpwm_cap_ch0_reg;
#[doc = "MCPWM_CAP_CH1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_ch1_reg](mcpwm_cap_ch1_reg) module"]
pub type MCPWM_CAP_CH1_REG = crate::Reg<u32, _MCPWM_CAP_CH1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_CH1_REG;
#[doc = "`read()` method returns [mcpwm_cap_ch1_reg::R](mcpwm_cap_ch1_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_CH1_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_ch1_reg::W](mcpwm_cap_ch1_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_CH1_REG {}
#[doc = "MCPWM_CAP_CH1_REG(i)"]
pub mod mcpwm_cap_ch1_reg;
#[doc = "MCPWM_CAP_CH2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_ch2_reg](mcpwm_cap_ch2_reg) module"]
pub type MCPWM_CAP_CH2_REG = crate::Reg<u32, _MCPWM_CAP_CH2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_CH2_REG;
#[doc = "`read()` method returns [mcpwm_cap_ch2_reg::R](mcpwm_cap_ch2_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_CH2_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_ch2_reg::W](mcpwm_cap_ch2_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_CH2_REG {}
#[doc = "MCPWM_CAP_CH2_REG(i)"]
pub mod mcpwm_cap_ch2_reg;
#[doc = "MCPWM_CAP_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_cap_status_reg](mcpwm_cap_status_reg) module"]
pub type MCPWM_CAP_STATUS_REG = crate::Reg<u32, _MCPWM_CAP_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CAP_STATUS_REG;
#[doc = "`read()` method returns [mcpwm_cap_status_reg::R](mcpwm_cap_status_reg::R) reader structure"]
impl crate::Readable for MCPWM_CAP_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_cap_status_reg::W](mcpwm_cap_status_reg::W) writer structure"]
impl crate::Writable for MCPWM_CAP_STATUS_REG {}
#[doc = "MCPWM_CAP_STATUS_REG(i)"]
pub mod mcpwm_cap_status_reg;
#[doc = "MCPWM_UPDATE_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_update_cfg_reg](mcpwm_update_cfg_reg) module"]
pub type MCPWM_UPDATE_CFG_REG = crate::Reg<u32, _MCPWM_UPDATE_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_UPDATE_CFG_REG;
#[doc = "`read()` method returns [mcpwm_update_cfg_reg::R](mcpwm_update_cfg_reg::R) reader structure"]
impl crate::Readable for MCPWM_UPDATE_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_update_cfg_reg::W](mcpwm_update_cfg_reg::W) writer structure"]
impl crate::Writable for MCPWM_UPDATE_CFG_REG {}
#[doc = "MCPWM_UPDATE_CFG_REG(i)"]
pub mod mcpwm_update_cfg_reg;
#[doc = "MCMCPWM_INT_ENA_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_ena_mcpwm_reg](mcmcpwm_int_ena_mcpwm_reg) module"]
pub type MCMCPWM_INT_ENA_MCPWM_REG = crate::Reg<u32, _MCMCPWM_INT_ENA_MCPWM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_ENA_MCPWM_REG;
#[doc = "`read()` method returns [mcmcpwm_int_ena_mcpwm_reg::R](mcmcpwm_int_ena_mcpwm_reg::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_ENA_MCPWM_REG {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_ena_mcpwm_reg::W](mcmcpwm_int_ena_mcpwm_reg::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_ENA_MCPWM_REG {}
#[doc = "MCMCPWM_INT_ENA_MCPWM_REG(i)"]
pub mod mcmcpwm_int_ena_mcpwm_reg;
#[doc = "MCMCPWM_INT_RAW_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_raw_mcpwm_reg](mcmcpwm_int_raw_mcpwm_reg) module"]
pub type MCMCPWM_INT_RAW_MCPWM_REG = crate::Reg<u32, _MCMCPWM_INT_RAW_MCPWM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_RAW_MCPWM_REG;
#[doc = "`read()` method returns [mcmcpwm_int_raw_mcpwm_reg::R](mcmcpwm_int_raw_mcpwm_reg::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_RAW_MCPWM_REG {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_raw_mcpwm_reg::W](mcmcpwm_int_raw_mcpwm_reg::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_RAW_MCPWM_REG {}
#[doc = "MCMCPWM_INT_RAW_MCPWM_REG(i)"]
pub mod mcmcpwm_int_raw_mcpwm_reg;
#[doc = "MCMCPWM_INT_ST_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_st_mcpwm_reg](mcmcpwm_int_st_mcpwm_reg) module"]
pub type MCMCPWM_INT_ST_MCPWM_REG = crate::Reg<u32, _MCMCPWM_INT_ST_MCPWM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_ST_MCPWM_REG;
#[doc = "`read()` method returns [mcmcpwm_int_st_mcpwm_reg::R](mcmcpwm_int_st_mcpwm_reg::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_ST_MCPWM_REG {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_st_mcpwm_reg::W](mcmcpwm_int_st_mcpwm_reg::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_ST_MCPWM_REG {}
#[doc = "MCMCPWM_INT_ST_MCPWM_REG(i)"]
pub mod mcmcpwm_int_st_mcpwm_reg;
#[doc = "MCMCPWM_INT_CLR_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_clr_mcpwm_reg](mcmcpwm_int_clr_mcpwm_reg) module"]
pub type MCMCPWM_INT_CLR_MCPWM_REG = crate::Reg<u32, _MCMCPWM_INT_CLR_MCPWM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_CLR_MCPWM_REG;
#[doc = "`read()` method returns [mcmcpwm_int_clr_mcpwm_reg::R](mcmcpwm_int_clr_mcpwm_reg::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_CLR_MCPWM_REG {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_clr_mcpwm_reg::W](mcmcpwm_int_clr_mcpwm_reg::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_CLR_MCPWM_REG {}
#[doc = "MCMCPWM_INT_CLR_MCPWM_REG(i)"]
pub mod mcmcpwm_int_clr_mcpwm_reg;
#[doc = "MCPWM_CLK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_clk_reg](mcpwm_clk_reg) module"]
pub type MCPWM_CLK_REG = crate::Reg<u32, _MCPWM_CLK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_CLK_REG;
#[doc = "`read()` method returns [mcpwm_clk_reg::R](mcpwm_clk_reg::R) reader structure"]
impl crate::Readable for MCPWM_CLK_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_clk_reg::W](mcpwm_clk_reg::W) writer structure"]
impl crate::Writable for MCPWM_CLK_REG {}
#[doc = "MCPWM_CLK_REG(i)"]
pub mod mcpwm_clk_reg;
#[doc = "MCPWM_VERSION_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcpwm_version_reg](mcpwm_version_reg) module"]
pub type MCPWM_VERSION_REG = crate::Reg<u32, _MCPWM_VERSION_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCPWM_VERSION_REG;
#[doc = "`read()` method returns [mcpwm_version_reg::R](mcpwm_version_reg::R) reader structure"]
impl crate::Readable for MCPWM_VERSION_REG {}
#[doc = "`write(|w| ..)` method takes [mcpwm_version_reg::W](mcpwm_version_reg::W) writer structure"]
impl crate::Writable for MCPWM_VERSION_REG {}
#[doc = "MCPWM_VERSION_REG(i)"]
pub mod mcpwm_version_reg;
