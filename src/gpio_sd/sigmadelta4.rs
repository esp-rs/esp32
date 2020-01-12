#[doc = "Reader of register SIGMADELTA4"]
pub type R = crate::R<u32, super::SIGMADELTA4>;
#[doc = "Writer for register SIGMADELTA4"]
pub type W = crate::W<u32, super::SIGMADELTA4>;
#[doc = "Register SIGMADELTA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGMADELTA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SD4_PRESCALE`"]
pub type SD4_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD4_PRESCALE`"]
pub struct SD4_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD4_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SD4_IN`"]
pub type SD4_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD4_IN`"]
pub struct SD4_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD4_IN_W<'a> {
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
    pub fn sd4_prescale(&self) -> SD4_PRESCALE_R {
        SD4_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd4_in(&self) -> SD4_IN_R {
        SD4_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd4_prescale(&mut self) -> SD4_PRESCALE_W {
        SD4_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd4_in(&mut self) -> SD4_IN_W {
        SD4_IN_W { w: self }
    }
}
