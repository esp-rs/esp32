#[doc = "Reader of register APP_RTC_CORE_INTR_MAP"]
pub type R = crate::R<u32, super::APP_RTC_CORE_INTR_MAP>;
#[doc = "Writer for register APP_RTC_CORE_INTR_MAP"]
pub type W = crate::W<u32, super::APP_RTC_CORE_INTR_MAP>;
#[doc = "Register APP_RTC_CORE_INTR_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_RTC_CORE_INTR_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APP_RTC_CORE_INTR_MAP`"]
pub type DPORT_APP_RTC_CORE_INTR_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_APP_RTC_CORE_INTR_MAP`"]
pub struct DPORT_APP_RTC_CORE_INTR_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_RTC_CORE_INTR_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dport_app_rtc_core_intr_map(&self) -> DPORT_APP_RTC_CORE_INTR_MAP_R {
        DPORT_APP_RTC_CORE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dport_app_rtc_core_intr_map(&mut self) -> DPORT_APP_RTC_CORE_INTR_MAP_W {
        DPORT_APP_RTC_CORE_INTR_MAP_W { w: self }
    }
}
