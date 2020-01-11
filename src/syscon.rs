#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCON_SYSCLK_CONF_REG"]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x04 - SYSCON_XTAL_TICK_CONF_REG"]
    pub xtal_tick_conf: XTAL_TICK_CONF,
    #[doc = "0x08 - SYSCON_PLL_TICK_CONF_REG"]
    pub pll_tick_conf: PLL_TICK_CONF,
    #[doc = "0x0c - SYSCON_CK8M_TICK_CONF_REG"]
    pub ck8m_tick_conf: CK8M_TICK_CONF,
    #[doc = "0x10 - SYSCON_SARADC_CTRL_REG"]
    pub saradc_ctrl: SARADC_CTRL,
    #[doc = "0x14 - SYSCON_SARADC_CTRL2_REG"]
    pub saradc_ctrl2: SARADC_CTRL2,
    #[doc = "0x18 - SYSCON_SARADC_FSM_REG"]
    pub saradc_fsm: SARADC_FSM,
    #[doc = "0x1c - SYSCON_SARADC_SAR1_PATT_TAB1_REG"]
    pub saradc_sar1_patt_tab1: SARADC_SAR1_PATT_TAB1,
    #[doc = "0x20 - SYSCON_SARADC_SAR1_PATT_TAB2_REG"]
    pub saradc_sar1_patt_tab2: SARADC_SAR1_PATT_TAB2,
    #[doc = "0x24 - SYSCON_SARADC_SAR1_PATT_TAB3_REG"]
    pub saradc_sar1_patt_tab3: SARADC_SAR1_PATT_TAB3,
    #[doc = "0x28 - SYSCON_SARADC_SAR1_PATT_TAB4_REG"]
    pub saradc_sar1_patt_tab4: SARADC_SAR1_PATT_TAB4,
    #[doc = "0x2c - SYSCON_SARADC_SAR2_PATT_TAB1_REG"]
    pub saradc_sar2_patt_tab1: SARADC_SAR2_PATT_TAB1,
    #[doc = "0x30 - SYSCON_SARADC_SAR2_PATT_TAB2_REG"]
    pub saradc_sar2_patt_tab2: SARADC_SAR2_PATT_TAB2,
    #[doc = "0x34 - SYSCON_SARADC_SAR2_PATT_TAB3_REG"]
    pub saradc_sar2_patt_tab3: SARADC_SAR2_PATT_TAB3,
    #[doc = "0x38 - SYSCON_SARADC_SAR2_PATT_TAB4_REG"]
    pub saradc_sar2_patt_tab4: SARADC_SAR2_PATT_TAB4,
    #[doc = "0x3c - SYSCON_APLL_TICK_CONF_REG"]
    pub apll_tick_conf: APLL_TICK_CONF,
    _reserved16: [u8; 60usize],
    #[doc = "0x7c - SYSCON_DATE_REG"]
    pub date: DATE,
}
#[doc = "SYSCON_SYSCLK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysclk_conf](sysclk_conf) module"]
pub type SYSCLK_CONF = crate::Reg<u32, _SYSCLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCLK_CONF;
#[doc = "`read()` method returns [sysclk_conf::R](sysclk_conf::R) reader structure"]
impl crate::Readable for SYSCLK_CONF {}
#[doc = "`write(|w| ..)` method takes [sysclk_conf::W](sysclk_conf::W) writer structure"]
impl crate::Writable for SYSCLK_CONF {}
#[doc = "SYSCON_SYSCLK_CONF_REG"]
pub mod sysclk_conf;
#[doc = "SYSCON_XTAL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xtal_tick_conf](xtal_tick_conf) module"]
pub type XTAL_TICK_CONF = crate::Reg<u32, _XTAL_TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL_TICK_CONF;
#[doc = "`read()` method returns [xtal_tick_conf::R](xtal_tick_conf::R) reader structure"]
impl crate::Readable for XTAL_TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [xtal_tick_conf::W](xtal_tick_conf::W) writer structure"]
impl crate::Writable for XTAL_TICK_CONF {}
#[doc = "SYSCON_XTAL_TICK_CONF_REG"]
pub mod xtal_tick_conf;
#[doc = "SYSCON_PLL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_tick_conf](pll_tick_conf) module"]
pub type PLL_TICK_CONF = crate::Reg<u32, _PLL_TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_TICK_CONF;
#[doc = "`read()` method returns [pll_tick_conf::R](pll_tick_conf::R) reader structure"]
impl crate::Readable for PLL_TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [pll_tick_conf::W](pll_tick_conf::W) writer structure"]
impl crate::Writable for PLL_TICK_CONF {}
#[doc = "SYSCON_PLL_TICK_CONF_REG"]
pub mod pll_tick_conf;
#[doc = "SYSCON_CK8M_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ck8m_tick_conf](ck8m_tick_conf) module"]
pub type CK8M_TICK_CONF = crate::Reg<u32, _CK8M_TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CK8M_TICK_CONF;
#[doc = "`read()` method returns [ck8m_tick_conf::R](ck8m_tick_conf::R) reader structure"]
impl crate::Readable for CK8M_TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [ck8m_tick_conf::W](ck8m_tick_conf::W) writer structure"]
impl crate::Writable for CK8M_TICK_CONF {}
#[doc = "SYSCON_CK8M_TICK_CONF_REG"]
pub mod ck8m_tick_conf;
#[doc = "SYSCON_SARADC_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_ctrl](saradc_ctrl) module"]
pub type SARADC_CTRL = crate::Reg<u32, _SARADC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_CTRL;
#[doc = "`read()` method returns [saradc_ctrl::R](saradc_ctrl::R) reader structure"]
impl crate::Readable for SARADC_CTRL {}
#[doc = "`write(|w| ..)` method takes [saradc_ctrl::W](saradc_ctrl::W) writer structure"]
impl crate::Writable for SARADC_CTRL {}
#[doc = "SYSCON_SARADC_CTRL_REG"]
pub mod saradc_ctrl;
#[doc = "SYSCON_SARADC_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_ctrl2](saradc_ctrl2) module"]
pub type SARADC_CTRL2 = crate::Reg<u32, _SARADC_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_CTRL2;
#[doc = "`read()` method returns [saradc_ctrl2::R](saradc_ctrl2::R) reader structure"]
impl crate::Readable for SARADC_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [saradc_ctrl2::W](saradc_ctrl2::W) writer structure"]
impl crate::Writable for SARADC_CTRL2 {}
#[doc = "SYSCON_SARADC_CTRL2_REG"]
pub mod saradc_ctrl2;
#[doc = "SYSCON_SARADC_FSM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_fsm](saradc_fsm) module"]
pub type SARADC_FSM = crate::Reg<u32, _SARADC_FSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_FSM;
#[doc = "`read()` method returns [saradc_fsm::R](saradc_fsm::R) reader structure"]
impl crate::Readable for SARADC_FSM {}
#[doc = "`write(|w| ..)` method takes [saradc_fsm::W](saradc_fsm::W) writer structure"]
impl crate::Writable for SARADC_FSM {}
#[doc = "SYSCON_SARADC_FSM_REG"]
pub mod saradc_fsm;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar1_patt_tab1](saradc_sar1_patt_tab1) module"]
pub type SARADC_SAR1_PATT_TAB1 = crate::Reg<u32, _SARADC_SAR1_PATT_TAB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR1_PATT_TAB1;
#[doc = "`read()` method returns [saradc_sar1_patt_tab1::R](saradc_sar1_patt_tab1::R) reader structure"]
impl crate::Readable for SARADC_SAR1_PATT_TAB1 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar1_patt_tab1::W](saradc_sar1_patt_tab1::W) writer structure"]
impl crate::Writable for SARADC_SAR1_PATT_TAB1 {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB1_REG"]
pub mod saradc_sar1_patt_tab1;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar1_patt_tab2](saradc_sar1_patt_tab2) module"]
pub type SARADC_SAR1_PATT_TAB2 = crate::Reg<u32, _SARADC_SAR1_PATT_TAB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR1_PATT_TAB2;
#[doc = "`read()` method returns [saradc_sar1_patt_tab2::R](saradc_sar1_patt_tab2::R) reader structure"]
impl crate::Readable for SARADC_SAR1_PATT_TAB2 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar1_patt_tab2::W](saradc_sar1_patt_tab2::W) writer structure"]
impl crate::Writable for SARADC_SAR1_PATT_TAB2 {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB2_REG"]
pub mod saradc_sar1_patt_tab2;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar1_patt_tab3](saradc_sar1_patt_tab3) module"]
pub type SARADC_SAR1_PATT_TAB3 = crate::Reg<u32, _SARADC_SAR1_PATT_TAB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR1_PATT_TAB3;
#[doc = "`read()` method returns [saradc_sar1_patt_tab3::R](saradc_sar1_patt_tab3::R) reader structure"]
impl crate::Readable for SARADC_SAR1_PATT_TAB3 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar1_patt_tab3::W](saradc_sar1_patt_tab3::W) writer structure"]
impl crate::Writable for SARADC_SAR1_PATT_TAB3 {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB3_REG"]
pub mod saradc_sar1_patt_tab3;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar1_patt_tab4](saradc_sar1_patt_tab4) module"]
pub type SARADC_SAR1_PATT_TAB4 = crate::Reg<u32, _SARADC_SAR1_PATT_TAB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR1_PATT_TAB4;
#[doc = "`read()` method returns [saradc_sar1_patt_tab4::R](saradc_sar1_patt_tab4::R) reader structure"]
impl crate::Readable for SARADC_SAR1_PATT_TAB4 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar1_patt_tab4::W](saradc_sar1_patt_tab4::W) writer structure"]
impl crate::Writable for SARADC_SAR1_PATT_TAB4 {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB4_REG"]
pub mod saradc_sar1_patt_tab4;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar2_patt_tab1](saradc_sar2_patt_tab1) module"]
pub type SARADC_SAR2_PATT_TAB1 = crate::Reg<u32, _SARADC_SAR2_PATT_TAB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR2_PATT_TAB1;
#[doc = "`read()` method returns [saradc_sar2_patt_tab1::R](saradc_sar2_patt_tab1::R) reader structure"]
impl crate::Readable for SARADC_SAR2_PATT_TAB1 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar2_patt_tab1::W](saradc_sar2_patt_tab1::W) writer structure"]
impl crate::Writable for SARADC_SAR2_PATT_TAB1 {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB1_REG"]
pub mod saradc_sar2_patt_tab1;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar2_patt_tab2](saradc_sar2_patt_tab2) module"]
pub type SARADC_SAR2_PATT_TAB2 = crate::Reg<u32, _SARADC_SAR2_PATT_TAB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR2_PATT_TAB2;
#[doc = "`read()` method returns [saradc_sar2_patt_tab2::R](saradc_sar2_patt_tab2::R) reader structure"]
impl crate::Readable for SARADC_SAR2_PATT_TAB2 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar2_patt_tab2::W](saradc_sar2_patt_tab2::W) writer structure"]
impl crate::Writable for SARADC_SAR2_PATT_TAB2 {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB2_REG"]
pub mod saradc_sar2_patt_tab2;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar2_patt_tab3](saradc_sar2_patt_tab3) module"]
pub type SARADC_SAR2_PATT_TAB3 = crate::Reg<u32, _SARADC_SAR2_PATT_TAB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR2_PATT_TAB3;
#[doc = "`read()` method returns [saradc_sar2_patt_tab3::R](saradc_sar2_patt_tab3::R) reader structure"]
impl crate::Readable for SARADC_SAR2_PATT_TAB3 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar2_patt_tab3::W](saradc_sar2_patt_tab3::W) writer structure"]
impl crate::Writable for SARADC_SAR2_PATT_TAB3 {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB3_REG"]
pub mod saradc_sar2_patt_tab3;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saradc_sar2_patt_tab4](saradc_sar2_patt_tab4) module"]
pub type SARADC_SAR2_PATT_TAB4 = crate::Reg<u32, _SARADC_SAR2_PATT_TAB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_SAR2_PATT_TAB4;
#[doc = "`read()` method returns [saradc_sar2_patt_tab4::R](saradc_sar2_patt_tab4::R) reader structure"]
impl crate::Readable for SARADC_SAR2_PATT_TAB4 {}
#[doc = "`write(|w| ..)` method takes [saradc_sar2_patt_tab4::W](saradc_sar2_patt_tab4::W) writer structure"]
impl crate::Writable for SARADC_SAR2_PATT_TAB4 {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB4_REG"]
pub mod saradc_sar2_patt_tab4;
#[doc = "SYSCON_APLL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apll_tick_conf](apll_tick_conf) module"]
pub type APLL_TICK_CONF = crate::Reg<u32, _APLL_TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLL_TICK_CONF;
#[doc = "`read()` method returns [apll_tick_conf::R](apll_tick_conf::R) reader structure"]
impl crate::Readable for APLL_TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [apll_tick_conf::W](apll_tick_conf::W) writer structure"]
impl crate::Writable for APLL_TICK_CONF {}
#[doc = "SYSCON_APLL_TICK_CONF_REG"]
pub mod apll_tick_conf;
#[doc = "SYSCON_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "SYSCON_DATE_REG"]
pub mod date;
