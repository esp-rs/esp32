#[doc = "Reader of register TIMER5"]
pub type R = crate::R<u32, super::TIMER5>;
#[doc = "Writer for register TIMER5"]
pub type W = crate::W<u32, super::TIMER5>;
#[doc = "Register TIMER5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCMEM_POWERUP_TIMER`"]
pub type RTCMEM_POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCMEM_POWERUP_TIMER`"]
pub struct RTCMEM_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCMEM_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTCMEM_WAIT_TIMER`"]
pub type RTCMEM_WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTCMEM_WAIT_TIMER`"]
pub struct RTCMEM_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCMEM_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MIN_SLP_VAL`"]
pub type MIN_SLP_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_SLP_VAL`"]
pub struct MIN_SLP_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_SLP_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ULP_CP_SUBTIMER_PREDIV`"]
pub type ULP_CP_SUBTIMER_PREDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ULP_CP_SUBTIMER_PREDIV`"]
pub struct ULP_CP_SUBTIMER_PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_SUBTIMER_PREDIV_W<'a> {
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
    pub fn rtcmem_powerup_timer(&self) -> RTCMEM_POWERUP_TIMER_R {
        RTCMEM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&self) -> RTCMEM_WAIT_TIMER_R {
        RTCMEM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&self) -> ULP_CP_SUBTIMER_PREDIV_R {
        ULP_CP_SUBTIMER_PREDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtcmem_powerup_timer(&mut self) -> RTCMEM_POWERUP_TIMER_W {
        RTCMEM_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&mut self) -> RTCMEM_WAIT_TIMER_W {
        RTCMEM_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W {
        MIN_SLP_VAL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&mut self) -> ULP_CP_SUBTIMER_PREDIV_W {
        ULP_CP_SUBTIMER_PREDIV_W { w: self }
    }
}
