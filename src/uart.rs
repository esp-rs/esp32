#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - UART_INT_RAW_REG(i)"]
    pub uart_int_raw_regi: UART_INT_RAW_REGI,
    #[doc = "0x08 - UART_INT_ST_REG(i)"]
    pub uart_int_st_regi: UART_INT_ST_REGI,
    #[doc = "0x0c - UART_INT_ENA_REG(i)"]
    pub uart_int_ena_regi: UART_INT_ENA_REGI,
    #[doc = "0x10 - UART_INT_CLR_REG(i)"]
    pub uart_int_clr_regi: UART_INT_CLR_REGI,
    #[doc = "0x14 - UART_CLKDIV_REG(i)"]
    pub uart_clkdiv_regi: UART_CLKDIV_REGI,
    #[doc = "0x18 - UART_AUTOBAUD_REG(i)"]
    pub uart_autobaud_regi: UART_AUTOBAUD_REGI,
    #[doc = "0x1c - UART_STATUS_REG(i)"]
    pub uart_status_regi: UART_STATUS_REGI,
    #[doc = "0x20 - UART_CONF0_REG(i)"]
    pub uart_conf0_regi: UART_CONF0_REGI,
    #[doc = "0x24 - UART_CONF1_REG(i)"]
    pub uart_conf1_regi: UART_CONF1_REGI,
    #[doc = "0x28 - UART_LOWPULSE_REG(i)"]
    pub uart_lowpulse_regi: UART_LOWPULSE_REGI,
    #[doc = "0x2c - UART_HIGHPULSE_REG(i)"]
    pub uart_highpulse_regi: UART_HIGHPULSE_REGI,
    #[doc = "0x30 - UART_RXD_CNT_REG(i)"]
    pub uart_rxd_cnt_regi: UART_RXD_CNT_REGI,
    #[doc = "0x34 - UART_FLOW_CONF_REG(i)"]
    pub uart_flow_conf_regi: UART_FLOW_CONF_REGI,
    #[doc = "0x38 - UART_SLEEP_CONF_REG(i)"]
    pub uart_sleep_conf_regi: UART_SLEEP_CONF_REGI,
    #[doc = "0x3c - UART_SWFC_CONF_REG(i)"]
    pub uart_swfc_conf_regi: UART_SWFC_CONF_REGI,
    #[doc = "0x40 - UART_IDLE_CONF_REG(i)"]
    pub uart_idle_conf_regi: UART_IDLE_CONF_REGI,
    #[doc = "0x44 - UART_RS485_CONF_REG(i)"]
    pub uart_rs485_conf_regi: UART_RS485_CONF_REGI,
    #[doc = "0x48 - UART_AT_CMD_PRECNT_REG(i)"]
    pub uart_at_cmd_precnt_regi: UART_AT_CMD_PRECNT_REGI,
    #[doc = "0x4c - UART_AT_CMD_POSTCNT_REG(i)"]
    pub uart_at_cmd_postcnt_regi: UART_AT_CMD_POSTCNT_REGI,
    #[doc = "0x50 - UART_AT_CMD_GAPTOUT_REG(i)"]
    pub uart_at_cmd_gaptout_regi: UART_AT_CMD_GAPTOUT_REGI,
    #[doc = "0x54 - UART_AT_CMD_CHAR_REG(i)"]
    pub uart_at_cmd_char_regi: UART_AT_CMD_CHAR_REGI,
    #[doc = "0x58 - UART_MEM_CONF_REG(i)"]
    pub uart_mem_conf_regi: UART_MEM_CONF_REGI,
    #[doc = "0x5c - UART_MEM_TX_STATUS_REG(i)"]
    pub uart_mem_tx_status_regi: UART_MEM_TX_STATUS_REGI,
    #[doc = "0x60 - UART_MEM_RX_STATUS_REG(i)"]
    pub uart_mem_rx_status_regi: UART_MEM_RX_STATUS_REGI,
    #[doc = "0x64 - UART_MEM_CNT_STATUS_REG(i)"]
    pub uart_mem_cnt_status_regi: UART_MEM_CNT_STATUS_REGI,
    #[doc = "0x68 - UART_POSPULSE_REG(i)"]
    pub uart_pospulse_regi: UART_POSPULSE_REGI,
    #[doc = "0x6c - UART_NEGPULSE_REG(i)"]
    pub uart_negpulse_regi: UART_NEGPULSE_REGI,
    _reserved27: [u8; 8usize],
    #[doc = "0x78 - UART_DATE_REG(i)"]
    pub uart_date_regi: UART_DATE_REGI,
    #[doc = "0x7c - UART_ID_REG(i)"]
    pub uart_id_regi: UART_ID_REGI,
}
#[doc = "UART_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_raw_regi](uart_int_raw_regi) module"]
pub type UART_INT_RAW_REGI = crate::Reg<u32, _UART_INT_RAW_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_RAW_REGI;
#[doc = "`read()` method returns [uart_int_raw_regi::R](uart_int_raw_regi::R) reader structure"]
impl crate::Readable for UART_INT_RAW_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_int_raw_regi::W](uart_int_raw_regi::W) writer structure"]
impl crate::Writable for UART_INT_RAW_REGI {}
#[doc = "UART_INT_RAW_REG(i)"]
pub mod uart_int_raw_regi;
#[doc = "UART_INT_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_st_regi](uart_int_st_regi) module"]
pub type UART_INT_ST_REGI = crate::Reg<u32, _UART_INT_ST_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_ST_REGI;
#[doc = "`read()` method returns [uart_int_st_regi::R](uart_int_st_regi::R) reader structure"]
impl crate::Readable for UART_INT_ST_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_int_st_regi::W](uart_int_st_regi::W) writer structure"]
impl crate::Writable for UART_INT_ST_REGI {}
#[doc = "UART_INT_ST_REG(i)"]
pub mod uart_int_st_regi;
#[doc = "UART_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_ena_regi](uart_int_ena_regi) module"]
pub type UART_INT_ENA_REGI = crate::Reg<u32, _UART_INT_ENA_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_ENA_REGI;
#[doc = "`read()` method returns [uart_int_ena_regi::R](uart_int_ena_regi::R) reader structure"]
impl crate::Readable for UART_INT_ENA_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_int_ena_regi::W](uart_int_ena_regi::W) writer structure"]
impl crate::Writable for UART_INT_ENA_REGI {}
#[doc = "UART_INT_ENA_REG(i)"]
pub mod uart_int_ena_regi;
#[doc = "UART_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_clr_regi](uart_int_clr_regi) module"]
pub type UART_INT_CLR_REGI = crate::Reg<u32, _UART_INT_CLR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_CLR_REGI;
#[doc = "`read()` method returns [uart_int_clr_regi::R](uart_int_clr_regi::R) reader structure"]
impl crate::Readable for UART_INT_CLR_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_int_clr_regi::W](uart_int_clr_regi::W) writer structure"]
impl crate::Writable for UART_INT_CLR_REGI {}
#[doc = "UART_INT_CLR_REG(i)"]
pub mod uart_int_clr_regi;
#[doc = "UART_CLKDIV_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_clkdiv_regi](uart_clkdiv_regi) module"]
pub type UART_CLKDIV_REGI = crate::Reg<u32, _UART_CLKDIV_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CLKDIV_REGI;
#[doc = "`read()` method returns [uart_clkdiv_regi::R](uart_clkdiv_regi::R) reader structure"]
impl crate::Readable for UART_CLKDIV_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_clkdiv_regi::W](uart_clkdiv_regi::W) writer structure"]
impl crate::Writable for UART_CLKDIV_REGI {}
#[doc = "UART_CLKDIV_REG(i)"]
pub mod uart_clkdiv_regi;
#[doc = "UART_AUTOBAUD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_autobaud_regi](uart_autobaud_regi) module"]
pub type UART_AUTOBAUD_REGI = crate::Reg<u32, _UART_AUTOBAUD_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AUTOBAUD_REGI;
#[doc = "`read()` method returns [uart_autobaud_regi::R](uart_autobaud_regi::R) reader structure"]
impl crate::Readable for UART_AUTOBAUD_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_autobaud_regi::W](uart_autobaud_regi::W) writer structure"]
impl crate::Writable for UART_AUTOBAUD_REGI {}
#[doc = "UART_AUTOBAUD_REG(i)"]
pub mod uart_autobaud_regi;
#[doc = "UART_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_status_regi](uart_status_regi) module"]
pub type UART_STATUS_REGI = crate::Reg<u32, _UART_STATUS_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_STATUS_REGI;
#[doc = "`read()` method returns [uart_status_regi::R](uart_status_regi::R) reader structure"]
impl crate::Readable for UART_STATUS_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_status_regi::W](uart_status_regi::W) writer structure"]
impl crate::Writable for UART_STATUS_REGI {}
#[doc = "UART_STATUS_REG(i)"]
pub mod uart_status_regi;
#[doc = "UART_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_conf0_regi](uart_conf0_regi) module"]
pub type UART_CONF0_REGI = crate::Reg<u32, _UART_CONF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CONF0_REGI;
#[doc = "`read()` method returns [uart_conf0_regi::R](uart_conf0_regi::R) reader structure"]
impl crate::Readable for UART_CONF0_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_conf0_regi::W](uart_conf0_regi::W) writer structure"]
impl crate::Writable for UART_CONF0_REGI {}
#[doc = "UART_CONF0_REG(i)"]
pub mod uart_conf0_regi;
#[doc = "UART_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_conf1_regi](uart_conf1_regi) module"]
pub type UART_CONF1_REGI = crate::Reg<u32, _UART_CONF1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CONF1_REGI;
#[doc = "`read()` method returns [uart_conf1_regi::R](uart_conf1_regi::R) reader structure"]
impl crate::Readable for UART_CONF1_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_conf1_regi::W](uart_conf1_regi::W) writer structure"]
impl crate::Writable for UART_CONF1_REGI {}
#[doc = "UART_CONF1_REG(i)"]
pub mod uart_conf1_regi;
#[doc = "UART_LOWPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_lowpulse_regi](uart_lowpulse_regi) module"]
pub type UART_LOWPULSE_REGI = crate::Reg<u32, _UART_LOWPULSE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_LOWPULSE_REGI;
#[doc = "`read()` method returns [uart_lowpulse_regi::R](uart_lowpulse_regi::R) reader structure"]
impl crate::Readable for UART_LOWPULSE_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_lowpulse_regi::W](uart_lowpulse_regi::W) writer structure"]
impl crate::Writable for UART_LOWPULSE_REGI {}
#[doc = "UART_LOWPULSE_REG(i)"]
pub mod uart_lowpulse_regi;
#[doc = "UART_HIGHPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_highpulse_regi](uart_highpulse_regi) module"]
pub type UART_HIGHPULSE_REGI = crate::Reg<u32, _UART_HIGHPULSE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_HIGHPULSE_REGI;
#[doc = "`read()` method returns [uart_highpulse_regi::R](uart_highpulse_regi::R) reader structure"]
impl crate::Readable for UART_HIGHPULSE_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_highpulse_regi::W](uart_highpulse_regi::W) writer structure"]
impl crate::Writable for UART_HIGHPULSE_REGI {}
#[doc = "UART_HIGHPULSE_REG(i)"]
pub mod uart_highpulse_regi;
#[doc = "UART_RXD_CNT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_rxd_cnt_regi](uart_rxd_cnt_regi) module"]
pub type UART_RXD_CNT_REGI = crate::Reg<u32, _UART_RXD_CNT_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RXD_CNT_REGI;
#[doc = "`read()` method returns [uart_rxd_cnt_regi::R](uart_rxd_cnt_regi::R) reader structure"]
impl crate::Readable for UART_RXD_CNT_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_rxd_cnt_regi::W](uart_rxd_cnt_regi::W) writer structure"]
impl crate::Writable for UART_RXD_CNT_REGI {}
#[doc = "UART_RXD_CNT_REG(i)"]
pub mod uart_rxd_cnt_regi;
#[doc = "UART_FLOW_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_flow_conf_regi](uart_flow_conf_regi) module"]
pub type UART_FLOW_CONF_REGI = crate::Reg<u32, _UART_FLOW_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FLOW_CONF_REGI;
#[doc = "`read()` method returns [uart_flow_conf_regi::R](uart_flow_conf_regi::R) reader structure"]
impl crate::Readable for UART_FLOW_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_flow_conf_regi::W](uart_flow_conf_regi::W) writer structure"]
impl crate::Writable for UART_FLOW_CONF_REGI {}
#[doc = "UART_FLOW_CONF_REG(i)"]
pub mod uart_flow_conf_regi;
#[doc = "UART_SLEEP_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_sleep_conf_regi](uart_sleep_conf_regi) module"]
pub type UART_SLEEP_CONF_REGI = crate::Reg<u32, _UART_SLEEP_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SLEEP_CONF_REGI;
#[doc = "`read()` method returns [uart_sleep_conf_regi::R](uart_sleep_conf_regi::R) reader structure"]
impl crate::Readable for UART_SLEEP_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_sleep_conf_regi::W](uart_sleep_conf_regi::W) writer structure"]
impl crate::Writable for UART_SLEEP_CONF_REGI {}
#[doc = "UART_SLEEP_CONF_REG(i)"]
pub mod uart_sleep_conf_regi;
#[doc = "UART_SWFC_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_swfc_conf_regi](uart_swfc_conf_regi) module"]
pub type UART_SWFC_CONF_REGI = crate::Reg<u32, _UART_SWFC_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SWFC_CONF_REGI;
#[doc = "`read()` method returns [uart_swfc_conf_regi::R](uart_swfc_conf_regi::R) reader structure"]
impl crate::Readable for UART_SWFC_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_swfc_conf_regi::W](uart_swfc_conf_regi::W) writer structure"]
impl crate::Writable for UART_SWFC_CONF_REGI {}
#[doc = "UART_SWFC_CONF_REG(i)"]
pub mod uart_swfc_conf_regi;
#[doc = "UART_IDLE_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_idle_conf_regi](uart_idle_conf_regi) module"]
pub type UART_IDLE_CONF_REGI = crate::Reg<u32, _UART_IDLE_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_IDLE_CONF_REGI;
#[doc = "`read()` method returns [uart_idle_conf_regi::R](uart_idle_conf_regi::R) reader structure"]
impl crate::Readable for UART_IDLE_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_idle_conf_regi::W](uart_idle_conf_regi::W) writer structure"]
impl crate::Writable for UART_IDLE_CONF_REGI {}
#[doc = "UART_IDLE_CONF_REG(i)"]
pub mod uart_idle_conf_regi;
#[doc = "UART_RS485_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_rs485_conf_regi](uart_rs485_conf_regi) module"]
pub type UART_RS485_CONF_REGI = crate::Reg<u32, _UART_RS485_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RS485_CONF_REGI;
#[doc = "`read()` method returns [uart_rs485_conf_regi::R](uart_rs485_conf_regi::R) reader structure"]
impl crate::Readable for UART_RS485_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_rs485_conf_regi::W](uart_rs485_conf_regi::W) writer structure"]
impl crate::Writable for UART_RS485_CONF_REGI {}
#[doc = "UART_RS485_CONF_REG(i)"]
pub mod uart_rs485_conf_regi;
#[doc = "UART_AT_CMD_PRECNT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_precnt_regi](uart_at_cmd_precnt_regi) module"]
pub type UART_AT_CMD_PRECNT_REGI = crate::Reg<u32, _UART_AT_CMD_PRECNT_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_PRECNT_REGI;
#[doc = "`read()` method returns [uart_at_cmd_precnt_regi::R](uart_at_cmd_precnt_regi::R) reader structure"]
impl crate::Readable for UART_AT_CMD_PRECNT_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_precnt_regi::W](uart_at_cmd_precnt_regi::W) writer structure"]
impl crate::Writable for UART_AT_CMD_PRECNT_REGI {}
#[doc = "UART_AT_CMD_PRECNT_REG(i)"]
pub mod uart_at_cmd_precnt_regi;
#[doc = "UART_AT_CMD_POSTCNT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_postcnt_regi](uart_at_cmd_postcnt_regi) module"]
pub type UART_AT_CMD_POSTCNT_REGI = crate::Reg<u32, _UART_AT_CMD_POSTCNT_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_POSTCNT_REGI;
#[doc = "`read()` method returns [uart_at_cmd_postcnt_regi::R](uart_at_cmd_postcnt_regi::R) reader structure"]
impl crate::Readable for UART_AT_CMD_POSTCNT_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_postcnt_regi::W](uart_at_cmd_postcnt_regi::W) writer structure"]
impl crate::Writable for UART_AT_CMD_POSTCNT_REGI {}
#[doc = "UART_AT_CMD_POSTCNT_REG(i)"]
pub mod uart_at_cmd_postcnt_regi;
#[doc = "UART_AT_CMD_GAPTOUT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_gaptout_regi](uart_at_cmd_gaptout_regi) module"]
pub type UART_AT_CMD_GAPTOUT_REGI = crate::Reg<u32, _UART_AT_CMD_GAPTOUT_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_GAPTOUT_REGI;
#[doc = "`read()` method returns [uart_at_cmd_gaptout_regi::R](uart_at_cmd_gaptout_regi::R) reader structure"]
impl crate::Readable for UART_AT_CMD_GAPTOUT_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_gaptout_regi::W](uart_at_cmd_gaptout_regi::W) writer structure"]
impl crate::Writable for UART_AT_CMD_GAPTOUT_REGI {}
#[doc = "UART_AT_CMD_GAPTOUT_REG(i)"]
pub mod uart_at_cmd_gaptout_regi;
#[doc = "UART_AT_CMD_CHAR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_char_regi](uart_at_cmd_char_regi) module"]
pub type UART_AT_CMD_CHAR_REGI = crate::Reg<u32, _UART_AT_CMD_CHAR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_CHAR_REGI;
#[doc = "`read()` method returns [uart_at_cmd_char_regi::R](uart_at_cmd_char_regi::R) reader structure"]
impl crate::Readable for UART_AT_CMD_CHAR_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_char_regi::W](uart_at_cmd_char_regi::W) writer structure"]
impl crate::Writable for UART_AT_CMD_CHAR_REGI {}
#[doc = "UART_AT_CMD_CHAR_REG(i)"]
pub mod uart_at_cmd_char_regi;
#[doc = "UART_MEM_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_conf_regi](uart_mem_conf_regi) module"]
pub type UART_MEM_CONF_REGI = crate::Reg<u32, _UART_MEM_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_CONF_REGI;
#[doc = "`read()` method returns [uart_mem_conf_regi::R](uart_mem_conf_regi::R) reader structure"]
impl crate::Readable for UART_MEM_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_mem_conf_regi::W](uart_mem_conf_regi::W) writer structure"]
impl crate::Writable for UART_MEM_CONF_REGI {}
#[doc = "UART_MEM_CONF_REG(i)"]
pub mod uart_mem_conf_regi;
#[doc = "UART_MEM_TX_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_tx_status_regi](uart_mem_tx_status_regi) module"]
pub type UART_MEM_TX_STATUS_REGI = crate::Reg<u32, _UART_MEM_TX_STATUS_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_TX_STATUS_REGI;
#[doc = "`read()` method returns [uart_mem_tx_status_regi::R](uart_mem_tx_status_regi::R) reader structure"]
impl crate::Readable for UART_MEM_TX_STATUS_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_mem_tx_status_regi::W](uart_mem_tx_status_regi::W) writer structure"]
impl crate::Writable for UART_MEM_TX_STATUS_REGI {}
#[doc = "UART_MEM_TX_STATUS_REG(i)"]
pub mod uart_mem_tx_status_regi;
#[doc = "UART_MEM_RX_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_rx_status_regi](uart_mem_rx_status_regi) module"]
pub type UART_MEM_RX_STATUS_REGI = crate::Reg<u32, _UART_MEM_RX_STATUS_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_RX_STATUS_REGI;
#[doc = "`read()` method returns [uart_mem_rx_status_regi::R](uart_mem_rx_status_regi::R) reader structure"]
impl crate::Readable for UART_MEM_RX_STATUS_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_mem_rx_status_regi::W](uart_mem_rx_status_regi::W) writer structure"]
impl crate::Writable for UART_MEM_RX_STATUS_REGI {}
#[doc = "UART_MEM_RX_STATUS_REG(i)"]
pub mod uart_mem_rx_status_regi;
#[doc = "UART_MEM_CNT_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_cnt_status_regi](uart_mem_cnt_status_regi) module"]
pub type UART_MEM_CNT_STATUS_REGI = crate::Reg<u32, _UART_MEM_CNT_STATUS_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_CNT_STATUS_REGI;
#[doc = "`read()` method returns [uart_mem_cnt_status_regi::R](uart_mem_cnt_status_regi::R) reader structure"]
impl crate::Readable for UART_MEM_CNT_STATUS_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_mem_cnt_status_regi::W](uart_mem_cnt_status_regi::W) writer structure"]
impl crate::Writable for UART_MEM_CNT_STATUS_REGI {}
#[doc = "UART_MEM_CNT_STATUS_REG(i)"]
pub mod uart_mem_cnt_status_regi;
#[doc = "UART_POSPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_pospulse_regi](uart_pospulse_regi) module"]
pub type UART_POSPULSE_REGI = crate::Reg<u32, _UART_POSPULSE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_POSPULSE_REGI;
#[doc = "`read()` method returns [uart_pospulse_regi::R](uart_pospulse_regi::R) reader structure"]
impl crate::Readable for UART_POSPULSE_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_pospulse_regi::W](uart_pospulse_regi::W) writer structure"]
impl crate::Writable for UART_POSPULSE_REGI {}
#[doc = "UART_POSPULSE_REG(i)"]
pub mod uart_pospulse_regi;
#[doc = "UART_NEGPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_negpulse_regi](uart_negpulse_regi) module"]
pub type UART_NEGPULSE_REGI = crate::Reg<u32, _UART_NEGPULSE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_NEGPULSE_REGI;
#[doc = "`read()` method returns [uart_negpulse_regi::R](uart_negpulse_regi::R) reader structure"]
impl crate::Readable for UART_NEGPULSE_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_negpulse_regi::W](uart_negpulse_regi::W) writer structure"]
impl crate::Writable for UART_NEGPULSE_REGI {}
#[doc = "UART_NEGPULSE_REG(i)"]
pub mod uart_negpulse_regi;
#[doc = "UART_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_date_regi](uart_date_regi) module"]
pub type UART_DATE_REGI = crate::Reg<u32, _UART_DATE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_DATE_REGI;
#[doc = "`read()` method returns [uart_date_regi::R](uart_date_regi::R) reader structure"]
impl crate::Readable for UART_DATE_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_date_regi::W](uart_date_regi::W) writer structure"]
impl crate::Writable for UART_DATE_REGI {}
#[doc = "UART_DATE_REG(i)"]
pub mod uart_date_regi;
#[doc = "UART_ID_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_id_regi](uart_id_regi) module"]
pub type UART_ID_REGI = crate::Reg<u32, _UART_ID_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_ID_REGI;
#[doc = "`read()` method returns [uart_id_regi::R](uart_id_regi::R) reader structure"]
impl crate::Readable for UART_ID_REGI {}
#[doc = "`write(|w| ..)` method takes [uart_id_regi::W](uart_id_regi::W) writer structure"]
impl crate::Writable for UART_ID_REGI {}
#[doc = "UART_ID_REG(i)"]
pub mod uart_id_regi;
