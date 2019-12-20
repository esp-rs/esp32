#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - I2S_CONF_REG(i)"]
    pub i2s_conf_regi: I2S_CONF_REGI,
    #[doc = "0x0c - I2S_INT_RAW_REG(i)"]
    pub i2s_int_raw_regi: I2S_INT_RAW_REGI,
    #[doc = "0x10 - I2S_INT_ST_REG(i)"]
    pub i2s_int_st_regi: I2S_INT_ST_REGI,
    #[doc = "0x14 - I2S_INT_ENA_REG(i)"]
    pub i2s_int_ena_regi: I2S_INT_ENA_REGI,
    #[doc = "0x18 - I2S_INT_CLR_REG(i)"]
    pub i2s_int_clr_regi: I2S_INT_CLR_REGI,
    #[doc = "0x1c - I2S_TIMING_REG(i)"]
    pub i2s_timing_regi: I2S_TIMING_REGI,
    #[doc = "0x20 - I2S_FIFO_CONF_REG(i)"]
    pub i2s_fifo_conf_regi: I2S_FIFO_CONF_REGI,
    #[doc = "0x24 - I2S_RXEOF_NUM_REG(i)"]
    pub i2s_rxeof_num_regi: I2S_RXEOF_NUM_REGI,
    #[doc = "0x28 - I2S_CONF_SIGLE_DATA_REG(i)"]
    pub i2s_conf_sigle_data_regi: I2S_CONF_SIGLE_DATA_REGI,
    #[doc = "0x2c - I2S_CONF_CHAN_REG(i)"]
    pub i2s_conf_chan_regi: I2S_CONF_CHAN_REGI,
    #[doc = "0x30 - I2S_OUT_LINK_REG(i)"]
    pub i2s_out_link_regi: I2S_OUT_LINK_REGI,
    #[doc = "0x34 - I2S_IN_LINK_REG(i)"]
    pub i2s_in_link_regi: I2S_IN_LINK_REGI,
    #[doc = "0x38 - I2S_OUT_EOF_DES_ADDR_REG(i)"]
    pub i2s_out_eof_des_addr_regi: I2S_OUT_EOF_DES_ADDR_REGI,
    #[doc = "0x3c - I2S_IN_EOF_DES_ADDR_REG(i)"]
    pub i2s_in_eof_des_addr_regi: I2S_IN_EOF_DES_ADDR_REGI,
    #[doc = "0x40 - I2S_OUT_EOF_BFR_DES_ADDR_REG(i)"]
    pub i2s_out_eof_bfr_des_addr_regi: I2S_OUT_EOF_BFR_DES_ADDR_REGI,
    #[doc = "0x44 - I2S_AHB_TEST_REG(i)"]
    pub i2s_ahb_test_regi: I2S_AHB_TEST_REGI,
    #[doc = "0x48 - I2S_INLINK_DSCR_REG(i)"]
    pub i2s_inlink_dscr_regi: I2S_INLINK_DSCR_REGI,
    #[doc = "0x4c - I2S_INLINK_DSCR_BF0_REG(i)"]
    pub i2s_inlink_dscr_bf0_regi: I2S_INLINK_DSCR_BF0_REGI,
    #[doc = "0x50 - I2S_INLINK_DSCR_BF1_REG(i)"]
    pub i2s_inlink_dscr_bf1_regi: I2S_INLINK_DSCR_BF1_REGI,
    #[doc = "0x54 - I2S_OUTLINK_DSCR_REG(i)"]
    pub i2s_outlink_dscr_regi: I2S_OUTLINK_DSCR_REGI,
    #[doc = "0x58 - I2S_OUTLINK_DSCR_BF0_REG(i)"]
    pub i2s_outlink_dscr_bf0_regi: I2S_OUTLINK_DSCR_BF0_REGI,
    #[doc = "0x5c - I2S_OUTLINK_DSCR_BF1_REG(i)"]
    pub i2s_outlink_dscr_bf1_regi: I2S_OUTLINK_DSCR_BF1_REGI,
    #[doc = "0x60 - I2S_LC_CONF_REG(i)"]
    pub i2s_lc_conf_regi: I2S_LC_CONF_REGI,
    #[doc = "0x64 - I2S_OUTFIFO_PUSH_REG(i)"]
    pub i2s_outfifo_push_regi: I2S_OUTFIFO_PUSH_REGI,
    #[doc = "0x68 - I2S_INFIFO_POP_REG(i)"]
    pub i2s_infifo_pop_regi: I2S_INFIFO_POP_REGI,
    #[doc = "0x6c - I2S_LC_STATE0_REG(i)"]
    pub i2s_lc_state0_regi: I2S_LC_STATE0_REGI,
    #[doc = "0x70 - I2S_LC_STATE1_REG(i)"]
    pub i2s_lc_state1_regi: I2S_LC_STATE1_REGI,
    #[doc = "0x74 - I2S_LC_HUNG_CONF_REG(i)"]
    pub i2s_lc_hung_conf_regi: I2S_LC_HUNG_CONF_REGI,
    _reserved28: [u8; 8usize],
    #[doc = "0x80 - I2S_CVSD_CONF0_REG(i)"]
    pub i2s_cvsd_conf0_regi: I2S_CVSD_CONF0_REGI,
    #[doc = "0x84 - I2S_CVSD_CONF1_REG(i)"]
    pub i2s_cvsd_conf1_regi: I2S_CVSD_CONF1_REGI,
    #[doc = "0x88 - I2S_CVSD_CONF2_REG(i)"]
    pub i2s_cvsd_conf2_regi: I2S_CVSD_CONF2_REGI,
    #[doc = "0x8c - I2S_PLC_CONF0_REG(i)"]
    pub i2s_plc_conf0_regi: I2S_PLC_CONF0_REGI,
    #[doc = "0x90 - I2S_PLC_CONF1_REG(i)"]
    pub i2s_plc_conf1_regi: I2S_PLC_CONF1_REGI,
    #[doc = "0x94 - I2S_PLC_CONF2_REG(i)"]
    pub i2s_plc_conf2_regi: I2S_PLC_CONF2_REGI,
    #[doc = "0x98 - I2S_ESCO_CONF0_REG(i)"]
    pub i2s_esco_conf0_regi: I2S_ESCO_CONF0_REGI,
    #[doc = "0x9c - I2S_SCO_CONF0_REG(i)"]
    pub i2s_sco_conf0_regi: I2S_SCO_CONF0_REGI,
    #[doc = "0xa0 - I2S_CONF1_REG(i)"]
    pub i2s_conf1_regi: I2S_CONF1_REGI,
    #[doc = "0xa4 - I2S_PD_CONF_REG(i)"]
    pub i2s_pd_conf_regi: I2S_PD_CONF_REGI,
    #[doc = "0xa8 - I2S_CONF2_REG(i)"]
    pub i2s_conf2_regi: I2S_CONF2_REGI,
    #[doc = "0xac - I2S_CLKM_CONF_REG(i)"]
    pub i2s_clkm_conf_regi: I2S_CLKM_CONF_REGI,
    #[doc = "0xb0 - I2S_SAMPLE_RATE_CONF_REG(i)"]
    pub i2s_sample_rate_conf_regi: I2S_SAMPLE_RATE_CONF_REGI,
    #[doc = "0xb4 - I2S_PDM_CONF_REG(i)"]
    pub i2s_pdm_conf_regi: I2S_PDM_CONF_REGI,
    #[doc = "0xb8 - I2S_PDM_FREQ_CONF_REG(i)"]
    pub i2s_pdm_freq_conf_regi: I2S_PDM_FREQ_CONF_REGI,
    #[doc = "0xbc - I2S_STATE_REG(i)"]
    pub i2s_state_regi: I2S_STATE_REGI,
    _reserved44: [u8; 60usize],
    #[doc = "0xfc - I2S_DATE_REG(i)"]
    pub i2s_date_regi: I2S_DATE_REGI,
}
#[doc = "I2S_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf_regi](i2s_conf_regi) module"]
pub type I2S_CONF_REGI = crate::Reg<u32, _I2S_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_REGI;
#[doc = "`read()` method returns [i2s_conf_regi::R](i2s_conf_regi::R) reader structure"]
impl crate::Readable for I2S_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_regi::W](i2s_conf_regi::W) writer structure"]
impl crate::Writable for I2S_CONF_REGI {}
#[doc = "I2S_CONF_REG(i)"]
pub mod i2s_conf_regi;
#[doc = "I2S_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_raw_regi](i2s_int_raw_regi) module"]
pub type I2S_INT_RAW_REGI = crate::Reg<u32, _I2S_INT_RAW_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_RAW_REGI;
#[doc = "`read()` method returns [i2s_int_raw_regi::R](i2s_int_raw_regi::R) reader structure"]
impl crate::Readable for I2S_INT_RAW_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_int_raw_regi::W](i2s_int_raw_regi::W) writer structure"]
impl crate::Writable for I2S_INT_RAW_REGI {}
#[doc = "I2S_INT_RAW_REG(i)"]
pub mod i2s_int_raw_regi;
#[doc = "I2S_INT_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_st_regi](i2s_int_st_regi) module"]
pub type I2S_INT_ST_REGI = crate::Reg<u32, _I2S_INT_ST_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_ST_REGI;
#[doc = "`read()` method returns [i2s_int_st_regi::R](i2s_int_st_regi::R) reader structure"]
impl crate::Readable for I2S_INT_ST_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_int_st_regi::W](i2s_int_st_regi::W) writer structure"]
impl crate::Writable for I2S_INT_ST_REGI {}
#[doc = "I2S_INT_ST_REG(i)"]
pub mod i2s_int_st_regi;
#[doc = "I2S_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_ena_regi](i2s_int_ena_regi) module"]
pub type I2S_INT_ENA_REGI = crate::Reg<u32, _I2S_INT_ENA_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_ENA_REGI;
#[doc = "`read()` method returns [i2s_int_ena_regi::R](i2s_int_ena_regi::R) reader structure"]
impl crate::Readable for I2S_INT_ENA_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_int_ena_regi::W](i2s_int_ena_regi::W) writer structure"]
impl crate::Writable for I2S_INT_ENA_REGI {}
#[doc = "I2S_INT_ENA_REG(i)"]
pub mod i2s_int_ena_regi;
#[doc = "I2S_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_clr_regi](i2s_int_clr_regi) module"]
pub type I2S_INT_CLR_REGI = crate::Reg<u32, _I2S_INT_CLR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_CLR_REGI;
#[doc = "`read()` method returns [i2s_int_clr_regi::R](i2s_int_clr_regi::R) reader structure"]
impl crate::Readable for I2S_INT_CLR_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_int_clr_regi::W](i2s_int_clr_regi::W) writer structure"]
impl crate::Writable for I2S_INT_CLR_REGI {}
#[doc = "I2S_INT_CLR_REG(i)"]
pub mod i2s_int_clr_regi;
#[doc = "I2S_TIMING_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_timing_regi](i2s_timing_regi) module"]
pub type I2S_TIMING_REGI = crate::Reg<u32, _I2S_TIMING_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TIMING_REGI;
#[doc = "`read()` method returns [i2s_timing_regi::R](i2s_timing_regi::R) reader structure"]
impl crate::Readable for I2S_TIMING_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_timing_regi::W](i2s_timing_regi::W) writer structure"]
impl crate::Writable for I2S_TIMING_REGI {}
#[doc = "I2S_TIMING_REG(i)"]
pub mod i2s_timing_regi;
#[doc = "I2S_FIFO_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_fifo_conf_regi](i2s_fifo_conf_regi) module"]
pub type I2S_FIFO_CONF_REGI = crate::Reg<u32, _I2S_FIFO_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_FIFO_CONF_REGI;
#[doc = "`read()` method returns [i2s_fifo_conf_regi::R](i2s_fifo_conf_regi::R) reader structure"]
impl crate::Readable for I2S_FIFO_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_fifo_conf_regi::W](i2s_fifo_conf_regi::W) writer structure"]
impl crate::Writable for I2S_FIFO_CONF_REGI {}
#[doc = "I2S_FIFO_CONF_REG(i)"]
pub mod i2s_fifo_conf_regi;
#[doc = "I2S_RXEOF_NUM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_rxeof_num_regi](i2s_rxeof_num_regi) module"]
pub type I2S_RXEOF_NUM_REGI = crate::Reg<u32, _I2S_RXEOF_NUM_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RXEOF_NUM_REGI;
#[doc = "`read()` method returns [i2s_rxeof_num_regi::R](i2s_rxeof_num_regi::R) reader structure"]
impl crate::Readable for I2S_RXEOF_NUM_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_rxeof_num_regi::W](i2s_rxeof_num_regi::W) writer structure"]
impl crate::Writable for I2S_RXEOF_NUM_REGI {}
#[doc = "I2S_RXEOF_NUM_REG(i)"]
pub mod i2s_rxeof_num_regi;
#[doc = "I2S_CONF_SIGLE_DATA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf_sigle_data_regi](i2s_conf_sigle_data_regi) module"]
pub type I2S_CONF_SIGLE_DATA_REGI = crate::Reg<u32, _I2S_CONF_SIGLE_DATA_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_SIGLE_DATA_REGI;
#[doc = "`read()` method returns [i2s_conf_sigle_data_regi::R](i2s_conf_sigle_data_regi::R) reader structure"]
impl crate::Readable for I2S_CONF_SIGLE_DATA_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_sigle_data_regi::W](i2s_conf_sigle_data_regi::W) writer structure"]
impl crate::Writable for I2S_CONF_SIGLE_DATA_REGI {}
#[doc = "I2S_CONF_SIGLE_DATA_REG(i)"]
pub mod i2s_conf_sigle_data_regi;
#[doc = "I2S_CONF_CHAN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf_chan_regi](i2s_conf_chan_regi) module"]
pub type I2S_CONF_CHAN_REGI = crate::Reg<u32, _I2S_CONF_CHAN_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_CHAN_REGI;
#[doc = "`read()` method returns [i2s_conf_chan_regi::R](i2s_conf_chan_regi::R) reader structure"]
impl crate::Readable for I2S_CONF_CHAN_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_chan_regi::W](i2s_conf_chan_regi::W) writer structure"]
impl crate::Writable for I2S_CONF_CHAN_REGI {}
#[doc = "I2S_CONF_CHAN_REG(i)"]
pub mod i2s_conf_chan_regi;
#[doc = "I2S_OUT_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_out_link_regi](i2s_out_link_regi) module"]
pub type I2S_OUT_LINK_REGI = crate::Reg<u32, _I2S_OUT_LINK_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUT_LINK_REGI;
#[doc = "`read()` method returns [i2s_out_link_regi::R](i2s_out_link_regi::R) reader structure"]
impl crate::Readable for I2S_OUT_LINK_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_out_link_regi::W](i2s_out_link_regi::W) writer structure"]
impl crate::Writable for I2S_OUT_LINK_REGI {}
#[doc = "I2S_OUT_LINK_REG(i)"]
pub mod i2s_out_link_regi;
#[doc = "I2S_IN_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_in_link_regi](i2s_in_link_regi) module"]
pub type I2S_IN_LINK_REGI = crate::Reg<u32, _I2S_IN_LINK_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_IN_LINK_REGI;
#[doc = "`read()` method returns [i2s_in_link_regi::R](i2s_in_link_regi::R) reader structure"]
impl crate::Readable for I2S_IN_LINK_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_in_link_regi::W](i2s_in_link_regi::W) writer structure"]
impl crate::Writable for I2S_IN_LINK_REGI {}
#[doc = "I2S_IN_LINK_REG(i)"]
pub mod i2s_in_link_regi;
#[doc = "I2S_OUT_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_out_eof_des_addr_regi](i2s_out_eof_des_addr_regi) module"]
pub type I2S_OUT_EOF_DES_ADDR_REGI = crate::Reg<u32, _I2S_OUT_EOF_DES_ADDR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUT_EOF_DES_ADDR_REGI;
#[doc = "`read()` method returns [i2s_out_eof_des_addr_regi::R](i2s_out_eof_des_addr_regi::R) reader structure"]
impl crate::Readable for I2S_OUT_EOF_DES_ADDR_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_out_eof_des_addr_regi::W](i2s_out_eof_des_addr_regi::W) writer structure"]
impl crate::Writable for I2S_OUT_EOF_DES_ADDR_REGI {}
#[doc = "I2S_OUT_EOF_DES_ADDR_REG(i)"]
pub mod i2s_out_eof_des_addr_regi;
#[doc = "I2S_IN_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_in_eof_des_addr_regi](i2s_in_eof_des_addr_regi) module"]
pub type I2S_IN_EOF_DES_ADDR_REGI = crate::Reg<u32, _I2S_IN_EOF_DES_ADDR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_IN_EOF_DES_ADDR_REGI;
#[doc = "`read()` method returns [i2s_in_eof_des_addr_regi::R](i2s_in_eof_des_addr_regi::R) reader structure"]
impl crate::Readable for I2S_IN_EOF_DES_ADDR_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_in_eof_des_addr_regi::W](i2s_in_eof_des_addr_regi::W) writer structure"]
impl crate::Writable for I2S_IN_EOF_DES_ADDR_REGI {}
#[doc = "I2S_IN_EOF_DES_ADDR_REG(i)"]
pub mod i2s_in_eof_des_addr_regi;
#[doc = "I2S_OUT_EOF_BFR_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_out_eof_bfr_des_addr_regi](i2s_out_eof_bfr_des_addr_regi) module"]
pub type I2S_OUT_EOF_BFR_DES_ADDR_REGI = crate::Reg<u32, _I2S_OUT_EOF_BFR_DES_ADDR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUT_EOF_BFR_DES_ADDR_REGI;
#[doc = "`read()` method returns [i2s_out_eof_bfr_des_addr_regi::R](i2s_out_eof_bfr_des_addr_regi::R) reader structure"]
impl crate::Readable for I2S_OUT_EOF_BFR_DES_ADDR_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_out_eof_bfr_des_addr_regi::W](i2s_out_eof_bfr_des_addr_regi::W) writer structure"]
impl crate::Writable for I2S_OUT_EOF_BFR_DES_ADDR_REGI {}
#[doc = "I2S_OUT_EOF_BFR_DES_ADDR_REG(i)"]
pub mod i2s_out_eof_bfr_des_addr_regi;
#[doc = "I2S_AHB_TEST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_ahb_test_regi](i2s_ahb_test_regi) module"]
pub type I2S_AHB_TEST_REGI = crate::Reg<u32, _I2S_AHB_TEST_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_AHB_TEST_REGI;
#[doc = "`read()` method returns [i2s_ahb_test_regi::R](i2s_ahb_test_regi::R) reader structure"]
impl crate::Readable for I2S_AHB_TEST_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_ahb_test_regi::W](i2s_ahb_test_regi::W) writer structure"]
impl crate::Writable for I2S_AHB_TEST_REGI {}
#[doc = "I2S_AHB_TEST_REG(i)"]
pub mod i2s_ahb_test_regi;
#[doc = "I2S_INLINK_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_inlink_dscr_regi](i2s_inlink_dscr_regi) module"]
pub type I2S_INLINK_DSCR_REGI = crate::Reg<u32, _I2S_INLINK_DSCR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INLINK_DSCR_REGI;
#[doc = "`read()` method returns [i2s_inlink_dscr_regi::R](i2s_inlink_dscr_regi::R) reader structure"]
impl crate::Readable for I2S_INLINK_DSCR_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_inlink_dscr_regi::W](i2s_inlink_dscr_regi::W) writer structure"]
impl crate::Writable for I2S_INLINK_DSCR_REGI {}
#[doc = "I2S_INLINK_DSCR_REG(i)"]
pub mod i2s_inlink_dscr_regi;
#[doc = "I2S_INLINK_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_inlink_dscr_bf0_regi](i2s_inlink_dscr_bf0_regi) module"]
pub type I2S_INLINK_DSCR_BF0_REGI = crate::Reg<u32, _I2S_INLINK_DSCR_BF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INLINK_DSCR_BF0_REGI;
#[doc = "`read()` method returns [i2s_inlink_dscr_bf0_regi::R](i2s_inlink_dscr_bf0_regi::R) reader structure"]
impl crate::Readable for I2S_INLINK_DSCR_BF0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_inlink_dscr_bf0_regi::W](i2s_inlink_dscr_bf0_regi::W) writer structure"]
impl crate::Writable for I2S_INLINK_DSCR_BF0_REGI {}
#[doc = "I2S_INLINK_DSCR_BF0_REG(i)"]
pub mod i2s_inlink_dscr_bf0_regi;
#[doc = "I2S_INLINK_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_inlink_dscr_bf1_regi](i2s_inlink_dscr_bf1_regi) module"]
pub type I2S_INLINK_DSCR_BF1_REGI = crate::Reg<u32, _I2S_INLINK_DSCR_BF1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INLINK_DSCR_BF1_REGI;
#[doc = "`read()` method returns [i2s_inlink_dscr_bf1_regi::R](i2s_inlink_dscr_bf1_regi::R) reader structure"]
impl crate::Readable for I2S_INLINK_DSCR_BF1_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_inlink_dscr_bf1_regi::W](i2s_inlink_dscr_bf1_regi::W) writer structure"]
impl crate::Writable for I2S_INLINK_DSCR_BF1_REGI {}
#[doc = "I2S_INLINK_DSCR_BF1_REG(i)"]
pub mod i2s_inlink_dscr_bf1_regi;
#[doc = "I2S_OUTLINK_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outlink_dscr_regi](i2s_outlink_dscr_regi) module"]
pub type I2S_OUTLINK_DSCR_REGI = crate::Reg<u32, _I2S_OUTLINK_DSCR_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTLINK_DSCR_REGI;
#[doc = "`read()` method returns [i2s_outlink_dscr_regi::R](i2s_outlink_dscr_regi::R) reader structure"]
impl crate::Readable for I2S_OUTLINK_DSCR_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_outlink_dscr_regi::W](i2s_outlink_dscr_regi::W) writer structure"]
impl crate::Writable for I2S_OUTLINK_DSCR_REGI {}
#[doc = "I2S_OUTLINK_DSCR_REG(i)"]
pub mod i2s_outlink_dscr_regi;
#[doc = "I2S_OUTLINK_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outlink_dscr_bf0_regi](i2s_outlink_dscr_bf0_regi) module"]
pub type I2S_OUTLINK_DSCR_BF0_REGI = crate::Reg<u32, _I2S_OUTLINK_DSCR_BF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTLINK_DSCR_BF0_REGI;
#[doc = "`read()` method returns [i2s_outlink_dscr_bf0_regi::R](i2s_outlink_dscr_bf0_regi::R) reader structure"]
impl crate::Readable for I2S_OUTLINK_DSCR_BF0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_outlink_dscr_bf0_regi::W](i2s_outlink_dscr_bf0_regi::W) writer structure"]
impl crate::Writable for I2S_OUTLINK_DSCR_BF0_REGI {}
#[doc = "I2S_OUTLINK_DSCR_BF0_REG(i)"]
pub mod i2s_outlink_dscr_bf0_regi;
#[doc = "I2S_OUTLINK_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outlink_dscr_bf1_regi](i2s_outlink_dscr_bf1_regi) module"]
pub type I2S_OUTLINK_DSCR_BF1_REGI = crate::Reg<u32, _I2S_OUTLINK_DSCR_BF1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTLINK_DSCR_BF1_REGI;
#[doc = "`read()` method returns [i2s_outlink_dscr_bf1_regi::R](i2s_outlink_dscr_bf1_regi::R) reader structure"]
impl crate::Readable for I2S_OUTLINK_DSCR_BF1_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_outlink_dscr_bf1_regi::W](i2s_outlink_dscr_bf1_regi::W) writer structure"]
impl crate::Writable for I2S_OUTLINK_DSCR_BF1_REGI {}
#[doc = "I2S_OUTLINK_DSCR_BF1_REG(i)"]
pub mod i2s_outlink_dscr_bf1_regi;
#[doc = "I2S_LC_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_conf_regi](i2s_lc_conf_regi) module"]
pub type I2S_LC_CONF_REGI = crate::Reg<u32, _I2S_LC_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_CONF_REGI;
#[doc = "`read()` method returns [i2s_lc_conf_regi::R](i2s_lc_conf_regi::R) reader structure"]
impl crate::Readable for I2S_LC_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_conf_regi::W](i2s_lc_conf_regi::W) writer structure"]
impl crate::Writable for I2S_LC_CONF_REGI {}
#[doc = "I2S_LC_CONF_REG(i)"]
pub mod i2s_lc_conf_regi;
#[doc = "I2S_OUTFIFO_PUSH_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outfifo_push_regi](i2s_outfifo_push_regi) module"]
pub type I2S_OUTFIFO_PUSH_REGI = crate::Reg<u32, _I2S_OUTFIFO_PUSH_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTFIFO_PUSH_REGI;
#[doc = "`read()` method returns [i2s_outfifo_push_regi::R](i2s_outfifo_push_regi::R) reader structure"]
impl crate::Readable for I2S_OUTFIFO_PUSH_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_outfifo_push_regi::W](i2s_outfifo_push_regi::W) writer structure"]
impl crate::Writable for I2S_OUTFIFO_PUSH_REGI {}
#[doc = "I2S_OUTFIFO_PUSH_REG(i)"]
pub mod i2s_outfifo_push_regi;
#[doc = "I2S_INFIFO_POP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_infifo_pop_regi](i2s_infifo_pop_regi) module"]
pub type I2S_INFIFO_POP_REGI = crate::Reg<u32, _I2S_INFIFO_POP_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INFIFO_POP_REGI;
#[doc = "`read()` method returns [i2s_infifo_pop_regi::R](i2s_infifo_pop_regi::R) reader structure"]
impl crate::Readable for I2S_INFIFO_POP_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_infifo_pop_regi::W](i2s_infifo_pop_regi::W) writer structure"]
impl crate::Writable for I2S_INFIFO_POP_REGI {}
#[doc = "I2S_INFIFO_POP_REG(i)"]
pub mod i2s_infifo_pop_regi;
#[doc = "I2S_LC_STATE0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_state0_regi](i2s_lc_state0_regi) module"]
pub type I2S_LC_STATE0_REGI = crate::Reg<u32, _I2S_LC_STATE0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_STATE0_REGI;
#[doc = "`read()` method returns [i2s_lc_state0_regi::R](i2s_lc_state0_regi::R) reader structure"]
impl crate::Readable for I2S_LC_STATE0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_state0_regi::W](i2s_lc_state0_regi::W) writer structure"]
impl crate::Writable for I2S_LC_STATE0_REGI {}
#[doc = "I2S_LC_STATE0_REG(i)"]
pub mod i2s_lc_state0_regi;
#[doc = "I2S_LC_STATE1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_state1_regi](i2s_lc_state1_regi) module"]
pub type I2S_LC_STATE1_REGI = crate::Reg<u32, _I2S_LC_STATE1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_STATE1_REGI;
#[doc = "`read()` method returns [i2s_lc_state1_regi::R](i2s_lc_state1_regi::R) reader structure"]
impl crate::Readable for I2S_LC_STATE1_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_state1_regi::W](i2s_lc_state1_regi::W) writer structure"]
impl crate::Writable for I2S_LC_STATE1_REGI {}
#[doc = "I2S_LC_STATE1_REG(i)"]
pub mod i2s_lc_state1_regi;
#[doc = "I2S_LC_HUNG_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_hung_conf_regi](i2s_lc_hung_conf_regi) module"]
pub type I2S_LC_HUNG_CONF_REGI = crate::Reg<u32, _I2S_LC_HUNG_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_HUNG_CONF_REGI;
#[doc = "`read()` method returns [i2s_lc_hung_conf_regi::R](i2s_lc_hung_conf_regi::R) reader structure"]
impl crate::Readable for I2S_LC_HUNG_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_hung_conf_regi::W](i2s_lc_hung_conf_regi::W) writer structure"]
impl crate::Writable for I2S_LC_HUNG_CONF_REGI {}
#[doc = "I2S_LC_HUNG_CONF_REG(i)"]
pub mod i2s_lc_hung_conf_regi;
#[doc = "I2S_CVSD_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_cvsd_conf0_regi](i2s_cvsd_conf0_regi) module"]
pub type I2S_CVSD_CONF0_REGI = crate::Reg<u32, _I2S_CVSD_CONF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CVSD_CONF0_REGI;
#[doc = "`read()` method returns [i2s_cvsd_conf0_regi::R](i2s_cvsd_conf0_regi::R) reader structure"]
impl crate::Readable for I2S_CVSD_CONF0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_cvsd_conf0_regi::W](i2s_cvsd_conf0_regi::W) writer structure"]
impl crate::Writable for I2S_CVSD_CONF0_REGI {}
#[doc = "I2S_CVSD_CONF0_REG(i)"]
pub mod i2s_cvsd_conf0_regi;
#[doc = "I2S_CVSD_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_cvsd_conf1_regi](i2s_cvsd_conf1_regi) module"]
pub type I2S_CVSD_CONF1_REGI = crate::Reg<u32, _I2S_CVSD_CONF1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CVSD_CONF1_REGI;
#[doc = "`read()` method returns [i2s_cvsd_conf1_regi::R](i2s_cvsd_conf1_regi::R) reader structure"]
impl crate::Readable for I2S_CVSD_CONF1_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_cvsd_conf1_regi::W](i2s_cvsd_conf1_regi::W) writer structure"]
impl crate::Writable for I2S_CVSD_CONF1_REGI {}
#[doc = "I2S_CVSD_CONF1_REG(i)"]
pub mod i2s_cvsd_conf1_regi;
#[doc = "I2S_CVSD_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_cvsd_conf2_regi](i2s_cvsd_conf2_regi) module"]
pub type I2S_CVSD_CONF2_REGI = crate::Reg<u32, _I2S_CVSD_CONF2_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CVSD_CONF2_REGI;
#[doc = "`read()` method returns [i2s_cvsd_conf2_regi::R](i2s_cvsd_conf2_regi::R) reader structure"]
impl crate::Readable for I2S_CVSD_CONF2_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_cvsd_conf2_regi::W](i2s_cvsd_conf2_regi::W) writer structure"]
impl crate::Writable for I2S_CVSD_CONF2_REGI {}
#[doc = "I2S_CVSD_CONF2_REG(i)"]
pub mod i2s_cvsd_conf2_regi;
#[doc = "I2S_PLC_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_plc_conf0_regi](i2s_plc_conf0_regi) module"]
pub type I2S_PLC_CONF0_REGI = crate::Reg<u32, _I2S_PLC_CONF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PLC_CONF0_REGI;
#[doc = "`read()` method returns [i2s_plc_conf0_regi::R](i2s_plc_conf0_regi::R) reader structure"]
impl crate::Readable for I2S_PLC_CONF0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_plc_conf0_regi::W](i2s_plc_conf0_regi::W) writer structure"]
impl crate::Writable for I2S_PLC_CONF0_REGI {}
#[doc = "I2S_PLC_CONF0_REG(i)"]
pub mod i2s_plc_conf0_regi;
#[doc = "I2S_PLC_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_plc_conf1_regi](i2s_plc_conf1_regi) module"]
pub type I2S_PLC_CONF1_REGI = crate::Reg<u32, _I2S_PLC_CONF1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PLC_CONF1_REGI;
#[doc = "`read()` method returns [i2s_plc_conf1_regi::R](i2s_plc_conf1_regi::R) reader structure"]
impl crate::Readable for I2S_PLC_CONF1_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_plc_conf1_regi::W](i2s_plc_conf1_regi::W) writer structure"]
impl crate::Writable for I2S_PLC_CONF1_REGI {}
#[doc = "I2S_PLC_CONF1_REG(i)"]
pub mod i2s_plc_conf1_regi;
#[doc = "I2S_PLC_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_plc_conf2_regi](i2s_plc_conf2_regi) module"]
pub type I2S_PLC_CONF2_REGI = crate::Reg<u32, _I2S_PLC_CONF2_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PLC_CONF2_REGI;
#[doc = "`read()` method returns [i2s_plc_conf2_regi::R](i2s_plc_conf2_regi::R) reader structure"]
impl crate::Readable for I2S_PLC_CONF2_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_plc_conf2_regi::W](i2s_plc_conf2_regi::W) writer structure"]
impl crate::Writable for I2S_PLC_CONF2_REGI {}
#[doc = "I2S_PLC_CONF2_REG(i)"]
pub mod i2s_plc_conf2_regi;
#[doc = "I2S_ESCO_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_esco_conf0_regi](i2s_esco_conf0_regi) module"]
pub type I2S_ESCO_CONF0_REGI = crate::Reg<u32, _I2S_ESCO_CONF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_ESCO_CONF0_REGI;
#[doc = "`read()` method returns [i2s_esco_conf0_regi::R](i2s_esco_conf0_regi::R) reader structure"]
impl crate::Readable for I2S_ESCO_CONF0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_esco_conf0_regi::W](i2s_esco_conf0_regi::W) writer structure"]
impl crate::Writable for I2S_ESCO_CONF0_REGI {}
#[doc = "I2S_ESCO_CONF0_REG(i)"]
pub mod i2s_esco_conf0_regi;
#[doc = "I2S_SCO_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_sco_conf0_regi](i2s_sco_conf0_regi) module"]
pub type I2S_SCO_CONF0_REGI = crate::Reg<u32, _I2S_SCO_CONF0_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_SCO_CONF0_REGI;
#[doc = "`read()` method returns [i2s_sco_conf0_regi::R](i2s_sco_conf0_regi::R) reader structure"]
impl crate::Readable for I2S_SCO_CONF0_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_sco_conf0_regi::W](i2s_sco_conf0_regi::W) writer structure"]
impl crate::Writable for I2S_SCO_CONF0_REGI {}
#[doc = "I2S_SCO_CONF0_REG(i)"]
pub mod i2s_sco_conf0_regi;
#[doc = "I2S_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf1_regi](i2s_conf1_regi) module"]
pub type I2S_CONF1_REGI = crate::Reg<u32, _I2S_CONF1_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF1_REGI;
#[doc = "`read()` method returns [i2s_conf1_regi::R](i2s_conf1_regi::R) reader structure"]
impl crate::Readable for I2S_CONF1_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_conf1_regi::W](i2s_conf1_regi::W) writer structure"]
impl crate::Writable for I2S_CONF1_REGI {}
#[doc = "I2S_CONF1_REG(i)"]
pub mod i2s_conf1_regi;
#[doc = "I2S_PD_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_pd_conf_regi](i2s_pd_conf_regi) module"]
pub type I2S_PD_CONF_REGI = crate::Reg<u32, _I2S_PD_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PD_CONF_REGI;
#[doc = "`read()` method returns [i2s_pd_conf_regi::R](i2s_pd_conf_regi::R) reader structure"]
impl crate::Readable for I2S_PD_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_pd_conf_regi::W](i2s_pd_conf_regi::W) writer structure"]
impl crate::Writable for I2S_PD_CONF_REGI {}
#[doc = "I2S_PD_CONF_REG(i)"]
pub mod i2s_pd_conf_regi;
#[doc = "I2S_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf2_regi](i2s_conf2_regi) module"]
pub type I2S_CONF2_REGI = crate::Reg<u32, _I2S_CONF2_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF2_REGI;
#[doc = "`read()` method returns [i2s_conf2_regi::R](i2s_conf2_regi::R) reader structure"]
impl crate::Readable for I2S_CONF2_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_conf2_regi::W](i2s_conf2_regi::W) writer structure"]
impl crate::Writable for I2S_CONF2_REGI {}
#[doc = "I2S_CONF2_REG(i)"]
pub mod i2s_conf2_regi;
#[doc = "I2S_CLKM_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_clkm_conf_regi](i2s_clkm_conf_regi) module"]
pub type I2S_CLKM_CONF_REGI = crate::Reg<u32, _I2S_CLKM_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CLKM_CONF_REGI;
#[doc = "`read()` method returns [i2s_clkm_conf_regi::R](i2s_clkm_conf_regi::R) reader structure"]
impl crate::Readable for I2S_CLKM_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_clkm_conf_regi::W](i2s_clkm_conf_regi::W) writer structure"]
impl crate::Writable for I2S_CLKM_CONF_REGI {}
#[doc = "I2S_CLKM_CONF_REG(i)"]
pub mod i2s_clkm_conf_regi;
#[doc = "I2S_SAMPLE_RATE_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_sample_rate_conf_regi](i2s_sample_rate_conf_regi) module"]
pub type I2S_SAMPLE_RATE_CONF_REGI = crate::Reg<u32, _I2S_SAMPLE_RATE_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_SAMPLE_RATE_CONF_REGI;
#[doc = "`read()` method returns [i2s_sample_rate_conf_regi::R](i2s_sample_rate_conf_regi::R) reader structure"]
impl crate::Readable for I2S_SAMPLE_RATE_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_sample_rate_conf_regi::W](i2s_sample_rate_conf_regi::W) writer structure"]
impl crate::Writable for I2S_SAMPLE_RATE_CONF_REGI {}
#[doc = "I2S_SAMPLE_RATE_CONF_REG(i)"]
pub mod i2s_sample_rate_conf_regi;
#[doc = "I2S_PDM_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_pdm_conf_regi](i2s_pdm_conf_regi) module"]
pub type I2S_PDM_CONF_REGI = crate::Reg<u32, _I2S_PDM_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PDM_CONF_REGI;
#[doc = "`read()` method returns [i2s_pdm_conf_regi::R](i2s_pdm_conf_regi::R) reader structure"]
impl crate::Readable for I2S_PDM_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_pdm_conf_regi::W](i2s_pdm_conf_regi::W) writer structure"]
impl crate::Writable for I2S_PDM_CONF_REGI {}
#[doc = "I2S_PDM_CONF_REG(i)"]
pub mod i2s_pdm_conf_regi;
#[doc = "I2S_PDM_FREQ_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_pdm_freq_conf_regi](i2s_pdm_freq_conf_regi) module"]
pub type I2S_PDM_FREQ_CONF_REGI = crate::Reg<u32, _I2S_PDM_FREQ_CONF_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PDM_FREQ_CONF_REGI;
#[doc = "`read()` method returns [i2s_pdm_freq_conf_regi::R](i2s_pdm_freq_conf_regi::R) reader structure"]
impl crate::Readable for I2S_PDM_FREQ_CONF_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_pdm_freq_conf_regi::W](i2s_pdm_freq_conf_regi::W) writer structure"]
impl crate::Writable for I2S_PDM_FREQ_CONF_REGI {}
#[doc = "I2S_PDM_FREQ_CONF_REG(i)"]
pub mod i2s_pdm_freq_conf_regi;
#[doc = "I2S_STATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_state_regi](i2s_state_regi) module"]
pub type I2S_STATE_REGI = crate::Reg<u32, _I2S_STATE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_STATE_REGI;
#[doc = "`read()` method returns [i2s_state_regi::R](i2s_state_regi::R) reader structure"]
impl crate::Readable for I2S_STATE_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_state_regi::W](i2s_state_regi::W) writer structure"]
impl crate::Writable for I2S_STATE_REGI {}
#[doc = "I2S_STATE_REG(i)"]
pub mod i2s_state_regi;
#[doc = "I2S_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_date_regi](i2s_date_regi) module"]
pub type I2S_DATE_REGI = crate::Reg<u32, _I2S_DATE_REGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_DATE_REGI;
#[doc = "`read()` method returns [i2s_date_regi::R](i2s_date_regi::R) reader structure"]
impl crate::Readable for I2S_DATE_REGI {}
#[doc = "`write(|w| ..)` method takes [i2s_date_regi::W](i2s_date_regi::W) writer structure"]
impl crate::Writable for I2S_DATE_REGI {}
#[doc = "I2S_DATE_REG(i)"]
pub mod i2s_date_regi;
