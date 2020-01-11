#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_SCL_LOW_PERIOD_REG(i)"]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - I2C_CTR_REG(i)"]
    pub ctr: CTR,
    #[doc = "0x08 - I2C_SR_REG(i)"]
    pub sr: SR,
    #[doc = "0x0c - I2C_TO_REG(i)"]
    pub to: TO,
    #[doc = "0x10 - I2C_SLAVE_ADDR_REG(i)"]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - I2C_RXFIFO_ST_REG(i)"]
    pub rxfifo_st: RXFIFO_ST,
    #[doc = "0x18 - I2C_FIFO_CONF_REG(i)"]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x1c - I2C_DATA_REG(i)"]
    pub data: DATA,
    #[doc = "0x20 - I2C_INT_RAW_REG(i)"]
    pub int_raw: INT_RAW,
    #[doc = "0x24 - I2C_INT_CLR_REG(i)"]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - I2C_INT_ENA_REG(i)"]
    pub int_ena: INT_ENA,
    #[doc = "0x2c - I2C_INT_STATUS_REG(i)"]
    pub int_status: INT_STATUS,
    #[doc = "0x30 - I2C_SDA_HOLD_REG(i)"]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x34 - I2C_SDA_SAMPLE_REG(i)"]
    pub sda_sample: SDA_SAMPLE,
    #[doc = "0x38 - I2C_SCL_HIGH_PERIOD_REG(i)"]
    pub scl_high_period: SCL_HIGH_PERIOD,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - I2C_SCL_START_HOLD_REG(i)"]
    pub scl_start_hold: SCL_START_HOLD,
    #[doc = "0x44 - I2C_SCL_RSTART_SETUP_REG(i)"]
    pub scl_rstart_setup: SCL_RSTART_SETUP,
    #[doc = "0x48 - I2C_SCL_STOP_HOLD_REG(i)"]
    pub scl_stop_hold: SCL_STOP_HOLD,
    #[doc = "0x4c - I2C_SCL_STOP_SETUP_REG(i)"]
    pub scl_stop_setup: SCL_STOP_SETUP,
    #[doc = "0x50 - I2C_SCL_FILTER_CFG_REG(i)"]
    pub scl_filter_cfg: SCL_FILTER_CFG,
    #[doc = "0x54 - I2C_SDA_FILTER_CFG_REG(i)"]
    pub sda_filter_cfg: SDA_FILTER_CFG,
    #[doc = "0x58 - I2C_COMD0_REG(i)"]
    pub comd0: COMD0,
    #[doc = "0x5c - I2C_COMD1_REG(i)"]
    pub comd1: COMD1,
    #[doc = "0x60 - I2C_COMD2_REG(i)"]
    pub comd2: COMD2,
    #[doc = "0x64 - I2C_COMD3_REG(i)"]
    pub comd3: COMD3,
    #[doc = "0x68 - I2C_COMD4_REG(i)"]
    pub comd4: COMD4,
    #[doc = "0x6c - I2C_COMD5_REG(i)"]
    pub comd5: COMD5,
    #[doc = "0x70 - I2C_COMD6_REG(i)"]
    pub comd6: COMD6,
    #[doc = "0x74 - I2C_COMD7_REG(i)"]
    pub comd7: COMD7,
    #[doc = "0x78 - I2C_COMD8_REG(i)"]
    pub comd8: COMD8,
    #[doc = "0x7c - I2C_COMD9_REG(i)"]
    pub comd9: COMD9,
    #[doc = "0x80 - I2C_COMD10_REG(i)"]
    pub comd10: COMD10,
    #[doc = "0x84 - I2C_COMD11_REG(i)"]
    pub comd11: COMD11,
    #[doc = "0x88 - I2C_COMD12_REG(i)"]
    pub comd12: COMD12,
    #[doc = "0x8c - I2C_COMD13_REG(i)"]
    pub comd13: COMD13,
    #[doc = "0x90 - I2C_COMD14_REG(i)"]
    pub comd14: COMD14,
    #[doc = "0x94 - I2C_COMD15_REG(i)"]
    pub comd15: COMD15,
    _reserved37: [u8; 96usize],
    #[doc = "0xf8 - I2C_DATE_REG(i)"]
    pub date: DATE,
}
#[doc = "I2C_SCL_LOW_PERIOD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_low_period](scl_low_period) module"]
pub type SCL_LOW_PERIOD = crate::Reg<u32, _SCL_LOW_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_LOW_PERIOD;
#[doc = "`read()` method returns [scl_low_period::R](scl_low_period::R) reader structure"]
impl crate::Readable for SCL_LOW_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_low_period::W](scl_low_period::W) writer structure"]
impl crate::Writable for SCL_LOW_PERIOD {}
#[doc = "I2C_SCL_LOW_PERIOD_REG(i)"]
pub mod scl_low_period;
#[doc = "I2C_CTR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "I2C_CTR_REG(i)"]
pub mod ctr;
#[doc = "I2C_SR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "I2C_SR_REG(i)"]
pub mod sr;
#[doc = "I2C_TO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [to](to) module"]
pub type TO = crate::Reg<u32, _TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TO;
#[doc = "`read()` method returns [to::R](to::R) reader structure"]
impl crate::Readable for TO {}
#[doc = "`write(|w| ..)` method takes [to::W](to::W) writer structure"]
impl crate::Writable for TO {}
#[doc = "I2C_TO_REG(i)"]
pub mod to;
#[doc = "I2C_SLAVE_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave_addr](slave_addr) module"]
pub type SLAVE_ADDR = crate::Reg<u32, _SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_ADDR;
#[doc = "`read()` method returns [slave_addr::R](slave_addr::R) reader structure"]
impl crate::Readable for SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](slave_addr::W) writer structure"]
impl crate::Writable for SLAVE_ADDR {}
#[doc = "I2C_SLAVE_ADDR_REG(i)"]
pub mod slave_addr;
#[doc = "I2C_RXFIFO_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfifo_st](rxfifo_st) module"]
pub type RXFIFO_ST = crate::Reg<u32, _RXFIFO_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIFO_ST;
#[doc = "`read()` method returns [rxfifo_st::R](rxfifo_st::R) reader structure"]
impl crate::Readable for RXFIFO_ST {}
#[doc = "`write(|w| ..)` method takes [rxfifo_st::W](rxfifo_st::W) writer structure"]
impl crate::Writable for RXFIFO_ST {}
#[doc = "I2C_RXFIFO_ST_REG(i)"]
pub mod rxfifo_st;
#[doc = "I2C_FIFO_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo_conf](fifo_conf) module"]
pub type FIFO_CONF = crate::Reg<u32, _FIFO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CONF;
#[doc = "`read()` method returns [fifo_conf::R](fifo_conf::R) reader structure"]
impl crate::Readable for FIFO_CONF {}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](fifo_conf::W) writer structure"]
impl crate::Writable for FIFO_CONF {}
#[doc = "I2C_FIFO_CONF_REG(i)"]
pub mod fifo_conf;
#[doc = "I2C_DATA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "I2C_DATA_REG(i)"]
pub mod data;
#[doc = "I2C_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "I2C_INT_RAW_REG(i)"]
pub mod int_raw;
#[doc = "I2C_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "I2C_INT_CLR_REG(i)"]
pub mod int_clr;
#[doc = "I2C_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "I2C_INT_ENA_REG(i)"]
pub mod int_ena;
#[doc = "I2C_INT_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [int_status::W](int_status::W) writer structure"]
impl crate::Writable for INT_STATUS {}
#[doc = "I2C_INT_STATUS_REG(i)"]
pub mod int_status;
#[doc = "I2C_SDA_HOLD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda_hold](sda_hold) module"]
pub type SDA_HOLD = crate::Reg<u32, _SDA_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_HOLD;
#[doc = "`read()` method returns [sda_hold::R](sda_hold::R) reader structure"]
impl crate::Readable for SDA_HOLD {}
#[doc = "`write(|w| ..)` method takes [sda_hold::W](sda_hold::W) writer structure"]
impl crate::Writable for SDA_HOLD {}
#[doc = "I2C_SDA_HOLD_REG(i)"]
pub mod sda_hold;
#[doc = "I2C_SDA_SAMPLE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda_sample](sda_sample) module"]
pub type SDA_SAMPLE = crate::Reg<u32, _SDA_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_SAMPLE;
#[doc = "`read()` method returns [sda_sample::R](sda_sample::R) reader structure"]
impl crate::Readable for SDA_SAMPLE {}
#[doc = "`write(|w| ..)` method takes [sda_sample::W](sda_sample::W) writer structure"]
impl crate::Writable for SDA_SAMPLE {}
#[doc = "I2C_SDA_SAMPLE_REG(i)"]
pub mod sda_sample;
#[doc = "I2C_SCL_HIGH_PERIOD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_high_period](scl_high_period) module"]
pub type SCL_HIGH_PERIOD = crate::Reg<u32, _SCL_HIGH_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_HIGH_PERIOD;
#[doc = "`read()` method returns [scl_high_period::R](scl_high_period::R) reader structure"]
impl crate::Readable for SCL_HIGH_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_high_period::W](scl_high_period::W) writer structure"]
impl crate::Writable for SCL_HIGH_PERIOD {}
#[doc = "I2C_SCL_HIGH_PERIOD_REG(i)"]
pub mod scl_high_period;
#[doc = "I2C_SCL_START_HOLD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_start_hold](scl_start_hold) module"]
pub type SCL_START_HOLD = crate::Reg<u32, _SCL_START_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_START_HOLD;
#[doc = "`read()` method returns [scl_start_hold::R](scl_start_hold::R) reader structure"]
impl crate::Readable for SCL_START_HOLD {}
#[doc = "`write(|w| ..)` method takes [scl_start_hold::W](scl_start_hold::W) writer structure"]
impl crate::Writable for SCL_START_HOLD {}
#[doc = "I2C_SCL_START_HOLD_REG(i)"]
pub mod scl_start_hold;
#[doc = "I2C_SCL_RSTART_SETUP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_rstart_setup](scl_rstart_setup) module"]
pub type SCL_RSTART_SETUP = crate::Reg<u32, _SCL_RSTART_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_RSTART_SETUP;
#[doc = "`read()` method returns [scl_rstart_setup::R](scl_rstart_setup::R) reader structure"]
impl crate::Readable for SCL_RSTART_SETUP {}
#[doc = "`write(|w| ..)` method takes [scl_rstart_setup::W](scl_rstart_setup::W) writer structure"]
impl crate::Writable for SCL_RSTART_SETUP {}
#[doc = "I2C_SCL_RSTART_SETUP_REG(i)"]
pub mod scl_rstart_setup;
#[doc = "I2C_SCL_STOP_HOLD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_stop_hold](scl_stop_hold) module"]
pub type SCL_STOP_HOLD = crate::Reg<u32, _SCL_STOP_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STOP_HOLD;
#[doc = "`read()` method returns [scl_stop_hold::R](scl_stop_hold::R) reader structure"]
impl crate::Readable for SCL_STOP_HOLD {}
#[doc = "`write(|w| ..)` method takes [scl_stop_hold::W](scl_stop_hold::W) writer structure"]
impl crate::Writable for SCL_STOP_HOLD {}
#[doc = "I2C_SCL_STOP_HOLD_REG(i)"]
pub mod scl_stop_hold;
#[doc = "I2C_SCL_STOP_SETUP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_stop_setup](scl_stop_setup) module"]
pub type SCL_STOP_SETUP = crate::Reg<u32, _SCL_STOP_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STOP_SETUP;
#[doc = "`read()` method returns [scl_stop_setup::R](scl_stop_setup::R) reader structure"]
impl crate::Readable for SCL_STOP_SETUP {}
#[doc = "`write(|w| ..)` method takes [scl_stop_setup::W](scl_stop_setup::W) writer structure"]
impl crate::Writable for SCL_STOP_SETUP {}
#[doc = "I2C_SCL_STOP_SETUP_REG(i)"]
pub mod scl_stop_setup;
#[doc = "I2C_SCL_FILTER_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scl_filter_cfg](scl_filter_cfg) module"]
pub type SCL_FILTER_CFG = crate::Reg<u32, _SCL_FILTER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_FILTER_CFG;
#[doc = "`read()` method returns [scl_filter_cfg::R](scl_filter_cfg::R) reader structure"]
impl crate::Readable for SCL_FILTER_CFG {}
#[doc = "`write(|w| ..)` method takes [scl_filter_cfg::W](scl_filter_cfg::W) writer structure"]
impl crate::Writable for SCL_FILTER_CFG {}
#[doc = "I2C_SCL_FILTER_CFG_REG(i)"]
pub mod scl_filter_cfg;
#[doc = "I2C_SDA_FILTER_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda_filter_cfg](sda_filter_cfg) module"]
pub type SDA_FILTER_CFG = crate::Reg<u32, _SDA_FILTER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_FILTER_CFG;
#[doc = "`read()` method returns [sda_filter_cfg::R](sda_filter_cfg::R) reader structure"]
impl crate::Readable for SDA_FILTER_CFG {}
#[doc = "`write(|w| ..)` method takes [sda_filter_cfg::W](sda_filter_cfg::W) writer structure"]
impl crate::Writable for SDA_FILTER_CFG {}
#[doc = "I2C_SDA_FILTER_CFG_REG(i)"]
pub mod sda_filter_cfg;
#[doc = "I2C_COMD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd0](comd0) module"]
pub type COMD0 = crate::Reg<u32, _COMD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD0;
#[doc = "`read()` method returns [comd0::R](comd0::R) reader structure"]
impl crate::Readable for COMD0 {}
#[doc = "`write(|w| ..)` method takes [comd0::W](comd0::W) writer structure"]
impl crate::Writable for COMD0 {}
#[doc = "I2C_COMD0_REG(i)"]
pub mod comd0;
#[doc = "I2C_COMD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd1](comd1) module"]
pub type COMD1 = crate::Reg<u32, _COMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD1;
#[doc = "`read()` method returns [comd1::R](comd1::R) reader structure"]
impl crate::Readable for COMD1 {}
#[doc = "`write(|w| ..)` method takes [comd1::W](comd1::W) writer structure"]
impl crate::Writable for COMD1 {}
#[doc = "I2C_COMD1_REG(i)"]
pub mod comd1;
#[doc = "I2C_COMD2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd2](comd2) module"]
pub type COMD2 = crate::Reg<u32, _COMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD2;
#[doc = "`read()` method returns [comd2::R](comd2::R) reader structure"]
impl crate::Readable for COMD2 {}
#[doc = "`write(|w| ..)` method takes [comd2::W](comd2::W) writer structure"]
impl crate::Writable for COMD2 {}
#[doc = "I2C_COMD2_REG(i)"]
pub mod comd2;
#[doc = "I2C_COMD3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd3](comd3) module"]
pub type COMD3 = crate::Reg<u32, _COMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD3;
#[doc = "`read()` method returns [comd3::R](comd3::R) reader structure"]
impl crate::Readable for COMD3 {}
#[doc = "`write(|w| ..)` method takes [comd3::W](comd3::W) writer structure"]
impl crate::Writable for COMD3 {}
#[doc = "I2C_COMD3_REG(i)"]
pub mod comd3;
#[doc = "I2C_COMD4_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd4](comd4) module"]
pub type COMD4 = crate::Reg<u32, _COMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD4;
#[doc = "`read()` method returns [comd4::R](comd4::R) reader structure"]
impl crate::Readable for COMD4 {}
#[doc = "`write(|w| ..)` method takes [comd4::W](comd4::W) writer structure"]
impl crate::Writable for COMD4 {}
#[doc = "I2C_COMD4_REG(i)"]
pub mod comd4;
#[doc = "I2C_COMD5_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd5](comd5) module"]
pub type COMD5 = crate::Reg<u32, _COMD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD5;
#[doc = "`read()` method returns [comd5::R](comd5::R) reader structure"]
impl crate::Readable for COMD5 {}
#[doc = "`write(|w| ..)` method takes [comd5::W](comd5::W) writer structure"]
impl crate::Writable for COMD5 {}
#[doc = "I2C_COMD5_REG(i)"]
pub mod comd5;
#[doc = "I2C_COMD6_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd6](comd6) module"]
pub type COMD6 = crate::Reg<u32, _COMD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD6;
#[doc = "`read()` method returns [comd6::R](comd6::R) reader structure"]
impl crate::Readable for COMD6 {}
#[doc = "`write(|w| ..)` method takes [comd6::W](comd6::W) writer structure"]
impl crate::Writable for COMD6 {}
#[doc = "I2C_COMD6_REG(i)"]
pub mod comd6;
#[doc = "I2C_COMD7_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd7](comd7) module"]
pub type COMD7 = crate::Reg<u32, _COMD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD7;
#[doc = "`read()` method returns [comd7::R](comd7::R) reader structure"]
impl crate::Readable for COMD7 {}
#[doc = "`write(|w| ..)` method takes [comd7::W](comd7::W) writer structure"]
impl crate::Writable for COMD7 {}
#[doc = "I2C_COMD7_REG(i)"]
pub mod comd7;
#[doc = "I2C_COMD8_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd8](comd8) module"]
pub type COMD8 = crate::Reg<u32, _COMD8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD8;
#[doc = "`read()` method returns [comd8::R](comd8::R) reader structure"]
impl crate::Readable for COMD8 {}
#[doc = "`write(|w| ..)` method takes [comd8::W](comd8::W) writer structure"]
impl crate::Writable for COMD8 {}
#[doc = "I2C_COMD8_REG(i)"]
pub mod comd8;
#[doc = "I2C_COMD9_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd9](comd9) module"]
pub type COMD9 = crate::Reg<u32, _COMD9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD9;
#[doc = "`read()` method returns [comd9::R](comd9::R) reader structure"]
impl crate::Readable for COMD9 {}
#[doc = "`write(|w| ..)` method takes [comd9::W](comd9::W) writer structure"]
impl crate::Writable for COMD9 {}
#[doc = "I2C_COMD9_REG(i)"]
pub mod comd9;
#[doc = "I2C_COMD10_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd10](comd10) module"]
pub type COMD10 = crate::Reg<u32, _COMD10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD10;
#[doc = "`read()` method returns [comd10::R](comd10::R) reader structure"]
impl crate::Readable for COMD10 {}
#[doc = "`write(|w| ..)` method takes [comd10::W](comd10::W) writer structure"]
impl crate::Writable for COMD10 {}
#[doc = "I2C_COMD10_REG(i)"]
pub mod comd10;
#[doc = "I2C_COMD11_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd11](comd11) module"]
pub type COMD11 = crate::Reg<u32, _COMD11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD11;
#[doc = "`read()` method returns [comd11::R](comd11::R) reader structure"]
impl crate::Readable for COMD11 {}
#[doc = "`write(|w| ..)` method takes [comd11::W](comd11::W) writer structure"]
impl crate::Writable for COMD11 {}
#[doc = "I2C_COMD11_REG(i)"]
pub mod comd11;
#[doc = "I2C_COMD12_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd12](comd12) module"]
pub type COMD12 = crate::Reg<u32, _COMD12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD12;
#[doc = "`read()` method returns [comd12::R](comd12::R) reader structure"]
impl crate::Readable for COMD12 {}
#[doc = "`write(|w| ..)` method takes [comd12::W](comd12::W) writer structure"]
impl crate::Writable for COMD12 {}
#[doc = "I2C_COMD12_REG(i)"]
pub mod comd12;
#[doc = "I2C_COMD13_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd13](comd13) module"]
pub type COMD13 = crate::Reg<u32, _COMD13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD13;
#[doc = "`read()` method returns [comd13::R](comd13::R) reader structure"]
impl crate::Readable for COMD13 {}
#[doc = "`write(|w| ..)` method takes [comd13::W](comd13::W) writer structure"]
impl crate::Writable for COMD13 {}
#[doc = "I2C_COMD13_REG(i)"]
pub mod comd13;
#[doc = "I2C_COMD14_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd14](comd14) module"]
pub type COMD14 = crate::Reg<u32, _COMD14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD14;
#[doc = "`read()` method returns [comd14::R](comd14::R) reader structure"]
impl crate::Readable for COMD14 {}
#[doc = "`write(|w| ..)` method takes [comd14::W](comd14::W) writer structure"]
impl crate::Writable for COMD14 {}
#[doc = "I2C_COMD14_REG(i)"]
pub mod comd14;
#[doc = "I2C_COMD15_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comd15](comd15) module"]
pub type COMD15 = crate::Reg<u32, _COMD15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD15;
#[doc = "`read()` method returns [comd15::R](comd15::R) reader structure"]
impl crate::Readable for COMD15 {}
#[doc = "`write(|w| ..)` method takes [comd15::W](comd15::W) writer structure"]
impl crate::Writable for COMD15 {}
#[doc = "I2C_COMD15_REG(i)"]
pub mod comd15;
#[doc = "I2C_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "I2C_DATE_REG(i)"]
pub mod date;
