#[doc = "Reader of register SENS_SAR_DAC_CTRL2_REG"]
pub type R = crate::R<u32, super::SENS_SAR_DAC_CTRL2_REG>;
#[doc = "Writer for register SENS_SAR_DAC_CTRL2_REG"]
pub type W = crate::W<u32, super::SENS_SAR_DAC_CTRL2_REG>;
#[doc = "Register SENS_SAR_DAC_CTRL2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_DAC_CTRL2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_DAC_CW_EN2`"]
pub type SENS_DAC_CW_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_DAC_CW_EN2`"]
pub struct SENS_DAC_CW_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_CW_EN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_CW_EN1`"]
pub type SENS_DAC_CW_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_DAC_CW_EN1`"]
pub struct SENS_DAC_CW_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_CW_EN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_INV2`"]
pub type SENS_DAC_INV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_DAC_INV2`"]
pub struct SENS_DAC_INV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_INV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_INV1`"]
pub type SENS_DAC_INV1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_DAC_INV1`"]
pub struct SENS_DAC_INV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_INV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_SCALE2`"]
pub type SENS_DAC_SCALE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_DAC_SCALE2`"]
pub struct SENS_DAC_SCALE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_SCALE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_SCALE1`"]
pub type SENS_DAC_SCALE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_DAC_SCALE1`"]
pub struct SENS_DAC_SCALE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_SCALE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_DC2`"]
pub type SENS_DAC_DC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_DAC_DC2`"]
pub struct SENS_DAC_DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_DC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SENS_DAC_DC1`"]
pub type SENS_DAC_DC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_DAC_DC1`"]
pub struct SENS_DAC_DC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_DAC_DC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\] 0: to select register reg_pdac2_dac\\[7:0\\] as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn sens_dac_cw_en2(&self) -> SENS_DAC_CW_EN2_R {
        SENS_DAC_CW_EN2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\] 0: to select register reg_pdac1_dac\\[7:0\\] as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn sens_dac_cw_en1(&self) -> SENS_DAC_CW_EN1_R {
        SENS_DAC_CW_EN1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn sens_dac_inv2(&self) -> SENS_DAC_INV2_R {
        SENS_DAC_INV2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn sens_dac_inv1(&self) -> SENS_DAC_INV1_R {
        SENS_DAC_INV1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn sens_dac_scale2(&self) -> SENS_DAC_SCALE2_R {
        SENS_DAC_SCALE2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn sens_dac_scale1(&self) -> SENS_DAC_SCALE1_R {
        SENS_DAC_SCALE1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn sens_dac_dc2(&self) -> SENS_DAC_DC2_R {
        SENS_DAC_DC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn sens_dac_dc1(&self) -> SENS_DAC_DC1_R {
        SENS_DAC_DC1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\] 0: to select register reg_pdac2_dac\\[7:0\\] as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn sens_dac_cw_en2(&mut self) -> SENS_DAC_CW_EN2_W {
        SENS_DAC_CW_EN2_W { w: self }
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\] 0: to select register reg_pdac1_dac\\[7:0\\] as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn sens_dac_cw_en1(&mut self) -> SENS_DAC_CW_EN1_W {
        SENS_DAC_CW_EN1_W { w: self }
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn sens_dac_inv2(&mut self) -> SENS_DAC_INV2_W {
        SENS_DAC_INV2_W { w: self }
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn sens_dac_inv1(&mut self) -> SENS_DAC_INV1_W {
        SENS_DAC_INV1_W { w: self }
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn sens_dac_scale2(&mut self) -> SENS_DAC_SCALE2_W {
        SENS_DAC_SCALE2_W { w: self }
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn sens_dac_scale1(&mut self) -> SENS_DAC_SCALE1_W {
        SENS_DAC_SCALE1_W { w: self }
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn sens_dac_dc2(&mut self) -> SENS_DAC_DC2_W {
        SENS_DAC_DC2_W { w: self }
    }
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn sens_dac_dc1(&mut self) -> SENS_DAC_DC1_W {
        SENS_DAC_DC1_W { w: self }
    }
}
