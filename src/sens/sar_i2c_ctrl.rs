#[doc = "Reader of register SAR_I2C_CTRL"]
pub type R = crate::R<u32, super::SAR_I2C_CTRL>;
#[doc = "Writer for register SAR_I2C_CTRL"]
pub type W = crate::W<u32, super::SAR_I2C_CTRL>;
#[doc = "Register SAR_I2C_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_I2C_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR_I2C_START_FORCE`"]
pub type SAR_I2C_START_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR_I2C_START_FORCE`"]
pub struct SAR_I2C_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_START_FORCE_W<'a> {
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
#[doc = "Reader of field `SAR_I2C_START`"]
pub type SAR_I2C_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR_I2C_START`"]
pub struct SAR_I2C_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_START_W<'a> {
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
#[doc = "Reader of field `SAR_I2C_CTRL`"]
pub type SAR_I2C_CTRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR_I2C_CTRL`"]
pub struct SAR_I2C_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    pub fn sar_i2c_start_force(&self) -> SAR_I2C_START_FORCE_R {
        SAR_I2C_START_FORCE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_start(&self) -> SAR_I2C_START_R {
        SAR_I2C_START_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_ctrl(&self) -> SAR_I2C_CTRL_R {
        SAR_I2C_CTRL_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    pub fn sar_i2c_start_force(&mut self) -> SAR_I2C_START_FORCE_W {
        SAR_I2C_START_FORCE_W { w: self }
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_start(&mut self) -> SAR_I2C_START_W {
        SAR_I2C_START_W { w: self }
    }
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_ctrl(&mut self) -> SAR_I2C_CTRL_W {
        SAR_I2C_CTRL_W { w: self }
    }
}
