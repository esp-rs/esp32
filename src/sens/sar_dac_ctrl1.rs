#[doc = "Reader of register SAR_DAC_CTRL1"]
pub type R = crate::R<u32, super::SAR_DAC_CTRL1>;
#[doc = "Writer for register SAR_DAC_CTRL1"]
pub type W = crate::W<u32, super::SAR_DAC_CTRL1>;
#[doc = "Register SAR_DAC_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_DAC_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAC_CLK_INV`"]
pub type DAC_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_CLK_INV`"]
pub struct DAC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_INV_W<'a> {
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
#[doc = "Reader of field `DAC_CLK_FORCE_HIGH`"]
pub type DAC_CLK_FORCE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_CLK_FORCE_HIGH`"]
pub struct DAC_CLK_FORCE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_FORCE_HIGH_W<'a> {
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
#[doc = "Reader of field `DAC_CLK_FORCE_LOW`"]
pub type DAC_CLK_FORCE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_CLK_FORCE_LOW`"]
pub struct DAC_CLK_FORCE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_FORCE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `DAC_DIG_FORCE`"]
pub type DAC_DIG_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_DIG_FORCE`"]
pub struct DAC_DIG_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DIG_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_BIT_SEL`"]
pub type DEBUG_BIT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_BIT_SEL`"]
pub struct DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `SW_TONE_EN`"]
pub type SW_TONE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_TONE_EN`"]
pub struct SW_TONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TONE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_FSTEP`"]
pub type SW_FSTEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SW_FSTEP`"]
pub struct SW_FSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_FSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - 1: invert PDAC_CLK"]
    #[inline(always)]
    pub fn dac_clk_inv(&self) -> DAC_CLK_INV_R {
        DAC_CLK_INV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    pub fn dac_clk_force_high(&self) -> DAC_CLK_FORCE_HIGH_R {
        DAC_CLK_FORCE_HIGH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    pub fn dac_clk_force_low(&self) -> DAC_CLK_FORCE_LOW_R {
        DAC_CLK_FORCE_LOW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: DAC1 & DAC2 use DMA 0: DAC1 & DAC2 do not use DMA"]
    #[inline(always)]
    pub fn dac_dig_force(&self) -> DAC_DIG_FORCE_R {
        DAC_DIG_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 1: enable CW generator 0: disable CW generator"]
    #[inline(always)]
    pub fn sw_tone_en(&self) -> SW_TONE_EN_R {
        SW_TONE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - frequency step for CW generator can be used to adjust the frequency"]
    #[inline(always)]
    pub fn sw_fstep(&self) -> SW_FSTEP_R {
        SW_FSTEP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 25 - 1: invert PDAC_CLK"]
    #[inline(always)]
    pub fn dac_clk_inv(&mut self) -> DAC_CLK_INV_W {
        DAC_CLK_INV_W { w: self }
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    pub fn dac_clk_force_high(&mut self) -> DAC_CLK_FORCE_HIGH_W {
        DAC_CLK_FORCE_HIGH_W { w: self }
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    pub fn dac_clk_force_low(&mut self) -> DAC_CLK_FORCE_LOW_W {
        DAC_CLK_FORCE_LOW_W { w: self }
    }
    #[doc = "Bit 22 - 1: DAC1 & DAC2 use DMA 0: DAC1 & DAC2 do not use DMA"]
    #[inline(always)]
    pub fn dac_dig_force(&mut self) -> DAC_DIG_FORCE_W {
        DAC_DIG_FORCE_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W {
        DEBUG_BIT_SEL_W { w: self }
    }
    #[doc = "Bit 16 - 1: enable CW generator 0: disable CW generator"]
    #[inline(always)]
    pub fn sw_tone_en(&mut self) -> SW_TONE_EN_W {
        SW_TONE_EN_W { w: self }
    }
    #[doc = "Bits 0:15 - frequency step for CW generator can be used to adjust the frequency"]
    #[inline(always)]
    pub fn sw_fstep(&mut self) -> SW_FSTEP_W {
        SW_FSTEP_W { w: self }
    }
}
