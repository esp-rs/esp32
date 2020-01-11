#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - RMT_CH0CONF0_REG"]
    pub ch0conf0: CH0CONF0,
    #[doc = "0x24 - RMT_CH0CONF1_REG"]
    pub ch0conf1: CH0CONF1,
    #[doc = "0x28 - RMT_CH1CONF0_REG"]
    pub ch1conf0: CH1CONF0,
    #[doc = "0x2c - RMT_CH1CONF1_REG"]
    pub ch1conf1: CH1CONF1,
    #[doc = "0x30 - RMT_CH2CONF0_REG"]
    pub ch2conf0: CH2CONF0,
    #[doc = "0x34 - RMT_CH2CONF1_REG"]
    pub ch2conf1: CH2CONF1,
    #[doc = "0x38 - RMT_CH3CONF0_REG"]
    pub ch3conf0: CH3CONF0,
    #[doc = "0x3c - RMT_CH3CONF1_REG"]
    pub ch3conf1: CH3CONF1,
    #[doc = "0x40 - RMT_CH4CONF0_REG"]
    pub ch4conf0: CH4CONF0,
    #[doc = "0x44 - RMT_CH4CONF1_REG"]
    pub ch4conf1: CH4CONF1,
    #[doc = "0x48 - RMT_CH5CONF0_REG"]
    pub ch5conf0: CH5CONF0,
    #[doc = "0x4c - RMT_CH5CONF1_REG"]
    pub ch5conf1: CH5CONF1,
    #[doc = "0x50 - RMT_CH6CONF0_REG"]
    pub ch6conf0: CH6CONF0,
    #[doc = "0x54 - RMT_CH6CONF1_REG"]
    pub ch6conf1: CH6CONF1,
    #[doc = "0x58 - RMT_CH7CONF0_REG"]
    pub ch7conf0: CH7CONF0,
    #[doc = "0x5c - RMT_CH7CONF1_REG"]
    pub ch7conf1: CH7CONF1,
    #[doc = "0x60 - RMT_CH0STATUS_REG"]
    pub ch0status: CH0STATUS,
    #[doc = "0x64 - RMT_CH1STATUS_REG"]
    pub ch1status: CH1STATUS,
    #[doc = "0x68 - RMT_CH2STATUS_REG"]
    pub ch2status: CH2STATUS,
    #[doc = "0x6c - RMT_CH3STATUS_REG"]
    pub ch3status: CH3STATUS,
    #[doc = "0x70 - RMT_CH4STATUS_REG"]
    pub ch4status: CH4STATUS,
    #[doc = "0x74 - RMT_CH5STATUS_REG"]
    pub ch5status: CH5STATUS,
    #[doc = "0x78 - RMT_CH6STATUS_REG"]
    pub ch6status: CH6STATUS,
    #[doc = "0x7c - RMT_CH7STATUS_REG"]
    pub ch7status: CH7STATUS,
    #[doc = "0x80 - RMT_CH0ADDR_REG"]
    pub ch0addr: CH0ADDR,
    #[doc = "0x84 - RMT_CH1ADDR_REG"]
    pub ch1addr: CH1ADDR,
    #[doc = "0x88 - RMT_CH2ADDR_REG"]
    pub ch2addr: CH2ADDR,
    #[doc = "0x8c - RMT_CH3ADDR_REG"]
    pub ch3addr: CH3ADDR,
    #[doc = "0x90 - RMT_CH4ADDR_REG"]
    pub ch4addr: CH4ADDR,
    #[doc = "0x94 - RMT_CH5ADDR_REG"]
    pub ch5addr: CH5ADDR,
    #[doc = "0x98 - RMT_CH6ADDR_REG"]
    pub ch6addr: CH6ADDR,
    #[doc = "0x9c - RMT_CH7ADDR_REG"]
    pub ch7addr: CH7ADDR,
    #[doc = "0xa0 - RMT_INT_RAW_REG"]
    pub int_raw: INT_RAW,
    #[doc = "0xa4 - RMT_INT_ST_REG"]
    pub int_st: INT_ST,
    #[doc = "0xa8 - RMT_INT_ENA_REG"]
    pub int_ena: INT_ENA,
    #[doc = "0xac - RMT_INT_CLR_REG"]
    pub int_clr: INT_CLR,
    #[doc = "0xb0 - RMT_CH0CARRIER_DUTY_REG"]
    pub ch0carrier_duty: CH0CARRIER_DUTY,
    #[doc = "0xb4 - RMT_CH1CARRIER_DUTY_REG"]
    pub ch1carrier_duty: CH1CARRIER_DUTY,
    #[doc = "0xb8 - RMT_CH2CARRIER_DUTY_REG"]
    pub ch2carrier_duty: CH2CARRIER_DUTY,
    #[doc = "0xbc - RMT_CH3CARRIER_DUTY_REG"]
    pub ch3carrier_duty: CH3CARRIER_DUTY,
    #[doc = "0xc0 - RMT_CH4CARRIER_DUTY_REG"]
    pub ch4carrier_duty: CH4CARRIER_DUTY,
    #[doc = "0xc4 - RMT_CH5CARRIER_DUTY_REG"]
    pub ch5carrier_duty: CH5CARRIER_DUTY,
    #[doc = "0xc8 - RMT_CH6CARRIER_DUTY_REG"]
    pub ch6carrier_duty: CH6CARRIER_DUTY,
    #[doc = "0xcc - RMT_CH7CARRIER_DUTY_REG"]
    pub ch7carrier_duty: CH7CARRIER_DUTY,
    #[doc = "0xd0 - RMT_CH0_TX_LIM_REG"]
    pub ch0_tx_lim: CH0_TX_LIM,
    #[doc = "0xd4 - RMT_CH1_TX_LIM_REG"]
    pub ch1_tx_lim: CH1_TX_LIM,
    #[doc = "0xd8 - RMT_CH2_TX_LIM_REG"]
    pub ch2_tx_lim: CH2_TX_LIM,
    #[doc = "0xdc - RMT_CH3_TX_LIM_REG"]
    pub ch3_tx_lim: CH3_TX_LIM,
    #[doc = "0xe0 - RMT_CH4_TX_LIM_REG"]
    pub ch4_tx_lim: CH4_TX_LIM,
    #[doc = "0xe4 - RMT_CH5_TX_LIM_REG"]
    pub ch5_tx_lim: CH5_TX_LIM,
    #[doc = "0xe8 - RMT_CH6_TX_LIM_REG"]
    pub ch6_tx_lim: CH6_TX_LIM,
    #[doc = "0xec - RMT_CH7_TX_LIM_REG"]
    pub ch7_tx_lim: CH7_TX_LIM,
    #[doc = "0xf0 - RMT_APB_CONF_REG"]
    pub apb_conf: APB_CONF,
    _reserved53: [u8; 8usize],
    #[doc = "0xfc - RMT_DATE_REG"]
    pub date: DATE,
}
#[doc = "RMT_CH0CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0conf0](ch0conf0) module"]
pub type CH0CONF0 = crate::Reg<u32, _CH0CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CONF0;
#[doc = "`read()` method returns [ch0conf0::R](ch0conf0::R) reader structure"]
impl crate::Readable for CH0CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch0conf0::W](ch0conf0::W) writer structure"]
impl crate::Writable for CH0CONF0 {}
#[doc = "RMT_CH0CONF0_REG"]
pub mod ch0conf0;
#[doc = "RMT_CH0CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0conf1](ch0conf1) module"]
pub type CH0CONF1 = crate::Reg<u32, _CH0CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CONF1;
#[doc = "`read()` method returns [ch0conf1::R](ch0conf1::R) reader structure"]
impl crate::Readable for CH0CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch0conf1::W](ch0conf1::W) writer structure"]
impl crate::Writable for CH0CONF1 {}
#[doc = "RMT_CH0CONF1_REG"]
pub mod ch0conf1;
#[doc = "RMT_CH1CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1conf0](ch1conf0) module"]
pub type CH1CONF0 = crate::Reg<u32, _CH1CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CONF0;
#[doc = "`read()` method returns [ch1conf0::R](ch1conf0::R) reader structure"]
impl crate::Readable for CH1CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch1conf0::W](ch1conf0::W) writer structure"]
impl crate::Writable for CH1CONF0 {}
#[doc = "RMT_CH1CONF0_REG"]
pub mod ch1conf0;
#[doc = "RMT_CH1CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1conf1](ch1conf1) module"]
pub type CH1CONF1 = crate::Reg<u32, _CH1CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CONF1;
#[doc = "`read()` method returns [ch1conf1::R](ch1conf1::R) reader structure"]
impl crate::Readable for CH1CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch1conf1::W](ch1conf1::W) writer structure"]
impl crate::Writable for CH1CONF1 {}
#[doc = "RMT_CH1CONF1_REG"]
pub mod ch1conf1;
#[doc = "RMT_CH2CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2conf0](ch2conf0) module"]
pub type CH2CONF0 = crate::Reg<u32, _CH2CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CONF0;
#[doc = "`read()` method returns [ch2conf0::R](ch2conf0::R) reader structure"]
impl crate::Readable for CH2CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch2conf0::W](ch2conf0::W) writer structure"]
impl crate::Writable for CH2CONF0 {}
#[doc = "RMT_CH2CONF0_REG"]
pub mod ch2conf0;
#[doc = "RMT_CH2CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2conf1](ch2conf1) module"]
pub type CH2CONF1 = crate::Reg<u32, _CH2CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CONF1;
#[doc = "`read()` method returns [ch2conf1::R](ch2conf1::R) reader structure"]
impl crate::Readable for CH2CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch2conf1::W](ch2conf1::W) writer structure"]
impl crate::Writable for CH2CONF1 {}
#[doc = "RMT_CH2CONF1_REG"]
pub mod ch2conf1;
#[doc = "RMT_CH3CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3conf0](ch3conf0) module"]
pub type CH3CONF0 = crate::Reg<u32, _CH3CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CONF0;
#[doc = "`read()` method returns [ch3conf0::R](ch3conf0::R) reader structure"]
impl crate::Readable for CH3CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch3conf0::W](ch3conf0::W) writer structure"]
impl crate::Writable for CH3CONF0 {}
#[doc = "RMT_CH3CONF0_REG"]
pub mod ch3conf0;
#[doc = "RMT_CH3CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3conf1](ch3conf1) module"]
pub type CH3CONF1 = crate::Reg<u32, _CH3CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CONF1;
#[doc = "`read()` method returns [ch3conf1::R](ch3conf1::R) reader structure"]
impl crate::Readable for CH3CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch3conf1::W](ch3conf1::W) writer structure"]
impl crate::Writable for CH3CONF1 {}
#[doc = "RMT_CH3CONF1_REG"]
pub mod ch3conf1;
#[doc = "RMT_CH4CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4conf0](ch4conf0) module"]
pub type CH4CONF0 = crate::Reg<u32, _CH4CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CONF0;
#[doc = "`read()` method returns [ch4conf0::R](ch4conf0::R) reader structure"]
impl crate::Readable for CH4CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch4conf0::W](ch4conf0::W) writer structure"]
impl crate::Writable for CH4CONF0 {}
#[doc = "RMT_CH4CONF0_REG"]
pub mod ch4conf0;
#[doc = "RMT_CH4CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4conf1](ch4conf1) module"]
pub type CH4CONF1 = crate::Reg<u32, _CH4CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CONF1;
#[doc = "`read()` method returns [ch4conf1::R](ch4conf1::R) reader structure"]
impl crate::Readable for CH4CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch4conf1::W](ch4conf1::W) writer structure"]
impl crate::Writable for CH4CONF1 {}
#[doc = "RMT_CH4CONF1_REG"]
pub mod ch4conf1;
#[doc = "RMT_CH5CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5conf0](ch5conf0) module"]
pub type CH5CONF0 = crate::Reg<u32, _CH5CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CONF0;
#[doc = "`read()` method returns [ch5conf0::R](ch5conf0::R) reader structure"]
impl crate::Readable for CH5CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch5conf0::W](ch5conf0::W) writer structure"]
impl crate::Writable for CH5CONF0 {}
#[doc = "RMT_CH5CONF0_REG"]
pub mod ch5conf0;
#[doc = "RMT_CH5CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5conf1](ch5conf1) module"]
pub type CH5CONF1 = crate::Reg<u32, _CH5CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CONF1;
#[doc = "`read()` method returns [ch5conf1::R](ch5conf1::R) reader structure"]
impl crate::Readable for CH5CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch5conf1::W](ch5conf1::W) writer structure"]
impl crate::Writable for CH5CONF1 {}
#[doc = "RMT_CH5CONF1_REG"]
pub mod ch5conf1;
#[doc = "RMT_CH6CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch6conf0](ch6conf0) module"]
pub type CH6CONF0 = crate::Reg<u32, _CH6CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CONF0;
#[doc = "`read()` method returns [ch6conf0::R](ch6conf0::R) reader structure"]
impl crate::Readable for CH6CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch6conf0::W](ch6conf0::W) writer structure"]
impl crate::Writable for CH6CONF0 {}
#[doc = "RMT_CH6CONF0_REG"]
pub mod ch6conf0;
#[doc = "RMT_CH6CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch6conf1](ch6conf1) module"]
pub type CH6CONF1 = crate::Reg<u32, _CH6CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CONF1;
#[doc = "`read()` method returns [ch6conf1::R](ch6conf1::R) reader structure"]
impl crate::Readable for CH6CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch6conf1::W](ch6conf1::W) writer structure"]
impl crate::Writable for CH6CONF1 {}
#[doc = "RMT_CH6CONF1_REG"]
pub mod ch6conf1;
#[doc = "RMT_CH7CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch7conf0](ch7conf0) module"]
pub type CH7CONF0 = crate::Reg<u32, _CH7CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CONF0;
#[doc = "`read()` method returns [ch7conf0::R](ch7conf0::R) reader structure"]
impl crate::Readable for CH7CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch7conf0::W](ch7conf0::W) writer structure"]
impl crate::Writable for CH7CONF0 {}
#[doc = "RMT_CH7CONF0_REG"]
pub mod ch7conf0;
#[doc = "RMT_CH7CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch7conf1](ch7conf1) module"]
pub type CH7CONF1 = crate::Reg<u32, _CH7CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CONF1;
#[doc = "`read()` method returns [ch7conf1::R](ch7conf1::R) reader structure"]
impl crate::Readable for CH7CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch7conf1::W](ch7conf1::W) writer structure"]
impl crate::Writable for CH7CONF1 {}
#[doc = "RMT_CH7CONF1_REG"]
pub mod ch7conf1;
#[doc = "RMT_CH0STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0status](ch0status) module"]
pub type CH0STATUS = crate::Reg<u32, _CH0STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STATUS;
#[doc = "`read()` method returns [ch0status::R](ch0status::R) reader structure"]
impl crate::Readable for CH0STATUS {}
#[doc = "`write(|w| ..)` method takes [ch0status::W](ch0status::W) writer structure"]
impl crate::Writable for CH0STATUS {}
#[doc = "RMT_CH0STATUS_REG"]
pub mod ch0status;
#[doc = "RMT_CH1STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1status](ch1status) module"]
pub type CH1STATUS = crate::Reg<u32, _CH1STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STATUS;
#[doc = "`read()` method returns [ch1status::R](ch1status::R) reader structure"]
impl crate::Readable for CH1STATUS {}
#[doc = "`write(|w| ..)` method takes [ch1status::W](ch1status::W) writer structure"]
impl crate::Writable for CH1STATUS {}
#[doc = "RMT_CH1STATUS_REG"]
pub mod ch1status;
#[doc = "RMT_CH2STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2status](ch2status) module"]
pub type CH2STATUS = crate::Reg<u32, _CH2STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STATUS;
#[doc = "`read()` method returns [ch2status::R](ch2status::R) reader structure"]
impl crate::Readable for CH2STATUS {}
#[doc = "`write(|w| ..)` method takes [ch2status::W](ch2status::W) writer structure"]
impl crate::Writable for CH2STATUS {}
#[doc = "RMT_CH2STATUS_REG"]
pub mod ch2status;
#[doc = "RMT_CH3STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3status](ch3status) module"]
pub type CH3STATUS = crate::Reg<u32, _CH3STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STATUS;
#[doc = "`read()` method returns [ch3status::R](ch3status::R) reader structure"]
impl crate::Readable for CH3STATUS {}
#[doc = "`write(|w| ..)` method takes [ch3status::W](ch3status::W) writer structure"]
impl crate::Writable for CH3STATUS {}
#[doc = "RMT_CH3STATUS_REG"]
pub mod ch3status;
#[doc = "RMT_CH4STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4status](ch4status) module"]
pub type CH4STATUS = crate::Reg<u32, _CH4STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4STATUS;
#[doc = "`read()` method returns [ch4status::R](ch4status::R) reader structure"]
impl crate::Readable for CH4STATUS {}
#[doc = "`write(|w| ..)` method takes [ch4status::W](ch4status::W) writer structure"]
impl crate::Writable for CH4STATUS {}
#[doc = "RMT_CH4STATUS_REG"]
pub mod ch4status;
#[doc = "RMT_CH5STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5status](ch5status) module"]
pub type CH5STATUS = crate::Reg<u32, _CH5STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5STATUS;
#[doc = "`read()` method returns [ch5status::R](ch5status::R) reader structure"]
impl crate::Readable for CH5STATUS {}
#[doc = "`write(|w| ..)` method takes [ch5status::W](ch5status::W) writer structure"]
impl crate::Writable for CH5STATUS {}
#[doc = "RMT_CH5STATUS_REG"]
pub mod ch5status;
#[doc = "RMT_CH6STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch6status](ch6status) module"]
pub type CH6STATUS = crate::Reg<u32, _CH6STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6STATUS;
#[doc = "`read()` method returns [ch6status::R](ch6status::R) reader structure"]
impl crate::Readable for CH6STATUS {}
#[doc = "`write(|w| ..)` method takes [ch6status::W](ch6status::W) writer structure"]
impl crate::Writable for CH6STATUS {}
#[doc = "RMT_CH6STATUS_REG"]
pub mod ch6status;
#[doc = "RMT_CH7STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch7status](ch7status) module"]
pub type CH7STATUS = crate::Reg<u32, _CH7STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7STATUS;
#[doc = "`read()` method returns [ch7status::R](ch7status::R) reader structure"]
impl crate::Readable for CH7STATUS {}
#[doc = "`write(|w| ..)` method takes [ch7status::W](ch7status::W) writer structure"]
impl crate::Writable for CH7STATUS {}
#[doc = "RMT_CH7STATUS_REG"]
pub mod ch7status;
#[doc = "RMT_CH0ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0addr](ch0addr) module"]
pub type CH0ADDR = crate::Reg<u32, _CH0ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0ADDR;
#[doc = "`read()` method returns [ch0addr::R](ch0addr::R) reader structure"]
impl crate::Readable for CH0ADDR {}
#[doc = "`write(|w| ..)` method takes [ch0addr::W](ch0addr::W) writer structure"]
impl crate::Writable for CH0ADDR {}
#[doc = "RMT_CH0ADDR_REG"]
pub mod ch0addr;
#[doc = "RMT_CH1ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1addr](ch1addr) module"]
pub type CH1ADDR = crate::Reg<u32, _CH1ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1ADDR;
#[doc = "`read()` method returns [ch1addr::R](ch1addr::R) reader structure"]
impl crate::Readable for CH1ADDR {}
#[doc = "`write(|w| ..)` method takes [ch1addr::W](ch1addr::W) writer structure"]
impl crate::Writable for CH1ADDR {}
#[doc = "RMT_CH1ADDR_REG"]
pub mod ch1addr;
#[doc = "RMT_CH2ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2addr](ch2addr) module"]
pub type CH2ADDR = crate::Reg<u32, _CH2ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2ADDR;
#[doc = "`read()` method returns [ch2addr::R](ch2addr::R) reader structure"]
impl crate::Readable for CH2ADDR {}
#[doc = "`write(|w| ..)` method takes [ch2addr::W](ch2addr::W) writer structure"]
impl crate::Writable for CH2ADDR {}
#[doc = "RMT_CH2ADDR_REG"]
pub mod ch2addr;
#[doc = "RMT_CH3ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3addr](ch3addr) module"]
pub type CH3ADDR = crate::Reg<u32, _CH3ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3ADDR;
#[doc = "`read()` method returns [ch3addr::R](ch3addr::R) reader structure"]
impl crate::Readable for CH3ADDR {}
#[doc = "`write(|w| ..)` method takes [ch3addr::W](ch3addr::W) writer structure"]
impl crate::Writable for CH3ADDR {}
#[doc = "RMT_CH3ADDR_REG"]
pub mod ch3addr;
#[doc = "RMT_CH4ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4addr](ch4addr) module"]
pub type CH4ADDR = crate::Reg<u32, _CH4ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4ADDR;
#[doc = "`read()` method returns [ch4addr::R](ch4addr::R) reader structure"]
impl crate::Readable for CH4ADDR {}
#[doc = "`write(|w| ..)` method takes [ch4addr::W](ch4addr::W) writer structure"]
impl crate::Writable for CH4ADDR {}
#[doc = "RMT_CH4ADDR_REG"]
pub mod ch4addr;
#[doc = "RMT_CH5ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5addr](ch5addr) module"]
pub type CH5ADDR = crate::Reg<u32, _CH5ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5ADDR;
#[doc = "`read()` method returns [ch5addr::R](ch5addr::R) reader structure"]
impl crate::Readable for CH5ADDR {}
#[doc = "`write(|w| ..)` method takes [ch5addr::W](ch5addr::W) writer structure"]
impl crate::Writable for CH5ADDR {}
#[doc = "RMT_CH5ADDR_REG"]
pub mod ch5addr;
#[doc = "RMT_CH6ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch6addr](ch6addr) module"]
pub type CH6ADDR = crate::Reg<u32, _CH6ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6ADDR;
#[doc = "`read()` method returns [ch6addr::R](ch6addr::R) reader structure"]
impl crate::Readable for CH6ADDR {}
#[doc = "`write(|w| ..)` method takes [ch6addr::W](ch6addr::W) writer structure"]
impl crate::Writable for CH6ADDR {}
#[doc = "RMT_CH6ADDR_REG"]
pub mod ch6addr;
#[doc = "RMT_CH7ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch7addr](ch7addr) module"]
pub type CH7ADDR = crate::Reg<u32, _CH7ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7ADDR;
#[doc = "`read()` method returns [ch7addr::R](ch7addr::R) reader structure"]
impl crate::Readable for CH7ADDR {}
#[doc = "`write(|w| ..)` method takes [ch7addr::W](ch7addr::W) writer structure"]
impl crate::Writable for CH7ADDR {}
#[doc = "RMT_CH7ADDR_REG"]
pub mod ch7addr;
#[doc = "RMT_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "RMT_INT_RAW_REG"]
pub mod int_raw;
#[doc = "RMT_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "`write(|w| ..)` method takes [int_st::W](int_st::W) writer structure"]
impl crate::Writable for INT_ST {}
#[doc = "RMT_INT_ST_REG"]
pub mod int_st;
#[doc = "RMT_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "RMT_INT_ENA_REG"]
pub mod int_ena;
#[doc = "RMT_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "RMT_INT_CLR_REG"]
pub mod int_clr;
#[doc = "RMT_CH0CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0carrier_duty](ch0carrier_duty) module"]
pub type CH0CARRIER_DUTY = crate::Reg<u32, _CH0CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CARRIER_DUTY;
#[doc = "`read()` method returns [ch0carrier_duty::R](ch0carrier_duty::R) reader structure"]
impl crate::Readable for CH0CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch0carrier_duty::W](ch0carrier_duty::W) writer structure"]
impl crate::Writable for CH0CARRIER_DUTY {}
#[doc = "RMT_CH0CARRIER_DUTY_REG"]
pub mod ch0carrier_duty;
#[doc = "RMT_CH1CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1carrier_duty](ch1carrier_duty) module"]
pub type CH1CARRIER_DUTY = crate::Reg<u32, _CH1CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CARRIER_DUTY;
#[doc = "`read()` method returns [ch1carrier_duty::R](ch1carrier_duty::R) reader structure"]
impl crate::Readable for CH1CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch1carrier_duty::W](ch1carrier_duty::W) writer structure"]
impl crate::Writable for CH1CARRIER_DUTY {}
#[doc = "RMT_CH1CARRIER_DUTY_REG"]
pub mod ch1carrier_duty;
#[doc = "RMT_CH2CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2carrier_duty](ch2carrier_duty) module"]
pub type CH2CARRIER_DUTY = crate::Reg<u32, _CH2CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CARRIER_DUTY;
#[doc = "`read()` method returns [ch2carrier_duty::R](ch2carrier_duty::R) reader structure"]
impl crate::Readable for CH2CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch2carrier_duty::W](ch2carrier_duty::W) writer structure"]
impl crate::Writable for CH2CARRIER_DUTY {}
#[doc = "RMT_CH2CARRIER_DUTY_REG"]
pub mod ch2carrier_duty;
#[doc = "RMT_CH3CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3carrier_duty](ch3carrier_duty) module"]
pub type CH3CARRIER_DUTY = crate::Reg<u32, _CH3CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CARRIER_DUTY;
#[doc = "`read()` method returns [ch3carrier_duty::R](ch3carrier_duty::R) reader structure"]
impl crate::Readable for CH3CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch3carrier_duty::W](ch3carrier_duty::W) writer structure"]
impl crate::Writable for CH3CARRIER_DUTY {}
#[doc = "RMT_CH3CARRIER_DUTY_REG"]
pub mod ch3carrier_duty;
#[doc = "RMT_CH4CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4carrier_duty](ch4carrier_duty) module"]
pub type CH4CARRIER_DUTY = crate::Reg<u32, _CH4CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CARRIER_DUTY;
#[doc = "`read()` method returns [ch4carrier_duty::R](ch4carrier_duty::R) reader structure"]
impl crate::Readable for CH4CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch4carrier_duty::W](ch4carrier_duty::W) writer structure"]
impl crate::Writable for CH4CARRIER_DUTY {}
#[doc = "RMT_CH4CARRIER_DUTY_REG"]
pub mod ch4carrier_duty;
#[doc = "RMT_CH5CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5carrier_duty](ch5carrier_duty) module"]
pub type CH5CARRIER_DUTY = crate::Reg<u32, _CH5CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CARRIER_DUTY;
#[doc = "`read()` method returns [ch5carrier_duty::R](ch5carrier_duty::R) reader structure"]
impl crate::Readable for CH5CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch5carrier_duty::W](ch5carrier_duty::W) writer structure"]
impl crate::Writable for CH5CARRIER_DUTY {}
#[doc = "RMT_CH5CARRIER_DUTY_REG"]
pub mod ch5carrier_duty;
#[doc = "RMT_CH6CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch6carrier_duty](ch6carrier_duty) module"]
pub type CH6CARRIER_DUTY = crate::Reg<u32, _CH6CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CARRIER_DUTY;
#[doc = "`read()` method returns [ch6carrier_duty::R](ch6carrier_duty::R) reader structure"]
impl crate::Readable for CH6CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch6carrier_duty::W](ch6carrier_duty::W) writer structure"]
impl crate::Writable for CH6CARRIER_DUTY {}
#[doc = "RMT_CH6CARRIER_DUTY_REG"]
pub mod ch6carrier_duty;
#[doc = "RMT_CH7CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch7carrier_duty](ch7carrier_duty) module"]
pub type CH7CARRIER_DUTY = crate::Reg<u32, _CH7CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CARRIER_DUTY;
#[doc = "`read()` method returns [ch7carrier_duty::R](ch7carrier_duty::R) reader structure"]
impl crate::Readable for CH7CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch7carrier_duty::W](ch7carrier_duty::W) writer structure"]
impl crate::Writable for CH7CARRIER_DUTY {}
#[doc = "RMT_CH7CARRIER_DUTY_REG"]
pub mod ch7carrier_duty;
#[doc = "RMT_CH0_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0_tx_lim](ch0_tx_lim) module"]
pub type CH0_TX_LIM = crate::Reg<u32, _CH0_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_TX_LIM;
#[doc = "`read()` method returns [ch0_tx_lim::R](ch0_tx_lim::R) reader structure"]
impl crate::Readable for CH0_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch0_tx_lim::W](ch0_tx_lim::W) writer structure"]
impl crate::Writable for CH0_TX_LIM {}
#[doc = "RMT_CH0_TX_LIM_REG"]
pub mod ch0_tx_lim;
#[doc = "RMT_CH1_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1_tx_lim](ch1_tx_lim) module"]
pub type CH1_TX_LIM = crate::Reg<u32, _CH1_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_TX_LIM;
#[doc = "`read()` method returns [ch1_tx_lim::R](ch1_tx_lim::R) reader structure"]
impl crate::Readable for CH1_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch1_tx_lim::W](ch1_tx_lim::W) writer structure"]
impl crate::Writable for CH1_TX_LIM {}
#[doc = "RMT_CH1_TX_LIM_REG"]
pub mod ch1_tx_lim;
#[doc = "RMT_CH2_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2_tx_lim](ch2_tx_lim) module"]
pub type CH2_TX_LIM = crate::Reg<u32, _CH2_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_TX_LIM;
#[doc = "`read()` method returns [ch2_tx_lim::R](ch2_tx_lim::R) reader structure"]
impl crate::Readable for CH2_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch2_tx_lim::W](ch2_tx_lim::W) writer structure"]
impl crate::Writable for CH2_TX_LIM {}
#[doc = "RMT_CH2_TX_LIM_REG"]
pub mod ch2_tx_lim;
#[doc = "RMT_CH3_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3_tx_lim](ch3_tx_lim) module"]
pub type CH3_TX_LIM = crate::Reg<u32, _CH3_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_TX_LIM;
#[doc = "`read()` method returns [ch3_tx_lim::R](ch3_tx_lim::R) reader structure"]
impl crate::Readable for CH3_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch3_tx_lim::W](ch3_tx_lim::W) writer structure"]
impl crate::Writable for CH3_TX_LIM {}
#[doc = "RMT_CH3_TX_LIM_REG"]
pub mod ch3_tx_lim;
#[doc = "RMT_CH4_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4_tx_lim](ch4_tx_lim) module"]
pub type CH4_TX_LIM = crate::Reg<u32, _CH4_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_TX_LIM;
#[doc = "`read()` method returns [ch4_tx_lim::R](ch4_tx_lim::R) reader structure"]
impl crate::Readable for CH4_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch4_tx_lim::W](ch4_tx_lim::W) writer structure"]
impl crate::Writable for CH4_TX_LIM {}
#[doc = "RMT_CH4_TX_LIM_REG"]
pub mod ch4_tx_lim;
#[doc = "RMT_CH5_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5_tx_lim](ch5_tx_lim) module"]
pub type CH5_TX_LIM = crate::Reg<u32, _CH5_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_TX_LIM;
#[doc = "`read()` method returns [ch5_tx_lim::R](ch5_tx_lim::R) reader structure"]
impl crate::Readable for CH5_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch5_tx_lim::W](ch5_tx_lim::W) writer structure"]
impl crate::Writable for CH5_TX_LIM {}
#[doc = "RMT_CH5_TX_LIM_REG"]
pub mod ch5_tx_lim;
#[doc = "RMT_CH6_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch6_tx_lim](ch6_tx_lim) module"]
pub type CH6_TX_LIM = crate::Reg<u32, _CH6_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_TX_LIM;
#[doc = "`read()` method returns [ch6_tx_lim::R](ch6_tx_lim::R) reader structure"]
impl crate::Readable for CH6_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch6_tx_lim::W](ch6_tx_lim::W) writer structure"]
impl crate::Writable for CH6_TX_LIM {}
#[doc = "RMT_CH6_TX_LIM_REG"]
pub mod ch6_tx_lim;
#[doc = "RMT_CH7_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch7_tx_lim](ch7_tx_lim) module"]
pub type CH7_TX_LIM = crate::Reg<u32, _CH7_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_TX_LIM;
#[doc = "`read()` method returns [ch7_tx_lim::R](ch7_tx_lim::R) reader structure"]
impl crate::Readable for CH7_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch7_tx_lim::W](ch7_tx_lim::W) writer structure"]
impl crate::Writable for CH7_TX_LIM {}
#[doc = "RMT_CH7_TX_LIM_REG"]
pub mod ch7_tx_lim;
#[doc = "RMT_APB_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_conf](apb_conf) module"]
pub type APB_CONF = crate::Reg<u32, _APB_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CONF;
#[doc = "`read()` method returns [apb_conf::R](apb_conf::R) reader structure"]
impl crate::Readable for APB_CONF {}
#[doc = "`write(|w| ..)` method takes [apb_conf::W](apb_conf::W) writer structure"]
impl crate::Writable for APB_CONF {}
#[doc = "RMT_APB_CONF_REG"]
pub mod apb_conf;
#[doc = "RMT_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "RMT_DATE_REG"]
pub mod date;
