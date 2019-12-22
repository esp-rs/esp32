#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UHCI_CONF0_REG(i)"]
    pub uhci_conf0_reg: UHCI_CONF0_REG,
    #[doc = "0x04 - UHCI_INT_RAW_REG(i)"]
    pub uhci_int_raw_reg: UHCI_INT_RAW_REG,
    #[doc = "0x08 - UHCI_INT_ST_REG(i)"]
    pub uhci_int_st_reg: UHCI_INT_ST_REG,
    #[doc = "0x0c - UHCI_INT_ENA_REG(i)"]
    pub uhci_int_ena_reg: UHCI_INT_ENA_REG,
    #[doc = "0x10 - UHCI_INT_CLR_REG(i)"]
    pub uhci_int_clr_reg: UHCI_INT_CLR_REG,
    #[doc = "0x14 - UHCI_DMA_OUT_STATUS_REG(i)"]
    pub uhci_dma_out_status_reg: UHCI_DMA_OUT_STATUS_REG,
    #[doc = "0x18 - UHCI_DMA_OUT_PUSH_REG(i)"]
    pub uhci_dma_out_push_reg: UHCI_DMA_OUT_PUSH_REG,
    #[doc = "0x1c - UHCI_DMA_IN_STATUS_REG(i)"]
    pub uhci_dma_in_status_reg: UHCI_DMA_IN_STATUS_REG,
    #[doc = "0x20 - UHCI_DMA_IN_POP_REG(i)"]
    pub uhci_dma_in_pop_reg: UHCI_DMA_IN_POP_REG,
    #[doc = "0x24 - UHCI_DMA_OUT_LINK_REG(i)"]
    pub uhci_dma_out_link_reg: UHCI_DMA_OUT_LINK_REG,
    #[doc = "0x28 - UHCI_DMA_IN_LINK_REG(i)"]
    pub uhci_dma_in_link_reg: UHCI_DMA_IN_LINK_REG,
    #[doc = "0x2c - UHCI_CONF1_REG(i)"]
    pub uhci_conf1_reg: UHCI_CONF1_REG,
    #[doc = "0x30 - UHCI_STATE0_REG(i)"]
    pub uhci_state0_reg: UHCI_STATE0_REG,
    #[doc = "0x34 - UHCI_STATE1_REG(i)"]
    pub uhci_state1_reg: UHCI_STATE1_REG,
    #[doc = "0x38 - UHCI_DMA_OUT_EOF_DES_ADDR_REG(i)"]
    pub uhci_dma_out_eof_des_addr_reg: UHCI_DMA_OUT_EOF_DES_ADDR_REG,
    #[doc = "0x3c - UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG(i)"]
    pub uhci_dma_in_suc_eof_des_addr_reg: UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG,
    #[doc = "0x40 - UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG(i)"]
    pub uhci_dma_in_err_eof_des_addr_reg: UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG,
    #[doc = "0x44 - UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG(i)"]
    pub uhci_dma_out_eof_bfr_des_addr_reg: UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG,
    #[doc = "0x48 - UHCI_AHB_TEST_REG(i)"]
    pub uhci_ahb_test_reg: UHCI_AHB_TEST_REG,
    #[doc = "0x4c - UHCI_DMA_IN_DSCR_REG(i)"]
    pub uhci_dma_in_dscr_reg: UHCI_DMA_IN_DSCR_REG,
    #[doc = "0x50 - UHCI_DMA_IN_DSCR_BF0_REG(i)"]
    pub uhci_dma_in_dscr_bf0_reg: UHCI_DMA_IN_DSCR_BF0_REG,
    #[doc = "0x54 - UHCI_DMA_IN_DSCR_BF1_REG(i)"]
    pub uhci_dma_in_dscr_bf1_reg: UHCI_DMA_IN_DSCR_BF1_REG,
    #[doc = "0x58 - UHCI_DMA_OUT_DSCR_REG(i)"]
    pub uhci_dma_out_dscr_reg: UHCI_DMA_OUT_DSCR_REG,
    #[doc = "0x5c - UHCI_DMA_OUT_DSCR_BF0_REG(i)"]
    pub uhci_dma_out_dscr_bf0_reg: UHCI_DMA_OUT_DSCR_BF0_REG,
    #[doc = "0x60 - UHCI_DMA_OUT_DSCR_BF1_REG(i)"]
    pub uhci_dma_out_dscr_bf1_reg: UHCI_DMA_OUT_DSCR_BF1_REG,
    #[doc = "0x64 - UHCI_ESCAPE_CONF_REG(i)"]
    pub uhci_escape_conf_reg: UHCI_ESCAPE_CONF_REG,
    #[doc = "0x68 - UHCI_HUNG_CONF_REG(i)"]
    pub uhci_hung_conf_reg: UHCI_HUNG_CONF_REG,
    _reserved27: [u8; 4usize],
    #[doc = "0x70 - UHCI_RX_HEAD_REG(i)"]
    pub uhci_rx_head_reg: UHCI_RX_HEAD_REG,
    #[doc = "0x74 - UHCI_QUICK_SENT_REG(i)"]
    pub uhci_quick_sent_reg: UHCI_QUICK_SENT_REG,
    #[doc = "0x78 - UHCI_Q0_WORD0_REG(i)"]
    pub uhci_q0_word0_reg: UHCI_Q0_WORD0_REG,
    #[doc = "0x7c - UHCI_Q0_WORD1_REG(i)"]
    pub uhci_q0_word1_reg: UHCI_Q0_WORD1_REG,
    #[doc = "0x80 - UHCI_Q1_WORD0_REG(i)"]
    pub uhci_q1_word0_reg: UHCI_Q1_WORD0_REG,
    #[doc = "0x84 - UHCI_Q1_WORD1_REG(i)"]
    pub uhci_q1_word1_reg: UHCI_Q1_WORD1_REG,
    #[doc = "0x88 - UHCI_Q2_WORD0_REG(i)"]
    pub uhci_q2_word0_reg: UHCI_Q2_WORD0_REG,
    #[doc = "0x8c - UHCI_Q2_WORD1_REG(i)"]
    pub uhci_q2_word1_reg: UHCI_Q2_WORD1_REG,
    #[doc = "0x90 - UHCI_Q3_WORD0_REG(i)"]
    pub uhci_q3_word0_reg: UHCI_Q3_WORD0_REG,
    #[doc = "0x94 - UHCI_Q3_WORD1_REG(i)"]
    pub uhci_q3_word1_reg: UHCI_Q3_WORD1_REG,
    #[doc = "0x98 - UHCI_Q4_WORD0_REG(i)"]
    pub uhci_q4_word0_reg: UHCI_Q4_WORD0_REG,
    #[doc = "0x9c - UHCI_Q4_WORD1_REG(i)"]
    pub uhci_q4_word1_reg: UHCI_Q4_WORD1_REG,
    #[doc = "0xa0 - UHCI_Q5_WORD0_REG(i)"]
    pub uhci_q5_word0_reg: UHCI_Q5_WORD0_REG,
    #[doc = "0xa4 - UHCI_Q5_WORD1_REG(i)"]
    pub uhci_q5_word1_reg: UHCI_Q5_WORD1_REG,
    #[doc = "0xa8 - UHCI_Q6_WORD0_REG(i)"]
    pub uhci_q6_word0_reg: UHCI_Q6_WORD0_REG,
    #[doc = "0xac - UHCI_Q6_WORD1_REG(i)"]
    pub uhci_q6_word1_reg: UHCI_Q6_WORD1_REG,
    #[doc = "0xb0 - UHCI_ESC_CONF0_REG(i)"]
    pub uhci_esc_conf0_reg: UHCI_ESC_CONF0_REG,
    #[doc = "0xb4 - UHCI_ESC_CONF1_REG(i)"]
    pub uhci_esc_conf1_reg: UHCI_ESC_CONF1_REG,
    #[doc = "0xb8 - UHCI_ESC_CONF2_REG(i)"]
    pub uhci_esc_conf2_reg: UHCI_ESC_CONF2_REG,
    #[doc = "0xbc - UHCI_ESC_CONF3_REG(i)"]
    pub uhci_esc_conf3_reg: UHCI_ESC_CONF3_REG,
    #[doc = "0xc0 - UHCI_PKT_THRES_REG(i)"]
    pub uhci_pkt_thres_reg: UHCI_PKT_THRES_REG,
    _reserved48: [u8; 56usize],
    #[doc = "0xfc - UHCI_DATE_REG(i)"]
    pub uhci_date_reg: UHCI_DATE_REG,
}
#[doc = "UHCI_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_conf0_reg](uhci_conf0_reg) module"]
pub type UHCI_CONF0_REG = crate::Reg<u32, _UHCI_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_CONF0_REG;
#[doc = "`read()` method returns [uhci_conf0_reg::R](uhci_conf0_reg::R) reader structure"]
impl crate::Readable for UHCI_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_conf0_reg::W](uhci_conf0_reg::W) writer structure"]
impl crate::Writable for UHCI_CONF0_REG {}
#[doc = "UHCI_CONF0_REG(i)"]
pub mod uhci_conf0_reg;
#[doc = "UHCI_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_int_raw_reg](uhci_int_raw_reg) module"]
pub type UHCI_INT_RAW_REG = crate::Reg<u32, _UHCI_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_RAW_REG;
#[doc = "`read()` method returns [uhci_int_raw_reg::R](uhci_int_raw_reg::R) reader structure"]
impl crate::Readable for UHCI_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_int_raw_reg::W](uhci_int_raw_reg::W) writer structure"]
impl crate::Writable for UHCI_INT_RAW_REG {}
#[doc = "UHCI_INT_RAW_REG(i)"]
pub mod uhci_int_raw_reg;
#[doc = "UHCI_INT_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_int_st_reg](uhci_int_st_reg) module"]
pub type UHCI_INT_ST_REG = crate::Reg<u32, _UHCI_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_ST_REG;
#[doc = "`read()` method returns [uhci_int_st_reg::R](uhci_int_st_reg::R) reader structure"]
impl crate::Readable for UHCI_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_int_st_reg::W](uhci_int_st_reg::W) writer structure"]
impl crate::Writable for UHCI_INT_ST_REG {}
#[doc = "UHCI_INT_ST_REG(i)"]
pub mod uhci_int_st_reg;
#[doc = "UHCI_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_int_ena_reg](uhci_int_ena_reg) module"]
pub type UHCI_INT_ENA_REG = crate::Reg<u32, _UHCI_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_ENA_REG;
#[doc = "`read()` method returns [uhci_int_ena_reg::R](uhci_int_ena_reg::R) reader structure"]
impl crate::Readable for UHCI_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_int_ena_reg::W](uhci_int_ena_reg::W) writer structure"]
impl crate::Writable for UHCI_INT_ENA_REG {}
#[doc = "UHCI_INT_ENA_REG(i)"]
pub mod uhci_int_ena_reg;
#[doc = "UHCI_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_int_clr_reg](uhci_int_clr_reg) module"]
pub type UHCI_INT_CLR_REG = crate::Reg<u32, _UHCI_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_CLR_REG;
#[doc = "`read()` method returns [uhci_int_clr_reg::R](uhci_int_clr_reg::R) reader structure"]
impl crate::Readable for UHCI_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_int_clr_reg::W](uhci_int_clr_reg::W) writer structure"]
impl crate::Writable for UHCI_INT_CLR_REG {}
#[doc = "UHCI_INT_CLR_REG(i)"]
pub mod uhci_int_clr_reg;
#[doc = "UHCI_DMA_OUT_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_status_reg](uhci_dma_out_status_reg) module"]
pub type UHCI_DMA_OUT_STATUS_REG = crate::Reg<u32, _UHCI_DMA_OUT_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_STATUS_REG;
#[doc = "`read()` method returns [uhci_dma_out_status_reg::R](uhci_dma_out_status_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_status_reg::W](uhci_dma_out_status_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_STATUS_REG {}
#[doc = "UHCI_DMA_OUT_STATUS_REG(i)"]
pub mod uhci_dma_out_status_reg;
#[doc = "UHCI_DMA_OUT_PUSH_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_push_reg](uhci_dma_out_push_reg) module"]
pub type UHCI_DMA_OUT_PUSH_REG = crate::Reg<u32, _UHCI_DMA_OUT_PUSH_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_PUSH_REG;
#[doc = "`read()` method returns [uhci_dma_out_push_reg::R](uhci_dma_out_push_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_PUSH_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_push_reg::W](uhci_dma_out_push_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_PUSH_REG {}
#[doc = "UHCI_DMA_OUT_PUSH_REG(i)"]
pub mod uhci_dma_out_push_reg;
#[doc = "UHCI_DMA_IN_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_status_reg](uhci_dma_in_status_reg) module"]
pub type UHCI_DMA_IN_STATUS_REG = crate::Reg<u32, _UHCI_DMA_IN_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_STATUS_REG;
#[doc = "`read()` method returns [uhci_dma_in_status_reg::R](uhci_dma_in_status_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_status_reg::W](uhci_dma_in_status_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_STATUS_REG {}
#[doc = "UHCI_DMA_IN_STATUS_REG(i)"]
pub mod uhci_dma_in_status_reg;
#[doc = "UHCI_DMA_IN_POP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_pop_reg](uhci_dma_in_pop_reg) module"]
pub type UHCI_DMA_IN_POP_REG = crate::Reg<u32, _UHCI_DMA_IN_POP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_POP_REG;
#[doc = "`read()` method returns [uhci_dma_in_pop_reg::R](uhci_dma_in_pop_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_POP_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_pop_reg::W](uhci_dma_in_pop_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_POP_REG {}
#[doc = "UHCI_DMA_IN_POP_REG(i)"]
pub mod uhci_dma_in_pop_reg;
#[doc = "UHCI_DMA_OUT_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_link_reg](uhci_dma_out_link_reg) module"]
pub type UHCI_DMA_OUT_LINK_REG = crate::Reg<u32, _UHCI_DMA_OUT_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_LINK_REG;
#[doc = "`read()` method returns [uhci_dma_out_link_reg::R](uhci_dma_out_link_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_link_reg::W](uhci_dma_out_link_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_LINK_REG {}
#[doc = "UHCI_DMA_OUT_LINK_REG(i)"]
pub mod uhci_dma_out_link_reg;
#[doc = "UHCI_DMA_IN_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_link_reg](uhci_dma_in_link_reg) module"]
pub type UHCI_DMA_IN_LINK_REG = crate::Reg<u32, _UHCI_DMA_IN_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_LINK_REG;
#[doc = "`read()` method returns [uhci_dma_in_link_reg::R](uhci_dma_in_link_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_link_reg::W](uhci_dma_in_link_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_LINK_REG {}
#[doc = "UHCI_DMA_IN_LINK_REG(i)"]
pub mod uhci_dma_in_link_reg;
#[doc = "UHCI_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_conf1_reg](uhci_conf1_reg) module"]
pub type UHCI_CONF1_REG = crate::Reg<u32, _UHCI_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_CONF1_REG;
#[doc = "`read()` method returns [uhci_conf1_reg::R](uhci_conf1_reg::R) reader structure"]
impl crate::Readable for UHCI_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_conf1_reg::W](uhci_conf1_reg::W) writer structure"]
impl crate::Writable for UHCI_CONF1_REG {}
#[doc = "UHCI_CONF1_REG(i)"]
pub mod uhci_conf1_reg;
#[doc = "UHCI_STATE0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_state0_reg](uhci_state0_reg) module"]
pub type UHCI_STATE0_REG = crate::Reg<u32, _UHCI_STATE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_STATE0_REG;
#[doc = "`read()` method returns [uhci_state0_reg::R](uhci_state0_reg::R) reader structure"]
impl crate::Readable for UHCI_STATE0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_state0_reg::W](uhci_state0_reg::W) writer structure"]
impl crate::Writable for UHCI_STATE0_REG {}
#[doc = "UHCI_STATE0_REG(i)"]
pub mod uhci_state0_reg;
#[doc = "UHCI_STATE1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_state1_reg](uhci_state1_reg) module"]
pub type UHCI_STATE1_REG = crate::Reg<u32, _UHCI_STATE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_STATE1_REG;
#[doc = "`read()` method returns [uhci_state1_reg::R](uhci_state1_reg::R) reader structure"]
impl crate::Readable for UHCI_STATE1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_state1_reg::W](uhci_state1_reg::W) writer structure"]
impl crate::Writable for UHCI_STATE1_REG {}
#[doc = "UHCI_STATE1_REG(i)"]
pub mod uhci_state1_reg;
#[doc = "UHCI_DMA_OUT_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_eof_des_addr_reg](uhci_dma_out_eof_des_addr_reg) module"]
pub type UHCI_DMA_OUT_EOF_DES_ADDR_REG = crate::Reg<u32, _UHCI_DMA_OUT_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [uhci_dma_out_eof_des_addr_reg::R](uhci_dma_out_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_eof_des_addr_reg::W](uhci_dma_out_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_EOF_DES_ADDR_REG {}
#[doc = "UHCI_DMA_OUT_EOF_DES_ADDR_REG(i)"]
pub mod uhci_dma_out_eof_des_addr_reg;
#[doc = "UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_suc_eof_des_addr_reg](uhci_dma_in_suc_eof_des_addr_reg) module"]
pub type UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG = crate::Reg<u32, _UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [uhci_dma_in_suc_eof_des_addr_reg::R](uhci_dma_in_suc_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_suc_eof_des_addr_reg::W](uhci_dma_in_suc_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG {}
#[doc = "UHCI_DMA_IN_SUC_EOF_DES_ADDR_REG(i)"]
pub mod uhci_dma_in_suc_eof_des_addr_reg;
#[doc = "UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_err_eof_des_addr_reg](uhci_dma_in_err_eof_des_addr_reg) module"]
pub type UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG = crate::Reg<u32, _UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [uhci_dma_in_err_eof_des_addr_reg::R](uhci_dma_in_err_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_err_eof_des_addr_reg::W](uhci_dma_in_err_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG {}
#[doc = "UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG(i)"]
pub mod uhci_dma_in_err_eof_des_addr_reg;
#[doc = "UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_eof_bfr_des_addr_reg](uhci_dma_out_eof_bfr_des_addr_reg) module"]
pub type UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG = crate::Reg<u32, _UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG;
#[doc = "`read()` method returns [uhci_dma_out_eof_bfr_des_addr_reg::R](uhci_dma_out_eof_bfr_des_addr_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_eof_bfr_des_addr_reg::W](uhci_dma_out_eof_bfr_des_addr_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG {}
#[doc = "UHCI_DMA_OUT_EOF_BFR_DES_ADDR_REG(i)"]
pub mod uhci_dma_out_eof_bfr_des_addr_reg;
#[doc = "UHCI_AHB_TEST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_ahb_test_reg](uhci_ahb_test_reg) module"]
pub type UHCI_AHB_TEST_REG = crate::Reg<u32, _UHCI_AHB_TEST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_AHB_TEST_REG;
#[doc = "`read()` method returns [uhci_ahb_test_reg::R](uhci_ahb_test_reg::R) reader structure"]
impl crate::Readable for UHCI_AHB_TEST_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_ahb_test_reg::W](uhci_ahb_test_reg::W) writer structure"]
impl crate::Writable for UHCI_AHB_TEST_REG {}
#[doc = "UHCI_AHB_TEST_REG(i)"]
pub mod uhci_ahb_test_reg;
#[doc = "UHCI_DMA_IN_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_dscr_reg](uhci_dma_in_dscr_reg) module"]
pub type UHCI_DMA_IN_DSCR_REG = crate::Reg<u32, _UHCI_DMA_IN_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_DSCR_REG;
#[doc = "`read()` method returns [uhci_dma_in_dscr_reg::R](uhci_dma_in_dscr_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_dscr_reg::W](uhci_dma_in_dscr_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_DSCR_REG {}
#[doc = "UHCI_DMA_IN_DSCR_REG(i)"]
pub mod uhci_dma_in_dscr_reg;
#[doc = "UHCI_DMA_IN_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_dscr_bf0_reg](uhci_dma_in_dscr_bf0_reg) module"]
pub type UHCI_DMA_IN_DSCR_BF0_REG = crate::Reg<u32, _UHCI_DMA_IN_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_DSCR_BF0_REG;
#[doc = "`read()` method returns [uhci_dma_in_dscr_bf0_reg::R](uhci_dma_in_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_dscr_bf0_reg::W](uhci_dma_in_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_DSCR_BF0_REG {}
#[doc = "UHCI_DMA_IN_DSCR_BF0_REG(i)"]
pub mod uhci_dma_in_dscr_bf0_reg;
#[doc = "UHCI_DMA_IN_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_in_dscr_bf1_reg](uhci_dma_in_dscr_bf1_reg) module"]
pub type UHCI_DMA_IN_DSCR_BF1_REG = crate::Reg<u32, _UHCI_DMA_IN_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_IN_DSCR_BF1_REG;
#[doc = "`read()` method returns [uhci_dma_in_dscr_bf1_reg::R](uhci_dma_in_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_IN_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_in_dscr_bf1_reg::W](uhci_dma_in_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_IN_DSCR_BF1_REG {}
#[doc = "UHCI_DMA_IN_DSCR_BF1_REG(i)"]
pub mod uhci_dma_in_dscr_bf1_reg;
#[doc = "UHCI_DMA_OUT_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_dscr_reg](uhci_dma_out_dscr_reg) module"]
pub type UHCI_DMA_OUT_DSCR_REG = crate::Reg<u32, _UHCI_DMA_OUT_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_DSCR_REG;
#[doc = "`read()` method returns [uhci_dma_out_dscr_reg::R](uhci_dma_out_dscr_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_dscr_reg::W](uhci_dma_out_dscr_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_DSCR_REG {}
#[doc = "UHCI_DMA_OUT_DSCR_REG(i)"]
pub mod uhci_dma_out_dscr_reg;
#[doc = "UHCI_DMA_OUT_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_dscr_bf0_reg](uhci_dma_out_dscr_bf0_reg) module"]
pub type UHCI_DMA_OUT_DSCR_BF0_REG = crate::Reg<u32, _UHCI_DMA_OUT_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_DSCR_BF0_REG;
#[doc = "`read()` method returns [uhci_dma_out_dscr_bf0_reg::R](uhci_dma_out_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_dscr_bf0_reg::W](uhci_dma_out_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_DSCR_BF0_REG {}
#[doc = "UHCI_DMA_OUT_DSCR_BF0_REG(i)"]
pub mod uhci_dma_out_dscr_bf0_reg;
#[doc = "UHCI_DMA_OUT_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_dma_out_dscr_bf1_reg](uhci_dma_out_dscr_bf1_reg) module"]
pub type UHCI_DMA_OUT_DSCR_BF1_REG = crate::Reg<u32, _UHCI_DMA_OUT_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DMA_OUT_DSCR_BF1_REG;
#[doc = "`read()` method returns [uhci_dma_out_dscr_bf1_reg::R](uhci_dma_out_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for UHCI_DMA_OUT_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_dma_out_dscr_bf1_reg::W](uhci_dma_out_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for UHCI_DMA_OUT_DSCR_BF1_REG {}
#[doc = "UHCI_DMA_OUT_DSCR_BF1_REG(i)"]
pub mod uhci_dma_out_dscr_bf1_reg;
#[doc = "UHCI_ESCAPE_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_escape_conf_reg](uhci_escape_conf_reg) module"]
pub type UHCI_ESCAPE_CONF_REG = crate::Reg<u32, _UHCI_ESCAPE_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESCAPE_CONF_REG;
#[doc = "`read()` method returns [uhci_escape_conf_reg::R](uhci_escape_conf_reg::R) reader structure"]
impl crate::Readable for UHCI_ESCAPE_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_escape_conf_reg::W](uhci_escape_conf_reg::W) writer structure"]
impl crate::Writable for UHCI_ESCAPE_CONF_REG {}
#[doc = "UHCI_ESCAPE_CONF_REG(i)"]
pub mod uhci_escape_conf_reg;
#[doc = "UHCI_HUNG_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_hung_conf_reg](uhci_hung_conf_reg) module"]
pub type UHCI_HUNG_CONF_REG = crate::Reg<u32, _UHCI_HUNG_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_HUNG_CONF_REG;
#[doc = "`read()` method returns [uhci_hung_conf_reg::R](uhci_hung_conf_reg::R) reader structure"]
impl crate::Readable for UHCI_HUNG_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_hung_conf_reg::W](uhci_hung_conf_reg::W) writer structure"]
impl crate::Writable for UHCI_HUNG_CONF_REG {}
#[doc = "UHCI_HUNG_CONF_REG(i)"]
pub mod uhci_hung_conf_reg;
#[doc = "UHCI_RX_HEAD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_rx_head_reg](uhci_rx_head_reg) module"]
pub type UHCI_RX_HEAD_REG = crate::Reg<u32, _UHCI_RX_HEAD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_RX_HEAD_REG;
#[doc = "`read()` method returns [uhci_rx_head_reg::R](uhci_rx_head_reg::R) reader structure"]
impl crate::Readable for UHCI_RX_HEAD_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_rx_head_reg::W](uhci_rx_head_reg::W) writer structure"]
impl crate::Writable for UHCI_RX_HEAD_REG {}
#[doc = "UHCI_RX_HEAD_REG(i)"]
pub mod uhci_rx_head_reg;
#[doc = "UHCI_QUICK_SENT_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_quick_sent_reg](uhci_quick_sent_reg) module"]
pub type UHCI_QUICK_SENT_REG = crate::Reg<u32, _UHCI_QUICK_SENT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_QUICK_SENT_REG;
#[doc = "`read()` method returns [uhci_quick_sent_reg::R](uhci_quick_sent_reg::R) reader structure"]
impl crate::Readable for UHCI_QUICK_SENT_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_quick_sent_reg::W](uhci_quick_sent_reg::W) writer structure"]
impl crate::Writable for UHCI_QUICK_SENT_REG {}
#[doc = "UHCI_QUICK_SENT_REG(i)"]
pub mod uhci_quick_sent_reg;
#[doc = "UHCI_Q0_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q0_word0_reg](uhci_q0_word0_reg) module"]
pub type UHCI_Q0_WORD0_REG = crate::Reg<u32, _UHCI_Q0_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q0_WORD0_REG;
#[doc = "`read()` method returns [uhci_q0_word0_reg::R](uhci_q0_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q0_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q0_word0_reg::W](uhci_q0_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q0_WORD0_REG {}
#[doc = "UHCI_Q0_WORD0_REG(i)"]
pub mod uhci_q0_word0_reg;
#[doc = "UHCI_Q0_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q0_word1_reg](uhci_q0_word1_reg) module"]
pub type UHCI_Q0_WORD1_REG = crate::Reg<u32, _UHCI_Q0_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q0_WORD1_REG;
#[doc = "`read()` method returns [uhci_q0_word1_reg::R](uhci_q0_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q0_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q0_word1_reg::W](uhci_q0_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q0_WORD1_REG {}
#[doc = "UHCI_Q0_WORD1_REG(i)"]
pub mod uhci_q0_word1_reg;
#[doc = "UHCI_Q1_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q1_word0_reg](uhci_q1_word0_reg) module"]
pub type UHCI_Q1_WORD0_REG = crate::Reg<u32, _UHCI_Q1_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q1_WORD0_REG;
#[doc = "`read()` method returns [uhci_q1_word0_reg::R](uhci_q1_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q1_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q1_word0_reg::W](uhci_q1_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q1_WORD0_REG {}
#[doc = "UHCI_Q1_WORD0_REG(i)"]
pub mod uhci_q1_word0_reg;
#[doc = "UHCI_Q1_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q1_word1_reg](uhci_q1_word1_reg) module"]
pub type UHCI_Q1_WORD1_REG = crate::Reg<u32, _UHCI_Q1_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q1_WORD1_REG;
#[doc = "`read()` method returns [uhci_q1_word1_reg::R](uhci_q1_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q1_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q1_word1_reg::W](uhci_q1_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q1_WORD1_REG {}
#[doc = "UHCI_Q1_WORD1_REG(i)"]
pub mod uhci_q1_word1_reg;
#[doc = "UHCI_Q2_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q2_word0_reg](uhci_q2_word0_reg) module"]
pub type UHCI_Q2_WORD0_REG = crate::Reg<u32, _UHCI_Q2_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q2_WORD0_REG;
#[doc = "`read()` method returns [uhci_q2_word0_reg::R](uhci_q2_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q2_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q2_word0_reg::W](uhci_q2_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q2_WORD0_REG {}
#[doc = "UHCI_Q2_WORD0_REG(i)"]
pub mod uhci_q2_word0_reg;
#[doc = "UHCI_Q2_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q2_word1_reg](uhci_q2_word1_reg) module"]
pub type UHCI_Q2_WORD1_REG = crate::Reg<u32, _UHCI_Q2_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q2_WORD1_REG;
#[doc = "`read()` method returns [uhci_q2_word1_reg::R](uhci_q2_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q2_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q2_word1_reg::W](uhci_q2_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q2_WORD1_REG {}
#[doc = "UHCI_Q2_WORD1_REG(i)"]
pub mod uhci_q2_word1_reg;
#[doc = "UHCI_Q3_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q3_word0_reg](uhci_q3_word0_reg) module"]
pub type UHCI_Q3_WORD0_REG = crate::Reg<u32, _UHCI_Q3_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q3_WORD0_REG;
#[doc = "`read()` method returns [uhci_q3_word0_reg::R](uhci_q3_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q3_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q3_word0_reg::W](uhci_q3_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q3_WORD0_REG {}
#[doc = "UHCI_Q3_WORD0_REG(i)"]
pub mod uhci_q3_word0_reg;
#[doc = "UHCI_Q3_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q3_word1_reg](uhci_q3_word1_reg) module"]
pub type UHCI_Q3_WORD1_REG = crate::Reg<u32, _UHCI_Q3_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q3_WORD1_REG;
#[doc = "`read()` method returns [uhci_q3_word1_reg::R](uhci_q3_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q3_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q3_word1_reg::W](uhci_q3_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q3_WORD1_REG {}
#[doc = "UHCI_Q3_WORD1_REG(i)"]
pub mod uhci_q3_word1_reg;
#[doc = "UHCI_Q4_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q4_word0_reg](uhci_q4_word0_reg) module"]
pub type UHCI_Q4_WORD0_REG = crate::Reg<u32, _UHCI_Q4_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q4_WORD0_REG;
#[doc = "`read()` method returns [uhci_q4_word0_reg::R](uhci_q4_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q4_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q4_word0_reg::W](uhci_q4_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q4_WORD0_REG {}
#[doc = "UHCI_Q4_WORD0_REG(i)"]
pub mod uhci_q4_word0_reg;
#[doc = "UHCI_Q4_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q4_word1_reg](uhci_q4_word1_reg) module"]
pub type UHCI_Q4_WORD1_REG = crate::Reg<u32, _UHCI_Q4_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q4_WORD1_REG;
#[doc = "`read()` method returns [uhci_q4_word1_reg::R](uhci_q4_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q4_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q4_word1_reg::W](uhci_q4_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q4_WORD1_REG {}
#[doc = "UHCI_Q4_WORD1_REG(i)"]
pub mod uhci_q4_word1_reg;
#[doc = "UHCI_Q5_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q5_word0_reg](uhci_q5_word0_reg) module"]
pub type UHCI_Q5_WORD0_REG = crate::Reg<u32, _UHCI_Q5_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q5_WORD0_REG;
#[doc = "`read()` method returns [uhci_q5_word0_reg::R](uhci_q5_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q5_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q5_word0_reg::W](uhci_q5_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q5_WORD0_REG {}
#[doc = "UHCI_Q5_WORD0_REG(i)"]
pub mod uhci_q5_word0_reg;
#[doc = "UHCI_Q5_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q5_word1_reg](uhci_q5_word1_reg) module"]
pub type UHCI_Q5_WORD1_REG = crate::Reg<u32, _UHCI_Q5_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q5_WORD1_REG;
#[doc = "`read()` method returns [uhci_q5_word1_reg::R](uhci_q5_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q5_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q5_word1_reg::W](uhci_q5_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q5_WORD1_REG {}
#[doc = "UHCI_Q5_WORD1_REG(i)"]
pub mod uhci_q5_word1_reg;
#[doc = "UHCI_Q6_WORD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q6_word0_reg](uhci_q6_word0_reg) module"]
pub type UHCI_Q6_WORD0_REG = crate::Reg<u32, _UHCI_Q6_WORD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q6_WORD0_REG;
#[doc = "`read()` method returns [uhci_q6_word0_reg::R](uhci_q6_word0_reg::R) reader structure"]
impl crate::Readable for UHCI_Q6_WORD0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q6_word0_reg::W](uhci_q6_word0_reg::W) writer structure"]
impl crate::Writable for UHCI_Q6_WORD0_REG {}
#[doc = "UHCI_Q6_WORD0_REG(i)"]
pub mod uhci_q6_word0_reg;
#[doc = "UHCI_Q6_WORD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_q6_word1_reg](uhci_q6_word1_reg) module"]
pub type UHCI_Q6_WORD1_REG = crate::Reg<u32, _UHCI_Q6_WORD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q6_WORD1_REG;
#[doc = "`read()` method returns [uhci_q6_word1_reg::R](uhci_q6_word1_reg::R) reader structure"]
impl crate::Readable for UHCI_Q6_WORD1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_q6_word1_reg::W](uhci_q6_word1_reg::W) writer structure"]
impl crate::Writable for UHCI_Q6_WORD1_REG {}
#[doc = "UHCI_Q6_WORD1_REG(i)"]
pub mod uhci_q6_word1_reg;
#[doc = "UHCI_ESC_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_esc_conf0_reg](uhci_esc_conf0_reg) module"]
pub type UHCI_ESC_CONF0_REG = crate::Reg<u32, _UHCI_ESC_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF0_REG;
#[doc = "`read()` method returns [uhci_esc_conf0_reg::R](uhci_esc_conf0_reg::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf0_reg::W](uhci_esc_conf0_reg::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF0_REG {}
#[doc = "UHCI_ESC_CONF0_REG(i)"]
pub mod uhci_esc_conf0_reg;
#[doc = "UHCI_ESC_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_esc_conf1_reg](uhci_esc_conf1_reg) module"]
pub type UHCI_ESC_CONF1_REG = crate::Reg<u32, _UHCI_ESC_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF1_REG;
#[doc = "`read()` method returns [uhci_esc_conf1_reg::R](uhci_esc_conf1_reg::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf1_reg::W](uhci_esc_conf1_reg::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF1_REG {}
#[doc = "UHCI_ESC_CONF1_REG(i)"]
pub mod uhci_esc_conf1_reg;
#[doc = "UHCI_ESC_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_esc_conf2_reg](uhci_esc_conf2_reg) module"]
pub type UHCI_ESC_CONF2_REG = crate::Reg<u32, _UHCI_ESC_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF2_REG;
#[doc = "`read()` method returns [uhci_esc_conf2_reg::R](uhci_esc_conf2_reg::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf2_reg::W](uhci_esc_conf2_reg::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF2_REG {}
#[doc = "UHCI_ESC_CONF2_REG(i)"]
pub mod uhci_esc_conf2_reg;
#[doc = "UHCI_ESC_CONF3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_esc_conf3_reg](uhci_esc_conf3_reg) module"]
pub type UHCI_ESC_CONF3_REG = crate::Reg<u32, _UHCI_ESC_CONF3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF3_REG;
#[doc = "`read()` method returns [uhci_esc_conf3_reg::R](uhci_esc_conf3_reg::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF3_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf3_reg::W](uhci_esc_conf3_reg::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF3_REG {}
#[doc = "UHCI_ESC_CONF3_REG(i)"]
pub mod uhci_esc_conf3_reg;
#[doc = "UHCI_PKT_THRES_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_pkt_thres_reg](uhci_pkt_thres_reg) module"]
pub type UHCI_PKT_THRES_REG = crate::Reg<u32, _UHCI_PKT_THRES_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_PKT_THRES_REG;
#[doc = "`read()` method returns [uhci_pkt_thres_reg::R](uhci_pkt_thres_reg::R) reader structure"]
impl crate::Readable for UHCI_PKT_THRES_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_pkt_thres_reg::W](uhci_pkt_thres_reg::W) writer structure"]
impl crate::Writable for UHCI_PKT_THRES_REG {}
#[doc = "UHCI_PKT_THRES_REG(i)"]
pub mod uhci_pkt_thres_reg;
#[doc = "UHCI_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uhci_date_reg](uhci_date_reg) module"]
pub type UHCI_DATE_REG = crate::Reg<u32, _UHCI_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DATE_REG;
#[doc = "`read()` method returns [uhci_date_reg::R](uhci_date_reg::R) reader structure"]
impl crate::Readable for UHCI_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [uhci_date_reg::W](uhci_date_reg::W) writer structure"]
impl crate::Writable for UHCI_DATE_REG {}
#[doc = "UHCI_DATE_REG(i)"]
pub mod uhci_date_reg;
