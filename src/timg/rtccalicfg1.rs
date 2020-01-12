#[doc = "Reader of register RTCCALICFG1"]
pub type R = crate::R<u32, super::RTCCALICFG1>;
#[doc = "Writer for register RTCCALICFG1"]
pub type W = crate::W<u32, super::RTCCALICFG1>;
#[doc = "Register RTCCALICFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCALICFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CALI_VALUE`"]
pub type RTC_CALI_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CALI_VALUE`"]
pub struct RTC_CALI_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> RTC_CALI_VALUE_R {
        RTC_CALI_VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn rtc_cali_value(&mut self) -> RTC_CALI_VALUE_W {
        RTC_CALI_VALUE_W { w: self }
    }
}