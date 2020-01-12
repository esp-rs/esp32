#[doc = "Reader of register EXT_WAKEUP1"]
pub type R = crate::R<u32, super::EXT_WAKEUP1>;
#[doc = "Writer for register EXT_WAKEUP1"]
pub type W = crate::W<u32, super::EXT_WAKEUP1>;
#[doc = "Register EXT_WAKEUP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_WAKEUP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXT_WAKEUP1_STATUS_CLR`"]
pub type EXT_WAKEUP1_STATUS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_WAKEUP1_STATUS_CLR`"]
pub struct EXT_WAKEUP1_STATUS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP1_STATUS_CLR_W<'a> {
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
#[doc = "Reader of field `EXT_WAKEUP1_SEL`"]
pub type EXT_WAKEUP1_SEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EXT_WAKEUP1_SEL`"]
pub struct EXT_WAKEUP1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - clear ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status_clr(&self) -> EXT_WAKEUP1_STATUS_CLR_R {
        EXT_WAKEUP1_STATUS_CLR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 0:17 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn ext_wakeup1_sel(&self) -> EXT_WAKEUP1_SEL_R {
        EXT_WAKEUP1_SEL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 18 - clear ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status_clr(&mut self) -> EXT_WAKEUP1_STATUS_CLR_W {
        EXT_WAKEUP1_STATUS_CLR_W { w: self }
    }
    #[doc = "Bits 0:17 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn ext_wakeup1_sel(&mut self) -> EXT_WAKEUP1_SEL_W {
        EXT_WAKEUP1_SEL_W { w: self }
    }
}
