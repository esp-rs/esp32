#[doc = "Reader of register UART_INT_RAW_REG"]
pub type R = crate::R<u32, super::UART_INT_RAW_REG>;
#[doc = "Writer for register UART_INT_RAW_REG"]
pub type W = crate::W<u32, super::UART_INT_RAW_REG>;
#[doc = "Register UART_INT_RAW_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_RAW_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_AT_CMD_CHAR_DET_INT_RAW`"]
pub type UART_AT_CMD_CHAR_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_AT_CMD_CHAR_DET_INT_RAW`"]
pub struct UART_AT_CMD_CHAR_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AT_CMD_CHAR_DET_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `UART_RS485_CLASH_INT_RAW`"]
pub type UART_RS485_CLASH_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_CLASH_INT_RAW`"]
pub struct UART_RS485_CLASH_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_CLASH_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `UART_RS485_FRM_ERR_INT_RAW`"]
pub type UART_RS485_FRM_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_FRM_ERR_INT_RAW`"]
pub struct UART_RS485_FRM_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_FRM_ERR_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART_RS485_PARITY_ERR_INT_RAW`"]
pub type UART_RS485_PARITY_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_PARITY_ERR_INT_RAW`"]
pub struct UART_RS485_PARITY_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_PARITY_ERR_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `UART_TX_DONE_INT_RAW`"]
pub type UART_TX_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_DONE_INT_RAW`"]
pub struct UART_TX_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_DONE_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `UART_TX_BRK_IDLE_DONE_INT_RAW`"]
pub type UART_TX_BRK_IDLE_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_BRK_IDLE_DONE_INT_RAW`"]
pub struct UART_TX_BRK_IDLE_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_IDLE_DONE_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `UART_TX_BRK_DONE_INT_RAW`"]
pub type UART_TX_BRK_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_BRK_DONE_INT_RAW`"]
pub struct UART_TX_BRK_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_DONE_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `UART_GLITCH_DET_INT_RAW`"]
pub type UART_GLITCH_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_GLITCH_DET_INT_RAW`"]
pub struct UART_GLITCH_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_GLITCH_DET_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `UART_SW_XOFF_INT_RAW`"]
pub type UART_SW_XOFF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_XOFF_INT_RAW`"]
pub struct UART_SW_XOFF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_XOFF_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `UART_SW_XON_INT_RAW`"]
pub type UART_SW_XON_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_XON_INT_RAW`"]
pub struct UART_SW_XON_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_XON_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UART_RXFIFO_TOUT_INT_RAW`"]
pub type UART_RXFIFO_TOUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_TOUT_INT_RAW`"]
pub struct UART_RXFIFO_TOUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_TOUT_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART_BRK_DET_INT_RAW`"]
pub type UART_BRK_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_BRK_DET_INT_RAW`"]
pub struct UART_BRK_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_BRK_DET_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `UART_CTS_CHG_INT_RAW`"]
pub type UART_CTS_CHG_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTS_CHG_INT_RAW`"]
pub struct UART_CTS_CHG_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTS_CHG_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UART_DSR_CHG_INT_RAW`"]
pub type UART_DSR_CHG_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DSR_CHG_INT_RAW`"]
pub struct UART_DSR_CHG_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DSR_CHG_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UART_RXFIFO_OVF_INT_RAW`"]
pub type UART_RXFIFO_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_OVF_INT_RAW`"]
pub struct UART_RXFIFO_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_OVF_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UART_FRM_ERR_INT_RAW`"]
pub type UART_FRM_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FRM_ERR_INT_RAW`"]
pub struct UART_FRM_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FRM_ERR_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UART_PARITY_ERR_INT_RAW`"]
pub type UART_PARITY_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_PARITY_ERR_INT_RAW`"]
pub struct UART_PARITY_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PARITY_ERR_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UART_TXFIFO_EMPTY_INT_RAW`"]
pub type UART_TXFIFO_EMPTY_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TXFIFO_EMPTY_INT_RAW`"]
pub struct UART_TXFIFO_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXFIFO_EMPTY_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UART_RXFIFO_FULL_INT_RAW`"]
pub type UART_RXFIFO_FULL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_FULL_INT_RAW`"]
pub struct UART_RXFIFO_FULL_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_FULL_INT_RAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
    #[inline(always)]
    pub fn uart_at_cmd_char_det_int_raw(&self) -> UART_AT_CMD_CHAR_DET_INT_RAW_R {
        UART_AT_CMD_CHAR_DET_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn uart_rs485_clash_int_raw(&self) -> UART_RS485_CLASH_INT_RAW_R {
        UART_RS485_CLASH_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn uart_rs485_frm_err_int_raw(&self) -> UART_RS485_FRM_ERR_INT_RAW_R {
        UART_RS485_FRM_ERR_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn uart_rs485_parity_err_int_raw(&self) -> UART_RS485_PARITY_ERR_INT_RAW_R {
        UART_RS485_PARITY_ERR_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn uart_tx_done_int_raw(&self) -> UART_TX_DONE_INT_RAW_R {
        UART_TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn uart_tx_brk_idle_done_int_raw(&self) -> UART_TX_BRK_IDLE_DONE_INT_RAW_R {
        UART_TX_BRK_IDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn uart_tx_brk_done_int_raw(&self) -> UART_TX_BRK_DONE_INT_RAW_R {
        UART_TX_BRK_DONE_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn uart_glitch_det_int_raw(&self) -> UART_GLITCH_DET_INT_RAW_R {
        UART_GLITCH_DET_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xoff_int_raw(&self) -> UART_SW_XOFF_INT_RAW_R {
        UART_SW_XOFF_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xon_int_raw(&self) -> UART_SW_XON_INT_RAW_R {
        UART_SW_XON_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn uart_rxfifo_tout_int_raw(&self) -> UART_RXFIFO_TOUT_INT_RAW_R {
        UART_RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn uart_brk_det_int_raw(&self) -> UART_BRK_DET_INT_RAW_R {
        UART_BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn uart_cts_chg_int_raw(&self) -> UART_CTS_CHG_INT_RAW_R {
        UART_CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn uart_dsr_chg_int_raw(&self) -> UART_DSR_CHG_INT_RAW_R {
        UART_DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn uart_rxfifo_ovf_int_raw(&self) -> UART_RXFIFO_OVF_INT_RAW_R {
        UART_RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn uart_frm_err_int_raw(&self) -> UART_FRM_ERR_INT_RAW_R {
        UART_FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn uart_parity_err_int_raw(&self) -> UART_PARITY_ERR_INT_RAW_R {
        UART_PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn uart_txfifo_empty_int_raw(&self) -> UART_TXFIFO_EMPTY_INT_RAW_R {
        UART_TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn uart_rxfifo_full_int_raw(&self) -> UART_RXFIFO_FULL_INT_RAW_R {
        UART_RXFIFO_FULL_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
    #[inline(always)]
    pub fn uart_at_cmd_char_det_int_raw(&mut self) -> UART_AT_CMD_CHAR_DET_INT_RAW_W {
        UART_AT_CMD_CHAR_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn uart_rs485_clash_int_raw(&mut self) -> UART_RS485_CLASH_INT_RAW_W {
        UART_RS485_CLASH_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn uart_rs485_frm_err_int_raw(&mut self) -> UART_RS485_FRM_ERR_INT_RAW_W {
        UART_RS485_FRM_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn uart_rs485_parity_err_int_raw(&mut self) -> UART_RS485_PARITY_ERR_INT_RAW_W {
        UART_RS485_PARITY_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn uart_tx_done_int_raw(&mut self) -> UART_TX_DONE_INT_RAW_W {
        UART_TX_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn uart_tx_brk_idle_done_int_raw(&mut self) -> UART_TX_BRK_IDLE_DONE_INT_RAW_W {
        UART_TX_BRK_IDLE_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn uart_tx_brk_done_int_raw(&mut self) -> UART_TX_BRK_DONE_INT_RAW_W {
        UART_TX_BRK_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn uart_glitch_det_int_raw(&mut self) -> UART_GLITCH_DET_INT_RAW_W {
        UART_GLITCH_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xoff_int_raw(&mut self) -> UART_SW_XOFF_INT_RAW_W {
        UART_SW_XOFF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xon_int_raw(&mut self) -> UART_SW_XON_INT_RAW_W {
        UART_SW_XON_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn uart_rxfifo_tout_int_raw(&mut self) -> UART_RXFIFO_TOUT_INT_RAW_W {
        UART_RXFIFO_TOUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn uart_brk_det_int_raw(&mut self) -> UART_BRK_DET_INT_RAW_W {
        UART_BRK_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn uart_cts_chg_int_raw(&mut self) -> UART_CTS_CHG_INT_RAW_W {
        UART_CTS_CHG_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn uart_dsr_chg_int_raw(&mut self) -> UART_DSR_CHG_INT_RAW_W {
        UART_DSR_CHG_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn uart_rxfifo_ovf_int_raw(&mut self) -> UART_RXFIFO_OVF_INT_RAW_W {
        UART_RXFIFO_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn uart_frm_err_int_raw(&mut self) -> UART_FRM_ERR_INT_RAW_W {
        UART_FRM_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn uart_parity_err_int_raw(&mut self) -> UART_PARITY_ERR_INT_RAW_W {
        UART_PARITY_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn uart_txfifo_empty_int_raw(&mut self) -> UART_TXFIFO_EMPTY_INT_RAW_W {
        UART_TXFIFO_EMPTY_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn uart_rxfifo_full_int_raw(&mut self) -> UART_RXFIFO_FULL_INT_RAW_W {
        UART_RXFIFO_FULL_INT_RAW_W { w: self }
    }
}
