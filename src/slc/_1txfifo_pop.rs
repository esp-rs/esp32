#[doc = "Reader of register 1TXFIFO_POP"]
pub type R = crate::R<u32, super::_1TXFIFO_POP>;
#[doc = "Writer for register 1TXFIFO_POP"]
pub type W = crate::W<u32, super::_1TXFIFO_POP>;
#[doc = "Register 1TXFIFO_POP `reset()`'s with value 0"]
impl crate::ResetValue for super::_1TXFIFO_POP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC1_TXFIFO_POP`"]
pub type SLC1_TXFIFO_POP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TXFIFO_POP`"]
pub struct SLC1_TXFIFO_POP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXFIFO_POP_W<'a> {
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
#[doc = "Reader of field `SLC1_TXFIFO_RDATA`"]
pub type SLC1_TXFIFO_RDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC1_TXFIFO_RDATA`"]
pub struct SLC1_TXFIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXFIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_txfifo_pop(&self) -> SLC1_TXFIFO_POP_R {
        SLC1_TXFIFO_POP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc1_txfifo_rdata(&self) -> SLC1_TXFIFO_RDATA_R {
        SLC1_TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_txfifo_pop(&mut self) -> SLC1_TXFIFO_POP_W {
        SLC1_TXFIFO_POP_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc1_txfifo_rdata(&mut self) -> SLC1_TXFIFO_RDATA_W {
        SLC1_TXFIFO_RDATA_W { w: self }
    }
}
