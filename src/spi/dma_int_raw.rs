#[doc = "Reader of register DMA_INT_RAW"]
pub type R = crate::R<u32, super::DMA_INT_RAW>;
#[doc = "Writer for register DMA_INT_RAW"]
pub type W = crate::W<u32, super::DMA_INT_RAW>;
#[doc = "Register DMA_INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT_TOTAL_EOF_INT_RAW`"]
pub type OUT_TOTAL_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_TOTAL_EOF_INT_RAW`"]
pub struct OUT_TOTAL_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `OUT_EOF_INT_RAW`"]
pub type OUT_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EOF_INT_RAW`"]
pub struct OUT_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `OUT_DONE_INT_RAW`"]
pub type OUT_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_DONE_INT_RAW`"]
pub struct OUT_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `IN_SUC_EOF_INT_RAW`"]
pub type IN_SUC_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_SUC_EOF_INT_RAW`"]
pub struct IN_SUC_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `IN_ERR_EOF_INT_RAW`"]
pub type IN_ERR_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_ERR_EOF_INT_RAW`"]
pub struct IN_ERR_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `IN_DONE_INT_RAW`"]
pub type IN_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DONE_INT_RAW`"]
pub struct IN_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `INLINK_DSCR_ERROR_INT_RAW`"]
pub type INLINK_DSCR_ERROR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INLINK_DSCR_ERROR_INT_RAW`"]
pub struct INLINK_DSCR_ERROR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_ERROR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `OUTLINK_DSCR_ERROR_INT_RAW`"]
pub type OUTLINK_DSCR_ERROR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_DSCR_ERROR_INT_RAW`"]
pub struct OUTLINK_DSCR_ERROR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_DSCR_ERROR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `INLINK_DSCR_EMPTY_INT_RAW`"]
pub type INLINK_DSCR_EMPTY_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INLINK_DSCR_EMPTY_INT_RAW`"]
pub struct INLINK_DSCR_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_EMPTY_INT_RAW_W<'a> {
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
    #[doc = "Bit 8 - The raw bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&self) -> OUT_TOTAL_EOF_INT_RAW_R {
        OUT_TOTAL_EOF_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The raw bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The raw bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_raw(&self) -> OUT_DONE_INT_RAW_R {
        OUT_DONE_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The raw bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&self) -> IN_SUC_EOF_INT_RAW_R {
        IN_SUC_EOF_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The raw bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&self) -> IN_ERR_EOF_INT_RAW_R {
        IN_ERR_EOF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The raw bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_raw(&self) -> IN_DONE_INT_RAW_R {
        IN_DONE_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The raw bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_raw(&self) -> INLINK_DSCR_ERROR_INT_RAW_R {
        INLINK_DSCR_ERROR_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The raw bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_raw(&self) -> OUTLINK_DSCR_ERROR_INT_RAW_R {
        OUTLINK_DSCR_ERROR_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The raw bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_raw(&self) -> INLINK_DSCR_EMPTY_INT_RAW_R {
        INLINK_DSCR_EMPTY_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - The raw bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&mut self) -> OUT_TOTAL_EOF_INT_RAW_W {
        OUT_TOTAL_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - The raw bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_raw(&mut self) -> OUT_EOF_INT_RAW_W {
        OUT_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - The raw bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_raw(&mut self) -> OUT_DONE_INT_RAW_W {
        OUT_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - The raw bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&mut self) -> IN_SUC_EOF_INT_RAW_W {
        IN_SUC_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - The raw bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&mut self) -> IN_ERR_EOF_INT_RAW_W {
        IN_ERR_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - The raw bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_raw(&mut self) -> IN_DONE_INT_RAW_W {
        IN_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - The raw bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_raw(&mut self) -> INLINK_DSCR_ERROR_INT_RAW_W {
        INLINK_DSCR_ERROR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - The raw bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_raw(&mut self) -> OUTLINK_DSCR_ERROR_INT_RAW_W {
        OUTLINK_DSCR_ERROR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - The raw bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_raw(&mut self) -> INLINK_DSCR_EMPTY_INT_RAW_W {
        INLINK_DSCR_EMPTY_INT_RAW_W { w: self }
    }
}
