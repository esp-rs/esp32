#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SENS_SAR_READ_CTRL"]
    pub sar_read_ctrl: SAR_READ_CTRL,
    #[doc = "0x04 - SENS_SAR_READ_STATUS1"]
    pub sar_read_status1: SAR_READ_STATUS1,
    #[doc = "0x08 - SENS_SAR_MEAS_WAIT1"]
    pub sar_meas_wait1: SAR_MEAS_WAIT1,
    #[doc = "0x0c - SENS_SAR_MEAS_WAIT2"]
    pub sar_meas_wait2: SAR_MEAS_WAIT2,
    #[doc = "0x10 - SENS_SAR_MEAS_CTRL"]
    pub sar_meas_ctrl: SAR_MEAS_CTRL,
    #[doc = "0x14 - SENS_SAR_READ_STATUS2"]
    pub sar_read_status2: SAR_READ_STATUS2,
    #[doc = "0x18 - SENS_ULP_CP_SLEEP_CYC0"]
    pub ulp_cp_sleep_cyc0: ULP_CP_SLEEP_CYC0,
    #[doc = "0x1c - SENS_ULP_CP_SLEEP_CYC1"]
    pub ulp_cp_sleep_cyc1: ULP_CP_SLEEP_CYC1,
    #[doc = "0x20 - SENS_ULP_CP_SLEEP_CYC2"]
    pub ulp_cp_sleep_cyc2: ULP_CP_SLEEP_CYC2,
    #[doc = "0x24 - SENS_ULP_CP_SLEEP_CYC3"]
    pub ulp_cp_sleep_cyc3: ULP_CP_SLEEP_CYC3,
    #[doc = "0x28 - SENS_ULP_CP_SLEEP_CYC4"]
    pub ulp_cp_sleep_cyc4: ULP_CP_SLEEP_CYC4,
    #[doc = "0x2c - SENS_SAR_START_FORCE"]
    pub sar_start_force: SAR_START_FORCE,
    #[doc = "0x30 - SENS_SAR_MEM_WR_CTRL"]
    pub sar_mem_wr_ctrl: SAR_MEM_WR_CTRL,
    #[doc = "0x34 - SENS_SAR_ATTEN1"]
    pub sar_atten1: SAR_ATTEN1,
    #[doc = "0x38 - SENS_SAR_ATTEN2"]
    pub sar_atten2: SAR_ATTEN2,
    #[doc = "0x3c - SENS_SAR_SLAVE_ADDR1"]
    pub sar_slave_addr1: SAR_SLAVE_ADDR1,
    #[doc = "0x40 - SENS_SAR_SLAVE_ADDR2"]
    pub sar_slave_addr2: SAR_SLAVE_ADDR2,
    #[doc = "0x44 - SENS_SAR_SLAVE_ADDR3"]
    pub sar_slave_addr3: SAR_SLAVE_ADDR3,
    #[doc = "0x48 - SENS_SAR_SLAVE_ADDR4"]
    pub sar_slave_addr4: SAR_SLAVE_ADDR4,
    #[doc = "0x4c - SENS_SAR_TSENS_CTRL"]
    pub sar_tsens_ctrl: SAR_TSENS_CTRL,
    #[doc = "0x50 - SENS_SAR_I2C_CTRL"]
    pub sar_i2c_ctrl: SAR_I2C_CTRL,
    #[doc = "0x54 - SENS_SAR_MEAS_START1"]
    pub sar_meas_start1: SAR_MEAS_START1,
    #[doc = "0x58 - SENS_SAR_TOUCH_CTRL1"]
    pub sar_touch_ctrl1: SAR_TOUCH_CTRL1,
    #[doc = "0x5c - SENS_SAR_TOUCH_THRES1"]
    pub sar_touch_thres1: SAR_TOUCH_THRES1,
    #[doc = "0x60 - SENS_SAR_TOUCH_THRES2"]
    pub sar_touch_thres2: SAR_TOUCH_THRES2,
    #[doc = "0x64 - SENS_SAR_TOUCH_THRES3"]
    pub sar_touch_thres3: SAR_TOUCH_THRES3,
    #[doc = "0x68 - SENS_SAR_TOUCH_THRES4"]
    pub sar_touch_thres4: SAR_TOUCH_THRES4,
    #[doc = "0x6c - SENS_SAR_TOUCH_THRES5"]
    pub sar_touch_thres5: SAR_TOUCH_THRES5,
    #[doc = "0x70 - SENS_SAR_TOUCH_OUT1"]
    pub sar_touch_out1: SAR_TOUCH_OUT1,
    #[doc = "0x74 - SENS_SAR_TOUCH_OUT2"]
    pub sar_touch_out2: SAR_TOUCH_OUT2,
    #[doc = "0x78 - SENS_SAR_TOUCH_OUT3"]
    pub sar_touch_out3: SAR_TOUCH_OUT3,
    #[doc = "0x7c - SENS_SAR_TOUCH_OUT4"]
    pub sar_touch_out4: SAR_TOUCH_OUT4,
    #[doc = "0x80 - SENS_SAR_TOUCH_OUT5"]
    pub sar_touch_out5: SAR_TOUCH_OUT5,
    #[doc = "0x84 - SENS_SAR_TOUCH_CTRL2"]
    pub sar_touch_ctrl2: SAR_TOUCH_CTRL2,
    _reserved34: [u8; 4usize],
    #[doc = "0x8c - SENS_SAR_TOUCH_ENABLE"]
    pub sar_touch_enable: SAR_TOUCH_ENABLE,
    #[doc = "0x90 - SENS_SAR_READ_CTRL2"]
    pub sar_read_ctrl2: SAR_READ_CTRL2,
    #[doc = "0x94 - SENS_SAR_MEAS_START2"]
    pub sar_meas_start2: SAR_MEAS_START2,
    #[doc = "0x98 - SENS_SAR_DAC_CTRL1"]
    pub sar_dac_ctrl1: SAR_DAC_CTRL1,
    #[doc = "0x9c - SENS_SAR_DAC_CTRL2"]
    pub sar_dac_ctrl2: SAR_DAC_CTRL2,
    #[doc = "0xa0 - SENS_SAR_MEAS_CTRL2"]
    pub sar_meas_ctrl2: SAR_MEAS_CTRL2,
    _reserved40: [u8; 84usize],
    #[doc = "0xf8 - SENS_SAR_NOUSE"]
    pub sar_nouse: SAR_NOUSE,
    #[doc = "0xfc - SENS_SARDATE"]
    pub sardate: SARDATE,
}
#[doc = "SENS_SAR_READ_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_read_ctrl](sar_read_ctrl) module"]
pub type SAR_READ_CTRL = crate::Reg<u32, _SAR_READ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_READ_CTRL;
#[doc = "`read()` method returns [sar_read_ctrl::R](sar_read_ctrl::R) reader structure"]
impl crate::Readable for SAR_READ_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_read_ctrl::W](sar_read_ctrl::W) writer structure"]
impl crate::Writable for SAR_READ_CTRL {}
#[doc = "SENS_SAR_READ_CTRL"]
pub mod sar_read_ctrl;
#[doc = "SENS_SAR_READ_STATUS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_read_status1](sar_read_status1) module"]
pub type SAR_READ_STATUS1 = crate::Reg<u32, _SAR_READ_STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_READ_STATUS1;
#[doc = "`read()` method returns [sar_read_status1::R](sar_read_status1::R) reader structure"]
impl crate::Readable for SAR_READ_STATUS1 {}
#[doc = "`write(|w| ..)` method takes [sar_read_status1::W](sar_read_status1::W) writer structure"]
impl crate::Writable for SAR_READ_STATUS1 {}
#[doc = "SENS_SAR_READ_STATUS1"]
pub mod sar_read_status1;
#[doc = "SENS_SAR_MEAS_WAIT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_meas_wait1](sar_meas_wait1) module"]
pub type SAR_MEAS_WAIT1 = crate::Reg<u32, _SAR_MEAS_WAIT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEAS_WAIT1;
#[doc = "`read()` method returns [sar_meas_wait1::R](sar_meas_wait1::R) reader structure"]
impl crate::Readable for SAR_MEAS_WAIT1 {}
#[doc = "`write(|w| ..)` method takes [sar_meas_wait1::W](sar_meas_wait1::W) writer structure"]
impl crate::Writable for SAR_MEAS_WAIT1 {}
#[doc = "SENS_SAR_MEAS_WAIT1"]
pub mod sar_meas_wait1;
#[doc = "SENS_SAR_MEAS_WAIT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_meas_wait2](sar_meas_wait2) module"]
pub type SAR_MEAS_WAIT2 = crate::Reg<u32, _SAR_MEAS_WAIT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEAS_WAIT2;
#[doc = "`read()` method returns [sar_meas_wait2::R](sar_meas_wait2::R) reader structure"]
impl crate::Readable for SAR_MEAS_WAIT2 {}
#[doc = "`write(|w| ..)` method takes [sar_meas_wait2::W](sar_meas_wait2::W) writer structure"]
impl crate::Writable for SAR_MEAS_WAIT2 {}
#[doc = "SENS_SAR_MEAS_WAIT2"]
pub mod sar_meas_wait2;
#[doc = "SENS_SAR_MEAS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_meas_ctrl](sar_meas_ctrl) module"]
pub type SAR_MEAS_CTRL = crate::Reg<u32, _SAR_MEAS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEAS_CTRL;
#[doc = "`read()` method returns [sar_meas_ctrl::R](sar_meas_ctrl::R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_meas_ctrl::W](sar_meas_ctrl::W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL {}
#[doc = "SENS_SAR_MEAS_CTRL"]
pub mod sar_meas_ctrl;
#[doc = "SENS_SAR_READ_STATUS2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_read_status2](sar_read_status2) module"]
pub type SAR_READ_STATUS2 = crate::Reg<u32, _SAR_READ_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_READ_STATUS2;
#[doc = "`read()` method returns [sar_read_status2::R](sar_read_status2::R) reader structure"]
impl crate::Readable for SAR_READ_STATUS2 {}
#[doc = "`write(|w| ..)` method takes [sar_read_status2::W](sar_read_status2::W) writer structure"]
impl crate::Writable for SAR_READ_STATUS2 {}
#[doc = "SENS_SAR_READ_STATUS2"]
pub mod sar_read_status2;
#[doc = "SENS_ULP_CP_SLEEP_CYC0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ulp_cp_sleep_cyc0](ulp_cp_sleep_cyc0) module"]
pub type ULP_CP_SLEEP_CYC0 = crate::Reg<u32, _ULP_CP_SLEEP_CYC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULP_CP_SLEEP_CYC0;
#[doc = "`read()` method returns [ulp_cp_sleep_cyc0::R](ulp_cp_sleep_cyc0::R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC0 {}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc0::W](ulp_cp_sleep_cyc0::W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC0 {}
#[doc = "SENS_ULP_CP_SLEEP_CYC0"]
pub mod ulp_cp_sleep_cyc0;
#[doc = "SENS_ULP_CP_SLEEP_CYC1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ulp_cp_sleep_cyc1](ulp_cp_sleep_cyc1) module"]
pub type ULP_CP_SLEEP_CYC1 = crate::Reg<u32, _ULP_CP_SLEEP_CYC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULP_CP_SLEEP_CYC1;
#[doc = "`read()` method returns [ulp_cp_sleep_cyc1::R](ulp_cp_sleep_cyc1::R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC1 {}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc1::W](ulp_cp_sleep_cyc1::W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC1 {}
#[doc = "SENS_ULP_CP_SLEEP_CYC1"]
pub mod ulp_cp_sleep_cyc1;
#[doc = "SENS_ULP_CP_SLEEP_CYC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ulp_cp_sleep_cyc2](ulp_cp_sleep_cyc2) module"]
pub type ULP_CP_SLEEP_CYC2 = crate::Reg<u32, _ULP_CP_SLEEP_CYC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULP_CP_SLEEP_CYC2;
#[doc = "`read()` method returns [ulp_cp_sleep_cyc2::R](ulp_cp_sleep_cyc2::R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC2 {}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc2::W](ulp_cp_sleep_cyc2::W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC2 {}
#[doc = "SENS_ULP_CP_SLEEP_CYC2"]
pub mod ulp_cp_sleep_cyc2;
#[doc = "SENS_ULP_CP_SLEEP_CYC3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ulp_cp_sleep_cyc3](ulp_cp_sleep_cyc3) module"]
pub type ULP_CP_SLEEP_CYC3 = crate::Reg<u32, _ULP_CP_SLEEP_CYC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULP_CP_SLEEP_CYC3;
#[doc = "`read()` method returns [ulp_cp_sleep_cyc3::R](ulp_cp_sleep_cyc3::R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC3 {}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc3::W](ulp_cp_sleep_cyc3::W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC3 {}
#[doc = "SENS_ULP_CP_SLEEP_CYC3"]
pub mod ulp_cp_sleep_cyc3;
#[doc = "SENS_ULP_CP_SLEEP_CYC4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ulp_cp_sleep_cyc4](ulp_cp_sleep_cyc4) module"]
pub type ULP_CP_SLEEP_CYC4 = crate::Reg<u32, _ULP_CP_SLEEP_CYC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULP_CP_SLEEP_CYC4;
#[doc = "`read()` method returns [ulp_cp_sleep_cyc4::R](ulp_cp_sleep_cyc4::R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC4 {}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc4::W](ulp_cp_sleep_cyc4::W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC4 {}
#[doc = "SENS_ULP_CP_SLEEP_CYC4"]
pub mod ulp_cp_sleep_cyc4;
#[doc = "SENS_SAR_START_FORCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_start_force](sar_start_force) module"]
pub type SAR_START_FORCE = crate::Reg<u32, _SAR_START_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_START_FORCE;
#[doc = "`read()` method returns [sar_start_force::R](sar_start_force::R) reader structure"]
impl crate::Readable for SAR_START_FORCE {}
#[doc = "`write(|w| ..)` method takes [sar_start_force::W](sar_start_force::W) writer structure"]
impl crate::Writable for SAR_START_FORCE {}
#[doc = "SENS_SAR_START_FORCE"]
pub mod sar_start_force;
#[doc = "SENS_SAR_MEM_WR_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_mem_wr_ctrl](sar_mem_wr_ctrl) module"]
pub type SAR_MEM_WR_CTRL = crate::Reg<u32, _SAR_MEM_WR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEM_WR_CTRL;
#[doc = "`read()` method returns [sar_mem_wr_ctrl::R](sar_mem_wr_ctrl::R) reader structure"]
impl crate::Readable for SAR_MEM_WR_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_mem_wr_ctrl::W](sar_mem_wr_ctrl::W) writer structure"]
impl crate::Writable for SAR_MEM_WR_CTRL {}
#[doc = "SENS_SAR_MEM_WR_CTRL"]
pub mod sar_mem_wr_ctrl;
#[doc = "SENS_SAR_ATTEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_atten1](sar_atten1) module"]
pub type SAR_ATTEN1 = crate::Reg<u32, _SAR_ATTEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_ATTEN1;
#[doc = "`read()` method returns [sar_atten1::R](sar_atten1::R) reader structure"]
impl crate::Readable for SAR_ATTEN1 {}
#[doc = "`write(|w| ..)` method takes [sar_atten1::W](sar_atten1::W) writer structure"]
impl crate::Writable for SAR_ATTEN1 {}
#[doc = "SENS_SAR_ATTEN1"]
pub mod sar_atten1;
#[doc = "SENS_SAR_ATTEN2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_atten2](sar_atten2) module"]
pub type SAR_ATTEN2 = crate::Reg<u32, _SAR_ATTEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_ATTEN2;
#[doc = "`read()` method returns [sar_atten2::R](sar_atten2::R) reader structure"]
impl crate::Readable for SAR_ATTEN2 {}
#[doc = "`write(|w| ..)` method takes [sar_atten2::W](sar_atten2::W) writer structure"]
impl crate::Writable for SAR_ATTEN2 {}
#[doc = "SENS_SAR_ATTEN2"]
pub mod sar_atten2;
#[doc = "SENS_SAR_SLAVE_ADDR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_slave_addr1](sar_slave_addr1) module"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<u32, _SAR_SLAVE_ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_SLAVE_ADDR1;
#[doc = "`read()` method returns [sar_slave_addr1::R](sar_slave_addr1::R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR1 {}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr1::W](sar_slave_addr1::W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR1 {}
#[doc = "SENS_SAR_SLAVE_ADDR1"]
pub mod sar_slave_addr1;
#[doc = "SENS_SAR_SLAVE_ADDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_slave_addr2](sar_slave_addr2) module"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<u32, _SAR_SLAVE_ADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_SLAVE_ADDR2;
#[doc = "`read()` method returns [sar_slave_addr2::R](sar_slave_addr2::R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR2 {}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr2::W](sar_slave_addr2::W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR2 {}
#[doc = "SENS_SAR_SLAVE_ADDR2"]
pub mod sar_slave_addr2;
#[doc = "SENS_SAR_SLAVE_ADDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_slave_addr3](sar_slave_addr3) module"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<u32, _SAR_SLAVE_ADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_SLAVE_ADDR3;
#[doc = "`read()` method returns [sar_slave_addr3::R](sar_slave_addr3::R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR3 {}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr3::W](sar_slave_addr3::W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR3 {}
#[doc = "SENS_SAR_SLAVE_ADDR3"]
pub mod sar_slave_addr3;
#[doc = "SENS_SAR_SLAVE_ADDR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_slave_addr4](sar_slave_addr4) module"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<u32, _SAR_SLAVE_ADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_SLAVE_ADDR4;
#[doc = "`read()` method returns [sar_slave_addr4::R](sar_slave_addr4::R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR4 {}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr4::W](sar_slave_addr4::W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR4 {}
#[doc = "SENS_SAR_SLAVE_ADDR4"]
pub mod sar_slave_addr4;
#[doc = "SENS_SAR_TSENS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_tsens_ctrl](sar_tsens_ctrl) module"]
pub type SAR_TSENS_CTRL = crate::Reg<u32, _SAR_TSENS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TSENS_CTRL;
#[doc = "`read()` method returns [sar_tsens_ctrl::R](sar_tsens_ctrl::R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_tsens_ctrl::W](sar_tsens_ctrl::W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL {}
#[doc = "SENS_SAR_TSENS_CTRL"]
pub mod sar_tsens_ctrl;
#[doc = "SENS_SAR_I2C_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_i2c_ctrl](sar_i2c_ctrl) module"]
pub type SAR_I2C_CTRL = crate::Reg<u32, _SAR_I2C_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_I2C_CTRL;
#[doc = "`read()` method returns [sar_i2c_ctrl::R](sar_i2c_ctrl::R) reader structure"]
impl crate::Readable for SAR_I2C_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_i2c_ctrl::W](sar_i2c_ctrl::W) writer structure"]
impl crate::Writable for SAR_I2C_CTRL {}
#[doc = "SENS_SAR_I2C_CTRL"]
pub mod sar_i2c_ctrl;
#[doc = "SENS_SAR_MEAS_START1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_meas_start1](sar_meas_start1) module"]
pub type SAR_MEAS_START1 = crate::Reg<u32, _SAR_MEAS_START1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEAS_START1;
#[doc = "`read()` method returns [sar_meas_start1::R](sar_meas_start1::R) reader structure"]
impl crate::Readable for SAR_MEAS_START1 {}
#[doc = "`write(|w| ..)` method takes [sar_meas_start1::W](sar_meas_start1::W) writer structure"]
impl crate::Writable for SAR_MEAS_START1 {}
#[doc = "SENS_SAR_MEAS_START1"]
pub mod sar_meas_start1;
#[doc = "SENS_SAR_TOUCH_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_ctrl1](sar_touch_ctrl1) module"]
pub type SAR_TOUCH_CTRL1 = crate::Reg<u32, _SAR_TOUCH_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_CTRL1;
#[doc = "`read()` method returns [sar_touch_ctrl1::R](sar_touch_ctrl1::R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl1::W](sar_touch_ctrl1::W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL1 {}
#[doc = "SENS_SAR_TOUCH_CTRL1"]
pub mod sar_touch_ctrl1;
#[doc = "SENS_SAR_TOUCH_THRES1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_thres1](sar_touch_thres1) module"]
pub type SAR_TOUCH_THRES1 = crate::Reg<u32, _SAR_TOUCH_THRES1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_THRES1;
#[doc = "`read()` method returns [sar_touch_thres1::R](sar_touch_thres1::R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES1 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres1::W](sar_touch_thres1::W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES1 {}
#[doc = "SENS_SAR_TOUCH_THRES1"]
pub mod sar_touch_thres1;
#[doc = "SENS_SAR_TOUCH_THRES2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_thres2](sar_touch_thres2) module"]
pub type SAR_TOUCH_THRES2 = crate::Reg<u32, _SAR_TOUCH_THRES2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_THRES2;
#[doc = "`read()` method returns [sar_touch_thres2::R](sar_touch_thres2::R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES2 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres2::W](sar_touch_thres2::W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES2 {}
#[doc = "SENS_SAR_TOUCH_THRES2"]
pub mod sar_touch_thres2;
#[doc = "SENS_SAR_TOUCH_THRES3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_thres3](sar_touch_thres3) module"]
pub type SAR_TOUCH_THRES3 = crate::Reg<u32, _SAR_TOUCH_THRES3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_THRES3;
#[doc = "`read()` method returns [sar_touch_thres3::R](sar_touch_thres3::R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES3 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres3::W](sar_touch_thres3::W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES3 {}
#[doc = "SENS_SAR_TOUCH_THRES3"]
pub mod sar_touch_thres3;
#[doc = "SENS_SAR_TOUCH_THRES4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_thres4](sar_touch_thres4) module"]
pub type SAR_TOUCH_THRES4 = crate::Reg<u32, _SAR_TOUCH_THRES4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_THRES4;
#[doc = "`read()` method returns [sar_touch_thres4::R](sar_touch_thres4::R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES4 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres4::W](sar_touch_thres4::W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES4 {}
#[doc = "SENS_SAR_TOUCH_THRES4"]
pub mod sar_touch_thres4;
#[doc = "SENS_SAR_TOUCH_THRES5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_thres5](sar_touch_thres5) module"]
pub type SAR_TOUCH_THRES5 = crate::Reg<u32, _SAR_TOUCH_THRES5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_THRES5;
#[doc = "`read()` method returns [sar_touch_thres5::R](sar_touch_thres5::R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES5 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres5::W](sar_touch_thres5::W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES5 {}
#[doc = "SENS_SAR_TOUCH_THRES5"]
pub mod sar_touch_thres5;
#[doc = "SENS_SAR_TOUCH_OUT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_out1](sar_touch_out1) module"]
pub type SAR_TOUCH_OUT1 = crate::Reg<u32, _SAR_TOUCH_OUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_OUT1;
#[doc = "`read()` method returns [sar_touch_out1::R](sar_touch_out1::R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT1 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_out1::W](sar_touch_out1::W) writer structure"]
impl crate::Writable for SAR_TOUCH_OUT1 {}
#[doc = "SENS_SAR_TOUCH_OUT1"]
pub mod sar_touch_out1;
#[doc = "SENS_SAR_TOUCH_OUT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_out2](sar_touch_out2) module"]
pub type SAR_TOUCH_OUT2 = crate::Reg<u32, _SAR_TOUCH_OUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_OUT2;
#[doc = "`read()` method returns [sar_touch_out2::R](sar_touch_out2::R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT2 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_out2::W](sar_touch_out2::W) writer structure"]
impl crate::Writable for SAR_TOUCH_OUT2 {}
#[doc = "SENS_SAR_TOUCH_OUT2"]
pub mod sar_touch_out2;
#[doc = "SENS_SAR_TOUCH_OUT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_out3](sar_touch_out3) module"]
pub type SAR_TOUCH_OUT3 = crate::Reg<u32, _SAR_TOUCH_OUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_OUT3;
#[doc = "`read()` method returns [sar_touch_out3::R](sar_touch_out3::R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT3 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_out3::W](sar_touch_out3::W) writer structure"]
impl crate::Writable for SAR_TOUCH_OUT3 {}
#[doc = "SENS_SAR_TOUCH_OUT3"]
pub mod sar_touch_out3;
#[doc = "SENS_SAR_TOUCH_OUT4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_out4](sar_touch_out4) module"]
pub type SAR_TOUCH_OUT4 = crate::Reg<u32, _SAR_TOUCH_OUT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_OUT4;
#[doc = "`read()` method returns [sar_touch_out4::R](sar_touch_out4::R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT4 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_out4::W](sar_touch_out4::W) writer structure"]
impl crate::Writable for SAR_TOUCH_OUT4 {}
#[doc = "SENS_SAR_TOUCH_OUT4"]
pub mod sar_touch_out4;
#[doc = "SENS_SAR_TOUCH_OUT5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_out5](sar_touch_out5) module"]
pub type SAR_TOUCH_OUT5 = crate::Reg<u32, _SAR_TOUCH_OUT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_OUT5;
#[doc = "`read()` method returns [sar_touch_out5::R](sar_touch_out5::R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT5 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_out5::W](sar_touch_out5::W) writer structure"]
impl crate::Writable for SAR_TOUCH_OUT5 {}
#[doc = "SENS_SAR_TOUCH_OUT5"]
pub mod sar_touch_out5;
#[doc = "SENS_SAR_TOUCH_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_ctrl2](sar_touch_ctrl2) module"]
pub type SAR_TOUCH_CTRL2 = crate::Reg<u32, _SAR_TOUCH_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_CTRL2;
#[doc = "`read()` method returns [sar_touch_ctrl2::R](sar_touch_ctrl2::R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl2::W](sar_touch_ctrl2::W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL2 {}
#[doc = "SENS_SAR_TOUCH_CTRL2"]
pub mod sar_touch_ctrl2;
#[doc = "SENS_SAR_TOUCH_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_touch_enable](sar_touch_enable) module"]
pub type SAR_TOUCH_ENABLE = crate::Reg<u32, _SAR_TOUCH_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TOUCH_ENABLE;
#[doc = "`read()` method returns [sar_touch_enable::R](sar_touch_enable::R) reader structure"]
impl crate::Readable for SAR_TOUCH_ENABLE {}
#[doc = "`write(|w| ..)` method takes [sar_touch_enable::W](sar_touch_enable::W) writer structure"]
impl crate::Writable for SAR_TOUCH_ENABLE {}
#[doc = "SENS_SAR_TOUCH_ENABLE"]
pub mod sar_touch_enable;
#[doc = "SENS_SAR_READ_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_read_ctrl2](sar_read_ctrl2) module"]
pub type SAR_READ_CTRL2 = crate::Reg<u32, _SAR_READ_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_READ_CTRL2;
#[doc = "`read()` method returns [sar_read_ctrl2::R](sar_read_ctrl2::R) reader structure"]
impl crate::Readable for SAR_READ_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sar_read_ctrl2::W](sar_read_ctrl2::W) writer structure"]
impl crate::Writable for SAR_READ_CTRL2 {}
#[doc = "SENS_SAR_READ_CTRL2"]
pub mod sar_read_ctrl2;
#[doc = "SENS_SAR_MEAS_START2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_meas_start2](sar_meas_start2) module"]
pub type SAR_MEAS_START2 = crate::Reg<u32, _SAR_MEAS_START2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEAS_START2;
#[doc = "`read()` method returns [sar_meas_start2::R](sar_meas_start2::R) reader structure"]
impl crate::Readable for SAR_MEAS_START2 {}
#[doc = "`write(|w| ..)` method takes [sar_meas_start2::W](sar_meas_start2::W) writer structure"]
impl crate::Writable for SAR_MEAS_START2 {}
#[doc = "SENS_SAR_MEAS_START2"]
pub mod sar_meas_start2;
#[doc = "SENS_SAR_DAC_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_dac_ctrl1](sar_dac_ctrl1) module"]
pub type SAR_DAC_CTRL1 = crate::Reg<u32, _SAR_DAC_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_DAC_CTRL1;
#[doc = "`read()` method returns [sar_dac_ctrl1::R](sar_dac_ctrl1::R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [sar_dac_ctrl1::W](sar_dac_ctrl1::W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL1 {}
#[doc = "SENS_SAR_DAC_CTRL1"]
pub mod sar_dac_ctrl1;
#[doc = "SENS_SAR_DAC_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_dac_ctrl2](sar_dac_ctrl2) module"]
pub type SAR_DAC_CTRL2 = crate::Reg<u32, _SAR_DAC_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_DAC_CTRL2;
#[doc = "`read()` method returns [sar_dac_ctrl2::R](sar_dac_ctrl2::R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sar_dac_ctrl2::W](sar_dac_ctrl2::W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL2 {}
#[doc = "SENS_SAR_DAC_CTRL2"]
pub mod sar_dac_ctrl2;
#[doc = "SENS_SAR_MEAS_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_meas_ctrl2](sar_meas_ctrl2) module"]
pub type SAR_MEAS_CTRL2 = crate::Reg<u32, _SAR_MEAS_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_MEAS_CTRL2;
#[doc = "`read()` method returns [sar_meas_ctrl2::R](sar_meas_ctrl2::R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sar_meas_ctrl2::W](sar_meas_ctrl2::W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL2 {}
#[doc = "SENS_SAR_MEAS_CTRL2"]
pub mod sar_meas_ctrl2;
#[doc = "SENS_SAR_NOUSE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar_nouse](sar_nouse) module"]
pub type SAR_NOUSE = crate::Reg<u32, _SAR_NOUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_NOUSE;
#[doc = "`read()` method returns [sar_nouse::R](sar_nouse::R) reader structure"]
impl crate::Readable for SAR_NOUSE {}
#[doc = "`write(|w| ..)` method takes [sar_nouse::W](sar_nouse::W) writer structure"]
impl crate::Writable for SAR_NOUSE {}
#[doc = "SENS_SAR_NOUSE"]
pub mod sar_nouse;
#[doc = "SENS_SARDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sardate](sardate) module"]
pub type SARDATE = crate::Reg<u32, _SARDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARDATE;
#[doc = "`read()` method returns [sardate::R](sardate::R) reader structure"]
impl crate::Readable for SARDATE {}
#[doc = "`write(|w| ..)` method takes [sardate::W](sardate::W) writer structure"]
impl crate::Writable for SARDATE {}
#[doc = "SENS_SARDATE"]
pub mod sardate;
