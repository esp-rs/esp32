#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Writer for register INT_STATUS"]
pub type W = crate::W<u32, super::INT_STATUS>;
#[doc = "Register INT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_SEND_EMPTY_INT_ST`"]
pub type TX_SEND_EMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SEND_EMPTY_INT_ST`"]
pub struct TX_SEND_EMPTY_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SEND_EMPTY_INT_ST_W<'a> {
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
#[doc = "Reader of field `RX_REC_FULL_INT_ST`"]
pub type RX_REC_FULL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_REC_FULL_INT_ST`"]
pub struct RX_REC_FULL_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REC_FULL_INT_ST_W<'a> {
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
#[doc = "Reader of field `ACK_ERR_INT_ST`"]
pub type ACK_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK_ERR_INT_ST`"]
pub struct ACK_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `TRANS_START_INT_ST`"]
pub type TRANS_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_START_INT_ST`"]
pub struct TRANS_START_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_START_INT_ST_W<'a> {
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
#[doc = "Reader of field `TIME_OUT_INT_ST`"]
pub type TIME_OUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_OUT_INT_ST`"]
pub struct TIME_OUT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_INT_ST_W<'a> {
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
#[doc = "Reader of field `TRANS_COMPLETE_INT_ST`"]
pub type TRANS_COMPLETE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_COMPLETE_INT_ST`"]
pub struct TRANS_COMPLETE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_INT_ST_W<'a> {
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
#[doc = "Reader of field `MASTER_TRAN_COMP_INT_ST`"]
pub type MASTER_TRAN_COMP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER_TRAN_COMP_INT_ST`"]
pub struct MASTER_TRAN_COMP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TRAN_COMP_INT_ST_W<'a> {
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
#[doc = "Reader of field `ARBITRATION_LOST_INT_ST`"]
pub type ARBITRATION_LOST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBITRATION_LOST_INT_ST`"]
pub struct ARBITRATION_LOST_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBITRATION_LOST_INT_ST_W<'a> {
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
#[doc = "Reader of field `SLAVE_TRAN_COMP_INT_ST`"]
pub type SLAVE_TRAN_COMP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_TRAN_COMP_INT_ST`"]
pub struct SLAVE_TRAN_COMP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TRAN_COMP_INT_ST_W<'a> {
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
#[doc = "Reader of field `END_DETECT_INT_ST`"]
pub type END_DETECT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_DETECT_INT_ST`"]
pub struct END_DETECT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DETECT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RXFIFO_OVF_INT_ST`"]
pub type RXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_OVF_INT_ST`"]
pub struct RXFIFO_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_ST_W<'a> {
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
#[doc = "Reader of field `TXFIFO_EMPTY_INT_ST`"]
pub type TXFIFO_EMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_EMPTY_INT_ST`"]
pub struct TXFIFO_EMPTY_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_ST_W<'a> {
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
#[doc = "Reader of field `RXFIFO_FULL_INT_ST`"]
pub type RXFIFO_FULL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_FULL_INT_ST`"]
pub struct RXFIFO_FULL_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_ST_W<'a> {
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
    #[doc = "Bit 12 - The masked interrupt status for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty_int_st(&self) -> TX_SEND_EMPTY_INT_ST_R {
        TX_SEND_EMPTY_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_st(&self) -> RX_REC_FULL_INT_ST_R {
        RX_REC_FULL_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> TRANS_START_INT_ST_R {
        TRANS_START_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> END_DETECT_INT_ST_R {
        END_DETECT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The masked interrupt status for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - The masked interrupt status for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty_int_st(&mut self) -> TX_SEND_EMPTY_INT_ST_W {
        TX_SEND_EMPTY_INT_ST_W { w: self }
    }
    #[doc = "Bit 11 - The masked interrupt status for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_st(&mut self) -> RX_REC_FULL_INT_ST_W {
        RX_REC_FULL_INT_ST_W { w: self }
    }
    #[doc = "Bit 10 - The masked interrupt status for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err_int_st(&mut self) -> ACK_ERR_INT_ST_W {
        ACK_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 9 - The masked interrupt status for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_st(&mut self) -> TRANS_START_INT_ST_W {
        TRANS_START_INT_ST_W { w: self }
    }
    #[doc = "Bit 8 - The masked interrupt status for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_st(&mut self) -> TIME_OUT_INT_ST_W {
        TIME_OUT_INT_ST_W { w: self }
    }
    #[doc = "Bit 7 - The masked interrupt status for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_st(&mut self) -> TRANS_COMPLETE_INT_ST_W {
        TRANS_COMPLETE_INT_ST_W { w: self }
    }
    #[doc = "Bit 6 - The masked interrupt status for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&mut self) -> MASTER_TRAN_COMP_INT_ST_W {
        MASTER_TRAN_COMP_INT_ST_W { w: self }
    }
    #[doc = "Bit 5 - The masked interrupt status for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&mut self) -> ARBITRATION_LOST_INT_ST_W {
        ARBITRATION_LOST_INT_ST_W { w: self }
    }
    #[doc = "Bit 4 - The masked interrupt status for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&mut self) -> SLAVE_TRAN_COMP_INT_ST_W {
        SLAVE_TRAN_COMP_INT_ST_W { w: self }
    }
    #[doc = "Bit 3 - The masked interrupt status for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_st(&mut self) -> END_DETECT_INT_ST_W {
        END_DETECT_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - The masked interrupt status for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&mut self) -> RXFIFO_OVF_INT_ST_W {
        RXFIFO_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - The masked interrupt status for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&mut self) -> TXFIFO_EMPTY_INT_ST_W {
        TXFIFO_EMPTY_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - The masked interrupt status for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&mut self) -> RXFIFO_FULL_INT_ST_W {
        RXFIFO_FULL_INT_ST_W { w: self }
    }
}
