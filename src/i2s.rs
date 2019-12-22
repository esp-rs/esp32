#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - I2S_CONF_REG(i)"]
    pub i2s_conf_reg: I2S_CONF_REG,
    #[doc = "0x0c - I2S_INT_RAW_REG(i)"]
    pub i2s_int_raw_reg: I2S_INT_RAW_REG,
    #[doc = "0x10 - I2S_INT_ST_REG(i)"]
    pub i2s_int_st_reg: I2S_INT_ST_REG,
    #[doc = "0x14 - I2S_INT_ENA_REG(i)"]
    pub i2s_int_ena_reg: I2S_INT_ENA_REG,
    #[doc = "0x18 - I2S_INT_CLR_REG(i)"]
    pub i2s_int_clr_reg: I2S_INT_CLR_REG,
    #[doc = "0x1c - I2S_TIMING_REG(i)"]
    pub i2s_timing_reg: I2S_TIMING_REG,
    #[doc = "0x20 - I2S_FIFO_CONF_REG(i)"]
    pub i2s_fifo_conf_reg: I2S_FIFO_CONF_REG,
    #[doc = "0x24 - I2S_RXEOF_NUM_REG(i)"]
    pub i2s_rxeof_num_reg: I2S_RXEOF_NUM_REG,
    #[doc = "0x28 - I2S_CONF_SIGLE_DATA_REG(i)"]
    pub i2s_conf_sigle_data_reg: I2S_CONF_SIGLE_DATA_REG,
    #[doc = "0x2c - I2S_CONF_CHAN_REG(i)"]
    pub i2s_conf_chan_reg: I2S_CONF_CHAN_REG,
    #[doc = "0x30 - I2S_OUT_LINK_REG(i)"]
    pub i2s_out_link_reg: I2S_OUT_LINK_REG,
    #[doc = "0x34 - I2S_IN_LINK_REG(i)"]
    pub i2s_in_link_reg: I2S_IN_LINK_REG,
    #[doc = "0x38 - I2S_OUT_EOF_DES_ADDR_REG(i)"]
    pub i2s_out_eof_des_addr_reg: I2S_OUT_EOF_DES_ADDR_REG,
    #[doc = "0x3c - I2S_IN_EOF_DES_ADDR_REG(i)"]
    pub i2s_in_eof_des_addr_reg: I2S_IN_EOF_DES_ADDR_REG,
    #[doc = "0x40 - I2S_OUT_EOF_BFR_DES_ADDR_REG(i)"]
    pub i2s_out_eof_bfr_des_addr_reg: I2S_OUT_EOF_BFR_DES_ADDR_REG,
    #[doc = "0x44 - I2S_AHB_TEST_REG(i)"]
    pub i2s_ahb_test_reg: I2S_AHB_TEST_REG,
    #[doc = "0x48 - I2S_INLINK_DSCR_REG(i)"]
    pub i2s_inlink_dscr_reg: I2S_INLINK_DSCR_REG,
    #[doc = "0x4c - I2S_INLINK_DSCR_BF0_REG(i)"]
    pub i2s_inlink_dscr_bf0_reg: I2S_INLINK_DSCR_BF0_REG,
    #[doc = "0x50 - I2S_INLINK_DSCR_BF1_REG(i)"]
    pub i2s_inlink_dscr_bf1_reg: I2S_INLINK_DSCR_BF1_REG,
    #[doc = "0x54 - I2S_OUTLINK_DSCR_REG(i)"]
    pub i2s_outlink_dscr_reg: I2S_OUTLINK_DSCR_REG,
    #[doc = "0x58 - I2S_OUTLINK_DSCR_BF0_REG(i)"]
    pub i2s_outlink_dscr_bf0_reg: I2S_OUTLINK_DSCR_BF0_REG,
    #[doc = "0x5c - I2S_OUTLINK_DSCR_BF1_REG(i)"]
    pub i2s_outlink_dscr_bf1_reg: I2S_OUTLINK_DSCR_BF1_REG,
    #[doc = "0x60 - I2S_LC_CONF_REG(i)"]
    pub i2s_lc_conf_reg: I2S_LC_CONF_REG,
    #[doc = "0x64 - I2S_OUTFIFO_PUSH_REG(i)"]
    pub i2s_outfifo_push_reg: I2S_OUTFIFO_PUSH_REG,
    #[doc = "0x68 - I2S_INFIFO_POP_REG(i)"]
    pub i2s_infifo_pop_reg: I2S_INFIFO_POP_REG,
    #[doc = "0x6c - I2S_LC_STATE0_REG(i)"]
    pub i2s_lc_state0_reg: I2S_LC_STATE0_REG,
    #[doc = "0x70 - I2S_LC_STATE1_REG(i)"]
    pub i2s_lc_state1_reg: I2S_LC_STATE1_REG,
    #[doc = "0x74 - I2S_LC_HUNG_CONF_REG(i)"]
    pub i2s_lc_hung_conf_reg: I2S_LC_HUNG_CONF_REG,
    _reserved28: [u8; 8usize],
    #[doc = "0x80 - I2S_CVSD_CONF0_REG(i)"]
    pub i2s_cvsd_conf0_reg: I2S_CVSD_CONF0_REG,
    #[doc = "0x84 - I2S_CVSD_CONF1_REG(i)"]
    pub i2s_cvsd_conf1_reg: I2S_CVSD_CONF1_REG,
    #[doc = "0x88 - I2S_CVSD_CONF2_REG(i)"]
    pub i2s_cvsd_conf2_reg: I2S_CVSD_CONF2_REG,
    #[doc = "0x8c - I2S_PLC_CONF0_REG(i)"]
    pub i2s_plc_conf0_reg: I2S_PLC_CONF0_REG,
    #[doc = "0x90 - I2S_PLC_CONF1_REG(i)"]
    pub i2s_plc_conf1_reg: I2S_PLC_CONF1_REG,
    #[doc = "0x94 - I2S_PLC_CONF2_REG(i)"]
    pub i2s_plc_conf2_reg: I2S_PLC_CONF2_REG,
    #[doc = "0x98 - I2S_ESCO_CONF0_REG(i)"]
    pub i2s_esco_conf0_reg: I2S_ESCO_CONF0_REG,
    #[doc = "0x9c - I2S_SCO_CONF0_REG(i)"]
    pub i2s_sco_conf0_reg: I2S_SCO_CONF0_REG,
    #[doc = "0xa0 - I2S_CONF1_REG(i)"]
    pub i2s_conf1_reg: I2S_CONF1_REG,
    #[doc = "0xa4 - I2S_PD_CONF_REG(i)"]
    pub i2s_pd_conf_reg: I2S_PD_CONF_REG,
    #[doc = "0xa8 - I2S_CONF2_REG(i)"]
    pub i2s_conf2_reg: I2S_CONF2_REG,
    #[doc = "0xac - I2S_CLKM_CONF_REG(i)"]
    pub i2s_clkm_conf_reg: I2S_CLKM_CONF_REG,
    #[doc = "0xb0 - I2S_SAMPLE_RATE_CONF_REG(i)"]
    pub i2s_sample_rate_conf_reg: I2S_SAMPLE_RATE_CONF_REG,
    #[doc = "0xb4 - I2S_PDM_CONF_REG(i)"]
    pub i2s_pdm_conf_reg: I2S_PDM_CONF_REG,
    #[doc = "0xb8 - I2S_PDM_FREQ_CONF_REG(i)"]
    pub i2s_pdm_freq_conf_reg: I2S_PDM_FREQ_CONF_REG,
    #[doc = "0xbc - I2S_STATE_REG(i)"]
    pub i2s_state_reg: I2S_STATE_REG,
    _reserved44: [u8; 60usize],
    #[doc = "0xfc - I2S_DATE_REG(i)"]
    pub i2s_date_reg: I2S_DATE_REG,
}
#[doc = "I2S_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf_reg](i2s_conf_reg) module"]
pub type I2S_CONF_REG = crate::Reg<u32, _I2S_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_REG;
#[doc = "`read()` method returns [i2s_conf_reg::R](i2s_conf_reg::R) reader structure"]
impl crate::Readable for I2S_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_reg::W](i2s_conf_reg::W) writer structure"]
impl crate::Writable for I2S_CONF_REG {}
#[doc = "I2S_CONF_REG(i)"]
pub mod i2s_conf_reg;
#[doc = "I2S_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_raw_reg](i2s_int_raw_reg) module"]
pub type I2S_INT_RAW_REG = crate::Reg<u32, _I2S_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_RAW_REG;
#[doc = "`read()` method returns [i2s_int_raw_reg::R](i2s_int_raw_reg::R) reader structure"]
impl crate::Readable for I2S_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_int_raw_reg::W](i2s_int_raw_reg::W) writer structure"]
impl crate::Writable for I2S_INT_RAW_REG {}
#[doc = "I2S_INT_RAW_REG(i)"]
pub mod i2s_int_raw_reg;
#[doc = "I2S_INT_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_st_reg](i2s_int_st_reg) module"]
pub type I2S_INT_ST_REG = crate::Reg<u32, _I2S_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_ST_REG;
#[doc = "`read()` method returns [i2s_int_st_reg::R](i2s_int_st_reg::R) reader structure"]
impl crate::Readable for I2S_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_int_st_reg::W](i2s_int_st_reg::W) writer structure"]
impl crate::Writable for I2S_INT_ST_REG {}
#[doc = "I2S_INT_ST_REG(i)"]
pub mod i2s_int_st_reg;
#[doc = "I2S_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_ena_reg](i2s_int_ena_reg) module"]
pub type I2S_INT_ENA_REG = crate::Reg<u32, _I2S_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_ENA_REG;
#[doc = "`read()` method returns [i2s_int_ena_reg::R](i2s_int_ena_reg::R) reader structure"]
impl crate::Readable for I2S_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_int_ena_reg::W](i2s_int_ena_reg::W) writer structure"]
impl crate::Writable for I2S_INT_ENA_REG {}
#[doc = "I2S_INT_ENA_REG(i)"]
pub mod i2s_int_ena_reg;
#[doc = "I2S_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_int_clr_reg](i2s_int_clr_reg) module"]
pub type I2S_INT_CLR_REG = crate::Reg<u32, _I2S_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_CLR_REG;
#[doc = "`read()` method returns [i2s_int_clr_reg::R](i2s_int_clr_reg::R) reader structure"]
impl crate::Readable for I2S_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_int_clr_reg::W](i2s_int_clr_reg::W) writer structure"]
impl crate::Writable for I2S_INT_CLR_REG {}
#[doc = "I2S_INT_CLR_REG(i)"]
pub mod i2s_int_clr_reg;
#[doc = "I2S_TIMING_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_timing_reg](i2s_timing_reg) module"]
pub type I2S_TIMING_REG = crate::Reg<u32, _I2S_TIMING_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TIMING_REG;
#[doc = "`read()` method returns [i2s_timing_reg::R](i2s_timing_reg::R) reader structure"]
impl crate::Readable for I2S_TIMING_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_timing_reg::W](i2s_timing_reg::W) writer structure"]
impl crate::Writable for I2S_TIMING_REG {}
#[doc = "I2S_TIMING_REG(i)"]
pub mod i2s_timing_reg;
#[doc = "I2S_FIFO_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_fifo_conf_reg](i2s_fifo_conf_reg) module"]
pub type I2S_FIFO_CONF_REG = crate::Reg<u32, _I2S_FIFO_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_FIFO_CONF_REG;
#[doc = "`read()` method returns [i2s_fifo_conf_reg::R](i2s_fifo_conf_reg::R) reader structure"]
impl crate::Readable for I2S_FIFO_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_fifo_conf_reg::W](i2s_fifo_conf_reg::W) writer structure"]
impl crate::Writable for I2S_FIFO_CONF_REG {}
#[doc = "I2S_FIFO_CONF_REG(i)"]
pub mod i2s_fifo_conf_reg;
#[doc = "I2S_RXEOF_NUM_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_rxeof_num_reg](i2s_rxeof_num_reg) module"]
pub type I2S_RXEOF_NUM_REG = crate::Reg<u32, _I2S_RXEOF_NUM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RXEOF_NUM_REG;
#[doc = "`read()` method returns [i2s_rxeof_num_reg::R](i2s_rxeof_num_reg::R) reader structure"]
impl crate::Readable for I2S_RXEOF_NUM_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_rxeof_num_reg::W](i2s_rxeof_num_reg::W) writer structure"]
impl crate::Writable for I2S_RXEOF_NUM_REG {}
#[doc = "I2S_RXEOF_NUM_REG(i)"]
pub mod i2s_rxeof_num_reg;
#[doc = "I2S_CONF_SIGLE_DATA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf_sigle_data_reg](i2s_conf_sigle_data_reg) module"]
pub type I2S_CONF_SIGLE_DATA_REG = crate::Reg<u32, _I2S_CONF_SIGLE_DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_SIGLE_DATA_REG;
#[doc = "`read()` method returns [i2s_conf_sigle_data_reg::R](i2s_conf_sigle_data_reg::R) reader structure"]
impl crate::Readable for I2S_CONF_SIGLE_DATA_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_sigle_data_reg::W](i2s_conf_sigle_data_reg::W) writer structure"]
impl crate::Writable for I2S_CONF_SIGLE_DATA_REG {}
#[doc = "I2S_CONF_SIGLE_DATA_REG(i)"]
pub mod i2s_conf_sigle_data_reg;
#[doc = "I2S_CONF_CHAN_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf_chan_reg](i2s_conf_chan_reg) module"]
pub type I2S_CONF_CHAN_REG = crate::Reg<u32, _I2S_CONF_CHAN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_CHAN_REG;
#[doc = "`read()` method returns [i2s_conf_chan_reg::R](i2s_conf_chan_reg::R) reader structure"]
impl crate::Readable for I2S_CONF_CHAN_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_chan_reg::W](i2s_conf_chan_reg::W) writer structure"]
impl crate::Writable for I2S_CONF_CHAN_REG {}
#[doc = "I2S_CONF_CHAN_REG(i)"]
pub mod i2s_conf_chan_reg;
#[doc = "I2S_OUT_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_out_link_reg](i2s_out_link_reg) module"]
pub type I2S_OUT_LINK_REG = crate::Reg<u32, _I2S_OUT_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUT_LINK_REG;
#[doc = "`read()` method returns [i2s_out_link_reg::R](i2s_out_link_reg::R) reader structure"]
impl crate::Readable for I2S_OUT_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_out_link_reg::W](i2s_out_link_reg::W) writer structure"]
impl crate::Writable for I2S_OUT_LINK_REG {}
#[doc = "I2S_OUT_LINK_REG(i)"]
pub mod i2s_out_link_reg;
#[doc = "I2S_IN_LINK_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_in_link_reg](i2s_in_link_reg) module"]
pub type I2S_IN_LINK_REG = crate::Reg<u32, _I2S_IN_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_IN_LINK_REG;
#[doc = "`read()` method returns [i2s_in_link_reg::R](i2s_in_link_reg::R) reader structure"]
impl crate::Readable for I2S_IN_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_in_link_reg::W](i2s_in_link_reg::W) writer structure"]
impl crate::Writable for I2S_IN_LINK_REG {}
#[doc = "I2S_IN_LINK_REG(i)"]
pub mod i2s_in_link_reg;
#[doc = "I2S_OUT_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_out_eof_des_addr_reg](i2s_out_eof_des_addr_reg) module"]
pub type I2S_OUT_EOF_DES_ADDR_REG = crate::Reg<u32, _I2S_OUT_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUT_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [i2s_out_eof_des_addr_reg::R](i2s_out_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for I2S_OUT_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_out_eof_des_addr_reg::W](i2s_out_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for I2S_OUT_EOF_DES_ADDR_REG {}
#[doc = "I2S_OUT_EOF_DES_ADDR_REG(i)"]
pub mod i2s_out_eof_des_addr_reg;
#[doc = "I2S_IN_EOF_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_in_eof_des_addr_reg](i2s_in_eof_des_addr_reg) module"]
pub type I2S_IN_EOF_DES_ADDR_REG = crate::Reg<u32, _I2S_IN_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_IN_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [i2s_in_eof_des_addr_reg::R](i2s_in_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for I2S_IN_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_in_eof_des_addr_reg::W](i2s_in_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for I2S_IN_EOF_DES_ADDR_REG {}
#[doc = "I2S_IN_EOF_DES_ADDR_REG(i)"]
pub mod i2s_in_eof_des_addr_reg;
#[doc = "I2S_OUT_EOF_BFR_DES_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_out_eof_bfr_des_addr_reg](i2s_out_eof_bfr_des_addr_reg) module"]
pub type I2S_OUT_EOF_BFR_DES_ADDR_REG = crate::Reg<u32, _I2S_OUT_EOF_BFR_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUT_EOF_BFR_DES_ADDR_REG;
#[doc = "`read()` method returns [i2s_out_eof_bfr_des_addr_reg::R](i2s_out_eof_bfr_des_addr_reg::R) reader structure"]
impl crate::Readable for I2S_OUT_EOF_BFR_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_out_eof_bfr_des_addr_reg::W](i2s_out_eof_bfr_des_addr_reg::W) writer structure"]
impl crate::Writable for I2S_OUT_EOF_BFR_DES_ADDR_REG {}
#[doc = "I2S_OUT_EOF_BFR_DES_ADDR_REG(i)"]
pub mod i2s_out_eof_bfr_des_addr_reg;
#[doc = "I2S_AHB_TEST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_ahb_test_reg](i2s_ahb_test_reg) module"]
pub type I2S_AHB_TEST_REG = crate::Reg<u32, _I2S_AHB_TEST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_AHB_TEST_REG;
#[doc = "`read()` method returns [i2s_ahb_test_reg::R](i2s_ahb_test_reg::R) reader structure"]
impl crate::Readable for I2S_AHB_TEST_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_ahb_test_reg::W](i2s_ahb_test_reg::W) writer structure"]
impl crate::Writable for I2S_AHB_TEST_REG {}
#[doc = "I2S_AHB_TEST_REG(i)"]
pub mod i2s_ahb_test_reg;
#[doc = "I2S_INLINK_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_inlink_dscr_reg](i2s_inlink_dscr_reg) module"]
pub type I2S_INLINK_DSCR_REG = crate::Reg<u32, _I2S_INLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INLINK_DSCR_REG;
#[doc = "`read()` method returns [i2s_inlink_dscr_reg::R](i2s_inlink_dscr_reg::R) reader structure"]
impl crate::Readable for I2S_INLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_inlink_dscr_reg::W](i2s_inlink_dscr_reg::W) writer structure"]
impl crate::Writable for I2S_INLINK_DSCR_REG {}
#[doc = "I2S_INLINK_DSCR_REG(i)"]
pub mod i2s_inlink_dscr_reg;
#[doc = "I2S_INLINK_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_inlink_dscr_bf0_reg](i2s_inlink_dscr_bf0_reg) module"]
pub type I2S_INLINK_DSCR_BF0_REG = crate::Reg<u32, _I2S_INLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [i2s_inlink_dscr_bf0_reg::R](i2s_inlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for I2S_INLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_inlink_dscr_bf0_reg::W](i2s_inlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for I2S_INLINK_DSCR_BF0_REG {}
#[doc = "I2S_INLINK_DSCR_BF0_REG(i)"]
pub mod i2s_inlink_dscr_bf0_reg;
#[doc = "I2S_INLINK_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_inlink_dscr_bf1_reg](i2s_inlink_dscr_bf1_reg) module"]
pub type I2S_INLINK_DSCR_BF1_REG = crate::Reg<u32, _I2S_INLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [i2s_inlink_dscr_bf1_reg::R](i2s_inlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for I2S_INLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_inlink_dscr_bf1_reg::W](i2s_inlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for I2S_INLINK_DSCR_BF1_REG {}
#[doc = "I2S_INLINK_DSCR_BF1_REG(i)"]
pub mod i2s_inlink_dscr_bf1_reg;
#[doc = "I2S_OUTLINK_DSCR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outlink_dscr_reg](i2s_outlink_dscr_reg) module"]
pub type I2S_OUTLINK_DSCR_REG = crate::Reg<u32, _I2S_OUTLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTLINK_DSCR_REG;
#[doc = "`read()` method returns [i2s_outlink_dscr_reg::R](i2s_outlink_dscr_reg::R) reader structure"]
impl crate::Readable for I2S_OUTLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_outlink_dscr_reg::W](i2s_outlink_dscr_reg::W) writer structure"]
impl crate::Writable for I2S_OUTLINK_DSCR_REG {}
#[doc = "I2S_OUTLINK_DSCR_REG(i)"]
pub mod i2s_outlink_dscr_reg;
#[doc = "I2S_OUTLINK_DSCR_BF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outlink_dscr_bf0_reg](i2s_outlink_dscr_bf0_reg) module"]
pub type I2S_OUTLINK_DSCR_BF0_REG = crate::Reg<u32, _I2S_OUTLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [i2s_outlink_dscr_bf0_reg::R](i2s_outlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for I2S_OUTLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_outlink_dscr_bf0_reg::W](i2s_outlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for I2S_OUTLINK_DSCR_BF0_REG {}
#[doc = "I2S_OUTLINK_DSCR_BF0_REG(i)"]
pub mod i2s_outlink_dscr_bf0_reg;
#[doc = "I2S_OUTLINK_DSCR_BF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outlink_dscr_bf1_reg](i2s_outlink_dscr_bf1_reg) module"]
pub type I2S_OUTLINK_DSCR_BF1_REG = crate::Reg<u32, _I2S_OUTLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [i2s_outlink_dscr_bf1_reg::R](i2s_outlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for I2S_OUTLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_outlink_dscr_bf1_reg::W](i2s_outlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for I2S_OUTLINK_DSCR_BF1_REG {}
#[doc = "I2S_OUTLINK_DSCR_BF1_REG(i)"]
pub mod i2s_outlink_dscr_bf1_reg;
#[doc = "I2S_LC_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_conf_reg](i2s_lc_conf_reg) module"]
pub type I2S_LC_CONF_REG = crate::Reg<u32, _I2S_LC_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_CONF_REG;
#[doc = "`read()` method returns [i2s_lc_conf_reg::R](i2s_lc_conf_reg::R) reader structure"]
impl crate::Readable for I2S_LC_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_conf_reg::W](i2s_lc_conf_reg::W) writer structure"]
impl crate::Writable for I2S_LC_CONF_REG {}
#[doc = "I2S_LC_CONF_REG(i)"]
pub mod i2s_lc_conf_reg;
#[doc = "I2S_OUTFIFO_PUSH_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_outfifo_push_reg](i2s_outfifo_push_reg) module"]
pub type I2S_OUTFIFO_PUSH_REG = crate::Reg<u32, _I2S_OUTFIFO_PUSH_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_OUTFIFO_PUSH_REG;
#[doc = "`read()` method returns [i2s_outfifo_push_reg::R](i2s_outfifo_push_reg::R) reader structure"]
impl crate::Readable for I2S_OUTFIFO_PUSH_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_outfifo_push_reg::W](i2s_outfifo_push_reg::W) writer structure"]
impl crate::Writable for I2S_OUTFIFO_PUSH_REG {}
#[doc = "I2S_OUTFIFO_PUSH_REG(i)"]
pub mod i2s_outfifo_push_reg;
#[doc = "I2S_INFIFO_POP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_infifo_pop_reg](i2s_infifo_pop_reg) module"]
pub type I2S_INFIFO_POP_REG = crate::Reg<u32, _I2S_INFIFO_POP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INFIFO_POP_REG;
#[doc = "`read()` method returns [i2s_infifo_pop_reg::R](i2s_infifo_pop_reg::R) reader structure"]
impl crate::Readable for I2S_INFIFO_POP_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_infifo_pop_reg::W](i2s_infifo_pop_reg::W) writer structure"]
impl crate::Writable for I2S_INFIFO_POP_REG {}
#[doc = "I2S_INFIFO_POP_REG(i)"]
pub mod i2s_infifo_pop_reg;
#[doc = "I2S_LC_STATE0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_state0_reg](i2s_lc_state0_reg) module"]
pub type I2S_LC_STATE0_REG = crate::Reg<u32, _I2S_LC_STATE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_STATE0_REG;
#[doc = "`read()` method returns [i2s_lc_state0_reg::R](i2s_lc_state0_reg::R) reader structure"]
impl crate::Readable for I2S_LC_STATE0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_state0_reg::W](i2s_lc_state0_reg::W) writer structure"]
impl crate::Writable for I2S_LC_STATE0_REG {}
#[doc = "I2S_LC_STATE0_REG(i)"]
pub mod i2s_lc_state0_reg;
#[doc = "I2S_LC_STATE1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_state1_reg](i2s_lc_state1_reg) module"]
pub type I2S_LC_STATE1_REG = crate::Reg<u32, _I2S_LC_STATE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_STATE1_REG;
#[doc = "`read()` method returns [i2s_lc_state1_reg::R](i2s_lc_state1_reg::R) reader structure"]
impl crate::Readable for I2S_LC_STATE1_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_state1_reg::W](i2s_lc_state1_reg::W) writer structure"]
impl crate::Writable for I2S_LC_STATE1_REG {}
#[doc = "I2S_LC_STATE1_REG(i)"]
pub mod i2s_lc_state1_reg;
#[doc = "I2S_LC_HUNG_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_lc_hung_conf_reg](i2s_lc_hung_conf_reg) module"]
pub type I2S_LC_HUNG_CONF_REG = crate::Reg<u32, _I2S_LC_HUNG_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_HUNG_CONF_REG;
#[doc = "`read()` method returns [i2s_lc_hung_conf_reg::R](i2s_lc_hung_conf_reg::R) reader structure"]
impl crate::Readable for I2S_LC_HUNG_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_hung_conf_reg::W](i2s_lc_hung_conf_reg::W) writer structure"]
impl crate::Writable for I2S_LC_HUNG_CONF_REG {}
#[doc = "I2S_LC_HUNG_CONF_REG(i)"]
pub mod i2s_lc_hung_conf_reg;
#[doc = "I2S_CVSD_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_cvsd_conf0_reg](i2s_cvsd_conf0_reg) module"]
pub type I2S_CVSD_CONF0_REG = crate::Reg<u32, _I2S_CVSD_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CVSD_CONF0_REG;
#[doc = "`read()` method returns [i2s_cvsd_conf0_reg::R](i2s_cvsd_conf0_reg::R) reader structure"]
impl crate::Readable for I2S_CVSD_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_cvsd_conf0_reg::W](i2s_cvsd_conf0_reg::W) writer structure"]
impl crate::Writable for I2S_CVSD_CONF0_REG {}
#[doc = "I2S_CVSD_CONF0_REG(i)"]
pub mod i2s_cvsd_conf0_reg;
#[doc = "I2S_CVSD_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_cvsd_conf1_reg](i2s_cvsd_conf1_reg) module"]
pub type I2S_CVSD_CONF1_REG = crate::Reg<u32, _I2S_CVSD_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CVSD_CONF1_REG;
#[doc = "`read()` method returns [i2s_cvsd_conf1_reg::R](i2s_cvsd_conf1_reg::R) reader structure"]
impl crate::Readable for I2S_CVSD_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_cvsd_conf1_reg::W](i2s_cvsd_conf1_reg::W) writer structure"]
impl crate::Writable for I2S_CVSD_CONF1_REG {}
#[doc = "I2S_CVSD_CONF1_REG(i)"]
pub mod i2s_cvsd_conf1_reg;
#[doc = "I2S_CVSD_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_cvsd_conf2_reg](i2s_cvsd_conf2_reg) module"]
pub type I2S_CVSD_CONF2_REG = crate::Reg<u32, _I2S_CVSD_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CVSD_CONF2_REG;
#[doc = "`read()` method returns [i2s_cvsd_conf2_reg::R](i2s_cvsd_conf2_reg::R) reader structure"]
impl crate::Readable for I2S_CVSD_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_cvsd_conf2_reg::W](i2s_cvsd_conf2_reg::W) writer structure"]
impl crate::Writable for I2S_CVSD_CONF2_REG {}
#[doc = "I2S_CVSD_CONF2_REG(i)"]
pub mod i2s_cvsd_conf2_reg;
#[doc = "I2S_PLC_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_plc_conf0_reg](i2s_plc_conf0_reg) module"]
pub type I2S_PLC_CONF0_REG = crate::Reg<u32, _I2S_PLC_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PLC_CONF0_REG;
#[doc = "`read()` method returns [i2s_plc_conf0_reg::R](i2s_plc_conf0_reg::R) reader structure"]
impl crate::Readable for I2S_PLC_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_plc_conf0_reg::W](i2s_plc_conf0_reg::W) writer structure"]
impl crate::Writable for I2S_PLC_CONF0_REG {}
#[doc = "I2S_PLC_CONF0_REG(i)"]
pub mod i2s_plc_conf0_reg;
#[doc = "I2S_PLC_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_plc_conf1_reg](i2s_plc_conf1_reg) module"]
pub type I2S_PLC_CONF1_REG = crate::Reg<u32, _I2S_PLC_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PLC_CONF1_REG;
#[doc = "`read()` method returns [i2s_plc_conf1_reg::R](i2s_plc_conf1_reg::R) reader structure"]
impl crate::Readable for I2S_PLC_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_plc_conf1_reg::W](i2s_plc_conf1_reg::W) writer structure"]
impl crate::Writable for I2S_PLC_CONF1_REG {}
#[doc = "I2S_PLC_CONF1_REG(i)"]
pub mod i2s_plc_conf1_reg;
#[doc = "I2S_PLC_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_plc_conf2_reg](i2s_plc_conf2_reg) module"]
pub type I2S_PLC_CONF2_REG = crate::Reg<u32, _I2S_PLC_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PLC_CONF2_REG;
#[doc = "`read()` method returns [i2s_plc_conf2_reg::R](i2s_plc_conf2_reg::R) reader structure"]
impl crate::Readable for I2S_PLC_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_plc_conf2_reg::W](i2s_plc_conf2_reg::W) writer structure"]
impl crate::Writable for I2S_PLC_CONF2_REG {}
#[doc = "I2S_PLC_CONF2_REG(i)"]
pub mod i2s_plc_conf2_reg;
#[doc = "I2S_ESCO_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_esco_conf0_reg](i2s_esco_conf0_reg) module"]
pub type I2S_ESCO_CONF0_REG = crate::Reg<u32, _I2S_ESCO_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_ESCO_CONF0_REG;
#[doc = "`read()` method returns [i2s_esco_conf0_reg::R](i2s_esco_conf0_reg::R) reader structure"]
impl crate::Readable for I2S_ESCO_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_esco_conf0_reg::W](i2s_esco_conf0_reg::W) writer structure"]
impl crate::Writable for I2S_ESCO_CONF0_REG {}
#[doc = "I2S_ESCO_CONF0_REG(i)"]
pub mod i2s_esco_conf0_reg;
#[doc = "I2S_SCO_CONF0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_sco_conf0_reg](i2s_sco_conf0_reg) module"]
pub type I2S_SCO_CONF0_REG = crate::Reg<u32, _I2S_SCO_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_SCO_CONF0_REG;
#[doc = "`read()` method returns [i2s_sco_conf0_reg::R](i2s_sco_conf0_reg::R) reader structure"]
impl crate::Readable for I2S_SCO_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_sco_conf0_reg::W](i2s_sco_conf0_reg::W) writer structure"]
impl crate::Writable for I2S_SCO_CONF0_REG {}
#[doc = "I2S_SCO_CONF0_REG(i)"]
pub mod i2s_sco_conf0_reg;
#[doc = "I2S_CONF1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf1_reg](i2s_conf1_reg) module"]
pub type I2S_CONF1_REG = crate::Reg<u32, _I2S_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF1_REG;
#[doc = "`read()` method returns [i2s_conf1_reg::R](i2s_conf1_reg::R) reader structure"]
impl crate::Readable for I2S_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_conf1_reg::W](i2s_conf1_reg::W) writer structure"]
impl crate::Writable for I2S_CONF1_REG {}
#[doc = "I2S_CONF1_REG(i)"]
pub mod i2s_conf1_reg;
#[doc = "I2S_PD_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_pd_conf_reg](i2s_pd_conf_reg) module"]
pub type I2S_PD_CONF_REG = crate::Reg<u32, _I2S_PD_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PD_CONF_REG;
#[doc = "`read()` method returns [i2s_pd_conf_reg::R](i2s_pd_conf_reg::R) reader structure"]
impl crate::Readable for I2S_PD_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_pd_conf_reg::W](i2s_pd_conf_reg::W) writer structure"]
impl crate::Writable for I2S_PD_CONF_REG {}
#[doc = "I2S_PD_CONF_REG(i)"]
pub mod i2s_pd_conf_reg;
#[doc = "I2S_CONF2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_conf2_reg](i2s_conf2_reg) module"]
pub type I2S_CONF2_REG = crate::Reg<u32, _I2S_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF2_REG;
#[doc = "`read()` method returns [i2s_conf2_reg::R](i2s_conf2_reg::R) reader structure"]
impl crate::Readable for I2S_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_conf2_reg::W](i2s_conf2_reg::W) writer structure"]
impl crate::Writable for I2S_CONF2_REG {}
#[doc = "I2S_CONF2_REG(i)"]
pub mod i2s_conf2_reg;
#[doc = "I2S_CLKM_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_clkm_conf_reg](i2s_clkm_conf_reg) module"]
pub type I2S_CLKM_CONF_REG = crate::Reg<u32, _I2S_CLKM_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CLKM_CONF_REG;
#[doc = "`read()` method returns [i2s_clkm_conf_reg::R](i2s_clkm_conf_reg::R) reader structure"]
impl crate::Readable for I2S_CLKM_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_clkm_conf_reg::W](i2s_clkm_conf_reg::W) writer structure"]
impl crate::Writable for I2S_CLKM_CONF_REG {}
#[doc = "I2S_CLKM_CONF_REG(i)"]
pub mod i2s_clkm_conf_reg;
#[doc = "I2S_SAMPLE_RATE_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_sample_rate_conf_reg](i2s_sample_rate_conf_reg) module"]
pub type I2S_SAMPLE_RATE_CONF_REG = crate::Reg<u32, _I2S_SAMPLE_RATE_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_SAMPLE_RATE_CONF_REG;
#[doc = "`read()` method returns [i2s_sample_rate_conf_reg::R](i2s_sample_rate_conf_reg::R) reader structure"]
impl crate::Readable for I2S_SAMPLE_RATE_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_sample_rate_conf_reg::W](i2s_sample_rate_conf_reg::W) writer structure"]
impl crate::Writable for I2S_SAMPLE_RATE_CONF_REG {}
#[doc = "I2S_SAMPLE_RATE_CONF_REG(i)"]
pub mod i2s_sample_rate_conf_reg;
#[doc = "I2S_PDM_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_pdm_conf_reg](i2s_pdm_conf_reg) module"]
pub type I2S_PDM_CONF_REG = crate::Reg<u32, _I2S_PDM_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PDM_CONF_REG;
#[doc = "`read()` method returns [i2s_pdm_conf_reg::R](i2s_pdm_conf_reg::R) reader structure"]
impl crate::Readable for I2S_PDM_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_pdm_conf_reg::W](i2s_pdm_conf_reg::W) writer structure"]
impl crate::Writable for I2S_PDM_CONF_REG {}
#[doc = "I2S_PDM_CONF_REG(i)"]
pub mod i2s_pdm_conf_reg;
#[doc = "I2S_PDM_FREQ_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_pdm_freq_conf_reg](i2s_pdm_freq_conf_reg) module"]
pub type I2S_PDM_FREQ_CONF_REG = crate::Reg<u32, _I2S_PDM_FREQ_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_PDM_FREQ_CONF_REG;
#[doc = "`read()` method returns [i2s_pdm_freq_conf_reg::R](i2s_pdm_freq_conf_reg::R) reader structure"]
impl crate::Readable for I2S_PDM_FREQ_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_pdm_freq_conf_reg::W](i2s_pdm_freq_conf_reg::W) writer structure"]
impl crate::Writable for I2S_PDM_FREQ_CONF_REG {}
#[doc = "I2S_PDM_FREQ_CONF_REG(i)"]
pub mod i2s_pdm_freq_conf_reg;
#[doc = "I2S_STATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_state_reg](i2s_state_reg) module"]
pub type I2S_STATE_REG = crate::Reg<u32, _I2S_STATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_STATE_REG;
#[doc = "`read()` method returns [i2s_state_reg::R](i2s_state_reg::R) reader structure"]
impl crate::Readable for I2S_STATE_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_state_reg::W](i2s_state_reg::W) writer structure"]
impl crate::Writable for I2S_STATE_REG {}
#[doc = "I2S_STATE_REG(i)"]
pub mod i2s_state_reg;
#[doc = "I2S_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_date_reg](i2s_date_reg) module"]
pub type I2S_DATE_REG = crate::Reg<u32, _I2S_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_DATE_REG;
#[doc = "`read()` method returns [i2s_date_reg::R](i2s_date_reg::R) reader structure"]
impl crate::Readable for I2S_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [i2s_date_reg::W](i2s_date_reg::W) writer structure"]
impl crate::Writable for I2S_DATE_REG {}
#[doc = "I2S_DATE_REG(i)"]
pub mod i2s_date_reg;