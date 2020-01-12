#[doc = "Reader of register SAR_TOUCH_OUT5"]
pub type R = crate::R<u32, super::SAR_TOUCH_OUT5>;
#[doc = "Writer for register SAR_TOUCH_OUT5"]
pub type W = crate::W<u32, super::SAR_TOUCH_OUT5>;
#[doc = "Register SAR_TOUCH_OUT5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_OUT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUCH_MEAS_OUT8`"]
pub type TOUCH_MEAS_OUT8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_MEAS_OUT8`"]
pub struct TOUCH_MEAS_OUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_OUT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_MEAS_OUT9`"]
pub type TOUCH_MEAS_OUT9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_MEAS_OUT9`"]
pub struct TOUCH_MEAS_OUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_OUT9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - the counter for touch pad 8"]
    #[inline(always)]
    pub fn touch_meas_out8(&self) -> TOUCH_MEAS_OUT8_R {
        TOUCH_MEAS_OUT8_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the counter for touch pad 9"]
    #[inline(always)]
    pub fn touch_meas_out9(&self) -> TOUCH_MEAS_OUT9_R {
        TOUCH_MEAS_OUT9_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the counter for touch pad 8"]
    #[inline(always)]
    pub fn touch_meas_out8(&mut self) -> TOUCH_MEAS_OUT8_W {
        TOUCH_MEAS_OUT8_W { w: self }
    }
    #[doc = "Bits 0:15 - the counter for touch pad 9"]
    #[inline(always)]
    pub fn touch_meas_out9(&mut self) -> TOUCH_MEAS_OUT9_W {
        TOUCH_MEAS_OUT9_W { w: self }
    }
}
