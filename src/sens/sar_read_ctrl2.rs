#[doc = "Reader of register SAR_READ_CTRL2"]
pub type R = crate::R<u32, super::SAR_READ_CTRL2>;
#[doc = "Writer for register SAR_READ_CTRL2"]
pub type W = crate::W<u32, super::SAR_READ_CTRL2>;
#[doc = "Register SAR_READ_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_READ_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR2_DATA_INV`"]
pub type SAR2_DATA_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR2_DATA_INV`"]
pub struct SAR2_DATA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_DATA_INV_W<'a> {
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
#[doc = "Reader of field `SAR2_DIG_FORCE`"]
pub type SAR2_DIG_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR2_DIG_FORCE`"]
pub struct SAR2_DIG_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_DIG_FORCE_W<'a> {
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
#[doc = "Reader of field `SAR2_PWDET_FORCE`"]
pub type SAR2_PWDET_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR2_PWDET_FORCE`"]
pub struct SAR2_PWDET_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SAR2_SAMPLE_NUM`"]
pub type SAR2_SAMPLE_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_SAMPLE_NUM`"]
pub struct SAR2_SAMPLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_SAMPLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 19)) | (((value as u32) & 0xff) << 19);
        self.w
    }
}
#[doc = "Reader of field `SAR2_CLK_GATED`"]
pub type SAR2_CLK_GATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR2_CLK_GATED`"]
pub struct SAR2_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_CLK_GATED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SAR2_SAMPLE_BIT`"]
pub type SAR2_SAMPLE_BIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_SAMPLE_BIT`"]
pub struct SAR2_SAMPLE_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_SAMPLE_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SAR2_SAMPLE_CYCLE`"]
pub type SAR2_SAMPLE_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_SAMPLE_CYCLE`"]
pub struct SAR2_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SAR2_CLK_DIV`"]
pub type SAR2_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_CLK_DIV`"]
pub struct SAR2_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&self) -> SAR2_DATA_INV_R {
        SAR2_DATA_INV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&self) -> SAR2_DIG_FORCE_R {
        SAR2_DIG_FORCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&self) -> SAR2_PWDET_FORCE_R {
        SAR2_PWDET_FORCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&self) -> SAR2_SAMPLE_NUM_R {
        SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&self) -> SAR2_CLK_GATED_R {
        SAR2_CLK_GATED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&self) -> SAR2_SAMPLE_BIT_R {
        SAR2_SAMPLE_BIT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&self) -> SAR2_SAMPLE_CYCLE_R {
        SAR2_SAMPLE_CYCLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&self) -> SAR2_CLK_DIV_R {
        SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&mut self) -> SAR2_DATA_INV_W {
        SAR2_DATA_INV_W { w: self }
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&mut self) -> SAR2_DIG_FORCE_W {
        SAR2_DIG_FORCE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&mut self) -> SAR2_PWDET_FORCE_W {
        SAR2_PWDET_FORCE_W { w: self }
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&mut self) -> SAR2_SAMPLE_NUM_W {
        SAR2_SAMPLE_NUM_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&mut self) -> SAR2_CLK_GATED_W {
        SAR2_CLK_GATED_W { w: self }
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&mut self) -> SAR2_SAMPLE_BIT_W {
        SAR2_SAMPLE_BIT_W { w: self }
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&mut self) -> SAR2_SAMPLE_CYCLE_W {
        SAR2_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&mut self) -> SAR2_CLK_DIV_W {
        SAR2_CLK_DIV_W { w: self }
    }
}
