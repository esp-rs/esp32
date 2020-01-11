#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SLC_CONF0"]
    pub conf0: CONF0,
    #[doc = "0x04 - SLC_0INT_RAW"]
    pub _0int_raw: _0INT_RAW,
    #[doc = "0x08 - SLC_0INT_ST"]
    pub _0int_st: _0INT_ST,
    #[doc = "0x0c - SLC_0INT_ENA"]
    pub _0int_ena: _0INT_ENA,
    #[doc = "0x10 - SLC_0INT_CLR"]
    pub _0int_clr: _0INT_CLR,
    #[doc = "0x14 - SLC_1INT_RAW"]
    pub _1int_raw: _1INT_RAW,
    #[doc = "0x18 - SLC_1INT_ST"]
    pub _1int_st: _1INT_ST,
    #[doc = "0x1c - SLC_1INT_ENA"]
    pub _1int_ena: _1INT_ENA,
    #[doc = "0x20 - SLC_1INT_CLR"]
    pub _1int_clr: _1INT_CLR,
    #[doc = "0x24 - SLC_RX_STATUS"]
    pub rx_status: RX_STATUS,
    #[doc = "0x28 - SLC_0RXFIFO_PUSH"]
    pub _0rxfifo_push: _0RXFIFO_PUSH,
    #[doc = "0x2c - SLC_1RXFIFO_PUSH"]
    pub _1rxfifo_push: _1RXFIFO_PUSH,
    #[doc = "0x30 - SLC_TX_STATUS"]
    pub tx_status: TX_STATUS,
    #[doc = "0x34 - SLC_0TXFIFO_POP"]
    pub _0txfifo_pop: _0TXFIFO_POP,
    #[doc = "0x38 - SLC_1TXFIFO_POP"]
    pub _1txfifo_pop: _1TXFIFO_POP,
    #[doc = "0x3c - SLC_0RX_LINK"]
    pub _0rx_link: _0RX_LINK,
    #[doc = "0x40 - SLC_0TX_LINK"]
    pub _0tx_link: _0TX_LINK,
    #[doc = "0x44 - SLC_1RX_LINK"]
    pub _1rx_link: _1RX_LINK,
    #[doc = "0x48 - SLC_1TX_LINK"]
    pub _1tx_link: _1TX_LINK,
    #[doc = "0x4c - SLC_INTVEC_TOHOST"]
    pub intvec_tohost: INTVEC_TOHOST,
    #[doc = "0x50 - SLC_0TOKEN0"]
    pub _0token0: _0TOKEN0,
    #[doc = "0x54 - SLC_0TOKEN1"]
    pub _0token1: _0TOKEN1,
    #[doc = "0x58 - SLC_1TOKEN0"]
    pub _1token0: _1TOKEN0,
    #[doc = "0x5c - SLC_1TOKEN1"]
    pub _1token1: _1TOKEN1,
    #[doc = "0x60 - SLC_CONF1"]
    pub conf1: CONF1,
    #[doc = "0x64 - SLC_0_STATE0"]
    pub _0_state0: _0_STATE0,
    #[doc = "0x68 - SLC_0_STATE1"]
    pub _0_state1: _0_STATE1,
    #[doc = "0x6c - SLC_1_STATE0"]
    pub _1_state0: _1_STATE0,
    #[doc = "0x70 - SLC_1_STATE1"]
    pub _1_state1: _1_STATE1,
    #[doc = "0x74 - SLC_BRIDGE_CONF"]
    pub bridge_conf: BRIDGE_CONF,
    #[doc = "0x78 - SLC_0_TO_EOF_DES_ADDR"]
    pub _0_to_eof_des_addr: _0_TO_EOF_DES_ADDR,
    #[doc = "0x7c - SLC_0_TX_EOF_DES_ADDR"]
    pub _0_tx_eof_des_addr: _0_TX_EOF_DES_ADDR,
    #[doc = "0x80 - SLC_0_TO_EOF_BFR_DES_ADDR"]
    pub _0_to_eof_bfr_des_addr: _0_TO_EOF_BFR_DES_ADDR,
    #[doc = "0x84 - SLC_1_TO_EOF_DES_ADDR"]
    pub _1_to_eof_des_addr: _1_TO_EOF_DES_ADDR,
    #[doc = "0x88 - SLC_1_TX_EOF_DES_ADDR"]
    pub _1_tx_eof_des_addr: _1_TX_EOF_DES_ADDR,
    #[doc = "0x8c - SLC_1_TO_EOF_BFR_DES_ADDR"]
    pub _1_to_eof_bfr_des_addr: _1_TO_EOF_BFR_DES_ADDR,
    #[doc = "0x90 - SLC_AHB_TEST"]
    pub ahb_test: AHB_TEST,
    #[doc = "0x94 - SLC_SDIO_ST"]
    pub sdio_st: SDIO_ST,
    #[doc = "0x98 - SLC_RX_DSCR_CONF"]
    pub rx_dscr_conf: RX_DSCR_CONF,
    #[doc = "0x9c - SLC_0_TXLINK_DSCR"]
    pub _0_txlink_dscr: _0_TXLINK_DSCR,
    #[doc = "0xa0 - SLC_0_TXLINK_DSCR_BF0"]
    pub _0_txlink_dscr_bf0: _0_TXLINK_DSCR_BF0,
    #[doc = "0xa4 - SLC_0_TXLINK_DSCR_BF1"]
    pub _0_txlink_dscr_bf1: _0_TXLINK_DSCR_BF1,
    #[doc = "0xa8 - SLC_0_RXLINK_DSCR"]
    pub _0_rxlink_dscr: _0_RXLINK_DSCR,
    #[doc = "0xac - SLC_0_RXLINK_DSCR_BF0"]
    pub _0_rxlink_dscr_bf0: _0_RXLINK_DSCR_BF0,
    #[doc = "0xb0 - SLC_0_RXLINK_DSCR_BF1"]
    pub _0_rxlink_dscr_bf1: _0_RXLINK_DSCR_BF1,
    #[doc = "0xb4 - SLC_1_TXLINK_DSCR"]
    pub _1_txlink_dscr: _1_TXLINK_DSCR,
    #[doc = "0xb8 - SLC_1_TXLINK_DSCR_BF0"]
    pub _1_txlink_dscr_bf0: _1_TXLINK_DSCR_BF0,
    #[doc = "0xbc - SLC_1_TXLINK_DSCR_BF1"]
    pub _1_txlink_dscr_bf1: _1_TXLINK_DSCR_BF1,
    #[doc = "0xc0 - SLC_1_RXLINK_DSCR"]
    pub _1_rxlink_dscr: _1_RXLINK_DSCR,
    #[doc = "0xc4 - SLC_1_RXLINK_DSCR_BF0"]
    pub _1_rxlink_dscr_bf0: _1_RXLINK_DSCR_BF0,
    #[doc = "0xc8 - SLC_1_RXLINK_DSCR_BF1"]
    pub _1_rxlink_dscr_bf1: _1_RXLINK_DSCR_BF1,
    #[doc = "0xcc - SLC_0_TX_ERREOF_DES_ADDR"]
    pub _0_tx_erreof_des_addr: _0_TX_ERREOF_DES_ADDR,
    #[doc = "0xd0 - SLC_1_TX_ERREOF_DES_ADDR"]
    pub _1_tx_erreof_des_addr: _1_TX_ERREOF_DES_ADDR,
    #[doc = "0xd4 - SLC_TOKEN_LAT"]
    pub token_lat: TOKEN_LAT,
    #[doc = "0xd8 - SLC_TX_DSCR_CONF"]
    pub tx_dscr_conf: TX_DSCR_CONF,
    #[doc = "0xdc - SLC_CMD_INFOR0"]
    pub cmd_infor0: CMD_INFOR0,
    #[doc = "0xe0 - SLC_CMD_INFOR1"]
    pub cmd_infor1: CMD_INFOR1,
    #[doc = "0xe4 - SLC_0_LEN_CONF"]
    pub _0_len_conf: _0_LEN_CONF,
    #[doc = "0xe8 - SLC_0_LENGTH"]
    pub _0_length: _0_LENGTH,
    #[doc = "0xec - SLC_0_TXPKT_H_DSCR"]
    pub _0_txpkt_h_dscr: _0_TXPKT_H_DSCR,
    #[doc = "0xf0 - SLC_0_TXPKT_E_DSCR"]
    pub _0_txpkt_e_dscr: _0_TXPKT_E_DSCR,
    #[doc = "0xf4 - SLC_0_RXPKT_H_DSCR"]
    pub _0_rxpkt_h_dscr: _0_RXPKT_H_DSCR,
    #[doc = "0xf8 - SLC_0_RXPKT_E_DSCR"]
    pub _0_rxpkt_e_dscr: _0_RXPKT_E_DSCR,
    #[doc = "0xfc - SLC_0_TXPKTU_H_DSCR"]
    pub _0_txpktu_h_dscr: _0_TXPKTU_H_DSCR,
    #[doc = "0x100 - SLC_0_TXPKTU_E_DSCR"]
    pub _0_txpktu_e_dscr: _0_TXPKTU_E_DSCR,
    #[doc = "0x104 - SLC_0_RXPKTU_H_DSCR"]
    pub _0_rxpktu_h_dscr: _0_RXPKTU_H_DSCR,
    #[doc = "0x108 - SLC_0_RXPKTU_E_DSCR"]
    pub _0_rxpktu_e_dscr: _0_RXPKTU_E_DSCR,
    _reserved67: [u8; 8usize],
    #[doc = "0x114 - SLC_SEQ_POSITION"]
    pub seq_position: SEQ_POSITION,
    #[doc = "0x118 - SLC_0_DSCR_REC_CONF"]
    pub _0_dscr_rec_conf: _0_DSCR_REC_CONF,
    #[doc = "0x11c - SLC_SDIO_CRC_ST0"]
    pub sdio_crc_st0: SDIO_CRC_ST0,
    #[doc = "0x120 - SLC_SDIO_CRC_ST1"]
    pub sdio_crc_st1: SDIO_CRC_ST1,
    #[doc = "0x124 - SLC_0_EOF_START_DES"]
    pub _0_eof_start_des: _0_EOF_START_DES,
    #[doc = "0x128 - SLC_0_PUSH_DSCR_ADDR"]
    pub _0_push_dscr_addr: _0_PUSH_DSCR_ADDR,
    #[doc = "0x12c - SLC_0_DONE_DSCR_ADDR"]
    pub _0_done_dscr_addr: _0_DONE_DSCR_ADDR,
    #[doc = "0x130 - SLC_0_SUB_START_DES"]
    pub _0_sub_start_des: _0_SUB_START_DES,
    #[doc = "0x134 - SLC_0_DSCR_CNT"]
    pub _0_dscr_cnt: _0_DSCR_CNT,
    #[doc = "0x138 - SLC_0_LEN_LIM_CONF"]
    pub _0_len_lim_conf: _0_LEN_LIM_CONF,
    #[doc = "0x13c - SLC_0INT_ST1"]
    pub _0int_st1: _0INT_ST1,
    #[doc = "0x140 - SLC_0INT_ENA1"]
    pub _0int_ena1: _0INT_ENA1,
    #[doc = "0x144 - SLC_1INT_ST1"]
    pub _1int_st1: _1INT_ST1,
    #[doc = "0x148 - SLC_1INT_ENA1"]
    pub _1int_ena1: _1INT_ENA1,
    _reserved81: [u8; 172usize],
    #[doc = "0x1f8 - SLC_DATE"]
    pub date: DATE,
    #[doc = "0x1fc - SLC_ID"]
    pub id: ID,
}
#[doc = "SLC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf0](conf0) module"]
pub type CONF0 = crate::Reg<u32, _CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF0;
#[doc = "`read()` method returns [conf0::R](conf0::R) reader structure"]
impl crate::Readable for CONF0 {}
#[doc = "`write(|w| ..)` method takes [conf0::W](conf0::W) writer structure"]
impl crate::Writable for CONF0 {}
#[doc = "SLC_CONF0"]
pub mod conf0;
#[doc = "SLC_0INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0int_raw](_0int_raw) module"]
pub type _0INT_RAW = crate::Reg<u32, __0INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0INT_RAW;
#[doc = "`read()` method returns [_0int_raw::R](_0int_raw::R) reader structure"]
impl crate::Readable for _0INT_RAW {}
#[doc = "`write(|w| ..)` method takes [_0int_raw::W](_0int_raw::W) writer structure"]
impl crate::Writable for _0INT_RAW {}
#[doc = "SLC_0INT_RAW"]
pub mod _0int_raw;
#[doc = "SLC_0INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0int_st](_0int_st) module"]
pub type _0INT_ST = crate::Reg<u32, __0INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0INT_ST;
#[doc = "`read()` method returns [_0int_st::R](_0int_st::R) reader structure"]
impl crate::Readable for _0INT_ST {}
#[doc = "`write(|w| ..)` method takes [_0int_st::W](_0int_st::W) writer structure"]
impl crate::Writable for _0INT_ST {}
#[doc = "SLC_0INT_ST"]
pub mod _0int_st;
#[doc = "SLC_0INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0int_ena](_0int_ena) module"]
pub type _0INT_ENA = crate::Reg<u32, __0INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0INT_ENA;
#[doc = "`read()` method returns [_0int_ena::R](_0int_ena::R) reader structure"]
impl crate::Readable for _0INT_ENA {}
#[doc = "`write(|w| ..)` method takes [_0int_ena::W](_0int_ena::W) writer structure"]
impl crate::Writable for _0INT_ENA {}
#[doc = "SLC_0INT_ENA"]
pub mod _0int_ena;
#[doc = "SLC_0INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0int_clr](_0int_clr) module"]
pub type _0INT_CLR = crate::Reg<u32, __0INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0INT_CLR;
#[doc = "`read()` method returns [_0int_clr::R](_0int_clr::R) reader structure"]
impl crate::Readable for _0INT_CLR {}
#[doc = "`write(|w| ..)` method takes [_0int_clr::W](_0int_clr::W) writer structure"]
impl crate::Writable for _0INT_CLR {}
#[doc = "SLC_0INT_CLR"]
pub mod _0int_clr;
#[doc = "SLC_1INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1int_raw](_1int_raw) module"]
pub type _1INT_RAW = crate::Reg<u32, __1INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1INT_RAW;
#[doc = "`read()` method returns [_1int_raw::R](_1int_raw::R) reader structure"]
impl crate::Readable for _1INT_RAW {}
#[doc = "`write(|w| ..)` method takes [_1int_raw::W](_1int_raw::W) writer structure"]
impl crate::Writable for _1INT_RAW {}
#[doc = "SLC_1INT_RAW"]
pub mod _1int_raw;
#[doc = "SLC_1INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1int_st](_1int_st) module"]
pub type _1INT_ST = crate::Reg<u32, __1INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1INT_ST;
#[doc = "`read()` method returns [_1int_st::R](_1int_st::R) reader structure"]
impl crate::Readable for _1INT_ST {}
#[doc = "`write(|w| ..)` method takes [_1int_st::W](_1int_st::W) writer structure"]
impl crate::Writable for _1INT_ST {}
#[doc = "SLC_1INT_ST"]
pub mod _1int_st;
#[doc = "SLC_1INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1int_ena](_1int_ena) module"]
pub type _1INT_ENA = crate::Reg<u32, __1INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1INT_ENA;
#[doc = "`read()` method returns [_1int_ena::R](_1int_ena::R) reader structure"]
impl crate::Readable for _1INT_ENA {}
#[doc = "`write(|w| ..)` method takes [_1int_ena::W](_1int_ena::W) writer structure"]
impl crate::Writable for _1INT_ENA {}
#[doc = "SLC_1INT_ENA"]
pub mod _1int_ena;
#[doc = "SLC_1INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1int_clr](_1int_clr) module"]
pub type _1INT_CLR = crate::Reg<u32, __1INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1INT_CLR;
#[doc = "`read()` method returns [_1int_clr::R](_1int_clr::R) reader structure"]
impl crate::Readable for _1INT_CLR {}
#[doc = "`write(|w| ..)` method takes [_1int_clr::W](_1int_clr::W) writer structure"]
impl crate::Writable for _1INT_CLR {}
#[doc = "SLC_1INT_CLR"]
pub mod _1int_clr;
#[doc = "SLC_RX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_status](rx_status) module"]
pub type RX_STATUS = crate::Reg<u32, _RX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_STATUS;
#[doc = "`read()` method returns [rx_status::R](rx_status::R) reader structure"]
impl crate::Readable for RX_STATUS {}
#[doc = "`write(|w| ..)` method takes [rx_status::W](rx_status::W) writer structure"]
impl crate::Writable for RX_STATUS {}
#[doc = "SLC_RX_STATUS"]
pub mod rx_status;
#[doc = "SLC_0RXFIFO_PUSH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0rxfifo_push](_0rxfifo_push) module"]
pub type _0RXFIFO_PUSH = crate::Reg<u32, __0RXFIFO_PUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0RXFIFO_PUSH;
#[doc = "`read()` method returns [_0rxfifo_push::R](_0rxfifo_push::R) reader structure"]
impl crate::Readable for _0RXFIFO_PUSH {}
#[doc = "`write(|w| ..)` method takes [_0rxfifo_push::W](_0rxfifo_push::W) writer structure"]
impl crate::Writable for _0RXFIFO_PUSH {}
#[doc = "SLC_0RXFIFO_PUSH"]
pub mod _0rxfifo_push;
#[doc = "SLC_1RXFIFO_PUSH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1rxfifo_push](_1rxfifo_push) module"]
pub type _1RXFIFO_PUSH = crate::Reg<u32, __1RXFIFO_PUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1RXFIFO_PUSH;
#[doc = "`read()` method returns [_1rxfifo_push::R](_1rxfifo_push::R) reader structure"]
impl crate::Readable for _1RXFIFO_PUSH {}
#[doc = "`write(|w| ..)` method takes [_1rxfifo_push::W](_1rxfifo_push::W) writer structure"]
impl crate::Writable for _1RXFIFO_PUSH {}
#[doc = "SLC_1RXFIFO_PUSH"]
pub mod _1rxfifo_push;
#[doc = "SLC_TX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_status](tx_status) module"]
pub type TX_STATUS = crate::Reg<u32, _TX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_STATUS;
#[doc = "`read()` method returns [tx_status::R](tx_status::R) reader structure"]
impl crate::Readable for TX_STATUS {}
#[doc = "`write(|w| ..)` method takes [tx_status::W](tx_status::W) writer structure"]
impl crate::Writable for TX_STATUS {}
#[doc = "SLC_TX_STATUS"]
pub mod tx_status;
#[doc = "SLC_0TXFIFO_POP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0txfifo_pop](_0txfifo_pop) module"]
pub type _0TXFIFO_POP = crate::Reg<u32, __0TXFIFO_POP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0TXFIFO_POP;
#[doc = "`read()` method returns [_0txfifo_pop::R](_0txfifo_pop::R) reader structure"]
impl crate::Readable for _0TXFIFO_POP {}
#[doc = "`write(|w| ..)` method takes [_0txfifo_pop::W](_0txfifo_pop::W) writer structure"]
impl crate::Writable for _0TXFIFO_POP {}
#[doc = "SLC_0TXFIFO_POP"]
pub mod _0txfifo_pop;
#[doc = "SLC_1TXFIFO_POP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1txfifo_pop](_1txfifo_pop) module"]
pub type _1TXFIFO_POP = crate::Reg<u32, __1TXFIFO_POP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1TXFIFO_POP;
#[doc = "`read()` method returns [_1txfifo_pop::R](_1txfifo_pop::R) reader structure"]
impl crate::Readable for _1TXFIFO_POP {}
#[doc = "`write(|w| ..)` method takes [_1txfifo_pop::W](_1txfifo_pop::W) writer structure"]
impl crate::Writable for _1TXFIFO_POP {}
#[doc = "SLC_1TXFIFO_POP"]
pub mod _1txfifo_pop;
#[doc = "SLC_0RX_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0rx_link](_0rx_link) module"]
pub type _0RX_LINK = crate::Reg<u32, __0RX_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0RX_LINK;
#[doc = "`read()` method returns [_0rx_link::R](_0rx_link::R) reader structure"]
impl crate::Readable for _0RX_LINK {}
#[doc = "`write(|w| ..)` method takes [_0rx_link::W](_0rx_link::W) writer structure"]
impl crate::Writable for _0RX_LINK {}
#[doc = "SLC_0RX_LINK"]
pub mod _0rx_link;
#[doc = "SLC_0TX_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0tx_link](_0tx_link) module"]
pub type _0TX_LINK = crate::Reg<u32, __0TX_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0TX_LINK;
#[doc = "`read()` method returns [_0tx_link::R](_0tx_link::R) reader structure"]
impl crate::Readable for _0TX_LINK {}
#[doc = "`write(|w| ..)` method takes [_0tx_link::W](_0tx_link::W) writer structure"]
impl crate::Writable for _0TX_LINK {}
#[doc = "SLC_0TX_LINK"]
pub mod _0tx_link;
#[doc = "SLC_1RX_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1rx_link](_1rx_link) module"]
pub type _1RX_LINK = crate::Reg<u32, __1RX_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1RX_LINK;
#[doc = "`read()` method returns [_1rx_link::R](_1rx_link::R) reader structure"]
impl crate::Readable for _1RX_LINK {}
#[doc = "`write(|w| ..)` method takes [_1rx_link::W](_1rx_link::W) writer structure"]
impl crate::Writable for _1RX_LINK {}
#[doc = "SLC_1RX_LINK"]
pub mod _1rx_link;
#[doc = "SLC_1TX_LINK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1tx_link](_1tx_link) module"]
pub type _1TX_LINK = crate::Reg<u32, __1TX_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1TX_LINK;
#[doc = "`read()` method returns [_1tx_link::R](_1tx_link::R) reader structure"]
impl crate::Readable for _1TX_LINK {}
#[doc = "`write(|w| ..)` method takes [_1tx_link::W](_1tx_link::W) writer structure"]
impl crate::Writable for _1TX_LINK {}
#[doc = "SLC_1TX_LINK"]
pub mod _1tx_link;
#[doc = "SLC_INTVEC_TOHOST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intvec_tohost](intvec_tohost) module"]
pub type INTVEC_TOHOST = crate::Reg<u32, _INTVEC_TOHOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTVEC_TOHOST;
#[doc = "`read()` method returns [intvec_tohost::R](intvec_tohost::R) reader structure"]
impl crate::Readable for INTVEC_TOHOST {}
#[doc = "`write(|w| ..)` method takes [intvec_tohost::W](intvec_tohost::W) writer structure"]
impl crate::Writable for INTVEC_TOHOST {}
#[doc = "SLC_INTVEC_TOHOST"]
pub mod intvec_tohost;
#[doc = "SLC_0TOKEN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0token0](_0token0) module"]
pub type _0TOKEN0 = crate::Reg<u32, __0TOKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0TOKEN0;
#[doc = "`read()` method returns [_0token0::R](_0token0::R) reader structure"]
impl crate::Readable for _0TOKEN0 {}
#[doc = "`write(|w| ..)` method takes [_0token0::W](_0token0::W) writer structure"]
impl crate::Writable for _0TOKEN0 {}
#[doc = "SLC_0TOKEN0"]
pub mod _0token0;
#[doc = "SLC_0TOKEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0token1](_0token1) module"]
pub type _0TOKEN1 = crate::Reg<u32, __0TOKEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0TOKEN1;
#[doc = "`read()` method returns [_0token1::R](_0token1::R) reader structure"]
impl crate::Readable for _0TOKEN1 {}
#[doc = "`write(|w| ..)` method takes [_0token1::W](_0token1::W) writer structure"]
impl crate::Writable for _0TOKEN1 {}
#[doc = "SLC_0TOKEN1"]
pub mod _0token1;
#[doc = "SLC_1TOKEN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1token0](_1token0) module"]
pub type _1TOKEN0 = crate::Reg<u32, __1TOKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1TOKEN0;
#[doc = "`read()` method returns [_1token0::R](_1token0::R) reader structure"]
impl crate::Readable for _1TOKEN0 {}
#[doc = "`write(|w| ..)` method takes [_1token0::W](_1token0::W) writer structure"]
impl crate::Writable for _1TOKEN0 {}
#[doc = "SLC_1TOKEN0"]
pub mod _1token0;
#[doc = "SLC_1TOKEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1token1](_1token1) module"]
pub type _1TOKEN1 = crate::Reg<u32, __1TOKEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1TOKEN1;
#[doc = "`read()` method returns [_1token1::R](_1token1::R) reader structure"]
impl crate::Readable for _1TOKEN1 {}
#[doc = "`write(|w| ..)` method takes [_1token1::W](_1token1::W) writer structure"]
impl crate::Writable for _1TOKEN1 {}
#[doc = "SLC_1TOKEN1"]
pub mod _1token1;
#[doc = "SLC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf1](conf1) module"]
pub type CONF1 = crate::Reg<u32, _CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF1;
#[doc = "`read()` method returns [conf1::R](conf1::R) reader structure"]
impl crate::Readable for CONF1 {}
#[doc = "`write(|w| ..)` method takes [conf1::W](conf1::W) writer structure"]
impl crate::Writable for CONF1 {}
#[doc = "SLC_CONF1"]
pub mod conf1;
#[doc = "SLC_0_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_state0](_0_state0) module"]
pub type _0_STATE0 = crate::Reg<u32, __0_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_STATE0;
#[doc = "`read()` method returns [_0_state0::R](_0_state0::R) reader structure"]
impl crate::Readable for _0_STATE0 {}
#[doc = "`write(|w| ..)` method takes [_0_state0::W](_0_state0::W) writer structure"]
impl crate::Writable for _0_STATE0 {}
#[doc = "SLC_0_STATE0"]
pub mod _0_state0;
#[doc = "SLC_0_STATE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_state1](_0_state1) module"]
pub type _0_STATE1 = crate::Reg<u32, __0_STATE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_STATE1;
#[doc = "`read()` method returns [_0_state1::R](_0_state1::R) reader structure"]
impl crate::Readable for _0_STATE1 {}
#[doc = "`write(|w| ..)` method takes [_0_state1::W](_0_state1::W) writer structure"]
impl crate::Writable for _0_STATE1 {}
#[doc = "SLC_0_STATE1"]
pub mod _0_state1;
#[doc = "SLC_1_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_state0](_1_state0) module"]
pub type _1_STATE0 = crate::Reg<u32, __1_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_STATE0;
#[doc = "`read()` method returns [_1_state0::R](_1_state0::R) reader structure"]
impl crate::Readable for _1_STATE0 {}
#[doc = "`write(|w| ..)` method takes [_1_state0::W](_1_state0::W) writer structure"]
impl crate::Writable for _1_STATE0 {}
#[doc = "SLC_1_STATE0"]
pub mod _1_state0;
#[doc = "SLC_1_STATE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_state1](_1_state1) module"]
pub type _1_STATE1 = crate::Reg<u32, __1_STATE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_STATE1;
#[doc = "`read()` method returns [_1_state1::R](_1_state1::R) reader structure"]
impl crate::Readable for _1_STATE1 {}
#[doc = "`write(|w| ..)` method takes [_1_state1::W](_1_state1::W) writer structure"]
impl crate::Writable for _1_STATE1 {}
#[doc = "SLC_1_STATE1"]
pub mod _1_state1;
#[doc = "SLC_BRIDGE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bridge_conf](bridge_conf) module"]
pub type BRIDGE_CONF = crate::Reg<u32, _BRIDGE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRIDGE_CONF;
#[doc = "`read()` method returns [bridge_conf::R](bridge_conf::R) reader structure"]
impl crate::Readable for BRIDGE_CONF {}
#[doc = "`write(|w| ..)` method takes [bridge_conf::W](bridge_conf::W) writer structure"]
impl crate::Writable for BRIDGE_CONF {}
#[doc = "SLC_BRIDGE_CONF"]
pub mod bridge_conf;
#[doc = "SLC_0_TO_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_to_eof_des_addr](_0_to_eof_des_addr) module"]
pub type _0_TO_EOF_DES_ADDR = crate::Reg<u32, __0_TO_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TO_EOF_DES_ADDR;
#[doc = "`read()` method returns [_0_to_eof_des_addr::R](_0_to_eof_des_addr::R) reader structure"]
impl crate::Readable for _0_TO_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_0_to_eof_des_addr::W](_0_to_eof_des_addr::W) writer structure"]
impl crate::Writable for _0_TO_EOF_DES_ADDR {}
#[doc = "SLC_0_TO_EOF_DES_ADDR"]
pub mod _0_to_eof_des_addr;
#[doc = "SLC_0_TX_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_tx_eof_des_addr](_0_tx_eof_des_addr) module"]
pub type _0_TX_EOF_DES_ADDR = crate::Reg<u32, __0_TX_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TX_EOF_DES_ADDR;
#[doc = "`read()` method returns [_0_tx_eof_des_addr::R](_0_tx_eof_des_addr::R) reader structure"]
impl crate::Readable for _0_TX_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_0_tx_eof_des_addr::W](_0_tx_eof_des_addr::W) writer structure"]
impl crate::Writable for _0_TX_EOF_DES_ADDR {}
#[doc = "SLC_0_TX_EOF_DES_ADDR"]
pub mod _0_tx_eof_des_addr;
#[doc = "SLC_0_TO_EOF_BFR_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_to_eof_bfr_des_addr](_0_to_eof_bfr_des_addr) module"]
pub type _0_TO_EOF_BFR_DES_ADDR = crate::Reg<u32, __0_TO_EOF_BFR_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TO_EOF_BFR_DES_ADDR;
#[doc = "`read()` method returns [_0_to_eof_bfr_des_addr::R](_0_to_eof_bfr_des_addr::R) reader structure"]
impl crate::Readable for _0_TO_EOF_BFR_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_0_to_eof_bfr_des_addr::W](_0_to_eof_bfr_des_addr::W) writer structure"]
impl crate::Writable for _0_TO_EOF_BFR_DES_ADDR {}
#[doc = "SLC_0_TO_EOF_BFR_DES_ADDR"]
pub mod _0_to_eof_bfr_des_addr;
#[doc = "SLC_1_TO_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_to_eof_des_addr](_1_to_eof_des_addr) module"]
pub type _1_TO_EOF_DES_ADDR = crate::Reg<u32, __1_TO_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TO_EOF_DES_ADDR;
#[doc = "`read()` method returns [_1_to_eof_des_addr::R](_1_to_eof_des_addr::R) reader structure"]
impl crate::Readable for _1_TO_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_1_to_eof_des_addr::W](_1_to_eof_des_addr::W) writer structure"]
impl crate::Writable for _1_TO_EOF_DES_ADDR {}
#[doc = "SLC_1_TO_EOF_DES_ADDR"]
pub mod _1_to_eof_des_addr;
#[doc = "SLC_1_TX_EOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_tx_eof_des_addr](_1_tx_eof_des_addr) module"]
pub type _1_TX_EOF_DES_ADDR = crate::Reg<u32, __1_TX_EOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TX_EOF_DES_ADDR;
#[doc = "`read()` method returns [_1_tx_eof_des_addr::R](_1_tx_eof_des_addr::R) reader structure"]
impl crate::Readable for _1_TX_EOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_1_tx_eof_des_addr::W](_1_tx_eof_des_addr::W) writer structure"]
impl crate::Writable for _1_TX_EOF_DES_ADDR {}
#[doc = "SLC_1_TX_EOF_DES_ADDR"]
pub mod _1_tx_eof_des_addr;
#[doc = "SLC_1_TO_EOF_BFR_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_to_eof_bfr_des_addr](_1_to_eof_bfr_des_addr) module"]
pub type _1_TO_EOF_BFR_DES_ADDR = crate::Reg<u32, __1_TO_EOF_BFR_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TO_EOF_BFR_DES_ADDR;
#[doc = "`read()` method returns [_1_to_eof_bfr_des_addr::R](_1_to_eof_bfr_des_addr::R) reader structure"]
impl crate::Readable for _1_TO_EOF_BFR_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_1_to_eof_bfr_des_addr::W](_1_to_eof_bfr_des_addr::W) writer structure"]
impl crate::Writable for _1_TO_EOF_BFR_DES_ADDR {}
#[doc = "SLC_1_TO_EOF_BFR_DES_ADDR"]
pub mod _1_to_eof_bfr_des_addr;
#[doc = "SLC_AHB_TEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_test](ahb_test) module"]
pub type AHB_TEST = crate::Reg<u32, _AHB_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_TEST;
#[doc = "`read()` method returns [ahb_test::R](ahb_test::R) reader structure"]
impl crate::Readable for AHB_TEST {}
#[doc = "`write(|w| ..)` method takes [ahb_test::W](ahb_test::W) writer structure"]
impl crate::Writable for AHB_TEST {}
#[doc = "SLC_AHB_TEST"]
pub mod ahb_test;
#[doc = "SLC_SDIO_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_st](sdio_st) module"]
pub type SDIO_ST = crate::Reg<u32, _SDIO_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_ST;
#[doc = "`read()` method returns [sdio_st::R](sdio_st::R) reader structure"]
impl crate::Readable for SDIO_ST {}
#[doc = "`write(|w| ..)` method takes [sdio_st::W](sdio_st::W) writer structure"]
impl crate::Writable for SDIO_ST {}
#[doc = "SLC_SDIO_ST"]
pub mod sdio_st;
#[doc = "SLC_RX_DSCR_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_dscr_conf](rx_dscr_conf) module"]
pub type RX_DSCR_CONF = crate::Reg<u32, _RX_DSCR_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DSCR_CONF;
#[doc = "`read()` method returns [rx_dscr_conf::R](rx_dscr_conf::R) reader structure"]
impl crate::Readable for RX_DSCR_CONF {}
#[doc = "`write(|w| ..)` method takes [rx_dscr_conf::W](rx_dscr_conf::W) writer structure"]
impl crate::Writable for RX_DSCR_CONF {}
#[doc = "SLC_RX_DSCR_CONF"]
pub mod rx_dscr_conf;
#[doc = "SLC_0_TXLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txlink_dscr](_0_txlink_dscr) module"]
pub type _0_TXLINK_DSCR = crate::Reg<u32, __0_TXLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXLINK_DSCR;
#[doc = "`read()` method returns [_0_txlink_dscr::R](_0_txlink_dscr::R) reader structure"]
impl crate::Readable for _0_TXLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_txlink_dscr::W](_0_txlink_dscr::W) writer structure"]
impl crate::Writable for _0_TXLINK_DSCR {}
#[doc = "SLC_0_TXLINK_DSCR"]
pub mod _0_txlink_dscr;
#[doc = "SLC_0_TXLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txlink_dscr_bf0](_0_txlink_dscr_bf0) module"]
pub type _0_TXLINK_DSCR_BF0 = crate::Reg<u32, __0_TXLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXLINK_DSCR_BF0;
#[doc = "`read()` method returns [_0_txlink_dscr_bf0::R](_0_txlink_dscr_bf0::R) reader structure"]
impl crate::Readable for _0_TXLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [_0_txlink_dscr_bf0::W](_0_txlink_dscr_bf0::W) writer structure"]
impl crate::Writable for _0_TXLINK_DSCR_BF0 {}
#[doc = "SLC_0_TXLINK_DSCR_BF0"]
pub mod _0_txlink_dscr_bf0;
#[doc = "SLC_0_TXLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txlink_dscr_bf1](_0_txlink_dscr_bf1) module"]
pub type _0_TXLINK_DSCR_BF1 = crate::Reg<u32, __0_TXLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXLINK_DSCR_BF1;
#[doc = "`read()` method returns [_0_txlink_dscr_bf1::R](_0_txlink_dscr_bf1::R) reader structure"]
impl crate::Readable for _0_TXLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [_0_txlink_dscr_bf1::W](_0_txlink_dscr_bf1::W) writer structure"]
impl crate::Writable for _0_TXLINK_DSCR_BF1 {}
#[doc = "SLC_0_TXLINK_DSCR_BF1"]
pub mod _0_txlink_dscr_bf1;
#[doc = "SLC_0_RXLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxlink_dscr](_0_rxlink_dscr) module"]
pub type _0_RXLINK_DSCR = crate::Reg<u32, __0_RXLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXLINK_DSCR;
#[doc = "`read()` method returns [_0_rxlink_dscr::R](_0_rxlink_dscr::R) reader structure"]
impl crate::Readable for _0_RXLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_rxlink_dscr::W](_0_rxlink_dscr::W) writer structure"]
impl crate::Writable for _0_RXLINK_DSCR {}
#[doc = "SLC_0_RXLINK_DSCR"]
pub mod _0_rxlink_dscr;
#[doc = "SLC_0_RXLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxlink_dscr_bf0](_0_rxlink_dscr_bf0) module"]
pub type _0_RXLINK_DSCR_BF0 = crate::Reg<u32, __0_RXLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXLINK_DSCR_BF0;
#[doc = "`read()` method returns [_0_rxlink_dscr_bf0::R](_0_rxlink_dscr_bf0::R) reader structure"]
impl crate::Readable for _0_RXLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [_0_rxlink_dscr_bf0::W](_0_rxlink_dscr_bf0::W) writer structure"]
impl crate::Writable for _0_RXLINK_DSCR_BF0 {}
#[doc = "SLC_0_RXLINK_DSCR_BF0"]
pub mod _0_rxlink_dscr_bf0;
#[doc = "SLC_0_RXLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxlink_dscr_bf1](_0_rxlink_dscr_bf1) module"]
pub type _0_RXLINK_DSCR_BF1 = crate::Reg<u32, __0_RXLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXLINK_DSCR_BF1;
#[doc = "`read()` method returns [_0_rxlink_dscr_bf1::R](_0_rxlink_dscr_bf1::R) reader structure"]
impl crate::Readable for _0_RXLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [_0_rxlink_dscr_bf1::W](_0_rxlink_dscr_bf1::W) writer structure"]
impl crate::Writable for _0_RXLINK_DSCR_BF1 {}
#[doc = "SLC_0_RXLINK_DSCR_BF1"]
pub mod _0_rxlink_dscr_bf1;
#[doc = "SLC_1_TXLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_txlink_dscr](_1_txlink_dscr) module"]
pub type _1_TXLINK_DSCR = crate::Reg<u32, __1_TXLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TXLINK_DSCR;
#[doc = "`read()` method returns [_1_txlink_dscr::R](_1_txlink_dscr::R) reader structure"]
impl crate::Readable for _1_TXLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [_1_txlink_dscr::W](_1_txlink_dscr::W) writer structure"]
impl crate::Writable for _1_TXLINK_DSCR {}
#[doc = "SLC_1_TXLINK_DSCR"]
pub mod _1_txlink_dscr;
#[doc = "SLC_1_TXLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_txlink_dscr_bf0](_1_txlink_dscr_bf0) module"]
pub type _1_TXLINK_DSCR_BF0 = crate::Reg<u32, __1_TXLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TXLINK_DSCR_BF0;
#[doc = "`read()` method returns [_1_txlink_dscr_bf0::R](_1_txlink_dscr_bf0::R) reader structure"]
impl crate::Readable for _1_TXLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [_1_txlink_dscr_bf0::W](_1_txlink_dscr_bf0::W) writer structure"]
impl crate::Writable for _1_TXLINK_DSCR_BF0 {}
#[doc = "SLC_1_TXLINK_DSCR_BF0"]
pub mod _1_txlink_dscr_bf0;
#[doc = "SLC_1_TXLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_txlink_dscr_bf1](_1_txlink_dscr_bf1) module"]
pub type _1_TXLINK_DSCR_BF1 = crate::Reg<u32, __1_TXLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TXLINK_DSCR_BF1;
#[doc = "`read()` method returns [_1_txlink_dscr_bf1::R](_1_txlink_dscr_bf1::R) reader structure"]
impl crate::Readable for _1_TXLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [_1_txlink_dscr_bf1::W](_1_txlink_dscr_bf1::W) writer structure"]
impl crate::Writable for _1_TXLINK_DSCR_BF1 {}
#[doc = "SLC_1_TXLINK_DSCR_BF1"]
pub mod _1_txlink_dscr_bf1;
#[doc = "SLC_1_RXLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_rxlink_dscr](_1_rxlink_dscr) module"]
pub type _1_RXLINK_DSCR = crate::Reg<u32, __1_RXLINK_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_RXLINK_DSCR;
#[doc = "`read()` method returns [_1_rxlink_dscr::R](_1_rxlink_dscr::R) reader structure"]
impl crate::Readable for _1_RXLINK_DSCR {}
#[doc = "`write(|w| ..)` method takes [_1_rxlink_dscr::W](_1_rxlink_dscr::W) writer structure"]
impl crate::Writable for _1_RXLINK_DSCR {}
#[doc = "SLC_1_RXLINK_DSCR"]
pub mod _1_rxlink_dscr;
#[doc = "SLC_1_RXLINK_DSCR_BF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_rxlink_dscr_bf0](_1_rxlink_dscr_bf0) module"]
pub type _1_RXLINK_DSCR_BF0 = crate::Reg<u32, __1_RXLINK_DSCR_BF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_RXLINK_DSCR_BF0;
#[doc = "`read()` method returns [_1_rxlink_dscr_bf0::R](_1_rxlink_dscr_bf0::R) reader structure"]
impl crate::Readable for _1_RXLINK_DSCR_BF0 {}
#[doc = "`write(|w| ..)` method takes [_1_rxlink_dscr_bf0::W](_1_rxlink_dscr_bf0::W) writer structure"]
impl crate::Writable for _1_RXLINK_DSCR_BF0 {}
#[doc = "SLC_1_RXLINK_DSCR_BF0"]
pub mod _1_rxlink_dscr_bf0;
#[doc = "SLC_1_RXLINK_DSCR_BF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_rxlink_dscr_bf1](_1_rxlink_dscr_bf1) module"]
pub type _1_RXLINK_DSCR_BF1 = crate::Reg<u32, __1_RXLINK_DSCR_BF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_RXLINK_DSCR_BF1;
#[doc = "`read()` method returns [_1_rxlink_dscr_bf1::R](_1_rxlink_dscr_bf1::R) reader structure"]
impl crate::Readable for _1_RXLINK_DSCR_BF1 {}
#[doc = "`write(|w| ..)` method takes [_1_rxlink_dscr_bf1::W](_1_rxlink_dscr_bf1::W) writer structure"]
impl crate::Writable for _1_RXLINK_DSCR_BF1 {}
#[doc = "SLC_1_RXLINK_DSCR_BF1"]
pub mod _1_rxlink_dscr_bf1;
#[doc = "SLC_0_TX_ERREOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_tx_erreof_des_addr](_0_tx_erreof_des_addr) module"]
pub type _0_TX_ERREOF_DES_ADDR = crate::Reg<u32, __0_TX_ERREOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TX_ERREOF_DES_ADDR;
#[doc = "`read()` method returns [_0_tx_erreof_des_addr::R](_0_tx_erreof_des_addr::R) reader structure"]
impl crate::Readable for _0_TX_ERREOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_0_tx_erreof_des_addr::W](_0_tx_erreof_des_addr::W) writer structure"]
impl crate::Writable for _0_TX_ERREOF_DES_ADDR {}
#[doc = "SLC_0_TX_ERREOF_DES_ADDR"]
pub mod _0_tx_erreof_des_addr;
#[doc = "SLC_1_TX_ERREOF_DES_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1_tx_erreof_des_addr](_1_tx_erreof_des_addr) module"]
pub type _1_TX_ERREOF_DES_ADDR = crate::Reg<u32, __1_TX_ERREOF_DES_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_TX_ERREOF_DES_ADDR;
#[doc = "`read()` method returns [_1_tx_erreof_des_addr::R](_1_tx_erreof_des_addr::R) reader structure"]
impl crate::Readable for _1_TX_ERREOF_DES_ADDR {}
#[doc = "`write(|w| ..)` method takes [_1_tx_erreof_des_addr::W](_1_tx_erreof_des_addr::W) writer structure"]
impl crate::Writable for _1_TX_ERREOF_DES_ADDR {}
#[doc = "SLC_1_TX_ERREOF_DES_ADDR"]
pub mod _1_tx_erreof_des_addr;
#[doc = "SLC_TOKEN_LAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [token_lat](token_lat) module"]
pub type TOKEN_LAT = crate::Reg<u32, _TOKEN_LAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOKEN_LAT;
#[doc = "`read()` method returns [token_lat::R](token_lat::R) reader structure"]
impl crate::Readable for TOKEN_LAT {}
#[doc = "`write(|w| ..)` method takes [token_lat::W](token_lat::W) writer structure"]
impl crate::Writable for TOKEN_LAT {}
#[doc = "SLC_TOKEN_LAT"]
pub mod token_lat;
#[doc = "SLC_TX_DSCR_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_dscr_conf](tx_dscr_conf) module"]
pub type TX_DSCR_CONF = crate::Reg<u32, _TX_DSCR_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DSCR_CONF;
#[doc = "`read()` method returns [tx_dscr_conf::R](tx_dscr_conf::R) reader structure"]
impl crate::Readable for TX_DSCR_CONF {}
#[doc = "`write(|w| ..)` method takes [tx_dscr_conf::W](tx_dscr_conf::W) writer structure"]
impl crate::Writable for TX_DSCR_CONF {}
#[doc = "SLC_TX_DSCR_CONF"]
pub mod tx_dscr_conf;
#[doc = "SLC_CMD_INFOR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd_infor0](cmd_infor0) module"]
pub type CMD_INFOR0 = crate::Reg<u32, _CMD_INFOR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_INFOR0;
#[doc = "`read()` method returns [cmd_infor0::R](cmd_infor0::R) reader structure"]
impl crate::Readable for CMD_INFOR0 {}
#[doc = "`write(|w| ..)` method takes [cmd_infor0::W](cmd_infor0::W) writer structure"]
impl crate::Writable for CMD_INFOR0 {}
#[doc = "SLC_CMD_INFOR0"]
pub mod cmd_infor0;
#[doc = "SLC_CMD_INFOR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd_infor1](cmd_infor1) module"]
pub type CMD_INFOR1 = crate::Reg<u32, _CMD_INFOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_INFOR1;
#[doc = "`read()` method returns [cmd_infor1::R](cmd_infor1::R) reader structure"]
impl crate::Readable for CMD_INFOR1 {}
#[doc = "`write(|w| ..)` method takes [cmd_infor1::W](cmd_infor1::W) writer structure"]
impl crate::Writable for CMD_INFOR1 {}
#[doc = "SLC_CMD_INFOR1"]
pub mod cmd_infor1;
#[doc = "SLC_0_LEN_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_len_conf](_0_len_conf) module"]
pub type _0_LEN_CONF = crate::Reg<u32, __0_LEN_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_LEN_CONF;
#[doc = "`read()` method returns [_0_len_conf::R](_0_len_conf::R) reader structure"]
impl crate::Readable for _0_LEN_CONF {}
#[doc = "`write(|w| ..)` method takes [_0_len_conf::W](_0_len_conf::W) writer structure"]
impl crate::Writable for _0_LEN_CONF {}
#[doc = "SLC_0_LEN_CONF"]
pub mod _0_len_conf;
#[doc = "SLC_0_LENGTH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_length](_0_length) module"]
pub type _0_LENGTH = crate::Reg<u32, __0_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_LENGTH;
#[doc = "`read()` method returns [_0_length::R](_0_length::R) reader structure"]
impl crate::Readable for _0_LENGTH {}
#[doc = "`write(|w| ..)` method takes [_0_length::W](_0_length::W) writer structure"]
impl crate::Writable for _0_LENGTH {}
#[doc = "SLC_0_LENGTH"]
pub mod _0_length;
#[doc = "SLC_0_TXPKT_H_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txpkt_h_dscr](_0_txpkt_h_dscr) module"]
pub type _0_TXPKT_H_DSCR = crate::Reg<u32, __0_TXPKT_H_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXPKT_H_DSCR;
#[doc = "`read()` method returns [_0_txpkt_h_dscr::R](_0_txpkt_h_dscr::R) reader structure"]
impl crate::Readable for _0_TXPKT_H_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_txpkt_h_dscr::W](_0_txpkt_h_dscr::W) writer structure"]
impl crate::Writable for _0_TXPKT_H_DSCR {}
#[doc = "SLC_0_TXPKT_H_DSCR"]
pub mod _0_txpkt_h_dscr;
#[doc = "SLC_0_TXPKT_E_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txpkt_e_dscr](_0_txpkt_e_dscr) module"]
pub type _0_TXPKT_E_DSCR = crate::Reg<u32, __0_TXPKT_E_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXPKT_E_DSCR;
#[doc = "`read()` method returns [_0_txpkt_e_dscr::R](_0_txpkt_e_dscr::R) reader structure"]
impl crate::Readable for _0_TXPKT_E_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_txpkt_e_dscr::W](_0_txpkt_e_dscr::W) writer structure"]
impl crate::Writable for _0_TXPKT_E_DSCR {}
#[doc = "SLC_0_TXPKT_E_DSCR"]
pub mod _0_txpkt_e_dscr;
#[doc = "SLC_0_RXPKT_H_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxpkt_h_dscr](_0_rxpkt_h_dscr) module"]
pub type _0_RXPKT_H_DSCR = crate::Reg<u32, __0_RXPKT_H_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXPKT_H_DSCR;
#[doc = "`read()` method returns [_0_rxpkt_h_dscr::R](_0_rxpkt_h_dscr::R) reader structure"]
impl crate::Readable for _0_RXPKT_H_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_rxpkt_h_dscr::W](_0_rxpkt_h_dscr::W) writer structure"]
impl crate::Writable for _0_RXPKT_H_DSCR {}
#[doc = "SLC_0_RXPKT_H_DSCR"]
pub mod _0_rxpkt_h_dscr;
#[doc = "SLC_0_RXPKT_E_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxpkt_e_dscr](_0_rxpkt_e_dscr) module"]
pub type _0_RXPKT_E_DSCR = crate::Reg<u32, __0_RXPKT_E_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXPKT_E_DSCR;
#[doc = "`read()` method returns [_0_rxpkt_e_dscr::R](_0_rxpkt_e_dscr::R) reader structure"]
impl crate::Readable for _0_RXPKT_E_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_rxpkt_e_dscr::W](_0_rxpkt_e_dscr::W) writer structure"]
impl crate::Writable for _0_RXPKT_E_DSCR {}
#[doc = "SLC_0_RXPKT_E_DSCR"]
pub mod _0_rxpkt_e_dscr;
#[doc = "SLC_0_TXPKTU_H_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txpktu_h_dscr](_0_txpktu_h_dscr) module"]
pub type _0_TXPKTU_H_DSCR = crate::Reg<u32, __0_TXPKTU_H_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXPKTU_H_DSCR;
#[doc = "`read()` method returns [_0_txpktu_h_dscr::R](_0_txpktu_h_dscr::R) reader structure"]
impl crate::Readable for _0_TXPKTU_H_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_txpktu_h_dscr::W](_0_txpktu_h_dscr::W) writer structure"]
impl crate::Writable for _0_TXPKTU_H_DSCR {}
#[doc = "SLC_0_TXPKTU_H_DSCR"]
pub mod _0_txpktu_h_dscr;
#[doc = "SLC_0_TXPKTU_E_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_txpktu_e_dscr](_0_txpktu_e_dscr) module"]
pub type _0_TXPKTU_E_DSCR = crate::Reg<u32, __0_TXPKTU_E_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_TXPKTU_E_DSCR;
#[doc = "`read()` method returns [_0_txpktu_e_dscr::R](_0_txpktu_e_dscr::R) reader structure"]
impl crate::Readable for _0_TXPKTU_E_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_txpktu_e_dscr::W](_0_txpktu_e_dscr::W) writer structure"]
impl crate::Writable for _0_TXPKTU_E_DSCR {}
#[doc = "SLC_0_TXPKTU_E_DSCR"]
pub mod _0_txpktu_e_dscr;
#[doc = "SLC_0_RXPKTU_H_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxpktu_h_dscr](_0_rxpktu_h_dscr) module"]
pub type _0_RXPKTU_H_DSCR = crate::Reg<u32, __0_RXPKTU_H_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXPKTU_H_DSCR;
#[doc = "`read()` method returns [_0_rxpktu_h_dscr::R](_0_rxpktu_h_dscr::R) reader structure"]
impl crate::Readable for _0_RXPKTU_H_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_rxpktu_h_dscr::W](_0_rxpktu_h_dscr::W) writer structure"]
impl crate::Writable for _0_RXPKTU_H_DSCR {}
#[doc = "SLC_0_RXPKTU_H_DSCR"]
pub mod _0_rxpktu_h_dscr;
#[doc = "SLC_0_RXPKTU_E_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_rxpktu_e_dscr](_0_rxpktu_e_dscr) module"]
pub type _0_RXPKTU_E_DSCR = crate::Reg<u32, __0_RXPKTU_E_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RXPKTU_E_DSCR;
#[doc = "`read()` method returns [_0_rxpktu_e_dscr::R](_0_rxpktu_e_dscr::R) reader structure"]
impl crate::Readable for _0_RXPKTU_E_DSCR {}
#[doc = "`write(|w| ..)` method takes [_0_rxpktu_e_dscr::W](_0_rxpktu_e_dscr::W) writer structure"]
impl crate::Writable for _0_RXPKTU_E_DSCR {}
#[doc = "SLC_0_RXPKTU_E_DSCR"]
pub mod _0_rxpktu_e_dscr;
#[doc = "SLC_SEQ_POSITION\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seq_position](seq_position) module"]
pub type SEQ_POSITION = crate::Reg<u32, _SEQ_POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_POSITION;
#[doc = "`read()` method returns [seq_position::R](seq_position::R) reader structure"]
impl crate::Readable for SEQ_POSITION {}
#[doc = "`write(|w| ..)` method takes [seq_position::W](seq_position::W) writer structure"]
impl crate::Writable for SEQ_POSITION {}
#[doc = "SLC_SEQ_POSITION"]
pub mod seq_position;
#[doc = "SLC_0_DSCR_REC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_dscr_rec_conf](_0_dscr_rec_conf) module"]
pub type _0_DSCR_REC_CONF = crate::Reg<u32, __0_DSCR_REC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_DSCR_REC_CONF;
#[doc = "`read()` method returns [_0_dscr_rec_conf::R](_0_dscr_rec_conf::R) reader structure"]
impl crate::Readable for _0_DSCR_REC_CONF {}
#[doc = "`write(|w| ..)` method takes [_0_dscr_rec_conf::W](_0_dscr_rec_conf::W) writer structure"]
impl crate::Writable for _0_DSCR_REC_CONF {}
#[doc = "SLC_0_DSCR_REC_CONF"]
pub mod _0_dscr_rec_conf;
#[doc = "SLC_SDIO_CRC_ST0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_crc_st0](sdio_crc_st0) module"]
pub type SDIO_CRC_ST0 = crate::Reg<u32, _SDIO_CRC_ST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_CRC_ST0;
#[doc = "`read()` method returns [sdio_crc_st0::R](sdio_crc_st0::R) reader structure"]
impl crate::Readable for SDIO_CRC_ST0 {}
#[doc = "`write(|w| ..)` method takes [sdio_crc_st0::W](sdio_crc_st0::W) writer structure"]
impl crate::Writable for SDIO_CRC_ST0 {}
#[doc = "SLC_SDIO_CRC_ST0"]
pub mod sdio_crc_st0;
#[doc = "SLC_SDIO_CRC_ST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_crc_st1](sdio_crc_st1) module"]
pub type SDIO_CRC_ST1 = crate::Reg<u32, _SDIO_CRC_ST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_CRC_ST1;
#[doc = "`read()` method returns [sdio_crc_st1::R](sdio_crc_st1::R) reader structure"]
impl crate::Readable for SDIO_CRC_ST1 {}
#[doc = "`write(|w| ..)` method takes [sdio_crc_st1::W](sdio_crc_st1::W) writer structure"]
impl crate::Writable for SDIO_CRC_ST1 {}
#[doc = "SLC_SDIO_CRC_ST1"]
pub mod sdio_crc_st1;
#[doc = "SLC_0_EOF_START_DES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_eof_start_des](_0_eof_start_des) module"]
pub type _0_EOF_START_DES = crate::Reg<u32, __0_EOF_START_DES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_EOF_START_DES;
#[doc = "`read()` method returns [_0_eof_start_des::R](_0_eof_start_des::R) reader structure"]
impl crate::Readable for _0_EOF_START_DES {}
#[doc = "`write(|w| ..)` method takes [_0_eof_start_des::W](_0_eof_start_des::W) writer structure"]
impl crate::Writable for _0_EOF_START_DES {}
#[doc = "SLC_0_EOF_START_DES"]
pub mod _0_eof_start_des;
#[doc = "SLC_0_PUSH_DSCR_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_push_dscr_addr](_0_push_dscr_addr) module"]
pub type _0_PUSH_DSCR_ADDR = crate::Reg<u32, __0_PUSH_DSCR_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_PUSH_DSCR_ADDR;
#[doc = "`read()` method returns [_0_push_dscr_addr::R](_0_push_dscr_addr::R) reader structure"]
impl crate::Readable for _0_PUSH_DSCR_ADDR {}
#[doc = "`write(|w| ..)` method takes [_0_push_dscr_addr::W](_0_push_dscr_addr::W) writer structure"]
impl crate::Writable for _0_PUSH_DSCR_ADDR {}
#[doc = "SLC_0_PUSH_DSCR_ADDR"]
pub mod _0_push_dscr_addr;
#[doc = "SLC_0_DONE_DSCR_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_done_dscr_addr](_0_done_dscr_addr) module"]
pub type _0_DONE_DSCR_ADDR = crate::Reg<u32, __0_DONE_DSCR_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_DONE_DSCR_ADDR;
#[doc = "`read()` method returns [_0_done_dscr_addr::R](_0_done_dscr_addr::R) reader structure"]
impl crate::Readable for _0_DONE_DSCR_ADDR {}
#[doc = "`write(|w| ..)` method takes [_0_done_dscr_addr::W](_0_done_dscr_addr::W) writer structure"]
impl crate::Writable for _0_DONE_DSCR_ADDR {}
#[doc = "SLC_0_DONE_DSCR_ADDR"]
pub mod _0_done_dscr_addr;
#[doc = "SLC_0_SUB_START_DES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_sub_start_des](_0_sub_start_des) module"]
pub type _0_SUB_START_DES = crate::Reg<u32, __0_SUB_START_DES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_SUB_START_DES;
#[doc = "`read()` method returns [_0_sub_start_des::R](_0_sub_start_des::R) reader structure"]
impl crate::Readable for _0_SUB_START_DES {}
#[doc = "`write(|w| ..)` method takes [_0_sub_start_des::W](_0_sub_start_des::W) writer structure"]
impl crate::Writable for _0_SUB_START_DES {}
#[doc = "SLC_0_SUB_START_DES"]
pub mod _0_sub_start_des;
#[doc = "SLC_0_DSCR_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_dscr_cnt](_0_dscr_cnt) module"]
pub type _0_DSCR_CNT = crate::Reg<u32, __0_DSCR_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_DSCR_CNT;
#[doc = "`read()` method returns [_0_dscr_cnt::R](_0_dscr_cnt::R) reader structure"]
impl crate::Readable for _0_DSCR_CNT {}
#[doc = "`write(|w| ..)` method takes [_0_dscr_cnt::W](_0_dscr_cnt::W) writer structure"]
impl crate::Writable for _0_DSCR_CNT {}
#[doc = "SLC_0_DSCR_CNT"]
pub mod _0_dscr_cnt;
#[doc = "SLC_0_LEN_LIM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0_len_lim_conf](_0_len_lim_conf) module"]
pub type _0_LEN_LIM_CONF = crate::Reg<u32, __0_LEN_LIM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_LEN_LIM_CONF;
#[doc = "`read()` method returns [_0_len_lim_conf::R](_0_len_lim_conf::R) reader structure"]
impl crate::Readable for _0_LEN_LIM_CONF {}
#[doc = "`write(|w| ..)` method takes [_0_len_lim_conf::W](_0_len_lim_conf::W) writer structure"]
impl crate::Writable for _0_LEN_LIM_CONF {}
#[doc = "SLC_0_LEN_LIM_CONF"]
pub mod _0_len_lim_conf;
#[doc = "SLC_0INT_ST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0int_st1](_0int_st1) module"]
pub type _0INT_ST1 = crate::Reg<u32, __0INT_ST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0INT_ST1;
#[doc = "`read()` method returns [_0int_st1::R](_0int_st1::R) reader structure"]
impl crate::Readable for _0INT_ST1 {}
#[doc = "`write(|w| ..)` method takes [_0int_st1::W](_0int_st1::W) writer structure"]
impl crate::Writable for _0INT_ST1 {}
#[doc = "SLC_0INT_ST1"]
pub mod _0int_st1;
#[doc = "SLC_0INT_ENA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_0int_ena1](_0int_ena1) module"]
pub type _0INT_ENA1 = crate::Reg<u32, __0INT_ENA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0INT_ENA1;
#[doc = "`read()` method returns [_0int_ena1::R](_0int_ena1::R) reader structure"]
impl crate::Readable for _0INT_ENA1 {}
#[doc = "`write(|w| ..)` method takes [_0int_ena1::W](_0int_ena1::W) writer structure"]
impl crate::Writable for _0INT_ENA1 {}
#[doc = "SLC_0INT_ENA1"]
pub mod _0int_ena1;
#[doc = "SLC_1INT_ST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1int_st1](_1int_st1) module"]
pub type _1INT_ST1 = crate::Reg<u32, __1INT_ST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1INT_ST1;
#[doc = "`read()` method returns [_1int_st1::R](_1int_st1::R) reader structure"]
impl crate::Readable for _1INT_ST1 {}
#[doc = "`write(|w| ..)` method takes [_1int_st1::W](_1int_st1::W) writer structure"]
impl crate::Writable for _1INT_ST1 {}
#[doc = "SLC_1INT_ST1"]
pub mod _1int_st1;
#[doc = "SLC_1INT_ENA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_1int_ena1](_1int_ena1) module"]
pub type _1INT_ENA1 = crate::Reg<u32, __1INT_ENA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1INT_ENA1;
#[doc = "`read()` method returns [_1int_ena1::R](_1int_ena1::R) reader structure"]
impl crate::Readable for _1INT_ENA1 {}
#[doc = "`write(|w| ..)` method takes [_1int_ena1::W](_1int_ena1::W) writer structure"]
impl crate::Writable for _1INT_ENA1 {}
#[doc = "SLC_1INT_ENA1"]
pub mod _1int_ena1;
#[doc = "SLC_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "SLC_DATE"]
pub mod date;
#[doc = "SLC_ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "`write(|w| ..)` method takes [id::W](id::W) writer structure"]
impl crate::Writable for ID {}
#[doc = "SLC_ID"]
pub mod id;
