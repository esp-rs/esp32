#[doc = "Reader of register GPIO_SIGMADELTA_VERSION_REG"]
pub type R = crate::R<u32, super::GPIO_SIGMADELTA_VERSION_REG>;
#[doc = "Writer for register GPIO_SIGMADELTA_VERSION_REG"]
pub type W = crate::W<u32, super::GPIO_SIGMADELTA_VERSION_REG>;
#[doc = "Register GPIO_SIGMADELTA_VERSION_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_SIGMADELTA_VERSION_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_SD_DATE`"]
pub type GPIO_SD_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_SD_DATE`"]
pub struct GPIO_SD_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn gpio_sd_date(&self) -> GPIO_SD_DATE_R {
        GPIO_SD_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn gpio_sd_date(&mut self) -> GPIO_SD_DATE_W {
        GPIO_SD_DATE_W { w: self }
    }
}
