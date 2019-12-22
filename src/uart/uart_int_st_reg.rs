#[doc = "Reader of register UART_INT_ST_REG"]
pub type R = crate::R<u32, super::UART_INT_ST_REG>;
#[doc = "Writer for register UART_INT_ST_REG"]
pub type W = crate::W<u32, super::UART_INT_ST_REG>;
#[doc = "Register UART_INT_ST_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_ST_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_AT_CMD_CHAR_DET_INT_ST`"]
pub type UART_AT_CMD_CHAR_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_AT_CMD_CHAR_DET_INT_ST`"]
pub struct UART_AT_CMD_CHAR_DET_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AT_CMD_CHAR_DET_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_RS485_CLASH_INT_ST`"]
pub type UART_RS485_CLASH_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_CLASH_INT_ST`"]
pub struct UART_RS485_CLASH_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_CLASH_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_RS485_FRM_ERR_INT_ST`"]
pub type UART_RS485_FRM_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_FRM_ERR_INT_ST`"]
pub struct UART_RS485_FRM_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_FRM_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_RS485_PARITY_ERR_INT_ST`"]
pub type UART_RS485_PARITY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_PARITY_ERR_INT_ST`"]
pub struct UART_RS485_PARITY_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_PARITY_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_TX_DONE_INT_ST`"]
pub type UART_TX_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_DONE_INT_ST`"]
pub struct UART_TX_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_DONE_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_TX_BRK_IDLE_DONE_INT_ST`"]
pub type UART_TX_BRK_IDLE_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_BRK_IDLE_DONE_INT_ST`"]
pub struct UART_TX_BRK_IDLE_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_IDLE_DONE_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_TX_BRK_DONE_INT_ST`"]
pub type UART_TX_BRK_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_BRK_DONE_INT_ST`"]
pub struct UART_TX_BRK_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_DONE_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_GLITCH_DET_INT_ST`"]
pub type UART_GLITCH_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_GLITCH_DET_INT_ST`"]
pub struct UART_GLITCH_DET_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_GLITCH_DET_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_SW_XOFF_INT_ST`"]
pub type UART_SW_XOFF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_XOFF_INT_ST`"]
pub struct UART_SW_XOFF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_XOFF_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_SW_XON_INT_ST`"]
pub type UART_SW_XON_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_XON_INT_ST`"]
pub struct UART_SW_XON_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_XON_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_RXFIFO_TOUT_INT_ST`"]
pub type UART_RXFIFO_TOUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_TOUT_INT_ST`"]
pub struct UART_RXFIFO_TOUT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_TOUT_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_BRK_DET_INT_ST`"]
pub type UART_BRK_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_BRK_DET_INT_ST`"]
pub struct UART_BRK_DET_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_BRK_DET_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_CTS_CHG_INT_ST`"]
pub type UART_CTS_CHG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTS_CHG_INT_ST`"]
pub struct UART_CTS_CHG_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTS_CHG_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_DSR_CHG_INT_ST`"]
pub type UART_DSR_CHG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DSR_CHG_INT_ST`"]
pub struct UART_DSR_CHG_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DSR_CHG_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_RXFIFO_OVF_INT_ST`"]
pub type UART_RXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_OVF_INT_ST`"]
pub struct UART_RXFIFO_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_OVF_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_FRM_ERR_INT_ST`"]
pub type UART_FRM_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FRM_ERR_INT_ST`"]
pub struct UART_FRM_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FRM_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_PARITY_ERR_INT_ST`"]
pub type UART_PARITY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_PARITY_ERR_INT_ST`"]
pub struct UART_PARITY_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PARITY_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_TXFIFO_EMPTY_INT_ST`"]
pub type UART_TXFIFO_EMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TXFIFO_EMPTY_INT_ST`"]
pub struct UART_TXFIFO_EMPTY_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXFIFO_EMPTY_INT_ST_W<'a> {
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
#[doc = "Reader of field `UART_RXFIFO_FULL_INT_ST`"]
pub type UART_RXFIFO_FULL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_FULL_INT_ST`"]
pub struct UART_RXFIFO_FULL_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_FULL_INT_ST_W<'a> {
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
    #[doc = "Bit 18 - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_at_cmd_char_det_int_st(&self) -> UART_AT_CMD_CHAR_DET_INT_ST_R {
        UART_AT_CMD_CHAR_DET_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rs485_clash_int_st(&self) -> UART_RS485_CLASH_INT_ST_R {
        UART_RS485_CLASH_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This is the status bit for rs485_fm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rs485_frm_err_int_st(&self) -> UART_RS485_FRM_ERR_INT_ST_R {
        UART_RS485_FRM_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rs485_parity_err_int_st(&self) -> UART_RS485_PARITY_ERR_INT_ST_R {
        UART_RS485_PARITY_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_tx_done_int_st(&self) -> UART_TX_DONE_INT_ST_R {
        UART_TX_DONE_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_tx_brk_idle_done_int_st(&self) -> UART_TX_BRK_IDLE_DONE_INT_ST_R {
        UART_TX_BRK_IDLE_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_tx_brk_done_int_st(&self) -> UART_TX_BRK_DONE_INT_ST_R {
        UART_TX_BRK_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_glitch_det_int_st(&self) -> UART_GLITCH_DET_INT_ST_R {
        UART_GLITCH_DET_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xoff_int_st(&self) -> UART_SW_XOFF_INT_ST_R {
        UART_SW_XOFF_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xon_int_st(&self) -> UART_SW_XON_INT_ST_R {
        UART_SW_XON_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rxfifo_tout_int_st(&self) -> UART_RXFIFO_TOUT_INT_ST_R {
        UART_RXFIFO_TOUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_brk_det_int_st(&self) -> UART_BRK_DET_INT_ST_R {
        UART_BRK_DET_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_cts_chg_int_st(&self) -> UART_CTS_CHG_INT_ST_R {
        UART_CTS_CHG_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_dsr_chg_int_st(&self) -> UART_DSR_CHG_INT_ST_R {
        UART_DSR_CHG_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rxfifo_ovf_int_st(&self) -> UART_RXFIFO_OVF_INT_ST_R {
        UART_RXFIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This is the status bit for frm_err_int_raw when fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_frm_err_int_st(&self) -> UART_FRM_ERR_INT_ST_R {
        UART_FRM_ERR_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_parity_err_int_st(&self) -> UART_PARITY_ERR_INT_ST_R {
        UART_PARITY_ERR_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_txfifo_empty_int_st(&self) -> UART_TXFIFO_EMPTY_INT_ST_R {
        UART_TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rxfifo_full_int_st(&self) -> UART_RXFIFO_FULL_INT_ST_R {
        UART_RXFIFO_FULL_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_at_cmd_char_det_int_st(&mut self) -> UART_AT_CMD_CHAR_DET_INT_ST_W {
        UART_AT_CMD_CHAR_DET_INT_ST_W { w: self }
    }
    #[doc = "Bit 17 - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rs485_clash_int_st(&mut self) -> UART_RS485_CLASH_INT_ST_W {
        UART_RS485_CLASH_INT_ST_W { w: self }
    }
    #[doc = "Bit 16 - This is the status bit for rs485_fm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rs485_frm_err_int_st(&mut self) -> UART_RS485_FRM_ERR_INT_ST_W {
        UART_RS485_FRM_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 15 - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rs485_parity_err_int_st(&mut self) -> UART_RS485_PARITY_ERR_INT_ST_W {
        UART_RS485_PARITY_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 14 - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_tx_done_int_st(&mut self) -> UART_TX_DONE_INT_ST_W {
        UART_TX_DONE_INT_ST_W { w: self }
    }
    #[doc = "Bit 13 - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_tx_brk_idle_done_int_st(&mut self) -> UART_TX_BRK_IDLE_DONE_INT_ST_W {
        UART_TX_BRK_IDLE_DONE_INT_ST_W { w: self }
    }
    #[doc = "Bit 12 - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_tx_brk_done_int_st(&mut self) -> UART_TX_BRK_DONE_INT_ST_W {
        UART_TX_BRK_DONE_INT_ST_W { w: self }
    }
    #[doc = "Bit 11 - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_glitch_det_int_st(&mut self) -> UART_GLITCH_DET_INT_ST_W {
        UART_GLITCH_DET_INT_ST_W { w: self }
    }
    #[doc = "Bit 10 - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xoff_int_st(&mut self) -> UART_SW_XOFF_INT_ST_W {
        UART_SW_XOFF_INT_ST_W { w: self }
    }
    #[doc = "Bit 9 - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_sw_xon_int_st(&mut self) -> UART_SW_XON_INT_ST_W {
        UART_SW_XON_INT_ST_W { w: self }
    }
    #[doc = "Bit 8 - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rxfifo_tout_int_st(&mut self) -> UART_RXFIFO_TOUT_INT_ST_W {
        UART_RXFIFO_TOUT_INT_ST_W { w: self }
    }
    #[doc = "Bit 7 - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_brk_det_int_st(&mut self) -> UART_BRK_DET_INT_ST_W {
        UART_BRK_DET_INT_ST_W { w: self }
    }
    #[doc = "Bit 6 - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_cts_chg_int_st(&mut self) -> UART_CTS_CHG_INT_ST_W {
        UART_CTS_CHG_INT_ST_W { w: self }
    }
    #[doc = "Bit 5 - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_dsr_chg_int_st(&mut self) -> UART_DSR_CHG_INT_ST_W {
        UART_DSR_CHG_INT_ST_W { w: self }
    }
    #[doc = "Bit 4 - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rxfifo_ovf_int_st(&mut self) -> UART_RXFIFO_OVF_INT_ST_W {
        UART_RXFIFO_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 3 - This is the status bit for frm_err_int_raw when fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_frm_err_int_st(&mut self) -> UART_FRM_ERR_INT_ST_W {
        UART_FRM_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_parity_err_int_st(&mut self) -> UART_PARITY_ERR_INT_ST_W {
        UART_PARITY_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_txfifo_empty_int_st(&mut self) -> UART_TXFIFO_EMPTY_INT_ST_W {
        UART_TXFIFO_EMPTY_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
    #[inline(always)]
    pub fn uart_rxfifo_full_int_st(&mut self) -> UART_RXFIFO_FULL_INT_ST_W {
        UART_RXFIFO_FULL_INT_ST_W { w: self }
    }
}
