#[doc = "Reader of register INT_ENA"]
pub type R = crate::R<u32, super::INT_ENA>;
#[doc = "Writer for register INT_ENA"]
pub type W = crate::W<u32, super::INT_ENA>;
#[doc = "Register INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AT_CMD_CHAR_DET_INT_ENA`"]
pub type AT_CMD_CHAR_DET_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AT_CMD_CHAR_DET_INT_ENA`"]
pub struct AT_CMD_CHAR_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_DET_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RS485_CLASH_INT_ENA`"]
pub type RS485_CLASH_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_CLASH_INT_ENA`"]
pub struct RS485_CLASH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_CLASH_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RS485_FRM_ERR_INT_ENA`"]
pub type RS485_FRM_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_FRM_ERR_INT_ENA`"]
pub struct RS485_FRM_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_FRM_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RS485_PARITY_ERR_INT_ENA`"]
pub type RS485_PARITY_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_PARITY_ERR_INT_ENA`"]
pub struct RS485_PARITY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_PARITY_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TX_DONE_INT_ENA`"]
pub type TX_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DONE_INT_ENA`"]
pub struct TX_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TX_BRK_IDLE_DONE_INT_ENA`"]
pub type TX_BRK_IDLE_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BRK_IDLE_DONE_INT_ENA`"]
pub struct TX_BRK_IDLE_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_IDLE_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TX_BRK_DONE_INT_ENA`"]
pub type TX_BRK_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BRK_DONE_INT_ENA`"]
pub struct TX_BRK_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `GLITCH_DET_INT_ENA`"]
pub type GLITCH_DET_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLITCH_DET_INT_ENA`"]
pub struct GLITCH_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SW_XOFF_INT_ENA`"]
pub type SW_XOFF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_XOFF_INT_ENA`"]
pub struct SW_XOFF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XOFF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SW_XON_INT_ENA`"]
pub type SW_XON_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_XON_INT_ENA`"]
pub struct SW_XON_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XON_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RXFIFO_TOUT_INT_ENA`"]
pub type RXFIFO_TOUT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_TOUT_INT_ENA`"]
pub struct RXFIFO_TOUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TOUT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `BRK_DET_INT_ENA`"]
pub type BRK_DET_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRK_DET_INT_ENA`"]
pub struct BRK_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_DET_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CTS_CHG_INT_ENA`"]
pub type CTS_CHG_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS_CHG_INT_ENA`"]
pub struct CTS_CHG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHG_INT_ENA_W<'a> {
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
#[doc = "Reader of field `DSR_CHG_INT_ENA`"]
pub type DSR_CHG_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSR_CHG_INT_ENA`"]
pub struct DSR_CHG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_CHG_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RXFIFO_OVF_INT_ENA`"]
pub type RXFIFO_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_OVF_INT_ENA`"]
pub struct RXFIFO_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `FRM_ERR_INT_ENA`"]
pub type FRM_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRM_ERR_INT_ENA`"]
pub struct FRM_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `PARITY_ERR_INT_ENA`"]
pub type PARITY_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY_ERR_INT_ENA`"]
pub struct PARITY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TXFIFO_EMPTY_INT_ENA`"]
pub type TXFIFO_EMPTY_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_EMPTY_INT_ENA`"]
pub struct TXFIFO_EMPTY_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RXFIFO_FULL_INT_ENA`"]
pub type RXFIFO_FULL_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_FULL_INT_ENA`"]
pub struct RXFIFO_FULL_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_ENA_W<'a> {
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
    #[doc = "Bit 18 - This is the enable bit for at_cmd_char_det_int_st register."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&self) -> AT_CMD_CHAR_DET_INT_ENA_R {
        AT_CMD_CHAR_DET_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This is the enable bit for rs485_clash_int_st register."]
    #[inline(always)]
    pub fn rs485_clash_int_ena(&self) -> RS485_CLASH_INT_ENA_R {
        RS485_CLASH_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_frm_err_int_ena(&self) -> RS485_FRM_ERR_INT_ENA_R {
        RS485_FRM_ERR_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_parity_err_int_ena(&self) -> RS485_PARITY_ERR_INT_ENA_R {
        RS485_PARITY_ERR_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for tx_done_int_st register."]
    #[inline(always)]
    pub fn tx_done_int_ena(&self) -> TX_DONE_INT_ENA_R {
        TX_DONE_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for tx_brk_idle_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&self) -> TX_BRK_IDLE_DONE_INT_ENA_R {
        TX_BRK_IDLE_DONE_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for tx_brk_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&self) -> TX_BRK_DONE_INT_ENA_R {
        TX_BRK_DONE_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for glitch_det_int_st register."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> GLITCH_DET_INT_ENA_R {
        GLITCH_DET_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This is the enable bit for sw_xoff_int_st register."]
    #[inline(always)]
    pub fn sw_xoff_int_ena(&self) -> SW_XOFF_INT_ENA_R {
        SW_XOFF_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for sw_xon_int_st register."]
    #[inline(always)]
    pub fn sw_xon_int_ena(&self) -> SW_XON_INT_ENA_R {
        SW_XON_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This is the enable bit for rxfifo_tout_int_st register."]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&self) -> RXFIFO_TOUT_INT_ENA_R {
        RXFIFO_TOUT_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This is the enable bit for brk_det_int_st register."]
    #[inline(always)]
    pub fn brk_det_int_ena(&self) -> BRK_DET_INT_ENA_R {
        BRK_DET_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This is the enable bit for cts_chg_int_st register."]
    #[inline(always)]
    pub fn cts_chg_int_ena(&self) -> CTS_CHG_INT_ENA_R {
        CTS_CHG_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the enable bit for dsr_chg_int_st register."]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&self) -> DSR_CHG_INT_ENA_R {
        DSR_CHG_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This is the enable bit for rxfifo_ovf_int_st register."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This is the enable bit for frm_err_int_st register."]
    #[inline(always)]
    pub fn frm_err_int_ena(&self) -> FRM_ERR_INT_ENA_R {
        FRM_ERR_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the enable bit for parity_err_int_st register."]
    #[inline(always)]
    pub fn parity_err_int_ena(&self) -> PARITY_ERR_INT_ENA_R {
        PARITY_ERR_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&self) -> TXFIFO_EMPTY_INT_ENA_R {
        TXFIFO_EMPTY_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&self) -> RXFIFO_FULL_INT_ENA_R {
        RXFIFO_FULL_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - This is the enable bit for at_cmd_char_det_int_st register."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&mut self) -> AT_CMD_CHAR_DET_INT_ENA_W {
        AT_CMD_CHAR_DET_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - This is the enable bit for rs485_clash_int_st register."]
    #[inline(always)]
    pub fn rs485_clash_int_ena(&mut self) -> RS485_CLASH_INT_ENA_W {
        RS485_CLASH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_frm_err_int_ena(&mut self) -> RS485_FRM_ERR_INT_ENA_W {
        RS485_FRM_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_parity_err_int_ena(&mut self) -> RS485_PARITY_ERR_INT_ENA_W {
        RS485_PARITY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - This is the enable bit for tx_done_int_st register."]
    #[inline(always)]
    pub fn tx_done_int_ena(&mut self) -> TX_DONE_INT_ENA_W {
        TX_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - This is the enable bit for tx_brk_idle_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&mut self) -> TX_BRK_IDLE_DONE_INT_ENA_W {
        TX_BRK_IDLE_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - This is the enable bit for tx_brk_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&mut self) -> TX_BRK_DONE_INT_ENA_W {
        TX_BRK_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - This is the enable bit for glitch_det_int_st register."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&mut self) -> GLITCH_DET_INT_ENA_W {
        GLITCH_DET_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - This is the enable bit for sw_xoff_int_st register."]
    #[inline(always)]
    pub fn sw_xoff_int_ena(&mut self) -> SW_XOFF_INT_ENA_W {
        SW_XOFF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - This is the enable bit for sw_xon_int_st register."]
    #[inline(always)]
    pub fn sw_xon_int_ena(&mut self) -> SW_XON_INT_ENA_W {
        SW_XON_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - This is the enable bit for rxfifo_tout_int_st register."]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&mut self) -> RXFIFO_TOUT_INT_ENA_W {
        RXFIFO_TOUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - This is the enable bit for brk_det_int_st register."]
    #[inline(always)]
    pub fn brk_det_int_ena(&mut self) -> BRK_DET_INT_ENA_W {
        BRK_DET_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - This is the enable bit for cts_chg_int_st register."]
    #[inline(always)]
    pub fn cts_chg_int_ena(&mut self) -> CTS_CHG_INT_ENA_W {
        CTS_CHG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - This is the enable bit for dsr_chg_int_st register."]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&mut self) -> DSR_CHG_INT_ENA_W {
        DSR_CHG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - This is the enable bit for rxfifo_ovf_int_st register."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W {
        RXFIFO_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - This is the enable bit for frm_err_int_st register."]
    #[inline(always)]
    pub fn frm_err_int_ena(&mut self) -> FRM_ERR_INT_ENA_W {
        FRM_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - This is the enable bit for parity_err_int_st register."]
    #[inline(always)]
    pub fn parity_err_int_ena(&mut self) -> PARITY_ERR_INT_ENA_W {
        PARITY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&mut self) -> TXFIFO_EMPTY_INT_ENA_W {
        TXFIFO_EMPTY_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&mut self) -> RXFIFO_FULL_INT_ENA_W {
        RXFIFO_FULL_INT_ENA_W { w: self }
    }
}
