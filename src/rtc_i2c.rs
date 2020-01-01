#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_I2C_SCL_LOW_PERIOD_REG"]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - RTC_I2C_CTRL_REG"]
    pub ctrl: CTRL,
    #[doc = "0x08 - RTC_I2C_DEBUG_STATUS_REG"]
    pub debug_status: DEBUG_STATUS,
    #[doc = "0x0c - RTC_I2C_TIMEOUT_REG"]
    pub timeout: TIMEOUT,
    #[doc = "0x10 - RTC_I2C_SLAVE_ADDR_REG"]
    pub slave_addr: SLAVE_ADDR,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - RTC_I2C_INT_RAW_REG"]
    pub int_raw: INT_RAW,
    #[doc = "0x24 - RTC_I2C_INT_CLR_REG"]
    pub int_clr: INT_CLR,
    _reserved7: [u8; 8usize],
    #[doc = "0x30 - RTC_I2C_SDA_DUTY_REG"]
    pub sda_duty: SDA_DUTY,
    _reserved8: [u8; 4usize],
    #[doc = "0x38 - RTC_I2C_SCL_HIGH_PERIOD_REG"]
    pub scl_high_period: SCL_HIGH_PERIOD,
    _reserved9: [u8; 4usize],
    #[doc = "0x40 - RTC_I2C_SCL_START_PERIOD_REG"]
    pub scl_start_period: SCL_START_PERIOD,
    #[doc = "0x44 - RTC_I2C_SCL_STOP_PERIOD_REG"]
    pub scl_stop_period: SCL_STOP_PERIOD,
}
#[doc = "RTC_I2C_SCL_LOW_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_low_period](scl_low_period) module"]
pub type SCL_LOW_PERIOD = crate::Reg<u32, _SCL_LOW_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_LOW_PERIOD;
#[doc = "`read()` method returns [scl_low_period::R](scl_low_period::R) reader structure"]
impl crate::Readable for SCL_LOW_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_low_period::W](scl_low_period::W) writer structure"]
impl crate::Writable for SCL_LOW_PERIOD {}
#[doc = "RTC_I2C_SCL_LOW_PERIOD_REG"]
pub mod scl_low_period;
#[doc = "RTC_I2C_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RTC_I2C_CTRL_REG"]
pub mod ctrl;
#[doc = "RTC_I2C_DEBUG_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug_status](debug_status) module"]
pub type DEBUG_STATUS = crate::Reg<u32, _DEBUG_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_STATUS;
#[doc = "`read()` method returns [debug_status::R](debug_status::R) reader structure"]
impl crate::Readable for DEBUG_STATUS {}
#[doc = "`write(|w| ..)` method takes [debug_status::W](debug_status::W) writer structure"]
impl crate::Writable for DEBUG_STATUS {}
#[doc = "RTC_I2C_DEBUG_STATUS_REG"]
pub mod debug_status;
#[doc = "RTC_I2C_TIMEOUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timeout](timeout) module"]
pub type TIMEOUT = crate::Reg<u32, _TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT;
#[doc = "`read()` method returns [timeout::R](timeout::R) reader structure"]
impl crate::Readable for TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [timeout::W](timeout::W) writer structure"]
impl crate::Writable for TIMEOUT {}
#[doc = "RTC_I2C_TIMEOUT_REG"]
pub mod timeout;
#[doc = "RTC_I2C_SLAVE_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave_addr](slave_addr) module"]
pub type SLAVE_ADDR = crate::Reg<u32, _SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_ADDR;
#[doc = "`read()` method returns [slave_addr::R](slave_addr::R) reader structure"]
impl crate::Readable for SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](slave_addr::W) writer structure"]
impl crate::Writable for SLAVE_ADDR {}
#[doc = "RTC_I2C_SLAVE_ADDR_REG"]
pub mod slave_addr;
#[doc = "RTC_I2C_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "RTC_I2C_INT_RAW_REG"]
pub mod int_raw;
#[doc = "RTC_I2C_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "RTC_I2C_INT_CLR_REG"]
pub mod int_clr;
#[doc = "RTC_I2C_SDA_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda_duty](sda_duty) module"]
pub type SDA_DUTY = crate::Reg<u32, _SDA_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_DUTY;
#[doc = "`read()` method returns [sda_duty::R](sda_duty::R) reader structure"]
impl crate::Readable for SDA_DUTY {}
#[doc = "`write(|w| ..)` method takes [sda_duty::W](sda_duty::W) writer structure"]
impl crate::Writable for SDA_DUTY {}
#[doc = "RTC_I2C_SDA_DUTY_REG"]
pub mod sda_duty;
#[doc = "RTC_I2C_SCL_HIGH_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_high_period](scl_high_period) module"]
pub type SCL_HIGH_PERIOD = crate::Reg<u32, _SCL_HIGH_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_HIGH_PERIOD;
#[doc = "`read()` method returns [scl_high_period::R](scl_high_period::R) reader structure"]
impl crate::Readable for SCL_HIGH_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_high_period::W](scl_high_period::W) writer structure"]
impl crate::Writable for SCL_HIGH_PERIOD {}
#[doc = "RTC_I2C_SCL_HIGH_PERIOD_REG"]
pub mod scl_high_period;
#[doc = "RTC_I2C_SCL_START_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_start_period](scl_start_period) module"]
pub type SCL_START_PERIOD = crate::Reg<u32, _SCL_START_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_START_PERIOD;
#[doc = "`read()` method returns [scl_start_period::R](scl_start_period::R) reader structure"]
impl crate::Readable for SCL_START_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_start_period::W](scl_start_period::W) writer structure"]
impl crate::Writable for SCL_START_PERIOD {}
#[doc = "RTC_I2C_SCL_START_PERIOD_REG"]
pub mod scl_start_period;
#[doc = "RTC_I2C_SCL_STOP_PERIOD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_stop_period](scl_stop_period) module"]
pub type SCL_STOP_PERIOD = crate::Reg<u32, _SCL_STOP_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STOP_PERIOD;
#[doc = "`read()` method returns [scl_stop_period::R](scl_stop_period::R) reader structure"]
impl crate::Readable for SCL_STOP_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_stop_period::W](scl_stop_period::W) writer structure"]
impl crate::Writable for SCL_STOP_PERIOD {}
#[doc = "RTC_I2C_SCL_STOP_PERIOD_REG"]
pub mod scl_stop_period;
