#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - UART_INT_RAW_REG(i)"]
    pub uart_int_raw_reg: UART_INT_RAW_REG,
    #[doc = "0x08 - UART_INT_ST_REG(i)"]
    pub uart_int_st_reg: UART_INT_ST_REG,
    #[doc = "0x0c - UART_INT_ENA_REG(i)"]
    pub uart_int_ena_reg: UART_INT_ENA_REG,
    #[doc = "0x10 - UART_INT_CLR_REG(i)"]
    pub uart_int_clr_reg: UART_INT_CLR_REG,
    #[doc = "0x14 - UART_CLKDIV_REG(i)"]
    pub uart_clkdiv_reg: UART_CLKDIV_REG,
    #[doc = "0x18 - UART_AUTOBAUD_REG(i)"]
    pub uart_autobaud_reg: UART_AUTOBAUD_REG,
    #[doc = "0x1c - UART_STATUS_REG(i)"]
    pub uart_status_reg: UART_STATUS_REG,
    #[doc = "0x20 - UART_CONF0_REG(i)"]
    pub uart_conf0_reg: UART_CONF0_REG,
    #[doc = "0x24 - UART_CONF1_REG(i)"]
    pub uart_conf1_reg: UART_CONF1_REG,
    #[doc = "0x28 - UART_LOWPULSE_REG(i)"]
    pub uart_lowpulse_reg: UART_LOWPULSE_REG,
    #[doc = "0x2c - UART_HIGHPULSE_REG(i)"]
    pub uart_highpulse_reg: UART_HIGHPULSE_REG,
    #[doc = "0x30 - UART_RXD_CNT_REG(i)"]
    pub uart_rxd_cnt_reg: UART_RXD_CNT_REG,
    #[doc = "0x34 - UART_FLOW_CONF_REG(i)"]
    pub uart_flow_conf_reg: UART_FLOW_CONF_REG,
    #[doc = "0x38 - UART_SLEEP_CONF_REG(i)"]
    pub uart_sleep_conf_reg: UART_SLEEP_CONF_REG,
    #[doc = "0x3c - UART_SWFC_CONF_REG(i)"]
    pub uart_swfc_conf_reg: UART_SWFC_CONF_REG,
    #[doc = "0x40 - UART_IDLE_CONF_REG(i)"]
    pub uart_idle_conf_reg: UART_IDLE_CONF_REG,
    #[doc = "0x44 - UART_RS485_CONF_REG(i)"]
    pub uart_rs485_conf_reg: UART_RS485_CONF_REG,
    #[doc = "0x48 - UART_AT_CMD_PRECNT_REG(i)"]
    pub uart_at_cmd_precnt_reg: UART_AT_CMD_PRECNT_REG,
    #[doc = "0x4c - UART_AT_CMD_POSTCNT_REG(i)"]
    pub uart_at_cmd_postcnt_reg: UART_AT_CMD_POSTCNT_REG,
    #[doc = "0x50 - UART_AT_CMD_GAPTOUT_REG(i)"]
    pub uart_at_cmd_gaptout_reg: UART_AT_CMD_GAPTOUT_REG,
    #[doc = "0x54 - UART_AT_CMD_CHAR_REG(i)"]
    pub uart_at_cmd_char_reg: UART_AT_CMD_CHAR_REG,
    #[doc = "0x58 - UART_MEM_CONF_REG(i)"]
    pub uart_mem_conf_reg: UART_MEM_CONF_REG,
    #[doc = "0x5c - UART_MEM_TX_STATUS_REG(i)"]
    pub uart_mem_tx_status_reg: UART_MEM_TX_STATUS_REG,
    #[doc = "0x60 - UART_MEM_RX_STATUS_REG(i)"]
    pub uart_mem_rx_status_reg: UART_MEM_RX_STATUS_REG,
    #[doc = "0x64 - UART_MEM_CNT_STATUS_REG(i)"]
    pub uart_mem_cnt_status_reg: UART_MEM_CNT_STATUS_REG,
    #[doc = "0x68 - UART_POSPULSE_REG(i)"]
    pub uart_pospulse_reg: UART_POSPULSE_REG,
    #[doc = "0x6c - UART_NEGPULSE_REG(i)"]
    pub uart_negpulse_reg: UART_NEGPULSE_REG,
    _reserved27: [u8; 8usize],
    #[doc = "0x78 - UART_DATE_REG(i)"]
    pub uart_date_reg: UART_DATE_REG,
    #[doc = "0x7c - UART_ID_REG(i)"]
    pub uart_id_reg: UART_ID_REG,
}
#[doc = "UART_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_raw_reg](uart_int_raw_reg) module"]
pub type UART_INT_RAW_REG = crate::Reg<u32, _UART_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_RAW_REG;
#[doc = "`read()` method returns [uart_int_raw_reg::R](uart_int_raw_reg::R) reader structure"]
impl crate::Readable for UART_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [uart_int_raw_reg::W](uart_int_raw_reg::W) writer structure"]
impl crate::Writable for UART_INT_RAW_REG {}
#[doc = "UART_INT_RAW_REG(i)"]
pub mod uart_int_raw_reg;
#[doc = "UART_INT_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_st_reg](uart_int_st_reg) module"]
pub type UART_INT_ST_REG = crate::Reg<u32, _UART_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_ST_REG;
#[doc = "`read()` method returns [uart_int_st_reg::R](uart_int_st_reg::R) reader structure"]
impl crate::Readable for UART_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [uart_int_st_reg::W](uart_int_st_reg::W) writer structure"]
impl crate::Writable for UART_INT_ST_REG {}
#[doc = "UART_INT_ST_REG(i)"]
pub mod uart_int_st_reg;
#[doc = "UART_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_ena_reg](uart_int_ena_reg) module"]
pub type UART_INT_ENA_REG = crate::Reg<u32, _UART_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_ENA_REG;
#[doc = "`read()` method returns [uart_int_ena_reg::R](uart_int_ena_reg::R) reader structure"]
impl crate::Readable for UART_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [uart_int_ena_reg::W](uart_int_ena_reg::W) writer structure"]
impl crate::Writable for UART_INT_ENA_REG {}
#[doc = "UART_INT_ENA_REG(i)"]
pub mod uart_int_ena_reg;
#[doc = "UART_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_int_clr_reg](uart_int_clr_reg) module"]
pub type UART_INT_CLR_REG = crate::Reg<u32, _UART_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_CLR_REG;
#[doc = "`read()` method returns [uart_int_clr_reg::R](uart_int_clr_reg::R) reader structure"]
impl crate::Readable for UART_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [uart_int_clr_reg::W](uart_int_clr_reg::W) writer structure"]
impl crate::Writable for UART_INT_CLR_REG {}
#[doc = "UART_INT_CLR_REG(i)"]
pub mod uart_int_clr_reg;
#[doc = "UART_CLKDIV_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_clkdiv_reg](uart_clkdiv_reg) module"]
pub type UART_CLKDIV_REG = crate::Reg<u32, _UART_CLKDIV_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CLKDIV_REG;
#[doc = "`read()` method returns [uart_clkdiv_reg::R](uart_clkdiv_reg::R) reader structure"]
impl crate::Readable for UART_CLKDIV_REG {}
#[doc = "`write(|w| ..)` method takes [uart_clkdiv_reg::W](uart_clkdiv_reg::W) writer structure"]
impl crate::Writable for UART_CLKDIV_REG {}
#[doc = "UART_CLKDIV_REG(i)"]
pub mod uart_clkdiv_reg;
#[doc = "UART_AUTOBAUD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_autobaud_reg](uart_autobaud_reg) module"]
pub type UART_AUTOBAUD_REG = crate::Reg<u32, _UART_AUTOBAUD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AUTOBAUD_REG;
#[doc = "`read()` method returns [uart_autobaud_reg::R](uart_autobaud_reg::R) reader structure"]
impl crate::Readable for UART_AUTOBAUD_REG {}
#[doc = "`write(|w| ..)` method takes [uart_autobaud_reg::W](uart_autobaud_reg::W) writer structure"]
impl crate::Writable for UART_AUTOBAUD_REG {}
#[doc = "UART_AUTOBAUD_REG(i)"]
pub mod uart_autobaud_reg;
#[doc = "UART_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_status_reg](uart_status_reg) module"]
pub type UART_STATUS_REG = crate::Reg<u32, _UART_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_STATUS_REG;
#[doc = "`read()` method returns [uart_status_reg::R](uart_status_reg::R) reader structure"]
impl crate::Readable for UART_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [uart_status_reg::W](uart_status_reg::W) writer structure"]
impl crate::Writable for UART_STATUS_REG {}
#[doc = "UART_STATUS_REG(i)"]
pub mod uart_status_reg;
#[doc = "UART_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_conf0_reg](uart_conf0_reg) module"]
pub type UART_CONF0_REG = crate::Reg<u32, _UART_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CONF0_REG;
#[doc = "`read()` method returns [uart_conf0_reg::R](uart_conf0_reg::R) reader structure"]
impl crate::Readable for UART_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [uart_conf0_reg::W](uart_conf0_reg::W) writer structure"]
impl crate::Writable for UART_CONF0_REG {}
#[doc = "UART_CONF0_REG(i)"]
pub mod uart_conf0_reg;
#[doc = "UART_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_conf1_reg](uart_conf1_reg) module"]
pub type UART_CONF1_REG = crate::Reg<u32, _UART_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CONF1_REG;
#[doc = "`read()` method returns [uart_conf1_reg::R](uart_conf1_reg::R) reader structure"]
impl crate::Readable for UART_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [uart_conf1_reg::W](uart_conf1_reg::W) writer structure"]
impl crate::Writable for UART_CONF1_REG {}
#[doc = "UART_CONF1_REG(i)"]
pub mod uart_conf1_reg;
#[doc = "UART_LOWPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_lowpulse_reg](uart_lowpulse_reg) module"]
pub type UART_LOWPULSE_REG = crate::Reg<u32, _UART_LOWPULSE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_LOWPULSE_REG;
#[doc = "`read()` method returns [uart_lowpulse_reg::R](uart_lowpulse_reg::R) reader structure"]
impl crate::Readable for UART_LOWPULSE_REG {}
#[doc = "`write(|w| ..)` method takes [uart_lowpulse_reg::W](uart_lowpulse_reg::W) writer structure"]
impl crate::Writable for UART_LOWPULSE_REG {}
#[doc = "UART_LOWPULSE_REG(i)"]
pub mod uart_lowpulse_reg;
#[doc = "UART_HIGHPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_highpulse_reg](uart_highpulse_reg) module"]
pub type UART_HIGHPULSE_REG = crate::Reg<u32, _UART_HIGHPULSE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_HIGHPULSE_REG;
#[doc = "`read()` method returns [uart_highpulse_reg::R](uart_highpulse_reg::R) reader structure"]
impl crate::Readable for UART_HIGHPULSE_REG {}
#[doc = "`write(|w| ..)` method takes [uart_highpulse_reg::W](uart_highpulse_reg::W) writer structure"]
impl crate::Writable for UART_HIGHPULSE_REG {}
#[doc = "UART_HIGHPULSE_REG(i)"]
pub mod uart_highpulse_reg;
#[doc = "UART_RXD_CNT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_rxd_cnt_reg](uart_rxd_cnt_reg) module"]
pub type UART_RXD_CNT_REG = crate::Reg<u32, _UART_RXD_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RXD_CNT_REG;
#[doc = "`read()` method returns [uart_rxd_cnt_reg::R](uart_rxd_cnt_reg::R) reader structure"]
impl crate::Readable for UART_RXD_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [uart_rxd_cnt_reg::W](uart_rxd_cnt_reg::W) writer structure"]
impl crate::Writable for UART_RXD_CNT_REG {}
#[doc = "UART_RXD_CNT_REG(i)"]
pub mod uart_rxd_cnt_reg;
#[doc = "UART_FLOW_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_flow_conf_reg](uart_flow_conf_reg) module"]
pub type UART_FLOW_CONF_REG = crate::Reg<u32, _UART_FLOW_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FLOW_CONF_REG;
#[doc = "`read()` method returns [uart_flow_conf_reg::R](uart_flow_conf_reg::R) reader structure"]
impl crate::Readable for UART_FLOW_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uart_flow_conf_reg::W](uart_flow_conf_reg::W) writer structure"]
impl crate::Writable for UART_FLOW_CONF_REG {}
#[doc = "UART_FLOW_CONF_REG(i)"]
pub mod uart_flow_conf_reg;
#[doc = "UART_SLEEP_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_sleep_conf_reg](uart_sleep_conf_reg) module"]
pub type UART_SLEEP_CONF_REG = crate::Reg<u32, _UART_SLEEP_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SLEEP_CONF_REG;
#[doc = "`read()` method returns [uart_sleep_conf_reg::R](uart_sleep_conf_reg::R) reader structure"]
impl crate::Readable for UART_SLEEP_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uart_sleep_conf_reg::W](uart_sleep_conf_reg::W) writer structure"]
impl crate::Writable for UART_SLEEP_CONF_REG {}
#[doc = "UART_SLEEP_CONF_REG(i)"]
pub mod uart_sleep_conf_reg;
#[doc = "UART_SWFC_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_swfc_conf_reg](uart_swfc_conf_reg) module"]
pub type UART_SWFC_CONF_REG = crate::Reg<u32, _UART_SWFC_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SWFC_CONF_REG;
#[doc = "`read()` method returns [uart_swfc_conf_reg::R](uart_swfc_conf_reg::R) reader structure"]
impl crate::Readable for UART_SWFC_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uart_swfc_conf_reg::W](uart_swfc_conf_reg::W) writer structure"]
impl crate::Writable for UART_SWFC_CONF_REG {}
#[doc = "UART_SWFC_CONF_REG(i)"]
pub mod uart_swfc_conf_reg;
#[doc = "UART_IDLE_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_idle_conf_reg](uart_idle_conf_reg) module"]
pub type UART_IDLE_CONF_REG = crate::Reg<u32, _UART_IDLE_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_IDLE_CONF_REG;
#[doc = "`read()` method returns [uart_idle_conf_reg::R](uart_idle_conf_reg::R) reader structure"]
impl crate::Readable for UART_IDLE_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uart_idle_conf_reg::W](uart_idle_conf_reg::W) writer structure"]
impl crate::Writable for UART_IDLE_CONF_REG {}
#[doc = "UART_IDLE_CONF_REG(i)"]
pub mod uart_idle_conf_reg;
#[doc = "UART_RS485_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_rs485_conf_reg](uart_rs485_conf_reg) module"]
pub type UART_RS485_CONF_REG = crate::Reg<u32, _UART_RS485_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RS485_CONF_REG;
#[doc = "`read()` method returns [uart_rs485_conf_reg::R](uart_rs485_conf_reg::R) reader structure"]
impl crate::Readable for UART_RS485_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uart_rs485_conf_reg::W](uart_rs485_conf_reg::W) writer structure"]
impl crate::Writable for UART_RS485_CONF_REG {}
#[doc = "UART_RS485_CONF_REG(i)"]
pub mod uart_rs485_conf_reg;
#[doc = "UART_AT_CMD_PRECNT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_precnt_reg](uart_at_cmd_precnt_reg) module"]
pub type UART_AT_CMD_PRECNT_REG = crate::Reg<u32, _UART_AT_CMD_PRECNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_PRECNT_REG;
#[doc = "`read()` method returns [uart_at_cmd_precnt_reg::R](uart_at_cmd_precnt_reg::R) reader structure"]
impl crate::Readable for UART_AT_CMD_PRECNT_REG {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_precnt_reg::W](uart_at_cmd_precnt_reg::W) writer structure"]
impl crate::Writable for UART_AT_CMD_PRECNT_REG {}
#[doc = "UART_AT_CMD_PRECNT_REG(i)"]
pub mod uart_at_cmd_precnt_reg;
#[doc = "UART_AT_CMD_POSTCNT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_postcnt_reg](uart_at_cmd_postcnt_reg) module"]
pub type UART_AT_CMD_POSTCNT_REG = crate::Reg<u32, _UART_AT_CMD_POSTCNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_POSTCNT_REG;
#[doc = "`read()` method returns [uart_at_cmd_postcnt_reg::R](uart_at_cmd_postcnt_reg::R) reader structure"]
impl crate::Readable for UART_AT_CMD_POSTCNT_REG {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_postcnt_reg::W](uart_at_cmd_postcnt_reg::W) writer structure"]
impl crate::Writable for UART_AT_CMD_POSTCNT_REG {}
#[doc = "UART_AT_CMD_POSTCNT_REG(i)"]
pub mod uart_at_cmd_postcnt_reg;
#[doc = "UART_AT_CMD_GAPTOUT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_gaptout_reg](uart_at_cmd_gaptout_reg) module"]
pub type UART_AT_CMD_GAPTOUT_REG = crate::Reg<u32, _UART_AT_CMD_GAPTOUT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_GAPTOUT_REG;
#[doc = "`read()` method returns [uart_at_cmd_gaptout_reg::R](uart_at_cmd_gaptout_reg::R) reader structure"]
impl crate::Readable for UART_AT_CMD_GAPTOUT_REG {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_gaptout_reg::W](uart_at_cmd_gaptout_reg::W) writer structure"]
impl crate::Writable for UART_AT_CMD_GAPTOUT_REG {}
#[doc = "UART_AT_CMD_GAPTOUT_REG(i)"]
pub mod uart_at_cmd_gaptout_reg;
#[doc = "UART_AT_CMD_CHAR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_at_cmd_char_reg](uart_at_cmd_char_reg) module"]
pub type UART_AT_CMD_CHAR_REG = crate::Reg<u32, _UART_AT_CMD_CHAR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_CHAR_REG;
#[doc = "`read()` method returns [uart_at_cmd_char_reg::R](uart_at_cmd_char_reg::R) reader structure"]
impl crate::Readable for UART_AT_CMD_CHAR_REG {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_char_reg::W](uart_at_cmd_char_reg::W) writer structure"]
impl crate::Writable for UART_AT_CMD_CHAR_REG {}
#[doc = "UART_AT_CMD_CHAR_REG(i)"]
pub mod uart_at_cmd_char_reg;
#[doc = "UART_MEM_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_conf_reg](uart_mem_conf_reg) module"]
pub type UART_MEM_CONF_REG = crate::Reg<u32, _UART_MEM_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_CONF_REG;
#[doc = "`read()` method returns [uart_mem_conf_reg::R](uart_mem_conf_reg::R) reader structure"]
impl crate::Readable for UART_MEM_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uart_mem_conf_reg::W](uart_mem_conf_reg::W) writer structure"]
impl crate::Writable for UART_MEM_CONF_REG {}
#[doc = "UART_MEM_CONF_REG(i)"]
pub mod uart_mem_conf_reg;
#[doc = "UART_MEM_TX_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_tx_status_reg](uart_mem_tx_status_reg) module"]
pub type UART_MEM_TX_STATUS_REG = crate::Reg<u32, _UART_MEM_TX_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_TX_STATUS_REG;
#[doc = "`read()` method returns [uart_mem_tx_status_reg::R](uart_mem_tx_status_reg::R) reader structure"]
impl crate::Readable for UART_MEM_TX_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [uart_mem_tx_status_reg::W](uart_mem_tx_status_reg::W) writer structure"]
impl crate::Writable for UART_MEM_TX_STATUS_REG {}
#[doc = "UART_MEM_TX_STATUS_REG(i)"]
pub mod uart_mem_tx_status_reg;
#[doc = "UART_MEM_RX_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_rx_status_reg](uart_mem_rx_status_reg) module"]
pub type UART_MEM_RX_STATUS_REG = crate::Reg<u32, _UART_MEM_RX_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_RX_STATUS_REG;
#[doc = "`read()` method returns [uart_mem_rx_status_reg::R](uart_mem_rx_status_reg::R) reader structure"]
impl crate::Readable for UART_MEM_RX_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [uart_mem_rx_status_reg::W](uart_mem_rx_status_reg::W) writer structure"]
impl crate::Writable for UART_MEM_RX_STATUS_REG {}
#[doc = "UART_MEM_RX_STATUS_REG(i)"]
pub mod uart_mem_rx_status_reg;
#[doc = "UART_MEM_CNT_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_mem_cnt_status_reg](uart_mem_cnt_status_reg) module"]
pub type UART_MEM_CNT_STATUS_REG = crate::Reg<u32, _UART_MEM_CNT_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_CNT_STATUS_REG;
#[doc = "`read()` method returns [uart_mem_cnt_status_reg::R](uart_mem_cnt_status_reg::R) reader structure"]
impl crate::Readable for UART_MEM_CNT_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [uart_mem_cnt_status_reg::W](uart_mem_cnt_status_reg::W) writer structure"]
impl crate::Writable for UART_MEM_CNT_STATUS_REG {}
#[doc = "UART_MEM_CNT_STATUS_REG(i)"]
pub mod uart_mem_cnt_status_reg;
#[doc = "UART_POSPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_pospulse_reg](uart_pospulse_reg) module"]
pub type UART_POSPULSE_REG = crate::Reg<u32, _UART_POSPULSE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_POSPULSE_REG;
#[doc = "`read()` method returns [uart_pospulse_reg::R](uart_pospulse_reg::R) reader structure"]
impl crate::Readable for UART_POSPULSE_REG {}
#[doc = "`write(|w| ..)` method takes [uart_pospulse_reg::W](uart_pospulse_reg::W) writer structure"]
impl crate::Writable for UART_POSPULSE_REG {}
#[doc = "UART_POSPULSE_REG(i)"]
pub mod uart_pospulse_reg;
#[doc = "UART_NEGPULSE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_negpulse_reg](uart_negpulse_reg) module"]
pub type UART_NEGPULSE_REG = crate::Reg<u32, _UART_NEGPULSE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_NEGPULSE_REG;
#[doc = "`read()` method returns [uart_negpulse_reg::R](uart_negpulse_reg::R) reader structure"]
impl crate::Readable for UART_NEGPULSE_REG {}
#[doc = "`write(|w| ..)` method takes [uart_negpulse_reg::W](uart_negpulse_reg::W) writer structure"]
impl crate::Writable for UART_NEGPULSE_REG {}
#[doc = "UART_NEGPULSE_REG(i)"]
pub mod uart_negpulse_reg;
#[doc = "UART_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_date_reg](uart_date_reg) module"]
pub type UART_DATE_REG = crate::Reg<u32, _UART_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_DATE_REG;
#[doc = "`read()` method returns [uart_date_reg::R](uart_date_reg::R) reader structure"]
impl crate::Readable for UART_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [uart_date_reg::W](uart_date_reg::W) writer structure"]
impl crate::Writable for UART_DATE_REG {}
#[doc = "UART_DATE_REG(i)"]
pub mod uart_date_reg;
#[doc = "UART_ID_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_id_reg](uart_id_reg) module"]
pub type UART_ID_REG = crate::Reg<u32, _UART_ID_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_ID_REG;
#[doc = "`read()` method returns [uart_id_reg::R](uart_id_reg::R) reader structure"]
impl crate::Readable for UART_ID_REG {}
#[doc = "`write(|w| ..)` method takes [uart_id_reg::W](uart_id_reg::W) writer structure"]
impl crate::Writable for UART_ID_REG {}
#[doc = "UART_ID_REG(i)"]
pub mod uart_id_reg;