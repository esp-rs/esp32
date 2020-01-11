#[doc = "Reader of register INT_CLR"]
pub type R = crate::R<u32, super::INT_CLR>;
#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_DMA_INFIFO_FULL_WM_INT_CLR`"]
pub type UHCI_DMA_INFIFO_FULL_WM_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_DMA_INFIFO_FULL_WM_INT_CLR`"]
pub struct UHCI_DMA_INFIFO_FULL_WM_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_DMA_INFIFO_FULL_WM_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_SEND_A_Q_INT_CLR`"]
pub type UHCI_SEND_A_Q_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SEND_A_Q_INT_CLR`"]
pub struct UHCI_SEND_A_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEND_A_Q_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_SEND_S_Q_INT_CLR`"]
pub type UHCI_SEND_S_Q_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SEND_S_Q_INT_CLR`"]
pub struct UHCI_SEND_S_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEND_S_Q_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_OUT_TOTAL_EOF_INT_CLR`"]
pub type UHCI_OUT_TOTAL_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_TOTAL_EOF_INT_CLR`"]
pub struct UHCI_OUT_TOTAL_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_TOTAL_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_OUTLINK_EOF_ERR_INT_CLR`"]
pub type UHCI_OUTLINK_EOF_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUTLINK_EOF_ERR_INT_CLR`"]
pub struct UHCI_OUTLINK_EOF_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_EOF_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_IN_DSCR_EMPTY_INT_CLR`"]
pub type UHCI_IN_DSCR_EMPTY_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_DSCR_EMPTY_INT_CLR`"]
pub struct UHCI_IN_DSCR_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_DSCR_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_OUT_DSCR_ERR_INT_CLR`"]
pub type UHCI_OUT_DSCR_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_DSCR_ERR_INT_CLR`"]
pub struct UHCI_OUT_DSCR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_DSCR_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_IN_DSCR_ERR_INT_CLR`"]
pub type UHCI_IN_DSCR_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_DSCR_ERR_INT_CLR`"]
pub struct UHCI_IN_DSCR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_DSCR_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_OUT_EOF_INT_CLR`"]
pub type UHCI_OUT_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_EOF_INT_CLR`"]
pub struct UHCI_OUT_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_OUT_DONE_INT_CLR`"]
pub type UHCI_OUT_DONE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_DONE_INT_CLR`"]
pub struct UHCI_OUT_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_DONE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_IN_ERR_EOF_INT_CLR`"]
pub type UHCI_IN_ERR_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_ERR_EOF_INT_CLR`"]
pub struct UHCI_IN_ERR_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_ERR_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_IN_SUC_EOF_INT_CLR`"]
pub type UHCI_IN_SUC_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_SUC_EOF_INT_CLR`"]
pub struct UHCI_IN_SUC_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_SUC_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_IN_DONE_INT_CLR`"]
pub type UHCI_IN_DONE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_DONE_INT_CLR`"]
pub struct UHCI_IN_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_DONE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_TX_HUNG_INT_CLR`"]
pub type UHCI_TX_HUNG_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_HUNG_INT_CLR`"]
pub struct UHCI_TX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_RX_HUNG_INT_CLR`"]
pub type UHCI_RX_HUNG_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_HUNG_INT_CLR`"]
pub struct UHCI_RX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_TX_START_INT_CLR`"]
pub type UHCI_TX_START_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_START_INT_CLR`"]
pub struct UHCI_TX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_START_INT_CLR_W<'a> {
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
#[doc = "Reader of field `UHCI_RX_START_INT_CLR`"]
pub type UHCI_RX_START_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_START_INT_CLR`"]
pub struct UHCI_RX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_START_INT_CLR_W<'a> {
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uhci_dma_infifo_full_wm_int_clr(&self) -> UHCI_DMA_INFIFO_FULL_WM_INT_CLR_R {
        UHCI_DMA_INFIFO_FULL_WM_INT_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uhci_send_a_q_int_clr(&self) -> UHCI_SEND_A_Q_INT_CLR_R {
        UHCI_SEND_A_Q_INT_CLR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uhci_send_s_q_int_clr(&self) -> UHCI_SEND_S_Q_INT_CLR_R {
        UHCI_SEND_S_Q_INT_CLR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uhci_out_total_eof_int_clr(&self) -> UHCI_OUT_TOTAL_EOF_INT_CLR_R {
        UHCI_OUT_TOTAL_EOF_INT_CLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci_outlink_eof_err_int_clr(&self) -> UHCI_OUTLINK_EOF_ERR_INT_CLR_R {
        UHCI_OUTLINK_EOF_ERR_INT_CLR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uhci_in_dscr_empty_int_clr(&self) -> UHCI_IN_DSCR_EMPTY_INT_CLR_R {
        UHCI_IN_DSCR_EMPTY_INT_CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhci_out_dscr_err_int_clr(&self) -> UHCI_OUT_DSCR_ERR_INT_CLR_R {
        UHCI_OUT_DSCR_ERR_INT_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uhci_in_dscr_err_int_clr(&self) -> UHCI_IN_DSCR_ERR_INT_CLR_R {
        UHCI_IN_DSCR_ERR_INT_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_out_eof_int_clr(&self) -> UHCI_OUT_EOF_INT_CLR_R {
        UHCI_OUT_EOF_INT_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_out_done_int_clr(&self) -> UHCI_OUT_DONE_INT_CLR_R {
        UHCI_OUT_DONE_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_in_err_eof_int_clr(&self) -> UHCI_IN_ERR_EOF_INT_CLR_R {
        UHCI_IN_ERR_EOF_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_in_suc_eof_int_clr(&self) -> UHCI_IN_SUC_EOF_INT_CLR_R {
        UHCI_IN_SUC_EOF_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_in_done_int_clr(&self) -> UHCI_IN_DONE_INT_CLR_R {
        UHCI_IN_DONE_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_tx_hung_int_clr(&self) -> UHCI_TX_HUNG_INT_CLR_R {
        UHCI_TX_HUNG_INT_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_rx_hung_int_clr(&self) -> UHCI_RX_HUNG_INT_CLR_R {
        UHCI_RX_HUNG_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_tx_start_int_clr(&self) -> UHCI_TX_START_INT_CLR_R {
        UHCI_TX_START_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_rx_start_int_clr(&self) -> UHCI_RX_START_INT_CLR_R {
        UHCI_RX_START_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uhci_dma_infifo_full_wm_int_clr(&mut self) -> UHCI_DMA_INFIFO_FULL_WM_INT_CLR_W {
        UHCI_DMA_INFIFO_FULL_WM_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uhci_send_a_q_int_clr(&mut self) -> UHCI_SEND_A_Q_INT_CLR_W {
        UHCI_SEND_A_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uhci_send_s_q_int_clr(&mut self) -> UHCI_SEND_S_Q_INT_CLR_W {
        UHCI_SEND_S_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uhci_out_total_eof_int_clr(&mut self) -> UHCI_OUT_TOTAL_EOF_INT_CLR_W {
        UHCI_OUT_TOTAL_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci_outlink_eof_err_int_clr(&mut self) -> UHCI_OUTLINK_EOF_ERR_INT_CLR_W {
        UHCI_OUTLINK_EOF_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uhci_in_dscr_empty_int_clr(&mut self) -> UHCI_IN_DSCR_EMPTY_INT_CLR_W {
        UHCI_IN_DSCR_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhci_out_dscr_err_int_clr(&mut self) -> UHCI_OUT_DSCR_ERR_INT_CLR_W {
        UHCI_OUT_DSCR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uhci_in_dscr_err_int_clr(&mut self) -> UHCI_IN_DSCR_ERR_INT_CLR_W {
        UHCI_IN_DSCR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_out_eof_int_clr(&mut self) -> UHCI_OUT_EOF_INT_CLR_W {
        UHCI_OUT_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_out_done_int_clr(&mut self) -> UHCI_OUT_DONE_INT_CLR_W {
        UHCI_OUT_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_in_err_eof_int_clr(&mut self) -> UHCI_IN_ERR_EOF_INT_CLR_W {
        UHCI_IN_ERR_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_in_suc_eof_int_clr(&mut self) -> UHCI_IN_SUC_EOF_INT_CLR_W {
        UHCI_IN_SUC_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_in_done_int_clr(&mut self) -> UHCI_IN_DONE_INT_CLR_W {
        UHCI_IN_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_tx_hung_int_clr(&mut self) -> UHCI_TX_HUNG_INT_CLR_W {
        UHCI_TX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_rx_hung_int_clr(&mut self) -> UHCI_RX_HUNG_INT_CLR_W {
        UHCI_RX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_tx_start_int_clr(&mut self) -> UHCI_TX_START_INT_CLR_W {
        UHCI_TX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_rx_start_int_clr(&mut self) -> UHCI_RX_START_INT_CLR_W {
        UHCI_RX_START_INT_CLR_W { w: self }
    }
}
