#[doc = "Reader of register SAR_TOUCH_ENABLE"]
pub type R = crate::R<u32, super::SAR_TOUCH_ENABLE>;
#[doc = "Writer for register SAR_TOUCH_ENABLE"]
pub type W = crate::W<u32, super::SAR_TOUCH_ENABLE>;
#[doc = "Register SAR_TOUCH_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUCH_PAD_OUTEN1`"]
pub type TOUCH_PAD_OUTEN1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_PAD_OUTEN1`"]
pub struct TOUCH_PAD_OUTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD_OUTEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_PAD_OUTEN2`"]
pub type TOUCH_PAD_OUTEN2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_PAD_OUTEN2`"]
pub struct TOUCH_PAD_OUTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD_OUTEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_PAD_WORKEN`"]
pub type TOUCH_PAD_WORKEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_PAD_WORKEN`"]
pub struct TOUCH_PAD_WORKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD_WORKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29 - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen1(&self) -> TOUCH_PAD_OUTEN1_R {
        TOUCH_PAD_OUTEN1_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen2(&self) -> TOUCH_PAD_OUTEN2_R {
        TOUCH_PAD_OUTEN2_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - Bitmap defining the working set during the measurement."]
    #[inline(always)]
    pub fn touch_pad_worken(&self) -> TOUCH_PAD_WORKEN_R {
        TOUCH_PAD_WORKEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:29 - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen1(&mut self) -> TOUCH_PAD_OUTEN1_W {
        TOUCH_PAD_OUTEN1_W { w: self }
    }
    #[doc = "Bits 10:19 - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen2(&mut self) -> TOUCH_PAD_OUTEN2_W {
        TOUCH_PAD_OUTEN2_W { w: self }
    }
    #[doc = "Bits 0:9 - Bitmap defining the working set during the measurement."]
    #[inline(always)]
    pub fn touch_pad_worken(&mut self) -> TOUCH_PAD_WORKEN_W {
        TOUCH_PAD_WORKEN_W { w: self }
    }
}
