#[doc = "Reader of register RTC_GPIO_OUT_W1TC"]
pub type R = crate::R<u32, super::RTC_GPIO_OUT_W1TC>;
#[doc = "Writer for register RTC_GPIO_OUT_W1TC"]
pub type W = crate::W<u32, super::RTC_GPIO_OUT_W1TC>;
#[doc = "Register RTC_GPIO_OUT_W1TC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_GPIO_OUT_W1TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_GPIO_OUT_DATA_W1TC`"]
pub type RTC_GPIO_OUT_DATA_W1TC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_GPIO_OUT_DATA_W1TC`"]
pub struct RTC_GPIO_OUT_DATA_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_OUT_DATA_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | (((value as u32) & 0x0003_ffff) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:31 - GPIO0~17 output value write 1 to clear"]
    #[inline(always)]
    pub fn rtc_gpio_out_data_w1tc(&self) -> RTC_GPIO_OUT_DATA_W1TC_R {
        RTC_GPIO_OUT_DATA_W1TC_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 14:31 - GPIO0~17 output value write 1 to clear"]
    #[inline(always)]
    pub fn rtc_gpio_out_data_w1tc(&mut self) -> RTC_GPIO_OUT_DATA_W1TC_W {
        RTC_GPIO_OUT_DATA_W1TC_W { w: self }
    }
}
