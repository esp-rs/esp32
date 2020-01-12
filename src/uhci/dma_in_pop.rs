#[doc = "Reader of register DMA_IN_POP"]
pub type R = crate::R<u32, super::DMA_IN_POP>;
#[doc = "Writer for register DMA_IN_POP"]
pub type W = crate::W<u32, super::DMA_IN_POP>;
#[doc = "Register DMA_IN_POP `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_POP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INFIFO_POP`"]
pub type INFIFO_POP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INFIFO_POP`"]
pub struct INFIFO_POP_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_POP_W<'a> {
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
#[doc = "Reader of field `INFIFO_RDATA`"]
pub type INFIFO_RDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INFIFO_RDATA`"]
pub struct INFIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Set this bit to pop data in in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_pop(&self) -> INFIFO_POP_R {
        INFIFO_POP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - This register stores the data pop from in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_rdata(&self) -> INFIFO_RDATA_R {
        INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Set this bit to pop data in in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_pop(&mut self) -> INFIFO_POP_W {
        INFIFO_POP_W { w: self }
    }
    #[doc = "Bits 0:11 - This register stores the data pop from in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_rdata(&mut self) -> INFIFO_RDATA_W {
        INFIFO_RDATA_W { w: self }
    }
}
