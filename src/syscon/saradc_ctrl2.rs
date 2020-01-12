#[doc = "Reader of register SARADC_CTRL2"]
pub type R = crate::R<u32, super::SARADC_CTRL2>;
#[doc = "Writer for register SARADC_CTRL2"]
pub type W = crate::W<u32, super::SARADC_CTRL2>;
#[doc = "Register SARADC_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SARADC_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SARADC_SAR2_INV`"]
pub type SARADC_SAR2_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SARADC_SAR2_INV`"]
pub struct SARADC_SAR2_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR2_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SARADC_SAR1_INV`"]
pub type SARADC_SAR1_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SARADC_SAR1_INV`"]
pub struct SARADC_SAR1_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SARADC_MAX_MEAS_NUM`"]
pub type SARADC_MAX_MEAS_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SARADC_MAX_MEAS_NUM`"]
pub struct SARADC_MAX_MEAS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_MAX_MEAS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 1)) | (((value as u32) & 0xff) << 1);
        self.w
    }
}
#[doc = "Reader of field `SARADC_MEAS_NUM_LIMIT`"]
pub type SARADC_MEAS_NUM_LIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SARADC_MEAS_NUM_LIMIT`"]
pub struct SARADC_MEAS_NUM_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_MEAS_NUM_LIMIT_W<'a> {
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
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted otherwise not"]
    #[inline(always)]
    pub fn saradc_sar2_inv(&self) -> SARADC_SAR2_INV_R {
        SARADC_SAR2_INV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted otherwise not"]
    #[inline(always)]
    pub fn saradc_sar1_inv(&self) -> SARADC_SAR1_INV_R {
        SARADC_SAR1_INV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn saradc_max_meas_num(&self) -> SARADC_MAX_MEAS_NUM_R {
        SARADC_MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn saradc_meas_num_limit(&self) -> SARADC_MEAS_NUM_LIMIT_R {
        SARADC_MEAS_NUM_LIMIT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted otherwise not"]
    #[inline(always)]
    pub fn saradc_sar2_inv(&mut self) -> SARADC_SAR2_INV_W {
        SARADC_SAR2_INV_W { w: self }
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted otherwise not"]
    #[inline(always)]
    pub fn saradc_sar1_inv(&mut self) -> SARADC_SAR1_INV_W {
        SARADC_SAR1_INV_W { w: self }
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn saradc_max_meas_num(&mut self) -> SARADC_MAX_MEAS_NUM_W {
        SARADC_MAX_MEAS_NUM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn saradc_meas_num_limit(&mut self) -> SARADC_MEAS_NUM_LIMIT_W {
        SARADC_MEAS_NUM_LIMIT_W { w: self }
    }
}
