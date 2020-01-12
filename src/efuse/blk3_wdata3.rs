#[doc = "Reader of register BLK3_WDATA3"]
pub type R = crate::R<u32, super::BLK3_WDATA3>;
#[doc = "Writer for register BLK3_WDATA3"]
pub type W = crate::W<u32, super::BLK3_WDATA3>;
#[doc = "Register BLK3_WDATA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK3_WDATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK3_DIN3`"]
pub type BLK3_DIN3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLK3_DIN3`"]
pub struct BLK3_DIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK3_DIN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `ADC2_TP_HIGH`"]
pub type ADC2_TP_HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC2_TP_HIGH`"]
pub struct ADC2_TP_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_TP_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADC2_TP_LOW`"]
pub type ADC2_TP_LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC2_TP_LOW`"]
pub struct ADC2_TP_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_TP_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADC1_TP_HIGH`"]
pub type ADC1_TP_HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC1_TP_HIGH`"]
pub struct ADC1_TP_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_TP_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Reader of field `ADC1_TP_LOW`"]
pub type ADC1_TP_LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC1_TP_LOW`"]
pub struct ADC1_TP_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_TP_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din3(&self) -> BLK3_DIN3_R {
        BLK3_DIN3_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_high(&self) -> ADC2_TP_HIGH_R {
        ADC2_TP_HIGH_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_low(&self) -> ADC2_TP_LOW_R {
        ADC2_TP_LOW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_high(&self) -> ADC1_TP_HIGH_R {
        ADC1_TP_HIGH_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_low(&self) -> ADC1_TP_LOW_R {
        ADC1_TP_LOW_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din3(&mut self) -> BLK3_DIN3_W {
        BLK3_DIN3_W { w: self }
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_high(&mut self) -> ADC2_TP_HIGH_W {
        ADC2_TP_HIGH_W { w: self }
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_low(&mut self) -> ADC2_TP_LOW_W {
        ADC2_TP_LOW_W { w: self }
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_high(&mut self) -> ADC1_TP_HIGH_W {
        ADC1_TP_HIGH_W { w: self }
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_low(&mut self) -> ADC1_TP_LOW_W {
        ADC1_TP_LOW_W { w: self }
    }
}
