#[doc = "Reader of register RTC_IO_ADC_PAD_REG"]
pub type R = crate::R<u32, super::RTC_IO_ADC_PAD_REG>;
#[doc = "Writer for register RTC_IO_ADC_PAD_REG"]
pub type W = crate::W<u32, super::RTC_IO_ADC_PAD_REG>;
#[doc = "Register RTC_IO_ADC_PAD_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_ADC_PAD_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_ADC1_HOLD`"]
pub type RTC_IO_ADC1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC1_HOLD`"]
pub struct RTC_IO_ADC1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC1_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_ADC2_HOLD`"]
pub type RTC_IO_ADC2_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC2_HOLD`"]
pub struct RTC_IO_ADC2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC2_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_ADC1_MUX_SEL`"]
pub type RTC_IO_ADC1_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC1_MUX_SEL`"]
pub struct RTC_IO_ADC1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC1_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_ADC2_MUX_SEL`"]
pub type RTC_IO_ADC2_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC2_MUX_SEL`"]
pub struct RTC_IO_ADC2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC2_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_ADC1_FUN_SEL`"]
pub type RTC_IO_ADC1_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_ADC1_FUN_SEL`"]
pub struct RTC_IO_ADC1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_ADC1_SLP_SEL`"]
pub type RTC_IO_ADC1_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC1_SLP_SEL`"]
pub struct RTC_IO_ADC1_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC1_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_ADC1_SLP_IE`"]
pub type RTC_IO_ADC1_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC1_SLP_IE`"]
pub struct RTC_IO_ADC1_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC1_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_ADC1_FUN_IE`"]
pub type RTC_IO_ADC1_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC1_FUN_IE`"]
pub struct RTC_IO_ADC1_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC1_FUN_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_ADC2_FUN_SEL`"]
pub type RTC_IO_ADC2_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_ADC2_FUN_SEL`"]
pub struct RTC_IO_ADC2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_ADC2_SLP_SEL`"]
pub type RTC_IO_ADC2_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC2_SLP_SEL`"]
pub struct RTC_IO_ADC2_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC2_SLP_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_ADC2_SLP_IE`"]
pub type RTC_IO_ADC2_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC2_SLP_IE`"]
pub struct RTC_IO_ADC2_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC2_SLP_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_ADC2_FUN_IE`"]
pub type RTC_IO_ADC2_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_ADC2_FUN_IE`"]
pub struct RTC_IO_ADC2_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_ADC2_FUN_IE_W<'a> {
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
impl R {
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_adc1_hold(&self) -> RTC_IO_ADC1_HOLD_R {
        RTC_IO_ADC1_HOLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_adc2_hold(&self) -> RTC_IO_ADC2_HOLD_R {
        RTC_IO_ADC2_HOLD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_adc1_mux_sel(&self) -> RTC_IO_ADC1_MUX_SEL_R {
        RTC_IO_ADC1_MUX_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_adc2_mux_sel(&self) -> RTC_IO_ADC2_MUX_SEL_R {
        RTC_IO_ADC2_MUX_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc1_fun_sel(&self) -> RTC_IO_ADC1_FUN_SEL_R {
        RTC_IO_ADC1_FUN_SEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc1_slp_sel(&self) -> RTC_IO_ADC1_SLP_SEL_R {
        RTC_IO_ADC1_SLP_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_adc1_slp_ie(&self) -> RTC_IO_ADC1_SLP_IE_R {
        RTC_IO_ADC1_SLP_IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc1_fun_ie(&self) -> RTC_IO_ADC1_FUN_IE_R {
        RTC_IO_ADC1_FUN_IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc2_fun_sel(&self) -> RTC_IO_ADC2_FUN_SEL_R {
        RTC_IO_ADC2_FUN_SEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc2_slp_sel(&self) -> RTC_IO_ADC2_SLP_SEL_R {
        RTC_IO_ADC2_SLP_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_adc2_slp_ie(&self) -> RTC_IO_ADC2_SLP_IE_R {
        RTC_IO_ADC2_SLP_IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc2_fun_ie(&self) -> RTC_IO_ADC2_FUN_IE_R {
        RTC_IO_ADC2_FUN_IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_adc1_hold(&mut self) -> RTC_IO_ADC1_HOLD_W {
        RTC_IO_ADC1_HOLD_W { w: self }
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_adc2_hold(&mut self) -> RTC_IO_ADC2_HOLD_W {
        RTC_IO_ADC2_HOLD_W { w: self }
    }
    #[doc = "Bit 29 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_adc1_mux_sel(&mut self) -> RTC_IO_ADC1_MUX_SEL_W {
        RTC_IO_ADC1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 28 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_adc2_mux_sel(&mut self) -> RTC_IO_ADC2_MUX_SEL_W {
        RTC_IO_ADC2_MUX_SEL_W { w: self }
    }
    #[doc = "Bits 26:27 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc1_fun_sel(&mut self) -> RTC_IO_ADC1_FUN_SEL_W {
        RTC_IO_ADC1_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 25 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc1_slp_sel(&mut self) -> RTC_IO_ADC1_SLP_SEL_W {
        RTC_IO_ADC1_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 24 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_adc1_slp_ie(&mut self) -> RTC_IO_ADC1_SLP_IE_W {
        RTC_IO_ADC1_SLP_IE_W { w: self }
    }
    #[doc = "Bit 23 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc1_fun_ie(&mut self) -> RTC_IO_ADC1_FUN_IE_W {
        RTC_IO_ADC1_FUN_IE_W { w: self }
    }
    #[doc = "Bits 21:22 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc2_fun_sel(&mut self) -> RTC_IO_ADC2_FUN_SEL_W {
        RTC_IO_ADC2_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 20 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc2_slp_sel(&mut self) -> RTC_IO_ADC2_SLP_SEL_W {
        RTC_IO_ADC2_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 19 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_adc2_slp_ie(&mut self) -> RTC_IO_ADC2_SLP_IE_W {
        RTC_IO_ADC2_SLP_IE_W { w: self }
    }
    #[doc = "Bit 18 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_adc2_fun_ie(&mut self) -> RTC_IO_ADC2_FUN_IE_W {
        RTC_IO_ADC2_FUN_IE_W { w: self }
    }
}
