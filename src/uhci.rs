#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UHCI_CONF0"]
    pub conf0: CONF0,
    #[doc = "0x04 - UHCI_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x08 - UHCI_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x0c - UHCI_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x10 - UHCI_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x14 - UHCI_DMA_OUT_STATUS"]
    pub dma_out_status: DMA_OUT_STATUS,
    #[doc = "0x18 - UHCI_DMA_OUT_PUSH"]
    pub dma_out_push: DMA_OUT_PUSH,
    #[doc = "0x1c - UHCI_DMA_IN_STATUS"]
    pub dma_in_status: DMA_IN_STATUS,
    #[doc = "0x20 - UHCI_DMA_IN_POP"]
    pub dma_in_pop: DMA_IN_POP,
    #[doc = "0x24 - UHCI_DMA_OUT_LINK"]
    pub dma_out_link: DMA_OUT_LINK,
    #[doc = "0x28 - UHCI_DMA_IN_LINK"]
    pub dma_in_link: DMA_IN_LINK,
    #[doc = "0x2c - UHCI_CONF1"]
    pub conf1: CONF1,
    #[doc = "0x30 - UHCI_STATE0"]
    pub state0: STATE0,
    #[doc = "0x34 - UHCI_STATE1"]
    pub state1: STATE1,
    #[doc = "0x38 - UHCI_DMA_OUT_EOF_DES_ADDR"]
    pub dma_out_eof_des_addr: DMA_OUT_EOF_DES_ADDR,
    #[doc = "0x3c - UHCI_DMA_IN_SUC_EOF_DES_ADDR"]
    pub dma_in_suc_eof_des_addr: DMA_IN_SUC_EOF_DES_ADDR,
    #[doc = "0x40 - UHCI_DMA_IN_ERR_EOF_DES_ADDR"]
    pub dma_in_err_eof_des_addr: DMA_IN_ERR_EOF_DES_ADDR,
    #[doc = "0x44 - UHCI_DMA_OUT_EOF_BFR_DES_ADDR"]
    pub dma_out_eof_bfr_des_addr: DMA_OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x48 - UHCI_AHB_TEST"]
    pub ahb_test: AHB_TEST,
    #[doc = "0x4c - UHCI_DMA_IN_DSCR"]
    pub dma_in_dscr: DMA_IN_DSCR,
    #[doc = "0x50 - UHCI_DMA_IN_DSCR_BF0"]
    pub dma_in_dscr_bf0: DMA_IN_DSCR_BF0,
    #[doc = "0x54 - UHCI_DMA_IN_DSCR_BF1"]
    pub dma_in_dscr_bf1: DMA_IN_DSCR_BF1,
    #[doc = "0x58 - UHCI_DMA_OUT_DSCR"]
    pub dma_out_dscr: DMA_OUT_DSCR,
    #[doc = "0x5c - UHCI_DMA_OUT_DSCR_BF0"]
    pub dma_out_dscr_bf0: DMA_OUT_DSCR_BF0,
    #[doc = "0x60 - UHCI_DMA_OUT_DSCR_BF1"]
    pub dma_out_dscr_bf1: DMA_OUT_DSCR_BF1,
    #[doc = "0x64 - UHCI_ESCAPE_CONF"]
    pub escape_conf: ESCAPE_CONF,
    #[doc = "0x68 - UHCI_HUNG_CONF"]
    pub hung_conf: HUNG_CONF,
    _reserved27: [u8; 4usize],
    #[doc = "0x70 - UHCI_RX_HEAD"]
    pub rx_head: RX_HEAD,
    #[doc = "0x74 - UHCI_QUICK_SENT"]
    pub quick_sent: QUICK_SENT,
    #[doc = "0x78 - UHCI_Q0_WORD0"]
    pub q0_word0: Q0_WORD0,
    #[doc = "0x7c - UHCI_Q0_WORD1"]
    pub q0_word1: Q0_WORD1,
    #[doc = "0x80 - UHCI_Q1_WORD0"]
    pub q1_word0: Q1_WORD0,
    #[doc = "0x84 - UHCI_Q1_WORD1"]
    pub q1_word1: Q1_WORD1,
    #[doc = "0x88 - UHCI_Q2_WORD0"]
    pub q2_word0: Q2_WORD0,
    #[doc = "0x8c - UHCI_Q2_WORD1"]
    pub q2_word1: Q2_WORD1,
    #[doc = "0x90 - UHCI_Q3_WORD0"]
    pub q3_word0: Q3_WORD0,
    #[doc = "0x94 - UHCI_Q3_WORD1"]
    pub q3_word1: Q3_WORD1,
    #[doc = "0x98 - UHCI_Q4_WORD0"]
    pub q4_word0: Q4_WORD0,
    #[doc = "0x9c - UHCI_Q4_WORD1"]
    pub q4_word1: Q4_WORD1,
    #[doc = "0xa0 - UHCI_Q5_WORD0"]
    pub q5_word0: Q5_WORD0,
    #[doc = "0xa4 - UHCI_Q5_WORD1"]
    pub q5_word1: Q5_WORD1,
    #[doc = "0xa8 - UHCI_Q6_WORD0"]
    pub q6_word0: Q6_WORD0,
    #[doc = "0xac - UHCI_Q6_WORD1"]
    pub q6_word1: Q6_WORD1,
    #[doc = "0xb0 - UHCI_ESC_CONF0"]
    pub esc_conf0: ESC_CONF0,
    #[doc = "0xb4 - UHCI_ESC_CONF1"]
    pub esc_conf1: ESC_CONF1,
    #[doc = "0xb8 - UHCI_ESC_CONF2"]
    pub esc_conf2: ESC_CONF2,
    #[doc = "0xbc - UHCI_ESC_CONF3"]
    pub esc_conf3: ESC_CONF3,
    #[doc = "0xc0 - UHCI_PKT_THRES"]
    pub pkt_thres: PKT_THRES,
    _reserved48: [u8; 56usize],
    #[doc = "0xfc - UHCI_DATE"]
    pub date: DATE,
}
#[doc = "UHCI_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf0](conf0) module"]
pub type CONF0 = crate::Reg<u32, _CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF0;
#[doc = "`read()` method returns [conf0::R](conf0::R) reader structure"]
impl crate::Readable for CONF0 {}
#[doc = "`write(|w| ..)` method takes [conf0::W](conf0::W) writer structure"]
impl crate::Writable for CONF0 {}
#[doc = "UHCI_CONF0"]
pub mod conf0;
#[doc = "UHCI_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "UHCI_INT_RAW"]
pub mod int_raw;
#[doc = "UHCI_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "UHCI_INT_ST"]
pub mod int_st;
#[doc = "UHCI_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "UHCI_INT_ENA"]
pub mod int_ena;
#[doc = "UHCI_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "UHCI_INT_CLR"]
pub mod int_clr;
#[doc = "UHCI_DMA_OUT_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_status](dma_out_status) module"]
pub type DMA_OUT_STATUS = crate::Reg<u32, _DMA_OUT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_STATUS;
#[doc = "`read()` method returns [dma_out_status::R](dma_out_status::R) reader structure"]
impl crate::Readable for DMA_OUT_STATUS {}
#[doc = "`write(|w| ..)` method takes [dma_out_status::W](dma_out_status::W) writer structure"]
impl crate::Writable for DMA_OUT_STATUS {}
#[doc = "UHCI_DMA_OUT_STATUS"]
pub mod dma_out_status;
#[doc = "UHCI_DMA_OUT_PUSH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_push](dma_out_push) module"]
pub type DMA_OUT_PUSH = crate::Reg<u32, _DMA_OUT_PUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PUSH;
#[doc = "`read()` method returns [dma_out_push::R](dma_out_push::R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH {}
#[doc = "`write(|w| ..)` method takes [dma_out_push::W](dma_out_push::W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH {}
#[doc = "UHCI_DMA_OUT_PUSH"]
pub mod dma_out_push;
#[doc = "UHCI_DMA_IN_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_status](dma_in_status) module"]
pub type DMA_IN_STATUS = crate::Reg<u32, _DMA_IN_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_STATUS;
#[doc = "`read()` method returns [dma_in_status::R](dma_in_status::R) reader structure"]
impl crate::Readable for DMA_IN_STATUS {}
#[doc = "`write(|w| ..)` method takes [dma_in_status::W](dma_in_status::W) writer structure"]
impl crate::Writable for DMA_IN_STATUS {}
#[doc = "UHCI_DMA_IN_STATUS"]
pub mod dma_in_status;
#[doc = "UHCI_DMA_IN_POP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_pop](dma_in_pop) module"]
pub type DMA_IN_POP = crate::Reg<u32, _DMA_IN_POP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_POP;
#[doc = "`read()` method returns [dma_in_pop::R](dma_in_pop::R) reader structure"]
impl crate::Readable for DMA_IN_POP {}
#[doc = "`write(|w| ..)` method takes [dma_in_pop::W](dma_in_pop::W) writer structure"]
impl crate::Writable for DMA_IN_POP {}
#[doc = "UHCI_DMA_IN_POP"]
pub mod dma_in_pop;
#[doc = "UHCI_DMA_OUT_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_link](dma_out_link) module"]
pub type DMA_OUT_LINK = crate::Reg<u32, _DMA_OUT_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_LINK;
#[doc = "`read()` method returns [dma_out_link::R](dma_out_link::R) reader structure"]
impl crate::Readable for DMA_OUT_LINK {}
#[doc = "`write(|w| ..)` method takes [dma_out_link::W](dma_out_link::W) writer structure"]
impl crate::Writable for DMA_OUT_LINK {}
#[doc = "UHCI_DMA_OUT_LINK"]
pub mod dma_out_link;
#[doc = "UHCI_DMA_IN_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_link](dma_in_link) module"]
pub type DMA_IN_LINK = crate::Reg<u32, _DMA_IN_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_LINK;
#[doc = "`read()` method returns [dma_in_link::R](dma_in_link::R) reader structure"]
impl crate::Readable for DMA_IN_LINK {}
#[doc = "`write(|w| ..)` method takes [dma_in_link::W](dma_in_link::W) writer structure"]
impl crate::Writable for DMA_IN_LINK {}
#[doc = "UHCI_DMA_IN_LINK"]
pub mod dma_in_link;
#[doc = "UHCI_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf1](conf1) module"]
pub type CONF1 = crate::Reg<u32, _CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF1;
#[doc = "`read()` method returns [conf1::R](conf1::R) reader structure"]
impl crate::Readable for CONF1 {}
#[doc = "`write(|w| ..)` method takes [conf1::W](conf1::W) writer structure"]
impl crate::Writable for CONF1 {}
#[doc = "UHCI_CONF1"]
pub mod conf1;
#[doc = "UHCI_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [state0](state0) module"]
pub type STATE0 = crate::Reg<u32, _STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE0;
#[doc = "`read()` method returns [state0::R](state0::R) reader structure"]
impl crate::Readable for STATE0 {}
#[doc = "`write(|w| ..)` method takes [state0::W](state0::W) writer structure"]
impl crate::Writable for STATE0 {}
#[doc = "UHCI_STATE0"]
pub mod state0;
#[doc = "UHCI_STATE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [state1](state1) module"]
pub type STATE1 = crate::Reg<u32, _STATE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE1;
#[doc = "`read()` method returns [state1::R](state1::R) reader structure"]
impl crate::Readable for STATE1 {}
#[doc = "`write(|w| ..)` method takes [state1::W](state1::W) writer structure"]
impl crate::Writable for STATE1 {}
#[doc = "UHCI_STATE1"]
pub mod state1;
#[doc = "UHCI_DMA_OUT_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_eof_des_addr](dma_out_eof_des_addr) module"]
pub type DMA_OUT_EOF_DES_ADDR = crate::Reg<u32, _DMA_OUT_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_DES_ADDR;
#[doc = "`read()` method returns [dma_out_eof_des_addr::R](dma_out_eof_des_addr::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_out_eof_des_addr::W](dma_out_eof_des_addr::W) writer structure"]
impl crate::Writable for DMA_OUT_EOF_DES_ADDR {}
#[doc = "UHCI_DMA_OUT_EOF_DES_ADDR"]
pub mod dma_out_eof_des_addr;
#[doc = "UHCI_DMA_IN_SUC_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_suc_eof_des_addr](dma_in_suc_eof_des_addr) module"]
pub type DMA_IN_SUC_EOF_DES_ADDR = crate::Reg<u32, _DMA_IN_SUC_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_SUC_EOF_DES_ADDR;
#[doc = "`read()` method returns [dma_in_suc_eof_des_addr::R](dma_in_suc_eof_des_addr::R) reader structure"]
impl crate::Readable for DMA_IN_SUC_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_in_suc_eof_des_addr::W](dma_in_suc_eof_des_addr::W) writer structure"]
impl crate::Writable for DMA_IN_SUC_EOF_DES_ADDR {}
#[doc = "UHCI_DMA_IN_SUC_EOF_DES_ADDR"]
pub mod dma_in_suc_eof_des_addr;
#[doc = "UHCI_DMA_IN_ERR_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_err_eof_des_addr](dma_in_err_eof_des_addr) module"]
pub type DMA_IN_ERR_EOF_DES_ADDR = crate::Reg<u32, _DMA_IN_ERR_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_ERR_EOF_DES_ADDR;
#[doc = "`read()` method returns [dma_in_err_eof_des_addr::R](dma_in_err_eof_des_addr::R) reader structure"]
impl crate::Readable for DMA_IN_ERR_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_in_err_eof_des_addr::W](dma_in_err_eof_des_addr::W) writer structure"]
impl crate::Writable for DMA_IN_ERR_EOF_DES_ADDR {}
#[doc = "UHCI_DMA_IN_ERR_EOF_DES_ADDR"]
pub mod dma_in_err_eof_des_addr;
#[doc = "UHCI_DMA_OUT_EOF_BFR_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_eof_bfr_des_addr](dma_out_eof_bfr_des_addr) module"]
pub type DMA_OUT_EOF_BFR_DES_ADDR = crate::Reg<u32, _DMA_OUT_EOF_BFR_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_BFR_DES_ADDR;
#[doc = "`read()` method returns [dma_out_eof_bfr_des_addr::R](dma_out_eof_bfr_des_addr::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_BFR_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_out_eof_bfr_des_addr::W](dma_out_eof_bfr_des_addr::W) writer structure"]
impl crate::Writable for DMA_OUT_EOF_BFR_DES_ADDR {}
#[doc = "UHCI_DMA_OUT_EOF_BFR_DES_ADDR"]
pub mod dma_out_eof_bfr_des_addr;
#[doc = "UHCI_AHB_TEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_test](ahb_test) module"]
pub type AHB_TEST = crate::Reg<u32, _AHB_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_TEST;
#[doc = "`read()` method returns [ahb_test::R](ahb_test::R) reader structure"]
impl crate::Readable for AHB_TEST {}
#[doc = "`write(|w| ..)` method takes [ahb_test::W](ahb_test::W) writer structure"]
impl crate::Writable for AHB_TEST {}
#[doc = "UHCI_AHB_TEST"]
pub mod ahb_test;
#[doc = "UHCI_DMA_IN_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_dscr](dma_in_dscr) module"]
pub type DMA_IN_DSCR = crate::Reg<u32, _DMA_IN_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR;
#[doc = "`read()` method returns [dma_in_dscr::R](dma_in_dscr::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR {}
#[doc = "`write(|w| ..)` method takes [dma_in_dscr::W](dma_in_dscr::W) writer structure"]
impl crate::Writable for DMA_IN_DSCR {}
#[doc = "UHCI_DMA_IN_DSCR"]
pub mod dma_in_dscr;
#[doc = "UHCI_DMA_IN_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_dscr_bf0](dma_in_dscr_bf0) module"]
pub type DMA_IN_DSCR_BF0 = crate::Reg<u32, _DMA_IN_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF0;
#[doc = "`read()` method returns [dma_in_dscr_bf0::R](dma_in_dscr_bf0::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_dscr_bf0::W](dma_in_dscr_bf0::W) writer structure"]
impl crate::Writable for DMA_IN_DSCR_BF0 {}
#[doc = "UHCI_DMA_IN_DSCR_BF0"]
pub mod dma_in_dscr_bf0;
#[doc = "UHCI_DMA_IN_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_in_dscr_bf1](dma_in_dscr_bf1) module"]
pub type DMA_IN_DSCR_BF1 = crate::Reg<u32, _DMA_IN_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF1;
#[doc = "`read()` method returns [dma_in_dscr_bf1::R](dma_in_dscr_bf1::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_dscr_bf1::W](dma_in_dscr_bf1::W) writer structure"]
impl crate::Writable for DMA_IN_DSCR_BF1 {}
#[doc = "UHCI_DMA_IN_DSCR_BF1"]
pub mod dma_in_dscr_bf1;
#[doc = "UHCI_DMA_OUT_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_dscr](dma_out_dscr) module"]
pub type DMA_OUT_DSCR = crate::Reg<u32, _DMA_OUT_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR;
#[doc = "`read()` method returns [dma_out_dscr::R](dma_out_dscr::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR {}
#[doc = "`write(|w| ..)` method takes [dma_out_dscr::W](dma_out_dscr::W) writer structure"]
impl crate::Writable for DMA_OUT_DSCR {}
#[doc = "UHCI_DMA_OUT_DSCR"]
pub mod dma_out_dscr;
#[doc = "UHCI_DMA_OUT_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_dscr_bf0](dma_out_dscr_bf0) module"]
pub type DMA_OUT_DSCR_BF0 = crate::Reg<u32, _DMA_OUT_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF0;
#[doc = "`read()` method returns [dma_out_dscr_bf0::R](dma_out_dscr_bf0::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_dscr_bf0::W](dma_out_dscr_bf0::W) writer structure"]
impl crate::Writable for DMA_OUT_DSCR_BF0 {}
#[doc = "UHCI_DMA_OUT_DSCR_BF0"]
pub mod dma_out_dscr_bf0;
#[doc = "UHCI_DMA_OUT_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_out_dscr_bf1](dma_out_dscr_bf1) module"]
pub type DMA_OUT_DSCR_BF1 = crate::Reg<u32, _DMA_OUT_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF1;
#[doc = "`read()` method returns [dma_out_dscr_bf1::R](dma_out_dscr_bf1::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_dscr_bf1::W](dma_out_dscr_bf1::W) writer structure"]
impl crate::Writable for DMA_OUT_DSCR_BF1 {}
#[doc = "UHCI_DMA_OUT_DSCR_BF1"]
pub mod dma_out_dscr_bf1;
#[doc = "UHCI_ESCAPE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [escape_conf](escape_conf) module"]
pub type ESCAPE_CONF = crate::Reg<u32, _ESCAPE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESCAPE_CONF;
#[doc = "`read()` method returns [escape_conf::R](escape_conf::R) reader structure"]
impl crate::Readable for ESCAPE_CONF {}
#[doc = "`write(|w| ..)` method takes [escape_conf::W](escape_conf::W) writer structure"]
impl crate::Writable for ESCAPE_CONF {}
#[doc = "UHCI_ESCAPE_CONF"]
pub mod escape_conf;
#[doc = "UHCI_HUNG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hung_conf](hung_conf) module"]
pub type HUNG_CONF = crate::Reg<u32, _HUNG_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUNG_CONF;
#[doc = "`read()` method returns [hung_conf::R](hung_conf::R) reader structure"]
impl crate::Readable for HUNG_CONF {}
#[doc = "`write(|w| ..)` method takes [hung_conf::W](hung_conf::W) writer structure"]
impl crate::Writable for HUNG_CONF {}
#[doc = "UHCI_HUNG_CONF"]
pub mod hung_conf;
#[doc = "UHCI_RX_HEAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_head](rx_head) module"]
pub type RX_HEAD = crate::Reg<u32, _RX_HEAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_HEAD;
#[doc = "`read()` method returns [rx_head::R](rx_head::R) reader structure"]
impl crate::Readable for RX_HEAD {}
#[doc = "`write(|w| ..)` method takes [rx_head::W](rx_head::W) writer structure"]
impl crate::Writable for RX_HEAD {}
#[doc = "UHCI_RX_HEAD"]
pub mod rx_head;
#[doc = "UHCI_QUICK_SENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [quick_sent](quick_sent) module"]
pub type QUICK_SENT = crate::Reg<u32, _QUICK_SENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUICK_SENT;
#[doc = "`read()` method returns [quick_sent::R](quick_sent::R) reader structure"]
impl crate::Readable for QUICK_SENT {}
#[doc = "`write(|w| ..)` method takes [quick_sent::W](quick_sent::W) writer structure"]
impl crate::Writable for QUICK_SENT {}
#[doc = "UHCI_QUICK_SENT"]
pub mod quick_sent;
#[doc = "UHCI_Q0_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q0_word0](q0_word0) module"]
pub type Q0_WORD0 = crate::Reg<u32, _Q0_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q0_WORD0;
#[doc = "`read()` method returns [q0_word0::R](q0_word0::R) reader structure"]
impl crate::Readable for Q0_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q0_word0::W](q0_word0::W) writer structure"]
impl crate::Writable for Q0_WORD0 {}
#[doc = "UHCI_Q0_WORD0"]
pub mod q0_word0;
#[doc = "UHCI_Q0_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q0_word1](q0_word1) module"]
pub type Q0_WORD1 = crate::Reg<u32, _Q0_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q0_WORD1;
#[doc = "`read()` method returns [q0_word1::R](q0_word1::R) reader structure"]
impl crate::Readable for Q0_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q0_word1::W](q0_word1::W) writer structure"]
impl crate::Writable for Q0_WORD1 {}
#[doc = "UHCI_Q0_WORD1"]
pub mod q0_word1;
#[doc = "UHCI_Q1_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q1_word0](q1_word0) module"]
pub type Q1_WORD0 = crate::Reg<u32, _Q1_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q1_WORD0;
#[doc = "`read()` method returns [q1_word0::R](q1_word0::R) reader structure"]
impl crate::Readable for Q1_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q1_word0::W](q1_word0::W) writer structure"]
impl crate::Writable for Q1_WORD0 {}
#[doc = "UHCI_Q1_WORD0"]
pub mod q1_word0;
#[doc = "UHCI_Q1_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q1_word1](q1_word1) module"]
pub type Q1_WORD1 = crate::Reg<u32, _Q1_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q1_WORD1;
#[doc = "`read()` method returns [q1_word1::R](q1_word1::R) reader structure"]
impl crate::Readable for Q1_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q1_word1::W](q1_word1::W) writer structure"]
impl crate::Writable for Q1_WORD1 {}
#[doc = "UHCI_Q1_WORD1"]
pub mod q1_word1;
#[doc = "UHCI_Q2_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q2_word0](q2_word0) module"]
pub type Q2_WORD0 = crate::Reg<u32, _Q2_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q2_WORD0;
#[doc = "`read()` method returns [q2_word0::R](q2_word0::R) reader structure"]
impl crate::Readable for Q2_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q2_word0::W](q2_word0::W) writer structure"]
impl crate::Writable for Q2_WORD0 {}
#[doc = "UHCI_Q2_WORD0"]
pub mod q2_word0;
#[doc = "UHCI_Q2_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q2_word1](q2_word1) module"]
pub type Q2_WORD1 = crate::Reg<u32, _Q2_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q2_WORD1;
#[doc = "`read()` method returns [q2_word1::R](q2_word1::R) reader structure"]
impl crate::Readable for Q2_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q2_word1::W](q2_word1::W) writer structure"]
impl crate::Writable for Q2_WORD1 {}
#[doc = "UHCI_Q2_WORD1"]
pub mod q2_word1;
#[doc = "UHCI_Q3_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q3_word0](q3_word0) module"]
pub type Q3_WORD0 = crate::Reg<u32, _Q3_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q3_WORD0;
#[doc = "`read()` method returns [q3_word0::R](q3_word0::R) reader structure"]
impl crate::Readable for Q3_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q3_word0::W](q3_word0::W) writer structure"]
impl crate::Writable for Q3_WORD0 {}
#[doc = "UHCI_Q3_WORD0"]
pub mod q3_word0;
#[doc = "UHCI_Q3_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q3_word1](q3_word1) module"]
pub type Q3_WORD1 = crate::Reg<u32, _Q3_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q3_WORD1;
#[doc = "`read()` method returns [q3_word1::R](q3_word1::R) reader structure"]
impl crate::Readable for Q3_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q3_word1::W](q3_word1::W) writer structure"]
impl crate::Writable for Q3_WORD1 {}
#[doc = "UHCI_Q3_WORD1"]
pub mod q3_word1;
#[doc = "UHCI_Q4_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q4_word0](q4_word0) module"]
pub type Q4_WORD0 = crate::Reg<u32, _Q4_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q4_WORD0;
#[doc = "`read()` method returns [q4_word0::R](q4_word0::R) reader structure"]
impl crate::Readable for Q4_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q4_word0::W](q4_word0::W) writer structure"]
impl crate::Writable for Q4_WORD0 {}
#[doc = "UHCI_Q4_WORD0"]
pub mod q4_word0;
#[doc = "UHCI_Q4_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q4_word1](q4_word1) module"]
pub type Q4_WORD1 = crate::Reg<u32, _Q4_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q4_WORD1;
#[doc = "`read()` method returns [q4_word1::R](q4_word1::R) reader structure"]
impl crate::Readable for Q4_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q4_word1::W](q4_word1::W) writer structure"]
impl crate::Writable for Q4_WORD1 {}
#[doc = "UHCI_Q4_WORD1"]
pub mod q4_word1;
#[doc = "UHCI_Q5_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q5_word0](q5_word0) module"]
pub type Q5_WORD0 = crate::Reg<u32, _Q5_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q5_WORD0;
#[doc = "`read()` method returns [q5_word0::R](q5_word0::R) reader structure"]
impl crate::Readable for Q5_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q5_word0::W](q5_word0::W) writer structure"]
impl crate::Writable for Q5_WORD0 {}
#[doc = "UHCI_Q5_WORD0"]
pub mod q5_word0;
#[doc = "UHCI_Q5_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q5_word1](q5_word1) module"]
pub type Q5_WORD1 = crate::Reg<u32, _Q5_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q5_WORD1;
#[doc = "`read()` method returns [q5_word1::R](q5_word1::R) reader structure"]
impl crate::Readable for Q5_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q5_word1::W](q5_word1::W) writer structure"]
impl crate::Writable for Q5_WORD1 {}
#[doc = "UHCI_Q5_WORD1"]
pub mod q5_word1;
#[doc = "UHCI_Q6_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q6_word0](q6_word0) module"]
pub type Q6_WORD0 = crate::Reg<u32, _Q6_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q6_WORD0;
#[doc = "`read()` method returns [q6_word0::R](q6_word0::R) reader structure"]
impl crate::Readable for Q6_WORD0 {}
#[doc = "`write(|w| ..)` method takes [q6_word0::W](q6_word0::W) writer structure"]
impl crate::Writable for Q6_WORD0 {}
#[doc = "UHCI_Q6_WORD0"]
pub mod q6_word0;
#[doc = "UHCI_Q6_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q6_word1](q6_word1) module"]
pub type Q6_WORD1 = crate::Reg<u32, _Q6_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q6_WORD1;
#[doc = "`read()` method returns [q6_word1::R](q6_word1::R) reader structure"]
impl crate::Readable for Q6_WORD1 {}
#[doc = "`write(|w| ..)` method takes [q6_word1::W](q6_word1::W) writer structure"]
impl crate::Writable for Q6_WORD1 {}
#[doc = "UHCI_Q6_WORD1"]
pub mod q6_word1;
#[doc = "UHCI_ESC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esc_conf0](esc_conf0) module"]
pub type ESC_CONF0 = crate::Reg<u32, _ESC_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_CONF0;
#[doc = "`read()` method returns [esc_conf0::R](esc_conf0::R) reader structure"]
impl crate::Readable for ESC_CONF0 {}
#[doc = "`write(|w| ..)` method takes [esc_conf0::W](esc_conf0::W) writer structure"]
impl crate::Writable for ESC_CONF0 {}
#[doc = "UHCI_ESC_CONF0"]
pub mod esc_conf0;
#[doc = "UHCI_ESC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esc_conf1](esc_conf1) module"]
pub type ESC_CONF1 = crate::Reg<u32, _ESC_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_CONF1;
#[doc = "`read()` method returns [esc_conf1::R](esc_conf1::R) reader structure"]
impl crate::Readable for ESC_CONF1 {}
#[doc = "`write(|w| ..)` method takes [esc_conf1::W](esc_conf1::W) writer structure"]
impl crate::Writable for ESC_CONF1 {}
#[doc = "UHCI_ESC_CONF1"]
pub mod esc_conf1;
#[doc = "UHCI_ESC_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esc_conf2](esc_conf2) module"]
pub type ESC_CONF2 = crate::Reg<u32, _ESC_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_CONF2;
#[doc = "`read()` method returns [esc_conf2::R](esc_conf2::R) reader structure"]
impl crate::Readable for ESC_CONF2 {}
#[doc = "`write(|w| ..)` method takes [esc_conf2::W](esc_conf2::W) writer structure"]
impl crate::Writable for ESC_CONF2 {}
#[doc = "UHCI_ESC_CONF2"]
pub mod esc_conf2;
#[doc = "UHCI_ESC_CONF3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esc_conf3](esc_conf3) module"]
pub type ESC_CONF3 = crate::Reg<u32, _ESC_CONF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_CONF3;
#[doc = "`read()` method returns [esc_conf3::R](esc_conf3::R) reader structure"]
impl crate::Readable for ESC_CONF3 {}
#[doc = "`write(|w| ..)` method takes [esc_conf3::W](esc_conf3::W) writer structure"]
impl crate::Writable for ESC_CONF3 {}
#[doc = "UHCI_ESC_CONF3"]
pub mod esc_conf3;
#[doc = "UHCI_PKT_THRES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkt_thres](pkt_thres) module"]
pub type PKT_THRES = crate::Reg<u32, _PKT_THRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKT_THRES;
#[doc = "`read()` method returns [pkt_thres::R](pkt_thres::R) reader structure"]
impl crate::Readable for PKT_THRES {}
#[doc = "`write(|w| ..)` method takes [pkt_thres::W](pkt_thres::W) writer structure"]
impl crate::Writable for PKT_THRES {}
#[doc = "UHCI_PKT_THRES"]
pub mod pkt_thres;
#[doc = "UHCI_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "UHCI_DATE"]
pub mod date;
