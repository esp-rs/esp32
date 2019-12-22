#[doc = "Reader of register I2S_IN_LINK_REG"]
pub type R = crate::R<u32, super::I2S_IN_LINK_REG>;
#[doc = "Writer for register I2S_IN_LINK_REG"]
pub type W = crate::W<u32, super::I2S_IN_LINK_REG>;
#[doc = "Register I2S_IN_LINK_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_IN_LINK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_INLINK_PARK`"]
pub type I2S_INLINK_PARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INLINK_PARK`"]
pub struct I2S_INLINK_PARK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INLINK_PARK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `I2S_INLINK_RESTART`"]
pub type I2S_INLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INLINK_RESTART`"]
pub struct I2S_INLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INLINK_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `I2S_INLINK_START`"]
pub type I2S_INLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INLINK_START`"]
pub struct I2S_INLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INLINK_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `I2S_INLINK_STOP`"]
pub type I2S_INLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INLINK_STOP`"]
pub struct I2S_INLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INLINK_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `I2S_INLINK_ADDR`"]
pub type I2S_INLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2S_INLINK_ADDR`"]
pub struct I2S_INLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i2s_inlink_park(&self) -> I2S_INLINK_PARK_R {
        I2S_INLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn i2s_inlink_restart(&self) -> I2S_INLINK_RESTART_R {
        I2S_INLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn i2s_inlink_start(&self) -> I2S_INLINK_START_R {
        I2S_INLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn i2s_inlink_stop(&self) -> I2S_INLINK_STOP_R {
        I2S_INLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn i2s_inlink_addr(&self) -> I2S_INLINK_ADDR_R {
        I2S_INLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i2s_inlink_park(&mut self) -> I2S_INLINK_PARK_W {
        I2S_INLINK_PARK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn i2s_inlink_restart(&mut self) -> I2S_INLINK_RESTART_W {
        I2S_INLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn i2s_inlink_start(&mut self) -> I2S_INLINK_START_W {
        I2S_INLINK_START_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn i2s_inlink_stop(&mut self) -> I2S_INLINK_STOP_W {
        I2S_INLINK_STOP_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn i2s_inlink_addr(&mut self) -> I2S_INLINK_ADDR_W {
        I2S_INLINK_ADDR_W { w: self }
    }
}
