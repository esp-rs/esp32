#[doc = "Reader of register LC_HUNG_CONF"]
pub type R = crate::R<u32, super::LC_HUNG_CONF>;
#[doc = "Writer for register LC_HUNG_CONF"]
pub type W = crate::W<u32, super::LC_HUNG_CONF>;
#[doc = "Register LC_HUNG_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::LC_HUNG_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_LC_FIFO_TIMEOUT_ENA`"]
pub type I2S_LC_FIFO_TIMEOUT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_LC_FIFO_TIMEOUT_ENA`"]
pub struct I2S_LC_FIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_FIFO_TIMEOUT_ENA_W<'a> {
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
#[doc = "Reader of field `I2S_LC_FIFO_TIMEOUT_SHIFT`"]
pub type I2S_LC_FIFO_TIMEOUT_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_LC_FIFO_TIMEOUT_SHIFT`"]
pub struct I2S_LC_FIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_FIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2S_LC_FIFO_TIMEOUT`"]
pub type I2S_LC_FIFO_TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_LC_FIFO_TIMEOUT`"]
pub struct I2S_LC_FIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_FIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_ena(&self) -> I2S_LC_FIFO_TIMEOUT_ENA_R {
        I2S_LC_FIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_shift(&self) -> I2S_LC_FIFO_TIMEOUT_SHIFT_R {
        I2S_LC_FIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout(&self) -> I2S_LC_FIFO_TIMEOUT_R {
        I2S_LC_FIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_ena(&mut self) -> I2S_LC_FIFO_TIMEOUT_ENA_W {
        I2S_LC_FIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_shift(&mut self) -> I2S_LC_FIFO_TIMEOUT_SHIFT_W {
        I2S_LC_FIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout(&mut self) -> I2S_LC_FIFO_TIMEOUT_W {
        I2S_LC_FIFO_TIMEOUT_W { w: self }
    }
}
