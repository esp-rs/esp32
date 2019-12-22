#[doc = "Reader of register EFUSE_BLK3_RDATA3_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK3_RDATA3_REG>;
#[doc = "Writer for register EFUSE_BLK3_RDATA3_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK3_RDATA3_REG>;
#[doc = "Register EFUSE_BLK3_RDATA3_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK3_RDATA3_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_BLK3_DOUT3`"]
pub type EFUSE_BLK3_DOUT3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_BLK3_DOUT3`"]
pub struct EFUSE_BLK3_DOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLK3_DOUT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_ADC2_TP_HIGH`"]
pub type EFUSE_RD_ADC2_TP_HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_RD_ADC2_TP_HIGH`"]
pub struct EFUSE_RD_ADC2_TP_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_ADC2_TP_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_ADC2_TP_LOW`"]
pub type EFUSE_RD_ADC2_TP_LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_ADC2_TP_LOW`"]
pub struct EFUSE_RD_ADC2_TP_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_ADC2_TP_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_ADC1_TP_HIGH`"]
pub type EFUSE_RD_ADC1_TP_HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_RD_ADC1_TP_HIGH`"]
pub struct EFUSE_RD_ADC1_TP_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_ADC1_TP_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_ADC1_TP_LOW`"]
pub type EFUSE_RD_ADC1_TP_LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_ADC1_TP_LOW`"]
pub struct EFUSE_RD_ADC1_TP_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_ADC1_TP_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn efuse_blk3_dout3(&self) -> EFUSE_BLK3_DOUT3_R {
        EFUSE_BLK3_DOUT3_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc2_tp_high(&self) -> EFUSE_RD_ADC2_TP_HIGH_R {
        EFUSE_RD_ADC2_TP_HIGH_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc2_tp_low(&self) -> EFUSE_RD_ADC2_TP_LOW_R {
        EFUSE_RD_ADC2_TP_LOW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc1_tp_high(&self) -> EFUSE_RD_ADC1_TP_HIGH_R {
        EFUSE_RD_ADC1_TP_HIGH_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc1_tp_low(&self) -> EFUSE_RD_ADC1_TP_LOW_R {
        EFUSE_RD_ADC1_TP_LOW_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn efuse_blk3_dout3(&mut self) -> EFUSE_BLK3_DOUT3_W {
        EFUSE_BLK3_DOUT3_W { w: self }
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc2_tp_high(&mut self) -> EFUSE_RD_ADC2_TP_HIGH_W {
        EFUSE_RD_ADC2_TP_HIGH_W { w: self }
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc2_tp_low(&mut self) -> EFUSE_RD_ADC2_TP_LOW_W {
        EFUSE_RD_ADC2_TP_LOW_W { w: self }
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc1_tp_high(&mut self) -> EFUSE_RD_ADC1_TP_HIGH_W {
        EFUSE_RD_ADC1_TP_HIGH_W { w: self }
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn efuse_rd_adc1_tp_low(&mut self) -> EFUSE_RD_ADC1_TP_LOW_W {
        EFUSE_RD_ADC1_TP_LOW_W { w: self }
    }
}
