#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCPWM_CLK_CFG_REG(i)"]
    pub clk_cfg: CLK_CFG,
    #[doc = "0x04 - MCPWM_TIMER0_CFG0_REG(i)"]
    pub timer0_cfg0: TIMER0_CFG0,
    #[doc = "0x08 - MCPWM_TIMER0_CFG1_REG(i)"]
    pub timer0_cfg1: TIMER0_CFG1,
    #[doc = "0x0c - MCPWM_TIMER0_SYNC_REG(i)"]
    pub timer0_sync: TIMER0_SYNC,
    #[doc = "0x10 - MCPWM_TIMER0_STATUS_REG(i)"]
    pub timer0_status: TIMER0_STATUS,
    #[doc = "0x14 - MCPWM_TIMER1_CFG0_REG(i)"]
    pub timer1_cfg0: TIMER1_CFG0,
    #[doc = "0x18 - MCPWM_TIMER1_CFG1_REG(i)"]
    pub timer1_cfg1: TIMER1_CFG1,
    #[doc = "0x1c - MCPWM_TIMER1_SYNC_REG(i)"]
    pub timer1_sync: TIMER1_SYNC,
    #[doc = "0x20 - MCPWM_TIMER1_STATUS_REG(i)"]
    pub timer1_status: TIMER1_STATUS,
    #[doc = "0x24 - MCPWM_TIMER2_CFG0_REG(i)"]
    pub timer2_cfg0: TIMER2_CFG0,
    #[doc = "0x28 - MCPWM_TIMER2_CFG1_REG(i)"]
    pub timer2_cfg1: TIMER2_CFG1,
    #[doc = "0x2c - MCPWM_TIMER2_SYNC_REG(i)"]
    pub timer2_sync: TIMER2_SYNC,
    #[doc = "0x30 - MCPWM_TIMER2_STATUS_REG(i)"]
    pub timer2_status: TIMER2_STATUS,
    #[doc = "0x34 - MCPWM_TIMER_SYNCI_CFG_REG(i)"]
    pub timer_synci_cfg: TIMER_SYNCI_CFG,
    #[doc = "0x38 - MCPWM_OPERATOR_TIMERSEL_REG(i)"]
    pub operator_timersel: OPERATOR_TIMERSEL,
    #[doc = "0x3c - MCPWM_GEN0_STMP_CFG_REG(i)"]
    pub gen0_stmp_cfg: GEN0_STMP_CFG,
    #[doc = "0x40 - MCPWM_GEN0_TSTMP_A_REG(i)"]
    pub gen0_tstmp_a: GEN0_TSTMP_A,
    #[doc = "0x44 - MCPWM_GEN0_TSTMP_B_REG(i)"]
    pub gen0_tstmp_b: GEN0_TSTMP_B,
    #[doc = "0x48 - MCPWM_GEN0_CFG0_REG(i)"]
    pub gen0_cfg0: GEN0_CFG0,
    #[doc = "0x4c - MCPWM_GEN0_FORCE_REG(i)"]
    pub gen0_force: GEN0_FORCE,
    #[doc = "0x50 - MCPWM_GEN0_A_REG(i)"]
    pub gen0_a: GEN0_A,
    #[doc = "0x54 - MCPWM_GEN0_B_REG(i)"]
    pub gen0_b: GEN0_B,
    #[doc = "0x58 - MCPWM_DT0_CFG_REG(i)"]
    pub dt0_cfg: DT0_CFG,
    #[doc = "0x5c - MCPWM_DT0_FED_CFG_REG(i)"]
    pub dt0_fed_cfg: DT0_FED_CFG,
    #[doc = "0x60 - MCPWM_DT0_RED_CFG_REG(i)"]
    pub dt0_red_cfg: DT0_RED_CFG,
    #[doc = "0x64 - MCPWM_CARRIER0_CFG_REG(i)"]
    pub carrier0_cfg: CARRIER0_CFG,
    #[doc = "0x68 - MCPWM_FH0_CFG0_REG(i)"]
    pub fh0_cfg0: FH0_CFG0,
    #[doc = "0x6c - MCPWM_FH0_CFG1_REG(i)"]
    pub fh0_cfg1: FH0_CFG1,
    #[doc = "0x70 - MCPWM_FH0_STATUS_REG(i)"]
    pub fh0_status: FH0_STATUS,
    #[doc = "0x74 - MCPWM_GEN1_STMP_CFG_REG(i)"]
    pub gen1_stmp_cfg: GEN1_STMP_CFG,
    #[doc = "0x78 - MCPWM_GEN1_TSTMP_A_REG(i)"]
    pub gen1_tstmp_a: GEN1_TSTMP_A,
    #[doc = "0x7c - MCPWM_GEN1_TSTMP_B_REG(i)"]
    pub gen1_tstmp_b: GEN1_TSTMP_B,
    #[doc = "0x80 - MCPWM_GEN1_CFG0_REG(i)"]
    pub gen1_cfg0: GEN1_CFG0,
    #[doc = "0x84 - MCPWM_GEN1_FORCE_REG(i)"]
    pub gen1_force: GEN1_FORCE,
    #[doc = "0x88 - MCPWM_GEN1_A_REG(i)"]
    pub gen1_a: GEN1_A,
    #[doc = "0x8c - MCPWM_GEN1_B_REG(i)"]
    pub gen1_b: GEN1_B,
    #[doc = "0x90 - MCPWM_DT1_CFG_REG(i)"]
    pub dt1_cfg: DT1_CFG,
    #[doc = "0x94 - MCPWM_DT1_FED_CFG_REG(i)"]
    pub dt1_fed_cfg: DT1_FED_CFG,
    #[doc = "0x98 - MCPWM_DT1_RED_CFG_REG(i)"]
    pub dt1_red_cfg: DT1_RED_CFG,
    #[doc = "0x9c - MCPWM_CARRIER1_CFG_REG(i)"]
    pub carrier1_cfg: CARRIER1_CFG,
    #[doc = "0xa0 - MCPWM_FH1_CFG0_REG(i)"]
    pub fh1_cfg0: FH1_CFG0,
    #[doc = "0xa4 - MCPWM_FH1_CFG1_REG(i)"]
    pub fh1_cfg1: FH1_CFG1,
    #[doc = "0xa8 - MCPWM_FH1_STATUS_REG(i)"]
    pub fh1_status: FH1_STATUS,
    #[doc = "0xac - MCPWM_GEN2_STMP_CFG_REG(i)"]
    pub gen2_stmp_cfg: GEN2_STMP_CFG,
    #[doc = "0xb0 - MCPWM_GEN2_TSTMP_A_REG(i)"]
    pub gen2_tstmp_a: GEN2_TSTMP_A,
    #[doc = "0xb4 - MCPWM_GEN2_TSTMP_B_REG(i)"]
    pub gen2_tstmp_b: GEN2_TSTMP_B,
    #[doc = "0xb8 - MCPWM_GEN2_CFG0_REG(i)"]
    pub gen2_cfg0: GEN2_CFG0,
    #[doc = "0xbc - MCPWM_GEN2_FORCE_REG(i)"]
    pub gen2_force: GEN2_FORCE,
    #[doc = "0xc0 - MCPWM_GEN2_A_REG(i)"]
    pub gen2_a: GEN2_A,
    #[doc = "0xc4 - MCPWM_GEN2_B_REG(i)"]
    pub gen2_b: GEN2_B,
    #[doc = "0xc8 - MCPWM_DT2_CFG_REG(i)"]
    pub dt2_cfg: DT2_CFG,
    #[doc = "0xcc - MCPWM_DT2_FED_CFG_REG(i)"]
    pub dt2_fed_cfg: DT2_FED_CFG,
    #[doc = "0xd0 - MCPWM_DT2_RED_CFG_REG(i)"]
    pub dt2_red_cfg: DT2_RED_CFG,
    #[doc = "0xd4 - MCPWM_CARRIER2_CFG_REG(i)"]
    pub carrier2_cfg: CARRIER2_CFG,
    #[doc = "0xd8 - MCPWM_FH2_CFG0_REG(i)"]
    pub fh2_cfg0: FH2_CFG0,
    #[doc = "0xdc - MCPWM_FH2_CFG1_REG(i)"]
    pub fh2_cfg1: FH2_CFG1,
    #[doc = "0xe0 - MCPWM_FH2_STATUS_REG(i)"]
    pub fh2_status: FH2_STATUS,
    #[doc = "0xe4 - MCPWM_FAULT_DETECT_REG(i)"]
    pub fault_detect: FAULT_DETECT,
    #[doc = "0xe8 - MCPWM_CAP_TIMER_CFG_REG(i)"]
    pub cap_timer_cfg: CAP_TIMER_CFG,
    #[doc = "0xec - MCPWM_CAP_TIMER_PHASE_REG(i)"]
    pub cap_timer_phase: CAP_TIMER_PHASE,
    #[doc = "0xf0 - MCPWM_CAP_CH0_CFG_REG(i)"]
    pub cap_ch0_cfg: CAP_CH0_CFG,
    #[doc = "0xf4 - MCPWM_CAP_CH1_CFG_REG(i)"]
    pub cap_ch1_cfg: CAP_CH1_CFG,
    #[doc = "0xf8 - MCPWM_CAP_CH2_CFG_REG(i)"]
    pub cap_ch2_cfg: CAP_CH2_CFG,
    #[doc = "0xfc - MCPWM_CAP_CH0_REG(i)"]
    pub cap_ch0: CAP_CH0,
    #[doc = "0x100 - MCPWM_CAP_CH1_REG(i)"]
    pub cap_ch1: CAP_CH1,
    #[doc = "0x104 - MCPWM_CAP_CH2_REG(i)"]
    pub cap_ch2: CAP_CH2,
    #[doc = "0x108 - MCPWM_CAP_STATUS_REG(i)"]
    pub cap_status: CAP_STATUS,
    #[doc = "0x10c - MCPWM_UPDATE_CFG_REG(i)"]
    pub update_cfg: UPDATE_CFG,
    #[doc = "0x110 - MCMCPWM_INT_ENA_MCPWM_REG(i)"]
    pub mcmcpwm_int_ena_mcpwm: MCMCPWM_INT_ENA_MCPWM,
    #[doc = "0x114 - MCMCPWM_INT_RAW_MCPWM_REG(i)"]
    pub mcmcpwm_int_raw_mcpwm: MCMCPWM_INT_RAW_MCPWM,
    #[doc = "0x118 - MCMCPWM_INT_ST_MCPWM_REG(i)"]
    pub mcmcpwm_int_st_mcpwm: MCMCPWM_INT_ST_MCPWM,
    #[doc = "0x11c - MCMCPWM_INT_CLR_MCPWM_REG(i)"]
    pub mcmcpwm_int_clr_mcpwm: MCMCPWM_INT_CLR_MCPWM,
    #[doc = "0x120 - MCPWM_CLK_REG(i)"]
    pub clk: CLK,
    #[doc = "0x124 - MCPWM_VERSION_REG(i)"]
    pub version: VERSION,
}
#[doc = "MCPWM_CLK_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_cfg](clk_cfg) module"]
pub type CLK_CFG = crate::Reg<u32, _CLK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG;
#[doc = "`read()` method returns [clk_cfg::R](clk_cfg::R) reader structure"]
impl crate::Readable for CLK_CFG {}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](clk_cfg::W) writer structure"]
impl crate::Writable for CLK_CFG {}
#[doc = "MCPWM_CLK_CFG_REG(i)"]
pub mod clk_cfg;
#[doc = "MCPWM_TIMER0_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer0_cfg0](timer0_cfg0) module"]
pub type TIMER0_CFG0 = crate::Reg<u32, _TIMER0_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0_CFG0;
#[doc = "`read()` method returns [timer0_cfg0::R](timer0_cfg0::R) reader structure"]
impl crate::Readable for TIMER0_CFG0 {}
#[doc = "`write(|w| ..)` method takes [timer0_cfg0::W](timer0_cfg0::W) writer structure"]
impl crate::Writable for TIMER0_CFG0 {}
#[doc = "MCPWM_TIMER0_CFG0_REG(i)"]
pub mod timer0_cfg0;
#[doc = "MCPWM_TIMER0_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer0_cfg1](timer0_cfg1) module"]
pub type TIMER0_CFG1 = crate::Reg<u32, _TIMER0_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0_CFG1;
#[doc = "`read()` method returns [timer0_cfg1::R](timer0_cfg1::R) reader structure"]
impl crate::Readable for TIMER0_CFG1 {}
#[doc = "`write(|w| ..)` method takes [timer0_cfg1::W](timer0_cfg1::W) writer structure"]
impl crate::Writable for TIMER0_CFG1 {}
#[doc = "MCPWM_TIMER0_CFG1_REG(i)"]
pub mod timer0_cfg1;
#[doc = "MCPWM_TIMER0_SYNC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer0_sync](timer0_sync) module"]
pub type TIMER0_SYNC = crate::Reg<u32, _TIMER0_SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0_SYNC;
#[doc = "`read()` method returns [timer0_sync::R](timer0_sync::R) reader structure"]
impl crate::Readable for TIMER0_SYNC {}
#[doc = "`write(|w| ..)` method takes [timer0_sync::W](timer0_sync::W) writer structure"]
impl crate::Writable for TIMER0_SYNC {}
#[doc = "MCPWM_TIMER0_SYNC_REG(i)"]
pub mod timer0_sync;
#[doc = "MCPWM_TIMER0_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer0_status](timer0_status) module"]
pub type TIMER0_STATUS = crate::Reg<u32, _TIMER0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0_STATUS;
#[doc = "`read()` method returns [timer0_status::R](timer0_status::R) reader structure"]
impl crate::Readable for TIMER0_STATUS {}
#[doc = "`write(|w| ..)` method takes [timer0_status::W](timer0_status::W) writer structure"]
impl crate::Writable for TIMER0_STATUS {}
#[doc = "MCPWM_TIMER0_STATUS_REG(i)"]
pub mod timer0_status;
#[doc = "MCPWM_TIMER1_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1_cfg0](timer1_cfg0) module"]
pub type TIMER1_CFG0 = crate::Reg<u32, _TIMER1_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1_CFG0;
#[doc = "`read()` method returns [timer1_cfg0::R](timer1_cfg0::R) reader structure"]
impl crate::Readable for TIMER1_CFG0 {}
#[doc = "`write(|w| ..)` method takes [timer1_cfg0::W](timer1_cfg0::W) writer structure"]
impl crate::Writable for TIMER1_CFG0 {}
#[doc = "MCPWM_TIMER1_CFG0_REG(i)"]
pub mod timer1_cfg0;
#[doc = "MCPWM_TIMER1_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1_cfg1](timer1_cfg1) module"]
pub type TIMER1_CFG1 = crate::Reg<u32, _TIMER1_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1_CFG1;
#[doc = "`read()` method returns [timer1_cfg1::R](timer1_cfg1::R) reader structure"]
impl crate::Readable for TIMER1_CFG1 {}
#[doc = "`write(|w| ..)` method takes [timer1_cfg1::W](timer1_cfg1::W) writer structure"]
impl crate::Writable for TIMER1_CFG1 {}
#[doc = "MCPWM_TIMER1_CFG1_REG(i)"]
pub mod timer1_cfg1;
#[doc = "MCPWM_TIMER1_SYNC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1_sync](timer1_sync) module"]
pub type TIMER1_SYNC = crate::Reg<u32, _TIMER1_SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1_SYNC;
#[doc = "`read()` method returns [timer1_sync::R](timer1_sync::R) reader structure"]
impl crate::Readable for TIMER1_SYNC {}
#[doc = "`write(|w| ..)` method takes [timer1_sync::W](timer1_sync::W) writer structure"]
impl crate::Writable for TIMER1_SYNC {}
#[doc = "MCPWM_TIMER1_SYNC_REG(i)"]
pub mod timer1_sync;
#[doc = "MCPWM_TIMER1_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1_status](timer1_status) module"]
pub type TIMER1_STATUS = crate::Reg<u32, _TIMER1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1_STATUS;
#[doc = "`read()` method returns [timer1_status::R](timer1_status::R) reader structure"]
impl crate::Readable for TIMER1_STATUS {}
#[doc = "`write(|w| ..)` method takes [timer1_status::W](timer1_status::W) writer structure"]
impl crate::Writable for TIMER1_STATUS {}
#[doc = "MCPWM_TIMER1_STATUS_REG(i)"]
pub mod timer1_status;
#[doc = "MCPWM_TIMER2_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2_cfg0](timer2_cfg0) module"]
pub type TIMER2_CFG0 = crate::Reg<u32, _TIMER2_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2_CFG0;
#[doc = "`read()` method returns [timer2_cfg0::R](timer2_cfg0::R) reader structure"]
impl crate::Readable for TIMER2_CFG0 {}
#[doc = "`write(|w| ..)` method takes [timer2_cfg0::W](timer2_cfg0::W) writer structure"]
impl crate::Writable for TIMER2_CFG0 {}
#[doc = "MCPWM_TIMER2_CFG0_REG(i)"]
pub mod timer2_cfg0;
#[doc = "MCPWM_TIMER2_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2_cfg1](timer2_cfg1) module"]
pub type TIMER2_CFG1 = crate::Reg<u32, _TIMER2_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2_CFG1;
#[doc = "`read()` method returns [timer2_cfg1::R](timer2_cfg1::R) reader structure"]
impl crate::Readable for TIMER2_CFG1 {}
#[doc = "`write(|w| ..)` method takes [timer2_cfg1::W](timer2_cfg1::W) writer structure"]
impl crate::Writable for TIMER2_CFG1 {}
#[doc = "MCPWM_TIMER2_CFG1_REG(i)"]
pub mod timer2_cfg1;
#[doc = "MCPWM_TIMER2_SYNC_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2_sync](timer2_sync) module"]
pub type TIMER2_SYNC = crate::Reg<u32, _TIMER2_SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2_SYNC;
#[doc = "`read()` method returns [timer2_sync::R](timer2_sync::R) reader structure"]
impl crate::Readable for TIMER2_SYNC {}
#[doc = "`write(|w| ..)` method takes [timer2_sync::W](timer2_sync::W) writer structure"]
impl crate::Writable for TIMER2_SYNC {}
#[doc = "MCPWM_TIMER2_SYNC_REG(i)"]
pub mod timer2_sync;
#[doc = "MCPWM_TIMER2_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2_status](timer2_status) module"]
pub type TIMER2_STATUS = crate::Reg<u32, _TIMER2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2_STATUS;
#[doc = "`read()` method returns [timer2_status::R](timer2_status::R) reader structure"]
impl crate::Readable for TIMER2_STATUS {}
#[doc = "`write(|w| ..)` method takes [timer2_status::W](timer2_status::W) writer structure"]
impl crate::Writable for TIMER2_STATUS {}
#[doc = "MCPWM_TIMER2_STATUS_REG(i)"]
pub mod timer2_status;
#[doc = "MCPWM_TIMER_SYNCI_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer_synci_cfg](timer_synci_cfg) module"]
pub type TIMER_SYNCI_CFG = crate::Reg<u32, _TIMER_SYNCI_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_SYNCI_CFG;
#[doc = "`read()` method returns [timer_synci_cfg::R](timer_synci_cfg::R) reader structure"]
impl crate::Readable for TIMER_SYNCI_CFG {}
#[doc = "`write(|w| ..)` method takes [timer_synci_cfg::W](timer_synci_cfg::W) writer structure"]
impl crate::Writable for TIMER_SYNCI_CFG {}
#[doc = "MCPWM_TIMER_SYNCI_CFG_REG(i)"]
pub mod timer_synci_cfg;
#[doc = "MCPWM_OPERATOR_TIMERSEL_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [operator_timersel](operator_timersel) module"]
pub type OPERATOR_TIMERSEL = crate::Reg<u32, _OPERATOR_TIMERSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPERATOR_TIMERSEL;
#[doc = "`read()` method returns [operator_timersel::R](operator_timersel::R) reader structure"]
impl crate::Readable for OPERATOR_TIMERSEL {}
#[doc = "`write(|w| ..)` method takes [operator_timersel::W](operator_timersel::W) writer structure"]
impl crate::Writable for OPERATOR_TIMERSEL {}
#[doc = "MCPWM_OPERATOR_TIMERSEL_REG(i)"]
pub mod operator_timersel;
#[doc = "MCPWM_GEN0_STMP_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_stmp_cfg](gen0_stmp_cfg) module"]
pub type GEN0_STMP_CFG = crate::Reg<u32, _GEN0_STMP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_STMP_CFG;
#[doc = "`read()` method returns [gen0_stmp_cfg::R](gen0_stmp_cfg::R) reader structure"]
impl crate::Readable for GEN0_STMP_CFG {}
#[doc = "`write(|w| ..)` method takes [gen0_stmp_cfg::W](gen0_stmp_cfg::W) writer structure"]
impl crate::Writable for GEN0_STMP_CFG {}
#[doc = "MCPWM_GEN0_STMP_CFG_REG(i)"]
pub mod gen0_stmp_cfg;
#[doc = "MCPWM_GEN0_TSTMP_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_tstmp_a](gen0_tstmp_a) module"]
pub type GEN0_TSTMP_A = crate::Reg<u32, _GEN0_TSTMP_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_TSTMP_A;
#[doc = "`read()` method returns [gen0_tstmp_a::R](gen0_tstmp_a::R) reader structure"]
impl crate::Readable for GEN0_TSTMP_A {}
#[doc = "`write(|w| ..)` method takes [gen0_tstmp_a::W](gen0_tstmp_a::W) writer structure"]
impl crate::Writable for GEN0_TSTMP_A {}
#[doc = "MCPWM_GEN0_TSTMP_A_REG(i)"]
pub mod gen0_tstmp_a;
#[doc = "MCPWM_GEN0_TSTMP_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_tstmp_b](gen0_tstmp_b) module"]
pub type GEN0_TSTMP_B = crate::Reg<u32, _GEN0_TSTMP_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_TSTMP_B;
#[doc = "`read()` method returns [gen0_tstmp_b::R](gen0_tstmp_b::R) reader structure"]
impl crate::Readable for GEN0_TSTMP_B {}
#[doc = "`write(|w| ..)` method takes [gen0_tstmp_b::W](gen0_tstmp_b::W) writer structure"]
impl crate::Writable for GEN0_TSTMP_B {}
#[doc = "MCPWM_GEN0_TSTMP_B_REG(i)"]
pub mod gen0_tstmp_b;
#[doc = "MCPWM_GEN0_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_cfg0](gen0_cfg0) module"]
pub type GEN0_CFG0 = crate::Reg<u32, _GEN0_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_CFG0;
#[doc = "`read()` method returns [gen0_cfg0::R](gen0_cfg0::R) reader structure"]
impl crate::Readable for GEN0_CFG0 {}
#[doc = "`write(|w| ..)` method takes [gen0_cfg0::W](gen0_cfg0::W) writer structure"]
impl crate::Writable for GEN0_CFG0 {}
#[doc = "MCPWM_GEN0_CFG0_REG(i)"]
pub mod gen0_cfg0;
#[doc = "MCPWM_GEN0_FORCE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_force](gen0_force) module"]
pub type GEN0_FORCE = crate::Reg<u32, _GEN0_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_FORCE;
#[doc = "`read()` method returns [gen0_force::R](gen0_force::R) reader structure"]
impl crate::Readable for GEN0_FORCE {}
#[doc = "`write(|w| ..)` method takes [gen0_force::W](gen0_force::W) writer structure"]
impl crate::Writable for GEN0_FORCE {}
#[doc = "MCPWM_GEN0_FORCE_REG(i)"]
pub mod gen0_force;
#[doc = "MCPWM_GEN0_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_a](gen0_a) module"]
pub type GEN0_A = crate::Reg<u32, _GEN0_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_A;
#[doc = "`read()` method returns [gen0_a::R](gen0_a::R) reader structure"]
impl crate::Readable for GEN0_A {}
#[doc = "`write(|w| ..)` method takes [gen0_a::W](gen0_a::W) writer structure"]
impl crate::Writable for GEN0_A {}
#[doc = "MCPWM_GEN0_A_REG(i)"]
pub mod gen0_a;
#[doc = "MCPWM_GEN0_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen0_b](gen0_b) module"]
pub type GEN0_B = crate::Reg<u32, _GEN0_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN0_B;
#[doc = "`read()` method returns [gen0_b::R](gen0_b::R) reader structure"]
impl crate::Readable for GEN0_B {}
#[doc = "`write(|w| ..)` method takes [gen0_b::W](gen0_b::W) writer structure"]
impl crate::Writable for GEN0_B {}
#[doc = "MCPWM_GEN0_B_REG(i)"]
pub mod gen0_b;
#[doc = "MCPWM_DT0_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt0_cfg](dt0_cfg) module"]
pub type DT0_CFG = crate::Reg<u32, _DT0_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT0_CFG;
#[doc = "`read()` method returns [dt0_cfg::R](dt0_cfg::R) reader structure"]
impl crate::Readable for DT0_CFG {}
#[doc = "`write(|w| ..)` method takes [dt0_cfg::W](dt0_cfg::W) writer structure"]
impl crate::Writable for DT0_CFG {}
#[doc = "MCPWM_DT0_CFG_REG(i)"]
pub mod dt0_cfg;
#[doc = "MCPWM_DT0_FED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt0_fed_cfg](dt0_fed_cfg) module"]
pub type DT0_FED_CFG = crate::Reg<u32, _DT0_FED_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT0_FED_CFG;
#[doc = "`read()` method returns [dt0_fed_cfg::R](dt0_fed_cfg::R) reader structure"]
impl crate::Readable for DT0_FED_CFG {}
#[doc = "`write(|w| ..)` method takes [dt0_fed_cfg::W](dt0_fed_cfg::W) writer structure"]
impl crate::Writable for DT0_FED_CFG {}
#[doc = "MCPWM_DT0_FED_CFG_REG(i)"]
pub mod dt0_fed_cfg;
#[doc = "MCPWM_DT0_RED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt0_red_cfg](dt0_red_cfg) module"]
pub type DT0_RED_CFG = crate::Reg<u32, _DT0_RED_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT0_RED_CFG;
#[doc = "`read()` method returns [dt0_red_cfg::R](dt0_red_cfg::R) reader structure"]
impl crate::Readable for DT0_RED_CFG {}
#[doc = "`write(|w| ..)` method takes [dt0_red_cfg::W](dt0_red_cfg::W) writer structure"]
impl crate::Writable for DT0_RED_CFG {}
#[doc = "MCPWM_DT0_RED_CFG_REG(i)"]
pub mod dt0_red_cfg;
#[doc = "MCPWM_CARRIER0_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [carrier0_cfg](carrier0_cfg) module"]
pub type CARRIER0_CFG = crate::Reg<u32, _CARRIER0_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CARRIER0_CFG;
#[doc = "`read()` method returns [carrier0_cfg::R](carrier0_cfg::R) reader structure"]
impl crate::Readable for CARRIER0_CFG {}
#[doc = "`write(|w| ..)` method takes [carrier0_cfg::W](carrier0_cfg::W) writer structure"]
impl crate::Writable for CARRIER0_CFG {}
#[doc = "MCPWM_CARRIER0_CFG_REG(i)"]
pub mod carrier0_cfg;
#[doc = "MCPWM_FH0_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh0_cfg0](fh0_cfg0) module"]
pub type FH0_CFG0 = crate::Reg<u32, _FH0_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH0_CFG0;
#[doc = "`read()` method returns [fh0_cfg0::R](fh0_cfg0::R) reader structure"]
impl crate::Readable for FH0_CFG0 {}
#[doc = "`write(|w| ..)` method takes [fh0_cfg0::W](fh0_cfg0::W) writer structure"]
impl crate::Writable for FH0_CFG0 {}
#[doc = "MCPWM_FH0_CFG0_REG(i)"]
pub mod fh0_cfg0;
#[doc = "MCPWM_FH0_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh0_cfg1](fh0_cfg1) module"]
pub type FH0_CFG1 = crate::Reg<u32, _FH0_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH0_CFG1;
#[doc = "`read()` method returns [fh0_cfg1::R](fh0_cfg1::R) reader structure"]
impl crate::Readable for FH0_CFG1 {}
#[doc = "`write(|w| ..)` method takes [fh0_cfg1::W](fh0_cfg1::W) writer structure"]
impl crate::Writable for FH0_CFG1 {}
#[doc = "MCPWM_FH0_CFG1_REG(i)"]
pub mod fh0_cfg1;
#[doc = "MCPWM_FH0_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh0_status](fh0_status) module"]
pub type FH0_STATUS = crate::Reg<u32, _FH0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH0_STATUS;
#[doc = "`read()` method returns [fh0_status::R](fh0_status::R) reader structure"]
impl crate::Readable for FH0_STATUS {}
#[doc = "`write(|w| ..)` method takes [fh0_status::W](fh0_status::W) writer structure"]
impl crate::Writable for FH0_STATUS {}
#[doc = "MCPWM_FH0_STATUS_REG(i)"]
pub mod fh0_status;
#[doc = "MCPWM_GEN1_STMP_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_stmp_cfg](gen1_stmp_cfg) module"]
pub type GEN1_STMP_CFG = crate::Reg<u32, _GEN1_STMP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_STMP_CFG;
#[doc = "`read()` method returns [gen1_stmp_cfg::R](gen1_stmp_cfg::R) reader structure"]
impl crate::Readable for GEN1_STMP_CFG {}
#[doc = "`write(|w| ..)` method takes [gen1_stmp_cfg::W](gen1_stmp_cfg::W) writer structure"]
impl crate::Writable for GEN1_STMP_CFG {}
#[doc = "MCPWM_GEN1_STMP_CFG_REG(i)"]
pub mod gen1_stmp_cfg;
#[doc = "MCPWM_GEN1_TSTMP_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_tstmp_a](gen1_tstmp_a) module"]
pub type GEN1_TSTMP_A = crate::Reg<u32, _GEN1_TSTMP_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_TSTMP_A;
#[doc = "`read()` method returns [gen1_tstmp_a::R](gen1_tstmp_a::R) reader structure"]
impl crate::Readable for GEN1_TSTMP_A {}
#[doc = "`write(|w| ..)` method takes [gen1_tstmp_a::W](gen1_tstmp_a::W) writer structure"]
impl crate::Writable for GEN1_TSTMP_A {}
#[doc = "MCPWM_GEN1_TSTMP_A_REG(i)"]
pub mod gen1_tstmp_a;
#[doc = "MCPWM_GEN1_TSTMP_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_tstmp_b](gen1_tstmp_b) module"]
pub type GEN1_TSTMP_B = crate::Reg<u32, _GEN1_TSTMP_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_TSTMP_B;
#[doc = "`read()` method returns [gen1_tstmp_b::R](gen1_tstmp_b::R) reader structure"]
impl crate::Readable for GEN1_TSTMP_B {}
#[doc = "`write(|w| ..)` method takes [gen1_tstmp_b::W](gen1_tstmp_b::W) writer structure"]
impl crate::Writable for GEN1_TSTMP_B {}
#[doc = "MCPWM_GEN1_TSTMP_B_REG(i)"]
pub mod gen1_tstmp_b;
#[doc = "MCPWM_GEN1_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_cfg0](gen1_cfg0) module"]
pub type GEN1_CFG0 = crate::Reg<u32, _GEN1_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_CFG0;
#[doc = "`read()` method returns [gen1_cfg0::R](gen1_cfg0::R) reader structure"]
impl crate::Readable for GEN1_CFG0 {}
#[doc = "`write(|w| ..)` method takes [gen1_cfg0::W](gen1_cfg0::W) writer structure"]
impl crate::Writable for GEN1_CFG0 {}
#[doc = "MCPWM_GEN1_CFG0_REG(i)"]
pub mod gen1_cfg0;
#[doc = "MCPWM_GEN1_FORCE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_force](gen1_force) module"]
pub type GEN1_FORCE = crate::Reg<u32, _GEN1_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_FORCE;
#[doc = "`read()` method returns [gen1_force::R](gen1_force::R) reader structure"]
impl crate::Readable for GEN1_FORCE {}
#[doc = "`write(|w| ..)` method takes [gen1_force::W](gen1_force::W) writer structure"]
impl crate::Writable for GEN1_FORCE {}
#[doc = "MCPWM_GEN1_FORCE_REG(i)"]
pub mod gen1_force;
#[doc = "MCPWM_GEN1_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_a](gen1_a) module"]
pub type GEN1_A = crate::Reg<u32, _GEN1_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_A;
#[doc = "`read()` method returns [gen1_a::R](gen1_a::R) reader structure"]
impl crate::Readable for GEN1_A {}
#[doc = "`write(|w| ..)` method takes [gen1_a::W](gen1_a::W) writer structure"]
impl crate::Writable for GEN1_A {}
#[doc = "MCPWM_GEN1_A_REG(i)"]
pub mod gen1_a;
#[doc = "MCPWM_GEN1_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen1_b](gen1_b) module"]
pub type GEN1_B = crate::Reg<u32, _GEN1_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN1_B;
#[doc = "`read()` method returns [gen1_b::R](gen1_b::R) reader structure"]
impl crate::Readable for GEN1_B {}
#[doc = "`write(|w| ..)` method takes [gen1_b::W](gen1_b::W) writer structure"]
impl crate::Writable for GEN1_B {}
#[doc = "MCPWM_GEN1_B_REG(i)"]
pub mod gen1_b;
#[doc = "MCPWM_DT1_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt1_cfg](dt1_cfg) module"]
pub type DT1_CFG = crate::Reg<u32, _DT1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT1_CFG;
#[doc = "`read()` method returns [dt1_cfg::R](dt1_cfg::R) reader structure"]
impl crate::Readable for DT1_CFG {}
#[doc = "`write(|w| ..)` method takes [dt1_cfg::W](dt1_cfg::W) writer structure"]
impl crate::Writable for DT1_CFG {}
#[doc = "MCPWM_DT1_CFG_REG(i)"]
pub mod dt1_cfg;
#[doc = "MCPWM_DT1_FED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt1_fed_cfg](dt1_fed_cfg) module"]
pub type DT1_FED_CFG = crate::Reg<u32, _DT1_FED_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT1_FED_CFG;
#[doc = "`read()` method returns [dt1_fed_cfg::R](dt1_fed_cfg::R) reader structure"]
impl crate::Readable for DT1_FED_CFG {}
#[doc = "`write(|w| ..)` method takes [dt1_fed_cfg::W](dt1_fed_cfg::W) writer structure"]
impl crate::Writable for DT1_FED_CFG {}
#[doc = "MCPWM_DT1_FED_CFG_REG(i)"]
pub mod dt1_fed_cfg;
#[doc = "MCPWM_DT1_RED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt1_red_cfg](dt1_red_cfg) module"]
pub type DT1_RED_CFG = crate::Reg<u32, _DT1_RED_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT1_RED_CFG;
#[doc = "`read()` method returns [dt1_red_cfg::R](dt1_red_cfg::R) reader structure"]
impl crate::Readable for DT1_RED_CFG {}
#[doc = "`write(|w| ..)` method takes [dt1_red_cfg::W](dt1_red_cfg::W) writer structure"]
impl crate::Writable for DT1_RED_CFG {}
#[doc = "MCPWM_DT1_RED_CFG_REG(i)"]
pub mod dt1_red_cfg;
#[doc = "MCPWM_CARRIER1_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [carrier1_cfg](carrier1_cfg) module"]
pub type CARRIER1_CFG = crate::Reg<u32, _CARRIER1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CARRIER1_CFG;
#[doc = "`read()` method returns [carrier1_cfg::R](carrier1_cfg::R) reader structure"]
impl crate::Readable for CARRIER1_CFG {}
#[doc = "`write(|w| ..)` method takes [carrier1_cfg::W](carrier1_cfg::W) writer structure"]
impl crate::Writable for CARRIER1_CFG {}
#[doc = "MCPWM_CARRIER1_CFG_REG(i)"]
pub mod carrier1_cfg;
#[doc = "MCPWM_FH1_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh1_cfg0](fh1_cfg0) module"]
pub type FH1_CFG0 = crate::Reg<u32, _FH1_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH1_CFG0;
#[doc = "`read()` method returns [fh1_cfg0::R](fh1_cfg0::R) reader structure"]
impl crate::Readable for FH1_CFG0 {}
#[doc = "`write(|w| ..)` method takes [fh1_cfg0::W](fh1_cfg0::W) writer structure"]
impl crate::Writable for FH1_CFG0 {}
#[doc = "MCPWM_FH1_CFG0_REG(i)"]
pub mod fh1_cfg0;
#[doc = "MCPWM_FH1_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh1_cfg1](fh1_cfg1) module"]
pub type FH1_CFG1 = crate::Reg<u32, _FH1_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH1_CFG1;
#[doc = "`read()` method returns [fh1_cfg1::R](fh1_cfg1::R) reader structure"]
impl crate::Readable for FH1_CFG1 {}
#[doc = "`write(|w| ..)` method takes [fh1_cfg1::W](fh1_cfg1::W) writer structure"]
impl crate::Writable for FH1_CFG1 {}
#[doc = "MCPWM_FH1_CFG1_REG(i)"]
pub mod fh1_cfg1;
#[doc = "MCPWM_FH1_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh1_status](fh1_status) module"]
pub type FH1_STATUS = crate::Reg<u32, _FH1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH1_STATUS;
#[doc = "`read()` method returns [fh1_status::R](fh1_status::R) reader structure"]
impl crate::Readable for FH1_STATUS {}
#[doc = "`write(|w| ..)` method takes [fh1_status::W](fh1_status::W) writer structure"]
impl crate::Writable for FH1_STATUS {}
#[doc = "MCPWM_FH1_STATUS_REG(i)"]
pub mod fh1_status;
#[doc = "MCPWM_GEN2_STMP_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_stmp_cfg](gen2_stmp_cfg) module"]
pub type GEN2_STMP_CFG = crate::Reg<u32, _GEN2_STMP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_STMP_CFG;
#[doc = "`read()` method returns [gen2_stmp_cfg::R](gen2_stmp_cfg::R) reader structure"]
impl crate::Readable for GEN2_STMP_CFG {}
#[doc = "`write(|w| ..)` method takes [gen2_stmp_cfg::W](gen2_stmp_cfg::W) writer structure"]
impl crate::Writable for GEN2_STMP_CFG {}
#[doc = "MCPWM_GEN2_STMP_CFG_REG(i)"]
pub mod gen2_stmp_cfg;
#[doc = "MCPWM_GEN2_TSTMP_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_tstmp_a](gen2_tstmp_a) module"]
pub type GEN2_TSTMP_A = crate::Reg<u32, _GEN2_TSTMP_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_TSTMP_A;
#[doc = "`read()` method returns [gen2_tstmp_a::R](gen2_tstmp_a::R) reader structure"]
impl crate::Readable for GEN2_TSTMP_A {}
#[doc = "`write(|w| ..)` method takes [gen2_tstmp_a::W](gen2_tstmp_a::W) writer structure"]
impl crate::Writable for GEN2_TSTMP_A {}
#[doc = "MCPWM_GEN2_TSTMP_A_REG(i)"]
pub mod gen2_tstmp_a;
#[doc = "MCPWM_GEN2_TSTMP_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_tstmp_b](gen2_tstmp_b) module"]
pub type GEN2_TSTMP_B = crate::Reg<u32, _GEN2_TSTMP_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_TSTMP_B;
#[doc = "`read()` method returns [gen2_tstmp_b::R](gen2_tstmp_b::R) reader structure"]
impl crate::Readable for GEN2_TSTMP_B {}
#[doc = "`write(|w| ..)` method takes [gen2_tstmp_b::W](gen2_tstmp_b::W) writer structure"]
impl crate::Writable for GEN2_TSTMP_B {}
#[doc = "MCPWM_GEN2_TSTMP_B_REG(i)"]
pub mod gen2_tstmp_b;
#[doc = "MCPWM_GEN2_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_cfg0](gen2_cfg0) module"]
pub type GEN2_CFG0 = crate::Reg<u32, _GEN2_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_CFG0;
#[doc = "`read()` method returns [gen2_cfg0::R](gen2_cfg0::R) reader structure"]
impl crate::Readable for GEN2_CFG0 {}
#[doc = "`write(|w| ..)` method takes [gen2_cfg0::W](gen2_cfg0::W) writer structure"]
impl crate::Writable for GEN2_CFG0 {}
#[doc = "MCPWM_GEN2_CFG0_REG(i)"]
pub mod gen2_cfg0;
#[doc = "MCPWM_GEN2_FORCE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_force](gen2_force) module"]
pub type GEN2_FORCE = crate::Reg<u32, _GEN2_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_FORCE;
#[doc = "`read()` method returns [gen2_force::R](gen2_force::R) reader structure"]
impl crate::Readable for GEN2_FORCE {}
#[doc = "`write(|w| ..)` method takes [gen2_force::W](gen2_force::W) writer structure"]
impl crate::Writable for GEN2_FORCE {}
#[doc = "MCPWM_GEN2_FORCE_REG(i)"]
pub mod gen2_force;
#[doc = "MCPWM_GEN2_A_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_a](gen2_a) module"]
pub type GEN2_A = crate::Reg<u32, _GEN2_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_A;
#[doc = "`read()` method returns [gen2_a::R](gen2_a::R) reader structure"]
impl crate::Readable for GEN2_A {}
#[doc = "`write(|w| ..)` method takes [gen2_a::W](gen2_a::W) writer structure"]
impl crate::Writable for GEN2_A {}
#[doc = "MCPWM_GEN2_A_REG(i)"]
pub mod gen2_a;
#[doc = "MCPWM_GEN2_B_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gen2_b](gen2_b) module"]
pub type GEN2_B = crate::Reg<u32, _GEN2_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN2_B;
#[doc = "`read()` method returns [gen2_b::R](gen2_b::R) reader structure"]
impl crate::Readable for GEN2_B {}
#[doc = "`write(|w| ..)` method takes [gen2_b::W](gen2_b::W) writer structure"]
impl crate::Writable for GEN2_B {}
#[doc = "MCPWM_GEN2_B_REG(i)"]
pub mod gen2_b;
#[doc = "MCPWM_DT2_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt2_cfg](dt2_cfg) module"]
pub type DT2_CFG = crate::Reg<u32, _DT2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT2_CFG;
#[doc = "`read()` method returns [dt2_cfg::R](dt2_cfg::R) reader structure"]
impl crate::Readable for DT2_CFG {}
#[doc = "`write(|w| ..)` method takes [dt2_cfg::W](dt2_cfg::W) writer structure"]
impl crate::Writable for DT2_CFG {}
#[doc = "MCPWM_DT2_CFG_REG(i)"]
pub mod dt2_cfg;
#[doc = "MCPWM_DT2_FED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt2_fed_cfg](dt2_fed_cfg) module"]
pub type DT2_FED_CFG = crate::Reg<u32, _DT2_FED_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT2_FED_CFG;
#[doc = "`read()` method returns [dt2_fed_cfg::R](dt2_fed_cfg::R) reader structure"]
impl crate::Readable for DT2_FED_CFG {}
#[doc = "`write(|w| ..)` method takes [dt2_fed_cfg::W](dt2_fed_cfg::W) writer structure"]
impl crate::Writable for DT2_FED_CFG {}
#[doc = "MCPWM_DT2_FED_CFG_REG(i)"]
pub mod dt2_fed_cfg;
#[doc = "MCPWM_DT2_RED_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dt2_red_cfg](dt2_red_cfg) module"]
pub type DT2_RED_CFG = crate::Reg<u32, _DT2_RED_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT2_RED_CFG;
#[doc = "`read()` method returns [dt2_red_cfg::R](dt2_red_cfg::R) reader structure"]
impl crate::Readable for DT2_RED_CFG {}
#[doc = "`write(|w| ..)` method takes [dt2_red_cfg::W](dt2_red_cfg::W) writer structure"]
impl crate::Writable for DT2_RED_CFG {}
#[doc = "MCPWM_DT2_RED_CFG_REG(i)"]
pub mod dt2_red_cfg;
#[doc = "MCPWM_CARRIER2_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [carrier2_cfg](carrier2_cfg) module"]
pub type CARRIER2_CFG = crate::Reg<u32, _CARRIER2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CARRIER2_CFG;
#[doc = "`read()` method returns [carrier2_cfg::R](carrier2_cfg::R) reader structure"]
impl crate::Readable for CARRIER2_CFG {}
#[doc = "`write(|w| ..)` method takes [carrier2_cfg::W](carrier2_cfg::W) writer structure"]
impl crate::Writable for CARRIER2_CFG {}
#[doc = "MCPWM_CARRIER2_CFG_REG(i)"]
pub mod carrier2_cfg;
#[doc = "MCPWM_FH2_CFG0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh2_cfg0](fh2_cfg0) module"]
pub type FH2_CFG0 = crate::Reg<u32, _FH2_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH2_CFG0;
#[doc = "`read()` method returns [fh2_cfg0::R](fh2_cfg0::R) reader structure"]
impl crate::Readable for FH2_CFG0 {}
#[doc = "`write(|w| ..)` method takes [fh2_cfg0::W](fh2_cfg0::W) writer structure"]
impl crate::Writable for FH2_CFG0 {}
#[doc = "MCPWM_FH2_CFG0_REG(i)"]
pub mod fh2_cfg0;
#[doc = "MCPWM_FH2_CFG1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh2_cfg1](fh2_cfg1) module"]
pub type FH2_CFG1 = crate::Reg<u32, _FH2_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH2_CFG1;
#[doc = "`read()` method returns [fh2_cfg1::R](fh2_cfg1::R) reader structure"]
impl crate::Readable for FH2_CFG1 {}
#[doc = "`write(|w| ..)` method takes [fh2_cfg1::W](fh2_cfg1::W) writer structure"]
impl crate::Writable for FH2_CFG1 {}
#[doc = "MCPWM_FH2_CFG1_REG(i)"]
pub mod fh2_cfg1;
#[doc = "MCPWM_FH2_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fh2_status](fh2_status) module"]
pub type FH2_STATUS = crate::Reg<u32, _FH2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FH2_STATUS;
#[doc = "`read()` method returns [fh2_status::R](fh2_status::R) reader structure"]
impl crate::Readable for FH2_STATUS {}
#[doc = "`write(|w| ..)` method takes [fh2_status::W](fh2_status::W) writer structure"]
impl crate::Writable for FH2_STATUS {}
#[doc = "MCPWM_FH2_STATUS_REG(i)"]
pub mod fh2_status;
#[doc = "MCPWM_FAULT_DETECT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fault_detect](fault_detect) module"]
pub type FAULT_DETECT = crate::Reg<u32, _FAULT_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULT_DETECT;
#[doc = "`read()` method returns [fault_detect::R](fault_detect::R) reader structure"]
impl crate::Readable for FAULT_DETECT {}
#[doc = "`write(|w| ..)` method takes [fault_detect::W](fault_detect::W) writer structure"]
impl crate::Writable for FAULT_DETECT {}
#[doc = "MCPWM_FAULT_DETECT_REG(i)"]
pub mod fault_detect;
#[doc = "MCPWM_CAP_TIMER_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_timer_cfg](cap_timer_cfg) module"]
pub type CAP_TIMER_CFG = crate::Reg<u32, _CAP_TIMER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_TIMER_CFG;
#[doc = "`read()` method returns [cap_timer_cfg::R](cap_timer_cfg::R) reader structure"]
impl crate::Readable for CAP_TIMER_CFG {}
#[doc = "`write(|w| ..)` method takes [cap_timer_cfg::W](cap_timer_cfg::W) writer structure"]
impl crate::Writable for CAP_TIMER_CFG {}
#[doc = "MCPWM_CAP_TIMER_CFG_REG(i)"]
pub mod cap_timer_cfg;
#[doc = "MCPWM_CAP_TIMER_PHASE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_timer_phase](cap_timer_phase) module"]
pub type CAP_TIMER_PHASE = crate::Reg<u32, _CAP_TIMER_PHASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_TIMER_PHASE;
#[doc = "`read()` method returns [cap_timer_phase::R](cap_timer_phase::R) reader structure"]
impl crate::Readable for CAP_TIMER_PHASE {}
#[doc = "`write(|w| ..)` method takes [cap_timer_phase::W](cap_timer_phase::W) writer structure"]
impl crate::Writable for CAP_TIMER_PHASE {}
#[doc = "MCPWM_CAP_TIMER_PHASE_REG(i)"]
pub mod cap_timer_phase;
#[doc = "MCPWM_CAP_CH0_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_ch0_cfg](cap_ch0_cfg) module"]
pub type CAP_CH0_CFG = crate::Reg<u32, _CAP_CH0_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CH0_CFG;
#[doc = "`read()` method returns [cap_ch0_cfg::R](cap_ch0_cfg::R) reader structure"]
impl crate::Readable for CAP_CH0_CFG {}
#[doc = "`write(|w| ..)` method takes [cap_ch0_cfg::W](cap_ch0_cfg::W) writer structure"]
impl crate::Writable for CAP_CH0_CFG {}
#[doc = "MCPWM_CAP_CH0_CFG_REG(i)"]
pub mod cap_ch0_cfg;
#[doc = "MCPWM_CAP_CH1_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_ch1_cfg](cap_ch1_cfg) module"]
pub type CAP_CH1_CFG = crate::Reg<u32, _CAP_CH1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CH1_CFG;
#[doc = "`read()` method returns [cap_ch1_cfg::R](cap_ch1_cfg::R) reader structure"]
impl crate::Readable for CAP_CH1_CFG {}
#[doc = "`write(|w| ..)` method takes [cap_ch1_cfg::W](cap_ch1_cfg::W) writer structure"]
impl crate::Writable for CAP_CH1_CFG {}
#[doc = "MCPWM_CAP_CH1_CFG_REG(i)"]
pub mod cap_ch1_cfg;
#[doc = "MCPWM_CAP_CH2_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_ch2_cfg](cap_ch2_cfg) module"]
pub type CAP_CH2_CFG = crate::Reg<u32, _CAP_CH2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CH2_CFG;
#[doc = "`read()` method returns [cap_ch2_cfg::R](cap_ch2_cfg::R) reader structure"]
impl crate::Readable for CAP_CH2_CFG {}
#[doc = "`write(|w| ..)` method takes [cap_ch2_cfg::W](cap_ch2_cfg::W) writer structure"]
impl crate::Writable for CAP_CH2_CFG {}
#[doc = "MCPWM_CAP_CH2_CFG_REG(i)"]
pub mod cap_ch2_cfg;
#[doc = "MCPWM_CAP_CH0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_ch0](cap_ch0) module"]
pub type CAP_CH0 = crate::Reg<u32, _CAP_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CH0;
#[doc = "`read()` method returns [cap_ch0::R](cap_ch0::R) reader structure"]
impl crate::Readable for CAP_CH0 {}
#[doc = "`write(|w| ..)` method takes [cap_ch0::W](cap_ch0::W) writer structure"]
impl crate::Writable for CAP_CH0 {}
#[doc = "MCPWM_CAP_CH0_REG(i)"]
pub mod cap_ch0;
#[doc = "MCPWM_CAP_CH1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_ch1](cap_ch1) module"]
pub type CAP_CH1 = crate::Reg<u32, _CAP_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CH1;
#[doc = "`read()` method returns [cap_ch1::R](cap_ch1::R) reader structure"]
impl crate::Readable for CAP_CH1 {}
#[doc = "`write(|w| ..)` method takes [cap_ch1::W](cap_ch1::W) writer structure"]
impl crate::Writable for CAP_CH1 {}
#[doc = "MCPWM_CAP_CH1_REG(i)"]
pub mod cap_ch1;
#[doc = "MCPWM_CAP_CH2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_ch2](cap_ch2) module"]
pub type CAP_CH2 = crate::Reg<u32, _CAP_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CH2;
#[doc = "`read()` method returns [cap_ch2::R](cap_ch2::R) reader structure"]
impl crate::Readable for CAP_CH2 {}
#[doc = "`write(|w| ..)` method takes [cap_ch2::W](cap_ch2::W) writer structure"]
impl crate::Writable for CAP_CH2 {}
#[doc = "MCPWM_CAP_CH2_REG(i)"]
pub mod cap_ch2;
#[doc = "MCPWM_CAP_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap_status](cap_status) module"]
pub type CAP_STATUS = crate::Reg<u32, _CAP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_STATUS;
#[doc = "`read()` method returns [cap_status::R](cap_status::R) reader structure"]
impl crate::Readable for CAP_STATUS {}
#[doc = "`write(|w| ..)` method takes [cap_status::W](cap_status::W) writer structure"]
impl crate::Writable for CAP_STATUS {}
#[doc = "MCPWM_CAP_STATUS_REG(i)"]
pub mod cap_status;
#[doc = "MCPWM_UPDATE_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [update_cfg](update_cfg) module"]
pub type UPDATE_CFG = crate::Reg<u32, _UPDATE_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPDATE_CFG;
#[doc = "`read()` method returns [update_cfg::R](update_cfg::R) reader structure"]
impl crate::Readable for UPDATE_CFG {}
#[doc = "`write(|w| ..)` method takes [update_cfg::W](update_cfg::W) writer structure"]
impl crate::Writable for UPDATE_CFG {}
#[doc = "MCPWM_UPDATE_CFG_REG(i)"]
pub mod update_cfg;
#[doc = "MCMCPWM_INT_ENA_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_ena_mcpwm](mcmcpwm_int_ena_mcpwm) module"]
pub type MCMCPWM_INT_ENA_MCPWM = crate::Reg<u32, _MCMCPWM_INT_ENA_MCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_ENA_MCPWM;
#[doc = "`read()` method returns [mcmcpwm_int_ena_mcpwm::R](mcmcpwm_int_ena_mcpwm::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_ENA_MCPWM {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_ena_mcpwm::W](mcmcpwm_int_ena_mcpwm::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_ENA_MCPWM {}
#[doc = "MCMCPWM_INT_ENA_MCPWM_REG(i)"]
pub mod mcmcpwm_int_ena_mcpwm;
#[doc = "MCMCPWM_INT_RAW_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_raw_mcpwm](mcmcpwm_int_raw_mcpwm) module"]
pub type MCMCPWM_INT_RAW_MCPWM = crate::Reg<u32, _MCMCPWM_INT_RAW_MCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_RAW_MCPWM;
#[doc = "`read()` method returns [mcmcpwm_int_raw_mcpwm::R](mcmcpwm_int_raw_mcpwm::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_RAW_MCPWM {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_raw_mcpwm::W](mcmcpwm_int_raw_mcpwm::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_RAW_MCPWM {}
#[doc = "MCMCPWM_INT_RAW_MCPWM_REG(i)"]
pub mod mcmcpwm_int_raw_mcpwm;
#[doc = "MCMCPWM_INT_ST_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_st_mcpwm](mcmcpwm_int_st_mcpwm) module"]
pub type MCMCPWM_INT_ST_MCPWM = crate::Reg<u32, _MCMCPWM_INT_ST_MCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_ST_MCPWM;
#[doc = "`read()` method returns [mcmcpwm_int_st_mcpwm::R](mcmcpwm_int_st_mcpwm::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_ST_MCPWM {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_st_mcpwm::W](mcmcpwm_int_st_mcpwm::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_ST_MCPWM {}
#[doc = "MCMCPWM_INT_ST_MCPWM_REG(i)"]
pub mod mcmcpwm_int_st_mcpwm;
#[doc = "MCMCPWM_INT_CLR_MCPWM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmcpwm_int_clr_mcpwm](mcmcpwm_int_clr_mcpwm) module"]
pub type MCMCPWM_INT_CLR_MCPWM = crate::Reg<u32, _MCMCPWM_INT_CLR_MCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMCPWM_INT_CLR_MCPWM;
#[doc = "`read()` method returns [mcmcpwm_int_clr_mcpwm::R](mcmcpwm_int_clr_mcpwm::R) reader structure"]
impl crate::Readable for MCMCPWM_INT_CLR_MCPWM {}
#[doc = "`write(|w| ..)` method takes [mcmcpwm_int_clr_mcpwm::W](mcmcpwm_int_clr_mcpwm::W) writer structure"]
impl crate::Writable for MCMCPWM_INT_CLR_MCPWM {}
#[doc = "MCMCPWM_INT_CLR_MCPWM_REG(i)"]
pub mod mcmcpwm_int_clr_mcpwm;
#[doc = "MCPWM_CLK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "MCPWM_CLK_REG(i)"]
pub mod clk;
#[doc = "MCPWM_VERSION_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "`write(|w| ..)` method takes [version::W](version::W) writer structure"]
impl crate::Writable for VERSION {}
#[doc = "MCPWM_VERSION_REG(i)"]
pub mod version;
