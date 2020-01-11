#[doc = "Reader of register CPU_PERIOD_CONF"]
pub type R = crate::R<u32, super::CPU_PERIOD_CONF>;
#[doc = "Writer for register CPU_PERIOD_CONF"]
pub type W = crate::W<u32, super::CPU_PERIOD_CONF>;
#[doc = "Register CPU_PERIOD_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_PERIOD_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_CPUPERIOD_SEL`"]
pub type RTC_CNTL_CPUPERIOD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_CPUPERIOD_SEL`"]
pub struct RTC_CNTL_CPUPERIOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPUPERIOD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CPUSEL_CONF`"]
pub type RTC_CNTL_CPUSEL_CONF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPUSEL_CONF`"]
pub struct RTC_CNTL_CPUSEL_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPUSEL_CONF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - CPU period sel"]
    #[inline(always)]
    pub fn rtc_cntl_cpuperiod_sel(&self) -> RTC_CNTL_CPUPERIOD_SEL_R {
        RTC_CNTL_CPUPERIOD_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn rtc_cntl_cpusel_conf(&self) -> RTC_CNTL_CPUSEL_CONF_R {
        RTC_CNTL_CPUSEL_CONF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - CPU period sel"]
    #[inline(always)]
    pub fn rtc_cntl_cpuperiod_sel(&mut self) -> RTC_CNTL_CPUPERIOD_SEL_W {
        RTC_CNTL_CPUPERIOD_SEL_W { w: self }
    }
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn rtc_cntl_cpusel_conf(&mut self) -> RTC_CNTL_CPUSEL_CONF_W {
        RTC_CNTL_CPUSEL_CONF_W { w: self }
    }
}
