#[doc = "Reader of register TIMG_RTCCALICFG1_REG"]
pub type R = crate::R<u32, super::TIMG_RTCCALICFG1_REG>;
#[doc = "Writer for register TIMG_RTCCALICFG1_REG"]
pub type W = crate::W<u32, super::TIMG_RTCCALICFG1_REG>;
#[doc = "Register TIMG_RTCCALICFG1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_RTCCALICFG1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_RTC_CALI_VALUE`"]
pub type TIMG_RTC_CALI_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_RTC_CALI_VALUE`"]
pub struct TIMG_RTC_CALI_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_RTC_CALI_VALUE_W<'a> {
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
    pub fn timg_rtc_cali_value(&self) -> TIMG_RTC_CALI_VALUE_R {
        TIMG_RTC_CALI_VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn timg_rtc_cali_value(&mut self) -> TIMG_RTC_CALI_VALUE_W {
        TIMG_RTC_CALI_VALUE_W { w: self }
    }
}
