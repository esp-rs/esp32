#[doc = "Reader of register SAR_TOUCH_THRES3"]
pub type R = crate::R<u32, super::SAR_TOUCH_THRES3>;
#[doc = "Writer for register SAR_TOUCH_THRES3"]
pub type W = crate::W<u32, super::SAR_TOUCH_THRES3>;
#[doc = "Register SAR_TOUCH_THRES3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_THRES3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUCH_OUT_TH4`"]
pub type TOUCH_OUT_TH4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_OUT_TH4`"]
pub struct TOUCH_OUT_TH4_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_TH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_OUT_TH5`"]
pub type TOUCH_OUT_TH5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_OUT_TH5`"]
pub struct TOUCH_OUT_TH5_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_TH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - the threshold for touch pad 4"]
    #[inline(always)]
    pub fn touch_out_th4(&self) -> TOUCH_OUT_TH4_R {
        TOUCH_OUT_TH4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the threshold for touch pad 5"]
    #[inline(always)]
    pub fn touch_out_th5(&self) -> TOUCH_OUT_TH5_R {
        TOUCH_OUT_TH5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the threshold for touch pad 4"]
    #[inline(always)]
    pub fn touch_out_th4(&mut self) -> TOUCH_OUT_TH4_W {
        TOUCH_OUT_TH4_W { w: self }
    }
    #[doc = "Bits 0:15 - the threshold for touch pad 5"]
    #[inline(always)]
    pub fn touch_out_th5(&mut self) -> TOUCH_OUT_TH5_W {
        TOUCH_OUT_TH5_W { w: self }
    }
}
