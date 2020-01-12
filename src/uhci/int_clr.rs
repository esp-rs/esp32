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
#[doc = "Reader of field `DMA_INFIFO_FULL_WM_INT_CLR`"]
pub type DMA_INFIFO_FULL_WM_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INFIFO_FULL_WM_INT_CLR`"]
pub struct DMA_INFIFO_FULL_WM_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_FULL_WM_INT_CLR_W<'a> {
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
#[doc = "Reader of field `SEND_A_Q_INT_CLR`"]
pub type SEND_A_Q_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_A_Q_INT_CLR`"]
pub struct SEND_A_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_A_Q_INT_CLR_W<'a> {
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
#[doc = "Reader of field `SEND_S_Q_INT_CLR`"]
pub type SEND_S_Q_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_S_Q_INT_CLR`"]
pub struct SEND_S_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_S_Q_INT_CLR_W<'a> {
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
#[doc = "Reader of field `OUT_TOTAL_EOF_INT_CLR`"]
pub type OUT_TOTAL_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_TOTAL_EOF_INT_CLR`"]
pub struct OUT_TOTAL_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `OUTLINK_EOF_ERR_INT_CLR`"]
pub type OUTLINK_EOF_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_EOF_ERR_INT_CLR`"]
pub struct OUTLINK_EOF_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_EOF_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `IN_DSCR_EMPTY_INT_CLR`"]
pub type IN_DSCR_EMPTY_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DSCR_EMPTY_INT_CLR`"]
pub struct IN_DSCR_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Reader of field `OUT_DSCR_ERR_INT_CLR`"]
pub type OUT_DSCR_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_DSCR_ERR_INT_CLR`"]
pub struct OUT_DSCR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DSCR_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `IN_DSCR_ERR_INT_CLR`"]
pub type IN_DSCR_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DSCR_ERR_INT_CLR`"]
pub struct IN_DSCR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `OUT_EOF_INT_CLR`"]
pub type OUT_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EOF_INT_CLR`"]
pub struct OUT_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `OUT_DONE_INT_CLR`"]
pub type OUT_DONE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_DONE_INT_CLR`"]
pub struct OUT_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `IN_ERR_EOF_INT_CLR`"]
pub type IN_ERR_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_ERR_EOF_INT_CLR`"]
pub struct IN_ERR_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `IN_SUC_EOF_INT_CLR`"]
pub type IN_SUC_EOF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_SUC_EOF_INT_CLR`"]
pub struct IN_SUC_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `IN_DONE_INT_CLR`"]
pub type IN_DONE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DONE_INT_CLR`"]
pub struct IN_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `TX_HUNG_INT_CLR`"]
pub type TX_HUNG_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_HUNG_INT_CLR`"]
pub struct TX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Reader of field `RX_HUNG_INT_CLR`"]
pub type RX_HUNG_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_HUNG_INT_CLR`"]
pub struct RX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Reader of field `TX_START_INT_CLR`"]
pub type TX_START_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_START_INT_CLR`"]
pub struct TX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_INT_CLR_W<'a> {
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
#[doc = "Reader of field `RX_START_INT_CLR`"]
pub type RX_START_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_START_INT_CLR`"]
pub struct RX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_INT_CLR_W<'a> {
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
    pub fn dma_infifo_full_wm_int_clr(&self) -> DMA_INFIFO_FULL_WM_INT_CLR_R {
        DMA_INFIFO_FULL_WM_INT_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_a_q_int_clr(&self) -> SEND_A_Q_INT_CLR_R {
        SEND_A_Q_INT_CLR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn send_s_q_int_clr(&self) -> SEND_S_Q_INT_CLR_R {
        SEND_S_Q_INT_CLR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&self) -> OUT_TOTAL_EOF_INT_CLR_R {
        OUT_TOTAL_EOF_INT_CLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn outlink_eof_err_int_clr(&self) -> OUTLINK_EOF_ERR_INT_CLR_R {
        OUTLINK_EOF_ERR_INT_CLR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn in_dscr_empty_int_clr(&self) -> IN_DSCR_EMPTY_INT_CLR_R {
        IN_DSCR_EMPTY_INT_CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn out_dscr_err_int_clr(&self) -> OUT_DSCR_ERR_INT_CLR_R {
        OUT_DSCR_ERR_INT_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_dscr_err_int_clr(&self) -> IN_DSCR_ERR_INT_CLR_R {
        IN_DSCR_ERR_INT_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_eof_int_clr(&self) -> OUT_EOF_INT_CLR_R {
        OUT_EOF_INT_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn out_done_int_clr(&self) -> OUT_DONE_INT_CLR_R {
        OUT_DONE_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&self) -> IN_ERR_EOF_INT_CLR_R {
        IN_ERR_EOF_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&self) -> IN_SUC_EOF_INT_CLR_R {
        IN_SUC_EOF_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn in_done_int_clr(&self) -> IN_DONE_INT_CLR_R {
        IN_DONE_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_clr(&self) -> TX_HUNG_INT_CLR_R {
        TX_HUNG_INT_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_clr(&self) -> RX_HUNG_INT_CLR_R {
        RX_HUNG_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_clr(&self) -> TX_START_INT_CLR_R {
        TX_START_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_clr(&self) -> RX_START_INT_CLR_R {
        RX_START_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_infifo_full_wm_int_clr(&mut self) -> DMA_INFIFO_FULL_WM_INT_CLR_W {
        DMA_INFIFO_FULL_WM_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_a_q_int_clr(&mut self) -> SEND_A_Q_INT_CLR_W {
        SEND_A_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn send_s_q_int_clr(&mut self) -> SEND_S_Q_INT_CLR_W {
        SEND_S_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W {
        OUT_TOTAL_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W {
        OUTLINK_EOF_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn in_dscr_empty_int_clr(&mut self) -> IN_DSCR_EMPTY_INT_CLR_W {
        IN_DSCR_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn out_dscr_err_int_clr(&mut self) -> OUT_DSCR_ERR_INT_CLR_W {
        OUT_DSCR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_dscr_err_int_clr(&mut self) -> IN_DSCR_ERR_INT_CLR_W {
        IN_DSCR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W {
        OUT_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W {
        OUT_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W {
        IN_ERR_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W {
        IN_SUC_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W {
        IN_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W {
        TX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W {
        RX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W {
        TX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W {
        RX_START_INT_CLR_W { w: self }
    }
}