#[doc = "Reader of register I2S_INFIFO_POP_REG"]
pub type R = crate::R<u32, super::I2S_INFIFO_POP_REG>;
#[doc = "Writer for register I2S_INFIFO_POP_REG"]
pub type W = crate::W<u32, super::I2S_INFIFO_POP_REG>;
#[doc = "Register I2S_INFIFO_POP_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_INFIFO_POP_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_INFIFO_POP`"]
pub type I2S_INFIFO_POP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INFIFO_POP`"]
pub struct I2S_INFIFO_POP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INFIFO_POP_W<'a> {
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
#[doc = "Reader of field `I2S_INFIFO_RDATA`"]
pub type I2S_INFIFO_RDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_INFIFO_RDATA`"]
pub struct I2S_INFIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INFIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_infifo_pop(&self) -> I2S_INFIFO_POP_R {
        I2S_INFIFO_POP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn i2s_infifo_rdata(&self) -> I2S_INFIFO_RDATA_R {
        I2S_INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_infifo_pop(&mut self) -> I2S_INFIFO_POP_W {
        I2S_INFIFO_POP_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn i2s_infifo_rdata(&mut self) -> I2S_INFIFO_RDATA_W {
        I2S_INFIFO_RDATA_W { w: self }
    }
}
