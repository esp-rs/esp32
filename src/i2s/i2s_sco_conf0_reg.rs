#[doc = "Reader of register I2S_SCO_CONF0_REG"]
pub type R = crate::R<u32, super::I2S_SCO_CONF0_REG>;
#[doc = "Writer for register I2S_SCO_CONF0_REG"]
pub type W = crate::W<u32, super::I2S_SCO_CONF0_REG>;
#[doc = "Register I2S_SCO_CONF0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_SCO_CONF0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_CVSD_ENC_RESET`"]
pub type I2S_CVSD_ENC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CVSD_ENC_RESET`"]
pub struct I2S_CVSD_ENC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_ENC_RESET_W<'a> {
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
#[doc = "Reader of field `I2S_CVSD_ENC_START`"]
pub type I2S_CVSD_ENC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CVSD_ENC_START`"]
pub struct I2S_CVSD_ENC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_ENC_START_W<'a> {
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
#[doc = "Reader of field `I2S_SCO_NO_I2S_EN`"]
pub type I2S_SCO_NO_I2S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_SCO_NO_I2S_EN`"]
pub struct I2S_SCO_NO_I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SCO_NO_I2S_EN_W<'a> {
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
#[doc = "Reader of field `I2S_SCO_WITH_I2S_EN`"]
pub type I2S_SCO_WITH_I2S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_SCO_WITH_I2S_EN`"]
pub struct I2S_SCO_WITH_I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SCO_WITH_I2S_EN_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_cvsd_enc_reset(&self) -> I2S_CVSD_ENC_RESET_R {
        I2S_CVSD_ENC_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_cvsd_enc_start(&self) -> I2S_CVSD_ENC_START_R {
        I2S_CVSD_ENC_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_sco_no_i2s_en(&self) -> I2S_SCO_NO_I2S_EN_R {
        I2S_SCO_NO_I2S_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_sco_with_i2s_en(&self) -> I2S_SCO_WITH_I2S_EN_R {
        I2S_SCO_WITH_I2S_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_cvsd_enc_reset(&mut self) -> I2S_CVSD_ENC_RESET_W {
        I2S_CVSD_ENC_RESET_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_cvsd_enc_start(&mut self) -> I2S_CVSD_ENC_START_W {
        I2S_CVSD_ENC_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_sco_no_i2s_en(&mut self) -> I2S_SCO_NO_I2S_EN_W {
        I2S_SCO_NO_I2S_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_sco_with_i2s_en(&mut self) -> I2S_SCO_WITH_I2S_EN_W {
        I2S_SCO_WITH_I2S_EN_W { w: self }
    }
}
