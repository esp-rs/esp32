#[doc = "Reader of register I2S_OUTFIFO_PUSH_REG"]
pub type R = crate::R<u32, super::I2S_OUTFIFO_PUSH_REG>;
#[doc = "Writer for register I2S_OUTFIFO_PUSH_REG"]
pub type W = crate::W<u32, super::I2S_OUTFIFO_PUSH_REG>;
#[doc = "Register I2S_OUTFIFO_PUSH_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_OUTFIFO_PUSH_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_OUTFIFO_PUSH`"]
pub type I2S_OUTFIFO_PUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUTFIFO_PUSH`"]
pub struct I2S_OUTFIFO_PUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUTFIFO_PUSH_W<'a> {
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
#[doc = "Reader of field `I2S_OUTFIFO_WDATA`"]
pub type I2S_OUTFIFO_WDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_OUTFIFO_WDATA`"]
pub struct I2S_OUTFIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUTFIFO_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_outfifo_push(&self) -> I2S_OUTFIFO_PUSH_R {
        I2S_OUTFIFO_PUSH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn i2s_outfifo_wdata(&self) -> I2S_OUTFIFO_WDATA_R {
        I2S_OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_outfifo_push(&mut self) -> I2S_OUTFIFO_PUSH_W {
        I2S_OUTFIFO_PUSH_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn i2s_outfifo_wdata(&mut self) -> I2S_OUTFIFO_WDATA_W {
        I2S_OUTFIFO_WDATA_W { w: self }
    }
}
