#[doc = "Reader of register SENS_SAR_TOUCH_THRES3_REG"]
pub type R = crate::R<u32, super::SENS_SAR_TOUCH_THRES3_REG>;
#[doc = "Writer for register SENS_SAR_TOUCH_THRES3_REG"]
pub type W = crate::W<u32, super::SENS_SAR_TOUCH_THRES3_REG>;
#[doc = "Register SENS_SAR_TOUCH_THRES3_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_TOUCH_THRES3_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_TOUCH_OUT_TH4`"]
pub type SENS_TOUCH_OUT_TH4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_OUT_TH4`"]
pub struct SENS_TOUCH_OUT_TH4_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_OUT_TH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_OUT_TH5`"]
pub type SENS_TOUCH_OUT_TH5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_OUT_TH5`"]
pub struct SENS_TOUCH_OUT_TH5_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_OUT_TH5_W<'a> {
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
    pub fn sens_touch_out_th4(&self) -> SENS_TOUCH_OUT_TH4_R {
        SENS_TOUCH_OUT_TH4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the threshold for touch pad 5"]
    #[inline(always)]
    pub fn sens_touch_out_th5(&self) -> SENS_TOUCH_OUT_TH5_R {
        SENS_TOUCH_OUT_TH5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the threshold for touch pad 4"]
    #[inline(always)]
    pub fn sens_touch_out_th4(&mut self) -> SENS_TOUCH_OUT_TH4_W {
        SENS_TOUCH_OUT_TH4_W { w: self }
    }
    #[doc = "Bits 0:15 - the threshold for touch pad 5"]
    #[inline(always)]
    pub fn sens_touch_out_th5(&mut self) -> SENS_TOUCH_OUT_TH5_W {
        SENS_TOUCH_OUT_TH5_W { w: self }
    }
}
