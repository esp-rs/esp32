#[doc = "Reader of register SAR_TOUCH_OUT2"]
pub type R = crate::R<u32, super::SAR_TOUCH_OUT2>;
#[doc = "Writer for register SAR_TOUCH_OUT2"]
pub type W = crate::W<u32, super::SAR_TOUCH_OUT2>;
#[doc = "Register SAR_TOUCH_OUT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_OUT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUCH_MEAS_OUT2`"]
pub type TOUCH_MEAS_OUT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_MEAS_OUT2`"]
pub struct TOUCH_MEAS_OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_OUT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_MEAS_OUT3`"]
pub type TOUCH_MEAS_OUT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_MEAS_OUT3`"]
pub struct TOUCH_MEAS_OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_OUT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - the counter for touch pad 2"]
    #[inline(always)]
    pub fn touch_meas_out2(&self) -> TOUCH_MEAS_OUT2_R {
        TOUCH_MEAS_OUT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the counter for touch pad 3"]
    #[inline(always)]
    pub fn touch_meas_out3(&self) -> TOUCH_MEAS_OUT3_R {
        TOUCH_MEAS_OUT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the counter for touch pad 2"]
    #[inline(always)]
    pub fn touch_meas_out2(&mut self) -> TOUCH_MEAS_OUT2_W {
        TOUCH_MEAS_OUT2_W { w: self }
    }
    #[doc = "Bits 0:15 - the counter for touch pad 3"]
    #[inline(always)]
    pub fn touch_meas_out3(&mut self) -> TOUCH_MEAS_OUT3_W {
        TOUCH_MEAS_OUT3_W { w: self }
    }
}
