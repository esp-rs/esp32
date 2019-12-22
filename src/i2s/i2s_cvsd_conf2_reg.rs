#[doc = "Reader of register I2S_CVSD_CONF2_REG"]
pub type R = crate::R<u32, super::I2S_CVSD_CONF2_REG>;
#[doc = "Writer for register I2S_CVSD_CONF2_REG"]
pub type W = crate::W<u32, super::I2S_CVSD_CONF2_REG>;
#[doc = "Register I2S_CVSD_CONF2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CVSD_CONF2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_CVSD_H`"]
pub type I2S_CVSD_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CVSD_H`"]
pub struct I2S_CVSD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_BETA`"]
pub type I2S_CVSD_BETA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_CVSD_BETA`"]
pub struct I2S_CVSD_BETA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_BETA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_J`"]
pub type I2S_CVSD_J_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CVSD_J`"]
pub struct I2S_CVSD_J_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_J_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_K`"]
pub type I2S_CVSD_K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CVSD_K`"]
pub struct I2S_CVSD_K_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2s_cvsd_h(&self) -> I2S_CVSD_H_R {
        I2S_CVSD_H_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn i2s_cvsd_beta(&self) -> I2S_CVSD_BETA_R {
        I2S_CVSD_BETA_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn i2s_cvsd_j(&self) -> I2S_CVSD_J_R {
        I2S_CVSD_J_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn i2s_cvsd_k(&self) -> I2S_CVSD_K_R {
        I2S_CVSD_K_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2s_cvsd_h(&mut self) -> I2S_CVSD_H_W {
        I2S_CVSD_H_W { w: self }
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn i2s_cvsd_beta(&mut self) -> I2S_CVSD_BETA_W {
        I2S_CVSD_BETA_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn i2s_cvsd_j(&mut self) -> I2S_CVSD_J_W {
        I2S_CVSD_J_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn i2s_cvsd_k(&mut self) -> I2S_CVSD_K_W {
        I2S_CVSD_K_W { w: self }
    }
}
