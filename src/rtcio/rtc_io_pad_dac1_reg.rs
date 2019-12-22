#[doc = "Reader of register RTC_IO_PAD_DAC1_REG"]
pub type R = crate::R<u32, super::RTC_IO_PAD_DAC1_REG>;
#[doc = "Writer for register RTC_IO_PAD_DAC1_REG"]
pub type W = crate::W<u32, super::RTC_IO_PAD_DAC1_REG>;
#[doc = "Register RTC_IO_PAD_DAC1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_PAD_DAC1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_PDAC1_DRV`"]
pub type RTC_IO_PDAC1_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_DRV`"]
pub struct RTC_IO_PDAC1_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_PDAC1_HOLD`"]
pub type RTC_IO_PDAC1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_HOLD`"]
pub struct RTC_IO_PDAC1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_RDE`"]
pub type RTC_IO_PDAC1_RDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_RDE`"]
pub struct RTC_IO_PDAC1_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_RDE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_RUE`"]
pub type RTC_IO_PDAC1_RUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_RUE`"]
pub struct RTC_IO_PDAC1_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_RUE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_DAC`"]
pub type RTC_IO_PDAC1_DAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_DAC`"]
pub struct RTC_IO_PDAC1_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 19)) | (((value as u32) & 0xff) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_PDAC1_XPD_DAC`"]
pub type RTC_IO_PDAC1_XPD_DAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_XPD_DAC`"]
pub struct RTC_IO_PDAC1_XPD_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_XPD_DAC_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_MUX_SEL`"]
pub type RTC_IO_PDAC1_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_MUX_SEL`"]
pub struct RTC_IO_PDAC1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_MUX_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_PDAC1_FUN_SEL`"]
pub type RTC_IO_PDAC1_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_FUN_SEL`"]
pub struct RTC_IO_PDAC1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_PDAC1_SLP_SEL`"]
pub type RTC_IO_PDAC1_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_SLP_SEL`"]
pub struct RTC_IO_PDAC1_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_SLP_IE`"]
pub type RTC_IO_PDAC1_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_SLP_IE`"]
pub struct RTC_IO_PDAC1_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_SLP_OE`"]
pub type RTC_IO_PDAC1_SLP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_SLP_OE`"]
pub struct RTC_IO_PDAC1_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_SLP_OE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_PDAC1_FUN_IE`"]
pub type RTC_IO_PDAC1_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_FUN_IE`"]
pub struct RTC_IO_PDAC1_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_FUN_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_PDAC1_DAC_XPD_FORCE`"]
pub type RTC_IO_PDAC1_DAC_XPD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_PDAC1_DAC_XPD_FORCE`"]
pub struct RTC_IO_PDAC1_DAC_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_PDAC1_DAC_XPD_FORCE_W<'a> {
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
impl R {
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_drv(&self) -> RTC_IO_PDAC1_DRV_R {
        RTC_IO_PDAC1_DRV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_pdac1_hold(&self) -> RTC_IO_PDAC1_HOLD_R {
        RTC_IO_PDAC1_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_rde(&self) -> RTC_IO_PDAC1_RDE_R {
        RTC_IO_PDAC1_RDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_rue(&self) -> RTC_IO_PDAC1_RUE_R {
        RTC_IO_PDAC1_RUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 19:26 - PAD DAC1 control code."]
    #[inline(always)]
    pub fn rtc_io_pdac1_dac(&self) -> RTC_IO_PDAC1_DAC_R {
        RTC_IO_PDAC1_DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn rtc_io_pdac1_xpd_dac(&self) -> RTC_IO_PDAC1_XPD_DAC_R {
        RTC_IO_PDAC1_XPD_DAC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_pdac1_mux_sel(&self) -> RTC_IO_PDAC1_MUX_SEL_R {
        RTC_IO_PDAC1_MUX_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_fun_sel(&self) -> RTC_IO_PDAC1_FUN_SEL_R {
        RTC_IO_PDAC1_FUN_SEL_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_slp_sel(&self) -> RTC_IO_PDAC1_SLP_SEL_R {
        RTC_IO_PDAC1_SLP_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_pdac1_slp_ie(&self) -> RTC_IO_PDAC1_SLP_IE_R {
        RTC_IO_PDAC1_SLP_IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_pdac1_slp_oe(&self) -> RTC_IO_PDAC1_SLP_OE_R {
        RTC_IO_PDAC1_SLP_OE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_fun_ie(&self) -> RTC_IO_PDAC1_FUN_IE_R {
        RTC_IO_PDAC1_FUN_IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn rtc_io_pdac1_dac_xpd_force(&self) -> RTC_IO_PDAC1_DAC_XPD_FORCE_R {
        RTC_IO_PDAC1_DAC_XPD_FORCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_drv(&mut self) -> RTC_IO_PDAC1_DRV_W {
        RTC_IO_PDAC1_DRV_W { w: self }
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_pdac1_hold(&mut self) -> RTC_IO_PDAC1_HOLD_W {
        RTC_IO_PDAC1_HOLD_W { w: self }
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_rde(&mut self) -> RTC_IO_PDAC1_RDE_W {
        RTC_IO_PDAC1_RDE_W { w: self }
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_rue(&mut self) -> RTC_IO_PDAC1_RUE_W {
        RTC_IO_PDAC1_RUE_W { w: self }
    }
    #[doc = "Bits 19:26 - PAD DAC1 control code."]
    #[inline(always)]
    pub fn rtc_io_pdac1_dac(&mut self) -> RTC_IO_PDAC1_DAC_W {
        RTC_IO_PDAC1_DAC_W { w: self }
    }
    #[doc = "Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn rtc_io_pdac1_xpd_dac(&mut self) -> RTC_IO_PDAC1_XPD_DAC_W {
        RTC_IO_PDAC1_XPD_DAC_W { w: self }
    }
    #[doc = "Bit 17 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_pdac1_mux_sel(&mut self) -> RTC_IO_PDAC1_MUX_SEL_W {
        RTC_IO_PDAC1_MUX_SEL_W { w: self }
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_fun_sel(&mut self) -> RTC_IO_PDAC1_FUN_SEL_W {
        RTC_IO_PDAC1_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_slp_sel(&mut self) -> RTC_IO_PDAC1_SLP_SEL_W {
        RTC_IO_PDAC1_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_pdac1_slp_ie(&mut self) -> RTC_IO_PDAC1_SLP_IE_W {
        RTC_IO_PDAC1_SLP_IE_W { w: self }
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_pdac1_slp_oe(&mut self) -> RTC_IO_PDAC1_SLP_OE_W {
        RTC_IO_PDAC1_SLP_OE_W { w: self }
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_pdac1_fun_ie(&mut self) -> RTC_IO_PDAC1_FUN_IE_W {
        RTC_IO_PDAC1_FUN_IE_W { w: self }
    }
    #[doc = "Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn rtc_io_pdac1_dac_xpd_force(&mut self) -> RTC_IO_PDAC1_DAC_XPD_FORCE_W {
        RTC_IO_PDAC1_DAC_XPD_FORCE_W { w: self }
    }
}
