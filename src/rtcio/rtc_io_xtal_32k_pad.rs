#[doc = "Reader of register RTC_IO_XTAL_32K_PAD"]
pub type R = crate::R<u32, super::RTC_IO_XTAL_32K_PAD>;
#[doc = "Writer for register RTC_IO_XTAL_32K_PAD"]
pub type W = crate::W<u32, super::RTC_IO_XTAL_32K_PAD>;
#[doc = "Register RTC_IO_XTAL_32K_PAD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_XTAL_32K_PAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_X32N_DRV`"]
pub type RTC_IO_X32N_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_X32N_DRV`"]
pub struct RTC_IO_X32N_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32N_HOLD`"]
pub type RTC_IO_X32N_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_HOLD`"]
pub struct RTC_IO_X32N_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_RDE`"]
pub type RTC_IO_X32N_RDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_RDE`"]
pub struct RTC_IO_X32N_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_RDE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_RUE`"]
pub type RTC_IO_X32N_RUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_RUE`"]
pub struct RTC_IO_X32N_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_RUE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32P_DRV`"]
pub type RTC_IO_X32P_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_X32P_DRV`"]
pub struct RTC_IO_X32P_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32P_HOLD`"]
pub type RTC_IO_X32P_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_HOLD`"]
pub struct RTC_IO_X32P_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32P_RDE`"]
pub type RTC_IO_X32P_RDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_RDE`"]
pub struct RTC_IO_X32P_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_RDE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32P_RUE`"]
pub type RTC_IO_X32P_RUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_RUE`"]
pub struct RTC_IO_X32P_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_RUE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_DAC_XTAL_32K`"]
pub type RTC_IO_DAC_XTAL_32K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DAC_XTAL_32K`"]
pub struct RTC_IO_DAC_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DAC_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_XPD_XTAL_32K`"]
pub type RTC_IO_XPD_XTAL_32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_XPD_XTAL_32K`"]
pub struct RTC_IO_XPD_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_XPD_XTAL_32K_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_MUX_SEL`"]
pub type RTC_IO_X32N_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_MUX_SEL`"]
pub struct RTC_IO_X32N_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32P_MUX_SEL`"]
pub type RTC_IO_X32P_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_MUX_SEL`"]
pub struct RTC_IO_X32P_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_FUN_SEL`"]
pub type RTC_IO_X32N_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_X32N_FUN_SEL`"]
pub struct RTC_IO_X32N_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32N_SLP_SEL`"]
pub type RTC_IO_X32N_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_SLP_SEL`"]
pub struct RTC_IO_X32N_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_SLP_IE`"]
pub type RTC_IO_X32N_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_SLP_IE`"]
pub struct RTC_IO_X32N_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_SLP_OE`"]
pub type RTC_IO_X32N_SLP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_SLP_OE`"]
pub struct RTC_IO_X32N_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_SLP_OE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32N_FUN_IE`"]
pub type RTC_IO_X32N_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32N_FUN_IE`"]
pub struct RTC_IO_X32N_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32N_FUN_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_X32P_FUN_SEL`"]
pub type RTC_IO_X32P_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_X32P_FUN_SEL`"]
pub struct RTC_IO_X32P_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32P_SLP_SEL`"]
pub type RTC_IO_X32P_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_SLP_SEL`"]
pub struct RTC_IO_X32P_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_SLP_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32P_SLP_IE`"]
pub type RTC_IO_X32P_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_SLP_IE`"]
pub struct RTC_IO_X32P_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_SLP_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32P_SLP_OE`"]
pub type RTC_IO_X32P_SLP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_SLP_OE`"]
pub struct RTC_IO_X32P_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_SLP_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_X32P_FUN_IE`"]
pub type RTC_IO_X32P_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_X32P_FUN_IE`"]
pub struct RTC_IO_X32P_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_X32P_FUN_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DRES_XTAL_32K`"]
pub type RTC_IO_DRES_XTAL_32K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DRES_XTAL_32K`"]
pub struct RTC_IO_DRES_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DRES_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DBIAS_XTAL_32K`"]
pub type RTC_IO_DBIAS_XTAL_32K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DBIAS_XTAL_32K`"]
pub struct RTC_IO_DBIAS_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DBIAS_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_drv(&self) -> RTC_IO_X32N_DRV_R {
        RTC_IO_X32N_DRV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_x32n_hold(&self) -> RTC_IO_X32N_HOLD_R {
        RTC_IO_X32N_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_rde(&self) -> RTC_IO_X32N_RDE_R {
        RTC_IO_X32N_RDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_rue(&self) -> RTC_IO_X32N_RUE_R {
        RTC_IO_X32N_RUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_drv(&self) -> RTC_IO_X32P_DRV_R {
        RTC_IO_X32P_DRV_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_x32p_hold(&self) -> RTC_IO_X32P_HOLD_R {
        RTC_IO_X32P_HOLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_rde(&self) -> RTC_IO_X32P_RDE_R {
        RTC_IO_X32P_RDE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_rue(&self) -> RTC_IO_X32P_RUE_R {
        RTC_IO_X32P_RUE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - 32K XTAL bias current DAC."]
    #[inline(always)]
    pub fn rtc_io_dac_xtal_32k(&self) -> RTC_IO_DAC_XTAL_32K_R {
        RTC_IO_DAC_XTAL_32K_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Power up 32kHz crystal oscillator"]
    #[inline(always)]
    pub fn rtc_io_xpd_xtal_32k(&self) -> RTC_IO_XPD_XTAL_32K_R {
        RTC_IO_XPD_XTAL_32K_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_x32n_mux_sel(&self) -> RTC_IO_X32N_MUX_SEL_R {
        RTC_IO_X32N_MUX_SEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_x32p_mux_sel(&self) -> RTC_IO_X32P_MUX_SEL_R {
        RTC_IO_X32P_MUX_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_fun_sel(&self) -> RTC_IO_X32N_FUN_SEL_R {
        RTC_IO_X32N_FUN_SEL_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_slp_sel(&self) -> RTC_IO_X32N_SLP_SEL_R {
        RTC_IO_X32N_SLP_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32n_slp_ie(&self) -> RTC_IO_X32N_SLP_IE_R {
        RTC_IO_X32N_SLP_IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32n_slp_oe(&self) -> RTC_IO_X32N_SLP_OE_R {
        RTC_IO_X32N_SLP_OE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_fun_ie(&self) -> RTC_IO_X32N_FUN_IE_R {
        RTC_IO_X32N_FUN_IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_fun_sel(&self) -> RTC_IO_X32P_FUN_SEL_R {
        RTC_IO_X32P_FUN_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_slp_sel(&self) -> RTC_IO_X32P_SLP_SEL_R {
        RTC_IO_X32P_SLP_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32p_slp_ie(&self) -> RTC_IO_X32P_SLP_IE_R {
        RTC_IO_X32P_SLP_IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32p_slp_oe(&self) -> RTC_IO_X32P_SLP_OE_R {
        RTC_IO_X32P_SLP_OE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_fun_ie(&self) -> RTC_IO_X32P_FUN_IE_R {
        RTC_IO_X32P_FUN_IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 32K XTAL resistor bias control."]
    #[inline(always)]
    pub fn rtc_io_dres_xtal_32k(&self) -> RTC_IO_DRES_XTAL_32K_R {
        RTC_IO_DRES_XTAL_32K_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - 32K XTAL self-bias reference control."]
    #[inline(always)]
    pub fn rtc_io_dbias_xtal_32k(&self) -> RTC_IO_DBIAS_XTAL_32K_R {
        RTC_IO_DBIAS_XTAL_32K_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_drv(&mut self) -> RTC_IO_X32N_DRV_W {
        RTC_IO_X32N_DRV_W { w: self }
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_x32n_hold(&mut self) -> RTC_IO_X32N_HOLD_W {
        RTC_IO_X32N_HOLD_W { w: self }
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_rde(&mut self) -> RTC_IO_X32N_RDE_W {
        RTC_IO_X32N_RDE_W { w: self }
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_rue(&mut self) -> RTC_IO_X32N_RUE_W {
        RTC_IO_X32N_RUE_W { w: self }
    }
    #[doc = "Bits 25:26 - the driver strength of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_drv(&mut self) -> RTC_IO_X32P_DRV_W {
        RTC_IO_X32P_DRV_W { w: self }
    }
    #[doc = "Bit 24 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_x32p_hold(&mut self) -> RTC_IO_X32P_HOLD_W {
        RTC_IO_X32P_HOLD_W { w: self }
    }
    #[doc = "Bit 23 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_rde(&mut self) -> RTC_IO_X32P_RDE_W {
        RTC_IO_X32P_RDE_W { w: self }
    }
    #[doc = "Bit 22 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_rue(&mut self) -> RTC_IO_X32P_RUE_W {
        RTC_IO_X32P_RUE_W { w: self }
    }
    #[doc = "Bits 20:21 - 32K XTAL bias current DAC."]
    #[inline(always)]
    pub fn rtc_io_dac_xtal_32k(&mut self) -> RTC_IO_DAC_XTAL_32K_W {
        RTC_IO_DAC_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 19 - Power up 32kHz crystal oscillator"]
    #[inline(always)]
    pub fn rtc_io_xpd_xtal_32k(&mut self) -> RTC_IO_XPD_XTAL_32K_W {
        RTC_IO_XPD_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 18 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_x32n_mux_sel(&mut self) -> RTC_IO_X32N_MUX_SEL_W {
        RTC_IO_X32N_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 17 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_x32p_mux_sel(&mut self) -> RTC_IO_X32P_MUX_SEL_W {
        RTC_IO_X32P_MUX_SEL_W { w: self }
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_fun_sel(&mut self) -> RTC_IO_X32N_FUN_SEL_W {
        RTC_IO_X32N_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_slp_sel(&mut self) -> RTC_IO_X32N_SLP_SEL_W {
        RTC_IO_X32N_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32n_slp_ie(&mut self) -> RTC_IO_X32N_SLP_IE_W {
        RTC_IO_X32N_SLP_IE_W { w: self }
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32n_slp_oe(&mut self) -> RTC_IO_X32N_SLP_OE_W {
        RTC_IO_X32N_SLP_OE_W { w: self }
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32n_fun_ie(&mut self) -> RTC_IO_X32N_FUN_IE_W {
        RTC_IO_X32N_FUN_IE_W { w: self }
    }
    #[doc = "Bits 9:10 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_fun_sel(&mut self) -> RTC_IO_X32P_FUN_SEL_W {
        RTC_IO_X32P_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 8 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_slp_sel(&mut self) -> RTC_IO_X32P_SLP_SEL_W {
        RTC_IO_X32P_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 7 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32p_slp_ie(&mut self) -> RTC_IO_X32P_SLP_IE_W {
        RTC_IO_X32P_SLP_IE_W { w: self }
    }
    #[doc = "Bit 6 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_x32p_slp_oe(&mut self) -> RTC_IO_X32P_SLP_OE_W {
        RTC_IO_X32P_SLP_OE_W { w: self }
    }
    #[doc = "Bit 5 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_x32p_fun_ie(&mut self) -> RTC_IO_X32P_FUN_IE_W {
        RTC_IO_X32P_FUN_IE_W { w: self }
    }
    #[doc = "Bits 3:4 - 32K XTAL resistor bias control."]
    #[inline(always)]
    pub fn rtc_io_dres_xtal_32k(&mut self) -> RTC_IO_DRES_XTAL_32K_W {
        RTC_IO_DRES_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 1:2 - 32K XTAL self-bias reference control."]
    #[inline(always)]
    pub fn rtc_io_dbias_xtal_32k(&mut self) -> RTC_IO_DBIAS_XTAL_32K_W {
        RTC_IO_DBIAS_XTAL_32K_W { w: self }
    }
}
