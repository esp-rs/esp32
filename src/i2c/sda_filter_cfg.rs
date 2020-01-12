#[doc = "Reader of register SDA_FILTER_CFG"]
pub type R = crate::R<u32, super::SDA_FILTER_CFG>;
#[doc = "Writer for register SDA_FILTER_CFG"]
pub type W = crate::W<u32, super::SDA_FILTER_CFG>;
#[doc = "Register SDA_FILTER_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SDA_FILTER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDA_FILTER_EN`"]
pub type SDA_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDA_FILTER_EN`"]
pub struct SDA_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_EN_W<'a> {
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
#[doc = "Reader of field `SDA_FILTER_THRES`"]
pub type SDA_FILTER_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_FILTER_THRES`"]
pub struct SDA_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W {
        SDA_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W {
        SDA_FILTER_THRES_W { w: self }
    }
}
