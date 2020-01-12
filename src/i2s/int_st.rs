#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Writer for register INT_ST"]
pub type W = crate::W<u32, super::INT_ST>;
#[doc = "Register INT_ST `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT_TOTAL_EOF_INT_ST`"]
pub type OUT_TOTAL_EOF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_TOTAL_EOF_INT_ST`"]
pub struct OUT_TOTAL_EOF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_INT_ST_W<'a> {
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
#[doc = "Reader of field `IN_DSCR_EMPTY_INT_ST`"]
pub type IN_DSCR_EMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DSCR_EMPTY_INT_ST`"]
pub struct IN_DSCR_EMPTY_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_EMPTY_INT_ST_W<'a> {
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
#[doc = "Reader of field `OUT_DSCR_ERR_INT_ST`"]
pub type OUT_DSCR_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_DSCR_ERR_INT_ST`"]
pub struct OUT_DSCR_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DSCR_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `IN_DSCR_ERR_INT_ST`"]
pub type IN_DSCR_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DSCR_ERR_INT_ST`"]
pub struct IN_DSCR_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `OUT_EOF_INT_ST`"]
pub type OUT_EOF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EOF_INT_ST`"]
pub struct OUT_EOF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_INT_ST_W<'a> {
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
#[doc = "Reader of field `OUT_DONE_INT_ST`"]
pub type OUT_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_DONE_INT_ST`"]
pub struct OUT_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_INT_ST_W<'a> {
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
#[doc = "Reader of field `IN_ERR_EOF_INT_ST`"]
pub type IN_ERR_EOF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_ERR_EOF_INT_ST`"]
pub struct IN_ERR_EOF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_INT_ST_W<'a> {
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
#[doc = "Reader of field `IN_SUC_EOF_INT_ST`"]
pub type IN_SUC_EOF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_SUC_EOF_INT_ST`"]
pub struct IN_SUC_EOF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_INT_ST_W<'a> {
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
#[doc = "Reader of field `IN_DONE_INT_ST`"]
pub type IN_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DONE_INT_ST`"]
pub struct IN_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_INT_ST_W<'a> {
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
#[doc = "Reader of field `TX_HUNG_INT_ST`"]
pub type TX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_HUNG_INT_ST`"]
pub struct TX_HUNG_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HUNG_INT_ST_W<'a> {
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
#[doc = "Reader of field `RX_HUNG_INT_ST`"]
pub type RX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_HUNG_INT_ST`"]
pub struct RX_HUNG_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HUNG_INT_ST_W<'a> {
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
#[doc = "Reader of field `TX_REMPTY_INT_ST`"]
pub type TX_REMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_REMPTY_INT_ST`"]
pub struct TX_REMPTY_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REMPTY_INT_ST_W<'a> {
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
#[doc = "Reader of field `TX_WFULL_INT_ST`"]
pub type TX_WFULL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_WFULL_INT_ST`"]
pub struct TX_WFULL_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WFULL_INT_ST_W<'a> {
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
#[doc = "Reader of field `RX_REMPTY_INT_ST`"]
pub type RX_REMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_REMPTY_INT_ST`"]
pub struct RX_REMPTY_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REMPTY_INT_ST_W<'a> {
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
#[doc = "Reader of field `RX_WFULL_INT_ST`"]
pub type RX_WFULL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_WFULL_INT_ST`"]
pub struct RX_WFULL_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WFULL_INT_ST_W<'a> {
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
#[doc = "Reader of field `TX_PUT_DATA_INT_ST`"]
pub type TX_PUT_DATA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PUT_DATA_INT_ST`"]
pub struct TX_PUT_DATA_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PUT_DATA_INT_ST_W<'a> {
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
#[doc = "Reader of field `RX_TAKE_DATA_INT_ST`"]
pub type RX_TAKE_DATA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TAKE_DATA_INT_ST`"]
pub struct RX_TAKE_DATA_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TAKE_DATA_INT_ST_W<'a> {
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
    pub fn out_total_eof_int_st(&self) -> OUT_TOTAL_EOF_INT_ST_R {
        OUT_TOTAL_EOF_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn in_dscr_empty_int_st(&self) -> IN_DSCR_EMPTY_INT_ST_R {
        IN_DSCR_EMPTY_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn out_dscr_err_int_st(&self) -> OUT_DSCR_ERR_INT_ST_R {
        OUT_DSCR_ERR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn in_dscr_err_int_st(&self) -> IN_DSCR_ERR_INT_ST_R {
        IN_DSCR_ERR_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn out_eof_int_st(&self) -> OUT_EOF_INT_ST_R {
        OUT_EOF_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn out_done_int_st(&self) -> OUT_DONE_INT_ST_R {
        OUT_DONE_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn in_err_eof_int_st(&self) -> IN_ERR_EOF_INT_ST_R {
        IN_ERR_EOF_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_suc_eof_int_st(&self) -> IN_SUC_EOF_INT_ST_R {
        IN_SUC_EOF_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn in_done_int_st(&self) -> IN_DONE_INT_ST_R {
        IN_DONE_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> TX_HUNG_INT_ST_R {
        TX_HUNG_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_rempty_int_st(&self) -> TX_REMPTY_INT_ST_R {
        TX_REMPTY_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_wfull_int_st(&self) -> TX_WFULL_INT_ST_R {
        TX_WFULL_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_rempty_int_st(&self) -> RX_REMPTY_INT_ST_R {
        RX_REMPTY_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_wfull_int_st(&self) -> RX_WFULL_INT_ST_R {
        RX_WFULL_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_put_data_int_st(&self) -> TX_PUT_DATA_INT_ST_R {
        TX_PUT_DATA_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_take_data_int_st(&self) -> RX_TAKE_DATA_INT_ST_R {
        RX_TAKE_DATA_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn out_total_eof_int_st(&mut self) -> OUT_TOTAL_EOF_INT_ST_W {
        OUT_TOTAL_EOF_INT_ST_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn in_dscr_empty_int_st(&mut self) -> IN_DSCR_EMPTY_INT_ST_W {
        IN_DSCR_EMPTY_INT_ST_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn out_dscr_err_int_st(&mut self) -> OUT_DSCR_ERR_INT_ST_W {
        OUT_DSCR_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn in_dscr_err_int_st(&mut self) -> IN_DSCR_ERR_INT_ST_W {
        IN_DSCR_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn out_eof_int_st(&mut self) -> OUT_EOF_INT_ST_W {
        OUT_EOF_INT_ST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn out_done_int_st(&mut self) -> OUT_DONE_INT_ST_W {
        OUT_DONE_INT_ST_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn in_err_eof_int_st(&mut self) -> IN_ERR_EOF_INT_ST_W {
        IN_ERR_EOF_INT_ST_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_suc_eof_int_st(&mut self) -> IN_SUC_EOF_INT_ST_W {
        IN_SUC_EOF_INT_ST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn in_done_int_st(&mut self) -> IN_DONE_INT_ST_W {
        IN_DONE_INT_ST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_hung_int_st(&mut self) -> TX_HUNG_INT_ST_W {
        TX_HUNG_INT_ST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_hung_int_st(&mut self) -> RX_HUNG_INT_ST_W {
        RX_HUNG_INT_ST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_rempty_int_st(&mut self) -> TX_REMPTY_INT_ST_W {
        TX_REMPTY_INT_ST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_wfull_int_st(&mut self) -> TX_WFULL_INT_ST_W {
        TX_WFULL_INT_ST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_rempty_int_st(&mut self) -> RX_REMPTY_INT_ST_W {
        RX_REMPTY_INT_ST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_wfull_int_st(&mut self) -> RX_WFULL_INT_ST_W {
        RX_WFULL_INT_ST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_put_data_int_st(&mut self) -> TX_PUT_DATA_INT_ST_W {
        TX_PUT_DATA_INT_ST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_take_data_int_st(&mut self) -> RX_TAKE_DATA_INT_ST_W {
        RX_TAKE_DATA_INT_ST_W { w: self }
    }
}
