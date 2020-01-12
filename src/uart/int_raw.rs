#[doc = "Reader of register INT_RAW"]
pub type R = crate::R<u32, super::INT_RAW>;
#[doc = "Writer for register INT_RAW"]
pub type W = crate::W<u32, super::INT_RAW>;
#[doc = "Register INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AT_CMD_CHAR_DET_INT_RAW`"]
pub type AT_CMD_CHAR_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AT_CMD_CHAR_DET_INT_RAW`"]
pub struct AT_CMD_CHAR_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_DET_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RS485_CLASH_INT_RAW`"]
pub type RS485_CLASH_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_CLASH_INT_RAW`"]
pub struct RS485_CLASH_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_CLASH_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RS485_FRM_ERR_INT_RAW`"]
pub type RS485_FRM_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_FRM_ERR_INT_RAW`"]
pub struct RS485_FRM_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_FRM_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RS485_PARITY_ERR_INT_RAW`"]
pub type RS485_PARITY_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_PARITY_ERR_INT_RAW`"]
pub struct RS485_PARITY_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_PARITY_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TX_DONE_INT_RAW`"]
pub type TX_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DONE_INT_RAW`"]
pub struct TX_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TX_BRK_IDLE_DONE_INT_RAW`"]
pub type TX_BRK_IDLE_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BRK_IDLE_DONE_INT_RAW`"]
pub struct TX_BRK_IDLE_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_IDLE_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TX_BRK_DONE_INT_RAW`"]
pub type TX_BRK_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BRK_DONE_INT_RAW`"]
pub struct TX_BRK_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `GLITCH_DET_INT_RAW`"]
pub type GLITCH_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLITCH_DET_INT_RAW`"]
pub struct GLITCH_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SW_XOFF_INT_RAW`"]
pub type SW_XOFF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_XOFF_INT_RAW`"]
pub struct SW_XOFF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XOFF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SW_XON_INT_RAW`"]
pub type SW_XON_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_XON_INT_RAW`"]
pub struct SW_XON_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XON_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RXFIFO_TOUT_INT_RAW`"]
pub type RXFIFO_TOUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_TOUT_INT_RAW`"]
pub struct RXFIFO_TOUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TOUT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `BRK_DET_INT_RAW`"]
pub type BRK_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRK_DET_INT_RAW`"]
pub struct BRK_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_DET_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CTS_CHG_INT_RAW`"]
pub type CTS_CHG_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS_CHG_INT_RAW`"]
pub struct CTS_CHG_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHG_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DSR_CHG_INT_RAW`"]
pub type DSR_CHG_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSR_CHG_INT_RAW`"]
pub struct DSR_CHG_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_CHG_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RXFIFO_OVF_INT_RAW`"]
pub type RXFIFO_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_OVF_INT_RAW`"]
pub struct RXFIFO_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `FRM_ERR_INT_RAW`"]
pub type FRM_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRM_ERR_INT_RAW`"]
pub struct FRM_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `PARITY_ERR_INT_RAW`"]
pub type PARITY_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY_ERR_INT_RAW`"]
pub struct PARITY_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TXFIFO_EMPTY_INT_RAW`"]
pub type TXFIFO_EMPTY_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_EMPTY_INT_RAW`"]
pub struct TXFIFO_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RXFIFO_FULL_INT_RAW`"]
pub type RXFIFO_FULL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_FULL_INT_RAW`"]
pub struct RXFIFO_FULL_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_RAW_W<'a> {
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
    pub fn at_cmd_char_det_int_raw(&self) -> AT_CMD_CHAR_DET_INT_RAW_R {
        AT_CMD_CHAR_DET_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn rs485_clash_int_raw(&self) -> RS485_CLASH_INT_RAW_R {
        RS485_CLASH_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn rs485_frm_err_int_raw(&self) -> RS485_FRM_ERR_INT_RAW_R {
        RS485_FRM_ERR_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn rs485_parity_err_int_raw(&self) -> RS485_PARITY_ERR_INT_RAW_R {
        RS485_PARITY_ERR_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> TX_DONE_INT_RAW_R {
        TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&self) -> TX_BRK_IDLE_DONE_INT_RAW_R {
        TX_BRK_IDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&self) -> TX_BRK_DONE_INT_RAW_R {
        TX_BRK_DONE_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&self) -> SW_XOFF_INT_RAW_R {
        SW_XOFF_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon_int_raw(&self) -> SW_XON_INT_RAW_R {
        SW_XON_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> RXFIFO_TOUT_INT_RAW_R {
        RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> BRK_DET_INT_RAW_R {
        BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> CTS_CHG_INT_RAW_R {
        CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> DSR_CHG_INT_RAW_R {
        DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> FRM_ERR_INT_RAW_R {
        FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> PARITY_ERR_INT_RAW_R {
        PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&mut self) -> AT_CMD_CHAR_DET_INT_RAW_W {
        AT_CMD_CHAR_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn rs485_clash_int_raw(&mut self) -> RS485_CLASH_INT_RAW_W {
        RS485_CLASH_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn rs485_frm_err_int_raw(&mut self) -> RS485_FRM_ERR_INT_RAW_W {
        RS485_FRM_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn rs485_parity_err_int_raw(&mut self) -> RS485_PARITY_ERR_INT_RAW_W {
        RS485_PARITY_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn tx_done_int_raw(&mut self) -> TX_DONE_INT_RAW_W {
        TX_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&mut self) -> TX_BRK_IDLE_DONE_INT_RAW_W {
        TX_BRK_IDLE_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&mut self) -> TX_BRK_DONE_INT_RAW_W {
        TX_BRK_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&mut self) -> GLITCH_DET_INT_RAW_W {
        GLITCH_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&mut self) -> SW_XOFF_INT_RAW_W {
        SW_XOFF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon_int_raw(&mut self) -> SW_XON_INT_RAW_W {
        SW_XON_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&mut self) -> RXFIFO_TOUT_INT_RAW_W {
        RXFIFO_TOUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det_int_raw(&mut self) -> BRK_DET_INT_RAW_W {
        BRK_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn cts_chg_int_raw(&mut self) -> CTS_CHG_INT_RAW_W {
        CTS_CHG_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&mut self) -> DSR_CHG_INT_RAW_W {
        DSR_CHG_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&mut self) -> RXFIFO_OVF_INT_RAW_W {
        RXFIFO_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn frm_err_int_raw(&mut self) -> FRM_ERR_INT_RAW_W {
        FRM_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn parity_err_int_raw(&mut self) -> PARITY_ERR_INT_RAW_W {
        PARITY_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&mut self) -> TXFIFO_EMPTY_INT_RAW_W {
        TXFIFO_EMPTY_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&mut self) -> RXFIFO_FULL_INT_RAW_W {
        RXFIFO_FULL_INT_RAW_W { w: self }
    }
}
