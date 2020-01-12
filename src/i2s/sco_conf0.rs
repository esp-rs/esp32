#[doc = "Reader of register SCO_CONF0"]
pub type R = crate::R<u32, super::SCO_CONF0>;
#[doc = "Writer for register SCO_CONF0"]
pub type W = crate::W<u32, super::SCO_CONF0>;
#[doc = "Register SCO_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCO_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CVSD_ENC_RESET`"]
pub type CVSD_ENC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVSD_ENC_RESET`"]
pub struct CVSD_ENC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_ENC_RESET_W<'a> {
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
#[doc = "Reader of field `CVSD_ENC_START`"]
pub type CVSD_ENC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVSD_ENC_START`"]
pub struct CVSD_ENC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_ENC_START_W<'a> {
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
#[doc = "Reader of field `SCO_NO_I2S_EN`"]
pub type SCO_NO_I2S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCO_NO_I2S_EN`"]
pub struct SCO_NO_I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCO_NO_I2S_EN_W<'a> {
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
#[doc = "Reader of field `SCO_WITH_I2S_EN`"]
pub type SCO_WITH_I2S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCO_WITH_I2S_EN`"]
pub struct SCO_WITH_I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCO_WITH_I2S_EN_W<'a> {
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
    pub fn cvsd_enc_reset(&self) -> CVSD_ENC_RESET_R {
        CVSD_ENC_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cvsd_enc_start(&self) -> CVSD_ENC_START_R {
        CVSD_ENC_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sco_no_i2s_en(&self) -> SCO_NO_I2S_EN_R {
        SCO_NO_I2S_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sco_with_i2s_en(&self) -> SCO_WITH_I2S_EN_R {
        SCO_WITH_I2S_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cvsd_enc_reset(&mut self) -> CVSD_ENC_RESET_W {
        CVSD_ENC_RESET_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cvsd_enc_start(&mut self) -> CVSD_ENC_START_W {
        CVSD_ENC_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sco_no_i2s_en(&mut self) -> SCO_NO_I2S_EN_W {
        SCO_NO_I2S_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sco_with_i2s_en(&mut self) -> SCO_WITH_I2S_EN_W {
        SCO_WITH_I2S_EN_W { w: self }
    }
}
