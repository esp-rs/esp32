#[doc = "Reader of register SIGMADELTA6"]
pub type R = crate::R<u32, super::SIGMADELTA6>;
#[doc = "Writer for register SIGMADELTA6"]
pub type W = crate::W<u32, super::SIGMADELTA6>;
#[doc = "Register SIGMADELTA6 `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGMADELTA6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_SD6_PRESCALE`"]
pub type GPIO_SD6_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_SD6_PRESCALE`"]
pub struct GPIO_SD6_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD6_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPIO_SD6_IN`"]
pub type GPIO_SD6_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_SD6_IN`"]
pub struct GPIO_SD6_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD6_IN_W<'a> {
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
    pub fn gpio_sd6_prescale(&self) -> GPIO_SD6_PRESCALE_R {
        GPIO_SD6_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sd6_in(&self) -> GPIO_SD6_IN_R {
        GPIO_SD6_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_sd6_prescale(&mut self) -> GPIO_SD6_PRESCALE_W {
        GPIO_SD6_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sd6_in(&mut self) -> GPIO_SD6_IN_W {
        GPIO_SD6_IN_W { w: self }
    }
}
