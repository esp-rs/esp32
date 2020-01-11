#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_GPIO_OUT"]
    pub rtc_gpio_out: RTC_GPIO_OUT,
    #[doc = "0x04 - RTC_GPIO_OUT_W1TS"]
    pub rtc_gpio_out_w1ts: RTC_GPIO_OUT_W1TS,
    #[doc = "0x08 - RTC_GPIO_OUT_W1TC"]
    pub rtc_gpio_out_w1tc: RTC_GPIO_OUT_W1TC,
    #[doc = "0x0c - RTC_GPIO_ENABLE"]
    pub rtc_gpio_enable: RTC_GPIO_ENABLE,
    #[doc = "0x10 - RTC_GPIO_ENABLE_W1TS"]
    pub rtc_gpio_enable_w1ts: RTC_GPIO_ENABLE_W1TS,
    #[doc = "0x14 - RTC_GPIO_ENABLE_W1TC"]
    pub rtc_gpio_enable_w1tc: RTC_GPIO_ENABLE_W1TC,
    #[doc = "0x18 - RTC_GPIO_STATUS"]
    pub rtc_gpio_status: RTC_GPIO_STATUS,
    #[doc = "0x1c - RTC_GPIO_STATUS_W1TS"]
    pub rtc_gpio_status_w1ts: RTC_GPIO_STATUS_W1TS,
    #[doc = "0x20 - RTC_GPIO_STATUS_W1TC"]
    pub rtc_gpio_status_w1tc: RTC_GPIO_STATUS_W1TC,
    #[doc = "0x24 - RTC_GPIO_IN"]
    pub rtc_gpio_in: RTC_GPIO_IN,
    #[doc = "0x28 - RTC_GPIO_PIN0"]
    pub rtc_gpio_pin0: RTC_GPIO_PIN0,
    #[doc = "0x2c - RTC_GPIO_PIN1"]
    pub rtc_gpio_pin1: RTC_GPIO_PIN1,
    #[doc = "0x30 - RTC_GPIO_PIN2"]
    pub rtc_gpio_pin2: RTC_GPIO_PIN2,
    #[doc = "0x34 - RTC_GPIO_PIN3"]
    pub rtc_gpio_pin3: RTC_GPIO_PIN3,
    #[doc = "0x38 - RTC_GPIO_PIN4"]
    pub rtc_gpio_pin4: RTC_GPIO_PIN4,
    #[doc = "0x3c - RTC_GPIO_PIN5"]
    pub rtc_gpio_pin5: RTC_GPIO_PIN5,
    #[doc = "0x40 - RTC_GPIO_PIN6"]
    pub rtc_gpio_pin6: RTC_GPIO_PIN6,
    #[doc = "0x44 - RTC_GPIO_PIN7"]
    pub rtc_gpio_pin7: RTC_GPIO_PIN7,
    #[doc = "0x48 - RTC_GPIO_PIN8"]
    pub rtc_gpio_pin8: RTC_GPIO_PIN8,
    #[doc = "0x4c - RTC_GPIO_PIN9"]
    pub rtc_gpio_pin9: RTC_GPIO_PIN9,
    #[doc = "0x50 - RTC_GPIO_PIN10"]
    pub rtc_gpio_pin10: RTC_GPIO_PIN10,
    #[doc = "0x54 - RTC_GPIO_PIN11"]
    pub rtc_gpio_pin11: RTC_GPIO_PIN11,
    #[doc = "0x58 - RTC_GPIO_PIN12"]
    pub rtc_gpio_pin12: RTC_GPIO_PIN12,
    #[doc = "0x5c - RTC_GPIO_PIN13"]
    pub rtc_gpio_pin13: RTC_GPIO_PIN13,
    #[doc = "0x60 - RTC_GPIO_PIN14"]
    pub rtc_gpio_pin14: RTC_GPIO_PIN14,
    #[doc = "0x64 - RTC_GPIO_PIN15"]
    pub rtc_gpio_pin15: RTC_GPIO_PIN15,
    #[doc = "0x68 - RTC_GPIO_PIN16"]
    pub rtc_gpio_pin16: RTC_GPIO_PIN16,
    #[doc = "0x6c - RTC_GPIO_PIN17"]
    pub rtc_gpio_pin17: RTC_GPIO_PIN17,
    #[doc = "0x70 - RTC_IO_RTC_DEBUG_SEL"]
    pub rtc_io_rtc_debug_sel: RTC_IO_RTC_DEBUG_SEL,
    #[doc = "0x74 - RTC_IO_DIG_PAD_HOLD"]
    pub rtc_io_dig_pad_hold: RTC_IO_DIG_PAD_HOLD,
    #[doc = "0x78 - RTC_IO_HALL_SENS"]
    pub rtc_io_hall_sens: RTC_IO_HALL_SENS,
    #[doc = "0x7c - RTC_IO_SENSOR_PADS"]
    pub rtc_io_sensor_pads: RTC_IO_SENSOR_PADS,
    #[doc = "0x80 - RTC_IO_ADC_PAD"]
    pub rtc_io_adc_pad: RTC_IO_ADC_PAD,
    #[doc = "0x84 - RTC_IO_PAD_DAC1"]
    pub rtc_io_pad_dac1: RTC_IO_PAD_DAC1,
    #[doc = "0x88 - RTC_IO_PAD_DAC2"]
    pub rtc_io_pad_dac2: RTC_IO_PAD_DAC2,
    #[doc = "0x8c - RTC_IO_XTAL_32K_PAD"]
    pub rtc_io_xtal_32k_pad: RTC_IO_XTAL_32K_PAD,
    #[doc = "0x90 - RTC_IO_TOUCH_CFG"]
    pub rtc_io_touch_cfg: RTC_IO_TOUCH_CFG,
    #[doc = "0x94 - RTC_IO_TOUCH_PAD0"]
    pub rtc_io_touch_pad0: RTC_IO_TOUCH_PAD0,
    #[doc = "0x98 - RTC_IO_TOUCH_PAD1"]
    pub rtc_io_touch_pad1: RTC_IO_TOUCH_PAD1,
    #[doc = "0x9c - RTC_IO_TOUCH_PAD2"]
    pub rtc_io_touch_pad2: RTC_IO_TOUCH_PAD2,
    #[doc = "0xa0 - RTC_IO_TOUCH_PAD3"]
    pub rtc_io_touch_pad3: RTC_IO_TOUCH_PAD3,
    #[doc = "0xa4 - RTC_IO_TOUCH_PAD4"]
    pub rtc_io_touch_pad4: RTC_IO_TOUCH_PAD4,
    #[doc = "0xa8 - RTC_IO_TOUCH_PAD5"]
    pub rtc_io_touch_pad5: RTC_IO_TOUCH_PAD5,
    #[doc = "0xac - RTC_IO_TOUCH_PAD6"]
    pub rtc_io_touch_pad6: RTC_IO_TOUCH_PAD6,
    #[doc = "0xb0 - RTC_IO_TOUCH_PAD7"]
    pub rtc_io_touch_pad7: RTC_IO_TOUCH_PAD7,
    #[doc = "0xb4 - RTC_IO_TOUCH_PAD8"]
    pub rtc_io_touch_pad8: RTC_IO_TOUCH_PAD8,
    #[doc = "0xb8 - RTC_IO_TOUCH_PAD9"]
    pub rtc_io_touch_pad9: RTC_IO_TOUCH_PAD9,
    #[doc = "0xbc - RTC_IO_EXT_WAKEUP0"]
    pub rtc_io_ext_wakeup0: RTC_IO_EXT_WAKEUP0,
    #[doc = "0xc0 - RTC_IO_XTL_EXT_CTR"]
    pub rtc_io_xtl_ext_ctr: RTC_IO_XTL_EXT_CTR,
    #[doc = "0xc4 - RTC_IO_SAR_I2C_IO"]
    pub rtc_io_sar_i2c_io: RTC_IO_SAR_I2C_IO,
    #[doc = "0xc8 - RTC_IO_DATE"]
    pub rtc_io_date: RTC_IO_DATE,
}
#[doc = "RTC_GPIO_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_out](rtc_gpio_out) module"]
pub type RTC_GPIO_OUT = crate::Reg<u32, _RTC_GPIO_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_OUT;
#[doc = "`read()` method returns [rtc_gpio_out::R](rtc_gpio_out::R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out::W](rtc_gpio_out::W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT {}
#[doc = "RTC_GPIO_OUT"]
pub mod rtc_gpio_out;
#[doc = "RTC_GPIO_OUT_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_out_w1ts](rtc_gpio_out_w1ts) module"]
pub type RTC_GPIO_OUT_W1TS = crate::Reg<u32, _RTC_GPIO_OUT_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_OUT_W1TS;
#[doc = "`read()` method returns [rtc_gpio_out_w1ts::R](rtc_gpio_out_w1ts::R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_W1TS {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out_w1ts::W](rtc_gpio_out_w1ts::W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_W1TS {}
#[doc = "RTC_GPIO_OUT_W1TS"]
pub mod rtc_gpio_out_w1ts;
#[doc = "RTC_GPIO_OUT_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_out_w1tc](rtc_gpio_out_w1tc) module"]
pub type RTC_GPIO_OUT_W1TC = crate::Reg<u32, _RTC_GPIO_OUT_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_OUT_W1TC;
#[doc = "`read()` method returns [rtc_gpio_out_w1tc::R](rtc_gpio_out_w1tc::R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_W1TC {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out_w1tc::W](rtc_gpio_out_w1tc::W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_W1TC {}
#[doc = "RTC_GPIO_OUT_W1TC"]
pub mod rtc_gpio_out_w1tc;
#[doc = "RTC_GPIO_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_enable](rtc_gpio_enable) module"]
pub type RTC_GPIO_ENABLE = crate::Reg<u32, _RTC_GPIO_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_ENABLE;
#[doc = "`read()` method returns [rtc_gpio_enable::R](rtc_gpio_enable::R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable::W](rtc_gpio_enable::W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE {}
#[doc = "RTC_GPIO_ENABLE"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_ENABLE_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_enable_w1ts](rtc_gpio_enable_w1ts) module"]
pub type RTC_GPIO_ENABLE_W1TS = crate::Reg<u32, _RTC_GPIO_ENABLE_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_ENABLE_W1TS;
#[doc = "`read()` method returns [rtc_gpio_enable_w1ts::R](rtc_gpio_enable_w1ts::R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_W1TS {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable_w1ts::W](rtc_gpio_enable_w1ts::W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_W1TS {}
#[doc = "RTC_GPIO_ENABLE_W1TS"]
pub mod rtc_gpio_enable_w1ts;
#[doc = "RTC_GPIO_ENABLE_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_enable_w1tc](rtc_gpio_enable_w1tc) module"]
pub type RTC_GPIO_ENABLE_W1TC = crate::Reg<u32, _RTC_GPIO_ENABLE_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_ENABLE_W1TC;
#[doc = "`read()` method returns [rtc_gpio_enable_w1tc::R](rtc_gpio_enable_w1tc::R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_W1TC {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable_w1tc::W](rtc_gpio_enable_w1tc::W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_W1TC {}
#[doc = "RTC_GPIO_ENABLE_W1TC"]
pub mod rtc_gpio_enable_w1tc;
#[doc = "RTC_GPIO_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_status](rtc_gpio_status) module"]
pub type RTC_GPIO_STATUS = crate::Reg<u32, _RTC_GPIO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_STATUS;
#[doc = "`read()` method returns [rtc_gpio_status::R](rtc_gpio_status::R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status::W](rtc_gpio_status::W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS {}
#[doc = "RTC_GPIO_STATUS"]
pub mod rtc_gpio_status;
#[doc = "RTC_GPIO_STATUS_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_status_w1ts](rtc_gpio_status_w1ts) module"]
pub type RTC_GPIO_STATUS_W1TS = crate::Reg<u32, _RTC_GPIO_STATUS_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_STATUS_W1TS;
#[doc = "`read()` method returns [rtc_gpio_status_w1ts::R](rtc_gpio_status_w1ts::R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_W1TS {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_w1ts::W](rtc_gpio_status_w1ts::W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TS {}
#[doc = "RTC_GPIO_STATUS_W1TS"]
pub mod rtc_gpio_status_w1ts;
#[doc = "RTC_GPIO_STATUS_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_status_w1tc](rtc_gpio_status_w1tc) module"]
pub type RTC_GPIO_STATUS_W1TC = crate::Reg<u32, _RTC_GPIO_STATUS_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_STATUS_W1TC;
#[doc = "`read()` method returns [rtc_gpio_status_w1tc::R](rtc_gpio_status_w1tc::R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_W1TC {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_w1tc::W](rtc_gpio_status_w1tc::W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TC {}
#[doc = "RTC_GPIO_STATUS_W1TC"]
pub mod rtc_gpio_status_w1tc;
#[doc = "RTC_GPIO_IN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_in](rtc_gpio_in) module"]
pub type RTC_GPIO_IN = crate::Reg<u32, _RTC_GPIO_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_IN;
#[doc = "`read()` method returns [rtc_gpio_in::R](rtc_gpio_in::R) reader structure"]
impl crate::Readable for RTC_GPIO_IN {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_in::W](rtc_gpio_in::W) writer structure"]
impl crate::Writable for RTC_GPIO_IN {}
#[doc = "RTC_GPIO_IN"]
pub mod rtc_gpio_in;
#[doc = "RTC_GPIO_PIN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin0](rtc_gpio_pin0) module"]
pub type RTC_GPIO_PIN0 = crate::Reg<u32, _RTC_GPIO_PIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN0;
#[doc = "`read()` method returns [rtc_gpio_pin0::R](rtc_gpio_pin0::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN0 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin0::W](rtc_gpio_pin0::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN0 {}
#[doc = "RTC_GPIO_PIN0"]
pub mod rtc_gpio_pin0;
#[doc = "RTC_GPIO_PIN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin1](rtc_gpio_pin1) module"]
pub type RTC_GPIO_PIN1 = crate::Reg<u32, _RTC_GPIO_PIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN1;
#[doc = "`read()` method returns [rtc_gpio_pin1::R](rtc_gpio_pin1::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN1 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin1::W](rtc_gpio_pin1::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN1 {}
#[doc = "RTC_GPIO_PIN1"]
pub mod rtc_gpio_pin1;
#[doc = "RTC_GPIO_PIN2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin2](rtc_gpio_pin2) module"]
pub type RTC_GPIO_PIN2 = crate::Reg<u32, _RTC_GPIO_PIN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN2;
#[doc = "`read()` method returns [rtc_gpio_pin2::R](rtc_gpio_pin2::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN2 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin2::W](rtc_gpio_pin2::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN2 {}
#[doc = "RTC_GPIO_PIN2"]
pub mod rtc_gpio_pin2;
#[doc = "RTC_GPIO_PIN3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin3](rtc_gpio_pin3) module"]
pub type RTC_GPIO_PIN3 = crate::Reg<u32, _RTC_GPIO_PIN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN3;
#[doc = "`read()` method returns [rtc_gpio_pin3::R](rtc_gpio_pin3::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN3 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin3::W](rtc_gpio_pin3::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN3 {}
#[doc = "RTC_GPIO_PIN3"]
pub mod rtc_gpio_pin3;
#[doc = "RTC_GPIO_PIN4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin4](rtc_gpio_pin4) module"]
pub type RTC_GPIO_PIN4 = crate::Reg<u32, _RTC_GPIO_PIN4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN4;
#[doc = "`read()` method returns [rtc_gpio_pin4::R](rtc_gpio_pin4::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN4 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin4::W](rtc_gpio_pin4::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN4 {}
#[doc = "RTC_GPIO_PIN4"]
pub mod rtc_gpio_pin4;
#[doc = "RTC_GPIO_PIN5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin5](rtc_gpio_pin5) module"]
pub type RTC_GPIO_PIN5 = crate::Reg<u32, _RTC_GPIO_PIN5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN5;
#[doc = "`read()` method returns [rtc_gpio_pin5::R](rtc_gpio_pin5::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN5 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin5::W](rtc_gpio_pin5::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN5 {}
#[doc = "RTC_GPIO_PIN5"]
pub mod rtc_gpio_pin5;
#[doc = "RTC_GPIO_PIN6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin6](rtc_gpio_pin6) module"]
pub type RTC_GPIO_PIN6 = crate::Reg<u32, _RTC_GPIO_PIN6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN6;
#[doc = "`read()` method returns [rtc_gpio_pin6::R](rtc_gpio_pin6::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN6 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin6::W](rtc_gpio_pin6::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN6 {}
#[doc = "RTC_GPIO_PIN6"]
pub mod rtc_gpio_pin6;
#[doc = "RTC_GPIO_PIN7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin7](rtc_gpio_pin7) module"]
pub type RTC_GPIO_PIN7 = crate::Reg<u32, _RTC_GPIO_PIN7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN7;
#[doc = "`read()` method returns [rtc_gpio_pin7::R](rtc_gpio_pin7::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN7 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin7::W](rtc_gpio_pin7::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN7 {}
#[doc = "RTC_GPIO_PIN7"]
pub mod rtc_gpio_pin7;
#[doc = "RTC_GPIO_PIN8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin8](rtc_gpio_pin8) module"]
pub type RTC_GPIO_PIN8 = crate::Reg<u32, _RTC_GPIO_PIN8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN8;
#[doc = "`read()` method returns [rtc_gpio_pin8::R](rtc_gpio_pin8::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN8 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin8::W](rtc_gpio_pin8::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN8 {}
#[doc = "RTC_GPIO_PIN8"]
pub mod rtc_gpio_pin8;
#[doc = "RTC_GPIO_PIN9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin9](rtc_gpio_pin9) module"]
pub type RTC_GPIO_PIN9 = crate::Reg<u32, _RTC_GPIO_PIN9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN9;
#[doc = "`read()` method returns [rtc_gpio_pin9::R](rtc_gpio_pin9::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN9 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin9::W](rtc_gpio_pin9::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN9 {}
#[doc = "RTC_GPIO_PIN9"]
pub mod rtc_gpio_pin9;
#[doc = "RTC_GPIO_PIN10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin10](rtc_gpio_pin10) module"]
pub type RTC_GPIO_PIN10 = crate::Reg<u32, _RTC_GPIO_PIN10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN10;
#[doc = "`read()` method returns [rtc_gpio_pin10::R](rtc_gpio_pin10::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN10 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin10::W](rtc_gpio_pin10::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN10 {}
#[doc = "RTC_GPIO_PIN10"]
pub mod rtc_gpio_pin10;
#[doc = "RTC_GPIO_PIN11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin11](rtc_gpio_pin11) module"]
pub type RTC_GPIO_PIN11 = crate::Reg<u32, _RTC_GPIO_PIN11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN11;
#[doc = "`read()` method returns [rtc_gpio_pin11::R](rtc_gpio_pin11::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN11 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin11::W](rtc_gpio_pin11::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN11 {}
#[doc = "RTC_GPIO_PIN11"]
pub mod rtc_gpio_pin11;
#[doc = "RTC_GPIO_PIN12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin12](rtc_gpio_pin12) module"]
pub type RTC_GPIO_PIN12 = crate::Reg<u32, _RTC_GPIO_PIN12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN12;
#[doc = "`read()` method returns [rtc_gpio_pin12::R](rtc_gpio_pin12::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN12 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin12::W](rtc_gpio_pin12::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN12 {}
#[doc = "RTC_GPIO_PIN12"]
pub mod rtc_gpio_pin12;
#[doc = "RTC_GPIO_PIN13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin13](rtc_gpio_pin13) module"]
pub type RTC_GPIO_PIN13 = crate::Reg<u32, _RTC_GPIO_PIN13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN13;
#[doc = "`read()` method returns [rtc_gpio_pin13::R](rtc_gpio_pin13::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN13 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin13::W](rtc_gpio_pin13::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN13 {}
#[doc = "RTC_GPIO_PIN13"]
pub mod rtc_gpio_pin13;
#[doc = "RTC_GPIO_PIN14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin14](rtc_gpio_pin14) module"]
pub type RTC_GPIO_PIN14 = crate::Reg<u32, _RTC_GPIO_PIN14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN14;
#[doc = "`read()` method returns [rtc_gpio_pin14::R](rtc_gpio_pin14::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN14 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin14::W](rtc_gpio_pin14::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN14 {}
#[doc = "RTC_GPIO_PIN14"]
pub mod rtc_gpio_pin14;
#[doc = "RTC_GPIO_PIN15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin15](rtc_gpio_pin15) module"]
pub type RTC_GPIO_PIN15 = crate::Reg<u32, _RTC_GPIO_PIN15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN15;
#[doc = "`read()` method returns [rtc_gpio_pin15::R](rtc_gpio_pin15::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN15 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin15::W](rtc_gpio_pin15::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN15 {}
#[doc = "RTC_GPIO_PIN15"]
pub mod rtc_gpio_pin15;
#[doc = "RTC_GPIO_PIN16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin16](rtc_gpio_pin16) module"]
pub type RTC_GPIO_PIN16 = crate::Reg<u32, _RTC_GPIO_PIN16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN16;
#[doc = "`read()` method returns [rtc_gpio_pin16::R](rtc_gpio_pin16::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN16 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin16::W](rtc_gpio_pin16::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN16 {}
#[doc = "RTC_GPIO_PIN16"]
pub mod rtc_gpio_pin16;
#[doc = "RTC_GPIO_PIN17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin17](rtc_gpio_pin17) module"]
pub type RTC_GPIO_PIN17 = crate::Reg<u32, _RTC_GPIO_PIN17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN17;
#[doc = "`read()` method returns [rtc_gpio_pin17::R](rtc_gpio_pin17::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN17 {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin17::W](rtc_gpio_pin17::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN17 {}
#[doc = "RTC_GPIO_PIN17"]
pub mod rtc_gpio_pin17;
#[doc = "RTC_IO_RTC_DEBUG_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_rtc_debug_sel](rtc_io_rtc_debug_sel) module"]
pub type RTC_IO_RTC_DEBUG_SEL = crate::Reg<u32, _RTC_IO_RTC_DEBUG_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_RTC_DEBUG_SEL;
#[doc = "`read()` method returns [rtc_io_rtc_debug_sel::R](rtc_io_rtc_debug_sel::R) reader structure"]
impl crate::Readable for RTC_IO_RTC_DEBUG_SEL {}
#[doc = "`write(|w| ..)` method takes [rtc_io_rtc_debug_sel::W](rtc_io_rtc_debug_sel::W) writer structure"]
impl crate::Writable for RTC_IO_RTC_DEBUG_SEL {}
#[doc = "RTC_IO_RTC_DEBUG_SEL"]
pub mod rtc_io_rtc_debug_sel;
#[doc = "RTC_IO_DIG_PAD_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_dig_pad_hold](rtc_io_dig_pad_hold) module"]
pub type RTC_IO_DIG_PAD_HOLD = crate::Reg<u32, _RTC_IO_DIG_PAD_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_DIG_PAD_HOLD;
#[doc = "`read()` method returns [rtc_io_dig_pad_hold::R](rtc_io_dig_pad_hold::R) reader structure"]
impl crate::Readable for RTC_IO_DIG_PAD_HOLD {}
#[doc = "`write(|w| ..)` method takes [rtc_io_dig_pad_hold::W](rtc_io_dig_pad_hold::W) writer structure"]
impl crate::Writable for RTC_IO_DIG_PAD_HOLD {}
#[doc = "RTC_IO_DIG_PAD_HOLD"]
pub mod rtc_io_dig_pad_hold;
#[doc = "RTC_IO_HALL_SENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_hall_sens](rtc_io_hall_sens) module"]
pub type RTC_IO_HALL_SENS = crate::Reg<u32, _RTC_IO_HALL_SENS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_HALL_SENS;
#[doc = "`read()` method returns [rtc_io_hall_sens::R](rtc_io_hall_sens::R) reader structure"]
impl crate::Readable for RTC_IO_HALL_SENS {}
#[doc = "`write(|w| ..)` method takes [rtc_io_hall_sens::W](rtc_io_hall_sens::W) writer structure"]
impl crate::Writable for RTC_IO_HALL_SENS {}
#[doc = "RTC_IO_HALL_SENS"]
pub mod rtc_io_hall_sens;
#[doc = "RTC_IO_SENSOR_PADS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_sensor_pads](rtc_io_sensor_pads) module"]
pub type RTC_IO_SENSOR_PADS = crate::Reg<u32, _RTC_IO_SENSOR_PADS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_SENSOR_PADS;
#[doc = "`read()` method returns [rtc_io_sensor_pads::R](rtc_io_sensor_pads::R) reader structure"]
impl crate::Readable for RTC_IO_SENSOR_PADS {}
#[doc = "`write(|w| ..)` method takes [rtc_io_sensor_pads::W](rtc_io_sensor_pads::W) writer structure"]
impl crate::Writable for RTC_IO_SENSOR_PADS {}
#[doc = "RTC_IO_SENSOR_PADS"]
pub mod rtc_io_sensor_pads;
#[doc = "RTC_IO_ADC_PAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_adc_pad](rtc_io_adc_pad) module"]
pub type RTC_IO_ADC_PAD = crate::Reg<u32, _RTC_IO_ADC_PAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_ADC_PAD;
#[doc = "`read()` method returns [rtc_io_adc_pad::R](rtc_io_adc_pad::R) reader structure"]
impl crate::Readable for RTC_IO_ADC_PAD {}
#[doc = "`write(|w| ..)` method takes [rtc_io_adc_pad::W](rtc_io_adc_pad::W) writer structure"]
impl crate::Writable for RTC_IO_ADC_PAD {}
#[doc = "RTC_IO_ADC_PAD"]
pub mod rtc_io_adc_pad;
#[doc = "RTC_IO_PAD_DAC1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_pad_dac1](rtc_io_pad_dac1) module"]
pub type RTC_IO_PAD_DAC1 = crate::Reg<u32, _RTC_IO_PAD_DAC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_PAD_DAC1;
#[doc = "`read()` method returns [rtc_io_pad_dac1::R](rtc_io_pad_dac1::R) reader structure"]
impl crate::Readable for RTC_IO_PAD_DAC1 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_pad_dac1::W](rtc_io_pad_dac1::W) writer structure"]
impl crate::Writable for RTC_IO_PAD_DAC1 {}
#[doc = "RTC_IO_PAD_DAC1"]
pub mod rtc_io_pad_dac1;
#[doc = "RTC_IO_PAD_DAC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_pad_dac2](rtc_io_pad_dac2) module"]
pub type RTC_IO_PAD_DAC2 = crate::Reg<u32, _RTC_IO_PAD_DAC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_PAD_DAC2;
#[doc = "`read()` method returns [rtc_io_pad_dac2::R](rtc_io_pad_dac2::R) reader structure"]
impl crate::Readable for RTC_IO_PAD_DAC2 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_pad_dac2::W](rtc_io_pad_dac2::W) writer structure"]
impl crate::Writable for RTC_IO_PAD_DAC2 {}
#[doc = "RTC_IO_PAD_DAC2"]
pub mod rtc_io_pad_dac2;
#[doc = "RTC_IO_XTAL_32K_PAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_xtal_32k_pad](rtc_io_xtal_32k_pad) module"]
pub type RTC_IO_XTAL_32K_PAD = crate::Reg<u32, _RTC_IO_XTAL_32K_PAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_XTAL_32K_PAD;
#[doc = "`read()` method returns [rtc_io_xtal_32k_pad::R](rtc_io_xtal_32k_pad::R) reader structure"]
impl crate::Readable for RTC_IO_XTAL_32K_PAD {}
#[doc = "`write(|w| ..)` method takes [rtc_io_xtal_32k_pad::W](rtc_io_xtal_32k_pad::W) writer structure"]
impl crate::Writable for RTC_IO_XTAL_32K_PAD {}
#[doc = "RTC_IO_XTAL_32K_PAD"]
pub mod rtc_io_xtal_32k_pad;
#[doc = "RTC_IO_TOUCH_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_cfg](rtc_io_touch_cfg) module"]
pub type RTC_IO_TOUCH_CFG = crate::Reg<u32, _RTC_IO_TOUCH_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_CFG;
#[doc = "`read()` method returns [rtc_io_touch_cfg::R](rtc_io_touch_cfg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_CFG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_cfg::W](rtc_io_touch_cfg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_CFG {}
#[doc = "RTC_IO_TOUCH_CFG"]
pub mod rtc_io_touch_cfg;
#[doc = "RTC_IO_TOUCH_PAD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad0](rtc_io_touch_pad0) module"]
pub type RTC_IO_TOUCH_PAD0 = crate::Reg<u32, _RTC_IO_TOUCH_PAD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD0;
#[doc = "`read()` method returns [rtc_io_touch_pad0::R](rtc_io_touch_pad0::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD0 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad0::W](rtc_io_touch_pad0::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD0 {}
#[doc = "RTC_IO_TOUCH_PAD0"]
pub mod rtc_io_touch_pad0;
#[doc = "RTC_IO_TOUCH_PAD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad1](rtc_io_touch_pad1) module"]
pub type RTC_IO_TOUCH_PAD1 = crate::Reg<u32, _RTC_IO_TOUCH_PAD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD1;
#[doc = "`read()` method returns [rtc_io_touch_pad1::R](rtc_io_touch_pad1::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD1 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad1::W](rtc_io_touch_pad1::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD1 {}
#[doc = "RTC_IO_TOUCH_PAD1"]
pub mod rtc_io_touch_pad1;
#[doc = "RTC_IO_TOUCH_PAD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad2](rtc_io_touch_pad2) module"]
pub type RTC_IO_TOUCH_PAD2 = crate::Reg<u32, _RTC_IO_TOUCH_PAD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD2;
#[doc = "`read()` method returns [rtc_io_touch_pad2::R](rtc_io_touch_pad2::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD2 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad2::W](rtc_io_touch_pad2::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD2 {}
#[doc = "RTC_IO_TOUCH_PAD2"]
pub mod rtc_io_touch_pad2;
#[doc = "RTC_IO_TOUCH_PAD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad3](rtc_io_touch_pad3) module"]
pub type RTC_IO_TOUCH_PAD3 = crate::Reg<u32, _RTC_IO_TOUCH_PAD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD3;
#[doc = "`read()` method returns [rtc_io_touch_pad3::R](rtc_io_touch_pad3::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD3 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad3::W](rtc_io_touch_pad3::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD3 {}
#[doc = "RTC_IO_TOUCH_PAD3"]
pub mod rtc_io_touch_pad3;
#[doc = "RTC_IO_TOUCH_PAD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad4](rtc_io_touch_pad4) module"]
pub type RTC_IO_TOUCH_PAD4 = crate::Reg<u32, _RTC_IO_TOUCH_PAD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD4;
#[doc = "`read()` method returns [rtc_io_touch_pad4::R](rtc_io_touch_pad4::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD4 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad4::W](rtc_io_touch_pad4::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD4 {}
#[doc = "RTC_IO_TOUCH_PAD4"]
pub mod rtc_io_touch_pad4;
#[doc = "RTC_IO_TOUCH_PAD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad5](rtc_io_touch_pad5) module"]
pub type RTC_IO_TOUCH_PAD5 = crate::Reg<u32, _RTC_IO_TOUCH_PAD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD5;
#[doc = "`read()` method returns [rtc_io_touch_pad5::R](rtc_io_touch_pad5::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD5 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad5::W](rtc_io_touch_pad5::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD5 {}
#[doc = "RTC_IO_TOUCH_PAD5"]
pub mod rtc_io_touch_pad5;
#[doc = "RTC_IO_TOUCH_PAD6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad6](rtc_io_touch_pad6) module"]
pub type RTC_IO_TOUCH_PAD6 = crate::Reg<u32, _RTC_IO_TOUCH_PAD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD6;
#[doc = "`read()` method returns [rtc_io_touch_pad6::R](rtc_io_touch_pad6::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD6 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad6::W](rtc_io_touch_pad6::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD6 {}
#[doc = "RTC_IO_TOUCH_PAD6"]
pub mod rtc_io_touch_pad6;
#[doc = "RTC_IO_TOUCH_PAD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad7](rtc_io_touch_pad7) module"]
pub type RTC_IO_TOUCH_PAD7 = crate::Reg<u32, _RTC_IO_TOUCH_PAD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD7;
#[doc = "`read()` method returns [rtc_io_touch_pad7::R](rtc_io_touch_pad7::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD7 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad7::W](rtc_io_touch_pad7::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD7 {}
#[doc = "RTC_IO_TOUCH_PAD7"]
pub mod rtc_io_touch_pad7;
#[doc = "RTC_IO_TOUCH_PAD8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad8](rtc_io_touch_pad8) module"]
pub type RTC_IO_TOUCH_PAD8 = crate::Reg<u32, _RTC_IO_TOUCH_PAD8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD8;
#[doc = "`read()` method returns [rtc_io_touch_pad8::R](rtc_io_touch_pad8::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD8 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad8::W](rtc_io_touch_pad8::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD8 {}
#[doc = "RTC_IO_TOUCH_PAD8"]
pub mod rtc_io_touch_pad8;
#[doc = "RTC_IO_TOUCH_PAD9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad9](rtc_io_touch_pad9) module"]
pub type RTC_IO_TOUCH_PAD9 = crate::Reg<u32, _RTC_IO_TOUCH_PAD9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD9;
#[doc = "`read()` method returns [rtc_io_touch_pad9::R](rtc_io_touch_pad9::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD9 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad9::W](rtc_io_touch_pad9::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD9 {}
#[doc = "RTC_IO_TOUCH_PAD9"]
pub mod rtc_io_touch_pad9;
#[doc = "RTC_IO_EXT_WAKEUP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_ext_wakeup0](rtc_io_ext_wakeup0) module"]
pub type RTC_IO_EXT_WAKEUP0 = crate::Reg<u32, _RTC_IO_EXT_WAKEUP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_EXT_WAKEUP0;
#[doc = "`read()` method returns [rtc_io_ext_wakeup0::R](rtc_io_ext_wakeup0::R) reader structure"]
impl crate::Readable for RTC_IO_EXT_WAKEUP0 {}
#[doc = "`write(|w| ..)` method takes [rtc_io_ext_wakeup0::W](rtc_io_ext_wakeup0::W) writer structure"]
impl crate::Writable for RTC_IO_EXT_WAKEUP0 {}
#[doc = "RTC_IO_EXT_WAKEUP0"]
pub mod rtc_io_ext_wakeup0;
#[doc = "RTC_IO_XTL_EXT_CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_xtl_ext_ctr](rtc_io_xtl_ext_ctr) module"]
pub type RTC_IO_XTL_EXT_CTR = crate::Reg<u32, _RTC_IO_XTL_EXT_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_XTL_EXT_CTR;
#[doc = "`read()` method returns [rtc_io_xtl_ext_ctr::R](rtc_io_xtl_ext_ctr::R) reader structure"]
impl crate::Readable for RTC_IO_XTL_EXT_CTR {}
#[doc = "`write(|w| ..)` method takes [rtc_io_xtl_ext_ctr::W](rtc_io_xtl_ext_ctr::W) writer structure"]
impl crate::Writable for RTC_IO_XTL_EXT_CTR {}
#[doc = "RTC_IO_XTL_EXT_CTR"]
pub mod rtc_io_xtl_ext_ctr;
#[doc = "RTC_IO_SAR_I2C_IO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_sar_i2c_io](rtc_io_sar_i2c_io) module"]
pub type RTC_IO_SAR_I2C_IO = crate::Reg<u32, _RTC_IO_SAR_I2C_IO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_SAR_I2C_IO;
#[doc = "`read()` method returns [rtc_io_sar_i2c_io::R](rtc_io_sar_i2c_io::R) reader structure"]
impl crate::Readable for RTC_IO_SAR_I2C_IO {}
#[doc = "`write(|w| ..)` method takes [rtc_io_sar_i2c_io::W](rtc_io_sar_i2c_io::W) writer structure"]
impl crate::Writable for RTC_IO_SAR_I2C_IO {}
#[doc = "RTC_IO_SAR_I2C_IO"]
pub mod rtc_io_sar_i2c_io;
#[doc = "RTC_IO_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_date](rtc_io_date) module"]
pub type RTC_IO_DATE = crate::Reg<u32, _RTC_IO_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_DATE;
#[doc = "`read()` method returns [rtc_io_date::R](rtc_io_date::R) reader structure"]
impl crate::Readable for RTC_IO_DATE {}
#[doc = "`write(|w| ..)` method takes [rtc_io_date::W](rtc_io_date::W) writer structure"]
impl crate::Writable for RTC_IO_DATE {}
#[doc = "RTC_IO_DATE"]
pub mod rtc_io_date;
