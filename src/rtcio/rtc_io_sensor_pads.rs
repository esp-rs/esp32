#[doc = "Reader of register RTC_IO_SENSOR_PADS"]
pub type R = crate::R<u32, super::RTC_IO_SENSOR_PADS>;
#[doc = "Writer for register RTC_IO_SENSOR_PADS"]
pub type W = crate::W<u32, super::RTC_IO_SENSOR_PADS>;
#[doc = "Register RTC_IO_SENSOR_PADS `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_SENSOR_PADS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_SENSE1_HOLD`"]
pub type RTC_IO_SENSE1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE1_HOLD`"]
pub struct RTC_IO_SENSE1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE1_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE2_HOLD`"]
pub type RTC_IO_SENSE2_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE2_HOLD`"]
pub struct RTC_IO_SENSE2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE2_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE3_HOLD`"]
pub type RTC_IO_SENSE3_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE3_HOLD`"]
pub struct RTC_IO_SENSE3_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE3_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE4_HOLD`"]
pub type RTC_IO_SENSE4_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE4_HOLD`"]
pub struct RTC_IO_SENSE4_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE4_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE1_MUX_SEL`"]
pub type RTC_IO_SENSE1_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE1_MUX_SEL`"]
pub struct RTC_IO_SENSE1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE1_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE2_MUX_SEL`"]
pub type RTC_IO_SENSE2_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE2_MUX_SEL`"]
pub struct RTC_IO_SENSE2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE2_MUX_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SENSE3_MUX_SEL`"]
pub type RTC_IO_SENSE3_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE3_MUX_SEL`"]
pub struct RTC_IO_SENSE3_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE3_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE4_MUX_SEL`"]
pub type RTC_IO_SENSE4_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE4_MUX_SEL`"]
pub struct RTC_IO_SENSE4_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE4_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE1_FUN_SEL`"]
pub type RTC_IO_SENSE1_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SENSE1_FUN_SEL`"]
pub struct RTC_IO_SENSE1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SENSE1_SLP_SEL`"]
pub type RTC_IO_SENSE1_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE1_SLP_SEL`"]
pub struct RTC_IO_SENSE1_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE1_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE1_SLP_IE`"]
pub type RTC_IO_SENSE1_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE1_SLP_IE`"]
pub struct RTC_IO_SENSE1_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE1_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE1_FUN_IE`"]
pub type RTC_IO_SENSE1_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE1_FUN_IE`"]
pub struct RTC_IO_SENSE1_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE1_FUN_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE2_FUN_SEL`"]
pub type RTC_IO_SENSE2_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SENSE2_FUN_SEL`"]
pub struct RTC_IO_SENSE2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SENSE2_SLP_SEL`"]
pub type RTC_IO_SENSE2_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE2_SLP_SEL`"]
pub struct RTC_IO_SENSE2_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE2_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE2_SLP_IE`"]
pub type RTC_IO_SENSE2_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE2_SLP_IE`"]
pub struct RTC_IO_SENSE2_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE2_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE2_FUN_IE`"]
pub type RTC_IO_SENSE2_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE2_FUN_IE`"]
pub struct RTC_IO_SENSE2_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE2_FUN_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE3_FUN_SEL`"]
pub type RTC_IO_SENSE3_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SENSE3_FUN_SEL`"]
pub struct RTC_IO_SENSE3_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE3_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SENSE3_SLP_SEL`"]
pub type RTC_IO_SENSE3_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE3_SLP_SEL`"]
pub struct RTC_IO_SENSE3_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE3_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE3_SLP_IE`"]
pub type RTC_IO_SENSE3_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE3_SLP_IE`"]
pub struct RTC_IO_SENSE3_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE3_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE3_FUN_IE`"]
pub type RTC_IO_SENSE3_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE3_FUN_IE`"]
pub struct RTC_IO_SENSE3_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE3_FUN_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SENSE4_FUN_SEL`"]
pub type RTC_IO_SENSE4_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SENSE4_FUN_SEL`"]
pub struct RTC_IO_SENSE4_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE4_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SENSE4_SLP_SEL`"]
pub type RTC_IO_SENSE4_SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE4_SLP_SEL`"]
pub struct RTC_IO_SENSE4_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE4_SLP_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE4_SLP_IE`"]
pub type RTC_IO_SENSE4_SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE4_SLP_IE`"]
pub struct RTC_IO_SENSE4_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE4_SLP_IE_W<'a> {
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
#[doc = "Reader of field `RTC_IO_SENSE4_FUN_IE`"]
pub type RTC_IO_SENSE4_FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_SENSE4_FUN_IE`"]
pub struct RTC_IO_SENSE4_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SENSE4_FUN_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense1_hold(&self) -> RTC_IO_SENSE1_HOLD_R {
        RTC_IO_SENSE1_HOLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense2_hold(&self) -> RTC_IO_SENSE2_HOLD_R {
        RTC_IO_SENSE2_HOLD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense3_hold(&self) -> RTC_IO_SENSE3_HOLD_R {
        RTC_IO_SENSE3_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense4_hold(&self) -> RTC_IO_SENSE4_HOLD_R {
        RTC_IO_SENSE4_HOLD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense1_mux_sel(&self) -> RTC_IO_SENSE1_MUX_SEL_R {
        RTC_IO_SENSE1_MUX_SEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense2_mux_sel(&self) -> RTC_IO_SENSE2_MUX_SEL_R {
        RTC_IO_SENSE2_MUX_SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense3_mux_sel(&self) -> RTC_IO_SENSE3_MUX_SEL_R {
        RTC_IO_SENSE3_MUX_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense4_mux_sel(&self) -> RTC_IO_SENSE4_MUX_SEL_R {
        RTC_IO_SENSE4_MUX_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense1_fun_sel(&self) -> RTC_IO_SENSE1_FUN_SEL_R {
        RTC_IO_SENSE1_FUN_SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense1_slp_sel(&self) -> RTC_IO_SENSE1_SLP_SEL_R {
        RTC_IO_SENSE1_SLP_SEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense1_slp_ie(&self) -> RTC_IO_SENSE1_SLP_IE_R {
        RTC_IO_SENSE1_SLP_IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense1_fun_ie(&self) -> RTC_IO_SENSE1_FUN_IE_R {
        RTC_IO_SENSE1_FUN_IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense2_fun_sel(&self) -> RTC_IO_SENSE2_FUN_SEL_R {
        RTC_IO_SENSE2_FUN_SEL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense2_slp_sel(&self) -> RTC_IO_SENSE2_SLP_SEL_R {
        RTC_IO_SENSE2_SLP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense2_slp_ie(&self) -> RTC_IO_SENSE2_SLP_IE_R {
        RTC_IO_SENSE2_SLP_IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense2_fun_ie(&self) -> RTC_IO_SENSE2_FUN_IE_R {
        RTC_IO_SENSE2_FUN_IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense3_fun_sel(&self) -> RTC_IO_SENSE3_FUN_SEL_R {
        RTC_IO_SENSE3_FUN_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense3_slp_sel(&self) -> RTC_IO_SENSE3_SLP_SEL_R {
        RTC_IO_SENSE3_SLP_SEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense3_slp_ie(&self) -> RTC_IO_SENSE3_SLP_IE_R {
        RTC_IO_SENSE3_SLP_IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense3_fun_ie(&self) -> RTC_IO_SENSE3_FUN_IE_R {
        RTC_IO_SENSE3_FUN_IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense4_fun_sel(&self) -> RTC_IO_SENSE4_FUN_SEL_R {
        RTC_IO_SENSE4_FUN_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense4_slp_sel(&self) -> RTC_IO_SENSE4_SLP_SEL_R {
        RTC_IO_SENSE4_SLP_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense4_slp_ie(&self) -> RTC_IO_SENSE4_SLP_IE_R {
        RTC_IO_SENSE4_SLP_IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense4_fun_ie(&self) -> RTC_IO_SENSE4_FUN_IE_R {
        RTC_IO_SENSE4_FUN_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense1_hold(&mut self) -> RTC_IO_SENSE1_HOLD_W {
        RTC_IO_SENSE1_HOLD_W { w: self }
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense2_hold(&mut self) -> RTC_IO_SENSE2_HOLD_W {
        RTC_IO_SENSE2_HOLD_W { w: self }
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense3_hold(&mut self) -> RTC_IO_SENSE3_HOLD_W {
        RTC_IO_SENSE3_HOLD_W { w: self }
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to \u{d2}1\u{d3}"]
    #[inline(always)]
    pub fn rtc_io_sense4_hold(&mut self) -> RTC_IO_SENSE4_HOLD_W {
        RTC_IO_SENSE4_HOLD_W { w: self }
    }
    #[doc = "Bit 27 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense1_mux_sel(&mut self) -> RTC_IO_SENSE1_MUX_SEL_W {
        RTC_IO_SENSE1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 26 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense2_mux_sel(&mut self) -> RTC_IO_SENSE2_MUX_SEL_W {
        RTC_IO_SENSE2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 25 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense3_mux_sel(&mut self) -> RTC_IO_SENSE3_MUX_SEL_W {
        RTC_IO_SENSE3_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 24 - \u{d2}1\u{d3} select the digital function \u{d3}0\u{d3}slection the rtc function"]
    #[inline(always)]
    pub fn rtc_io_sense4_mux_sel(&mut self) -> RTC_IO_SENSE4_MUX_SEL_W {
        RTC_IO_SENSE4_MUX_SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense1_fun_sel(&mut self) -> RTC_IO_SENSE1_FUN_SEL_W {
        RTC_IO_SENSE1_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense1_slp_sel(&mut self) -> RTC_IO_SENSE1_SLP_SEL_W {
        RTC_IO_SENSE1_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense1_slp_ie(&mut self) -> RTC_IO_SENSE1_SLP_IE_W {
        RTC_IO_SENSE1_SLP_IE_W { w: self }
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense1_fun_ie(&mut self) -> RTC_IO_SENSE1_FUN_IE_W {
        RTC_IO_SENSE1_FUN_IE_W { w: self }
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense2_fun_sel(&mut self) -> RTC_IO_SENSE2_FUN_SEL_W {
        RTC_IO_SENSE2_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense2_slp_sel(&mut self) -> RTC_IO_SENSE2_SLP_SEL_W {
        RTC_IO_SENSE2_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense2_slp_ie(&mut self) -> RTC_IO_SENSE2_SLP_IE_W {
        RTC_IO_SENSE2_SLP_IE_W { w: self }
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense2_fun_ie(&mut self) -> RTC_IO_SENSE2_FUN_IE_W {
        RTC_IO_SENSE2_FUN_IE_W { w: self }
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense3_fun_sel(&mut self) -> RTC_IO_SENSE3_FUN_SEL_W {
        RTC_IO_SENSE3_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense3_slp_sel(&mut self) -> RTC_IO_SENSE3_SLP_SEL_W {
        RTC_IO_SENSE3_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense3_slp_ie(&mut self) -> RTC_IO_SENSE3_SLP_IE_W {
        RTC_IO_SENSE3_SLP_IE_W { w: self }
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense3_fun_ie(&mut self) -> RTC_IO_SENSE3_FUN_IE_W {
        RTC_IO_SENSE3_FUN_IE_W { w: self }
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense4_fun_sel(&mut self) -> RTC_IO_SENSE4_FUN_SEL_W {
        RTC_IO_SENSE4_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense4_slp_sel(&mut self) -> RTC_IO_SENSE4_SLP_SEL_W {
        RTC_IO_SENSE4_SLP_SEL_W { w: self }
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn rtc_io_sense4_slp_ie(&mut self) -> RTC_IO_SENSE4_SLP_IE_W {
        RTC_IO_SENSE4_SLP_IE_W { w: self }
    }
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    pub fn rtc_io_sense4_fun_ie(&mut self) -> RTC_IO_SENSE4_FUN_IE_W {
        RTC_IO_SENSE4_FUN_IE_W { w: self }
    }
}
