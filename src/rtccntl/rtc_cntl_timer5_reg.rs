#[doc = "Reader of register RTC_CNTL_TIMER5_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_TIMER5_REG>;
#[doc = "Writer for register RTC_CNTL_TIMER5_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_TIMER5_REG>;
#[doc = "Register RTC_CNTL_TIMER5_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_TIMER5_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_RTCMEM_POWERUP_TIMER`"]
pub type RTC_CNTL_RTCMEM_POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_RTCMEM_POWERUP_TIMER`"]
pub struct RTC_CNTL_RTCMEM_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RTCMEM_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_RTCMEM_WAIT_TIMER`"]
pub type RTC_CNTL_RTCMEM_WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_RTCMEM_WAIT_TIMER`"]
pub struct RTC_CNTL_RTCMEM_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RTCMEM_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_MIN_SLP_VAL`"]
pub type RTC_CNTL_MIN_SLP_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_MIN_SLP_VAL`"]
pub struct RTC_CNTL_MIN_SLP_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_MIN_SLP_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ULP_CP_SUBTIMER_PREDIV`"]
pub type RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_ULP_CP_SUBTIMER_PREDIV`"]
pub struct RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtc_cntl_rtcmem_powerup_timer(&self) -> RTC_CNTL_RTCMEM_POWERUP_TIMER_R {
        RTC_CNTL_RTCMEM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtc_cntl_rtcmem_wait_timer(&self) -> RTC_CNTL_RTCMEM_WAIT_TIMER_R {
        RTC_CNTL_RTCMEM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_min_slp_val(&self) -> RTC_CNTL_MIN_SLP_VAL_R {
        RTC_CNTL_MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_subtimer_prediv(&self) -> RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_R {
        RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtc_cntl_rtcmem_powerup_timer(&mut self) -> RTC_CNTL_RTCMEM_POWERUP_TIMER_W {
        RTC_CNTL_RTCMEM_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtc_cntl_rtcmem_wait_timer(&mut self) -> RTC_CNTL_RTCMEM_WAIT_TIMER_W {
        RTC_CNTL_RTCMEM_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_min_slp_val(&mut self) -> RTC_CNTL_MIN_SLP_VAL_W {
        RTC_CNTL_MIN_SLP_VAL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_subtimer_prediv(&mut self) -> RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_W {
        RTC_CNTL_ULP_CP_SUBTIMER_PREDIV_W { w: self }
    }
}
