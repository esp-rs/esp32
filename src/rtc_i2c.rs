#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_I2C_SCL_LOW_PERIOD_REG"]
    pub rtc_i2c_scl_low_period_reg: RTC_I2C_SCL_LOW_PERIOD_REG,
    #[doc = "0x04 - RTC_I2C_CTRL_REG"]
    pub rtc_i2c_ctrl_reg: RTC_I2C_CTRL_REG,
    #[doc = "0x08 - RTC_I2C_DEBUG_STATUS_REG"]
    pub rtc_i2c_debug_status_reg: RTC_I2C_DEBUG_STATUS_REG,
    #[doc = "0x0c - RTC_I2C_TIMEOUT_REG"]
    pub rtc_i2c_timeout_reg: RTC_I2C_TIMEOUT_REG,
    #[doc = "0x10 - RTC_I2C_SLAVE_ADDR_REG"]
    pub rtc_i2c_slave_addr_reg: RTC_I2C_SLAVE_ADDR_REG,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - RTC_I2C_INT_RAW_REG"]
    pub rtc_i2c_int_raw_reg: RTC_I2C_INT_RAW_REG,
    #[doc = "0x24 - RTC_I2C_INT_CLR_REG"]
    pub rtc_i2c_int_clr_reg: RTC_I2C_INT_CLR_REG,
    _reserved7: [u8; 8usize],
    #[doc = "0x30 - RTC_I2C_SDA_DUTY_REG"]
    pub rtc_i2c_sda_duty_reg: RTC_I2C_SDA_DUTY_REG,
    _reserved8: [u8; 4usize],
    #[doc = "0x38 - RTC_I2C_SCL_HIGH_PERIOD_REG"]
    pub rtc_i2c_scl_high_period_reg: RTC_I2C_SCL_HIGH_PERIOD_REG,
    _reserved9: [u8; 4usize],
    #[doc = "0x40 - RTC_I2C_SCL_START_PERIOD_REG"]
    pub rtc_i2c_scl_start_period_reg: RTC_I2C_SCL_START_PERIOD_REG,
    #[doc = "0x44 - RTC_I2C_SCL_STOP_PERIOD_REG"]
    pub rtc_i2c_scl_stop_period_reg: RTC_I2C_SCL_STOP_PERIOD_REG,
}
#[doc = "RTC_I2C_SCL_LOW_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_scl_low_period_reg](rtc_i2c_scl_low_period_reg) module"]
pub type RTC_I2C_SCL_LOW_PERIOD_REG = crate::Reg<u32, _RTC_I2C_SCL_LOW_PERIOD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_LOW_PERIOD_REG;
#[doc = "`read()` method returns [rtc_i2c_scl_low_period_reg::R](rtc_i2c_scl_low_period_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_LOW_PERIOD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_low_period_reg::W](rtc_i2c_scl_low_period_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_LOW_PERIOD_REG {}
#[doc = "RTC_I2C_SCL_LOW_PERIOD_REG"]
pub mod rtc_i2c_scl_low_period_reg;
#[doc = "RTC_I2C_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_ctrl_reg](rtc_i2c_ctrl_reg) module"]
pub type RTC_I2C_CTRL_REG = crate::Reg<u32, _RTC_I2C_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CTRL_REG;
#[doc = "`read()` method returns [rtc_i2c_ctrl_reg::R](rtc_i2c_ctrl_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_ctrl_reg::W](rtc_i2c_ctrl_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_CTRL_REG {}
#[doc = "RTC_I2C_CTRL_REG"]
pub mod rtc_i2c_ctrl_reg;
#[doc = "RTC_I2C_DEBUG_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_debug_status_reg](rtc_i2c_debug_status_reg) module"]
pub type RTC_I2C_DEBUG_STATUS_REG = crate::Reg<u32, _RTC_I2C_DEBUG_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_DEBUG_STATUS_REG;
#[doc = "`read()` method returns [rtc_i2c_debug_status_reg::R](rtc_i2c_debug_status_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_DEBUG_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_debug_status_reg::W](rtc_i2c_debug_status_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_DEBUG_STATUS_REG {}
#[doc = "RTC_I2C_DEBUG_STATUS_REG"]
pub mod rtc_i2c_debug_status_reg;
#[doc = "RTC_I2C_TIMEOUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_timeout_reg](rtc_i2c_timeout_reg) module"]
pub type RTC_I2C_TIMEOUT_REG = crate::Reg<u32, _RTC_I2C_TIMEOUT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_TIMEOUT_REG;
#[doc = "`read()` method returns [rtc_i2c_timeout_reg::R](rtc_i2c_timeout_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_TIMEOUT_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_timeout_reg::W](rtc_i2c_timeout_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_TIMEOUT_REG {}
#[doc = "RTC_I2C_TIMEOUT_REG"]
pub mod rtc_i2c_timeout_reg;
#[doc = "RTC_I2C_SLAVE_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_slave_addr_reg](rtc_i2c_slave_addr_reg) module"]
pub type RTC_I2C_SLAVE_ADDR_REG = crate::Reg<u32, _RTC_I2C_SLAVE_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SLAVE_ADDR_REG;
#[doc = "`read()` method returns [rtc_i2c_slave_addr_reg::R](rtc_i2c_slave_addr_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_SLAVE_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_slave_addr_reg::W](rtc_i2c_slave_addr_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_SLAVE_ADDR_REG {}
#[doc = "RTC_I2C_SLAVE_ADDR_REG"]
pub mod rtc_i2c_slave_addr_reg;
#[doc = "RTC_I2C_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_int_raw_reg](rtc_i2c_int_raw_reg) module"]
pub type RTC_I2C_INT_RAW_REG = crate::Reg<u32, _RTC_I2C_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_INT_RAW_REG;
#[doc = "`read()` method returns [rtc_i2c_int_raw_reg::R](rtc_i2c_int_raw_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_int_raw_reg::W](rtc_i2c_int_raw_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_INT_RAW_REG {}
#[doc = "RTC_I2C_INT_RAW_REG"]
pub mod rtc_i2c_int_raw_reg;
#[doc = "RTC_I2C_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_int_clr_reg](rtc_i2c_int_clr_reg) module"]
pub type RTC_I2C_INT_CLR_REG = crate::Reg<u32, _RTC_I2C_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_INT_CLR_REG;
#[doc = "`read()` method returns [rtc_i2c_int_clr_reg::R](rtc_i2c_int_clr_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_int_clr_reg::W](rtc_i2c_int_clr_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_INT_CLR_REG {}
#[doc = "RTC_I2C_INT_CLR_REG"]
pub mod rtc_i2c_int_clr_reg;
#[doc = "RTC_I2C_SDA_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_sda_duty_reg](rtc_i2c_sda_duty_reg) module"]
pub type RTC_I2C_SDA_DUTY_REG = crate::Reg<u32, _RTC_I2C_SDA_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SDA_DUTY_REG;
#[doc = "`read()` method returns [rtc_i2c_sda_duty_reg::R](rtc_i2c_sda_duty_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_SDA_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_sda_duty_reg::W](rtc_i2c_sda_duty_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_SDA_DUTY_REG {}
#[doc = "RTC_I2C_SDA_DUTY_REG"]
pub mod rtc_i2c_sda_duty_reg;
#[doc = "RTC_I2C_SCL_HIGH_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_scl_high_period_reg](rtc_i2c_scl_high_period_reg) module"]
pub type RTC_I2C_SCL_HIGH_PERIOD_REG = crate::Reg<u32, _RTC_I2C_SCL_HIGH_PERIOD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_HIGH_PERIOD_REG;
#[doc = "`read()` method returns [rtc_i2c_scl_high_period_reg::R](rtc_i2c_scl_high_period_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_HIGH_PERIOD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_high_period_reg::W](rtc_i2c_scl_high_period_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_HIGH_PERIOD_REG {}
#[doc = "RTC_I2C_SCL_HIGH_PERIOD_REG"]
pub mod rtc_i2c_scl_high_period_reg;
#[doc = "RTC_I2C_SCL_START_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_scl_start_period_reg](rtc_i2c_scl_start_period_reg) module"]
pub type RTC_I2C_SCL_START_PERIOD_REG = crate::Reg<u32, _RTC_I2C_SCL_START_PERIOD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_START_PERIOD_REG;
#[doc = "`read()` method returns [rtc_i2c_scl_start_period_reg::R](rtc_i2c_scl_start_period_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_START_PERIOD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_start_period_reg::W](rtc_i2c_scl_start_period_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_START_PERIOD_REG {}
#[doc = "RTC_I2C_SCL_START_PERIOD_REG"]
pub mod rtc_i2c_scl_start_period_reg;
#[doc = "RTC_I2C_SCL_STOP_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_i2c_scl_stop_period_reg](rtc_i2c_scl_stop_period_reg) module"]
pub type RTC_I2C_SCL_STOP_PERIOD_REG = crate::Reg<u32, _RTC_I2C_SCL_STOP_PERIOD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_STOP_PERIOD_REG;
#[doc = "`read()` method returns [rtc_i2c_scl_stop_period_reg::R](rtc_i2c_scl_stop_period_reg::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_STOP_PERIOD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_stop_period_reg::W](rtc_i2c_scl_stop_period_reg::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_STOP_PERIOD_REG {}
#[doc = "RTC_I2C_SCL_STOP_PERIOD_REG"]
pub mod rtc_i2c_scl_stop_period_reg;
