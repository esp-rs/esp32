#[doc = "Reader of register RTC_CNTL_WDTFEED"]
pub type R = crate::R<u32, super::RTC_CNTL_WDTFEED>;
#[doc = "Writer for register RTC_CNTL_WDTFEED"]
pub type W = crate::W<u32, super::RTC_CNTL_WDTFEED>;
#[doc = "Register RTC_CNTL_WDTFEED `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_WDTFEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_FEED`"]
pub type RTC_CNTL_WDT_FEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_FEED`"]
pub struct RTC_CNTL_WDT_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_FEED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_feed(&self) -> RTC_CNTL_WDT_FEED_R {
        RTC_CNTL_WDT_FEED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_feed(&mut self) -> RTC_CNTL_WDT_FEED_W {
        RTC_CNTL_WDT_FEED_W { w: self }
    }
}
