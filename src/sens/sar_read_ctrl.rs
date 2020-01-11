#[doc = "Reader of register SAR_READ_CTRL"]
pub type R = crate::R<u32, super::SAR_READ_CTRL>;
#[doc = "Writer for register SAR_READ_CTRL"]
pub type W = crate::W<u32, super::SAR_READ_CTRL>;
#[doc = "Register SAR_READ_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_READ_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR1_DATA_INV`"]
pub type SENS_SAR1_DATA_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR1_DATA_INV`"]
pub struct SENS_SAR1_DATA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_DATA_INV_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_DIG_FORCE`"]
pub type SENS_SAR1_DIG_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR1_DIG_FORCE`"]
pub struct SENS_SAR1_DIG_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_DIG_FORCE_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_SAMPLE_NUM`"]
pub type SENS_SAR1_SAMPLE_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR1_SAMPLE_NUM`"]
pub struct SENS_SAR1_SAMPLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_SAMPLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 19)) | (((value as u32) & 0xff) << 19);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR1_CLK_GATED`"]
pub type SENS_SAR1_CLK_GATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR1_CLK_GATED`"]
pub struct SENS_SAR1_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_CLK_GATED_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_SAMPLE_BIT`"]
pub type SENS_SAR1_SAMPLE_BIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR1_SAMPLE_BIT`"]
pub struct SENS_SAR1_SAMPLE_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_SAMPLE_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR1_SAMPLE_CYCLE`"]
pub type SENS_SAR1_SAMPLE_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR1_SAMPLE_CYCLE`"]
pub struct SENS_SAR1_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR1_CLK_DIV`"]
pub type SENS_SAR1_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR1_CLK_DIV`"]
pub struct SENS_SAR1_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Invert SAR ADC1 data"]
    #[inline(always)]
    pub fn sens_sar1_data_inv(&self) -> SENS_SAR1_DATA_INV_R {
        SENS_SAR1_DATA_INV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1: SAR ADC1 controlled by DIG ADC1 CTRL 0: SAR ADC1 controlled by RTC ADC1 CTRL"]
    #[inline(always)]
    pub fn sens_sar1_dig_force(&self) -> SENS_SAR1_DIG_FORCE_R {
        SENS_SAR1_DIG_FORCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sens_sar1_sample_num(&self) -> SENS_SAR1_SAMPLE_NUM_R {
        SENS_SAR1_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sens_sar1_clk_gated(&self) -> SENS_SAR1_CLK_GATED_R {
        SENS_SAR1_CLK_GATED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sens_sar1_sample_bit(&self) -> SENS_SAR1_SAMPLE_BIT_R {
        SENS_SAR1_SAMPLE_BIT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC1"]
    #[inline(always)]
    pub fn sens_sar1_sample_cycle(&self) -> SENS_SAR1_SAMPLE_CYCLE_R {
        SENS_SAR1_SAMPLE_CYCLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sens_sar1_clk_div(&self) -> SENS_SAR1_CLK_DIV_R {
        SENS_SAR1_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 28 - Invert SAR ADC1 data"]
    #[inline(always)]
    pub fn sens_sar1_data_inv(&mut self) -> SENS_SAR1_DATA_INV_W {
        SENS_SAR1_DATA_INV_W { w: self }
    }
    #[doc = "Bit 27 - 1: SAR ADC1 controlled by DIG ADC1 CTRL 0: SAR ADC1 controlled by RTC ADC1 CTRL"]
    #[inline(always)]
    pub fn sens_sar1_dig_force(&mut self) -> SENS_SAR1_DIG_FORCE_W {
        SENS_SAR1_DIG_FORCE_W { w: self }
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sens_sar1_sample_num(&mut self) -> SENS_SAR1_SAMPLE_NUM_W {
        SENS_SAR1_SAMPLE_NUM_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sens_sar1_clk_gated(&mut self) -> SENS_SAR1_CLK_GATED_W {
        SENS_SAR1_CLK_GATED_W { w: self }
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sens_sar1_sample_bit(&mut self) -> SENS_SAR1_SAMPLE_BIT_W {
        SENS_SAR1_SAMPLE_BIT_W { w: self }
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC1"]
    #[inline(always)]
    pub fn sens_sar1_sample_cycle(&mut self) -> SENS_SAR1_SAMPLE_CYCLE_W {
        SENS_SAR1_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sens_sar1_clk_div(&mut self) -> SENS_SAR1_CLK_DIV_W {
        SENS_SAR1_CLK_DIV_W { w: self }
    }
}
