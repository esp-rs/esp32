#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - HOST_SLCHOST_FUNC2_0"]
    pub host_slchost_func2_0: HOST_SLCHOST_FUNC2_0,
    #[doc = "0x14 - HOST_SLCHOST_FUNC2_1"]
    pub host_slchost_func2_1: HOST_SLCHOST_FUNC2_1,
    _reserved2: [u8; 8usize],
    #[doc = "0x20 - HOST_SLCHOST_FUNC2_2"]
    pub host_slchost_func2_2: HOST_SLCHOST_FUNC2_2,
    _reserved3: [u8; 16usize],
    #[doc = "0x34 - HOST_SLCHOST_GPIO_STATUS0"]
    pub host_slchost_gpio_status0: HOST_SLCHOST_GPIO_STATUS0,
    #[doc = "0x38 - HOST_SLCHOST_GPIO_STATUS1"]
    pub host_slchost_gpio_status1: HOST_SLCHOST_GPIO_STATUS1,
    #[doc = "0x3c - HOST_SLCHOST_GPIO_IN0"]
    pub host_slchost_gpio_in0: HOST_SLCHOST_GPIO_IN0,
    #[doc = "0x40 - HOST_SLCHOST_GPIO_IN1"]
    pub host_slchost_gpio_in1: HOST_SLCHOST_GPIO_IN1,
    #[doc = "0x44 - HOST_SLC0HOST_TOKEN_RDATA"]
    pub host_slc0host_token_rdata: HOST_SLC0HOST_TOKEN_RDATA,
    #[doc = "0x48 - HOST_SLC0_HOST_PF"]
    pub host_slc0_host_pf: HOST_SLC0_HOST_PF,
    #[doc = "0x4c - HOST_SLC1_HOST_PF"]
    pub host_slc1_host_pf: HOST_SLC1_HOST_PF,
    #[doc = "0x50 - HOST_SLC0HOST_INT_RAW"]
    pub host_slc0host_int_raw: HOST_SLC0HOST_INT_RAW,
    #[doc = "0x54 - HOST_SLC1HOST_INT_RAW"]
    pub host_slc1host_int_raw: HOST_SLC1HOST_INT_RAW,
    #[doc = "0x58 - HOST_SLC0HOST_INT_ST"]
    pub host_slc0host_int_st: HOST_SLC0HOST_INT_ST,
    #[doc = "0x5c - HOST_SLC1HOST_INT_ST"]
    pub host_slc1host_int_st: HOST_SLC1HOST_INT_ST,
    #[doc = "0x60 - HOST_SLCHOST_PKT_LEN"]
    pub host_slchost_pkt_len: HOST_SLCHOST_PKT_LEN,
    #[doc = "0x64 - HOST_SLCHOST_STATE_W0"]
    pub host_slchost_state_w0: HOST_SLCHOST_STATE_W0,
    #[doc = "0x68 - HOST_SLCHOST_STATE_W1"]
    pub host_slchost_state_w1: HOST_SLCHOST_STATE_W1,
    #[doc = "0x6c - HOST_SLCHOST_CONF_W0"]
    pub host_slchost_conf_w0: HOST_SLCHOST_CONF_W0,
    #[doc = "0x70 - HOST_SLCHOST_CONF_W1"]
    pub host_slchost_conf_w1: HOST_SLCHOST_CONF_W1,
    #[doc = "0x74 - HOST_SLCHOST_CONF_W2"]
    pub host_slchost_conf_w2: HOST_SLCHOST_CONF_W2,
    #[doc = "0x78 - HOST_SLCHOST_CONF_W3"]
    pub host_slchost_conf_w3: HOST_SLCHOST_CONF_W3,
    #[doc = "0x7c - HOST_SLCHOST_CONF_W4"]
    pub host_slchost_conf_w4: HOST_SLCHOST_CONF_W4,
    #[doc = "0x80 - HOST_SLCHOST_CONF_W5"]
    pub host_slchost_conf_w5: HOST_SLCHOST_CONF_W5,
    _reserved23: [u8; 4usize],
    #[doc = "0x88 - HOST_SLCHOST_CONF_W6"]
    pub host_slchost_conf_w6: HOST_SLCHOST_CONF_W6,
    #[doc = "0x8c - HOST_SLCHOST_CONF_W7"]
    pub host_slchost_conf_w7: HOST_SLCHOST_CONF_W7,
    #[doc = "0x90 - HOST_SLCHOST_PKT_LEN0"]
    pub host_slchost_pkt_len0: HOST_SLCHOST_PKT_LEN0,
    #[doc = "0x94 - HOST_SLCHOST_PKT_LEN1"]
    pub host_slchost_pkt_len1: HOST_SLCHOST_PKT_LEN1,
    #[doc = "0x98 - HOST_SLCHOST_PKT_LEN2"]
    pub host_slchost_pkt_len2: HOST_SLCHOST_PKT_LEN2,
    #[doc = "0x9c - HOST_SLCHOST_CONF_W8"]
    pub host_slchost_conf_w8: HOST_SLCHOST_CONF_W8,
    #[doc = "0xa0 - HOST_SLCHOST_CONF_W9"]
    pub host_slchost_conf_w9: HOST_SLCHOST_CONF_W9,
    #[doc = "0xa4 - HOST_SLCHOST_CONF_W10"]
    pub host_slchost_conf_w10: HOST_SLCHOST_CONF_W10,
    #[doc = "0xa8 - HOST_SLCHOST_CONF_W11"]
    pub host_slchost_conf_w11: HOST_SLCHOST_CONF_W11,
    #[doc = "0xac - HOST_SLCHOST_CONF_W12"]
    pub host_slchost_conf_w12: HOST_SLCHOST_CONF_W12,
    #[doc = "0xb0 - HOST_SLCHOST_CONF_W13"]
    pub host_slchost_conf_w13: HOST_SLCHOST_CONF_W13,
    #[doc = "0xb4 - HOST_SLCHOST_CONF_W14"]
    pub host_slchost_conf_w14: HOST_SLCHOST_CONF_W14,
    #[doc = "0xb8 - HOST_SLCHOST_CONF_W15"]
    pub host_slchost_conf_w15: HOST_SLCHOST_CONF_W15,
    #[doc = "0xbc - HOST_SLCHOST_CHECK_SUM0"]
    pub host_slchost_check_sum0: HOST_SLCHOST_CHECK_SUM0,
    #[doc = "0xc0 - HOST_SLCHOST_CHECK_SUM1"]
    pub host_slchost_check_sum1: HOST_SLCHOST_CHECK_SUM1,
    #[doc = "0xc4 - HOST_SLC1HOST_TOKEN_RDATA"]
    pub host_slc1host_token_rdata: HOST_SLC1HOST_TOKEN_RDATA,
    #[doc = "0xc8 - HOST_SLC0HOST_TOKEN_WDATA"]
    pub host_slc0host_token_wdata: HOST_SLC0HOST_TOKEN_WDATA,
    #[doc = "0xcc - HOST_SLC1HOST_TOKEN_WDATA"]
    pub host_slc1host_token_wdata: HOST_SLC1HOST_TOKEN_WDATA,
    #[doc = "0xd0 - HOST_SLCHOST_TOKEN_CON"]
    pub host_slchost_token_con: HOST_SLCHOST_TOKEN_CON,
    #[doc = "0xd4 - HOST_SLC0HOST_INT_CLR"]
    pub host_slc0host_int_clr: HOST_SLC0HOST_INT_CLR,
    #[doc = "0xd8 - HOST_SLC1HOST_INT_CLR"]
    pub host_slc1host_int_clr: HOST_SLC1HOST_INT_CLR,
    #[doc = "0xdc - HOST_SLC0HOST_FUNC1_INT_ENA"]
    pub host_slc0host_func1_int_ena: HOST_SLC0HOST_FUNC1_INT_ENA,
    #[doc = "0xe0 - HOST_SLC1HOST_FUNC1_INT_ENA"]
    pub host_slc1host_func1_int_ena: HOST_SLC1HOST_FUNC1_INT_ENA,
    #[doc = "0xe4 - HOST_SLC0HOST_FUNC2_INT_ENA"]
    pub host_slc0host_func2_int_ena: HOST_SLC0HOST_FUNC2_INT_ENA,
    #[doc = "0xe8 - HOST_SLC1HOST_FUNC2_INT_ENA"]
    pub host_slc1host_func2_int_ena: HOST_SLC1HOST_FUNC2_INT_ENA,
    #[doc = "0xec - HOST_SLC0HOST_INT_ENA"]
    pub host_slc0host_int_ena: HOST_SLC0HOST_INT_ENA,
    #[doc = "0xf0 - HOST_SLC1HOST_INT_ENA"]
    pub host_slc1host_int_ena: HOST_SLC1HOST_INT_ENA,
    #[doc = "0xf4 - HOST_SLC0HOST_RX_INFOR"]
    pub host_slc0host_rx_infor: HOST_SLC0HOST_RX_INFOR,
    #[doc = "0xf8 - HOST_SLC1HOST_RX_INFOR"]
    pub host_slc1host_rx_infor: HOST_SLC1HOST_RX_INFOR,
    #[doc = "0xfc - HOST_SLC0HOST_LEN_WD"]
    pub host_slc0host_len_wd: HOST_SLC0HOST_LEN_WD,
    #[doc = "0x100 - HOST_SLC_APBWIN_WDATA"]
    pub host_slc_apbwin_wdata: HOST_SLC_APBWIN_WDATA,
    #[doc = "0x104 - HOST_SLC_APBWIN_CONF"]
    pub host_slc_apbwin_conf: HOST_SLC_APBWIN_CONF,
    #[doc = "0x108 - HOST_SLC_APBWIN_RDATA"]
    pub host_slc_apbwin_rdata: HOST_SLC_APBWIN_RDATA,
    #[doc = "0x10c - HOST_SLCHOST_RDCLR0"]
    pub host_slchost_rdclr0: HOST_SLCHOST_RDCLR0,
    #[doc = "0x110 - HOST_SLCHOST_RDCLR1"]
    pub host_slchost_rdclr1: HOST_SLCHOST_RDCLR1,
    #[doc = "0x114 - HOST_SLC0HOST_INT_ENA1"]
    pub host_slc0host_int_ena1: HOST_SLC0HOST_INT_ENA1,
    #[doc = "0x118 - HOST_SLC1HOST_INT_ENA1"]
    pub host_slc1host_int_ena1: HOST_SLC1HOST_INT_ENA1,
    _reserved60: [u8; 92usize],
    #[doc = "0x178 - HOST_SLCHOSTDATE"]
    pub host_slchostdate: HOST_SLCHOSTDATE,
    #[doc = "0x17c - HOST_SLCHOSTID"]
    pub host_slchostid: HOST_SLCHOSTID,
    _reserved62: [u8; 112usize],
    #[doc = "0x1f0 - HOST_SLCHOST_CONF"]
    pub host_slchost_conf: HOST_SLCHOST_CONF,
    #[doc = "0x1f4 - HOST_SLCHOST_INF_ST"]
    pub host_slchost_inf_st: HOST_SLCHOST_INF_ST,
}
#[doc = "HOST_SLCHOST_FUNC2_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_func2_0](host_slchost_func2_0) module"]
pub type HOST_SLCHOST_FUNC2_0 = crate::Reg<u32, _HOST_SLCHOST_FUNC2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_FUNC2_0;
#[doc = "`read()` method returns [host_slchost_func2_0::R](host_slchost_func2_0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_0::W](host_slchost_func2_0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_0 {}
#[doc = "HOST_SLCHOST_FUNC2_0"]
pub mod host_slchost_func2_0;
#[doc = "HOST_SLCHOST_FUNC2_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_func2_1](host_slchost_func2_1) module"]
pub type HOST_SLCHOST_FUNC2_1 = crate::Reg<u32, _HOST_SLCHOST_FUNC2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_FUNC2_1;
#[doc = "`read()` method returns [host_slchost_func2_1::R](host_slchost_func2_1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_1::W](host_slchost_func2_1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_1 {}
#[doc = "HOST_SLCHOST_FUNC2_1"]
pub mod host_slchost_func2_1;
#[doc = "HOST_SLCHOST_FUNC2_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_func2_2](host_slchost_func2_2) module"]
pub type HOST_SLCHOST_FUNC2_2 = crate::Reg<u32, _HOST_SLCHOST_FUNC2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_FUNC2_2;
#[doc = "`read()` method returns [host_slchost_func2_2::R](host_slchost_func2_2::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_2 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_2::W](host_slchost_func2_2::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_2 {}
#[doc = "HOST_SLCHOST_FUNC2_2"]
pub mod host_slchost_func2_2;
#[doc = "HOST_SLCHOST_GPIO_STATUS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_status0](host_slchost_gpio_status0) module"]
pub type HOST_SLCHOST_GPIO_STATUS0 = crate::Reg<u32, _HOST_SLCHOST_GPIO_STATUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_STATUS0;
#[doc = "`read()` method returns [host_slchost_gpio_status0::R](host_slchost_gpio_status0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_STATUS0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_status0::W](host_slchost_gpio_status0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_STATUS0 {}
#[doc = "HOST_SLCHOST_GPIO_STATUS0"]
pub mod host_slchost_gpio_status0;
#[doc = "HOST_SLCHOST_GPIO_STATUS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_status1](host_slchost_gpio_status1) module"]
pub type HOST_SLCHOST_GPIO_STATUS1 = crate::Reg<u32, _HOST_SLCHOST_GPIO_STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_STATUS1;
#[doc = "`read()` method returns [host_slchost_gpio_status1::R](host_slchost_gpio_status1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_STATUS1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_status1::W](host_slchost_gpio_status1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_STATUS1 {}
#[doc = "HOST_SLCHOST_GPIO_STATUS1"]
pub mod host_slchost_gpio_status1;
#[doc = "HOST_SLCHOST_GPIO_IN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_in0](host_slchost_gpio_in0) module"]
pub type HOST_SLCHOST_GPIO_IN0 = crate::Reg<u32, _HOST_SLCHOST_GPIO_IN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_IN0;
#[doc = "`read()` method returns [host_slchost_gpio_in0::R](host_slchost_gpio_in0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_in0::W](host_slchost_gpio_in0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_IN0 {}
#[doc = "HOST_SLCHOST_GPIO_IN0"]
pub mod host_slchost_gpio_in0;
#[doc = "HOST_SLCHOST_GPIO_IN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_in1](host_slchost_gpio_in1) module"]
pub type HOST_SLCHOST_GPIO_IN1 = crate::Reg<u32, _HOST_SLCHOST_GPIO_IN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_IN1;
#[doc = "`read()` method returns [host_slchost_gpio_in1::R](host_slchost_gpio_in1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_in1::W](host_slchost_gpio_in1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_IN1 {}
#[doc = "HOST_SLCHOST_GPIO_IN1"]
pub mod host_slchost_gpio_in1;
#[doc = "HOST_SLC0HOST_TOKEN_RDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_token_rdata](host_slc0host_token_rdata) module"]
pub type HOST_SLC0HOST_TOKEN_RDATA = crate::Reg<u32, _HOST_SLC0HOST_TOKEN_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_TOKEN_RDATA;
#[doc = "`read()` method returns [host_slc0host_token_rdata::R](host_slc0host_token_rdata::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_TOKEN_RDATA {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_token_rdata::W](host_slc0host_token_rdata::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_TOKEN_RDATA {}
#[doc = "HOST_SLC0HOST_TOKEN_RDATA"]
pub mod host_slc0host_token_rdata;
#[doc = "HOST_SLC0_HOST_PF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0_host_pf](host_slc0_host_pf) module"]
pub type HOST_SLC0_HOST_PF = crate::Reg<u32, _HOST_SLC0_HOST_PF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0_HOST_PF;
#[doc = "`read()` method returns [host_slc0_host_pf::R](host_slc0_host_pf::R) reader structure"]
impl crate::Readable for HOST_SLC0_HOST_PF {}
#[doc = "`write(|w| ..)` method takes [host_slc0_host_pf::W](host_slc0_host_pf::W) writer structure"]
impl crate::Writable for HOST_SLC0_HOST_PF {}
#[doc = "HOST_SLC0_HOST_PF"]
pub mod host_slc0_host_pf;
#[doc = "HOST_SLC1_HOST_PF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1_host_pf](host_slc1_host_pf) module"]
pub type HOST_SLC1_HOST_PF = crate::Reg<u32, _HOST_SLC1_HOST_PF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1_HOST_PF;
#[doc = "`read()` method returns [host_slc1_host_pf::R](host_slc1_host_pf::R) reader structure"]
impl crate::Readable for HOST_SLC1_HOST_PF {}
#[doc = "`write(|w| ..)` method takes [host_slc1_host_pf::W](host_slc1_host_pf::W) writer structure"]
impl crate::Writable for HOST_SLC1_HOST_PF {}
#[doc = "HOST_SLC1_HOST_PF"]
pub mod host_slc1_host_pf;
#[doc = "HOST_SLC0HOST_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_raw](host_slc0host_int_raw) module"]
pub type HOST_SLC0HOST_INT_RAW = crate::Reg<u32, _HOST_SLC0HOST_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_RAW;
#[doc = "`read()` method returns [host_slc0host_int_raw::R](host_slc0host_int_raw::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_raw::W](host_slc0host_int_raw::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_RAW {}
#[doc = "HOST_SLC0HOST_INT_RAW"]
pub mod host_slc0host_int_raw;
#[doc = "HOST_SLC1HOST_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_raw](host_slc1host_int_raw) module"]
pub type HOST_SLC1HOST_INT_RAW = crate::Reg<u32, _HOST_SLC1HOST_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_RAW;
#[doc = "`read()` method returns [host_slc1host_int_raw::R](host_slc1host_int_raw::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_raw::W](host_slc1host_int_raw::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_RAW {}
#[doc = "HOST_SLC1HOST_INT_RAW"]
pub mod host_slc1host_int_raw;
#[doc = "HOST_SLC0HOST_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_st](host_slc0host_int_st) module"]
pub type HOST_SLC0HOST_INT_ST = crate::Reg<u32, _HOST_SLC0HOST_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_ST;
#[doc = "`read()` method returns [host_slc0host_int_st::R](host_slc0host_int_st::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ST {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_st::W](host_slc0host_int_st::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ST {}
#[doc = "HOST_SLC0HOST_INT_ST"]
pub mod host_slc0host_int_st;
#[doc = "HOST_SLC1HOST_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_st](host_slc1host_int_st) module"]
pub type HOST_SLC1HOST_INT_ST = crate::Reg<u32, _HOST_SLC1HOST_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_ST;
#[doc = "`read()` method returns [host_slc1host_int_st::R](host_slc1host_int_st::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ST {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_st::W](host_slc1host_int_st::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ST {}
#[doc = "HOST_SLC1HOST_INT_ST"]
pub mod host_slc1host_int_st;
#[doc = "HOST_SLCHOST_PKT_LEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len](host_slchost_pkt_len) module"]
pub type HOST_SLCHOST_PKT_LEN = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN;
#[doc = "`read()` method returns [host_slchost_pkt_len::R](host_slchost_pkt_len::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len::W](host_slchost_pkt_len::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN {}
#[doc = "HOST_SLCHOST_PKT_LEN"]
pub mod host_slchost_pkt_len;
#[doc = "HOST_SLCHOST_STATE_W0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_state_w0](host_slchost_state_w0) module"]
pub type HOST_SLCHOST_STATE_W0 = crate::Reg<u32, _HOST_SLCHOST_STATE_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_STATE_W0;
#[doc = "`read()` method returns [host_slchost_state_w0::R](host_slchost_state_w0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_STATE_W0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_state_w0::W](host_slchost_state_w0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_STATE_W0 {}
#[doc = "HOST_SLCHOST_STATE_W0"]
pub mod host_slchost_state_w0;
#[doc = "HOST_SLCHOST_STATE_W1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_state_w1](host_slchost_state_w1) module"]
pub type HOST_SLCHOST_STATE_W1 = crate::Reg<u32, _HOST_SLCHOST_STATE_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_STATE_W1;
#[doc = "`read()` method returns [host_slchost_state_w1::R](host_slchost_state_w1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_STATE_W1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_state_w1::W](host_slchost_state_w1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_STATE_W1 {}
#[doc = "HOST_SLCHOST_STATE_W1"]
pub mod host_slchost_state_w1;
#[doc = "HOST_SLCHOST_CONF_W0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w0](host_slchost_conf_w0) module"]
pub type HOST_SLCHOST_CONF_W0 = crate::Reg<u32, _HOST_SLCHOST_CONF_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W0;
#[doc = "`read()` method returns [host_slchost_conf_w0::R](host_slchost_conf_w0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w0::W](host_slchost_conf_w0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W0 {}
#[doc = "HOST_SLCHOST_CONF_W0"]
pub mod host_slchost_conf_w0;
#[doc = "HOST_SLCHOST_CONF_W1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w1](host_slchost_conf_w1) module"]
pub type HOST_SLCHOST_CONF_W1 = crate::Reg<u32, _HOST_SLCHOST_CONF_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W1;
#[doc = "`read()` method returns [host_slchost_conf_w1::R](host_slchost_conf_w1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w1::W](host_slchost_conf_w1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W1 {}
#[doc = "HOST_SLCHOST_CONF_W1"]
pub mod host_slchost_conf_w1;
#[doc = "HOST_SLCHOST_CONF_W2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w2](host_slchost_conf_w2) module"]
pub type HOST_SLCHOST_CONF_W2 = crate::Reg<u32, _HOST_SLCHOST_CONF_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W2;
#[doc = "`read()` method returns [host_slchost_conf_w2::R](host_slchost_conf_w2::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W2 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w2::W](host_slchost_conf_w2::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W2 {}
#[doc = "HOST_SLCHOST_CONF_W2"]
pub mod host_slchost_conf_w2;
#[doc = "HOST_SLCHOST_CONF_W3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w3](host_slchost_conf_w3) module"]
pub type HOST_SLCHOST_CONF_W3 = crate::Reg<u32, _HOST_SLCHOST_CONF_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W3;
#[doc = "`read()` method returns [host_slchost_conf_w3::R](host_slchost_conf_w3::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W3 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w3::W](host_slchost_conf_w3::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W3 {}
#[doc = "HOST_SLCHOST_CONF_W3"]
pub mod host_slchost_conf_w3;
#[doc = "HOST_SLCHOST_CONF_W4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w4](host_slchost_conf_w4) module"]
pub type HOST_SLCHOST_CONF_W4 = crate::Reg<u32, _HOST_SLCHOST_CONF_W4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W4;
#[doc = "`read()` method returns [host_slchost_conf_w4::R](host_slchost_conf_w4::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W4 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w4::W](host_slchost_conf_w4::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W4 {}
#[doc = "HOST_SLCHOST_CONF_W4"]
pub mod host_slchost_conf_w4;
#[doc = "HOST_SLCHOST_CONF_W5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w5](host_slchost_conf_w5) module"]
pub type HOST_SLCHOST_CONF_W5 = crate::Reg<u32, _HOST_SLCHOST_CONF_W5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W5;
#[doc = "`read()` method returns [host_slchost_conf_w5::R](host_slchost_conf_w5::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W5 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w5::W](host_slchost_conf_w5::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W5 {}
#[doc = "HOST_SLCHOST_CONF_W5"]
pub mod host_slchost_conf_w5;
#[doc = "HOST_SLCHOST_CONF_W6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w6](host_slchost_conf_w6) module"]
pub type HOST_SLCHOST_CONF_W6 = crate::Reg<u32, _HOST_SLCHOST_CONF_W6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W6;
#[doc = "`read()` method returns [host_slchost_conf_w6::R](host_slchost_conf_w6::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W6 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w6::W](host_slchost_conf_w6::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W6 {}
#[doc = "HOST_SLCHOST_CONF_W6"]
pub mod host_slchost_conf_w6;
#[doc = "HOST_SLCHOST_CONF_W7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w7](host_slchost_conf_w7) module"]
pub type HOST_SLCHOST_CONF_W7 = crate::Reg<u32, _HOST_SLCHOST_CONF_W7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W7;
#[doc = "`read()` method returns [host_slchost_conf_w7::R](host_slchost_conf_w7::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W7 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w7::W](host_slchost_conf_w7::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W7 {}
#[doc = "HOST_SLCHOST_CONF_W7"]
pub mod host_slchost_conf_w7;
#[doc = "HOST_SLCHOST_PKT_LEN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len0](host_slchost_pkt_len0) module"]
pub type HOST_SLCHOST_PKT_LEN0 = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN0;
#[doc = "`read()` method returns [host_slchost_pkt_len0::R](host_slchost_pkt_len0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len0::W](host_slchost_pkt_len0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN0 {}
#[doc = "HOST_SLCHOST_PKT_LEN0"]
pub mod host_slchost_pkt_len0;
#[doc = "HOST_SLCHOST_PKT_LEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len1](host_slchost_pkt_len1) module"]
pub type HOST_SLCHOST_PKT_LEN1 = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN1;
#[doc = "`read()` method returns [host_slchost_pkt_len1::R](host_slchost_pkt_len1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len1::W](host_slchost_pkt_len1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN1 {}
#[doc = "HOST_SLCHOST_PKT_LEN1"]
pub mod host_slchost_pkt_len1;
#[doc = "HOST_SLCHOST_PKT_LEN2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len2](host_slchost_pkt_len2) module"]
pub type HOST_SLCHOST_PKT_LEN2 = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN2;
#[doc = "`read()` method returns [host_slchost_pkt_len2::R](host_slchost_pkt_len2::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN2 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len2::W](host_slchost_pkt_len2::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN2 {}
#[doc = "HOST_SLCHOST_PKT_LEN2"]
pub mod host_slchost_pkt_len2;
#[doc = "HOST_SLCHOST_CONF_W8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w8](host_slchost_conf_w8) module"]
pub type HOST_SLCHOST_CONF_W8 = crate::Reg<u32, _HOST_SLCHOST_CONF_W8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W8;
#[doc = "`read()` method returns [host_slchost_conf_w8::R](host_slchost_conf_w8::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W8 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w8::W](host_slchost_conf_w8::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W8 {}
#[doc = "HOST_SLCHOST_CONF_W8"]
pub mod host_slchost_conf_w8;
#[doc = "HOST_SLCHOST_CONF_W9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w9](host_slchost_conf_w9) module"]
pub type HOST_SLCHOST_CONF_W9 = crate::Reg<u32, _HOST_SLCHOST_CONF_W9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W9;
#[doc = "`read()` method returns [host_slchost_conf_w9::R](host_slchost_conf_w9::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W9 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w9::W](host_slchost_conf_w9::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W9 {}
#[doc = "HOST_SLCHOST_CONF_W9"]
pub mod host_slchost_conf_w9;
#[doc = "HOST_SLCHOST_CONF_W10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w10](host_slchost_conf_w10) module"]
pub type HOST_SLCHOST_CONF_W10 = crate::Reg<u32, _HOST_SLCHOST_CONF_W10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W10;
#[doc = "`read()` method returns [host_slchost_conf_w10::R](host_slchost_conf_w10::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W10 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w10::W](host_slchost_conf_w10::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W10 {}
#[doc = "HOST_SLCHOST_CONF_W10"]
pub mod host_slchost_conf_w10;
#[doc = "HOST_SLCHOST_CONF_W11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w11](host_slchost_conf_w11) module"]
pub type HOST_SLCHOST_CONF_W11 = crate::Reg<u32, _HOST_SLCHOST_CONF_W11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W11;
#[doc = "`read()` method returns [host_slchost_conf_w11::R](host_slchost_conf_w11::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W11 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w11::W](host_slchost_conf_w11::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W11 {}
#[doc = "HOST_SLCHOST_CONF_W11"]
pub mod host_slchost_conf_w11;
#[doc = "HOST_SLCHOST_CONF_W12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w12](host_slchost_conf_w12) module"]
pub type HOST_SLCHOST_CONF_W12 = crate::Reg<u32, _HOST_SLCHOST_CONF_W12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W12;
#[doc = "`read()` method returns [host_slchost_conf_w12::R](host_slchost_conf_w12::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W12 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w12::W](host_slchost_conf_w12::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W12 {}
#[doc = "HOST_SLCHOST_CONF_W12"]
pub mod host_slchost_conf_w12;
#[doc = "HOST_SLCHOST_CONF_W13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w13](host_slchost_conf_w13) module"]
pub type HOST_SLCHOST_CONF_W13 = crate::Reg<u32, _HOST_SLCHOST_CONF_W13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W13;
#[doc = "`read()` method returns [host_slchost_conf_w13::R](host_slchost_conf_w13::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W13 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w13::W](host_slchost_conf_w13::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W13 {}
#[doc = "HOST_SLCHOST_CONF_W13"]
pub mod host_slchost_conf_w13;
#[doc = "HOST_SLCHOST_CONF_W14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w14](host_slchost_conf_w14) module"]
pub type HOST_SLCHOST_CONF_W14 = crate::Reg<u32, _HOST_SLCHOST_CONF_W14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W14;
#[doc = "`read()` method returns [host_slchost_conf_w14::R](host_slchost_conf_w14::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W14 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w14::W](host_slchost_conf_w14::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W14 {}
#[doc = "HOST_SLCHOST_CONF_W14"]
pub mod host_slchost_conf_w14;
#[doc = "HOST_SLCHOST_CONF_W15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w15](host_slchost_conf_w15) module"]
pub type HOST_SLCHOST_CONF_W15 = crate::Reg<u32, _HOST_SLCHOST_CONF_W15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W15;
#[doc = "`read()` method returns [host_slchost_conf_w15::R](host_slchost_conf_w15::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W15 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w15::W](host_slchost_conf_w15::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W15 {}
#[doc = "HOST_SLCHOST_CONF_W15"]
pub mod host_slchost_conf_w15;
#[doc = "HOST_SLCHOST_CHECK_SUM0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_check_sum0](host_slchost_check_sum0) module"]
pub type HOST_SLCHOST_CHECK_SUM0 = crate::Reg<u32, _HOST_SLCHOST_CHECK_SUM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CHECK_SUM0;
#[doc = "`read()` method returns [host_slchost_check_sum0::R](host_slchost_check_sum0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CHECK_SUM0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_check_sum0::W](host_slchost_check_sum0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CHECK_SUM0 {}
#[doc = "HOST_SLCHOST_CHECK_SUM0"]
pub mod host_slchost_check_sum0;
#[doc = "HOST_SLCHOST_CHECK_SUM1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_check_sum1](host_slchost_check_sum1) module"]
pub type HOST_SLCHOST_CHECK_SUM1 = crate::Reg<u32, _HOST_SLCHOST_CHECK_SUM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CHECK_SUM1;
#[doc = "`read()` method returns [host_slchost_check_sum1::R](host_slchost_check_sum1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CHECK_SUM1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_check_sum1::W](host_slchost_check_sum1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CHECK_SUM1 {}
#[doc = "HOST_SLCHOST_CHECK_SUM1"]
pub mod host_slchost_check_sum1;
#[doc = "HOST_SLC1HOST_TOKEN_RDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_token_rdata](host_slc1host_token_rdata) module"]
pub type HOST_SLC1HOST_TOKEN_RDATA = crate::Reg<u32, _HOST_SLC1HOST_TOKEN_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_TOKEN_RDATA;
#[doc = "`read()` method returns [host_slc1host_token_rdata::R](host_slc1host_token_rdata::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_TOKEN_RDATA {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_token_rdata::W](host_slc1host_token_rdata::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_TOKEN_RDATA {}
#[doc = "HOST_SLC1HOST_TOKEN_RDATA"]
pub mod host_slc1host_token_rdata;
#[doc = "HOST_SLC0HOST_TOKEN_WDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_token_wdata](host_slc0host_token_wdata) module"]
pub type HOST_SLC0HOST_TOKEN_WDATA = crate::Reg<u32, _HOST_SLC0HOST_TOKEN_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_TOKEN_WDATA;
#[doc = "`read()` method returns [host_slc0host_token_wdata::R](host_slc0host_token_wdata::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_TOKEN_WDATA {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_token_wdata::W](host_slc0host_token_wdata::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_TOKEN_WDATA {}
#[doc = "HOST_SLC0HOST_TOKEN_WDATA"]
pub mod host_slc0host_token_wdata;
#[doc = "HOST_SLC1HOST_TOKEN_WDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_token_wdata](host_slc1host_token_wdata) module"]
pub type HOST_SLC1HOST_TOKEN_WDATA = crate::Reg<u32, _HOST_SLC1HOST_TOKEN_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_TOKEN_WDATA;
#[doc = "`read()` method returns [host_slc1host_token_wdata::R](host_slc1host_token_wdata::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_TOKEN_WDATA {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_token_wdata::W](host_slc1host_token_wdata::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_TOKEN_WDATA {}
#[doc = "HOST_SLC1HOST_TOKEN_WDATA"]
pub mod host_slc1host_token_wdata;
#[doc = "HOST_SLCHOST_TOKEN_CON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_token_con](host_slchost_token_con) module"]
pub type HOST_SLCHOST_TOKEN_CON = crate::Reg<u32, _HOST_SLCHOST_TOKEN_CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_TOKEN_CON;
#[doc = "`read()` method returns [host_slchost_token_con::R](host_slchost_token_con::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_TOKEN_CON {}
#[doc = "`write(|w| ..)` method takes [host_slchost_token_con::W](host_slchost_token_con::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_TOKEN_CON {}
#[doc = "HOST_SLCHOST_TOKEN_CON"]
pub mod host_slchost_token_con;
#[doc = "HOST_SLC0HOST_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_clr](host_slc0host_int_clr) module"]
pub type HOST_SLC0HOST_INT_CLR = crate::Reg<u32, _HOST_SLC0HOST_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_CLR;
#[doc = "`read()` method returns [host_slc0host_int_clr::R](host_slc0host_int_clr::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_CLR {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_clr::W](host_slc0host_int_clr::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_CLR {}
#[doc = "HOST_SLC0HOST_INT_CLR"]
pub mod host_slc0host_int_clr;
#[doc = "HOST_SLC1HOST_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_clr](host_slc1host_int_clr) module"]
pub type HOST_SLC1HOST_INT_CLR = crate::Reg<u32, _HOST_SLC1HOST_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_CLR;
#[doc = "`read()` method returns [host_slc1host_int_clr::R](host_slc1host_int_clr::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_CLR {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_clr::W](host_slc1host_int_clr::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_CLR {}
#[doc = "HOST_SLC1HOST_INT_CLR"]
pub mod host_slc1host_int_clr;
#[doc = "HOST_SLC0HOST_FUNC1_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_func1_int_ena](host_slc0host_func1_int_ena) module"]
pub type HOST_SLC0HOST_FUNC1_INT_ENA = crate::Reg<u32, _HOST_SLC0HOST_FUNC1_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_FUNC1_INT_ENA;
#[doc = "`read()` method returns [host_slc0host_func1_int_ena::R](host_slc0host_func1_int_ena::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_FUNC1_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_func1_int_ena::W](host_slc0host_func1_int_ena::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_FUNC1_INT_ENA {}
#[doc = "HOST_SLC0HOST_FUNC1_INT_ENA"]
pub mod host_slc0host_func1_int_ena;
#[doc = "HOST_SLC1HOST_FUNC1_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_func1_int_ena](host_slc1host_func1_int_ena) module"]
pub type HOST_SLC1HOST_FUNC1_INT_ENA = crate::Reg<u32, _HOST_SLC1HOST_FUNC1_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_FUNC1_INT_ENA;
#[doc = "`read()` method returns [host_slc1host_func1_int_ena::R](host_slc1host_func1_int_ena::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_FUNC1_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_func1_int_ena::W](host_slc1host_func1_int_ena::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_FUNC1_INT_ENA {}
#[doc = "HOST_SLC1HOST_FUNC1_INT_ENA"]
pub mod host_slc1host_func1_int_ena;
#[doc = "HOST_SLC0HOST_FUNC2_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_func2_int_ena](host_slc0host_func2_int_ena) module"]
pub type HOST_SLC0HOST_FUNC2_INT_ENA = crate::Reg<u32, _HOST_SLC0HOST_FUNC2_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_FUNC2_INT_ENA;
#[doc = "`read()` method returns [host_slc0host_func2_int_ena::R](host_slc0host_func2_int_ena::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_FUNC2_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_func2_int_ena::W](host_slc0host_func2_int_ena::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_FUNC2_INT_ENA {}
#[doc = "HOST_SLC0HOST_FUNC2_INT_ENA"]
pub mod host_slc0host_func2_int_ena;
#[doc = "HOST_SLC1HOST_FUNC2_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_func2_int_ena](host_slc1host_func2_int_ena) module"]
pub type HOST_SLC1HOST_FUNC2_INT_ENA = crate::Reg<u32, _HOST_SLC1HOST_FUNC2_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_FUNC2_INT_ENA;
#[doc = "`read()` method returns [host_slc1host_func2_int_ena::R](host_slc1host_func2_int_ena::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_FUNC2_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_func2_int_ena::W](host_slc1host_func2_int_ena::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_FUNC2_INT_ENA {}
#[doc = "HOST_SLC1HOST_FUNC2_INT_ENA"]
pub mod host_slc1host_func2_int_ena;
#[doc = "HOST_SLC0HOST_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_ena](host_slc0host_int_ena) module"]
pub type HOST_SLC0HOST_INT_ENA = crate::Reg<u32, _HOST_SLC0HOST_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_ENA;
#[doc = "`read()` method returns [host_slc0host_int_ena::R](host_slc0host_int_ena::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_ena::W](host_slc0host_int_ena::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ENA {}
#[doc = "HOST_SLC0HOST_INT_ENA"]
pub mod host_slc0host_int_ena;
#[doc = "HOST_SLC1HOST_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_ena](host_slc1host_int_ena) module"]
pub type HOST_SLC1HOST_INT_ENA = crate::Reg<u32, _HOST_SLC1HOST_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_ENA;
#[doc = "`read()` method returns [host_slc1host_int_ena::R](host_slc1host_int_ena::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_ena::W](host_slc1host_int_ena::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ENA {}
#[doc = "HOST_SLC1HOST_INT_ENA"]
pub mod host_slc1host_int_ena;
#[doc = "HOST_SLC0HOST_RX_INFOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_rx_infor](host_slc0host_rx_infor) module"]
pub type HOST_SLC0HOST_RX_INFOR = crate::Reg<u32, _HOST_SLC0HOST_RX_INFOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_RX_INFOR;
#[doc = "`read()` method returns [host_slc0host_rx_infor::R](host_slc0host_rx_infor::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_RX_INFOR {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_rx_infor::W](host_slc0host_rx_infor::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_RX_INFOR {}
#[doc = "HOST_SLC0HOST_RX_INFOR"]
pub mod host_slc0host_rx_infor;
#[doc = "HOST_SLC1HOST_RX_INFOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_rx_infor](host_slc1host_rx_infor) module"]
pub type HOST_SLC1HOST_RX_INFOR = crate::Reg<u32, _HOST_SLC1HOST_RX_INFOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_RX_INFOR;
#[doc = "`read()` method returns [host_slc1host_rx_infor::R](host_slc1host_rx_infor::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_RX_INFOR {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_rx_infor::W](host_slc1host_rx_infor::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_RX_INFOR {}
#[doc = "HOST_SLC1HOST_RX_INFOR"]
pub mod host_slc1host_rx_infor;
#[doc = "HOST_SLC0HOST_LEN_WD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_len_wd](host_slc0host_len_wd) module"]
pub type HOST_SLC0HOST_LEN_WD = crate::Reg<u32, _HOST_SLC0HOST_LEN_WD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_LEN_WD;
#[doc = "`read()` method returns [host_slc0host_len_wd::R](host_slc0host_len_wd::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_LEN_WD {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_len_wd::W](host_slc0host_len_wd::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_LEN_WD {}
#[doc = "HOST_SLC0HOST_LEN_WD"]
pub mod host_slc0host_len_wd;
#[doc = "HOST_SLC_APBWIN_WDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc_apbwin_wdata](host_slc_apbwin_wdata) module"]
pub type HOST_SLC_APBWIN_WDATA = crate::Reg<u32, _HOST_SLC_APBWIN_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC_APBWIN_WDATA;
#[doc = "`read()` method returns [host_slc_apbwin_wdata::R](host_slc_apbwin_wdata::R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_WDATA {}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_wdata::W](host_slc_apbwin_wdata::W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_WDATA {}
#[doc = "HOST_SLC_APBWIN_WDATA"]
pub mod host_slc_apbwin_wdata;
#[doc = "HOST_SLC_APBWIN_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc_apbwin_conf](host_slc_apbwin_conf) module"]
pub type HOST_SLC_APBWIN_CONF = crate::Reg<u32, _HOST_SLC_APBWIN_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC_APBWIN_CONF;
#[doc = "`read()` method returns [host_slc_apbwin_conf::R](host_slc_apbwin_conf::R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_CONF {}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_conf::W](host_slc_apbwin_conf::W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_CONF {}
#[doc = "HOST_SLC_APBWIN_CONF"]
pub mod host_slc_apbwin_conf;
#[doc = "HOST_SLC_APBWIN_RDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc_apbwin_rdata](host_slc_apbwin_rdata) module"]
pub type HOST_SLC_APBWIN_RDATA = crate::Reg<u32, _HOST_SLC_APBWIN_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC_APBWIN_RDATA;
#[doc = "`read()` method returns [host_slc_apbwin_rdata::R](host_slc_apbwin_rdata::R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_RDATA {}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_rdata::W](host_slc_apbwin_rdata::W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_RDATA {}
#[doc = "HOST_SLC_APBWIN_RDATA"]
pub mod host_slc_apbwin_rdata;
#[doc = "HOST_SLCHOST_RDCLR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_rdclr0](host_slchost_rdclr0) module"]
pub type HOST_SLCHOST_RDCLR0 = crate::Reg<u32, _HOST_SLCHOST_RDCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_RDCLR0;
#[doc = "`read()` method returns [host_slchost_rdclr0::R](host_slchost_rdclr0::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR0 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr0::W](host_slchost_rdclr0::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR0 {}
#[doc = "HOST_SLCHOST_RDCLR0"]
pub mod host_slchost_rdclr0;
#[doc = "HOST_SLCHOST_RDCLR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_rdclr1](host_slchost_rdclr1) module"]
pub type HOST_SLCHOST_RDCLR1 = crate::Reg<u32, _HOST_SLCHOST_RDCLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_RDCLR1;
#[doc = "`read()` method returns [host_slchost_rdclr1::R](host_slchost_rdclr1::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR1 {}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr1::W](host_slchost_rdclr1::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR1 {}
#[doc = "HOST_SLCHOST_RDCLR1"]
pub mod host_slchost_rdclr1;
#[doc = "HOST_SLC0HOST_INT_ENA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_ena1](host_slc0host_int_ena1) module"]
pub type HOST_SLC0HOST_INT_ENA1 = crate::Reg<u32, _HOST_SLC0HOST_INT_ENA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_ENA1;
#[doc = "`read()` method returns [host_slc0host_int_ena1::R](host_slc0host_int_ena1::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ENA1 {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_ena1::W](host_slc0host_int_ena1::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ENA1 {}
#[doc = "HOST_SLC0HOST_INT_ENA1"]
pub mod host_slc0host_int_ena1;
#[doc = "HOST_SLC1HOST_INT_ENA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_ena1](host_slc1host_int_ena1) module"]
pub type HOST_SLC1HOST_INT_ENA1 = crate::Reg<u32, _HOST_SLC1HOST_INT_ENA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_ENA1;
#[doc = "`read()` method returns [host_slc1host_int_ena1::R](host_slc1host_int_ena1::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ENA1 {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_ena1::W](host_slc1host_int_ena1::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ENA1 {}
#[doc = "HOST_SLC1HOST_INT_ENA1"]
pub mod host_slc1host_int_ena1;
#[doc = "HOST_SLCHOSTDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchostdate](host_slchostdate) module"]
pub type HOST_SLCHOSTDATE = crate::Reg<u32, _HOST_SLCHOSTDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOSTDATE;
#[doc = "`read()` method returns [host_slchostdate::R](host_slchostdate::R) reader structure"]
impl crate::Readable for HOST_SLCHOSTDATE {}
#[doc = "`write(|w| ..)` method takes [host_slchostdate::W](host_slchostdate::W) writer structure"]
impl crate::Writable for HOST_SLCHOSTDATE {}
#[doc = "HOST_SLCHOSTDATE"]
pub mod host_slchostdate;
#[doc = "HOST_SLCHOSTID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchostid](host_slchostid) module"]
pub type HOST_SLCHOSTID = crate::Reg<u32, _HOST_SLCHOSTID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOSTID;
#[doc = "`read()` method returns [host_slchostid::R](host_slchostid::R) reader structure"]
impl crate::Readable for HOST_SLCHOSTID {}
#[doc = "`write(|w| ..)` method takes [host_slchostid::W](host_slchostid::W) writer structure"]
impl crate::Writable for HOST_SLCHOSTID {}
#[doc = "HOST_SLCHOSTID"]
pub mod host_slchostid;
#[doc = "HOST_SLCHOST_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf](host_slchost_conf) module"]
pub type HOST_SLCHOST_CONF = crate::Reg<u32, _HOST_SLCHOST_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF;
#[doc = "`read()` method returns [host_slchost_conf::R](host_slchost_conf::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf::W](host_slchost_conf::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF {}
#[doc = "HOST_SLCHOST_CONF"]
pub mod host_slchost_conf;
#[doc = "HOST_SLCHOST_INF_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_inf_st](host_slchost_inf_st) module"]
pub type HOST_SLCHOST_INF_ST = crate::Reg<u32, _HOST_SLCHOST_INF_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_INF_ST;
#[doc = "`read()` method returns [host_slchost_inf_st::R](host_slchost_inf_st::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_INF_ST {}
#[doc = "`write(|w| ..)` method takes [host_slchost_inf_st::W](host_slchost_inf_st::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_INF_ST {}
#[doc = "HOST_SLCHOST_INF_ST"]
pub mod host_slchost_inf_st;
