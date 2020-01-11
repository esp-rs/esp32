#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - I2S_CONF"]
    pub conf: CONF,
    #[doc = "0x0c - I2S_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x10 - I2S_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x14 - I2S_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x18 - I2S_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x1c - I2S_TIMING"]
    pub timing: TIMING,
    #[doc = "0x20 - I2S_FIFO_CONF"]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x24 - I2S_RXEOF_NUM"]
    pub rxeof_num: RXEOF_NUM,
    #[doc = "0x28 - I2S_CONF_SIGLE_DATA"]
    pub conf_sigle_data: CONF_SIGLE_DATA,
    #[doc = "0x2c - I2S_CONF_CHAN"]
    pub conf_chan: CONF_CHAN,
    #[doc = "0x30 - I2S_OUT_LINK"]
    pub out_link: OUT_LINK,
    #[doc = "0x34 - I2S_IN_LINK"]
    pub in_link: IN_LINK,
    #[doc = "0x38 - I2S_OUT_EOF_DES_ADDR"]
    pub out_eof_des_addr: OUT_EOF_DES_ADDR,
    #[doc = "0x3c - I2S_IN_EOF_DES_ADDR"]
    pub in_eof_des_addr: IN_EOF_DES_ADDR,
    #[doc = "0x40 - I2S_OUT_EOF_BFR_DES_ADDR"]
    pub out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x44 - I2S_AHB_TEST"]
    pub ahb_test: AHB_TEST,
    #[doc = "0x48 - I2S_INLINK_DSCR"]
    pub inlink_dscr: INLINK_DSCR,
    #[doc = "0x4c - I2S_INLINK_DSCR_BF0"]
    pub inlink_dscr_bf0: INLINK_DSCR_BF0,
    #[doc = "0x50 - I2S_INLINK_DSCR_BF1"]
    pub inlink_dscr_bf1: INLINK_DSCR_BF1,
    #[doc = "0x54 - I2S_OUTLINK_DSCR"]
    pub outlink_dscr: OUTLINK_DSCR,
    #[doc = "0x58 - I2S_OUTLINK_DSCR_BF0"]
    pub outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    #[doc = "0x5c - I2S_OUTLINK_DSCR_BF1"]
    pub outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    #[doc = "0x60 - I2S_LC_CONF"]
    pub lc_conf: LC_CONF,
    #[doc = "0x64 - I2S_OUTFIFO_PUSH"]
    pub outfifo_push: OUTFIFO_PUSH,
    #[doc = "0x68 - I2S_INFIFO_POP"]
    pub infifo_pop: INFIFO_POP,
    #[doc = "0x6c - I2S_LC_STATE0"]
    pub lc_state0: LC_STATE0,
    #[doc = "0x70 - I2S_LC_STATE1"]
    pub lc_state1: LC_STATE1,
    #[doc = "0x74 - I2S_LC_HUNG_CONF"]
    pub lc_hung_conf: LC_HUNG_CONF,
    _reserved28: [u8; 8usize],
    #[doc = "0x80 - I2S_CVSD_CONF0"]
    pub cvsd_conf0: CVSD_CONF0,
    #[doc = "0x84 - I2S_CVSD_CONF1"]
    pub cvsd_conf1: CVSD_CONF1,
    #[doc = "0x88 - I2S_CVSD_CONF2"]
    pub cvsd_conf2: CVSD_CONF2,
    #[doc = "0x8c - I2S_PLC_CONF0"]
    pub plc_conf0: PLC_CONF0,
    #[doc = "0x90 - I2S_PLC_CONF1"]
    pub plc_conf1: PLC_CONF1,
    #[doc = "0x94 - I2S_PLC_CONF2"]
    pub plc_conf2: PLC_CONF2,
    #[doc = "0x98 - I2S_ESCO_CONF0"]
    pub esco_conf0: ESCO_CONF0,
    #[doc = "0x9c - I2S_SCO_CONF0"]
    pub sco_conf0: SCO_CONF0,
    #[doc = "0xa0 - I2S_CONF1"]
    pub conf1: CONF1,
    #[doc = "0xa4 - I2S_PD_CONF"]
    pub pd_conf: PD_CONF,
    #[doc = "0xa8 - I2S_CONF2"]
    pub conf2: CONF2,
    #[doc = "0xac - I2S_CLKM_CONF"]
    pub clkm_conf: CLKM_CONF,
    #[doc = "0xb0 - I2S_SAMPLE_RATE_CONF"]
    pub sample_rate_conf: SAMPLE_RATE_CONF,
    #[doc = "0xb4 - I2S_PDM_CONF"]
    pub pdm_conf: PDM_CONF,
    #[doc = "0xb8 - I2S_PDM_FREQ_CONF"]
    pub pdm_freq_conf: PDM_FREQ_CONF,
    #[doc = "0xbc - I2S_STATE"]
    pub state: STATE,
    _reserved44: [u8; 60usize],
    #[doc = "0xfc - I2S_DATE"]
    pub date: DATE,
}
#[doc = "I2S_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "I2S_CONF"]
pub mod conf;
#[doc = "I2S_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "I2S_INT_RAW"]
pub mod int_raw;
#[doc = "I2S_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "I2S_INT_ST"]
pub mod int_st;
#[doc = "I2S_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "I2S_INT_ENA"]
pub mod int_ena;
#[doc = "I2S_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "I2S_INT_CLR"]
pub mod int_clr;
#[doc = "I2S_TIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timing](timing) module"]
pub type TIMING = crate::Reg<u32, _TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMING;
#[doc = "`read()` method returns [timing::R](timing::R) reader structure"]
impl crate::Readable for TIMING {}
#[doc = "`write(|w| ..)` method takes [timing::W](timing::W) writer structure"]
impl crate::Writable for TIMING {}
#[doc = "I2S_TIMING"]
pub mod timing;
#[doc = "I2S_FIFO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo_conf](fifo_conf) module"]
pub type FIFO_CONF = crate::Reg<u32, _FIFO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CONF;
#[doc = "`read()` method returns [fifo_conf::R](fifo_conf::R) reader structure"]
impl crate::Readable for FIFO_CONF {}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](fifo_conf::W) writer structure"]
impl crate::Writable for FIFO_CONF {}
#[doc = "I2S_FIFO_CONF"]
pub mod fifo_conf;
#[doc = "I2S_RXEOF_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxeof_num](rxeof_num) module"]
pub type RXEOF_NUM = crate::Reg<u32, _RXEOF_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXEOF_NUM;
#[doc = "`read()` method returns [rxeof_num::R](rxeof_num::R) reader structure"]
impl crate::Readable for RXEOF_NUM {}
#[doc = "`write(|w| ..)` method takes [rxeof_num::W](rxeof_num::W) writer structure"]
impl crate::Writable for RXEOF_NUM {}
#[doc = "I2S_RXEOF_NUM"]
pub mod rxeof_num;
#[doc = "I2S_CONF_SIGLE_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf_sigle_data](conf_sigle_data) module"]
pub type CONF_SIGLE_DATA = crate::Reg<u32, _CONF_SIGLE_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF_SIGLE_DATA;
#[doc = "`read()` method returns [conf_sigle_data::R](conf_sigle_data::R) reader structure"]
impl crate::Readable for CONF_SIGLE_DATA {}
#[doc = "`write(|w| ..)` method takes [conf_sigle_data::W](conf_sigle_data::W) writer structure"]
impl crate::Writable for CONF_SIGLE_DATA {}
#[doc = "I2S_CONF_SIGLE_DATA"]
pub mod conf_sigle_data;
#[doc = "I2S_CONF_CHAN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf_chan](conf_chan) module"]
pub type CONF_CHAN = crate::Reg<u32, _CONF_CHAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF_CHAN;
#[doc = "`read()` method returns [conf_chan::R](conf_chan::R) reader structure"]
impl crate::Readable for CONF_CHAN {}
#[doc = "`write(|w| ..)` method takes [conf_chan::W](conf_chan::W) writer structure"]
impl crate::Writable for CONF_CHAN {}
#[doc = "I2S_CONF_CHAN"]
pub mod conf_chan;
#[doc = "I2S_OUT_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_link](out_link) module"]
pub type OUT_LINK = crate::Reg<u32, _OUT_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_LINK;
#[doc = "`read()` method returns [out_link::R](out_link::R) reader structure"]
impl crate::Readable for OUT_LINK {}
#[doc = "`write(|w| ..)` method takes [out_link::W](out_link::W) writer structure"]
impl crate::Writable for OUT_LINK {}
#[doc = "I2S_OUT_LINK"]
pub mod out_link;
#[doc = "I2S_IN_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_link](in_link) module"]
pub type IN_LINK = crate::Reg<u32, _IN_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_LINK;
#[doc = "`read()` method returns [in_link::R](in_link::R) reader structure"]
impl crate::Readable for IN_LINK {}
#[doc = "`write(|w| ..)` method takes [in_link::W](in_link::W) writer structure"]
impl crate::Writable for IN_LINK {}
#[doc = "I2S_IN_LINK"]
pub mod in_link;
#[doc = "I2S_OUT_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_eof_des_addr](out_eof_des_addr) module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<u32, _OUT_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EOF_DES_ADDR;
#[doc = "`read()` method returns [out_eof_des_addr::R](out_eof_des_addr::R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [out_eof_des_addr::W](out_eof_des_addr::W) writer structure"]
impl crate::Writable for OUT_EOF_DES_ADDR {}
#[doc = "I2S_OUT_EOF_DES_ADDR"]
pub mod out_eof_des_addr;
#[doc = "I2S_IN_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_eof_des_addr](in_eof_des_addr) module"]
pub type IN_EOF_DES_ADDR = crate::Reg<u32, _IN_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_EOF_DES_ADDR;
#[doc = "`read()` method returns [in_eof_des_addr::R](in_eof_des_addr::R) reader structure"]
impl crate::Readable for IN_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [in_eof_des_addr::W](in_eof_des_addr::W) writer structure"]
impl crate::Writable for IN_EOF_DES_ADDR {}
#[doc = "I2S_IN_EOF_DES_ADDR"]
pub mod in_eof_des_addr;
#[doc = "I2S_OUT_EOF_BFR_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_eof_bfr_des_addr](out_eof_bfr_des_addr) module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<u32, _OUT_EOF_BFR_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EOF_BFR_DES_ADDR;
#[doc = "`read()` method returns [out_eof_bfr_des_addr::R](out_eof_bfr_des_addr::R) reader structure"]
impl crate::Readable for OUT_EOF_BFR_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [out_eof_bfr_des_addr::W](out_eof_bfr_des_addr::W) writer structure"]
impl crate::Writable for OUT_EOF_BFR_DES_ADDR {}
#[doc = "I2S_OUT_EOF_BFR_DES_ADDR"]
pub mod out_eof_bfr_des_addr;
#[doc = "I2S_AHB_TEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_test](ahb_test) module"]
pub type AHB_TEST = crate::Reg<u32, _AHB_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_TEST;
#[doc = "`read()` method returns [ahb_test::R](ahb_test::R) reader structure"]
impl crate::Readable for AHB_TEST {}
#[doc = "`write(|w| ..)` method takes [ahb_test::W](ahb_test::W) writer structure"]
impl crate::Writable for AHB_TEST {}
#[doc = "I2S_AHB_TEST"]
pub mod ahb_test;
#[doc = "I2S_INLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inlink_dscr](inlink_dscr) module"]
pub type INLINK_DSCR = crate::Reg<u32, _INLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INLINK_DSCR;
#[doc = "`read()` method returns [inlink_dscr::R](inlink_dscr::R) reader structure"]
impl crate::Readable for INLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [inlink_dscr::W](inlink_dscr::W) writer structure"]
impl crate::Writable for INLINK_DSCR {}
#[doc = "I2S_INLINK_DSCR"]
pub mod inlink_dscr;
#[doc = "I2S_INLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inlink_dscr_bf0](inlink_dscr_bf0) module"]
pub type INLINK_DSCR_BF0 = crate::Reg<u32, _INLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INLINK_DSCR_BF0;
#[doc = "`read()` method returns [inlink_dscr_bf0::R](inlink_dscr_bf0::R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [inlink_dscr_bf0::W](inlink_dscr_bf0::W) writer structure"]
impl crate::Writable for INLINK_DSCR_BF0 {}
#[doc = "I2S_INLINK_DSCR_BF0"]
pub mod inlink_dscr_bf0;
#[doc = "I2S_INLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inlink_dscr_bf1](inlink_dscr_bf1) module"]
pub type INLINK_DSCR_BF1 = crate::Reg<u32, _INLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INLINK_DSCR_BF1;
#[doc = "`read()` method returns [inlink_dscr_bf1::R](inlink_dscr_bf1::R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [inlink_dscr_bf1::W](inlink_dscr_bf1::W) writer structure"]
impl crate::Writable for INLINK_DSCR_BF1 {}
#[doc = "I2S_INLINK_DSCR_BF1"]
pub mod inlink_dscr_bf1;
#[doc = "I2S_OUTLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outlink_dscr](outlink_dscr) module"]
pub type OUTLINK_DSCR = crate::Reg<u32, _OUTLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTLINK_DSCR;
#[doc = "`read()` method returns [outlink_dscr::R](outlink_dscr::R) reader structure"]
impl crate::Readable for OUTLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [outlink_dscr::W](outlink_dscr::W) writer structure"]
impl crate::Writable for OUTLINK_DSCR {}
#[doc = "I2S_OUTLINK_DSCR"]
pub mod outlink_dscr;
#[doc = "I2S_OUTLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outlink_dscr_bf0](outlink_dscr_bf0) module"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<u32, _OUTLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTLINK_DSCR_BF0;
#[doc = "`read()` method returns [outlink_dscr_bf0::R](outlink_dscr_bf0::R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [outlink_dscr_bf0::W](outlink_dscr_bf0::W) writer structure"]
impl crate::Writable for OUTLINK_DSCR_BF0 {}
#[doc = "I2S_OUTLINK_DSCR_BF0"]
pub mod outlink_dscr_bf0;
#[doc = "I2S_OUTLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outlink_dscr_bf1](outlink_dscr_bf1) module"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<u32, _OUTLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTLINK_DSCR_BF1;
#[doc = "`read()` method returns [outlink_dscr_bf1::R](outlink_dscr_bf1::R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [outlink_dscr_bf1::W](outlink_dscr_bf1::W) writer structure"]
impl crate::Writable for OUTLINK_DSCR_BF1 {}
#[doc = "I2S_OUTLINK_DSCR_BF1"]
pub mod outlink_dscr_bf1;
#[doc = "I2S_LC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lc_conf](lc_conf) module"]
pub type LC_CONF = crate::Reg<u32, _LC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC_CONF;
#[doc = "`read()` method returns [lc_conf::R](lc_conf::R) reader structure"]
impl crate::Readable for LC_CONF {}
#[doc = "`write(|w| ..)` method takes [lc_conf::W](lc_conf::W) writer structure"]
impl crate::Writable for LC_CONF {}
#[doc = "I2S_LC_CONF"]
pub mod lc_conf;
#[doc = "I2S_OUTFIFO_PUSH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outfifo_push](outfifo_push) module"]
pub type OUTFIFO_PUSH = crate::Reg<u32, _OUTFIFO_PUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTFIFO_PUSH;
#[doc = "`read()` method returns [outfifo_push::R](outfifo_push::R) reader structure"]
impl crate::Readable for OUTFIFO_PUSH {}
#[doc = "`write(|w| ..)` method takes [outfifo_push::W](outfifo_push::W) writer structure"]
impl crate::Writable for OUTFIFO_PUSH {}
#[doc = "I2S_OUTFIFO_PUSH"]
pub mod outfifo_push;
#[doc = "I2S_INFIFO_POP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [infifo_pop](infifo_pop) module"]
pub type INFIFO_POP = crate::Reg<u32, _INFIFO_POP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INFIFO_POP;
#[doc = "`read()` method returns [infifo_pop::R](infifo_pop::R) reader structure"]
impl crate::Readable for INFIFO_POP {}
#[doc = "`write(|w| ..)` method takes [infifo_pop::W](infifo_pop::W) writer structure"]
impl crate::Writable for INFIFO_POP {}
#[doc = "I2S_INFIFO_POP"]
pub mod infifo_pop;
#[doc = "I2S_LC_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lc_state0](lc_state0) module"]
pub type LC_STATE0 = crate::Reg<u32, _LC_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC_STATE0;
#[doc = "`read()` method returns [lc_state0::R](lc_state0::R) reader structure"]
impl crate::Readable for LC_STATE0 {}
#[doc = "`write(|w| ..)` method takes [lc_state0::W](lc_state0::W) writer structure"]
impl crate::Writable for LC_STATE0 {}
#[doc = "I2S_LC_STATE0"]
pub mod lc_state0;
#[doc = "I2S_LC_STATE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lc_state1](lc_state1) module"]
pub type LC_STATE1 = crate::Reg<u32, _LC_STATE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC_STATE1;
#[doc = "`read()` method returns [lc_state1::R](lc_state1::R) reader structure"]
impl crate::Readable for LC_STATE1 {}
#[doc = "`write(|w| ..)` method takes [lc_state1::W](lc_state1::W) writer structure"]
impl crate::Writable for LC_STATE1 {}
#[doc = "I2S_LC_STATE1"]
pub mod lc_state1;
#[doc = "I2S_LC_HUNG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lc_hung_conf](lc_hung_conf) module"]
pub type LC_HUNG_CONF = crate::Reg<u32, _LC_HUNG_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC_HUNG_CONF;
#[doc = "`read()` method returns [lc_hung_conf::R](lc_hung_conf::R) reader structure"]
impl crate::Readable for LC_HUNG_CONF {}
#[doc = "`write(|w| ..)` method takes [lc_hung_conf::W](lc_hung_conf::W) writer structure"]
impl crate::Writable for LC_HUNG_CONF {}
#[doc = "I2S_LC_HUNG_CONF"]
pub mod lc_hung_conf;
#[doc = "I2S_CVSD_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cvsd_conf0](cvsd_conf0) module"]
pub type CVSD_CONF0 = crate::Reg<u32, _CVSD_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVSD_CONF0;
#[doc = "`read()` method returns [cvsd_conf0::R](cvsd_conf0::R) reader structure"]
impl crate::Readable for CVSD_CONF0 {}
#[doc = "`write(|w| ..)` method takes [cvsd_conf0::W](cvsd_conf0::W) writer structure"]
impl crate::Writable for CVSD_CONF0 {}
#[doc = "I2S_CVSD_CONF0"]
pub mod cvsd_conf0;
#[doc = "I2S_CVSD_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cvsd_conf1](cvsd_conf1) module"]
pub type CVSD_CONF1 = crate::Reg<u32, _CVSD_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVSD_CONF1;
#[doc = "`read()` method returns [cvsd_conf1::R](cvsd_conf1::R) reader structure"]
impl crate::Readable for CVSD_CONF1 {}
#[doc = "`write(|w| ..)` method takes [cvsd_conf1::W](cvsd_conf1::W) writer structure"]
impl crate::Writable for CVSD_CONF1 {}
#[doc = "I2S_CVSD_CONF1"]
pub mod cvsd_conf1;
#[doc = "I2S_CVSD_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cvsd_conf2](cvsd_conf2) module"]
pub type CVSD_CONF2 = crate::Reg<u32, _CVSD_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVSD_CONF2;
#[doc = "`read()` method returns [cvsd_conf2::R](cvsd_conf2::R) reader structure"]
impl crate::Readable for CVSD_CONF2 {}
#[doc = "`write(|w| ..)` method takes [cvsd_conf2::W](cvsd_conf2::W) writer structure"]
impl crate::Writable for CVSD_CONF2 {}
#[doc = "I2S_CVSD_CONF2"]
pub mod cvsd_conf2;
#[doc = "I2S_PLC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plc_conf0](plc_conf0) module"]
pub type PLC_CONF0 = crate::Reg<u32, _PLC_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLC_CONF0;
#[doc = "`read()` method returns [plc_conf0::R](plc_conf0::R) reader structure"]
impl crate::Readable for PLC_CONF0 {}
#[doc = "`write(|w| ..)` method takes [plc_conf0::W](plc_conf0::W) writer structure"]
impl crate::Writable for PLC_CONF0 {}
#[doc = "I2S_PLC_CONF0"]
pub mod plc_conf0;
#[doc = "I2S_PLC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plc_conf1](plc_conf1) module"]
pub type PLC_CONF1 = crate::Reg<u32, _PLC_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLC_CONF1;
#[doc = "`read()` method returns [plc_conf1::R](plc_conf1::R) reader structure"]
impl crate::Readable for PLC_CONF1 {}
#[doc = "`write(|w| ..)` method takes [plc_conf1::W](plc_conf1::W) writer structure"]
impl crate::Writable for PLC_CONF1 {}
#[doc = "I2S_PLC_CONF1"]
pub mod plc_conf1;
#[doc = "I2S_PLC_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plc_conf2](plc_conf2) module"]
pub type PLC_CONF2 = crate::Reg<u32, _PLC_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLC_CONF2;
#[doc = "`read()` method returns [plc_conf2::R](plc_conf2::R) reader structure"]
impl crate::Readable for PLC_CONF2 {}
#[doc = "`write(|w| ..)` method takes [plc_conf2::W](plc_conf2::W) writer structure"]
impl crate::Writable for PLC_CONF2 {}
#[doc = "I2S_PLC_CONF2"]
pub mod plc_conf2;
#[doc = "I2S_ESCO_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esco_conf0](esco_conf0) module"]
pub type ESCO_CONF0 = crate::Reg<u32, _ESCO_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESCO_CONF0;
#[doc = "`read()` method returns [esco_conf0::R](esco_conf0::R) reader structure"]
impl crate::Readable for ESCO_CONF0 {}
#[doc = "`write(|w| ..)` method takes [esco_conf0::W](esco_conf0::W) writer structure"]
impl crate::Writable for ESCO_CONF0 {}
#[doc = "I2S_ESCO_CONF0"]
pub mod esco_conf0;
#[doc = "I2S_SCO_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sco_conf0](sco_conf0) module"]
pub type SCO_CONF0 = crate::Reg<u32, _SCO_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCO_CONF0;
#[doc = "`read()` method returns [sco_conf0::R](sco_conf0::R) reader structure"]
impl crate::Readable for SCO_CONF0 {}
#[doc = "`write(|w| ..)` method takes [sco_conf0::W](sco_conf0::W) writer structure"]
impl crate::Writable for SCO_CONF0 {}
#[doc = "I2S_SCO_CONF0"]
pub mod sco_conf0;
#[doc = "I2S_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf1](conf1) module"]
pub type CONF1 = crate::Reg<u32, _CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF1;
#[doc = "`read()` method returns [conf1::R](conf1::R) reader structure"]
impl crate::Readable for CONF1 {}
#[doc = "`write(|w| ..)` method takes [conf1::W](conf1::W) writer structure"]
impl crate::Writable for CONF1 {}
#[doc = "I2S_CONF1"]
pub mod conf1;
#[doc = "I2S_PD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_conf](pd_conf) module"]
pub type PD_CONF = crate::Reg<u32, _PD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_CONF;
#[doc = "`read()` method returns [pd_conf::R](pd_conf::R) reader structure"]
impl crate::Readable for PD_CONF {}
#[doc = "`write(|w| ..)` method takes [pd_conf::W](pd_conf::W) writer structure"]
impl crate::Writable for PD_CONF {}
#[doc = "I2S_PD_CONF"]
pub mod pd_conf;
#[doc = "I2S_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf2](conf2) module"]
pub type CONF2 = crate::Reg<u32, _CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF2;
#[doc = "`read()` method returns [conf2::R](conf2::R) reader structure"]
impl crate::Readable for CONF2 {}
#[doc = "`write(|w| ..)` method takes [conf2::W](conf2::W) writer structure"]
impl crate::Writable for CONF2 {}
#[doc = "I2S_CONF2"]
pub mod conf2;
#[doc = "I2S_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkm_conf](clkm_conf) module"]
pub type CLKM_CONF = crate::Reg<u32, _CLKM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKM_CONF;
#[doc = "`read()` method returns [clkm_conf::R](clkm_conf::R) reader structure"]
impl crate::Readable for CLKM_CONF {}
#[doc = "`write(|w| ..)` method takes [clkm_conf::W](clkm_conf::W) writer structure"]
impl crate::Writable for CLKM_CONF {}
#[doc = "I2S_CLKM_CONF"]
pub mod clkm_conf;
#[doc = "I2S_SAMPLE_RATE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sample_rate_conf](sample_rate_conf) module"]
pub type SAMPLE_RATE_CONF = crate::Reg<u32, _SAMPLE_RATE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLE_RATE_CONF;
#[doc = "`read()` method returns [sample_rate_conf::R](sample_rate_conf::R) reader structure"]
impl crate::Readable for SAMPLE_RATE_CONF {}
#[doc = "`write(|w| ..)` method takes [sample_rate_conf::W](sample_rate_conf::W) writer structure"]
impl crate::Writable for SAMPLE_RATE_CONF {}
#[doc = "I2S_SAMPLE_RATE_CONF"]
pub mod sample_rate_conf;
#[doc = "I2S_PDM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdm_conf](pdm_conf) module"]
pub type PDM_CONF = crate::Reg<u32, _PDM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDM_CONF;
#[doc = "`read()` method returns [pdm_conf::R](pdm_conf::R) reader structure"]
impl crate::Readable for PDM_CONF {}
#[doc = "`write(|w| ..)` method takes [pdm_conf::W](pdm_conf::W) writer structure"]
impl crate::Writable for PDM_CONF {}
#[doc = "I2S_PDM_CONF"]
pub mod pdm_conf;
#[doc = "I2S_PDM_FREQ_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdm_freq_conf](pdm_freq_conf) module"]
pub type PDM_FREQ_CONF = crate::Reg<u32, _PDM_FREQ_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDM_FREQ_CONF;
#[doc = "`read()` method returns [pdm_freq_conf::R](pdm_freq_conf::R) reader structure"]
impl crate::Readable for PDM_FREQ_CONF {}
#[doc = "`write(|w| ..)` method takes [pdm_freq_conf::W](pdm_freq_conf::W) writer structure"]
impl crate::Writable for PDM_FREQ_CONF {}
#[doc = "I2S_PDM_FREQ_CONF"]
pub mod pdm_freq_conf;
#[doc = "I2S_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [state](state) module"]
pub type STATE = crate::Reg<u32, _STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE;
#[doc = "`read()` method returns [state::R](state::R) reader structure"]
impl crate::Readable for STATE {}
#[doc = "`write(|w| ..)` method takes [state::W](state::W) writer structure"]
impl crate::Writable for STATE {}
#[doc = "I2S_STATE"]
pub mod state;
#[doc = "I2S_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "I2S_DATE"]
pub mod date;
