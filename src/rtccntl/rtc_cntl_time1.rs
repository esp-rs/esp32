#[doc = "Reader of register RTC_CNTL_TIME1"]
pub type R = crate::R<u32, super::RTC_CNTL_TIME1>;
#[doc = "Writer for register RTC_CNTL_TIME1"]
pub type W = crate::W<u32, super::RTC_CNTL_TIME1>;
#[doc = "Register RTC_CNTL_TIME1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_TIME1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_TIME_HI`"]
pub type RTC_CNTL_TIME_HI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_TIME_HI`"]
pub struct RTC_CNTL_TIME_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIME_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_cntl_time_hi(&self) -> RTC_CNTL_TIME_HI_R {
        RTC_CNTL_TIME_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_cntl_time_hi(&mut self) -> RTC_CNTL_TIME_HI_W {
        RTC_CNTL_TIME_HI_W { w: self }
    }
}
