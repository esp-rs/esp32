#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SENS_SAR_READ_CTRL_REG"]
    pub sens_sar_read_ctrl_reg: SENS_SAR_READ_CTRL_REG,
    #[doc = "0x04 - SENS_SAR_READ_STATUS1_REG"]
    pub sens_sar_read_status1_reg: SENS_SAR_READ_STATUS1_REG,
    #[doc = "0x08 - SENS_SAR_MEAS_WAIT1_REG"]
    pub sens_sar_meas_wait1_reg: SENS_SAR_MEAS_WAIT1_REG,
    #[doc = "0x0c - SENS_SAR_MEAS_WAIT2_REG"]
    pub sens_sar_meas_wait2_reg: SENS_SAR_MEAS_WAIT2_REG,
    #[doc = "0x10 - SENS_SAR_MEAS_CTRL_REG"]
    pub sens_sar_meas_ctrl_reg: SENS_SAR_MEAS_CTRL_REG,
    #[doc = "0x14 - SENS_SAR_READ_STATUS2_REG"]
    pub sens_sar_read_status2_reg: SENS_SAR_READ_STATUS2_REG,
    #[doc = "0x18 - SENS_ULP_CP_SLEEP_CYC0_REG"]
    pub sens_ulp_cp_sleep_cyc0_reg: SENS_ULP_CP_SLEEP_CYC0_REG,
    #[doc = "0x1c - SENS_ULP_CP_SLEEP_CYC1_REG"]
    pub sens_ulp_cp_sleep_cyc1_reg: SENS_ULP_CP_SLEEP_CYC1_REG,
    #[doc = "0x20 - SENS_ULP_CP_SLEEP_CYC2_REG"]
    pub sens_ulp_cp_sleep_cyc2_reg: SENS_ULP_CP_SLEEP_CYC2_REG,
    #[doc = "0x24 - SENS_ULP_CP_SLEEP_CYC3_REG"]
    pub sens_ulp_cp_sleep_cyc3_reg: SENS_ULP_CP_SLEEP_CYC3_REG,
    #[doc = "0x28 - SENS_ULP_CP_SLEEP_CYC4_REG"]
    pub sens_ulp_cp_sleep_cyc4_reg: SENS_ULP_CP_SLEEP_CYC4_REG,
    #[doc = "0x2c - SENS_SAR_START_FORCE_REG"]
    pub sens_sar_start_force_reg: SENS_SAR_START_FORCE_REG,
    #[doc = "0x30 - SENS_SAR_MEM_WR_CTRL_REG"]
    pub sens_sar_mem_wr_ctrl_reg: SENS_SAR_MEM_WR_CTRL_REG,
    #[doc = "0x34 - SENS_SAR_ATTEN1_REG"]
    pub sens_sar_atten1_reg: SENS_SAR_ATTEN1_REG,
    #[doc = "0x38 - SENS_SAR_ATTEN2_REG"]
    pub sens_sar_atten2_reg: SENS_SAR_ATTEN2_REG,
    #[doc = "0x3c - SENS_SAR_SLAVE_ADDR1_REG"]
    pub sens_sar_slave_addr1_reg: SENS_SAR_SLAVE_ADDR1_REG,
    #[doc = "0x40 - SENS_SAR_SLAVE_ADDR2_REG"]
    pub sens_sar_slave_addr2_reg: SENS_SAR_SLAVE_ADDR2_REG,
    #[doc = "0x44 - SENS_SAR_SLAVE_ADDR3_REG"]
    pub sens_sar_slave_addr3_reg: SENS_SAR_SLAVE_ADDR3_REG,
    #[doc = "0x48 - SENS_SAR_SLAVE_ADDR4_REG"]
    pub sens_sar_slave_addr4_reg: SENS_SAR_SLAVE_ADDR4_REG,
    #[doc = "0x4c - SENS_SAR_TSENS_CTRL_REG"]
    pub sens_sar_tsens_ctrl_reg: SENS_SAR_TSENS_CTRL_REG,
    #[doc = "0x50 - SENS_SAR_I2C_CTRL_REG"]
    pub sens_sar_i2c_ctrl_reg: SENS_SAR_I2C_CTRL_REG,
    #[doc = "0x54 - SENS_SAR_MEAS_START1_REG"]
    pub sens_sar_meas_start1_reg: SENS_SAR_MEAS_START1_REG,
    #[doc = "0x58 - SENS_SAR_TOUCH_CTRL1_REG"]
    pub sens_sar_touch_ctrl1_reg: SENS_SAR_TOUCH_CTRL1_REG,
    #[doc = "0x5c - SENS_SAR_TOUCH_THRES1_REG"]
    pub sens_sar_touch_thres1_reg: SENS_SAR_TOUCH_THRES1_REG,
    #[doc = "0x60 - SENS_SAR_TOUCH_THRES2_REG"]
    pub sens_sar_touch_thres2_reg: SENS_SAR_TOUCH_THRES2_REG,
    #[doc = "0x64 - SENS_SAR_TOUCH_THRES3_REG"]
    pub sens_sar_touch_thres3_reg: SENS_SAR_TOUCH_THRES3_REG,
    #[doc = "0x68 - SENS_SAR_TOUCH_THRES4_REG"]
    pub sens_sar_touch_thres4_reg: SENS_SAR_TOUCH_THRES4_REG,
    #[doc = "0x6c - SENS_SAR_TOUCH_THRES5_REG"]
    pub sens_sar_touch_thres5_reg: SENS_SAR_TOUCH_THRES5_REG,
    #[doc = "0x70 - SENS_SAR_TOUCH_OUT1_REG"]
    pub sens_sar_touch_out1_reg: SENS_SAR_TOUCH_OUT1_REG,
    #[doc = "0x74 - SENS_SAR_TOUCH_OUT2_REG"]
    pub sens_sar_touch_out2_reg: SENS_SAR_TOUCH_OUT2_REG,
    #[doc = "0x78 - SENS_SAR_TOUCH_OUT3_REG"]
    pub sens_sar_touch_out3_reg: SENS_SAR_TOUCH_OUT3_REG,
    #[doc = "0x7c - SENS_SAR_TOUCH_OUT4_REG"]
    pub sens_sar_touch_out4_reg: SENS_SAR_TOUCH_OUT4_REG,
    #[doc = "0x80 - SENS_SAR_TOUCH_OUT5_REG"]
    pub sens_sar_touch_out5_reg: SENS_SAR_TOUCH_OUT5_REG,
    #[doc = "0x84 - SENS_SAR_TOUCH_CTRL2_REG"]
    pub sens_sar_touch_ctrl2_reg: SENS_SAR_TOUCH_CTRL2_REG,
    _reserved34: [u8; 4usize],
    #[doc = "0x8c - SENS_SAR_TOUCH_ENABLE_REG"]
    pub sens_sar_touch_enable_reg: SENS_SAR_TOUCH_ENABLE_REG,
    #[doc = "0x90 - SENS_SAR_READ_CTRL2_REG"]
    pub sens_sar_read_ctrl2_reg: SENS_SAR_READ_CTRL2_REG,
    #[doc = "0x94 - SENS_SAR_MEAS_START2_REG"]
    pub sens_sar_meas_start2_reg: SENS_SAR_MEAS_START2_REG,
    #[doc = "0x98 - SENS_SAR_DAC_CTRL1_REG"]
    pub sens_sar_dac_ctrl1_reg: SENS_SAR_DAC_CTRL1_REG,
    #[doc = "0x9c - SENS_SAR_DAC_CTRL2_REG"]
    pub sens_sar_dac_ctrl2_reg: SENS_SAR_DAC_CTRL2_REG,
    #[doc = "0xa0 - SENS_SAR_MEAS_CTRL2_REG"]
    pub sens_sar_meas_ctrl2_reg: SENS_SAR_MEAS_CTRL2_REG,
    _reserved40: [u8; 84usize],
    #[doc = "0xf8 - SENS_SAR_NOUSE_REG"]
    pub sens_sar_nouse_reg: SENS_SAR_NOUSE_REG,
    #[doc = "0xfc - SENS_SARDATE_REG"]
    pub sens_sardate_reg: SENS_SARDATE_REG,
}
#[doc = "SENS_SAR_READ_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_read_ctrl_reg](sens_sar_read_ctrl_reg) module"]
pub type SENS_SAR_READ_CTRL_REG = crate::Reg<u32, _SENS_SAR_READ_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_READ_CTRL_REG;
#[doc = "`read()` method returns [sens_sar_read_ctrl_reg::R](sens_sar_read_ctrl_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_READ_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_read_ctrl_reg::W](sens_sar_read_ctrl_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_READ_CTRL_REG {}
#[doc = "SENS_SAR_READ_CTRL_REG"]
pub mod sens_sar_read_ctrl_reg;
#[doc = "SENS_SAR_READ_STATUS1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_read_status1_reg](sens_sar_read_status1_reg) module"]
pub type SENS_SAR_READ_STATUS1_REG = crate::Reg<u32, _SENS_SAR_READ_STATUS1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_READ_STATUS1_REG;
#[doc = "`read()` method returns [sens_sar_read_status1_reg::R](sens_sar_read_status1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_READ_STATUS1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_read_status1_reg::W](sens_sar_read_status1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_READ_STATUS1_REG {}
#[doc = "SENS_SAR_READ_STATUS1_REG"]
pub mod sens_sar_read_status1_reg;
#[doc = "SENS_SAR_MEAS_WAIT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_meas_wait1_reg](sens_sar_meas_wait1_reg) module"]
pub type SENS_SAR_MEAS_WAIT1_REG = crate::Reg<u32, _SENS_SAR_MEAS_WAIT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEAS_WAIT1_REG;
#[doc = "`read()` method returns [sens_sar_meas_wait1_reg::R](sens_sar_meas_wait1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEAS_WAIT1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_meas_wait1_reg::W](sens_sar_meas_wait1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEAS_WAIT1_REG {}
#[doc = "SENS_SAR_MEAS_WAIT1_REG"]
pub mod sens_sar_meas_wait1_reg;
#[doc = "SENS_SAR_MEAS_WAIT2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_meas_wait2_reg](sens_sar_meas_wait2_reg) module"]
pub type SENS_SAR_MEAS_WAIT2_REG = crate::Reg<u32, _SENS_SAR_MEAS_WAIT2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEAS_WAIT2_REG;
#[doc = "`read()` method returns [sens_sar_meas_wait2_reg::R](sens_sar_meas_wait2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEAS_WAIT2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_meas_wait2_reg::W](sens_sar_meas_wait2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEAS_WAIT2_REG {}
#[doc = "SENS_SAR_MEAS_WAIT2_REG"]
pub mod sens_sar_meas_wait2_reg;
#[doc = "SENS_SAR_MEAS_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_meas_ctrl_reg](sens_sar_meas_ctrl_reg) module"]
pub type SENS_SAR_MEAS_CTRL_REG = crate::Reg<u32, _SENS_SAR_MEAS_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEAS_CTRL_REG;
#[doc = "`read()` method returns [sens_sar_meas_ctrl_reg::R](sens_sar_meas_ctrl_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEAS_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_meas_ctrl_reg::W](sens_sar_meas_ctrl_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEAS_CTRL_REG {}
#[doc = "SENS_SAR_MEAS_CTRL_REG"]
pub mod sens_sar_meas_ctrl_reg;
#[doc = "SENS_SAR_READ_STATUS2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_read_status2_reg](sens_sar_read_status2_reg) module"]
pub type SENS_SAR_READ_STATUS2_REG = crate::Reg<u32, _SENS_SAR_READ_STATUS2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_READ_STATUS2_REG;
#[doc = "`read()` method returns [sens_sar_read_status2_reg::R](sens_sar_read_status2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_READ_STATUS2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_read_status2_reg::W](sens_sar_read_status2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_READ_STATUS2_REG {}
#[doc = "SENS_SAR_READ_STATUS2_REG"]
pub mod sens_sar_read_status2_reg;
#[doc = "SENS_ULP_CP_SLEEP_CYC0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_ulp_cp_sleep_cyc0_reg](sens_ulp_cp_sleep_cyc0_reg) module"]
pub type SENS_ULP_CP_SLEEP_CYC0_REG = crate::Reg<u32, _SENS_ULP_CP_SLEEP_CYC0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_ULP_CP_SLEEP_CYC0_REG;
#[doc = "`read()` method returns [sens_ulp_cp_sleep_cyc0_reg::R](sens_ulp_cp_sleep_cyc0_reg::R) reader structure"]
impl crate::Readable for SENS_ULP_CP_SLEEP_CYC0_REG {}
#[doc = "`write(|w| ..)` method takes [sens_ulp_cp_sleep_cyc0_reg::W](sens_ulp_cp_sleep_cyc0_reg::W) writer structure"]
impl crate::Writable for SENS_ULP_CP_SLEEP_CYC0_REG {}
#[doc = "SENS_ULP_CP_SLEEP_CYC0_REG"]
pub mod sens_ulp_cp_sleep_cyc0_reg;
#[doc = "SENS_ULP_CP_SLEEP_CYC1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_ulp_cp_sleep_cyc1_reg](sens_ulp_cp_sleep_cyc1_reg) module"]
pub type SENS_ULP_CP_SLEEP_CYC1_REG = crate::Reg<u32, _SENS_ULP_CP_SLEEP_CYC1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_ULP_CP_SLEEP_CYC1_REG;
#[doc = "`read()` method returns [sens_ulp_cp_sleep_cyc1_reg::R](sens_ulp_cp_sleep_cyc1_reg::R) reader structure"]
impl crate::Readable for SENS_ULP_CP_SLEEP_CYC1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_ulp_cp_sleep_cyc1_reg::W](sens_ulp_cp_sleep_cyc1_reg::W) writer structure"]
impl crate::Writable for SENS_ULP_CP_SLEEP_CYC1_REG {}
#[doc = "SENS_ULP_CP_SLEEP_CYC1_REG"]
pub mod sens_ulp_cp_sleep_cyc1_reg;
#[doc = "SENS_ULP_CP_SLEEP_CYC2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_ulp_cp_sleep_cyc2_reg](sens_ulp_cp_sleep_cyc2_reg) module"]
pub type SENS_ULP_CP_SLEEP_CYC2_REG = crate::Reg<u32, _SENS_ULP_CP_SLEEP_CYC2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_ULP_CP_SLEEP_CYC2_REG;
#[doc = "`read()` method returns [sens_ulp_cp_sleep_cyc2_reg::R](sens_ulp_cp_sleep_cyc2_reg::R) reader structure"]
impl crate::Readable for SENS_ULP_CP_SLEEP_CYC2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_ulp_cp_sleep_cyc2_reg::W](sens_ulp_cp_sleep_cyc2_reg::W) writer structure"]
impl crate::Writable for SENS_ULP_CP_SLEEP_CYC2_REG {}
#[doc = "SENS_ULP_CP_SLEEP_CYC2_REG"]
pub mod sens_ulp_cp_sleep_cyc2_reg;
#[doc = "SENS_ULP_CP_SLEEP_CYC3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_ulp_cp_sleep_cyc3_reg](sens_ulp_cp_sleep_cyc3_reg) module"]
pub type SENS_ULP_CP_SLEEP_CYC3_REG = crate::Reg<u32, _SENS_ULP_CP_SLEEP_CYC3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_ULP_CP_SLEEP_CYC3_REG;
#[doc = "`read()` method returns [sens_ulp_cp_sleep_cyc3_reg::R](sens_ulp_cp_sleep_cyc3_reg::R) reader structure"]
impl crate::Readable for SENS_ULP_CP_SLEEP_CYC3_REG {}
#[doc = "`write(|w| ..)` method takes [sens_ulp_cp_sleep_cyc3_reg::W](sens_ulp_cp_sleep_cyc3_reg::W) writer structure"]
impl crate::Writable for SENS_ULP_CP_SLEEP_CYC3_REG {}
#[doc = "SENS_ULP_CP_SLEEP_CYC3_REG"]
pub mod sens_ulp_cp_sleep_cyc3_reg;
#[doc = "SENS_ULP_CP_SLEEP_CYC4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_ulp_cp_sleep_cyc4_reg](sens_ulp_cp_sleep_cyc4_reg) module"]
pub type SENS_ULP_CP_SLEEP_CYC4_REG = crate::Reg<u32, _SENS_ULP_CP_SLEEP_CYC4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_ULP_CP_SLEEP_CYC4_REG;
#[doc = "`read()` method returns [sens_ulp_cp_sleep_cyc4_reg::R](sens_ulp_cp_sleep_cyc4_reg::R) reader structure"]
impl crate::Readable for SENS_ULP_CP_SLEEP_CYC4_REG {}
#[doc = "`write(|w| ..)` method takes [sens_ulp_cp_sleep_cyc4_reg::W](sens_ulp_cp_sleep_cyc4_reg::W) writer structure"]
impl crate::Writable for SENS_ULP_CP_SLEEP_CYC4_REG {}
#[doc = "SENS_ULP_CP_SLEEP_CYC4_REG"]
pub mod sens_ulp_cp_sleep_cyc4_reg;
#[doc = "SENS_SAR_START_FORCE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_start_force_reg](sens_sar_start_force_reg) module"]
pub type SENS_SAR_START_FORCE_REG = crate::Reg<u32, _SENS_SAR_START_FORCE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_START_FORCE_REG;
#[doc = "`read()` method returns [sens_sar_start_force_reg::R](sens_sar_start_force_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_START_FORCE_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_start_force_reg::W](sens_sar_start_force_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_START_FORCE_REG {}
#[doc = "SENS_SAR_START_FORCE_REG"]
pub mod sens_sar_start_force_reg;
#[doc = "SENS_SAR_MEM_WR_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_mem_wr_ctrl_reg](sens_sar_mem_wr_ctrl_reg) module"]
pub type SENS_SAR_MEM_WR_CTRL_REG = crate::Reg<u32, _SENS_SAR_MEM_WR_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEM_WR_CTRL_REG;
#[doc = "`read()` method returns [sens_sar_mem_wr_ctrl_reg::R](sens_sar_mem_wr_ctrl_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEM_WR_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_mem_wr_ctrl_reg::W](sens_sar_mem_wr_ctrl_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEM_WR_CTRL_REG {}
#[doc = "SENS_SAR_MEM_WR_CTRL_REG"]
pub mod sens_sar_mem_wr_ctrl_reg;
#[doc = "SENS_SAR_ATTEN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_atten1_reg](sens_sar_atten1_reg) module"]
pub type SENS_SAR_ATTEN1_REG = crate::Reg<u32, _SENS_SAR_ATTEN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_ATTEN1_REG;
#[doc = "`read()` method returns [sens_sar_atten1_reg::R](sens_sar_atten1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_ATTEN1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_atten1_reg::W](sens_sar_atten1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_ATTEN1_REG {}
#[doc = "SENS_SAR_ATTEN1_REG"]
pub mod sens_sar_atten1_reg;
#[doc = "SENS_SAR_ATTEN2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_atten2_reg](sens_sar_atten2_reg) module"]
pub type SENS_SAR_ATTEN2_REG = crate::Reg<u32, _SENS_SAR_ATTEN2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_ATTEN2_REG;
#[doc = "`read()` method returns [sens_sar_atten2_reg::R](sens_sar_atten2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_ATTEN2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_atten2_reg::W](sens_sar_atten2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_ATTEN2_REG {}
#[doc = "SENS_SAR_ATTEN2_REG"]
pub mod sens_sar_atten2_reg;
#[doc = "SENS_SAR_SLAVE_ADDR1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_slave_addr1_reg](sens_sar_slave_addr1_reg) module"]
pub type SENS_SAR_SLAVE_ADDR1_REG = crate::Reg<u32, _SENS_SAR_SLAVE_ADDR1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_SLAVE_ADDR1_REG;
#[doc = "`read()` method returns [sens_sar_slave_addr1_reg::R](sens_sar_slave_addr1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_SLAVE_ADDR1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_slave_addr1_reg::W](sens_sar_slave_addr1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_SLAVE_ADDR1_REG {}
#[doc = "SENS_SAR_SLAVE_ADDR1_REG"]
pub mod sens_sar_slave_addr1_reg;
#[doc = "SENS_SAR_SLAVE_ADDR2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_slave_addr2_reg](sens_sar_slave_addr2_reg) module"]
pub type SENS_SAR_SLAVE_ADDR2_REG = crate::Reg<u32, _SENS_SAR_SLAVE_ADDR2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_SLAVE_ADDR2_REG;
#[doc = "`read()` method returns [sens_sar_slave_addr2_reg::R](sens_sar_slave_addr2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_SLAVE_ADDR2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_slave_addr2_reg::W](sens_sar_slave_addr2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_SLAVE_ADDR2_REG {}
#[doc = "SENS_SAR_SLAVE_ADDR2_REG"]
pub mod sens_sar_slave_addr2_reg;
#[doc = "SENS_SAR_SLAVE_ADDR3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_slave_addr3_reg](sens_sar_slave_addr3_reg) module"]
pub type SENS_SAR_SLAVE_ADDR3_REG = crate::Reg<u32, _SENS_SAR_SLAVE_ADDR3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_SLAVE_ADDR3_REG;
#[doc = "`read()` method returns [sens_sar_slave_addr3_reg::R](sens_sar_slave_addr3_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_SLAVE_ADDR3_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_slave_addr3_reg::W](sens_sar_slave_addr3_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_SLAVE_ADDR3_REG {}
#[doc = "SENS_SAR_SLAVE_ADDR3_REG"]
pub mod sens_sar_slave_addr3_reg;
#[doc = "SENS_SAR_SLAVE_ADDR4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_slave_addr4_reg](sens_sar_slave_addr4_reg) module"]
pub type SENS_SAR_SLAVE_ADDR4_REG = crate::Reg<u32, _SENS_SAR_SLAVE_ADDR4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_SLAVE_ADDR4_REG;
#[doc = "`read()` method returns [sens_sar_slave_addr4_reg::R](sens_sar_slave_addr4_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_SLAVE_ADDR4_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_slave_addr4_reg::W](sens_sar_slave_addr4_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_SLAVE_ADDR4_REG {}
#[doc = "SENS_SAR_SLAVE_ADDR4_REG"]
pub mod sens_sar_slave_addr4_reg;
#[doc = "SENS_SAR_TSENS_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_tsens_ctrl_reg](sens_sar_tsens_ctrl_reg) module"]
pub type SENS_SAR_TSENS_CTRL_REG = crate::Reg<u32, _SENS_SAR_TSENS_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TSENS_CTRL_REG;
#[doc = "`read()` method returns [sens_sar_tsens_ctrl_reg::R](sens_sar_tsens_ctrl_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TSENS_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_tsens_ctrl_reg::W](sens_sar_tsens_ctrl_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TSENS_CTRL_REG {}
#[doc = "SENS_SAR_TSENS_CTRL_REG"]
pub mod sens_sar_tsens_ctrl_reg;
#[doc = "SENS_SAR_I2C_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_i2c_ctrl_reg](sens_sar_i2c_ctrl_reg) module"]
pub type SENS_SAR_I2C_CTRL_REG = crate::Reg<u32, _SENS_SAR_I2C_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_I2C_CTRL_REG;
#[doc = "`read()` method returns [sens_sar_i2c_ctrl_reg::R](sens_sar_i2c_ctrl_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_I2C_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_i2c_ctrl_reg::W](sens_sar_i2c_ctrl_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_I2C_CTRL_REG {}
#[doc = "SENS_SAR_I2C_CTRL_REG"]
pub mod sens_sar_i2c_ctrl_reg;
#[doc = "SENS_SAR_MEAS_START1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_meas_start1_reg](sens_sar_meas_start1_reg) module"]
pub type SENS_SAR_MEAS_START1_REG = crate::Reg<u32, _SENS_SAR_MEAS_START1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEAS_START1_REG;
#[doc = "`read()` method returns [sens_sar_meas_start1_reg::R](sens_sar_meas_start1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEAS_START1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_meas_start1_reg::W](sens_sar_meas_start1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEAS_START1_REG {}
#[doc = "SENS_SAR_MEAS_START1_REG"]
pub mod sens_sar_meas_start1_reg;
#[doc = "SENS_SAR_TOUCH_CTRL1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_ctrl1_reg](sens_sar_touch_ctrl1_reg) module"]
pub type SENS_SAR_TOUCH_CTRL1_REG = crate::Reg<u32, _SENS_SAR_TOUCH_CTRL1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_CTRL1_REG;
#[doc = "`read()` method returns [sens_sar_touch_ctrl1_reg::R](sens_sar_touch_ctrl1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_CTRL1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_ctrl1_reg::W](sens_sar_touch_ctrl1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_CTRL1_REG {}
#[doc = "SENS_SAR_TOUCH_CTRL1_REG"]
pub mod sens_sar_touch_ctrl1_reg;
#[doc = "SENS_SAR_TOUCH_THRES1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_thres1_reg](sens_sar_touch_thres1_reg) module"]
pub type SENS_SAR_TOUCH_THRES1_REG = crate::Reg<u32, _SENS_SAR_TOUCH_THRES1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_THRES1_REG;
#[doc = "`read()` method returns [sens_sar_touch_thres1_reg::R](sens_sar_touch_thres1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_THRES1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_thres1_reg::W](sens_sar_touch_thres1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_THRES1_REG {}
#[doc = "SENS_SAR_TOUCH_THRES1_REG"]
pub mod sens_sar_touch_thres1_reg;
#[doc = "SENS_SAR_TOUCH_THRES2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_thres2_reg](sens_sar_touch_thres2_reg) module"]
pub type SENS_SAR_TOUCH_THRES2_REG = crate::Reg<u32, _SENS_SAR_TOUCH_THRES2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_THRES2_REG;
#[doc = "`read()` method returns [sens_sar_touch_thres2_reg::R](sens_sar_touch_thres2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_THRES2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_thres2_reg::W](sens_sar_touch_thres2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_THRES2_REG {}
#[doc = "SENS_SAR_TOUCH_THRES2_REG"]
pub mod sens_sar_touch_thres2_reg;
#[doc = "SENS_SAR_TOUCH_THRES3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_thres3_reg](sens_sar_touch_thres3_reg) module"]
pub type SENS_SAR_TOUCH_THRES3_REG = crate::Reg<u32, _SENS_SAR_TOUCH_THRES3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_THRES3_REG;
#[doc = "`read()` method returns [sens_sar_touch_thres3_reg::R](sens_sar_touch_thres3_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_THRES3_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_thres3_reg::W](sens_sar_touch_thres3_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_THRES3_REG {}
#[doc = "SENS_SAR_TOUCH_THRES3_REG"]
pub mod sens_sar_touch_thres3_reg;
#[doc = "SENS_SAR_TOUCH_THRES4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_thres4_reg](sens_sar_touch_thres4_reg) module"]
pub type SENS_SAR_TOUCH_THRES4_REG = crate::Reg<u32, _SENS_SAR_TOUCH_THRES4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_THRES4_REG;
#[doc = "`read()` method returns [sens_sar_touch_thres4_reg::R](sens_sar_touch_thres4_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_THRES4_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_thres4_reg::W](sens_sar_touch_thres4_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_THRES4_REG {}
#[doc = "SENS_SAR_TOUCH_THRES4_REG"]
pub mod sens_sar_touch_thres4_reg;
#[doc = "SENS_SAR_TOUCH_THRES5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_thres5_reg](sens_sar_touch_thres5_reg) module"]
pub type SENS_SAR_TOUCH_THRES5_REG = crate::Reg<u32, _SENS_SAR_TOUCH_THRES5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_THRES5_REG;
#[doc = "`read()` method returns [sens_sar_touch_thres5_reg::R](sens_sar_touch_thres5_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_THRES5_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_thres5_reg::W](sens_sar_touch_thres5_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_THRES5_REG {}
#[doc = "SENS_SAR_TOUCH_THRES5_REG"]
pub mod sens_sar_touch_thres5_reg;
#[doc = "SENS_SAR_TOUCH_OUT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_out1_reg](sens_sar_touch_out1_reg) module"]
pub type SENS_SAR_TOUCH_OUT1_REG = crate::Reg<u32, _SENS_SAR_TOUCH_OUT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_OUT1_REG;
#[doc = "`read()` method returns [sens_sar_touch_out1_reg::R](sens_sar_touch_out1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_OUT1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_out1_reg::W](sens_sar_touch_out1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_OUT1_REG {}
#[doc = "SENS_SAR_TOUCH_OUT1_REG"]
pub mod sens_sar_touch_out1_reg;
#[doc = "SENS_SAR_TOUCH_OUT2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_out2_reg](sens_sar_touch_out2_reg) module"]
pub type SENS_SAR_TOUCH_OUT2_REG = crate::Reg<u32, _SENS_SAR_TOUCH_OUT2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_OUT2_REG;
#[doc = "`read()` method returns [sens_sar_touch_out2_reg::R](sens_sar_touch_out2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_OUT2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_out2_reg::W](sens_sar_touch_out2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_OUT2_REG {}
#[doc = "SENS_SAR_TOUCH_OUT2_REG"]
pub mod sens_sar_touch_out2_reg;
#[doc = "SENS_SAR_TOUCH_OUT3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_out3_reg](sens_sar_touch_out3_reg) module"]
pub type SENS_SAR_TOUCH_OUT3_REG = crate::Reg<u32, _SENS_SAR_TOUCH_OUT3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_OUT3_REG;
#[doc = "`read()` method returns [sens_sar_touch_out3_reg::R](sens_sar_touch_out3_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_OUT3_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_out3_reg::W](sens_sar_touch_out3_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_OUT3_REG {}
#[doc = "SENS_SAR_TOUCH_OUT3_REG"]
pub mod sens_sar_touch_out3_reg;
#[doc = "SENS_SAR_TOUCH_OUT4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_out4_reg](sens_sar_touch_out4_reg) module"]
pub type SENS_SAR_TOUCH_OUT4_REG = crate::Reg<u32, _SENS_SAR_TOUCH_OUT4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_OUT4_REG;
#[doc = "`read()` method returns [sens_sar_touch_out4_reg::R](sens_sar_touch_out4_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_OUT4_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_out4_reg::W](sens_sar_touch_out4_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_OUT4_REG {}
#[doc = "SENS_SAR_TOUCH_OUT4_REG"]
pub mod sens_sar_touch_out4_reg;
#[doc = "SENS_SAR_TOUCH_OUT5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_out5_reg](sens_sar_touch_out5_reg) module"]
pub type SENS_SAR_TOUCH_OUT5_REG = crate::Reg<u32, _SENS_SAR_TOUCH_OUT5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_OUT5_REG;
#[doc = "`read()` method returns [sens_sar_touch_out5_reg::R](sens_sar_touch_out5_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_OUT5_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_out5_reg::W](sens_sar_touch_out5_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_OUT5_REG {}
#[doc = "SENS_SAR_TOUCH_OUT5_REG"]
pub mod sens_sar_touch_out5_reg;
#[doc = "SENS_SAR_TOUCH_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_ctrl2_reg](sens_sar_touch_ctrl2_reg) module"]
pub type SENS_SAR_TOUCH_CTRL2_REG = crate::Reg<u32, _SENS_SAR_TOUCH_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_CTRL2_REG;
#[doc = "`read()` method returns [sens_sar_touch_ctrl2_reg::R](sens_sar_touch_ctrl2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_ctrl2_reg::W](sens_sar_touch_ctrl2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_CTRL2_REG {}
#[doc = "SENS_SAR_TOUCH_CTRL2_REG"]
pub mod sens_sar_touch_ctrl2_reg;
#[doc = "SENS_SAR_TOUCH_ENABLE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_touch_enable_reg](sens_sar_touch_enable_reg) module"]
pub type SENS_SAR_TOUCH_ENABLE_REG = crate::Reg<u32, _SENS_SAR_TOUCH_ENABLE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_TOUCH_ENABLE_REG;
#[doc = "`read()` method returns [sens_sar_touch_enable_reg::R](sens_sar_touch_enable_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_TOUCH_ENABLE_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_touch_enable_reg::W](sens_sar_touch_enable_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_TOUCH_ENABLE_REG {}
#[doc = "SENS_SAR_TOUCH_ENABLE_REG"]
pub mod sens_sar_touch_enable_reg;
#[doc = "SENS_SAR_READ_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_read_ctrl2_reg](sens_sar_read_ctrl2_reg) module"]
pub type SENS_SAR_READ_CTRL2_REG = crate::Reg<u32, _SENS_SAR_READ_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_READ_CTRL2_REG;
#[doc = "`read()` method returns [sens_sar_read_ctrl2_reg::R](sens_sar_read_ctrl2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_READ_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_read_ctrl2_reg::W](sens_sar_read_ctrl2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_READ_CTRL2_REG {}
#[doc = "SENS_SAR_READ_CTRL2_REG"]
pub mod sens_sar_read_ctrl2_reg;
#[doc = "SENS_SAR_MEAS_START2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_meas_start2_reg](sens_sar_meas_start2_reg) module"]
pub type SENS_SAR_MEAS_START2_REG = crate::Reg<u32, _SENS_SAR_MEAS_START2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEAS_START2_REG;
#[doc = "`read()` method returns [sens_sar_meas_start2_reg::R](sens_sar_meas_start2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEAS_START2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_meas_start2_reg::W](sens_sar_meas_start2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEAS_START2_REG {}
#[doc = "SENS_SAR_MEAS_START2_REG"]
pub mod sens_sar_meas_start2_reg;
#[doc = "SENS_SAR_DAC_CTRL1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_dac_ctrl1_reg](sens_sar_dac_ctrl1_reg) module"]
pub type SENS_SAR_DAC_CTRL1_REG = crate::Reg<u32, _SENS_SAR_DAC_CTRL1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_DAC_CTRL1_REG;
#[doc = "`read()` method returns [sens_sar_dac_ctrl1_reg::R](sens_sar_dac_ctrl1_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_DAC_CTRL1_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_dac_ctrl1_reg::W](sens_sar_dac_ctrl1_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_DAC_CTRL1_REG {}
#[doc = "SENS_SAR_DAC_CTRL1_REG"]
pub mod sens_sar_dac_ctrl1_reg;
#[doc = "SENS_SAR_DAC_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_dac_ctrl2_reg](sens_sar_dac_ctrl2_reg) module"]
pub type SENS_SAR_DAC_CTRL2_REG = crate::Reg<u32, _SENS_SAR_DAC_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_DAC_CTRL2_REG;
#[doc = "`read()` method returns [sens_sar_dac_ctrl2_reg::R](sens_sar_dac_ctrl2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_DAC_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_dac_ctrl2_reg::W](sens_sar_dac_ctrl2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_DAC_CTRL2_REG {}
#[doc = "SENS_SAR_DAC_CTRL2_REG"]
pub mod sens_sar_dac_ctrl2_reg;
#[doc = "SENS_SAR_MEAS_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_meas_ctrl2_reg](sens_sar_meas_ctrl2_reg) module"]
pub type SENS_SAR_MEAS_CTRL2_REG = crate::Reg<u32, _SENS_SAR_MEAS_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_MEAS_CTRL2_REG;
#[doc = "`read()` method returns [sens_sar_meas_ctrl2_reg::R](sens_sar_meas_ctrl2_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_MEAS_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_meas_ctrl2_reg::W](sens_sar_meas_ctrl2_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_MEAS_CTRL2_REG {}
#[doc = "SENS_SAR_MEAS_CTRL2_REG"]
pub mod sens_sar_meas_ctrl2_reg;
#[doc = "SENS_SAR_NOUSE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sar_nouse_reg](sens_sar_nouse_reg) module"]
pub type SENS_SAR_NOUSE_REG = crate::Reg<u32, _SENS_SAR_NOUSE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SAR_NOUSE_REG;
#[doc = "`read()` method returns [sens_sar_nouse_reg::R](sens_sar_nouse_reg::R) reader structure"]
impl crate::Readable for SENS_SAR_NOUSE_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sar_nouse_reg::W](sens_sar_nouse_reg::W) writer structure"]
impl crate::Writable for SENS_SAR_NOUSE_REG {}
#[doc = "SENS_SAR_NOUSE_REG"]
pub mod sens_sar_nouse_reg;
#[doc = "SENS_SARDATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sens_sardate_reg](sens_sardate_reg) module"]
pub type SENS_SARDATE_REG = crate::Reg<u32, _SENS_SARDATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENS_SARDATE_REG;
#[doc = "`read()` method returns [sens_sardate_reg::R](sens_sardate_reg::R) reader structure"]
impl crate::Readable for SENS_SARDATE_REG {}
#[doc = "`write(|w| ..)` method takes [sens_sardate_reg::W](sens_sardate_reg::W) writer structure"]
impl crate::Writable for SENS_SARDATE_REG {}
#[doc = "SENS_SARDATE_REG"]
pub mod sens_sardate_reg;
