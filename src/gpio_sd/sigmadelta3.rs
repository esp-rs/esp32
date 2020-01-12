#[doc = "Reader of register SIGMADELTA3"]
pub type R = crate::R<u32, super::SIGMADELTA3>;
#[doc = "Writer for register SIGMADELTA3"]
pub type W = crate::W<u32, super::SIGMADELTA3>;
#[doc = "Register SIGMADELTA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGMADELTA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SD3_PRESCALE`"]
pub type SD3_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD3_PRESCALE`"]
pub struct SD3_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD3_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SD3_IN`"]
pub type SD3_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD3_IN`"]
pub struct SD3_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD3_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd3_prescale(&self) -> SD3_PRESCALE_R {
        SD3_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd3_in(&self) -> SD3_IN_R {
        SD3_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd3_prescale(&mut self) -> SD3_PRESCALE_W {
        SD3_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd3_in(&mut self) -> SD3_IN_W {
        SD3_IN_W { w: self }
    }
}
