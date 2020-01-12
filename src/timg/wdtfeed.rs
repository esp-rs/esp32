#[doc = "Reader of register WDTFEED"]
pub type R = crate::R<u32, super::WDTFEED>;
#[doc = "Writer for register WDTFEED"]
pub type W = crate::W<u32, super::WDTFEED>;
#[doc = "Register WDTFEED `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTFEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_FEED`"]
pub type WDT_FEED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDT_FEED`"]
pub struct WDT_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_FEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write any value will feed SWDT"]
    #[inline(always)]
    pub fn wdt_feed(&self) -> WDT_FEED_R {
        WDT_FEED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value will feed SWDT"]
    #[inline(always)]
    pub fn wdt_feed(&mut self) -> WDT_FEED_W {
        WDT_FEED_W { w: self }
    }
}
