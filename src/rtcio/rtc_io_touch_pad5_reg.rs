#[doc = "Reader of register RTC_IO_TOUCH_PAD5_REG"]
pub type R = crate::R<u32, super::RTC_IO_TOUCH_PAD5_REG>;
#[doc = "Writer for register RTC_IO_TOUCH_PAD5_REG"]
pub type W = crate::W<u32, super::RTC_IO_TOUCH_PAD5_REG>;
#[doc = "Register RTC_IO_TOUCH_PAD5_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_TOUCH_PAD5_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_HOLD`"]
pub type RTC_IO_TOUCH_PAD5_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_HOLD`"]
pub struct RTC_IO_TOUCH_PAD5_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_DRV`"]
pub type RTC_IO_TOUCH_PAD5_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_DRV`"]
pub struct RTC_IO_TOUCH_PAD5_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_RDE`"]
pub type RTC_IO_TOUCH_PAD5_RDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_RDE`"]
pub struct RTC_IO_TOUCH_PAD5_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_RDE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_RUE`"]
pub type RTC_IO_TOUCH_PAD5_RUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_RUE`"]
pub struct RTC_IO_TOUCH_PAD5_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_RUE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_DAC`"]
pub type RTC_IO_TOUCH_PAD5_DAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_DAC`"]
pub struct RTC_IO_TOUCH_PAD5_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_START`"]
pub type RTC_IO_TOUCH_PAD5_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_START`"]
pub struct RTC_IO_TOUCH_PAD5_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_START_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_TIE_OPT`"]
pub type RTC_IO_TOUCH_PAD5_TIE_OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_TIE_OPT`"]
pub struct RTC_IO_TOUCH_PAD5_TIE_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_TIE_OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_XPD`"]
pub type RTC_IO_TOUCH_PAD5_XPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_XPD`"]
pub struct RTC_IO_TOUCH_PAD5_XPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_XPD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_MUX_SEL`"]
pub type RTC_IO_TOUCH_PAD5_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_MUX_SEL`"]
pub struct RTC_IO_TOUCH_PAD5_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_FUN_SEL`"]
pub type RTC_IO_TOUCH_PAD5_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_FUN_SEL`"]
pub struct RTC_IO_TOUCH_PAD5_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_SLP_SEL`"]
pub type RTC_IO_TOUCH_PAD5_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_SLP_SEL`"]
pub struct RTC_IO_TOUCH_PAD5_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_SLP_IE`"]
pub type RTC_IO_TOUCH_PAD5_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_SLP_IE`"]
pub struct RTC_IO_TOUCH_PAD5_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_SLP_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_SLP_OE`"]
pub type RTC_IO_TOUCH_PAD5_SLP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_SLP_OE`"]
pub struct RTC_IO_TOUCH_PAD5_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_SLP_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_FUN_IE`"]
pub type RTC_IO_TOUCH_PAD5_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_FUN_IE`"]
pub struct RTC_IO_TOUCH_PAD5_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_FUN_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD5_TO_GPIO`"]
pub type RTC_IO_TOUCH_PAD5_TO_GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD5_TO_GPIO`"]
pub struct RTC_IO_TOUCH_PAD5_TO_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD5_TO_GPIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_hold(&self) -> RTC_IO_TOUCH_PAD5_HOLD_R {
        RTC_IO_TOUCH_PAD5_HOLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_drv(&self) -> RTC_IO_TOUCH_PAD5_DRV_R {
        RTC_IO_TOUCH_PAD5_DRV_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_rde(&self) -> RTC_IO_TOUCH_PAD5_RDE_R {
        RTC_IO_TOUCH_PAD5_RDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_rue(&self) -> RTC_IO_TOUCH_PAD5_RUE_R {
        RTC_IO_TOUCH_PAD5_RUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_dac(&self) -> RTC_IO_TOUCH_PAD5_DAC_R {
        RTC_IO_TOUCH_PAD5_DAC_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_start(&self) -> RTC_IO_TOUCH_PAD5_START_R {
        RTC_IO_TOUCH_PAD5_START_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_tie_opt(&self) -> RTC_IO_TOUCH_PAD5_TIE_OPT_R {
        RTC_IO_TOUCH_PAD5_TIE_OPT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_xpd(&self) -> RTC_IO_TOUCH_PAD5_XPD_R {
        RTC_IO_TOUCH_PAD5_XPD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_mux_sel(&self) -> RTC_IO_TOUCH_PAD5_MUX_SEL_R {
        RTC_IO_TOUCH_PAD5_MUX_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_fun_sel(&self) -> RTC_IO_TOUCH_PAD5_FUN_SEL_R {
        RTC_IO_TOUCH_PAD5_FUN_SEL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_slp_sel(&self) -> RTC_IO_TOUCH_PAD5_SLP_SEL_R {
        RTC_IO_TOUCH_PAD5_SLP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_slp_ie(&self) -> RTC_IO_TOUCH_PAD5_SLP_IE_R {
        RTC_IO_TOUCH_PAD5_SLP_IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_slp_oe(&self) -> RTC_IO_TOUCH_PAD5_SLP_OE_R {
        RTC_IO_TOUCH_PAD5_SLP_OE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_fun_ie(&self) -> RTC_IO_TOUCH_PAD5_FUN_IE_R {
        RTC_IO_TOUCH_PAD5_FUN_IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - connect the rtc pad input to digital pad input \u{d3}0\u{d3} is availbale.MTDI"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_to_gpio(&self) -> RTC_IO_TOUCH_PAD5_TO_GPIO_R {
        RTC_IO_TOUCH_PAD5_TO_GPIO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_hold(&mut self) -> RTC_IO_TOUCH_PAD5_HOLD_W {
        RTC_IO_TOUCH_PAD5_HOLD_W { w: self }
    }
    #[doc = "Bits 29:30 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_drv(&mut self) -> RTC_IO_TOUCH_PAD5_DRV_W {
        RTC_IO_TOUCH_PAD5_DRV_W { w: self }
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_rde(&mut self) -> RTC_IO_TOUCH_PAD5_RDE_W {
        RTC_IO_TOUCH_PAD5_RDE_W { w: self }
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_rue(&mut self) -> RTC_IO_TOUCH_PAD5_RUE_W {
        RTC_IO_TOUCH_PAD5_RUE_W { w: self }
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_dac(&mut self) -> RTC_IO_TOUCH_PAD5_DAC_W {
        RTC_IO_TOUCH_PAD5_DAC_W { w: self }
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_start(&mut self) -> RTC_IO_TOUCH_PAD5_START_W {
        RTC_IO_TOUCH_PAD5_START_W { w: self }
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_tie_opt(&mut self) -> RTC_IO_TOUCH_PAD5_TIE_OPT_W {
        RTC_IO_TOUCH_PAD5_TIE_OPT_W { w: self }
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_xpd(&mut self) -> RTC_IO_TOUCH_PAD5_XPD_W {
        RTC_IO_TOUCH_PAD5_XPD_W { w: self }
    }
    #[doc = "Bit 19 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_mux_sel(&mut self) -> RTC_IO_TOUCH_PAD5_MUX_SEL_W {
        RTC_IO_TOUCH_PAD5_MUX_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_fun_sel(&mut self) -> RTC_IO_TOUCH_PAD5_FUN_SEL_W {
        RTC_IO_TOUCH_PAD5_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_slp_sel(&mut self) -> RTC_IO_TOUCH_PAD5_SLP_SEL_W {
        RTC_IO_TOUCH_PAD5_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_slp_ie(&mut self) -> RTC_IO_TOUCH_PAD5_SLP_IE_W {
        RTC_IO_TOUCH_PAD5_SLP_IE_W { w: self }
    }
    #[doc = "Bit 14 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_slp_oe(&mut self) -> RTC_IO_TOUCH_PAD5_SLP_OE_W {
        RTC_IO_TOUCH_PAD5_SLP_OE_W { w: self }
    }
    #[doc = "Bit 13 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_fun_ie(&mut self) -> RTC_IO_TOUCH_PAD5_FUN_IE_W {
        RTC_IO_TOUCH_PAD5_FUN_IE_W { w: self }
    }
    #[doc = "Bit 12 - connect the rtc pad input to digital pad input \u{d3}0\u{d3} is availbale.MTDI"]
    #[inline(always)]
    pub fn rtc_io_touch_pad5_to_gpio(&mut self) -> RTC_IO_TOUCH_PAD5_TO_GPIO_W {
        RTC_IO_TOUCH_PAD5_TO_GPIO_W { w: self }
    }
}
